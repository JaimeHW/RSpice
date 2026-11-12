//! Contract tests for the plan-manager shell.
//!
//! A sibling file rather than an inline module: `plan_manager.rs` owns the
//! browse surface and its commits, and the tests that measure them are the
//! larger half of the text. Splitting them keeps the shipped half reviewable
//! and is the convention the rest of the crate already uses.

#[cfg(not(target_arch = "wasm32"))]
use super::*;
#[cfg(not(target_arch = "wasm32"))]
use crate::product::ProcessCorner;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::simulate::test_host::SimulateTestHost;

/// James's real viewport. Display scaling makes the usable area smaller than
/// a naive test window, and the product rule is that a dialog fits without
/// scrolling — so this, not a comfortable size, is what fit means here.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const REAL_VIEWPORT: Vec2 = egui::Vec2::new(1024.0, 640.0);

/// The three viewports every route is gated at.
///
/// 1024x640 is the real desktop one — display scaling makes the usable area
/// smaller than a comfortable test window. 820 is where `WideWorkflow` stops
/// being a fixed-width panel and becomes the whole viewport, and 560x900 is the
/// portrait shape a global command can open this dialog into. The names are the
/// ones the two browse gates above already use for the same three shapes.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const GATED_VIEWPORTS: [(&str, Vec2); 3] = [
    ("desktop", REAL_VIEWPORT),
    ("full-viewport landscape", egui::Vec2::new(820.0, 640.0)),
    ("portrait", egui::Vec2::new(560.0, 900.0)),
];

/// Creating a plan installs the configuration its route committed, rather than
/// leaving the defaults `create_plan` mints.
///
/// The three inheritance flags are off by default, so nothing else in this suite
/// observes the transaction reading them at all. The assertion that matters most
/// is the last one: the reference point is the sole owner of the solver's `TEMP`
/// option, so a plan created at -40 °C has to carry -40 °C even when the options
/// block it inherited was written at 85 °C.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_created_plan_inherits_exactly_what_its_route_committed() {
    let mut harness = SimulateTestHost::harness();
    let mut setup = harness.model.sim_setup.clone();
    setup
        .set_reference_pvt(ProcessCorner::FF, 85.0)
        .expect("a physical reference corner");
    setup.options.reltol = 3.5e-6;
    setup.save_policy.retained_dataset_limit = 42;
    setup
        .model_bindings
        .push(crate::state::model_library::SimulationPlanModelBinding {
            library_name: "foundry-models".to_owned(),
            source_digest: crate::product::ContentDigest::from_bytes([3; 32]),
            selected_corner: Some("TT".to_owned()),
        });
    harness.model.sim_setup = setup;

    create::commit_create_plan(
        &mut harness.cx(),
        "Inheriting plan",
        &crate::workbench::state::NewSimulationPlanDraft {
            reference_pvt: rspice_simulation_contract::run_set::ReferencePoint {
                process: ProcessCorner::SS,
                temperature_celsius: -40.0,
            },
            inherit_model_closure: true,
            inherit_solver_options: true,
            inherit_save_policy: true,
        },
    )
    .expect("the create transaction commits");

    let setup = &harness.model.sim_setup;
    assert_eq!(setup.active_plan_name().as_str(), "Inheriting plan");
    assert_eq!(setup.reference_pvt.process, ProcessCorner::SS);
    assert_eq!(setup.reference_pvt.temperature_celsius, -40.0);
    assert_eq!(
        setup.options.reltol, 3.5e-6,
        "the inherited solver options are the previous active plan's"
    );
    assert_eq!(
        setup.save_policy.retained_dataset_limit, 42,
        "the inherited save policy is the previous active plan's"
    );
    assert_eq!(
        setup
            .model_bindings
            .first()
            .and_then(|binding| binding.selected_corner.as_deref()),
        Some("TT"),
        "the inherited model closure is the previous active plan's"
    );
    assert_eq!(
        setup.options.temp, -40.0,
        "the chosen reference temperature owns the solver's TEMP option, so it \
         has to survive an inherited options block written at another one"
    );
    // The options editor's draft is rebuilt from the options actually installed.
    // `create_plan` had built it from the defaults it minted, so without the
    // rebuild the Solver page would have opened showing the engine defaults over
    // an inherited options block. The two are asserted by value rather than by
    // string, because `set_reference_pvt` and `OptionsDialogState::from_options`
    // format a temperature differently — "-40" against "-40.0" — and pinning
    // either spelling here would assert a formatter instead of the invariant.
    assert_eq!(
        setup.session.options_draft.temp.parse::<f64>(),
        Ok(-40.0),
        "the draft states the plan's reference temperature"
    );
    let rebuilt = crate::simulation::plan::OptionsDialogState::from_options(&setup.options);
    assert_eq!(
        setup.session.options_draft.reltol, rebuilt.reltol,
        "the draft states the inherited options, not the defaults create_plan minted"
    );
    assert_ne!(
        rebuilt.reltol,
        crate::simulation::plan::OptionsDialogState::from_options(
            &crate::simulation::dialog::SimulationOptions::default()
        )
        .reltol,
        "the fixture has to make the inherited reltol differ from the default, or \
         the assertion above holds for a plan that inherited nothing"
    );
}

/// The same transaction inheriting nothing is the fresh root plan the catalog has
/// always minted. Asserting both directions is what makes the flags observable as
/// flags rather than as a copy that happens unconditionally.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_created_plan_that_inherits_nothing_keeps_the_catalog_defaults() {
    let mut harness = SimulateTestHost::harness();
    let mut setup = harness.model.sim_setup.clone();
    setup.options.reltol = 3.5e-6;
    setup.save_policy.retained_dataset_limit = 42;
    setup
        .model_bindings
        .push(crate::state::model_library::SimulationPlanModelBinding {
            library_name: "foundry-models".to_owned(),
            source_digest: crate::product::ContentDigest::from_bytes([3; 32]),
            selected_corner: Some("TT".to_owned()),
        });
    harness.model.sim_setup = setup;

    create::commit_create_plan(
        &mut harness.cx(),
        "Default root plan",
        &crate::workbench::state::NewSimulationPlanDraft::default(),
    )
    .expect("the create transaction commits");

    let setup = &harness.model.sim_setup;
    let defaults = rspice_simulation_contract::run_set::ReferencePoint::default();
    assert_eq!(setup.reference_pvt.process, defaults.process);
    assert_eq!(
        setup.reference_pvt.temperature_celsius,
        defaults.temperature_celsius
    );
    assert_eq!(
        setup.options.reltol,
        crate::simulation::dialog::SimulationOptions::default().reltol
    );
    assert_eq!(
        setup.save_policy.retained_dataset_limit,
        crate::workbench::app_state::SimulationSavePolicy::default().retained_dataset_limit
    );
    assert!(
        setup.model_bindings.is_empty(),
        "a plan that inherits no closure declares an explicitly empty one"
    );
    // The retired plan keeps everything the new one declined to inherit.
    assert_eq!(
        setup
            .inactive_plans()
            .last()
            .map(|plan| plan.model_bindings().len()),
        Some(1)
    );
}

/// A comparison diffs the two plans its own route picked, and an unpicked side
/// falls back to the plan this surface could already name.
///
/// The third case is the one a selector alone would miss: the catalog can lose a
/// plan between the frame that picked it and the frame that renders, and a stale
/// pick has to degrade to a comparison rather than to an empty surface.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_comparison_diffs_the_two_plans_its_route_picked() {
    let mut harness = SimulateTestHost::harness();
    let first = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let mut setup = harness.model.sim_setup.clone();
    let second = setup
        .create_plan("Retired sweep")
        .expect("a fresh root plan is created");
    let active = setup
        .create_plan("Corner characterization")
        .expect("a second fresh root plan is created");
    harness.model.sim_setup = setup;
    let records = plan_catalog_records(&harness.model);

    let mut draft = SimulationPlanManagerDraft::new(first, "Lab characterization");
    let (base, target) = compare::compared_plans(&draft, &records);
    assert_eq!(
        (base.map(|record| record.id), target.map(|record| record.id)),
        (Some(active), Some(first)),
        "unpicked, the pair is the active plan against the selected row"
    );

    draft.comparison.base_plan_id = Some(second);
    draft.comparison.target_plan_id = Some(first);
    let (base, target) = compare::compared_plans(&draft, &records);
    assert_eq!(
        (base.map(|record| record.id), target.map(|record| record.id)),
        (Some(second), Some(first))
    );
    assert!(
        base.is_some_and(|record| !record.active) && target.is_some_and(|record| !record.active),
        "comparing two plans that are neither of them open is the case this \
         route exists for and the one it could not express"
    );

    draft.comparison.base_plan_id = Some(crate::product::SimulationPlanId::new());
    let (base, target) = compare::compared_plans(&draft, &records);
    assert_eq!(
        (base.map(|record| record.id), target.map(|record| record.id)),
        (Some(active), Some(first)),
        "a pick the catalog no longer carries falls back instead of blanking"
    );
}

// ---------------------------------------------------------------------------
// Every route, held to the same four claims
// ---------------------------------------------------------------------------

/// Every route the manager dispatches to, in dispatch order; [`route_position`]
/// keeps it complete.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const EVERY_ROUTE: [SimulationPlanManagerMode; 8] = [
    SimulationPlanManagerMode::Browse,
    SimulationPlanManagerMode::Create,
    SimulationPlanManagerMode::Rename,
    SimulationPlanManagerMode::ConfirmArchive,
    SimulationPlanManagerMode::Compare,
    SimulationPlanManagerMode::Export,
    SimulationPlanManagerMode::Import,
    SimulationPlanManagerMode::Campaign,
];

// ---------------------------------------------------------------------------
// Route coverage
// ---------------------------------------------------------------------------

/// Where `mode` sits in [`EVERY_ROUTE`].
///
/// Exhaustive on purpose: a new mode does not compile until it is given a
/// place in the list every route gate walks.
#[cfg(not(target_arch = "wasm32"))]
fn route_position(mode: SimulationPlanManagerMode) -> usize {
    match mode {
        SimulationPlanManagerMode::Browse => 0,
        SimulationPlanManagerMode::Create => 1,
        SimulationPlanManagerMode::Rename => 2,
        SimulationPlanManagerMode::ConfirmArchive => 3,
        SimulationPlanManagerMode::Compare => 4,
        SimulationPlanManagerMode::Export => 5,
        SimulationPlanManagerMode::Import => 6,
        SimulationPlanManagerMode::Campaign => 7,
    }
}

/// Every route is listed once, at its own position.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_route_is_listed_at_its_own_position() {
    for (position, mode) in EVERY_ROUTE.into_iter().enumerate() {
        assert_eq!(
            route_position(mode),
            position,
            "{mode:?} is listed at another route's position"
        );
    }
}

/// Every source file of the plan manager, which the elision gate reads to tell
/// a string the module wrote with an ellipsis from one elided to fit.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const PLAN_MANAGER_SOURCES: &[(&str, &str)] = &[
    ("plan_manager.rs", include_str!("../plan_manager.rs")),
    ("tests.rs", include_str!("tests.rs")),
    ("campaign.rs", include_str!("campaign.rs")),
    ("compare.rs", include_str!("compare.rs")),
    ("create.rs", include_str!("create.rs")),
    ("exchange.rs", include_str!("exchange.rs")),
    ("kit.rs", include_str!("kit.rs")),
    ("lifecycle.rs", include_str!("lifecycle.rs")),
    ("records.rs", include_str!("records.rs")),
];

// ---------------------------------------------------------------------------
// Elision gates for the routes that do not gate themselves
// ---------------------------------------------------------------------------
