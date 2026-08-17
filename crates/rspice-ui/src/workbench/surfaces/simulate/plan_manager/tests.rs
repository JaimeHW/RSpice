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
                    .size(DialogSize::WideWorkflow)
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
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(body.x, 4_000.0),
                )),
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

/// Every string the real dialog painted at one viewport, split by whether it
/// actually landed on screen.
///
/// The body-level height measurement proves the content is no taller than
/// the budget. It cannot prove the dialog put it somewhere the reader can
/// see: egui still emits a shape its clip rect excludes, so a label scrolled
/// under the footer is painted and invisible at once. A text shape counts as
/// visible only when its whole box is inside the clip rect it was painted
/// with — the same question as "would the user have to scroll to read this".
#[cfg(not(target_arch = "wasm32"))]
fn dialog_visible_text(screen: Vec2, extra_plans: usize) -> (Vec<String>, Vec<String>) {
    let (mut app, active, _, _) = app_with_every_lifecycle_state();
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
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut pass = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, screen)),
                ..Default::default()
            },
            |ctx| {
                plan_manager_dialog(
                    ctx,
                    &mut app,
                    SimulationPlanManagerDraft::new(active, "Corner characterization"),
                );
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
fn app_with_every_lifecycle_state() -> (
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
            "Declared work",
            "Plan-owned records",
            "Name",
            "Stable identity",
            "Revision",
            "Reference PVT corner",
            "Declared run set",
            "Model closure",
            "Variables, outputs, specifications",
            "Source lineage",
            "Runs referencing this plan",
            "Switching is atomic",
            "Results are references",
            "Stable identity retained",
        ];
        // A property label is elided to its column, so the painted string
        // may be a prefix plus an ellipsis. Only an actually-elided string
        // is allowed to match by prefix: without that rule the "Stable
        // identity" row would satisfy "Stable identity retained" and the
        // note could go missing unnoticed.
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
        // point of selecting a row, and the eleven detail rows are the
        // plan's facts.
        //
        // And fitting the fixture is not enough. The records table is the
        // one unbounded thing here — a catalog with more plans is a taller
        // table — so the same claim has to hold with more plans than the
        // fixture, or the surface overflows on the user's next plan rather
        // than on a later edit to this file.
        for extra_plans in [0, 2] {
            let (visible, clipped) = dialog_visible_text(screen, extra_plans);
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
/// not co-occur. Here the detail spends the full width on three side-by-side
/// groups, which is what buys back the height stacking costs.
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
        let (visible, clipped) = dialog_visible_text(screen, extra_plans);
        for label in [
            "Rename…",
            "Clone…",
            "Compare…",
            "Export…",
            "Archive…",
            "Selected plan",
            "Declared work",
            "Plan-owned records",
            "Switching is atomic",
            "Results are references",
            "Stable identity retained",
        ] {
            let shows = |painted: &[String]| {
                painted.iter().any(|text| {
                    text == label
                        || text.strip_suffix('\u{2026}').is_some_and(|prefix| {
                            !prefix.is_empty() && label.starts_with(prefix)
                        })
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
    for absent in ["Design / testbench binding", "Modified", "Testbench"] {
        assert!(
            !rendered.painted.iter().any(|text| text == absent),
            "the table paints '{absent}', which no RSpice plan owns"
        );
    }
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
    for unowned in ["permission", "entitlement", "schema migration", "dirty editor"] {
        assert!(
            !notes.contains(unowned),
            "a boundary note cites '{unowned}', which nothing in this module \
             checks or reports"
        );
    }
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
    lifecycle::commit_restore_plan(&mut app.state.sim_setup, retired).expect("an archived plan restores");
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
#[test]
fn the_shell_composes_the_geometry_these_tests_measure() {
    let shell = crate::source_guard::production_source(include_str!("../plan_manager.rs"));
    for required in [
        "DialogSize::WideWorkflow",
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
                // One column, so the eleven rows land in one reading order
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

/// The aside is the manager's only statement of what the selected plan is
/// and what it declares. It named the plan, its identity and its result
/// count and stopped there, so six facts the projection had already
/// collected — the corner, the forecast, the model closure, the plan-owned
/// record counts, the pinned baseline, and the source plan — were
/// unreachable from the surface that exists to compare plans.
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

    // Eleven rows are grouped under three headings rather than listed flat.
    assert_eq!(
        painted
            .iter()
            .filter(|text| matches!(
                text.as_str(),
                "Selected plan" | "Declared work" | "Plan-owned records"
            ))
            .collect::<Vec<_>>(),
        ["Selected plan", "Declared work", "Plan-owned records"]
    );
    assert_eq!(
        rows.iter()
            .map(|(label, _)| label.as_str())
            .collect::<Vec<_>>(),
        [
            "Name",
            "Stable identity",
            "Revision",
            "Reference PVT corner",
            "Declared run set",
            "Model closure",
            "Variables, outputs, specifications",
            "Source lineage",
            "Runs referencing this plan",
        ]
    );

    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan");
    assert_eq!(
        aside_value(&rows, "Name"),
        app.state.sim_setup.active_plan_name().as_str()
    );
    assert_eq!(aside_value(&rows, "Stable identity"), plan_id.to_string());
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
    // The same row carries the modelled cost, from the same forecast.
    for cost in [
        record.estimated_duration().expect("a validated forecast"),
        record.estimated_storage().expect("a validated forecast"),
    ] {
        assert!(
            run_set.contains(&cost),
            "the run-set row must state the forecast's own cost: {run_set}"
        );
    }

    let bindings = app.state.sim_setup.model_bindings.len();
    assert_eq!(
        aside_value(&rows, "Model closure"),
        format!("{bindings} binding{}", plan_plural_suffix(bindings))
    );
    assert_eq!(
        aside_value(&rows, "Variables, outputs, specifications"),
        format!(
            "{} · {} · {}",
            record.design_variables, record.saved_outputs, record.specifications
        )
    );
    assert_eq!(
        aside_value(&rows, "Source lineage"),
        "root plan · no source"
    );
    assert_eq!(
        aside_value(&rows, "Runs referencing this plan"),
        format!(
            "{} immutable reference{} · no baseline pinned",
            record.results,
            plan_plural_suffix(record.results)
        )
    );

    // The baseline is the payload's, not a fixed string: pinning a run has
    // to change what the row says.
    let run = crate::product::RunId::new();
    app.state
        .workspace
        .ensure_active_plan_data(plan_id)
        .regression_baseline_run = Some(run);
    assert_eq!(
        aside_value(
            &rendered_aside(&app, plan_id).rows,
            "Runs referencing this plan"
        ),
        format!(
            "{} immutable reference{} · baseline run {run}",
            record.results,
            plan_plural_suffix(record.results)
        )
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
    setup.run_set.composition.mode =
        crate::simulation::run_set::RunSetCompositionMode::Nested;
    setup.run_set.composition.maximum_depth = 0;
    app.state.sim_setup = setup;

    let RenderedAside { record, rows, .. } = rendered_aside(&app, plan_id);

    assert!(
        record.point_count().is_none(),
        "the fixture's run set must not validate for this case to mean anything"
    );
    for label in ["Declared run set"] {
        let value = aside_value(&rows, label);
        assert_eq!(value, RUN_SET_DOES_NOT_VALIDATE);
        assert!(
            !value.contains('0'),
            "'{label}' reported a zero for an absent forecast: {value}"
        );
    }
    // The plan is still identified and its records are still stated: an
    // invalid run space is not a reason to stop describing the plan.
    assert_eq!(aside_value(&rows, "Stable identity"), plan_id.to_string());
    assert_eq!(
        aside_value(&rows, "Reference PVT corner"),
        reference_pvt_label(app.state.sim_setup.reference_pvt)
    );
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
    assert_eq!(aside_value(&clone_rows, "Reference PVT corner"), "SS · 125.0 °C");

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
    assert_eq!(setup.inactive_plans().last().map(|plan| plan.model_bindings().len()), Some(1));
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
