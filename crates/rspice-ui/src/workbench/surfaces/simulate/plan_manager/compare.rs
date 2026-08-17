//! Comparing one plan in the catalog against another.
//!
//! Which two is the route's own selection, resolved by [`compared_plans`] and
//! chosen by the two pickers at the top of the surface. Until a side is picked
//! it is the active plan against the selected row, which is the only pair this
//! surface could name before either side was choosable.
//!
//! One of the five routes the plan-manager shell dispatches to, and the only one
//! with nothing to commit: it states a difference and closes. Nothing in this
//! file is handed the application, and nothing in it writes a plan — a
//! comparison is a reading of two plans, so both are left exactly as they were.
//!
//! Every quantity is read from [`PlanCatalogRecord`], so a count shown here is
//! the same count the records table shows rather than a second derivation free
//! to disagree with it. The names behind those counts come from the same place:
//! each roster is collected beside the count it belongs to, in one pass over one
//! owner, which is what lets this surface say *which* entries differ rather than
//! only how many.
//!
//! Four domains, not the authored five. The authored table's fifth row is
//! *Execution binding and target*, and neither half of it exists here: no plan
//! binds a design or a testbench, and no plan and no campaign owns an execution
//! target — `SimulationRun::execution_target` is a property of a run, chosen
//! when that run is dispatched. Painting the row as unchanged would assert a
//! comparison that never ran, so the row is dropped and its absence is stated
//! as the fact about RSpice that it is.

use super::records::PlanEntry;
use super::*;

use crate::workbench::design_system::elide_text;

/// The comparison table, left to right — exactly the authored column set.
///
/// Only the domain column is elastic. Every other column's longest value is
/// known — three counts and one short verdict — so they hold their width and
/// stay readable by position down the four rows while the dialog resizes.
///
/// The three count columns are as narrow as their own headings, and that is
/// what makes the rest fit. The narrowest viewport this route is gated at hands
/// the body 536 points, so every point a count column does not need is a point
/// the domain column has — and the domain column is the one that has to hold a
/// domain name over a comparison key without eliding either. A count is at most
/// three digits; a domain name cannot be guessed from its first two words.
///
/// The set totals 526 points against that 536-point body, so the arrangement
/// has ten points of margin at its narrowest. Widening any column here spends
/// that margin, and past it the table overflows a dialog that may not scroll.
const COMPARE_COLUMNS: [TableColumn; 5] = [
    TableColumn {
        heading: "Domain",
        track: ColumnTrack::Elastic(196.0),
    },
    TableColumn {
        heading: "Added",
        track: ColumnTrack::Fixed(44.0),
    },
    TableColumn {
        heading: "Removed",
        track: ColumnTrack::Fixed(52.0),
    },
    TableColumn {
        heading: "Changed",
        track: ColumnTrack::Fixed(52.0),
    },
    // Wide enough for the longest verdict below plus a four-digit point count
    // on each side of the arrow. A catalog whose plans declare five-digit point
    // counts would elide it, and that is the one value here whose length is the
    // project's rather than this file's.
    TableColumn {
        heading: "Compatibility",
        track: ColumnTrack::Fixed(136.0),
    },
];

/// The width the word between the two pickers and the row's own spacing cost.
const PICKER_GAP: f32 = 76.0;
/// Narrowest and widest a picker is drawn at. Below the floor a plan name is
/// elided to nothing; above the ceiling two pickers stop reading as one row.
const PICKER_MIN_WIDTH: f32 = 148.0;
const PICKER_MAX_WIDTH: f32 = 300.0;
/// What a select spends on its own inset and chevron before its value.
///
/// The control clips its value rather than eliding it, so a label longer than
/// this leaves the clip rect — which is the same defect as a label the reader
/// has to scroll to. The value is elided to what is left instead.
const PICKER_TEXT_INSET: f32 = 34.0;

/// The most names one domain lists before the rest are counted.
///
/// A dialog that may not scroll cannot promise the whole list, and a promise it
/// keeps only for short lists is worse than a stated count: `and 4 more` says
/// exactly how much is not shown. The full list is in the row's accessibility
/// node, which has no width to run out of.
const NAMED_LIMIT: usize = 3;

/// Whitespace closing the body, for the reason `kit`'s `SPLIT_BOTTOM_INSET`
/// documents: this dialog's body viewport resolves a couple of points under the
/// content it was measured from, so whatever is painted last is cut mid-glyph.
/// The inset spends that shortfall on nothing and costs no screen, because it
/// falls outside the viewport that cut it.
const BODY_BOTTOM_INSET: f32 = 6.0;

/// The changed entry a moved regression baseline contributes.
///
/// The pinned baseline is not a specification, so it is not in that roster; it
/// is a fact of the same domain the authored table calls *Specifications and
/// baselines*, so it is compared as one more named entry of it.
const PINNED_BASELINE: &str = "pinned baseline";

/// How a domain's two declarations stand to each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Compatibility {
    /// Both plans declare the same entries, and anything derived from them
    /// agrees.
    Same,
    /// They declare different entries, or the same entries differently.
    Differs,
    /// One side declares something the other cannot be compared against. Only
    /// the run set can reach this: a declaration that does not validate
    /// predicts no workload, so there is no workload to hold the other's
    /// against.
    Unavailable,
}

impl Compatibility {
    fn tone(self, t: &Tokens) -> Color32 {
        match self {
            Self::Same => t.color.ok,
            Self::Differs => t.color.warn,
            Self::Unavailable => t.color.err,
        }
    }
}

/// A set difference over one domain's roster, by name.
struct Difference {
    added: Vec<String>,
    removed: Vec<String>,
    changed: Vec<String>,
}

impl Difference {
    fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }
}

/// One row of the comparison: what the two plans declare in one domain.
struct ComparedDomain {
    /// The authored domain name.
    domain: &'static str,
    /// What one entry is, and what makes it changed rather than merely present.
    ///
    /// It is painted under the domain name because a difference is only as
    /// precise as the key it was taken over: "changed 1" means nothing until
    /// the reader knows that a variable is compared by its name and its
    /// expression, and that a quantity moving under an unchanged expression is
    /// therefore not what this row calls changed.
    key: &'static str,
    difference: Difference,
    /// The short verdict painted in the compatibility column.
    verdict: String,
    compatibility: Compatibility,
}

/// The route's dialog. See the shell's child-dialog signature contract.
pub(super) fn dialog(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    let (base, target) = compared_plans(draft, records);
    let domains = match (base, target) {
        (Some(base), Some(target)) => Some((base, target, compared_domains(base, target))),
        _ => None,
    };
    let mut action = None;
    let choice = Dialog::new(
        "SIMULATION · SEMANTIC PLAN DIFF · READ ONLY",
        "Compare simulation plans",
        "Close comparison",
    )
    .description(
        "Both sides are chosen. Each domain states what the compared plan declares that the base plan does not, and nothing here is committed to either plan.",
    )
    // The one rule the surface cannot show. Which side is which is legible from
    // the row — base, then the word, then the compared plan — but the direction
    // the three count columns are stated in is not, and reading them backwards
    // inverts every added and removed on the surface.
    .hint("Differences are stated from the base plan to the compared plan")
    .size(DialogSize::SimulationWorkflow)
    .show(ctx, |ui| {
        let t = Tokens::get(ui.ctx());
        match &domains {
            Some((base, target, domains)) => {
                plan_picker_row(ui, draft, records, base, target);
                ui.add_space(8.0);
                domain_table(ui, domains);
                ui.add_space(8.0);
                named_differences(ui, domains);
                ui.add_space(8.0);
                kit::note_grid(ui, &COMPARISON_BOUNDARY_NOTES);
            }
            // The catalog can lose a plan between the frame that picked it and
            // this one, and both sides resolve through the projection for
            // exactly that reason. Reaching here means it lost both.
            None => {
                property_row_status(
                    ui,
                    "Comparison",
                    "the catalog carries neither side of this comparison",
                    t.color.err,
                    StatusMark::Failure,
                );
            }
        }
        ui.add_space(BODY_BOTTOM_INSET);
    });
    match choice {
        DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

/// What a comparison does, and what it has nothing to say about.
///
/// The second note is the authored fifth domain, stated as the fact that
/// removed it. Left out entirely, a reader who knows the authored table would
/// read its absence as an omission; painted as a row of zeroes it would assert a
/// comparison that never ran. Saying there is nothing to compare is the only one
/// of the three that is true.
const COMPARISON_BOUNDARY_NOTES: [(&str, &str); 2] = [
    (
        "Read-only comparison",
        "This surface performs no transaction. It reads the same catalog \
         projection the plan table reads and writes nothing back, so both plans \
         are left exactly as they were and closing it changes neither. Applying \
         a difference means editing the plan that should carry it, after \
         opening that plan.",
    ),
    (
        "Not compared",
        "A simulation plan binds no design and no testbench, and neither a plan \
         nor a campaign owns an execution target — a target belongs to a run \
         and is chosen when that run is dispatched. There is nothing in those \
         domains for a plan comparison to differ about.",
    ),
];

/// The two plans, each pickable, in the order the counts are stated in.
///
/// Both lists offer every plan in the catalog, including the one already on the
/// other side. Comparing a plan with itself is the state this route opens in
/// whenever the selected row is the active plan, so a list that excluded it
/// would be unable to express the pair the surface is already showing.
fn plan_picker_row(
    ui: &mut Ui,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
    base: &PlanCatalogRecord,
    target: &PlanCatalogRecord,
) {
    let width = ((ui.available_width() - PICKER_GAP) / 2.0).clamp(PICKER_MIN_WIDTH, PICKER_MAX_WIDTH);
    ui.horizontal(|ui| {
        if let Some(picked) = plan_picker(
            ui,
            "simulation.plan-manager.compare.base",
            "Base simulation plan",
            width,
            base,
            records,
        ) {
            draft.comparison.base_plan_id = Some(picked);
        }
        ui.label("against");
        if let Some(picked) = plan_picker(
            ui,
            "simulation.plan-manager.compare.target",
            "Compared simulation plan",
            width,
            target,
            records,
        ) {
            draft.comparison.target_plan_id = Some(picked);
        }
    });
}

/// One side's picker. Returns the plan picked this frame.
fn plan_picker(
    ui: &mut Ui,
    id_salt: &str,
    accessible_label: &str,
    width: f32,
    current: &PlanCatalogRecord,
    records: &[PlanCatalogRecord],
) -> Option<SimulationPlanId> {
    let options = records.iter().map(plan_option_label).collect::<Vec<_>>();
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let shown = elide_text(
        ui,
        &plan_option_label(current),
        &font,
        width - PICKER_TEXT_INSET,
    );
    select(ui, id_salt, accessible_label, &shown, &options, width)
        .and_then(|index| records.get(index))
        .map(|record| record.id)
}

/// A plan as one side of a comparison: its name and the revision this
/// comparison was taken at.
///
/// The revision is not decoration. Two plans can carry one name — a clone keeps
/// the source's name until it is renamed — and a comparison between two plans
/// the reader cannot tell apart states nothing.
fn plan_option_label(record: &PlanCatalogRecord) -> String {
    format!("{} · revision {}", record.name, record.revision)
}

/// The four domains, and what the two plans declare differently in each.
fn domain_table(ui: &mut Ui, domains: &[ComparedDomain]) {
    let rows = domains
        .iter()
        .map(|domain| TableRow {
            selected: false,
            announced: announced_domain_row(domain),
        })
        .collect::<Vec<_>>();
    // The rows are a statement rather than a control, so the table is asked for
    // the read-only treatment: no hover fill under a row that cannot be picked,
    // and no row announced as selectable. The row still carries the whole fact
    // as one accessibility node, for the reason the browse table states: a cell
    // is elided to its column and the counts alone name no domain.
    kit::read_only_records_table(
        ui,
        "simulation.plan-manager.compare.domains",
        &COMPARE_COLUMNS,
        &rows,
        |ui, row, column| {
            let domain = &domains[row];
            let t = Tokens::get(ui.ctx());
            match column {
                0 => kit::cell_identity(ui, domain.domain, domain.key),
                1 => count_cell(ui, &domain.difference.added, &t),
                2 => count_cell(ui, &domain.difference.removed, &t),
                3 => count_cell(ui, &domain.difference.changed, &t),
                _ => kit::cell_value(ui, &domain.verdict, domain.compatibility.tone(&t)),
            }
        },
    );
}

/// One count, receding when there is nothing to count.
///
/// A zero is the answer in most rows of most comparisons, and four rows of
/// equally-weighted zeroes hide the one number that is not one.
fn count_cell(ui: &mut Ui, names: &[String], t: &Tokens) {
    kit::cell_value(
        ui,
        &names.len().to_string(),
        if names.is_empty() {
            t.color.text_faint
        } else {
            t.color.text
        },
    );
}

/// The names behind the counts, for the domains that have any.
///
/// The table states how many entries differ; this states which. They are two
/// different questions and the answer to the second does not fit a fixed column,
/// so it is stated where it can wrap instead of being elided to a prefix.
fn named_differences(ui: &mut Ui, domains: &[ComparedDomain]) {
    kit::section_head(ui, "Named differences", None);
    let differing = domains
        .iter()
        .filter(|domain| !domain.difference.is_empty())
        .map(|domain| (domain.domain, named_difference(&domain.difference)))
        .collect::<Vec<_>>();
    if differing.is_empty() {
        kit::note_grid(
            ui,
            &[(
                "No named differences",
                "Every entry the base plan declares, the compared plan declares \
                 under the same name and with the same declaration, in all four \
                 domains.",
            )],
        );
        return;
    }
    let notes = differing
        .iter()
        .map(|(domain, names)| (*domain, names.as_str()))
        .collect::<Vec<_>>();
    kit::note_grid(ui, &notes);
}

/// One domain's difference as a sentence: only the parts of it that exist.
///
/// A domain with two added entries and nothing else states two added entries.
/// Printing `removed none · changed none` beside them spends three lines saying
/// what the table's two zeroes already say.
fn named_difference(difference: &Difference) -> String {
    [
        ("added", &difference.added),
        ("removed", &difference.removed),
        ("changed", &difference.changed),
    ]
    .into_iter()
    .filter(|(_, names)| !names.is_empty())
    .map(|(word, names)| format!("{word} {}", named_list(names)))
    .collect::<Vec<_>>()
    .join(" · ")
}

/// Up to [`NAMED_LIMIT`] names, and an exact count of the ones not named.
fn named_list(names: &[String]) -> String {
    if names.len() <= NAMED_LIMIT {
        return names.join(", ");
    }
    format!(
        "{}, and {} more",
        names[..NAMED_LIMIT].join(", "),
        names.len() - NAMED_LIMIT
    )
}

/// Every fact the row paints, in one accessibility node, with no list capped.
fn announced_domain_row(domain: &ComparedDomain) -> String {
    let list = |names: &[String]| {
        if names.is_empty() {
            "none".to_owned()
        } else {
            format!("{} · {}", names.len(), names.join(", "))
        }
    };
    format!(
        "{} · compared by {} · added {} · removed {} · changed {} · {}",
        domain.domain,
        domain.key,
        list(&domain.difference.added),
        list(&domain.difference.removed),
        list(&domain.difference.changed),
        domain.verdict
    )
}

/// The four domains a RSpice plan can be compared in.
///
/// Each one's difference is a set difference over the roster the projection
/// collected beside the count it counts, so a domain's numbers and the browse
/// table's numbers are the same numbers. Nothing here reads a plan.
fn compared_domains(base: &PlanCatalogRecord, target: &PlanCatalogRecord) -> Vec<ComparedDomain> {
    let analyses = domain_difference(&base.analysis_roster, &target.analysis_roster);
    let variables = domain_difference(&base.variable_roster, &target.variable_roster);
    let run_set = domain_difference(&base.run_set_roster, &target.run_set_roster);
    let mut specifications =
        domain_difference(&base.specification_roster, &target.specification_roster);
    let requirements_differ = !specifications.is_empty();
    if base.regression_baseline != target.regression_baseline {
        specifications.changed.push(PINNED_BASELINE.to_owned());
    }

    let (run_set_verdict, run_set_compatibility) = run_set_verdict(base, target, &run_set);
    vec![
        // Shortened from the authored *Analysis instances and dependencies*,
        // which is 207 points wide in the domain column's own face and does not
        // paint whole in the 206 points the narrowest gated viewport leaves it.
        // A shorter true name beats an elided authored one, and the key line
        // under it states that the entries are instances.
        ComparedDomain {
            domain: "Analyses and dependencies",
            key: "kind · enabled · dependencies",
            verdict: verdict(&analyses, "same analyses", "analyses differ"),
            compatibility: compatibility(&analyses),
            difference: analyses,
        },
        ComparedDomain {
            domain: "Variables and expressions",
            key: "name · expression",
            verdict: verdict(&variables, "same expressions", "expressions differ"),
            compatibility: compatibility(&variables),
            difference: variables,
        },
        ComparedDomain {
            domain: "Run set and budgets",
            key: "axis values · declared limits",
            verdict: run_set_verdict,
            compatibility: run_set_compatibility,
            difference: run_set,
        },
        ComparedDomain {
            domain: "Specifications and baselines",
            key: "measurement · bound",
            // A moved baseline under an unchanged requirement set is its own
            // verdict: the two plans agree about what has to pass and disagree
            // about the run they are held against.
            verdict: if specifications.is_empty() {
                "same contract".to_owned()
            } else if requirements_differ {
                "contract differs".to_owned()
            } else {
                "baseline differs".to_owned()
            },
            compatibility: compatibility(&specifications),
            difference: specifications,
        },
    ]
}

fn verdict(difference: &Difference, same: &str, differs: &str) -> String {
    if difference.is_empty() {
        same.to_owned()
    } else {
        differs.to_owned()
    }
}

fn compatibility(difference: &Difference) -> Compatibility {
    if difference.is_empty() {
        Compatibility::Same
    } else {
        Compatibility::Differs
    }
}

/// What the two run spaces resolve to, which is the one thing in this domain a
/// roster cannot state.
///
/// Two plans can declare different axes and still resolve to the same number of
/// points, and the same axes composed differently resolve to different numbers.
/// So the verdict is the resolved point count and the tone is the declaration:
/// a row reading `same 27 points` in the differing tone is a run space that was
/// re-declared without changing how much work it is.
fn run_set_verdict(
    base: &PlanCatalogRecord,
    target: &PlanCatalogRecord,
    difference: &Difference,
) -> (String, Compatibility) {
    match (base.point_count(), target.point_count()) {
        (Some(from), Some(to)) if from == to => (
            format!("same {from} point{}", plan_plural_suffix(from)),
            compatibility(difference),
        ),
        (Some(from), Some(to)) => (
            format!("{from} → {to} point{}", plan_plural_suffix(to)),
            Compatibility::Differs,
        ),
        _ => (
            RUN_SET_DOES_NOT_VALIDATE.to_owned(),
            Compatibility::Unavailable,
        ),
    }
}

/// The set difference between two rosters, by name.
///
/// Names are matched one for one rather than by lookup, so a roster carrying the
/// same name twice is diffed as the two entries it is instead of collapsing to
/// one. An entry present on both sides whose declaration differs is *changed*,
/// which is the distinction that makes this a comparison rather than two lists.
fn domain_difference(base: &[PlanEntry], target: &[PlanEntry]) -> Difference {
    let mut matched = vec![false; base.len()];
    let mut difference = Difference {
        added: Vec::new(),
        removed: Vec::new(),
        changed: Vec::new(),
    };
    for entry in target {
        let found = (0..base.len()).find(|index| !matched[*index] && base[*index].name == entry.name);
        match found {
            Some(index) => {
                matched[index] = true;
                if base[index].detail != entry.detail {
                    difference.changed.push(entry.name.clone());
                }
            }
            None => difference.added.push(entry.name.clone()),
        }
    }
    difference.removed = base
        .iter()
        .zip(&matched)
        .filter(|(_, matched)| !**matched)
        .map(|(entry, _)| entry.name.clone())
        .collect();
    difference
}

/// The two plans this comparison diffs, left side first.
///
/// Each side comes from the route's own selection. An unpicked side falls back to
/// the plan this surface would have compared anyway — the active plan on the
/// base, the selected row on the target — so the pair a freshly opened manager
/// states is the one pair this route could state before either side was
/// choosable, and picking a side narrows it rather than emptying the surface.
///
/// A side naming a plan the projection no longer carries falls back the same
/// way, because the catalog can lose a plan between the frame that picked it and
/// this one. Resolving through `records` is what makes that a stale selection
/// rather than a blank comparison.
pub(super) fn compared_plans<'records>(
    draft: &SimulationPlanManagerDraft,
    records: &'records [PlanCatalogRecord],
) -> (
    Option<&'records PlanCatalogRecord>,
    Option<&'records PlanCatalogRecord>,
) {
    let by_id = |id: SimulationPlanId| records.iter().find(move |record| record.id == id);
    let base = draft
        .comparison
        .base_plan_id
        .and_then(by_id)
        .or_else(|| records.iter().find(|record| record.active));
    let target = draft
        .comparison
        .target_plan_id
        .and_then(by_id)
        .or_else(|| by_id(draft.selected_plan_id));
    (base, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::plan::AnalysisKind;
    use crate::simulation::run_set::{RunSetDimension, RunSetDimensionKind};

    /// The name a design variable is added under, and the requirement a
    /// specification is added under. Named here because several cases assert
    /// the exact string this surface reports back.
    const ADDED_VARIABLE: &str = "VDD_HI";
    const ADDED_REQUIREMENT: &str = "settling_time";
    const ADDED_AXIS: &str = "supply headroom";

    /// Two plans that start identical, so every difference a case asserts is
    /// the difference that case declared and not one the fixture came with.
    ///
    /// Returns the app, the active plan and the retained one. The retained plan
    /// is the fixture's original: `create_plan` activates what it creates.
    fn app_with_two_identical_plans() -> (RSpiceApp, SimulationPlanId, SimulationPlanId) {
        let mut app = RSpiceApp::test_instance();
        let retained = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the fixture has a stable plan")
            .id();
        let mut setup = app.state.sim_setup.clone();
        let active = setup
            .create_plan("Corner characterization")
            .expect("a fresh root plan is created");
        app.state.workspace.migrate_active_plan_data(retained);
        app.state.workspace.migrate_inactive_plan_data(active);
        app.state.sim_setup = setup;

        let records = plan_catalog_records(&app);
        let (base, target) = (record_of(&records, retained), record_of(&records, active));
        for domain in compared_domains(base, target) {
            assert!(
                domain.difference.is_empty(),
                "the two fixture plans differ in '{}' before any case has \
                 changed anything: {}",
                domain.domain,
                named_difference(&domain.difference)
            );
        }
        (app, active, retained)
    }

    fn record_of(records: &[PlanCatalogRecord], id: SimulationPlanId) -> &PlanCatalogRecord {
        records
            .iter()
            .find(|record| record.id == id)
            .expect("the fixture's plan is projected")
    }

    fn domain<'domains>(
        domains: &'domains [ComparedDomain],
        name: &str,
    ) -> &'domains ComparedDomain {
        domains
            .iter()
            .find(|domain| domain.domain == name)
            .unwrap_or_else(|| panic!("no '{name}' domain"))
    }

    /// Make the active plan differ from the retained one in all four domains,
    /// through each domain's own owner. Returns the axis whose declaration was
    /// changed, whose name is the run set's own rather than one chosen here.
    fn diverge_the_active_plan(app: &mut RSpiceApp, active: SimulationPlanId) -> String {
        let mut setup = app.state.sim_setup.clone();
        {
            let plan = setup
                .stable_analysis_plan_mut()
                .expect("the fixture's active plan is stable");
            plan.insert(AnalysisKind::Ac)
                .expect("an appended analysis has no prerequisites");
            let first = plan.instances()[0].id();
            plan.set_enabled(first, false)
                .expect("the first instance carries no enabled dependent");
        }
        // A new axis, declared the way the run set's own template declares a
        // parameter axis, and left out of the space so that adding it cannot
        // also change what the space resolves to.
        let revision = setup.run_set.revision;
        let mut added = RunSetDimension::new(
            "dimension-headroom",
            RunSetDimensionKind::Parameter,
            &["0.9", "1.1"],
            revision,
        );
        added.name = ADDED_AXIS.to_owned();
        added.source = format!("design-variable:{ADDED_VARIABLE}");
        added.enabled = false;
        setup.run_set.dimensions.push(added);
        // And one existing axis flipped in or out of the space, which changes
        // both its own declaration and the number of points the space resolves
        // to whichever way the fixture's template had it.
        let temperature = setup
            .run_set
            .dimensions
            .iter()
            .position(|dimension| dimension.kind == RunSetDimensionKind::Temperature)
            .expect("the fixture's run set declares a temperature axis");
        setup.run_set.dimensions[temperature].enabled =
            !setup.run_set.dimensions[temperature].enabled;
        let changed_axis = setup.run_set.dimensions[temperature].name.clone();
        setup.run_set.budgets.maximum_tasks = 4_096;
        app.state.sim_setup = setup;

        app.state
            .workspace
            .add_design_variable(
                active,
                crate::state::DesignVariable::new(
                    ADDED_VARIABLE,
                    "1.98 V",
                    crate::state::DesignVariableQuantity::Voltage,
                    crate::state::DesignVariableScope::Project,
                    "comparison fixture",
                    None,
                    crate::state::DesignVariableSweepEligibility::FixedParameter,
                    crate::state::DesignVariableOverridePolicy::InheritOwnerOnly,
                )
                .expect("a valid project variable"),
            )
            .expect("the active plan takes a variable");
        let payload = app.state.workspace.ensure_active_plan_data(active);
        payload.specs.push(crate::state::SpecEntry {
            measurement: ADDED_REQUIREMENT.to_owned(),
            expression: "trig v(in) val=0.9 targ v(out) val=1.8".to_owned(),
            min: None,
            max: Some(2.5e-9),
            unit: "s".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        });
        payload.regression_baseline_run = Some(crate::product::RunId::new());
        changed_axis
    }

    /// Every domain reports the difference its own owner carries, by name.
    ///
    /// This is the claim the surface exists for. A comparison that painted four
    /// plausible rows without deriving them would pass every fit gate in this
    /// module and be worth nothing, so each row is put to the two records it
    /// was taken from.
    #[test]
    fn every_domain_states_the_difference_its_owner_carries() {
        let (mut app, active, retained) = app_with_two_identical_plans();
        let changed_axis = diverge_the_active_plan(&mut app, active);
        let records = plan_catalog_records(&app);
        let base = record_of(&records, retained);
        let target = record_of(&records, active);
        let domains = compared_domains(base, target);
        assert_eq!(domains.len(), 4, "four domains, not the authored five");

        // One analysis added and one disabled: the disabled instance keeps its
        // name and changes its declaration, which is the distinction between a
        // changed entry and an added one.
        let analyses = domain(&domains, "Analyses and dependencies");
        assert!(
            !base
                .analysis_roster
                .iter()
                .any(|entry| entry.name == AnalysisKind::Ac.code()),
            "the fixture already declares an AC analysis, so 'added AC' would \
             not be this case's doing"
        );
        assert_eq!(analyses.difference.added, vec![AnalysisKind::Ac.code()]);
        assert!(analyses.difference.removed.is_empty());
        assert_eq!(
            analyses.difference.changed,
            vec![base.analysis_roster[0].name.clone()],
            "the disabled instance is the changed one"
        );
        assert_eq!(analyses.verdict, "analyses differ");
        assert_eq!(analyses.compatibility, Compatibility::Differs);

        let variables = domain(&domains, "Variables and expressions");
        assert_eq!(variables.difference.added, vec![ADDED_VARIABLE]);
        assert!(variables.difference.removed.is_empty());
        assert!(variables.difference.changed.is_empty());
        assert_eq!(variables.verdict, "expressions differ");

        // One axis added and one budget re-declared. Both are entries of this
        // domain because both are declared policy of the run space.
        let run_set = domain(&domains, "Run set and budgets");
        assert_eq!(run_set.difference.added, vec![ADDED_AXIS]);
        assert_eq!(
            run_set.difference.changed,
            vec![changed_axis, "maximum tasks".to_owned()],
            "an axis re-declared and a limit re-declared are both entries of \
             this domain"
        );
        assert!(run_set.difference.removed.is_empty());
        let (from, to) = (
            base.point_count().expect("the fixture's run set validates"),
            target.point_count().expect("the fixture's run set validates"),
        );
        assert_ne!(from, to, "the fixture did not move the declared space");
        assert_eq!(
            run_set.verdict,
            format!("{from} → {to} point{}", plan_plural_suffix(to)),
            "the resolved point counts are the two plans' own"
        );
        assert_eq!(run_set.compatibility, Compatibility::Differs);

        // The requirement is added and the pinned baseline moved, which is the
        // one entry of this domain that is not a specification.
        let specifications = domain(&domains, "Specifications and baselines");
        assert_eq!(specifications.difference.added, vec![ADDED_REQUIREMENT]);
        assert_eq!(specifications.difference.changed, vec![PINNED_BASELINE]);
        assert_eq!(specifications.verdict, "contract differs");

        for domain in &domains {
            assert!(
                !domain.difference.is_empty(),
                "'{}' reports nothing, so this case is not testing it",
                domain.domain
            );
            // The row's node states the whole list, uncapped: it is where a
            // reader who cannot see the table learns which entries these
            // counts are about, and it has no width to run out of.
            let announced = announced_domain_row(domain);
            for name in domain
                .difference
                .added
                .iter()
                .chain(&domain.difference.removed)
                .chain(&domain.difference.changed)
            {
                assert!(
                    announced.contains(name.as_str()),
                    "the '{}' row does not announce '{name}': {announced}",
                    domain.domain
                );
            }
        }
    }

    /// A moved baseline under an unchanged requirement set is its own verdict.
    #[test]
    fn a_moved_baseline_alone_is_named_and_verdicted_as_the_baseline() {
        let (mut app, active, retained) = app_with_two_identical_plans();
        app.state
            .workspace
            .ensure_active_plan_data(active)
            .regression_baseline_run = Some(crate::product::RunId::new());
        let records = plan_catalog_records(&app);
        let domains = compared_domains(record_of(&records, retained), record_of(&records, active));
        let specifications = domain(&domains, "Specifications and baselines");

        assert!(specifications.difference.added.is_empty());
        assert!(specifications.difference.removed.is_empty());
        assert_eq!(specifications.difference.changed, vec![PINNED_BASELINE]);
        assert_eq!(
            specifications.verdict, "baseline differs",
            "an unchanged requirement set held against a different run is not \
             a changed contract"
        );
    }

    /// A plan compared with itself differs from itself in nothing.
    ///
    /// This is not a corner case: the route opens on the active plan against
    /// the selected row, and those are the same plan whenever the reader
    /// selected the plan they already have open. So it is the pair this surface
    /// states most often, and every domain has to say so.
    #[test]
    fn a_plan_compared_with_itself_reports_no_difference_in_any_domain() {
        let (mut app, active, _) = app_with_two_identical_plans();
        diverge_the_active_plan(&mut app, active);
        let records = plan_catalog_records(&app);

        for record in &records {
            let domains = compared_domains(record, record);
            assert_eq!(domains.len(), 4);
            for domain in &domains {
                assert!(
                    domain.difference.added.is_empty()
                        && domain.difference.removed.is_empty()
                        && domain.difference.changed.is_empty(),
                    "'{}' compared '{}' with itself and found {}",
                    domain.domain,
                    record.name,
                    named_difference(&domain.difference)
                );
                assert_eq!(
                    domain.compatibility,
                    Compatibility::Same,
                    "'{}' is not toned as identical against itself",
                    domain.domain
                );
            }
            let points = record.point_count().expect("the fixture's run set validates");
            assert_eq!(
                domain(&domains, "Run set and budgets").verdict,
                format!("same {points} point{}", plan_plural_suffix(points))
            );
        }
    }

    /// Swapping the sides inverts added and removed and leaves changed alone.
    ///
    /// The direction is the whole meaning of the two columns, and it is the one
    /// thing about them the surface cannot show — which is why the footer states
    /// it. If the difference were not symmetric the hint would be a claim about
    /// arithmetic that does not hold.
    #[test]
    fn swapping_the_sides_inverts_added_and_removed() {
        let (mut app, active, retained) = app_with_two_identical_plans();
        diverge_the_active_plan(&mut app, active);
        let records = plan_catalog_records(&app);
        let base = record_of(&records, retained);
        let target = record_of(&records, active);

        let forward = compared_domains(base, target);
        let reverse = compared_domains(target, base);
        assert_eq!(forward.len(), reverse.len());
        for (forward, reverse) in forward.iter().zip(&reverse) {
            assert_eq!(forward.domain, reverse.domain);
            assert_eq!(
                forward.difference.added, reverse.difference.removed,
                "'{}' added is not the reverse comparison's removed",
                forward.domain
            );
            assert_eq!(
                forward.difference.removed, reverse.difference.added,
                "'{}' removed is not the reverse comparison's added",
                forward.domain
            );
            assert_eq!(
                forward.difference.changed, reverse.difference.changed,
                "'{}' changed is not symmetric",
                forward.domain
            );
        }
        // The run set's verdict is the one that is not symmetric, and must not
        // be: it states which way the point count moved.
        let points =
            |domains: &[ComparedDomain]| domain(domains, "Run set and budgets").verdict.clone();
        assert_ne!(points(&forward), points(&reverse));
        let to = target.point_count().expect("validates");
        assert_eq!(
            points(&forward),
            format!(
                "{} → {to} point{}",
                base.point_count().expect("validates"),
                plan_plural_suffix(to)
            )
        );
    }

    /// Each roster is the names behind the count the browse table shows.
    ///
    /// A roster that counted differently from the count beside it would put two
    /// numbers for one fact on two surfaces of one dialog, which is the exact
    /// defect this projection exists to prevent.
    #[test]
    fn each_roster_is_the_names_behind_the_count_beside_it() {
        let (mut app, active, _) = app_with_two_identical_plans();
        diverge_the_active_plan(&mut app, active);

        for record in plan_catalog_records(&app) {
            assert_eq!(
                record.analysis_roster.len(),
                record.analyses,
                "'{}' rosters a different number of analyses than it counts",
                record.name
            );
            assert_eq!(record.variable_roster.len(), record.design_variables);
            assert_eq!(record.specification_roster.len(), record.specifications);
            // Every entry is named. An unnamed entry is a row of the named
            // differences that names nothing.
            for roster in [
                &record.analysis_roster,
                &record.variable_roster,
                &record.specification_roster,
                &record.run_set_roster,
            ] {
                for entry in roster {
                    assert!(
                        !entry.name.trim().is_empty(),
                        "'{}' carries an unnamed entry",
                        record.name
                    );
                }
            }
        }
    }

    /// A run set that does not validate predicts no workload, and the verdict
    /// says so rather than reporting a point count neither plan has.
    #[test]
    fn a_run_set_that_does_not_validate_has_no_workload_to_compare() {
        let (mut app, active, retained) = app_with_two_identical_plans();
        let mut setup = app.state.sim_setup.clone();
        // A nested composition with a zero maximum depth is a declared
        // run-space error, so the active plan predicts nothing.
        setup.run_set.composition.mode =
            crate::simulation::run_set::RunSetCompositionMode::Nested;
        setup.run_set.composition.maximum_depth = 0;
        app.state.sim_setup = setup;
        let records = plan_catalog_records(&app);
        let domains = compared_domains(record_of(&records, retained), record_of(&records, active));
        let run_set = domain(&domains, "Run set and budgets");

        assert_eq!(run_set.verdict, RUN_SET_DOES_NOT_VALIDATE);
        assert_eq!(run_set.compatibility, Compatibility::Unavailable);
        assert!(
            !run_set.verdict.contains('0'),
            "an absent prediction reported a zero: {}",
            run_set.verdict
        );
        // The axes are still compared: an invalid composition does not stop the
        // two plans declaring the axes they declare.
        assert!(run_set.difference.is_empty());
    }

    /// The capped list states how many names it did not state.
    #[test]
    fn a_capped_name_list_states_how_many_it_did_not_name() {
        let names = (0..NAMED_LIMIT + 2)
            .map(|index| format!("VAR_{index}"))
            .collect::<Vec<_>>();
        assert_eq!(named_list(&names[..NAMED_LIMIT]), "VAR_0, VAR_1, VAR_2");
        assert_eq!(named_list(&names), "VAR_0, VAR_1, VAR_2, and 2 more");
        assert_eq!(
            named_difference(&Difference {
                added: names[..1].to_vec(),
                removed: Vec::new(),
                changed: names[1..2].to_vec(),
            }),
            "added VAR_0 · changed VAR_1",
            "a difference states only the parts of itself that exist"
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

    /// The three shapes every route in this module is gated at.
    #[cfg(not(target_arch = "wasm32"))]
    const GATED_VIEWPORTS: [egui::Vec2; 3] = [
        egui::Vec2::new(1024.0, 640.0),
        egui::Vec2::new(820.0, 640.0),
        egui::Vec2::new(560.0, 900.0),
    ];

    /// Every fact this surface states is painted whole at every gated viewport.
    ///
    /// The module's fit gate proves nothing is off screen, which is a different
    /// question from whether it is readable: a table cell and a select both
    /// shorten a value that outgrows their track, and a domain elided to
    /// `Analysis instances and…` still lies inside its clip rect and passes.
    /// These are the exact strings, so an arrangement that cannot hold one of
    /// them fails here rather than shipping a fact the reader has to guess at.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_fact_the_comparison_states_is_painted_whole_at_every_gated_viewport() {
        let (mut app, active, retained) = app_with_two_identical_plans();
        diverge_the_active_plan(&mut app, active);
        let records = plan_catalog_records(&app);
        let base = record_of(&records, retained);
        let target = record_of(&records, active);
        let domains = compared_domains(base, target);

        let mut owed = vec![
            plan_option_label(base),
            plan_option_label(target),
            "against".to_owned(),
            "Named differences".to_owned(),
        ];
        for column in COMPARE_COLUMNS {
            owed.push(column.heading.to_owned());
        }
        for domain in &domains {
            owed.push(domain.domain.to_owned());
            owed.push(domain.key.to_owned());
            owed.push(domain.verdict.clone());
            owed.push(named_difference(&domain.difference));
        }
        for (caption, body) in COMPARISON_BOUNDARY_NOTES {
            owed.push(caption.to_owned());
            owed.push(body.to_owned());
        }

        for screen in GATED_VIEWPORTS {
            let mut draft = SimulationPlanManagerDraft::new(active, target.name.clone());
            draft.mode = SimulationPlanManagerMode::Compare;
            draft.comparison.base_plan_id = Some(retained);
            draft.comparison.target_plan_id = Some(active);
            let painted = painted_route_text(screen, &mut draft, &records);
            for fact in &owed {
                assert!(
                    painted.contains(fact),
                    "{screen:?} did not paint {fact:?} whole; it painted {painted:?}"
                );
            }
        }
    }

    /// The authored fifth domain's absence is stated rather than left to be
    /// noticed.
    ///
    /// Painting it as a row of zeroes would assert a comparison that never ran;
    /// dropping it silently would read as an omission to anyone who knows the
    /// authored table. So the surface says why there is nothing to compare, and
    /// this is the half of that claim only this route can make.
    ///
    /// The other half — that the domain, the binding column and the execution
    /// profile are nowhere in what this route paints — moved to the omission
    /// register in `tests.rs`, which holds it over every route at every gated
    /// viewport and in the accessibility tree as well. Four surfaces refused the
    /// same authored facts and each had grown its own list.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_execution_domain_is_absent_and_the_surface_says_why() {
        let (app, active, retained) = app_with_two_identical_plans();
        let records = plan_catalog_records(&app);
        let mut draft = SimulationPlanManagerDraft::new(active, "Corner characterization");
        draft.mode = SimulationPlanManagerMode::Compare;
        draft.comparison.base_plan_id = Some(retained);
        let painted = painted_route_text(GATED_VIEWPORTS[0], &mut draft, &records);

        let stated = painted
            .iter()
            .any(|text| text.contains("owns an execution target"));
        assert!(stated, "the surface never says why the domain is absent");
    }

    /// The comparison changes neither plan.
    ///
    /// Rendered through the shell, so the claim covers the route *and* what the
    /// shell does with what it returns, and it is the whole catalog projection
    /// that is held equal rather than one field of it.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_comparison_leaves_both_plans_exactly_as_they_were() {
        let (mut app, active, retained) = app_with_two_identical_plans();
        diverge_the_active_plan(&mut app, active);
        let before = format!("{:?}", plan_catalog_records(&app));
        let mut draft = SimulationPlanManagerDraft::new(active, "Corner characterization");
        draft.mode = SimulationPlanManagerMode::Compare;
        draft.comparison.base_plan_id = Some(retained);
        draft.comparison.target_plan_id = Some(active);

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        for _ in 0..3 {
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(
                        egui::Pos2::ZERO,
                        GATED_VIEWPORTS[0],
                    )),
                    ..Default::default()
                },
                |ctx| {
                    plan_manager_dialog(ctx, &mut app, draft.clone());
                },
            );
        }

        assert_eq!(
            format!("{:?}", plan_catalog_records(&app)),
            before,
            "the comparison moved something in the catalog"
        );
        // And there is nothing in this file that could: a route with a commit
        // path would be a route with something to undo.
        let production = crate::source_guard::production_source(include_str!("compare.rs"));
        // Assembled rather than spelled whole. `module_layering.rs` ratchets
        // the crate's `&mut RSpiceApp` parameters by counting the pattern in
        // every source file, and it strips comments but not string literals —
        // so a guard that forbids the pattern by naming it was itself counted
        // as a use of it, and this file spent one of the crate's budgeted
        // whole-app signatures on forbidding whole-app signatures.
        let whole_app = concat!("&mut ", "RSpiceApp");
        for forbidden in ["fn commit_", whole_app] {
            assert!(
                !production.contains(forbidden),
                "the comparison route contains '{forbidden}', so it is no \
                 longer read-only"
            );
        }
    }

    /// Both sides are pickable, and picking one is what the comparison diffs.
    ///
    /// The two identifiers this route resolves through existed before this
    /// surface did and nothing could write them, so the fallback pair was the
    /// only pair the route could ever state. This drives the real control: it
    /// finds the picker in the accessibility tree, clicks it, clicks the option
    /// for the other plan, and asserts the draft now names it.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn both_sides_are_pickable_and_a_pick_is_what_the_comparison_diffs() {
        let (app, active, retained) = app_with_two_identical_plans();
        let records = plan_catalog_records(&app);
        let mut draft = SimulationPlanManagerDraft::new(active, "Corner characterization");
        draft.mode = SimulationPlanManagerMode::Compare;

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let pass = |draft: &mut SimulationPlanManagerDraft, events: Vec<egui::Event>| {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(
                        egui::Pos2::ZERO,
                        GATED_VIEWPORTS[0],
                    )),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    dialog(ctx, draft, &records);
                },
            )
        };
        let control = |output: &egui::FullOutput, role: egui::accesskit::Role, label: &str| {
            output
                .platform_output
                .accesskit_update
                .as_ref()
                .expect("accessibility tree")
                .nodes
                .iter()
                .find(|(_, node)| {
                    node.role() == role
                        && (node.label() == Some(label) || node.value() == Some(label))
                })
                .and_then(|(_, node)| node.bounds())
                .map(|bounds| {
                    Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    )
                })
                .unwrap_or_else(|| panic!("no {role:?} for {label:?}"))
        };
        let click = |pos: egui::Pos2| {
            vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ]
        };

        let _ = pass(&mut draft, Vec::new());
        let settled = pass(&mut draft, Vec::new());
        // Unpicked, the base is the active plan: the pair the route falls back
        // to. The picker names it, so it is the picker's own current value.
        assert_eq!(draft.comparison.base_plan_id, None);
        let picker = control(
            &settled,
            egui::accesskit::Role::ComboBox,
            "Base simulation plan",
        );
        // The click opens the list; the settling pass is what registers its
        // rows as things a later click can land on, because egui resolves a
        // press against the layout of the frame before it.
        let _ = pass(&mut draft, click(picker.center()));
        let opened = pass(&mut draft, Vec::new());
        let option = control(
            &opened,
            egui::accesskit::Role::ListBoxOption,
            &plan_option_label(record_of(&records, retained)),
        );
        let _ = pass(&mut draft, click(option.center()));

        assert_eq!(
            draft.comparison.base_plan_id,
            Some(retained),
            "clicking the retained plan in the base picker did not pick it"
        );
        let (base, target) = compared_plans(&draft, &records);
        assert_eq!(base.map(|record| record.id), Some(retained));
        assert_eq!(
            target.map(|record| record.id),
            Some(active),
            "the unpicked side still falls back to the selected row"
        );

        // The other picker is driven too, and separately. Two controls written
        // side by side are exactly where one of them ends up writing the
        // other's identifier, and a comparison whose target followed its base
        // would silently diff a plan with itself.
        let settled = pass(&mut draft, Vec::new());
        let picker = control(
            &settled,
            egui::accesskit::Role::ComboBox,
            "Compared simulation plan",
        );
        let _ = pass(&mut draft, click(picker.center()));
        let opened = pass(&mut draft, Vec::new());
        let option = control(
            &opened,
            egui::accesskit::Role::ListBoxOption,
            &plan_option_label(record_of(&records, retained)),
        );
        let _ = pass(&mut draft, click(option.center()));
        assert_eq!(draft.comparison.target_plan_id, Some(retained));
        assert_eq!(
            draft.comparison.base_plan_id,
            Some(retained),
            "picking a compared plan moved the base with it"
        );
    }
}
