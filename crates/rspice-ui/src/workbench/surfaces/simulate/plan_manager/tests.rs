//! Contract tests for the plan-manager shell.
//!
//! A sibling file rather than an inline module: `plan_manager.rs` owns the
//! browse surface and its commits, and the tests that measure them are the
//! larger half of the text. Splitting them keeps the shipped half reviewable
//! and is the convention the rest of the crate already uses.

use super::*;
use crate::product::ProcessCorner;

/// James's real viewport. Display scaling makes the usable area smaller than
/// a naive test window, and the product rule is that a dialog fits without
/// scrolling — so this, not a comfortable size, is what fit means here.
#[cfg(not(target_arch = "wasm32"))]
const REAL_VIEWPORT: Vec2 = egui::Vec2::new(1024.0, 640.0);

/// The size the real dialog hands its body at one viewport.
///
/// Measured from the `Dialog` widget rather than derived from its size spec.
/// Header height, footer height, the scroll area's own inset and the
/// narrow-viewport rule all belong to that widget, and a number copied out
/// of them here would go stale the first time one of them moved.
/// `the_shell_composes_the_geometry_these_tests_measure` holds the two
/// builder settings this reconstructs.
#[cfg(not(target_arch = "wasm32"))]
fn measured_dialog_body_size(screen: Vec2) -> Vec2 {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut size = Vec2::ZERO;
    let mut run = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, screen)),
                ..Default::default()
            },
            |root| {
                Dialog::new(PLAN_DIALOG_EYEBROW, PLAN_DIALOG_TITLE, PLAN_DIALOG_PRIMARY)
                    .description(PLAN_DIALOG_DESCRIPTION)
                    .size(DialogSize::CapabilityReview)
                    .flush_body()
                    .ghost("Close")
                    .show(root, |ui| {
                        // The body's clip rect is its scroll viewport, which is
                        // the height there actually is. `available_height` is
                        // only the cursor's remainder inside that viewport.
                        //
                        // The probe claims more height than any viewport can
                        // give, because the body's scroll area shrinks to its
                        // content: a probe that paints nothing would measure a
                        // viewport collapsed around the probe instead of the
                        // budget a real body would be held to.
                        let width = ui.available_width();
                        ui.allocate_space(vec2(width, 4_000.0));
                        size = vec2(width, ui.clip_rect().height());
                    });
            },
        )
    };
    let _ = run();
    let _ = run();
    let _ = run();
    assert!(
        size.x > 0.0 && size.y > 0.0,
        "the dialog painted no body at {screen:?}"
    );
    size
}

/// One render of the manager body, read four ways.
#[cfg(not(target_arch = "wasm32"))]
struct RenderedBody {
    /// Every string the body painted, in paint order.
    painted: Vec<String>,
    /// Each row's complete announced fact, in painted row order.
    rows: Vec<String>,
    /// The size the body actually used, on both axes. This is the quantity
    /// an enclosing scroll area measures to decide whether to scroll, so a
    /// value over the body size is the overflow defect itself rather than a
    /// proxy for it.
    used: Vec2,
    /// The size the real dialog hands its body at this viewport.
    body: Vec2,
}

/// Render the manager body at the body size the real dialog would give it.
///
/// Order comes from the paint and values come from the accessibility nodes,
/// for the reason [`rendered_aside`] documents: a cell's text is elided to
/// its column, so only the node carries the whole fact, and only the paint
/// carries an order.
///
/// The render surface is deliberately taller than the body. Clipping the
/// content to the height under test would measure the clip rather than the
/// content, and the whole point is to find out how tall the surface wants to
/// be so it can be held to what there is.
#[cfg(not(target_arch = "wasm32"))]
fn rendered_body(
    app: &RSpiceApp,
    draft: &mut SimulationPlanManagerDraft,
    screen: Vec2,
) -> RenderedBody {
    let records = plan_catalog_records(app);
    let body = measured_dialog_body_size(screen);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut used = Vec2::ZERO;
    let mut run = |draft: &mut SimulationPlanManagerDraft| {
        let mut action = None;
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(body.x, 4_000.0))),
                ..Default::default()
            },
            |root| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(root, |ui| {
                        // Measured on a nested `vertical`, not on the panel:
                        // a `CentralPanel` fills its viewport, so its own
                        // `min_rect` reports the canvas height rather than
                        // the height the content wanted.
                        used = ui
                            .vertical(|ui| {
                                plan_manager_body(ui, draft, &records, &mut action);
                            })
                            .response
                            .rect
                            .size();
                    });
            },
        )
    };
    let _ = run(draft);
    let _ = run(draft);
    let output = run(draft);

    let announced = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AccessKit body tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .filter(|label| label.contains(" · revision "))
        .collect::<Vec<_>>();
    let mut painted = Vec::new();
    let mut rows: Vec<String> = Vec::new();
    for shape in &output.shapes {
        if let egui::epaint::Shape::Text(text) = &shape.shape {
            let painted_text = text.galley.job.text.clone();
            // The identity cell elides its name to the column, so the
            // painted glyphs are a prefix of the announced fact plus an
            // ellipsis. Matching on the raw string would silently stop
            // finding rows at the widths where the column is narrow —
            // which are exactly the widths worth testing.
            let probe = painted_text.trim_end_matches('\u{2026}');
            if !probe.is_empty()
                && let Some(row) = announced.iter().find(|row| row.starts_with(probe))
                && !rows.contains(row)
            {
                rows.push(row.clone());
            }
            painted.push(painted_text);
        }
    }
    RenderedBody {
        painted,
        rows,
        used,
        body,
    }
}

/// The draft a route is actually used with.
///
/// Every arm is a transcription of one arm of `handle_plan_manager_action`,
/// which is the only thing in the module that ever writes `mode`, so the state
/// it leaves behind is the only state a route can be *entered* with. Nothing
/// here is invented on top of that: an input no control in the dialog can
/// produce would gate a layout the product cannot reach.
///
/// Import is the one arm that goes past the entry state, and deliberately. The
/// shell hands it an empty field for the reader to paste into, so a gate on the
/// entry state alone would certify a dialog that has never been asked to hold a
/// package — which is the only thing the route is for.
///
/// Both exchange directions are given a real export of a real plan, so the size
/// of what they render is that plan's rather than a number chosen here.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn route_draft(
    app: &RSpiceApp,
    mode: SimulationPlanManagerMode,
    active: SimulationPlanId,
    inactive: SimulationPlanId,
) -> SimulationPlanManagerDraft {
    let records = plan_catalog_records(app);
    let name_of = |id: SimulationPlanId| {
        records
            .iter()
            .find(|record| record.id == id)
            .unwrap_or_else(|| panic!("the fixture's plan {id} is projected"))
            .name
            .clone()
    };
    let mut draft = SimulationPlanManagerDraft::new(active, name_of(active));
    draft.mode = mode;
    match mode {
        // Both open on the selected row under its current name, which is what
        // the browse surface already put in the draft.
        SimulationPlanManagerMode::Browse | SimulationPlanManagerMode::Rename => {}
        SimulationPlanManagerMode::Create => draft.name = "New simulation plan".to_owned(),
        // The catalog refuses archiving the active plan and the browse
        // surface disables the button there, so this confirmation is only ever
        // reached from an inactive row.
        SimulationPlanManagerMode::ConfirmArchive => {
            draft.selected_plan_id = inactive;
            draft.name = name_of(inactive);
        }
        // Neither side is picked, because the route paints no picker: both
        // resolve through `compared_plans` to the active plan against the
        // selected row. Selecting an inactive row is what makes that pair two
        // different plans, and it is the reader's own click; writing
        // `comparison.base_plan_id` here would be state no control can set.
        SimulationPlanManagerMode::Compare => {
            draft.selected_plan_id = inactive;
            draft.name = name_of(inactive);
        }
        // The shell exports the package before it switches routes, so Export is
        // only ever entered with a real one already in the draft.
        SimulationPlanManagerMode::Export => {
            draft.exchange_text = exchange::export_simulation_plan_package(app, active)
                .expect("the active plan exports");
        }
        // The shell clears the field before importing and the reader pastes a
        // package into it. That paste is the route's one input and the only way
        // to reach its primary action, so it is part of using the route rather
        // than state invented for the gate.
        SimulationPlanManagerMode::Import => {
            draft.name = "Imported simulation plan".to_owned();
            draft.exchange_text = exchange::export_simulation_plan_package(app, active)
                .expect("the active plan exports");
        }
        // Two members: what the shell seeds, and the fewest
        // `prepare_and_start_campaign` will queue. It caps the other end at 64
        // members, which no catalog these gates build comes close to.
        SimulationPlanManagerMode::Campaign => {
            draft.campaign.member_ids = records
                .iter()
                .filter(|record| !record.archived)
                .map(|record| record.id)
                .take(2)
                .collect();
        }
    }
    draft
}

/// Every string the real dialog painted at one viewport, split by whether it
/// actually landed on screen.
///
/// The body-level height measurement proves the content is no taller than
/// the budget. It cannot prove the dialog put it somewhere the reader can
/// see: egui still emits a shape its clip rect excludes, so a label scrolled
/// under the footer is painted and invisible at once. A text shape counts as
/// visible only when its whole box is inside the clip rect it was painted
/// with — the same question as "would the user have to scroll to read this".
///
/// Both lists come from the paint and nothing else. There is deliberately no
/// list of expected strings anywhere in this file's fit gates: a route that
/// adds, drops or rewords a label is covered by that alone, so five lanes can
/// redesign five routes without any of them editing this file — and an
/// "improvement" that introduced an expected-label list would quietly take
/// that guarantee away.
#[cfg(not(target_arch = "wasm32"))]
fn dialog_visible_text(
    screen: Vec2,
    extra_plans: usize,
    mode: SimulationPlanManagerMode,
) -> (Vec<String>, Vec<String>) {
    let (mut app, active, available, _) = app_with_every_lifecycle_state();
    if extra_plans > 0 {
        let mut setup = app.state.sim_setup.clone();
        for index in 0..extra_plans {
            let id = setup
                .create_plan(format!("Spare plan {index}"))
                .expect("a fresh root plan is created");
            app.state.workspace.migrate_inactive_plan_data(id);
        }
        setup
            .activate_plan(active)
            .expect("the fixture's active plan reactivates");
        app.state.sim_setup = setup;
    }
    let draft = route_draft(&app, mode, active, available);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    // The dialog owns the draft for a frame and re-arms it in
    // `simulation_workflow`, so each pass is handed its own copy and the three
    // passes render the same state rather than accumulating it.
    let mut pass = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, screen)),
                ..Default::default()
            },
            |ctx| {
                plan_manager_dialog(ctx, &mut app, draft.clone());
            },
        )
    };
    let _ = pass();
    let _ = pass();
    let output = pass();

    let mut visible = Vec::new();
    let mut clipped = Vec::new();
    for shape in &output.shapes {
        if let egui::epaint::Shape::Text(text) = &shape.shape {
            let rect = Rect::from_min_size(text.pos, text.galley.size());
            let painted = text.galley.job.text.clone();
            // One point of tolerance. A galley's box can sit a fraction of
            // a point past a container edge through row-height rounding and
            // still be entirely readable; a row the reader would have to
            // scroll to is out by tens of points, so the distinction this
            // draws is not a close call.
            if shape.clip_rect.expand(1.0).contains_rect(rect) {
                visible.push(painted);
            } else {
                clipped.push(painted);
            }
        }
    }
    (visible, clipped)
}

/// A catalog with one plan of each lifecycle state, so a scope, a tone and a
/// filter each have all three to distinguish. Returns the active, available and
/// archived ids.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn app_with_every_lifecycle_state() -> (
    RSpiceApp,
    SimulationPlanId,
    SimulationPlanId,
    SimulationPlanId,
) {
    let mut app = RSpiceApp::test_instance();
    let available = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("the fixture has a stable plan")
        .id();
    let mut setup = app.state.sim_setup.clone();
    let retired = setup
        .create_plan("Retired sweep")
        .expect("a fresh root plan is created");
    let active = setup
        .create_plan("Corner characterization")
        .expect("a second fresh root plan is created");
    setup
        .archive_plan(retired)
        .expect("an inactive plan archives");
    app.state.workspace.migrate_active_plan_data(available);
    app.state.workspace.migrate_inactive_plan_data(retired);
    app.state.workspace.migrate_inactive_plan_data(active);
    app.state.sim_setup = setup;
    (app, active, available, retired)
}

/// The one defect the authored reference has: its records table is 899
/// points wide inside a 685-point cell, so the dialog scrolls sideways.
///
/// This measures the width the body actually consumes against the width the
/// real dialog gives it, at the desktop width and at the width where the
/// surface goes edge-to-edge. That quantity is exactly what a scroll area
/// compares to decide whether to scroll, so an assertion on it is the fit
/// itself and not a stand-in — and because the split's columns allocate
/// their content rather than clip it, an inner region that overflowed would
/// widen this number too.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_manager_fits_the_real_viewport_at_every_supported_width() {
    let (app, active, _, _) = app_with_every_lifecycle_state();
    let minimum = kit::table_minimum_width(&PLAN_COLUMNS);
    // The real viewport, and the width at which `WideWorkflow` stops being a
    // fixed-width panel and becomes the whole viewport. 820 is that
    // threshold, not a floor: a narrower window makes a narrower dialog, and
    // those are portrait shapes gated by
    // `a_portrait_viewport_stacks_the_surface_and_still_fits`.
    let viewports = [
        ("desktop", REAL_VIEWPORT),
        ("full-viewport landscape", vec2(820.0, REAL_VIEWPORT.y)),
    ];

    for (arrangement, screen) in viewports {
        let mut draft = SimulationPlanManagerDraft::new(active, "Corner characterization");
        let rendered = rendered_body(&app, &mut draft, screen);
        assert!(
            rendered.used.x <= rendered.body.x + 0.5,
            "{arrangement} at {screen:?}: the manager used {} of a {} body \
             width, {:.1} points of horizontal overflow",
            rendered.used.x,
            rendered.body.x,
            rendered.used.x - rendered.body.x
        );
        // Landscape stays two-column. Stacking costs the sum of both
        // columns' heights instead of the taller one, which 640 points of
        // viewport does not have — so at these shapes the surface must not
        // reach for it.
        assert!(
            !kit::split_tracks(rendered.body.x, minimum).stacked,
            "{arrangement} at {screen:?}: the split stacked for a {} body, \
             and stacking does not fit this height",
            rendered.body.x
        );
        assert_eq!(
            rendered.rows.len(),
            3,
            "every catalog row is painted in {arrangement}: {:?}",
            rendered.rows
        );
        let owed = [
            "Rename…",
            "Clone…",
            "Compare…",
            "Export…",
            "Archive…",
            "Selected plan",
            "Name",
            "Revision",
            "Reference PVT corner",
            "Declared run set",
            "Modelled cost",
            "Model closure",
            "Regression baseline",
            "Source lineage",
            "Switching is atomic",
            "Results are references",
            "Stable identity retained",
        ];
        // A property label is elided to its column, so the painted string
        // may be a prefix plus an ellipsis. Only an actually-elided string
        // is allowed to match by prefix: without that rule a shortened label
        // would satisfy a longer one that is in fact missing, and the aside's
        // closing status could go unnoticed behind a property row.
        let shows = |painted: &[String], label: &str| {
            painted.iter().any(|text| {
                text == label
                    || text
                        .strip_suffix('\u{2026}')
                        .is_some_and(|prefix| !prefix.is_empty() && label.starts_with(prefix))
            })
        };
        // Vertical fit is asserted at the dialog level, where clipping is
        // real: every label this surface owes the reader has to be on
        // screen, not merely emitted. The five per-plan operations are the
        // point of selecting a row, and the eight detail rows are the
        // plan's facts.
        //
        // And fitting the fixture is not enough. The records table is the
        // one unbounded thing here — a catalog with more plans is a taller
        // table — so the same claim has to hold with more plans than the
        // fixture, or the surface overflows on the user's next plan rather
        // than on a later edit to this file.
        for extra_plans in [0, 2] {
            let (visible, clipped) =
                dialog_visible_text(screen, extra_plans, SimulationPlanManagerMode::Browse);
            for label in owed {
                assert!(
                    !shows(&clipped, label),
                    "{arrangement} at {screen:?} with {extra_plans} extra \
                     plan(s): '{label}' is painted outside its clip rect, so \
                     the reader would have to scroll to it. This dialog has \
                     to fit."
                );
                assert!(
                    shows(&visible, label),
                    "{arrangement} at {screen:?} with {extra_plans} extra \
                     plan(s): '{label}' is not on screen"
                );
            }
        }
    }
}

/// A portrait viewport stacks the two columns, and the surface still fits.
///
/// This is the arrangement the stacked path exists for, and it is reachable:
/// `Command::ManageSimulationPlans` is a `ShortcutContext::Global` entry in
/// `COMMAND_REGISTRY`, and the command palette that lists it has its own
/// phone layout — so a narrow viewport can open this dialog, and at 560 the
/// dialog is the whole viewport rather than a panel inside it.
///
/// Portrait is narrow *and tall*, which is why the earlier gate at 820x640
/// did not cover it: that is a landscape shape, and the two constraints do
/// not co-occur. Here the detail spends the full width on two side-by-side
/// halves of its row list, which is what buys back the height stacking costs.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_portrait_viewport_stacks_the_surface_and_still_fits() {
    // Narrower than `segmented::PHONE_MAX_WIDTH`, and tall as portrait is.
    let screen = vec2(560.0, 900.0);
    let body = measured_dialog_body_size(screen);
    assert!(
        kit::split_tracks(body.x, kit::table_minimum_width(&PLAN_COLUMNS)).stacked,
        "a {} body did not stack; this test would then be asserting the \
         landscape arrangement over again",
        body.x
    );

    for extra_plans in [0, 2] {
        let (visible, clipped) =
            dialog_visible_text(screen, extra_plans, SimulationPlanManagerMode::Browse);
        for label in [
            "Rename…",
            "Clone…",
            "Compare…",
            "Export…",
            "Archive…",
            "Selected plan",
            "Declared run set",
            "Modelled cost",
            "Regression baseline",
            "Switching is atomic",
            "Results are references",
            "Stable identity retained",
        ] {
            let shows = |painted: &[String]| {
                painted.iter().any(|text| {
                    text == label
                        || text
                            .strip_suffix('\u{2026}')
                            .is_some_and(|prefix| !prefix.is_empty() && label.starts_with(prefix))
                })
            };
            assert!(
                !shows(&clipped),
                "portrait at {screen:?} with {extra_plans} extra plan(s): \
                 '{label}' is painted outside its clip rect"
            );
            assert!(
                shows(&visible),
                "portrait at {screen:?} with {extra_plans} extra plan(s): \
                 '{label}' is not on screen"
            );
        }
    }
}

/// The three viewports every route is gated at.
///
/// 1024x640 is the real desktop one — display scaling makes the usable area
/// smaller than a comfortable test window. 820 is where `WideWorkflow` stops
/// being a fixed-width panel and becomes the whole viewport, and 560x900 is the
/// portrait shape a global command can open this dialog into. The names are the
/// ones the two browse gates above already use for the same three shapes.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const GATED_VIEWPORTS: [(&str, Vec2); 3] = [
    ("desktop", REAL_VIEWPORT),
    ("full-viewport landscape", egui::Vec2::new(820.0, 640.0)),
    ("portrait", egui::Vec2::new(560.0, 900.0)),
];

/// A clipped label as an assertion message names it: one short line. A route
/// can paint a whole document into a single galley, and the failure is no
/// clearer for quoting all of it.
///
/// Whitespace is flattened rather than cut at the first newline. A pretty
/// printed package opens with a line containing one brace, and a message
/// reporting that a `{` is off screen names nothing a reader could act on.
#[cfg(not(target_arch = "wasm32"))]
fn clipped_label_preview(label: &str) -> String {
    let flattened = label.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut preview = flattened.chars().take(60).collect::<String>();
    if preview.len() < flattened.len() {
        preview.push('\u{2026}');
    }
    preview
}

/// One route fits every gated viewport, with the catalog the reader has and
/// with the catalog they will have after two more plans.
///
/// The claim is the product's hard rule stated at the only level where it is
/// checkable: no dialog scrolls, so no text the route paints may land outside
/// the clip rect it was painted with.
///
/// There is no list of expected labels here, and there must not be one. The
/// labels come from the paint, so a lane that adds, drops or rewords one is
/// covered without touching this file — which is exactly what lets five lanes
/// redesign five routes at once. A list of owed strings would turn every
/// redesign into an edit of this file and every merge into a collision. The two
/// browse gates above do carry such a list, deliberately: they assert *which*
/// facts that surface owes the reader, which is a claim about browse and not a
/// fit gate.
///
/// The extra plans matter because the catalog is the one unbounded input every
/// route reads: a surface that fits three plans and not five fails on the
/// reader's next plan rather than on a later edit here.
///
/// Every mode has one gate below, one test each, so a lane deletes its own
/// `#[ignore]` and no one else's. A ninth mode cannot be added without a gate
/// by accident either: [`route_draft`] matches the enum exhaustively, so a new
/// variant stops this file compiling until someone states how the reader
/// reaches it — which is the moment its gate gets written.
#[cfg(not(target_arch = "wasm32"))]
fn assert_route_fits_every_gated_viewport(mode: SimulationPlanManagerMode) {
    for (arrangement, screen) in GATED_VIEWPORTS {
        for extra_plans in [0, 2] {
            let (visible, clipped) = dialog_visible_text(screen, extra_plans, mode);
            assert!(
                !visible.is_empty(),
                "{mode:?} in {arrangement} at {screen:?} with {extra_plans} \
                 extra plan(s) painted no text at all, so this gate would pass \
                 whatever it did with the rest"
            );
            assert!(
                clipped.is_empty(),
                "{mode:?} in {arrangement} at {screen:?} with {extra_plans} \
                 extra plan(s): {} label(s) are painted outside their clip \
                 rect, so the reader would have to scroll to read them. This \
                 dialog has to fit. {:?}",
                clipped.len(),
                clipped
                    .iter()
                    .map(|label| clipped_label_preview(label))
                    .collect::<Vec<_>>()
            );
        }
    }
}

/// Browse fits at all three viewports with nothing clipped at all.
///
/// Stronger than the two gates above and not a repeat of them: they name the
/// labels that surface owes the reader and check those are on screen, which
/// says nothing about a label neither of them lists. This says no painted text
/// is off screen, whatever it is.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_browse_route_fits_every_gated_viewport() {
    assert_route_fits_every_gated_viewport(SimulationPlanManagerMode::Browse);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_create_route_fits_every_gated_viewport() {
    assert_route_fits_every_gated_viewport(SimulationPlanManagerMode::Create);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_rename_route_fits_every_gated_viewport() {
    assert_route_fits_every_gated_viewport(SimulationPlanManagerMode::Rename);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_archive_confirmation_fits_every_gated_viewport() {
    assert_route_fits_every_gated_viewport(SimulationPlanManagerMode::ConfirmArchive);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_compare_route_fits_every_gated_viewport() {
    assert_route_fits_every_gated_viewport(SimulationPlanManagerMode::Compare);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_export_route_fits_every_gated_viewport() {
    assert_route_fits_every_gated_viewport(SimulationPlanManagerMode::Export);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_import_route_fits_every_gated_viewport() {
    assert_route_fits_every_gated_viewport(SimulationPlanManagerMode::Import);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_campaign_route_fits_every_gated_viewport() {
    assert_route_fits_every_gated_viewport(SimulationPlanManagerMode::Campaign);
}

/// The seven columns are painted, in order, and the two the authored table
/// carries and RSpice cannot own are not among them.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_table_paints_the_seven_columns_whose_facts_have_owners() {
    let (app, active, _, _) = app_with_every_lifecycle_state();
    let mut draft = SimulationPlanManagerDraft::new(active, "Corner characterization");
    let rendered = rendered_body(&app, &mut draft, REAL_VIEWPORT);

    let headings = PLAN_COLUMNS
        .iter()
        .map(|column| column.heading)
        .collect::<Vec<_>>();
    // The aside repeats two of these words as property-row labels, so the
    // count is taken over the records column alone: everything the body
    // paints before the aside's own head.
    let aside = rendered
        .painted
        .iter()
        .position(|text| text == "Selected plan")
        .expect("the aside's head is painted");
    assert_eq!(
        rendered.painted[..aside]
            .iter()
            .filter(|text| headings.contains(&text.as_str()))
            .map(String::as_str)
            .collect::<Vec<_>>(),
        headings,
        "the column headings are painted once each, left to right"
    );
    // The two columns this table does *not* carry were guarded here. They are
    // now three entries in `UNOWNED_AUTHORED_FACTS`, which holds the same
    // ground over every route rather than over this one — a binding column has
    // no owner on Compare either, and that was a second guard in a second file.
}

/// Every row states the projection's own facts, in the projection's order.
///
/// A cell elides to its column, so the row carries one node with the whole
/// row in it. That node is the only place a reader who cannot see the table
/// learns which plan a revision or a forecast belongs to.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_row_announces_the_facts_its_cells_paint() {
    let (app, active, available, retired) = app_with_every_lifecycle_state();
    let mut draft = SimulationPlanManagerDraft::new(active, "Corner characterization");
    let rendered = rendered_body(&app, &mut draft, REAL_VIEWPORT);
    let records = plan_catalog_records(&app);

    assert_eq!(
        rendered
            .rows
            .iter()
            .map(|row| row.split(" · ").next().unwrap_or_default())
            .collect::<Vec<_>>(),
        records.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(),
        "the painted rows follow the projection's order"
    );
    for (id, lifecycle) in [
        (active, "active"),
        (available, "available"),
        (retired, "archived"),
    ] {
        let record = records
            .iter()
            .find(|record| record.id == id)
            .expect("the projection carries every catalog entry");
        let row = rendered
            .rows
            .iter()
            .find(|row| row.starts_with(record.name.as_str()))
            .unwrap_or_else(|| panic!("no row announced for '{}'", record.name));
        assert_eq!(record.lifecycle_label(), lifecycle);
        for fact in [
            record.id.to_string(),
            record.lifecycle_label().to_owned(),
            format!("revision {}", record.revision),
            format!("{} of {} analyses enabled", record.enabled, record.analyses),
            format!("run set {}", record.run_set_label()),
            format!("{} model binding", record.model_bindings),
            format!("{} result reference", record.results),
        ] {
            assert!(
                row.contains(&fact),
                "the row for '{}' does not state '{fact}': {row}",
                record.name
            );
        }
    }
}

/// The two narrowing scopes partition the catalog.
///
/// Every plan is admitted by exactly one of them, so the reader's ordinary
/// view hides retired plans without also hiding the plan being worked on,
/// and no plan is reachable only by widening to every record.
#[test]
fn the_narrowing_scopes_partition_the_catalog() {
    let (app, active, available, retired) = app_with_every_lifecycle_state();
    let records = plan_catalog_records(&app);
    let admitted = |scope| {
        records
            .iter()
            .filter(|record| plan_scope_admits(scope, record))
            .map(|record| record.id)
            .collect::<Vec<_>>()
    };

    assert_eq!(admitted(SimulationPlanScope::All).len(), 3);
    assert_eq!(
        admitted(SimulationPlanScope::Working),
        vec![active, available],
        "the working view is the active plan and every retained one"
    );
    assert_eq!(admitted(SimulationPlanScope::Archived), vec![retired]);

    for record in &records {
        assert_eq!(
            PLAN_SCOPES
                .iter()
                .filter(|(scope, _)| *scope != SimulationPlanScope::All
                    && plan_scope_admits(*scope, record))
                .count(),
            1,
            "'{}' is admitted by no narrowing scope, or by both",
            record.name
        );
    }
    assert_eq!(
        PLAN_SCOPES.len(),
        3,
        "the scope control offers every record plus the two halves of the \
         catalog; a fourth option would have to name a slice the catalog \
         can actually produce"
    );
}

/// The filter matches the four fields its placeholder names, and the
/// placeholder names no fifth.
///
/// It matched name and identity only, so a plan was unfindable by two facts
/// its own row paints: the revision and the declared run-set size.
#[test]
fn the_filter_matches_the_four_fields_its_placeholder_names() {
    let (app, _, _, _) = app_with_every_lifecycle_state();
    let records = plan_catalog_records(&app);
    let record = records.first().expect("the active plan is projected");

    for field in [
        record.name.to_ascii_lowercase(),
        record.id.to_string().to_ascii_lowercase(),
        record.revision.to_string(),
        record.run_set_label().to_ascii_lowercase(),
    ] {
        assert!(
            matches_plan_filter(record, &field),
            "the filter does not match '{field}'"
        );
    }
    assert!(
        matches_plan_filter(record, ""),
        "an empty filter admits every plan"
    );
    assert!(!matches_plan_filter(record, "testbench"));

    for named in ["name", "identity", "revision", "run set"] {
        assert!(
            PLAN_FILTER_HINT.to_ascii_lowercase().contains(named),
            "the placeholder does not name '{named}', which the filter matches"
        );
    }
    assert!(
        !PLAN_FILTER_HINT.to_ascii_lowercase().contains("binding"),
        "the placeholder names a binding, which no RSpice plan has and the \
         filter cannot match"
    );
}

/// Three states, three words, three tones, all from their one owner.
#[test]
fn the_lifecycle_tone_follows_the_word_its_owner_publishes() {
    let (app, active, available, retired) = app_with_every_lifecycle_state();
    let records = plan_catalog_records(&app);
    for (id, word, tone) in [
        (active, "active", LifecycleTone::Active),
        (available, "available", LifecycleTone::Available),
        (retired, "archived", LifecycleTone::Archived),
    ] {
        let record = records
            .iter()
            .find(|record| record.id == id)
            .expect("the projection carries every catalog entry");
        assert_eq!(record.lifecycle_label(), word);
        assert_eq!(
            lifecycle_tone(record),
            tone,
            "'{word}' is toned as another state"
        );
    }
}

/// Importing creates a plan, so it sits with the other two actions that
/// create one rather than in the row of operations on the selected plan.
///
/// The check is positional because that is the claim: Import is painted in
/// the toolbar, before the table exists, and no longer between Export and
/// Archive where it read as "import over this plan".
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn import_is_a_toolbar_action_and_not_an_operation_on_the_selected_plan() {
    let (app, active, _, _) = app_with_every_lifecycle_state();
    let mut draft = SimulationPlanManagerDraft::new(active, "Corner characterization");
    let rendered = rendered_body(&app, &mut draft, REAL_VIEWPORT);
    let at = |label: &str| {
        rendered
            .painted
            .iter()
            .position(|text| text == label)
            .unwrap_or_else(|| panic!("'{label}' is not painted"))
    };

    assert!(
        at("Import…") < at("Plan / identity"),
        "Import is painted after the table's first heading, so it is not in \
         the toolbar"
    );
    assert!(
        at("Import…") < at("Rename…"),
        "Import is painted inside the selected-plan action row"
    );
    for kept in ["Rename…", "Clone…", "Compare…", "Export…", "Archive…"] {
        assert!(
            at(kept) > at("Plan / identity"),
            "'{kept}' is painted before the table, so it left the \
             selected-plan action row"
        );
    }
}

/// The two boundaries are stated, and neither borrows a clause from the
/// authored notes that RSpice has no owner for.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_boundary_notes_claim_only_what_the_commit_paths_guarantee() {
    let (app, active, _, _) = app_with_every_lifecycle_state();
    let mut draft = SimulationPlanManagerDraft::new(active, "Corner characterization");
    let rendered = rendered_body(&app, &mut draft, REAL_VIEWPORT);

    for (caption, _) in PLAN_BOUNDARY_NOTES {
        assert!(
            rendered.painted.iter().any(|text| text == caption),
            "the '{caption}' boundary is not painted"
        );
    }
    let notes = PLAN_BOUNDARY_NOTES
        .iter()
        .map(|(_, body)| (*body).to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    // The four authored clauses these notes do not borrow were listed here.
    // They are now four entries in `UNOWNED_AUTHORED_FACTS`, checked against
    // what every route paints and announces rather than against this one
    // constant — the notes are painted, so nothing is given up by the move.
    for guaranteed in ["validates", "receipt"] {
        assert!(
            notes.contains(guaranteed),
            "the notes never mention '{guaranteed}', which is what makes \
             them true of RSpice"
        );
    }
}

/// The aside states the selected plan's lifecycle and closes with what the
/// four operations preserve.
///
/// The identity note's clauses are checked against the catalog itself here,
/// not just asserted to be painted: a note that says archiving is refused on
/// the active plan has to be a claim the commit path actually enforces.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_aside_states_the_lifecycle_and_the_identity_the_operations_preserve() {
    let (mut app, active, available, retired) = app_with_every_lifecycle_state();

    for (id, word) in [
        (active, "active"),
        (available, "available"),
        (retired, "archived"),
    ] {
        let mut draft = SimulationPlanManagerDraft::new(id, "selected");
        let rendered = rendered_body(&app, &mut draft, REAL_VIEWPORT);
        let head = rendered
            .painted
            .iter()
            .position(|text| text == "Selected plan")
            .expect("the aside's head is painted");
        assert_eq!(
            rendered.painted[head..]
                .iter()
                .find(|text| matches!(text.as_str(), "active" | "available" | "archived"))
                .map(String::as_str),
            Some(word),
            "the aside head states the wrong lifecycle for {id}"
        );
        assert!(
            rendered
                .painted
                .iter()
                .any(|text| text == "Stable identity retained"),
            "the aside does not close with what the operations preserve"
        );
    }

    // Renaming keeps the identity and the revision, as the note claims.
    let before = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();
    lifecycle::commit_rename_plan(&mut app, active, "Corner characterization v2")
        .expect("the active plan renames");
    let after = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan");
    assert_eq!(after.id(), active, "a rename moved the plan's identity");
    assert_eq!(after.revision(), before, "a rename moved the revision");

    // Archiving is refused on the active plan by the catalog itself, not
    // merely by a disabled button.
    let error = lifecycle::commit_archive_plan(&mut app.state.sim_setup, active)
        .expect_err("archiving the active plan is refused");
    assert!(
        error.to_ascii_lowercase().contains("active"),
        "the refusal does not say the plan is active: {error}"
    );
    // And an archive is reversible.
    lifecycle::commit_restore_plan(&mut app.state.sim_setup, retired)
        .expect("an archived plan restores");
    assert!(
        plan_catalog_records(&app)
            .iter()
            .all(|record| !record.archived),
        "restoring left the plan archived"
    );
}

/// The geometry the fit test measures is the geometry the shell composes.
///
/// `measured_dialog_body_size` reconstructs the dialog with two builder
/// settings, and the split's breakpoint is only the table's if the shell
/// hands it the table's own minimum. Both are source facts, so both are
/// checked here rather than assumed by the test that depends on them.
///
/// The header is the third geometry-bearing setting, and the only one pinned by
/// its absence rather than its presence: the probe reconstructs a dialog that
/// has one, so a shell that hid its own would be measured against 57 points of
/// body budget it never paints.
#[test]
fn the_shell_composes_the_geometry_these_tests_measure() {
    let shell = crate::source_guard::production_source(include_str!("../plan_manager.rs"));
    for required in [
        "DialogSize::CapabilityReview",
        ".flush_body()",
        "kit::table_minimum_width(&PLAN_COLUMNS)",
        "&PLAN_COLUMNS,",
    ] {
        assert!(
            shell.contains(required),
            "the shell no longer contains `{required}`, so the fit \
             measurement no longer describes what it paints"
        );
    }
    assert!(
        !shell.contains("egui::Grid::new(\"simulation.plan-manager.rows\")"),
        "the records table is back on an auto-sized grid, which cannot be \
         held to a column budget"
    );
    assert!(
        !shell.contains(".without_header()"),
        "the shell hides its header, so the fit measurement is taken against 57 \
         points of body budget nothing paints and reports a fit while the last \
         row sits under the footer"
    );
}

/// One render of the Browse aside, read two ways.
#[cfg(not(target_arch = "wasm32"))]
struct RenderedAside {
    /// The projection the aside was painted from. `records.rs` owns proving
    /// these numbers come from their owners; this module owns proving the
    /// aside states them.
    record: PlanCatalogRecord,
    /// Every string the aside painted, in paint order — section headings
    /// included, which carry no widget and so no accessibility node.
    painted: Vec<String>,
    /// Each property row as `(label, value)`, in paint order.
    rows: Vec<(String, String)>,
}

/// Render the aside for one catalog entry.
///
/// A row's value is elided to fit its column, so the painted glyphs are only
/// what survived the fit while the accessibility node carries the whole
/// fact. Neither source alone is enough: `property_row` publishes its node
/// without widget info, so egui never stamps bounds on it and the tree has
/// no order to sort by. So the order comes from the paint and the values
/// come from the nodes, which also means a label that is announced but never
/// painted does not count as a row.
#[cfg(not(target_arch = "wasm32"))]
fn rendered_aside(app: &RSpiceApp, plan_id: SimulationPlanId) -> RenderedAside {
    let records = plan_catalog_records(app);
    let selected = records
        .iter()
        .find(|record| record.id == plan_id)
        .expect("the requested plan is projected");
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(980.0, 760.0))),
            ..Default::default()
        },
        |root| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                // One column, so the eight rows land in one reading order
                // for the assertions below to be about.
                .show(root, |ui| selected_plan_properties(ui, selected, 1));
        },
    );
    let announced = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AccessKit aside tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| Some((node.label()?.to_owned(), node.value()?.to_owned())))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut painted = Vec::new();
    let mut rows = Vec::new();
    for shape in &output.shapes {
        if let egui::epaint::Shape::Text(text) = &shape.shape {
            let painted_text = text.galley.job.text.clone();
            if let Some(value) = announced.get(&painted_text) {
                rows.push((painted_text.clone(), value.clone()));
            }
            painted.push(painted_text);
        }
    }
    RenderedAside {
        record: selected.clone(),
        painted,
        rows,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn aside_value<'rows>(rows: &'rows [(String, String)], label: &str) -> &'rows str {
    rows.iter()
        .find(|(row_label, _)| row_label == label)
        .map_or_else(
            || panic!("the aside has no '{label}' row: {rows:?}"),
            |(_, value)| value.as_str(),
        )
}

/// The aside is the manager's only statement of what the selected plan
/// declares. It named the plan, its identity and its result count and stopped
/// there, so five facts the projection had already collected — the corner, the
/// forecast, the cost it models, the model closure, the pinned baseline, and the
/// source plan — were unreachable from the surface that exists to compare plans.
///
/// It is also the one list, so it must not repeat the table: the stable identity
/// is the second line of every identity cell and the result count is a column,
/// and a row for either would be the same fact painted twice on one screen.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_browse_aside_states_every_fact_the_catalog_owns_about_the_selected_plan() {
    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    app.state.workspace.migrate_active_plan_data(plan_id);
    let mut setup = app.state.sim_setup.clone();
    setup
        .set_reference_pvt(ProcessCorner::FF, -40.0)
        .expect("a physical reference corner");
    app.state.sim_setup = setup;

    let RenderedAside {
        record,
        painted,
        rows,
    } = rendered_aside(&app, plan_id);

    // One heading over one flat list, as the authored aside has.
    assert_eq!(
        painted
            .iter()
            .filter(|text| matches!(
                text.as_str(),
                "Selected plan" | "Declared work" | "Plan-owned records"
            ))
            .collect::<Vec<_>>(),
        ["Selected plan"],
        "the aside is grouped again; the authored one is a single list"
    );
    assert_eq!(
        rows.iter()
            .map(|(label, _)| label.as_str())
            .collect::<Vec<_>>(),
        [
            "Name",
            "Revision",
            "Reference PVT corner",
            "Declared run set",
            "Modelled cost",
            "Model closure",
            "Regression baseline",
            "Source lineage",
        ]
    );
    // Nothing the table already paints is repeated here.
    for repeated in ["Stable identity", "Results"] {
        assert!(
            !rows.iter().any(|(label, _)| label == repeated),
            "the aside repeats '{repeated}', which every table row already paints"
        );
    }

    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan");
    assert_eq!(
        aside_value(&rows, "Name"),
        app.state.sim_setup.active_plan_name().as_str()
    );
    assert_eq!(
        aside_value(&rows, "Revision"),
        plan.revision().get().to_string()
    );
    assert_eq!(aside_value(&rows, "Reference PVT corner"), "FF · -40.0 °C");

    // The declared workload is the forecast's, named as points and tasks.
    let points = record.point_count().expect("the default run set validates");
    let tasks = record.task_count().expect("the default run set validates");
    let run_set = aside_value(&rows, "Declared run set");
    assert!(
        run_set.contains("PVT point") && run_set.contains("task"),
        "{run_set}"
    );
    assert!(
        run_set.contains(&points.to_string()) && run_set.contains(&tasks.to_string()),
        "the run-set row must state the forecast's own counts: {run_set}"
    );
    // The cost is its own row, from the same forecast — the combined value did
    // not fit the aside's value column.
    let cost = aside_value(&rows, "Modelled cost");
    for modelled in [
        record.estimated_duration().expect("a validated forecast"),
        record.estimated_storage().expect("a validated forecast"),
    ] {
        assert!(
            cost.contains(&modelled),
            "the cost row must state the forecast's own cost: {cost}"
        );
    }

    let bindings = app.state.sim_setup.model_bindings.len();
    assert_eq!(
        aside_value(&rows, "Model closure"),
        format!("{bindings} binding{}", plural_suffix(bindings))
    );
    assert_eq!(
        aside_value(&rows, "Source lineage"),
        "root plan · no source"
    );
    assert_eq!(aside_value(&rows, "Regression baseline"), "none pinned");

    // The baseline is the payload's, not a fixed string: pinning a run has
    // to change what the row says.
    let run = crate::product::RunId::new();
    app.state
        .workspace
        .ensure_active_plan_data(plan_id)
        .regression_baseline_run = Some(run);
    assert_eq!(
        aside_value(&rendered_aside(&app, plan_id).rows, "Regression baseline"),
        format!("run {run}")
    );
}

/// A run set that does not validate carries no forecast, and every quantity
/// derived from it is absent with it. Zeros there would read as "this plan
/// declares no work", which is a different claim from work that cannot be
/// expanded — and the one the reader would act on.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_undeclarable_run_set_says_so_instead_of_reporting_zeros() {
    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let mut setup = app.state.sim_setup.clone();
    // Nested composition with a zero maximum depth is a declared run-space
    // error, so there is no workload to forecast.
    setup.run_set.composition.mode = crate::simulation::run_set::RunSetCompositionMode::Nested;
    setup.run_set.composition.maximum_depth = 0;
    app.state.sim_setup = setup;

    let RenderedAside { record, rows, .. } = rendered_aside(&app, plan_id);

    assert!(
        record.point_count().is_none(),
        "the fixture's run set must not validate for this case to mean anything"
    );
    // The declaration is the failure; the cost is merely absent with it. Two
    // error-toned rows saying the same words would read as two problems.
    assert_eq!(
        aside_value(&rows, "Declared run set"),
        RUN_SET_DOES_NOT_VALIDATE
    );
    assert_eq!(aside_value(&rows, "Modelled cost"), NO_MODELLED_COST);
    assert_ne!(
        NO_MODELLED_COST, RUN_SET_DOES_NOT_VALIDATE,
        "the cost restates the declaration's failure instead of its own absence"
    );
    for label in ["Declared run set", "Modelled cost"] {
        let value = aside_value(&rows, label);
        assert!(
            !value.contains('0'),
            "'{label}' reported a zero for an absent forecast: {value}"
        );
    }
    // The plan is still identified and its records are still stated: an
    // invalid run space is not a reason to stop describing the plan.
    assert_eq!(aside_value(&rows, "Name"), plan_name_of(&app, plan_id));
    assert_eq!(
        aside_value(&rows, "Reference PVT corner"),
        reference_pvt_label(app.state.sim_setup.reference_pvt)
    );
}

/// The catalog's own name for a plan, so a test asserting the aside states it
/// does not carry a second spelling of it.
#[cfg(not(target_arch = "wasm32"))]
fn plan_name_of(app: &RSpiceApp, plan_id: SimulationPlanId) -> String {
    plan_catalog_records(app)
        .iter()
        .find(|record| record.id == plan_id)
        .expect("the requested plan is projected")
        .name
        .clone()
}

/// A clone names the plan and revision it came from, and the source it
/// leaves behind keeps reporting its own corner. Both facts used to be
/// unreadable: the lineage had no row, and a stored plan had no accessor
/// for its reference point, so every inactive plan's corner read as absent.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_clone_names_its_source_and_the_source_keeps_its_own_reference_corner() {
    let mut app = RSpiceApp::test_instance();
    let source = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan");
    let source_id = source.id();
    let source_revision = source.revision();
    let source_corner = app.state.sim_setup.reference_pvt;
    let mut setup = app.state.sim_setup.clone();
    let outcome = setup
        .clone_active_plan(
            "Cloned characterization",
            crate::workbench::app_state::SimulationPlanCloneOptions::ALL_PLAN_CONTENTS,
        )
        .expect("the active plan clones");
    setup
        .set_reference_pvt(ProcessCorner::SS, 125.0)
        .expect("a physical reference corner");
    app.state.sim_setup = setup;

    let clone_rows = rendered_aside(&app, outcome.cloned_plan_id).rows;
    assert_eq!(
        aside_value(&clone_rows, "Source lineage"),
        format!("from {source_id} · revision {}", source_revision.get())
    );
    assert_eq!(
        aside_value(&clone_rows, "Reference PVT corner"),
        "SS · 125.0 °C"
    );

    let source_rows = rendered_aside(&app, source_id).rows;
    assert_eq!(
        aside_value(&source_rows, "Source lineage"),
        "root plan · no source"
    );
    assert_eq!(
        aside_value(&source_rows, "Reference PVT corner"),
        reference_pvt_label(source_corner),
        "the retained source reports the corner it was cloned at, not the \
         active plan's and not 'unknown'"
    );
}

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
    let mut app = RSpiceApp::test_instance();
    let mut setup = app.state.sim_setup.clone();
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
    app.state.sim_setup = setup;

    create::commit_create_plan(
        &mut app,
        "Inheriting plan",
        &crate::workbench::state::NewSimulationPlanDraft {
            reference_pvt: crate::simulation::run_set::ReferencePoint {
                process: ProcessCorner::SS,
                temperature_celsius: -40.0,
            },
            inherit_model_closure: true,
            inherit_solver_options: true,
            inherit_save_policy: true,
        },
    )
    .expect("the create transaction commits");

    let setup = &app.state.sim_setup;
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
        setup.options_draft.temp.parse::<f64>(),
        Ok(-40.0),
        "the draft states the plan's reference temperature"
    );
    let rebuilt = crate::simulation::dialog::OptionsDialogState::from_options(&setup.options);
    assert_eq!(
        setup.options_draft.reltol, rebuilt.reltol,
        "the draft states the inherited options, not the defaults create_plan minted"
    );
    assert_ne!(
        rebuilt.reltol,
        crate::simulation::dialog::OptionsDialogState::from_options(
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
    let mut app = RSpiceApp::test_instance();
    let mut setup = app.state.sim_setup.clone();
    setup.options.reltol = 3.5e-6;
    setup.save_policy.retained_dataset_limit = 42;
    setup
        .model_bindings
        .push(crate::state::model_library::SimulationPlanModelBinding {
            library_name: "foundry-models".to_owned(),
            source_digest: crate::product::ContentDigest::from_bytes([3; 32]),
            selected_corner: Some("TT".to_owned()),
        });
    app.state.sim_setup = setup;

    create::commit_create_plan(
        &mut app,
        "Default root plan",
        &crate::workbench::state::NewSimulationPlanDraft::default(),
    )
    .expect("the create transaction commits");

    let setup = &app.state.sim_setup;
    let defaults = crate::simulation::run_set::ReferencePoint::default();
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
    let mut app = RSpiceApp::test_instance();
    let first = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let mut setup = app.state.sim_setup.clone();
    let second = setup
        .create_plan("Retired sweep")
        .expect("a fresh root plan is created");
    let active = setup
        .create_plan("Corner characterization")
        .expect("a second fresh root plan is created");
    app.state.sim_setup = setup;
    let records = plan_catalog_records(&app);

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

/// Which authored facts have no owner here, and the guard that keeps them out
/// of every route. Its own file: it is a statement about all six surfaces at
/// once, and it is the larger half of the text either way.
#[cfg(not(target_arch = "wasm32"))]
mod register;

/// Every route the manager dispatches to, in dispatch order.
///
/// [`route_coverage`] is what keeps this complete: it matches the mode enum
/// exhaustively and indexes this array, so a ninth mode needs an arm here, and
/// an arm returning an index this array does not have is a compile error rather
/// than a test that quietly stops covering one route.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const EVERY_ROUTE: [SimulationPlanManagerMode; 8] = [
    SimulationPlanManagerMode::Browse,
    SimulationPlanManagerMode::Create,
    SimulationPlanManagerMode::Rename,
    SimulationPlanManagerMode::ConfirmArchive,
    SimulationPlanManagerMode::Compare,
    SimulationPlanManagerMode::Export,
    SimulationPlanManagerMode::Import,
    SimulationPlanManagerMode::Campaign,
];

/// Everything one route said at one viewport, in both channels.
///
/// Both are needed and neither is enough. A cell elides to its column, so the
/// paint carries only what survived the fit; a section heading carries no
/// widget, so the accessibility tree never hears it. A fact that came back in
/// either channel came back.
#[cfg(not(target_arch = "wasm32"))]
struct RouteSpeech {
    /// Every string the route painted, in paint order.
    painted: Vec<String>,
    /// Every label, value and description the route published to AccessKit.
    announced: Vec<String>,
}

/// Render one route through the real shell and collect everything it said.
#[cfg(not(target_arch = "wasm32"))]
fn route_speech(screen: Vec2, mode: SimulationPlanManagerMode) -> RouteSpeech {
    let (mut app, active, available, _) = app_with_every_lifecycle_state();
    let draft = route_draft(&app, mode, active, available);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    // The dialog owns the draft for a frame and the shell re-arms it, so each
    // pass is handed its own copy and the three passes render one state rather
    // than accumulating it.
    let mut pass = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, screen)),
                ..Default::default()
            },
            |ctx| plan_manager_dialog(ctx, &mut app, draft.clone()),
        )
    };
    let _ = pass();
    let _ = pass();
    let output = pass();

    let painted = output
        .shapes
        .iter()
        .filter_map(|shape| match &shape.shape {
            egui::epaint::Shape::Text(text) => Some(text.galley.job.text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut announced = Vec::new();
    if let Some(update) = output.platform_output.accesskit_update.as_ref() {
        for (_, node) in &update.nodes {
            announced.extend(node.label().map(str::to_owned));
            announced.extend(node.value().map(str::to_owned));
            announced.extend(node.description().map(str::to_owned));
        }
    }
    RouteSpeech { painted, announced }
}

// ---------------------------------------------------------------------------
// Route coverage
// ---------------------------------------------------------------------------

/// What covers one route, named.
///
/// Five lanes rebuilt five routes and each wrote its own tests where it was
/// working. That is the right place for them and it leaves no one able to
/// answer "is every route covered?" — the answer was spread over six files, and
/// a route whose only test was deleted would go on passing everything else.
///
/// Every field here is a claim that a specific thing exists, checked against
/// the source. The names are not documentation of the tests; they are the
/// coverage itself.
#[cfg(not(target_arch = "wasm32"))]
struct RouteCoverage {
    /// The route this covers. Checked against the position it was found at, so
    /// an arm cannot claim another route's coverage.
    mode: SimulationPlanManagerMode,
    /// The function the shell's dispatch match hands this mode to.
    dispatch: &'static str,
    /// The test that proves the route fits every gated viewport without
    /// scrolling. Always in this file: fit is the shell's rule, not a route's.
    fit_gate: &'static str,
    /// The file and test that prove no fact this route states is lost to
    /// elision. Three routes gate this themselves, by asserting their own exact
    /// strings are painted whole; the rest are held to the weaker but general
    /// claim next door, that anything shortened is announced whole somewhere.
    elision_gate: (&'static str, &'static str),
    /// The file and test that exercise what this route actually does — its
    /// transaction where it has one, its content where it does not. Compare is
    /// read-only by design and its named test is what proves that, which is a
    /// test of what the route does and not an absence of one.
    behaviour: (&'static str, &'static str),
}

/// What covers `mode`.
///
/// Exhaustive on purpose. A ninth mode cannot be added without an arm here, and
/// the arm has to index a slot [`ROUTE_COVERAGE`] actually has — indexing a
/// fixed-length array past its end with a literal does not compile. So the
/// compiler, not a reviewer, is what notices a route that arrived without
/// coverage.
///
/// What the compiler cannot notice is a ninth mode left out of [`EVERY_ROUTE`]
/// while its arm here reuses an existing slot; the test below closes that by
/// checking every slot answers for its own mode.
#[cfg(not(target_arch = "wasm32"))]
fn route_coverage(mode: SimulationPlanManagerMode) -> &'static RouteCoverage {
    &ROUTE_COVERAGE[match mode {
        SimulationPlanManagerMode::Browse => 0,
        SimulationPlanManagerMode::Create => 1,
        SimulationPlanManagerMode::Rename => 2,
        SimulationPlanManagerMode::ConfirmArchive => 3,
        SimulationPlanManagerMode::Compare => 4,
        SimulationPlanManagerMode::Export => 5,
        SimulationPlanManagerMode::Import => 6,
        SimulationPlanManagerMode::Campaign => 7,
    }]
}

#[cfg(not(target_arch = "wasm32"))]
const ROUTE_COVERAGE: [RouteCoverage; 8] = [
    RouteCoverage {
        mode: SimulationPlanManagerMode::Browse,
        dispatch: "browse_dialog(",
        fit_gate: "the_browse_route_fits_every_gated_viewport",
        elision_gate: ("tests.rs", "the_browse_route_loses_no_fact_to_elision"),
        behaviour: ("tests.rs", "every_row_announces_the_facts_its_cells_paint"),
    },
    RouteCoverage {
        mode: SimulationPlanManagerMode::Create,
        dispatch: "create::dialog(",
        fit_gate: "the_create_route_fits_every_gated_viewport",
        elision_gate: (
            "create.rs",
            "every_stated_fact_paints_whole_at_every_gated_viewport",
        ),
        behaviour: (
            "create.rs",
            "each_of_the_four_inputs_reaches_the_stored_plan",
        ),
    },
    RouteCoverage {
        mode: SimulationPlanManagerMode::Rename,
        dispatch: "lifecycle::dialog(",
        fit_gate: "the_rename_route_fits_every_gated_viewport",
        elision_gate: ("tests.rs", "the_rename_route_loses_no_fact_to_elision"),
        behaviour: (
            "lifecycle.rs",
            "a_rename_preserves_the_identity_the_revision_and_the_result_references",
        ),
    },
    RouteCoverage {
        mode: SimulationPlanManagerMode::ConfirmArchive,
        dispatch: "lifecycle::dialog(",
        fit_gate: "the_archive_confirmation_fits_every_gated_viewport",
        elision_gate: (
            "tests.rs",
            "the_archive_confirmation_loses_no_fact_to_elision",
        ),
        behaviour: (
            "lifecycle.rs",
            "archiving_retains_the_configuration_and_every_result_reference",
        ),
    },
    RouteCoverage {
        mode: SimulationPlanManagerMode::Compare,
        dispatch: "compare::dialog(",
        fit_gate: "the_compare_route_fits_every_gated_viewport",
        elision_gate: (
            "compare.rs",
            "every_fact_the_comparison_states_is_painted_whole_at_every_gated_viewport",
        ),
        behaviour: (
            "compare.rs",
            "a_comparison_leaves_both_plans_exactly_as_they_were",
        ),
    },
    RouteCoverage {
        mode: SimulationPlanManagerMode::Export,
        dispatch: "exchange::dialog(",
        fit_gate: "the_export_route_fits_every_gated_viewport",
        elision_gate: ("tests.rs", "the_export_route_loses_no_fact_to_elision"),
        behaviour: (
            "exchange.rs",
            "the_envelope_admits_only_this_format_and_this_version",
        ),
    },
    RouteCoverage {
        mode: SimulationPlanManagerMode::Import,
        dispatch: "exchange::dialog(",
        fit_gate: "the_import_route_fits_every_gated_viewport",
        elision_gate: ("tests.rs", "the_import_route_loses_no_fact_to_elision"),
        behaviour: (
            "exchange.rs",
            "the_previewed_specification_count_is_what_the_import_creates",
        ),
    },
    RouteCoverage {
        mode: SimulationPlanManagerMode::Campaign,
        dispatch: "campaign::dialog(",
        fit_gate: "the_campaign_route_fits_every_gated_viewport",
        elision_gate: (
            "campaign.rs",
            "every_fact_the_notes_state_is_painted_whole_at_every_gated_viewport",
        ),
        behaviour: (
            "campaign.rs",
            "the_commit_walks_the_members_in_the_declared_order",
        ),
    },
];

/// Every source file of the plan manager.
///
/// Two claims read it: the coverage one below, which looks for a named test,
/// and the elision gate, which asks whether a string ending in an ellipsis is
/// one the module wrote that way. `records.rs` keeps a list of the same files
/// for its duplication guards; that module is not this wave's and the two are
/// not merged here.
#[cfg(not(target_arch = "wasm32"))]
const PLAN_MANAGER_SOURCES: &[(&str, &str)] = &[
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

/// Whether `file` declares `name` as a test.
///
/// Scanned line by line rather than by an offset into the text: the working
/// copy may be checked out with either line ending, and a scan keyed on `\n\n`
/// finds nothing under CRLF and reports every test as missing.
///
/// The `#[test]` has to be in the same attribute block as the function — a
/// blank line resets — so a helper that merely sits below some other test does
/// not satisfy a coverage claim.
#[cfg(not(target_arch = "wasm32"))]
fn declares_test(file: &str, name: &str) -> bool {
    let source = PLAN_MANAGER_SOURCES
        .iter()
        .find(|(named, _)| *named == file)
        .map(|(_, source)| *source)
        .unwrap_or_else(|| {
            panic!("the coverage claim names {file}, which is not a scanned source")
        });
    let signature = format!("fn {name}(");
    let mut attributed = false;
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() {
            attributed = false;
        } else if line == "#[test]" {
            attributed = true;
        } else if line.starts_with(&signature) {
            return attributed;
        }
    }
    false
}

/// Every route is dispatched, gated for fit, gated for elision, and tested for
/// what it does — and stays that way.
///
/// The claim this makes is not "these tests pass". It is that they exist and
/// are reachable as tests, checked against the source of the six files they
/// live in. A route whose behaviour test was deleted in a refactor, or renamed
/// without the coverage claim following, fails here — which is the failure the
/// deletion should have caused and did not.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_route_is_dispatched_gated_and_tested() {
    let shell = crate::source_guard::production_source(include_str!("../plan_manager.rs"));
    // The dispatch match alone. Searching the whole shell would let a mention
    // in a doc comment or a neighbouring function stand in for an arm.
    let arms = shell
        .lines()
        .skip_while(|line| !line.contains("match draft.mode {"))
        .take_while(|line| line.trim() != "};")
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        arms.contains("match draft.mode {"),
        "the shell no longer dispatches on `draft.mode`, so this claim is \
         scanning nothing"
    );

    for (position, coverage) in ROUTE_COVERAGE.iter().enumerate() {
        let mode = coverage.mode;
        assert_eq!(
            EVERY_ROUTE[position], mode,
            "ROUTE_COVERAGE[{position}] answers for {mode:?}, which is not the \
             route EVERY_ROUTE holds there"
        );
        assert!(
            std::ptr::eq(route_coverage(mode), coverage),
            "{mode:?} resolves to another route's coverage entry"
        );

        assert!(
            arms.contains(&format!("SimulationPlanManagerMode::{mode:?}")),
            "{mode:?} has no arm in the shell's dispatch match, so nothing the \
             rest of this claim names is reachable"
        );
        assert!(
            arms.contains(coverage.dispatch),
            "{mode:?} is dispatched, but no longer to `{}`",
            coverage.dispatch
        );
        assert!(
            declares_test("tests.rs", coverage.fit_gate),
            "{mode:?} has no fit gate: tests.rs declares no `{}`. Every route \
             is held to the product rule that a dialog does not scroll.",
            coverage.fit_gate
        );
        let (file, name) = coverage.elision_gate;
        assert!(
            declares_test(file, name),
            "{mode:?} has no elision gate: {file} declares no `{name}`. Fit \
             cannot see a truncated value — a label elided to `Corner char…` \
             sits inside its clip rect and passes."
        );
        let (file, name) = coverage.behaviour;
        assert!(
            declares_test(file, name),
            "{mode:?} has no test of what it does: {file} declares no `{name}`. \
             A route can fit perfectly and commit the wrong transaction."
        );
    }
}

// ---------------------------------------------------------------------------
// Elision gates for the routes that do not gate themselves
// ---------------------------------------------------------------------------

/// Whether the module ships `text` exactly as painted, ellipsis included.
///
/// An ellipsis means two different things in this dialog and they do not differ
/// in the glyphs. `Rename…` and the filter's `Name, identity, revision, or run
/// set…` end in one because their author wrote one: the affordance mark on an
/// action that opens something, and a placeholder's trailing form. `Corner
/// characteriz…` ends in one because [`elide_text`] cut it to a column. Only
/// the second kind can cost the reader a fact, and the only thing that tells
/// them apart is whether the whole string is one the module ships — so the
/// shipped sources are the discriminator, not a list of exceptions maintained
/// beside the gate.
///
/// [`elide_text`]: crate::workbench::design_system::elide_text
#[cfg(not(target_arch = "wasm32"))]
fn is_authored_whole(text: &str) -> bool {
    PLAN_MANAGER_SOURCES
        .iter()
        .any(|(_, source)| crate::source_guard::production_source(source).contains(text))
}

/// A route shortens nothing without announcing it whole.
///
/// The weaker of the two elision claims in this module, and the one that suits
/// a surface which elides on purpose. Create, Compare and Campaign each list
/// their own exact strings and demand every one be painted whole; Browse cannot
/// make that claim, because its table elides every cell to its column by
/// design and the row's accessibility node is what carries the whole fact.
///
/// So the claim here is the one that is true of both kinds of surface: nothing
/// is *lost*. Every shortened string is announced somewhere in full, which is
/// exactly the condition under which elision costs the reader nothing they
/// cannot recover.
///
/// A string elided down to the ellipsis alone fails outright — there is no
/// prefix left to match, and a cell painting nothing but `…` is unreadable
/// whatever the tree says.
#[cfg(not(target_arch = "wasm32"))]
fn assert_route_loses_no_fact_to_elision(mode: SimulationPlanManagerMode) {
    for (arrangement, screen) in GATED_VIEWPORTS {
        let RouteSpeech { painted, announced } = route_speech(screen, mode);
        assert!(
            !painted.is_empty(),
            "{mode:?} in {arrangement} painted nothing at all"
        );
        for text in painted
            .iter()
            .filter(|text| text.ends_with('\u{2026}') && !is_authored_whole(text))
        {
            let prefix = text.trim_end_matches('\u{2026}');
            assert!(
                !prefix.is_empty(),
                "{mode:?} in {arrangement} painted a value elided to nothing but \
                 an ellipsis, so its box is too narrow to hold any of it"
            );
            // Carried, not led. A table row announces every cell in one node —
            // `name · identity · lifecycle · revision …` — so an elided
            // identity sits in the middle of the string that carries it, and a
            // check anchored at the start would find nothing. What has to be
            // true is that the shortened text goes on somewhere it can be read:
            // an announcement that stops exactly where the paint stopped is the
            // one that has lost the rest.
            assert!(
                announced
                    .iter()
                    .any(|whole| whole.contains(prefix) && !whole.ends_with(prefix)),
                "{mode:?} in {arrangement} shortened {:?} and no announcement \
                 carries more of it than the paint does, so the rest of that \
                 fact is unreachable — not merely unread. Either the box has to \
                 hold it or the widget has to publish it.",
                clipped_label_preview(text)
            );
        }
    }
}

/// A table whose rows select says so; a table that only states says nothing.
///
/// Both tables come out of `kit`, and until the call sites said which they were,
/// the read-only one inherited the selectable one's whole treatment: a hover
/// fill under a row that cannot be picked, and every row published as a
/// selectable item. Two promises the comparison cannot keep, and neither is
/// visible to a fit gate — the geometry is identical either way.
///
/// So the claim is made as a difference between the two routes rather than
/// against a role name. Nothing here names how egui spells "selectable": the
/// browse table defines that spelling at run time and the comparison is required
/// not to use it. An egui release that changed the mapping would keep this test
/// meaningful; a test naming the role would quietly stop checking.
///
/// The row's text has to be looked for in two places, and that is egui's rule
/// rather than a hedge: a node whose role is `Label` carries its text as the
/// node's *value*, and every other role carries it as the node's *label*. So a
/// search of one field alone would find the browse rows and conclude the
/// comparison announces nothing at all — which is the opposite of what changed.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn only_a_table_whose_rows_select_announces_a_selectable_row() {
    // A row's own accessibility bounds are where the pointer has to go. A
    // coordinate worked out from the layout here would be a second, disagreeing
    // account of where the table is.
    let rows_of = |output: &egui::FullOutput, needle: &'static str| {
        let announced = |node: &egui::accesskit::Node| {
            node.label()
                .or_else(|| node.value())
                .is_some_and(|text| text.contains(needle))
        };
        let rows = output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("accessibility tree")
            .nodes
            .iter()
            .filter(|(_, node)| announced(node))
            .map(|(_, node)| {
                let bounds = node
                    .bounds()
                    .unwrap_or_else(|| panic!("a {needle:?} row has no bounds"));
                (
                    node.role(),
                    node.toggled(),
                    egui::pos2(
                        ((bounds.x0 + bounds.x1) / 2.0) as f32,
                        ((bounds.y0 + bounds.y1) / 2.0) as f32,
                    ),
                )
            })
            .collect::<Vec<_>>();
        assert!(
            !rows.is_empty(),
            "nothing announces a row containing {needle:?}, so the table \
             publishes no row at all and this test would pass a surface a \
             screen reader cannot read"
        );
        rows
    };
    // Counted, not colour-matched. A hover fill is one filled rectangle, and
    // that is measurable; its colour is not, because the dialog's `Area` is
    // still fading in at the third pass and egui multiplies every painted
    // colour by that opacity — a headless context never advances far enough to
    // settle it, so a token compared by value never matches what was painted.
    let filled_rectangles = |output: &egui::FullOutput| {
        output
            .shapes
            .iter()
            .filter(|shape| matches!(&shape.shape, egui::epaint::Shape::Rect(_)))
            .count()
    };
    let render = |mode: SimulationPlanManagerMode, pointer: Option<egui::Pos2>| {
        let (mut app, active, available, _) = app_with_every_lifecycle_state();
        let draft = route_draft(&app, mode, active, available);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        // The pointer is held at the same place for every pass. egui decides
        // whether a widget is hovered against the layout of the frame before,
        // so a pointer that arrives only on the last pass is a pointer over a
        // table that did not exist yet.
        let events = || {
            pointer
                .map(egui::Event::PointerMoved)
                .into_iter()
                .collect::<Vec<_>>()
        };
        let mut pass = || {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, REAL_VIEWPORT)),
                    events: events(),
                    ..Default::default()
                },
                |ctx| plan_manager_dialog(ctx, &mut app, draft.clone()),
            )
        };
        let _ = pass();
        let _ = pass();
        pass()
    };

    // " · revision " belongs to a plan row's announcement, " · compared by " to
    // a comparison domain's. Both are built by their route from the same table.
    let browse = render(SimulationPlanManagerMode::Browse, None);
    let plan_rows = rows_of(&browse, " · revision ");
    let (selectable, _, _) = plan_rows[0];
    // Both states have to be present, or the reference is not a selectable
    // table: a row that announces no selectedness at all is the thing the
    // comparison is supposed to look like.
    for state in [
        egui::accesskit::Toggled::True,
        egui::accesskit::Toggled::False,
    ] {
        assert!(
            plan_rows
                .iter()
                .any(|(_, selectedness, _)| *selectedness == Some(state)),
            "no plan row announces itself as {state:?}, so the browse table is \
             not the selectable reference this test measures against"
        );
    }
    // The unselected one. A selected row paints its own fill and takes the
    // branch before the hover one, so hovering it could never show a hover
    // fill and the measurement below would read as suppression.
    let (_, _, plan_row) = *plan_rows
        .iter()
        .find(|(_, selectedness, _)| *selectedness == Some(egui::accesskit::Toggled::False))
        .expect("a row that is not the selected one");
    let compare = render(SimulationPlanManagerMode::Compare, None);
    let domain_rows = rows_of(&compare, " · compared by ");
    assert_eq!(
        domain_rows.len(),
        4,
        "the four comparison domains are not four announced rows"
    );
    for (role, selectedness, _) in &domain_rows {
        assert_ne!(
            *role, selectable,
            "a comparison domain row is announced in the same role as a plan \
             row the reader can pick, so nothing was gained by asking for the \
             read-only treatment"
        );
        assert_eq!(
            *selectedness, None,
            "a comparison domain row publishes a selected state, so a screen \
             reader offers a selection that cannot be made"
        );
    }
    let (_, _, domain_row) = domain_rows[0];

    let hovered = render(SimulationPlanManagerMode::Browse, Some(plan_row));
    assert_eq!(
        filled_rectangles(&hovered),
        filled_rectangles(&browse) + 1,
        "the pointer over a selectable plan row painted no hover fill, so this \
         test cannot tell a table that suppresses one from a table that never \
         had one"
    );
    let hovered = render(SimulationPlanManagerMode::Compare, Some(domain_row));
    assert_eq!(
        filled_rectangles(&hovered),
        filled_rectangles(&compare),
        "the pointer over a comparison domain row painted a fill it does not \
         paint idle, which offers the reader a click that does nothing"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_browse_route_loses_no_fact_to_elision() {
    assert_route_loses_no_fact_to_elision(SimulationPlanManagerMode::Browse);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_rename_route_loses_no_fact_to_elision() {
    assert_route_loses_no_fact_to_elision(SimulationPlanManagerMode::Rename);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_archive_confirmation_loses_no_fact_to_elision() {
    assert_route_loses_no_fact_to_elision(SimulationPlanManagerMode::ConfirmArchive);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_export_route_loses_no_fact_to_elision() {
    assert_route_loses_no_fact_to_elision(SimulationPlanManagerMode::Export);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_import_route_loses_no_fact_to_elision() {
    assert_route_loses_no_fact_to_elision(SimulationPlanManagerMode::Import);
}
