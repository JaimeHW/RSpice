//! Queueing several plans as one campaign.
//!
//! One of the five routes the plan-manager shell dispatches to. A campaign is
//! several authenticated runs and not one run over several plans: every member
//! is frozen as its own prepared snapshot before any engine work starts, and
//! dispatch mints a separate run, receipt, job and dataset for each, stamped
//! with the campaign's identity and that member's position in it.
//!
//! Every claim painted here is checked against
//! `SimulationController::prepare_and_start_campaign`, which is the only thing
//! that can queue one. Three claims the authored reference makes did not
//! survive that check:
//!
//! * *A failing member never cancels its siblings.* True only once dispatch has
//!   begun. Preparation walks the members with `?`, so a member that cannot be
//!   prepared refuses the whole campaign and nothing is queued at all. The
//!   reader is deciding at the moment the strict half applies, so the surface
//!   states both: admission is all or nothing, and after that a failed member
//!   does not stop the ones behind it.
//! * *Per-plan manifests plus one campaign summary record.* The per-member
//!   half holds — each member is its own prepared snapshot, and dispatch gives
//!   it its own receipt, run, job and dataset identity — but nothing in RSpice
//!   calls that a manifest, and there is no campaign summary record at all.
//!   `ActiveSimulationCampaign` is controller state that ends with the last
//!   member; the only thing that outlives the dispatch is
//!   `SimulationCampaignMembership`, written onto each member's own run and
//!   persisted with it. So the evidence is per member, which is what this says.
//! * *Members dispatch in declared order · shared target queue.* The order half
//!   is real and is this table's `Order` column. The queue is shared with
//!   nothing: the controller refuses to start while any run or campaign is
//!   active and hands out one member at a time. No campaign owns an execution
//!   target either — `prepare_and_start_campaign` takes no target parameter —
//!   so the authored target control is not ported rather than stubbed.
//!
//! Every task count here is the run set's own, read through
//! [`PlanCatalogRecord::task_count`]. Multiplying a point count by an
//! enabled-analysis count is different arithmetic, because the run set expands
//! analysis families and their prerequisites, and doing it locally stood a
//! second and wrong answer beside the authoritative one.

use super::*;

/// The three limits `prepare_and_start_campaign` refuses outside of.
///
/// It holds them privately, so they are restated rather than imported, and
/// `the_limits_this_route_prints_are_the_ones_the_controller_enforces` puts
/// each one to that function and fails if it has moved. Printing them is the
/// point: a reader should meet the caps here, not in a rejection.
const MIN_CAMPAIGN_MEMBERS: usize = 2;
const MAX_CAMPAIGN_MEMBERS: usize = 64;
const MAX_CAMPAIGN_NAME_CHARS: usize = 160;

/// What a forecast column shows for a plan whose run set does not validate.
///
/// Absent, not zero. A zero here would read as "this plan declares no work",
/// which is a different claim and a false one. The run set's own word for the
/// state is already in that row's `Run set` column, and the combined scope
/// above states how many members are in it, so the three derived columns say
/// only that they have nothing to report.
const NO_FORECAST: &str = "—";

/// Width at which the campaign's two note groups stop stacking.
///
/// A property row elides its value to one line, so the pairs cost height in one
/// track and legibility in two. Below this the seven rows are worth 203 points
/// of a body that has them; above it they are two columns of four and three.
const NOTE_TWO_COLUMN_WIDTH: f32 = 700.0;

/// Whitespace closing the body, for the reason `kit`'s `SPLIT_BOTTOM_INSET`
/// documents: this dialog's body viewport resolves a couple of points under the
/// content it was measured from, so whatever is painted last is cut mid-glyph.
///
/// That, and not a fold, is what the route's fit gate was reporting. Measured
/// at the desktop viewport, the summary row this route painted last had a
/// galley bottom of 407.5 against a clip bottom of 405.0 — two and a half
/// points — while the whole surface ended 200 points above the viewport's
/// bottom edge and both other gated shapes passed. The last row was clipped
/// because it was last, not because it was low, so moving it would only have
/// clipped whatever took its place. Whitespace here spends the shortfall on
/// nothing, and it costs no screen: it falls outside the viewport that cut it.
const BODY_BOTTOM_INSET: f32 = 6.0;

/// The member table, left to right.
///
/// `Order` is this route's own column and the reason the table is not a column
/// of checkboxes: `prepare_and_start_campaign` dispatches `member_ids` in the
/// order they are declared, so a member's position is a fact about what will
/// happen. Clicking a row appends it, which is what makes that number the
/// reader's own rather than the catalog's.
///
/// The four scale columns are the run set's own numbers, read off the
/// projection every other plan-manager surface reads. Only the identity column
/// is elastic, for the reason the browse table states: every other column's
/// longest value is known, so they hold their width and stay readable by
/// position down the rows.
///
/// The set totals eight points less than the browse table's, which is the
/// budget that decides whether this fits the portrait viewport — the narrowest
/// the dialog is gated at. A wider column here can push the table past it.
const MEMBER_COLUMNS: [TableColumn; 7] = [
    TableColumn {
        heading: "Order",
        track: ColumnTrack::Fixed(42.0),
    },
    TableColumn {
        heading: "Plan / identity",
        track: ColumnTrack::Elastic(120.0),
    },
    TableColumn {
        heading: "Analyses",
        track: ColumnTrack::Fixed(56.0),
    },
    TableColumn {
        heading: "Run set",
        track: ColumnTrack::Fixed(62.0),
    },
    TableColumn {
        heading: "Tasks",
        track: ColumnTrack::Fixed(48.0),
    },
    TableColumn {
        heading: "Est. time",
        track: ColumnTrack::Fixed(60.0),
    },
    TableColumn {
        heading: "Storage",
        track: ColumnTrack::Fixed(58.0),
    },
];

/// The combined declared scope of the members, summed from the per-member
/// forecasts the projection reports and from nothing else.
struct CampaignScope {
    /// Distinct members, counted the way the controller counts them.
    members: usize,
    /// Plans that could be members. Archived plans are not among them:
    /// `activate_campaign_plan` refuses one outright.
    eligible: usize,
    /// The sum of the members' own task counts.
    tasks: usize,
    /// Members whose run set does not validate and so forecast nothing. They
    /// contribute no tasks, and they are why the sum is not the whole story.
    unforecast: usize,
}

/// The route's dialog. See the shell's child-dialog signature contract.
pub(super) fn dialog(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    let eligible = records
        .iter()
        .filter(|record| !record.archived)
        .collect::<Vec<_>>();
    let has_archived = eligible.len() < records.len();
    let scope = campaign_scope(&draft.campaign.member_ids, records);
    let refusal = declared_refusal(&draft.campaign.name, &draft.campaign.member_ids);
    let mut action = None;
    let choice = Dialog::new(
        "SIMULATION · CAMPAIGN · ONE AUTHENTICATED RUN PER PLAN",
        "Queue simulation campaign",
        "Queue reviewed campaign",
    )
    .description(
        "Click a plan to add it as a member. Members dispatch in the order they were added, one at a time, each frozen now and run under its own authenticated identity.",
    )
    // The description above is the dialog's accessibility description and is
    // never painted, so the one rule a reader cannot deduce from the surface —
    // that the row is the control, and that adding puts a plan last — is stated
    // in the footer hint, which is.
    .hint("Click a plan to add or remove it · a plan added last dispatches last")
    .size(DialogSize::WideWorkflow)
    .ghost("Cancel")
    .primary_enabled(refusal.is_none())
    .show(ctx, |ui| {
        campaign_name_row(ui, &mut draft.campaign.name);
        ui.add_space(8.0);
        // The scope is stated above the members it sums, and deliberately.
        // It is the one number the confirmation exists for, and the member
        // table is the one part of this surface whose height the catalog
        // decides — so below it, the scope is what a catalog one plan larger
        // pushes out. `BODY_BOTTOM_INSET` is what fixed the clipping the fit
        // gate found; this is what keeps the catalog from putting it back.
        campaign_notes(ui, &scope, refusal.as_deref());
        ui.add_space(8.0);
        member_table(ui, &mut draft.campaign.member_ids, &eligible, has_archived);
        workflow_validation_message(ui, draft.validation_error.as_deref());
        ui.add_space(BODY_BOTTOM_INSET);
    });
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::ApplyCampaign),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

/// The campaign's name, with the length the controller refuses past shown
/// against it. A live count states the limit without a sentence about it, and
/// states it before the name is rejected rather than after.
fn campaign_name_row(ui: &mut Ui, name: &mut String) {
    let t = Tokens::get(ui.ctx());
    let count = name.chars().count();
    ui.horizontal(|ui| {
        ui.label("Campaign name");
        mono_input(ui, name, ui.available_width().min(360.0));
        ui.label(
            egui::RichText::new(format!("{count} / {MAX_CAMPAIGN_NAME_CHARS}"))
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(if count > MAX_CAMPAIGN_NAME_CHARS {
                    t.color.err
                } else {
                    t.color.text_dim
                }),
        );
    });
}

/// What this campaign declares, and what queueing it does.
///
/// Two groups rather than one list: the left is about this draft and changes as
/// the reader picks members, the right is about the transaction and does not.
/// They are side by side where there is width for both to stay legible, and
/// stacked where a value would elide to nothing.
fn campaign_notes(ui: &mut Ui, scope: &CampaignScope, refusal: Option<&str>) {
    if ui.available_width() >= NOTE_TWO_COLUMN_WIDTH {
        kit::equal_columns(ui, 2, |ui, index| {
            if index == 0 {
                declared_scope_group(ui, scope, refusal);
            } else {
                dispatch_guarantee_group(ui);
            }
        });
    } else {
        declared_scope_group(ui, scope, refusal);
        dispatch_guarantee_group(ui);
    }
}

/// How much work the selected members declare, and what stands between this
/// draft and being queued.
fn declared_scope_group(ui: &mut Ui, scope: &CampaignScope, refusal: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    kit::section_head(ui, "Combined declared scope", None);
    // "Members", not "Member plans": the table below is headed that, and one
    // name over two different things reads as the same thing said twice.
    property_row(
        ui,
        "Members",
        &format!(
            "{} of {} eligible · at most {MAX_CAMPAIGN_MEMBERS}",
            scope.members, scope.eligible
        ),
    );
    // "After", not "before". This route used to call the same number
    // "approximately N tasks before dependency expansion", and it is neither
    // approximate nor before: `task_count` is the run set's own expansion, and
    // that expansion is what adds analysis families and their prerequisites.
    // Queueing then re-derives the identical quantity from each member's
    // prepared snapshot and reports it on the receipt, which is the count the
    // success message carries.
    if scope.unforecast == 0 {
        property_row(
            ui,
            "Authenticated tasks",
            &format!("{} · after dependency expansion", scope.tasks),
        );
    } else {
        property_row_status(
            ui,
            "Authenticated tasks",
            &format!(
                "{} of {} {RUN_SET_DOES_NOT_VALIDATE}",
                scope.unforecast, scope.members
            ),
            t.color.err,
            StatusMark::Failure,
        );
    }
    // Admission is all or nothing at preparation: the controller activates and
    // prepares each member with `?`, so one member it cannot prepare refuses
    // the campaign before anything is dispatched.
    if let Some(reason) = refusal {
        property_row_status(ui, "Admission", reason, t.color.err, StatusMark::Failure);
    } else {
        property_row(
            ui,
            "Admission",
            &format!(
                "{MIN_CAMPAIGN_MEMBERS} to {MAX_CAMPAIGN_MEMBERS} plans, each preparable"
            ),
        );
    }
}

/// What queueing the campaign does, stated from the dispatch path rather than
/// from the authored reference's claims about it.
fn dispatch_guarantee_group(ui: &mut Ui) {
    kit::section_head(ui, "What queueing does", None);
    // Every member is prepared against one clone of the project taken before
    // the first one starts, so a later member never sees an edit made while an
    // earlier member is running.
    property_row(ui, "Freeze", "all members frozen up front");
    property_row(ui, "Dispatch", "one at a time, in order shown");
    // After dispatch a member that fails is counted and the next is dispatched;
    // the campaign closes as completed with errors rather than stopping.
    property_row(ui, "After a failure", "later members still run");
    property_row(ui, "Evidence", "own run, receipt and dataset");
}

/// The catalog as members: which plans are in, in what position, and what each
/// one declares.
fn member_table(
    ui: &mut Ui,
    member_ids: &mut Vec<SimulationPlanId>,
    eligible: &[&PlanCatalogRecord],
    has_archived: bool,
) {
    kit::section_head(
        ui,
        "Member plans",
        // Stated only when the catalog actually holds one. A rule about a set
        // that is empty is a sentence the reader has to rule out.
        has_archived.then_some(HeadStatus {
            label: "archived plans are not eligible",
            tone: LifecycleTone::Archived,
        }),
    );
    let positions = eligible
        .iter()
        .map(|record| member_position(member_ids, record.id))
        .collect::<Vec<_>>();
    let rows = eligible
        .iter()
        .zip(&positions)
        .map(|(record, position)| TableRow {
            selected: position.is_some(),
            announced: announced_member_row(record, *position, member_ids.len()),
        })
        .collect::<Vec<_>>();
    let clicked = kit::records_table(
        ui,
        "simulation.plan-manager.campaign.members",
        &MEMBER_COLUMNS,
        &rows,
        |ui, row, column| {
            let record = eligible[row];
            let t = Tokens::get(ui.ctx());
            // A member's own numbers are stated in the text tone; a plan that
            // is not a member is still readable but is not what the campaign
            // is about, so its row recedes.
            let tone = if positions[row].is_some() {
                t.color.text
            } else {
                t.color.text_dim
            };
            match column {
                0 => kit::cell_value(
                    ui,
                    &positions[row].map_or_else(
                        || NO_FORECAST.to_owned(),
                        |position| (position + 1).to_string(),
                    ),
                    if positions[row].is_some() {
                        t.color.accent
                    } else {
                        t.color.text_faint
                    },
                ),
                1 => kit::cell_identity(ui, &record.name, &record.id.to_string()),
                2 => kit::cell_value(
                    ui,
                    &format!("{} / {}", record.enabled, record.analyses),
                    tone,
                ),
                // A run set that does not validate is toned as the error it is,
                // and the three columns derived from it go absent rather than
                // printing a zero none of them measured.
                3 => kit::cell_value(
                    ui,
                    &record.run_set_label(),
                    if record.point_count().is_some() {
                        tone
                    } else {
                        t.color.err
                    },
                ),
                4 => kit::cell_value(
                    ui,
                    &record
                        .task_count()
                        .map_or_else(|| NO_FORECAST.to_owned(), |tasks| tasks.to_string()),
                    tone,
                ),
                5 => kit::cell_value(
                    ui,
                    &record
                        .estimated_duration()
                        .unwrap_or_else(|| NO_FORECAST.to_owned()),
                    tone,
                ),
                _ => kit::cell_value(
                    ui,
                    &record
                        .estimated_storage()
                        .unwrap_or_else(|| NO_FORECAST.to_owned()),
                    tone,
                ),
            }
        },
    );
    if let Some(row) = clicked {
        toggle_member(member_ids, eligible[row].id);
    }
}

/// Where `id` sits in the declared order, if it is a member at all.
fn member_position(member_ids: &[SimulationPlanId], id: SimulationPlanId) -> Option<usize> {
    member_ids.iter().position(|member| *member == id)
}

/// Add a plan at the end of the declared order, or take it out.
///
/// Appending is what makes the `Order` column the reader's: the position a
/// member is shown at is the position `prepare_and_start_campaign` will
/// dispatch it in. Removing closes the gap and leaves the surviving members in
/// the order they were already in, so taking one out never reorders the rest.
fn toggle_member(member_ids: &mut Vec<SimulationPlanId>, id: SimulationPlanId) {
    if let Some(index) = member_position(member_ids, id) {
        member_ids.remove(index);
    } else {
        member_ids.push(id);
    }
}

/// Every fact the row paints, in one accessibility node.
///
/// The row is the interactive unit and the interaction is what makes a plan a
/// member, so the node states both the membership and the position — the two
/// things the `Order` cell paints as one character.
fn announced_member_row(
    record: &PlanCatalogRecord,
    position: Option<usize>,
    members: usize,
) -> String {
    let membership = position.map_or_else(
        || "not a member · click to add".to_owned(),
        |index| format!("member {} of {members} · click to remove", index + 1),
    );
    let scale = match (
        record.task_count(),
        record.estimated_duration(),
        record.estimated_storage(),
    ) {
        (Some(tasks), Some(duration), Some(storage)) => format!(
            "run set {} · {tasks} task{} · {duration} · {storage}",
            record.run_set_label(),
            plan_plural_suffix(tasks)
        ),
        _ => format!("declared run set {RUN_SET_DOES_NOT_VALIDATE}"),
    };
    format!(
        "{} · {} · {membership} · {} of {} analyses enabled · {scale}",
        record.name, record.id, record.enabled, record.analyses
    )
}

/// Sum the members' declared scope from the projection's own per-plan numbers.
///
/// The members are counted distinctly, because `prepare_and_start_campaign`
/// deduplicates before it applies either limit, and a count that disagreed with
/// the one the limits are applied to would state a campaign the controller does
/// not see.
fn campaign_scope(member_ids: &[SimulationPlanId], records: &[PlanCatalogRecord]) -> CampaignScope {
    let mut seen = HashSet::with_capacity(member_ids.len());
    let mut scope = CampaignScope {
        members: 0,
        eligible: records.iter().filter(|record| !record.archived).count(),
        tasks: 0,
        unforecast: 0,
    };
    for id in member_ids.iter().filter(|id| seen.insert(**id)) {
        scope.members += 1;
        match records
            .iter()
            .find(|record| record.id == *id)
            .and_then(PlanCatalogRecord::task_count)
        {
            Some(tasks) => scope.tasks = scope.tasks.saturating_add(tasks),
            None => scope.unforecast += 1,
        }
    }
    scope
}

/// The refusal `prepare_and_start_campaign` would give this draft, checked in
/// the order it checks them, or `None` when nothing in the draft blocks it.
///
/// It cannot see the one refusal that is not the draft's — a run or campaign
/// already active — which [`commit_simulation_campaign`] reports when it
/// happens. Everything here is a property of what the reader has typed and
/// picked, so it is answerable before the button is pressed rather than after.
fn declared_refusal(name: &str, member_ids: &[SimulationPlanId]) -> Option<String> {
    if name.trim().is_empty() {
        return Some("a campaign name is required".to_owned());
    }
    if name.trim().chars().count() > MAX_CAMPAIGN_NAME_CHARS {
        return Some(format!("name over {MAX_CAMPAIGN_NAME_CHARS} characters"));
    }
    let distinct = member_ids.iter().collect::<HashSet<_>>().len();
    if distinct < MIN_CAMPAIGN_MEMBERS {
        return Some(format!("at least {MIN_CAMPAIGN_MEMBERS} distinct plans"));
    }
    if distinct > MAX_CAMPAIGN_MEMBERS {
        return Some(format!("at most {MAX_CAMPAIGN_MEMBERS} plans"));
    }
    None
}

pub(super) fn commit_simulation_campaign(
    app: &mut RSpiceApp,
    name: &str,
    member_ids: &[SimulationPlanId],
) -> Result<String, String> {
    if app.state.simulation.has_active_execution() || app.state.simulation.trigger_simulation {
        return Err("A simulation is already running or waiting to start".to_owned());
    }
    app.state.sync_active_schematic_to_workspace();
    crate::workbench::menu_bar::run_design_rule_check(&mut app.state);
    let receipt =
        app.simulation_controller
            .prepare_and_start_campaign(&mut app.state, name, member_ids)?;
    Ok(format!(
        "Campaign {} queued {} plans as {} authenticated tasks.",
        receipt.campaign_id, receipt.member_count, receipt.task_count
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::run_set::RunSetCompositionMode;

    /// Three root plans in one catalog: the active one the shell projects
    /// first, and two retained beside it. Returns them in projection order.
    fn app_with_three_plans() -> (RSpiceApp, [SimulationPlanId; 3]) {
        let mut app = RSpiceApp::test_instance();
        let root = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the fixture has a stable plan")
            .id();
        let mut setup = app.state.sim_setup.clone();
        let second = setup
            .create_plan("Corner characterization")
            .expect("a fresh root plan is created");
        let third = setup
            .create_plan("Startup and recovery")
            .expect("a second fresh root plan is created");
        app.state.workspace.migrate_active_plan_data(root);
        app.state.workspace.migrate_inactive_plan_data(second);
        app.state.workspace.migrate_inactive_plan_data(third);
        app.state.sim_setup = setup;
        // `create_plan` activates what it creates, so the last one made leads
        // the projection and the other two are the retained entries.
        (app, [third, root, second])
    }

    fn record_of(records: &[PlanCatalogRecord], id: SimulationPlanId) -> &PlanCatalogRecord {
        records
            .iter()
            .find(|record| record.id == id)
            .expect("the fixture's plan is projected")
    }

    /// Every limit this surface prints, put to the function that enforces it.
    ///
    /// The controller holds all three privately, so the constants here are a
    /// copy and this is what stops the copy drifting: each refusal message has
    /// to still carry the number the dialog shows.
    #[test]
    fn the_limits_this_route_prints_are_the_ones_the_controller_enforces() {
        let (mut app, plans) = app_with_three_plans();

        let one = commit_simulation_campaign(&mut app, "Nightly", &plans[..1])
            .expect_err("one plan is not a campaign");
        assert!(
            one.contains("at least two distinct plans"),
            "below the floor the controller said: {one}"
        );
        // Distinctness, not length: the controller deduplicates before it
        // counts, so the same plan twice is still one member.
        let repeated = commit_simulation_campaign(&mut app, "Nightly", &[plans[0], plans[0]])
            .expect_err("the same plan twice is one distinct member");
        assert!(repeated.contains("at least two distinct plans"));

        let over = (0..=MAX_CAMPAIGN_MEMBERS)
            .map(|_| SimulationPlanId::new())
            .collect::<Vec<_>>();
        assert_eq!(over.len(), MAX_CAMPAIGN_MEMBERS + 1);
        let too_many = commit_simulation_campaign(&mut app, "Nightly", &over)
            .expect_err("one over the cap is refused");
        assert!(
            too_many.contains(&MAX_CAMPAIGN_MEMBERS.to_string()),
            "the member cap moved away from {MAX_CAMPAIGN_MEMBERS}: {too_many}"
        );

        let unnamed = commit_simulation_campaign(&mut app, "   ", &plans[..2])
            .expect_err("a blank name is refused");
        assert!(unnamed.contains("must not be empty"), "{unnamed}");

        let long = "n".repeat(MAX_CAMPAIGN_NAME_CHARS + 1);
        let too_long = commit_simulation_campaign(&mut app, &long, &plans[..2])
            .expect_err("a name past the cap is refused");
        assert!(
            too_long.contains(&MAX_CAMPAIGN_NAME_CHARS.to_string()),
            "the name cap moved away from {MAX_CAMPAIGN_NAME_CHARS}: {too_long}"
        );
    }

    /// The refusal the surface paints is the one the commit would give, and it
    /// is checked in the same order.
    ///
    /// A surface that refused on a different rule than the transaction would
    /// disable its own button over something the controller does not mind, or
    /// enable it into a rejection.
    #[test]
    fn the_painted_refusal_agrees_with_the_transaction() {
        let (mut app, plans) = app_with_three_plans();

        assert_eq!(declared_refusal("Nightly", &plans[..2]), None);
        // Name before members, exactly as the controller checks them: a draft
        // wrong in both ways names the name.
        assert_eq!(
            declared_refusal("", &plans[..1]).as_deref(),
            Some("a campaign name is required")
        );
        assert_eq!(
            declared_refusal("Nightly", &plans[..1]).as_deref(),
            Some("at least 2 distinct plans")
        );
        assert_eq!(
            declared_refusal(&"n".repeat(MAX_CAMPAIGN_NAME_CHARS + 1), &plans[..2]).as_deref(),
            Some("name over 160 characters")
        );
        let over = (0..=MAX_CAMPAIGN_MEMBERS)
            .map(|_| SimulationPlanId::new())
            .collect::<Vec<_>>();
        assert_eq!(
            declared_refusal("Nightly", &over).as_deref(),
            Some("at most 64 plans")
        );

        // And where it says nothing blocks, the transaction gets past every
        // draft-level check: what stops this fixture is the design itself.
        let error = commit_simulation_campaign(&mut app, "Nightly", &plans[..2])
            .expect_err("the fixture's empty design cannot be prepared");
        for draft_refusal in [
            "at least two distinct plans",
            "must not be empty",
            "must not exceed",
            "at most 64",
        ] {
            assert!(
                !error.contains(draft_refusal),
                "a draft the surface admitted was refused on a draft rule: {error}"
            );
        }
    }

    /// The order the table shows is the order the commit walks.
    ///
    /// Two archived plans make that observable without an engine: the commit
    /// activates each member in declared order and refuses an archived one on
    /// sight, so the member it names is the one the table numbered first.
    #[test]
    fn the_commit_walks_the_members_in_the_declared_order() {
        let (mut app, plans) = app_with_three_plans();
        let mut setup = app.state.sim_setup.clone();
        setup.archive_plan(plans[1]).expect("an inactive plan archives");
        setup.archive_plan(plans[2]).expect("an inactive plan archives");
        app.state.sim_setup = setup;
        let records = plan_catalog_records(&app);
        let second = record_of(&records, plans[1]).name.clone();
        let third = record_of(&records, plans[2]).name.clone();
        assert_ne!(second, third);

        let forward = commit_simulation_campaign(&mut app, "Nightly", &[plans[1], plans[2]])
            .expect_err("an archived member is refused");
        let reverse = commit_simulation_campaign(&mut app, "Nightly", &[plans[2], plans[1]])
            .expect_err("an archived member is refused");

        assert!(
            forward.contains(&second) && !forward.contains(&third),
            "declared {second} first and the commit stopped at: {forward}"
        );
        assert!(
            reverse.contains(&third) && !reverse.contains(&second),
            "declared {third} first and the commit stopped at: {reverse}"
        );
    }

    /// Clicking a plan puts it last, which is what makes the `Order` column a
    /// statement about dispatch rather than about the catalog.
    #[test]
    fn a_member_is_added_at_the_end_and_removing_one_keeps_the_rest_in_order() {
        let (_, plans) = app_with_three_plans();
        let mut members = Vec::new();
        for id in plans {
            toggle_member(&mut members, id);
        }
        assert_eq!(members, plans.to_vec());
        assert_eq!(member_position(&members, plans[2]), Some(2));

        toggle_member(&mut members, plans[0]);
        assert_eq!(members, vec![plans[1], plans[2]]);
        assert_eq!(member_position(&members, plans[1]), Some(0));
        assert_eq!(member_position(&members, plans[0]), None);

        toggle_member(&mut members, plans[0]);
        assert_eq!(
            members,
            vec![plans[1], plans[2], plans[0]],
            "a re-added member goes to the end, not back to where it was"
        );
    }

    /// The combined scope is the sum of the per-member forecasts the
    /// projection reports, over the distinct members the controller would
    /// count.
    #[test]
    fn the_combined_scope_is_the_sum_of_the_per_member_forecasts() {
        let (app, plans) = app_with_three_plans();
        let records = plan_catalog_records(&app);
        let members = [plans[0], plans[2]];
        let expected = members
            .iter()
            .map(|id| {
                record_of(&records, *id)
                    .task_count()
                    .expect("the fixture's run sets validate")
            })
            .sum::<usize>();

        let scope = campaign_scope(&members, &records);
        assert!(expected > 0, "the fixture declares work to sum");
        assert_eq!(scope.tasks, expected);
        assert_eq!(scope.members, 2);
        assert_eq!(scope.eligible, 3);
        assert_eq!(scope.unforecast, 0);

        // Deduplicated the way the transaction deduplicates, so the count the
        // surface states is the count the limits are applied to.
        let repeated = campaign_scope(&[plans[0], plans[2], plans[0]], &records);
        assert_eq!(repeated.members, 2);
        assert_eq!(repeated.tasks, expected);
    }

    /// A member whose run set does not validate forecasts nothing, and the
    /// scope says so instead of folding a zero into the total.
    #[test]
    fn a_member_that_does_not_validate_is_counted_and_not_summed_as_zero() {
        let (mut app, plans) = app_with_three_plans();
        let mut setup = app.state.sim_setup.clone();
        // A nested composition with a zero maximum depth is a declared
        // run-space error, so the active plan forecasts nothing.
        setup.run_set.composition.mode = RunSetCompositionMode::Nested;
        setup.run_set.composition.maximum_depth = 0;
        app.state.sim_setup = setup;
        let records = plan_catalog_records(&app);
        assert_eq!(record_of(&records, plans[0]).task_count(), None);

        let scope = campaign_scope(&plans[..2], &records);
        assert_eq!(scope.members, 2);
        assert_eq!(scope.unforecast, 1);
        assert_eq!(
            scope.tasks,
            record_of(&records, plans[1])
                .task_count()
                .expect("the retained plan still forecasts"),
            "an absent forecast contributes nothing rather than a zero that \
             reads as 'declares no work'"
        );
        assert_eq!(
            announced_member_row(record_of(&records, plans[0]), Some(0), 2),
            format!(
                "{} · {} · member 1 of 2 · click to remove · {} of {} analyses enabled · declared run set {RUN_SET_DOES_NOT_VALIDATE}",
                record_of(&records, plans[0]).name,
                plans[0],
                record_of(&records, plans[0]).enabled,
                record_of(&records, plans[0]).analyses
            )
        );
    }

    /// Every string the route paints at one viewport, in paint order.
    #[cfg(not(target_arch = "wasm32"))]
    fn painted_route_text(
        screen: egui::Vec2,
        draft: &mut SimulationPlanManagerDraft,
        records: &[PlanCatalogRecord],
    ) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let pass = |draft: &mut SimulationPlanManagerDraft| {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, screen)),
                    ..Default::default()
                },
                |ctx| {
                    dialog(ctx, draft, records);
                },
            )
        };
        let _ = pass(draft);
        let _ = pass(draft);
        pass(draft)
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::epaint::Shape::Text(text) => Some(text.galley.job.text.clone()),
                _ => None,
            })
            .collect()
    }

    /// The scope is painted before the members it sums, and it is the sum.
    ///
    /// Order is the point. The scope states how much work this queues, which
    /// is the one thing the confirmation exists for, and under an unbounded
    /// member table it was the first thing a short viewport took away.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_scope_is_painted_above_the_members_and_states_their_sum() {
        let (app, plans) = app_with_three_plans();
        let records = plan_catalog_records(&app);
        let mut draft = SimulationPlanManagerDraft::new(plans[0], records[0].name.clone());
        draft.mode = SimulationPlanManagerMode::Campaign;
        draft.campaign.member_ids = vec![plans[0], plans[1]];
        let expected = campaign_scope(&draft.campaign.member_ids, &records).tasks;

        let painted = painted_route_text(egui::Vec2::new(1024.0, 640.0), &mut draft, &records);
        let index = |needle: &str| {
            painted
                .iter()
                .position(|text| text.contains(needle))
                .unwrap_or_else(|| panic!("the route never painted {needle:?}: {painted:?}"))
        };

        assert!(
            index("Combined declared scope") < index("Member plans"),
            "the members were painted before the scope that sums them"
        );
        assert!(
            painted
                .iter()
                .any(|text| text == &format!("{expected} · after dependency expansion")),
            "the scope did not state the summed task count {expected}: {painted:?}"
        );
        // The caps are on screen before anything is rejected.
        assert!(
            painted
                .iter()
                .any(|text| text.contains(&format!("at most {MAX_CAMPAIGN_MEMBERS}"))),
            "the member cap is not stated: {painted:?}"
        );
        assert!(
            painted
                .iter()
                .any(|text| text.contains(&format!("/ {MAX_CAMPAIGN_NAME_CHARS}"))),
            "the name cap is not stated: {painted:?}"
        );
    }

    /// Every fact the two note groups state is painted whole at all three
    /// gated shapes.
    ///
    /// The fit gate next door proves nothing is off screen, which is a
    /// different question from whether it is readable: a property row elides
    /// its value to one line, so a note that outgrows its track is silently
    /// truncated on screen and the gate still passes. These are the exact
    /// strings, so the arrangement has to keep every one of them intact — which
    /// is what the two-column breakpoint above is for.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_fact_the_notes_state_is_painted_whole_at_every_gated_viewport() {
        let (app, plans) = app_with_three_plans();
        let records = plan_catalog_records(&app);
        let members = vec![plans[0], plans[1]];
        let scope = campaign_scope(&members, &records);
        let owed = [
            format!("2 of 3 eligible · at most {MAX_CAMPAIGN_MEMBERS}"),
            format!("{} · after dependency expansion", scope.tasks),
            format!("{MIN_CAMPAIGN_MEMBERS} to {MAX_CAMPAIGN_MEMBERS} plans, each preparable"),
            "all members frozen up front".to_owned(),
            "one at a time, in order shown".to_owned(),
            "later members still run".to_owned(),
            "own run, receipt and dataset".to_owned(),
            // The footer hint carries the one rule the surface cannot show,
            // so it has to survive the narrow footer too.
            "Click a plan to add or remove it · a plan added last dispatches last".to_owned(),
        ];

        // The three shapes the route's fit gate uses, for the reasons it
        // states: the real viewport, the width where the dialog goes
        // edge-to-edge, and the portrait shape a global command can open it in.
        for screen in [
            egui::Vec2::new(1024.0, 640.0),
            egui::Vec2::new(820.0, 640.0),
            egui::Vec2::new(560.0, 900.0),
        ] {
            let mut draft = SimulationPlanManagerDraft::new(plans[0], records[0].name.clone());
            draft.mode = SimulationPlanManagerMode::Campaign;
            draft.campaign.member_ids.clone_from(&members);
            let painted = painted_route_text(screen, &mut draft, &records);
            for fact in &owed {
                assert!(
                    painted.contains(fact),
                    "{screen:?} did not paint {fact:?} whole; it painted {painted:?}"
                );
            }
        }
    }
}
