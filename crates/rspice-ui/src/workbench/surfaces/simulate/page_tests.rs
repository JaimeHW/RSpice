//! Contract tests for the Simulation Studio setup pages.
//!
//! These check what the pages promise rather than how they paint: every route
//! renders without an egui identity clash, the page's own subject reaches the
//! screen, and the numerical controls resolve to exactly the values the run
//! will use.

use egui::{Rect, vec2};

use super::page_kit::column_rects;
use super::pages;
use crate::simulation::dialog::{OptionsDialogState, SimulationOptions};
use crate::workbench::RSpiceApp;
use crate::workbench::state::SimulationPage;

/// Render one page and collect the text it painted.
fn render(page: SimulationPage, width: f32) -> String {
    render_with(page, width, |_| {})
}

/// Render one page over an app the caller has seeded first.
///
/// A registry page shows its record editor only once something is selected, so
/// a test that wants to see those controls has to put a record in the plan and
/// select it before the frame runs.
fn render_with(page: SimulationPage, width: f32, seed: impl FnOnce(&mut RSpiceApp)) -> String {
    fn collect(shape: &egui::epaint::Shape, rendered: &mut String) {
        match shape {
            egui::epaint::Shape::Text(text) => {
                rendered.push_str(&text.galley.job.text);
                rendered.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, rendered);
                }
            }
            _ => {}
        }
    }

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.simulation_page = page;
    seed(&mut app);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(width, 1400.0))),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| pages::show(ui, &mut app, page));
                });
        },
    );
    let mut rendered = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut rendered);
    }
    rendered
}

/// The active plan's identity, for tests that seed plan-owned records.
fn plan_id(app: &RSpiceApp) -> crate::product::SimulationPlanId {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance has a stable plan")
        .id()
}

/// Every field the record editors expose has to reach the screen as a control,
/// not just exist in the model. These two pages are the only place an authored
/// record's contract can be changed, so a field that silently stopped painting
/// would take its capability with it.
#[test]
fn a_selected_record_paints_every_field_of_its_contract() {
    use crate::state::{
        DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableScope,
        DesignVariableSweepEligibility, SavedOutput, SavedOutputCompatibility, SavedOutputKind,
        SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
    };

    let rendered = render_with(SimulationPage::Outputs, 1200.0, |app| {
        let id = plan_id(app);
        let output = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            "vout",
            "V(out)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("valid saved output");
        app.state
            .workspace
            .add_saved_output(id, output)
            .expect("the plan accepts it");
        app.state.workbench.selected_saved_output = Some("vout".to_owned());
    });
    for field in [
        "Name",
        "Expression",
        "Applies to",
        "Save policy",
        "Stored precision",
        "Streaming",
    ] {
        assert!(
            rendered.contains(field),
            "the selected output's {field} control did not reach the screen:\n{rendered}"
        );
    }

    let rendered = render_with(SimulationPage::Variables, 1200.0, |app| {
        let id = plan_id(app);
        let variable = DesignVariable::new(
            "rload",
            "1kohm",
            DesignVariableQuantity::Resistance,
            DesignVariableScope::Testbench,
            "load resistance",
            None,
            DesignVariableSweepEligibility::FixedParameter,
            DesignVariableOverridePolicy::InheritOwnerOnly,
        )
        .expect("valid design variable");
        app.state
            .workspace
            .add_design_variable(id, variable)
            .expect("the plan accepts it");
        app.state.workbench.selected_design_variable = Some("rload".to_owned());
    });
    for field in [
        "Expression",
        "Quantity",
        "Scope",
        "Sweep role",
        "Override policy",
        "Minimum",
        "Maximum",
    ] {
        assert!(
            rendered.contains(field),
            "the selected variable's {field} control did not reach the screen:\n{rendered}"
        );
    }
    assert!(
        rendered.contains("unbounded"),
        "an unbounded variable must say that preflight cannot reject an out-of-range value:\n\
         {rendered}"
    );
}

#[test]
fn every_setup_route_renders_its_own_heading_without_an_identity_clash() {
    for page in SimulationPage::NAVIGATION {
        if page == SimulationPage::Analyses {
            continue;
        }
        for width in [720.0, 1200.0] {
            let rendered = render(page, width);
            assert!(
                rendered.contains(page.title()),
                "{page:?} at {width}px did not render its own title:\n{rendered}"
            );
            assert!(
                !rendered.contains("First use of")
                    && !rendered.contains("Second use of")
                    && !rendered.contains("Double use of"),
                "{page:?} rendered an egui ID-clash diagnostic at {width}px:\n{rendered}"
            );
        }
    }
}

#[test]
fn the_solver_page_states_both_acceptance_criteria_and_the_whole_ladder() {
    let rendered = render(SimulationPage::Solver, 1200.0);
    for symbol in [
        "RELTOL",
        "VNTOL",
        "ABSTOL",
        "CHGTOL",
        "RESIDUAL_RELTOL",
        "IABSTOL",
        "ITL1",
        "ITL4",
    ] {
        assert!(
            rendered.contains(symbol),
            "the solver page did not state {symbol}:\n{rendered}"
        );
    }
    for stage in [
        "Damped Newton",
        "Adaptive GMIN stepping",
        "Source stepping",
        "Pseudo-transient continuation",
        "Arc-length continuation",
    ] {
        assert!(
            rendered.contains(stage),
            "the continuation ladder omitted {stage}:\n{rendered}"
        );
    }
}

/// The run-space page states its declaration, its cost and its composition.
///
/// Each of these is a fact the page is the owner of: the axes it declares, the
/// forecast they resolve to, and the rule that combines them. A page that drew
/// the strip without the forecast would be showing a space with no stated cost.
#[test]
fn the_run_space_page_states_every_axis_and_what_they_resolve_to() {
    let rendered = render(SimulationPage::RunSet, 1200.0);

    for fragment in [
        "Run space",
        "Process section",
        "Supply voltage",
        "Temperature",
        "cartesian composition",
        "Tasks",
        "Storage",
        "Resolved point table",
        "Execution budgets",
        "Composition",
    ] {
        assert!(
            rendered.contains(fragment),
            "the run-space page omitted {fragment:?}:\n{rendered}"
        );
    }
    assert!(
        rendered.contains("27"),
        "the forecast must state the resolved point count:\n{rendered}"
    );
}

/// A refusal is shown against the declaration that caused it, and the invalid
/// value stays on screen. Dropping it would leave the user with an error about
/// a value they could no longer see.
#[test]
fn an_unresolvable_run_space_shows_its_refusal_and_keeps_the_invalid_value() {
    let rendered = render_with(SimulationPage::RunSet, 1200.0, |app| {
        let id = app.state.sim_setup.corner.run_set.dimensions[2].id.clone();
        crate::simulation::run_set::dispatch(
            &mut app.state.sim_setup.corner.run_set,
            crate::simulation::run_set::RunSetAction::SetValues {
                id,
                text: "-40\nwarm\n125".to_owned(),
            },
            1,
        );
    });

    assert!(
        rendered.contains("RUNSET-VALUE-SOURCE"),
        "the refusal must be identified by its stable code:\n{rendered}"
    );
    assert!(
        rendered.contains("warm"),
        "the invalid value must stay visible:\n{rendered}"
    );
    assert!(
        rendered.contains("Run-set preview blocked"),
        "the page must say that nothing was dispatched:\n{rendered}"
    );
}

/// The strip, the forecast and the point table read one declaration.
///
/// Before the run set they read three: the axis cards read the corner dialog's
/// fixed buffers, the forecast read `num_corners`, and the table read
/// `corner_names`. Disabling an axis is the cheapest way to prove they move
/// together.
#[test]
fn disabling_an_axis_moves_the_forecast_and_the_point_table_together() {
    let rendered = render_with(SimulationPage::RunSet, 1200.0, |app| {
        let id = app.state.sim_setup.corner.run_set.dimensions[1].id.clone();
        crate::simulation::run_set::dispatch(
            &mut app.state.sim_setup.corner.run_set,
            crate::simulation::run_set::RunSetAction::SetEnabled { id, enabled: false },
            1,
        );
    });

    assert!(
        rendered.contains('9'),
        "3 process × 3 temperature is nine points:\n{rendered}"
    );
    assert!(
        rendered.contains("9 points"),
        "the point table must report the same nine:\n{rendered}"
    );
}

/// Every option a page edits must survive a draft round trip.
///
/// A field with no draft buffer was silently reset to a literal on the next
/// commit, so a value the user set never reached the engine. The pivot
/// thresholds, the DC-sweep budget and arc-length continuation all behaved
/// that way before they were given buffers.
#[test]
fn every_edited_option_survives_a_draft_round_trip() {
    let options = SimulationOptions {
        pivrel: 0.125,
        pivtol: 2.5e-14,
        itl1: 77,
        itl4: 21,
        arc_length: true,
        gmin_stepping: false,
        source_stepping: false,
        pseudo_transient: false,
        reltol: 4.5e-5,
        residual_reltol: 6.5e-5,
        vntol: 3.5e-7,
        abstol: 7.5e-13,
        iabstol: 8.5e-13,
        chgtol: 9.5e-15,
        ..SimulationOptions::default()
    };
    let round_tripped = OptionsDialogState::from_options(&options)
        .to_options()
        .expect("a draft built from valid options must parse");
    assert_eq!(
        serde_json::to_value(&round_tripped).expect("serializable"),
        serde_json::to_value(&options).expect("serializable"),
        "a draft round trip changed the effective options"
    );
}

/// The resolved-policy ledger states what the next run freezes, so it must not
/// state a policy for a solve the plan will not perform. Time-stepping rows
/// appear only when the plan holds a time-stepped analysis.
#[test]
fn the_resolved_policy_ledger_only_claims_analyses_the_plan_holds() {
    let default_plan = render(SimulationPage::Solver, 1200.0);
    assert!(
        default_plan.contains("Update bound"),
        "the ledger must always state the plan-wide bounds:\n{default_plan}"
    );
    assert!(
        default_plan.contains("Time stepped"),
        "the test instance holds a transient, so time-stepping rows belong:\n{default_plan}"
    );

    // With every analysis disabled there is no time-stepped solve to describe.
    let no_time_stepping = render_with(SimulationPage::Solver, 1200.0, |app| {
        let Ok(plan) = app.state.sim_setup.stable_analysis_plan_mut() else {
            return;
        };
        let ids: Vec<_> = plan.instances().iter().map(|i| i.id()).collect();
        for id in ids {
            let _ = plan.set_enabled(id, false);
        }
    });
    assert!(
        !no_time_stepping.contains("Iterations per step"),
        "the ledger stated a time-stepping budget with no time-stepped analysis enabled:\n\
         {no_time_stepping}"
    );
    assert!(
        no_time_stepping.contains("Update bound"),
        "the plan-wide bounds still apply with nothing enabled:\n{no_time_stepping}"
    );
}

/// The page is titled "…& per-analysis overrides", so an analysis that stops
/// resolving to the plan policy has to be visible on it. A step ceiling is
/// stored by the transient's own form and only reported here, which is why the
/// row still has to appear when the form is what wrote it.
#[test]
fn an_analysis_that_leaves_the_plan_policy_is_named_in_the_ledger() {
    use crate::simulation::plan::AnalysisDraft;

    // The page title contains "per-analysis overrides", so the owner cell is
    // counted rather than searched for — the title would match either way.
    let owner_rows = |rendered: &str| rendered.matches("analysis override").count();

    let default_plan = render(SimulationPage::Solver, 1200.0);
    assert_eq!(
        owner_rows(&default_plan),
        1,
        "nothing diverges yet, so only the page title should say it:\n{default_plan}"
    );

    let overridden = render_with(SimulationPage::Solver, 1200.0, |app| {
        let Ok(plan) = app.state.sim_setup.stable_analysis_plan_mut() else {
            return;
        };
        let Some(id) = plan
            .instances()
            .iter()
            .find(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_)))
            .map(|instance| instance.id())
        else {
            return;
        };
        let _ = plan.edit(id, |draft| {
            if let AnalysisDraft::Transient(setup) = draft {
                setup.max_step = "5n".to_owned();
            }
        });
    });
    assert_eq!(
        owner_rows(&overridden),
        2,
        "a transient with its own step ceiling must be named as an override:\n{overridden}"
    );
    assert!(
        overridden.contains("TRAN"),
        "the override row must name which analysis diverged:\n{overridden}"
    );
    assert!(
        overridden.contains("5n"),
        "the ledger must state the value the analysis actually resolves to:\n{overridden}"
    );
}

/// The solver strip pairs each preset with its intent by position, so the two
/// lists have to stay the same length and in the same order. Getting this
/// wrong would describe one policy while applying another.
#[test]
fn preset_intents_line_up_with_the_named_presets() {
    assert_eq!(
        SimulationOptions::PRESETS.len(),
        super::page_solver::PRESET_INTENT.len()
    );
    let labels: Vec<&str> = SimulationOptions::PRESETS
        .iter()
        .map(|(label, _)| *label)
        .collect();
    assert_eq!(labels, ["Fast", "Balanced", "Accurate", "Robust"]);

    // Each preset must recognize itself, and an edited set must recognize none.
    for (label, build) in SimulationOptions::PRESETS {
        assert_eq!(build().preset_name(), Some(label));
        assert_eq!(build().preset_label(), label);
    }
    let mut edited = SimulationOptions::default();
    edited.reltol *= 2.0;
    assert_eq!(edited.preset_name(), None);
    assert_eq!(edited.preset_label(), "Custom");
}

/// Presets must be distinguishable by exact comparison, because that is how
/// the strip decides which chip is active. Two presets that serialize
/// identically would make the strip report the wrong policy.
#[test]
fn the_named_presets_are_pairwise_distinct() {
    let presets = [
        SimulationOptions::fast(),
        SimulationOptions::default(),
        SimulationOptions::accurate(),
        SimulationOptions::robust(),
    ];
    for (index, left) in presets.iter().enumerate() {
        for right in presets.iter().skip(index + 1) {
            assert_ne!(
                serde_json::to_value(left).expect("serializable"),
                serde_json::to_value(right).expect("serializable"),
                "two named presets resolve to the same option set"
            );
        }
    }
}

/// A registry that can only grow is not an editor. Removal has to be a real,
/// validated transaction on the plan payload — not a view-local filter.
#[test]
fn a_saved_output_can_be_removed_and_the_removal_is_validated() {
    use crate::product::SavedOutputId;
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };

    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("test instance has a stable plan")
        .id();
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "vout",
        "V(out)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::ALL[0],
        SavedOutputPrecision::ALL[0],
        SavedOutputStreaming::StoreOnly,
    )
    .expect("valid saved output");
    let output_id = output.id;
    app.state
        .workspace
        .add_saved_output(plan_id, output)
        .expect("the plan accepts a valid output");
    assert_eq!(
        app.state
            .workspace
            .active_plan_data(plan_id)
            .expect("payload")
            .saved_outputs
            .len(),
        1
    );

    let removed = app
        .state
        .workspace
        .remove_saved_output(plan_id, output_id)
        .expect("the output it just accepted can be removed");
    assert_eq!(removed.name, "vout");
    assert!(
        app.state
            .workspace
            .active_plan_data(plan_id)
            .expect("payload")
            .saved_outputs
            .is_empty()
    );

    // Removing the same identity twice must fail rather than silently succeed.
    assert!(
        app.state
            .workspace
            .remove_saved_output(plan_id, output_id)
            .is_err()
    );
    assert!(
        app.state
            .workspace
            .remove_saved_output(plan_id, SavedOutputId::new())
            .is_err()
    );
}

/// Duplicating exists so a second signal can inherit a capture contract that
/// was already tuned — the expression is the one thing the copy is meant to
/// change. If the copy shared the original's identity it would be a second
/// reference rather than a new record, and renaming one would rename both.
#[test]
fn a_duplicated_saved_output_inherits_everything_but_its_identity_and_name() {
    use crate::product::{ObjectRevision, SavedOutputId};
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };

    let mut app = RSpiceApp::test_instance();
    let plan_id = plan_id(&app);
    // Deliberately non-default capture decisions: a copy that silently reset
    // them to the defaults would still pass a weaker assertion.
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "vout",
        "V(out)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::ALL[SavedOutputPolicy::ALL.len() - 1],
        SavedOutputPrecision::ALL[SavedOutputPrecision::ALL.len() - 1],
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
    )
    .expect("valid saved output");
    let output_id = output.id;
    let source = output.clone();
    app.state
        .workspace
        .add_saved_output(plan_id, output)
        .expect("the plan accepts a valid output");

    let copy_id = app
        .state
        .workspace
        .duplicate_saved_output(plan_id, output_id, "vout_copy")
        .expect("a free name duplicates");
    assert!(copy_id != output_id, "the copy is its own record");

    let payload = app
        .state
        .workspace
        .active_plan_data(plan_id)
        .expect("payload");
    assert_eq!(payload.saved_outputs.len(), 2);
    let copy = payload
        .saved_outputs
        .iter()
        .find(|output| output.id == copy_id)
        .expect("the committed copy is in the registry");
    assert_eq!(copy.name, "vout_copy");
    assert!(copy.revision == ObjectRevision::INITIAL);
    // Comparing the whole record, rather than listing the fields, is what keeps
    // this honest when SavedOutput grows another capture decision.
    let mut expected = source;
    expected.id = copy.id;
    expected.revision = copy.revision;
    expected.name.clone_from(&copy.name);
    assert!(
        *copy == expected,
        "the copy differs from its source only in identity and name"
    );

    // The registry validates uniqueness case-insensitively, so a differently
    // cased collision has to be refused too — and refused atomically.
    assert!(
        app.state
            .workspace
            .duplicate_saved_output(plan_id, output_id, "VOUT")
            .is_err()
    );
    assert!(
        app.state
            .workspace
            .duplicate_saved_output(plan_id, SavedOutputId::new(), "orphan")
            .is_err()
    );
    assert_eq!(
        app.state
            .workspace
            .active_plan_data(plan_id)
            .expect("payload")
            .saved_outputs
            .len(),
        2,
        "a refused duplicate leaves the registry exactly as it was"
    );
}

/// Both registries derive the copy's name rather than opening a dialog over a
/// table that is about to change, so the derivation has to agree with the
/// case-insensitive uniqueness rule the plan itself enforces.
#[test]
fn a_duplicate_name_skips_every_name_the_registry_already_holds() {
    use super::workflows::unique_copy_name;

    let held = |names: &'static [&'static str]| {
        move |candidate: &str| {
            names
                .iter()
                .any(|name| name.eq_ignore_ascii_case(candidate))
        }
    };

    assert_eq!(unique_copy_name("vout", held(&[])), "vout_copy");
    assert_eq!(unique_copy_name("vout", held(&["vout"])), "vout_copy");
    assert_eq!(
        unique_copy_name("vout", held(&["vout", "vout_copy"])),
        "vout_copy_2"
    );
    assert_eq!(
        unique_copy_name("vout", held(&["VOUT_COPY", "vout_copy_2"])),
        "vout_copy_3"
    );
}

/// Authoring belongs to the specification editor in Results, so this page's
/// selection has to survive that list being edited underneath it. Keyed by row
/// index it could not: removing one requirement re-points the selection at
/// whichever one moved into its place, and reporting the wrong requirement's
/// margin is the exact failure this page exists to prevent.
#[test]
fn a_selected_specification_is_identified_by_its_measurement_not_its_row() {
    use crate::state::SpecEntry;

    fn spec(measurement: &str) -> SpecEntry {
        SpecEntry {
            measurement: measurement.to_owned(),
            expression: format!("meas {measurement}"),
            min: Some(0.0),
            max: Some(1.0),
            unit: "V".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        }
    }

    // The selected requirement outlives the removal of an earlier row.
    let rendered = render_with(SimulationPage::Specifications, 1400.0, |app| {
        let id = plan_id(app);
        app.state.workbench.selected_specification = Some("bandwidth".to_owned());
        app.state
            .workspace
            .replace_active_specs(id, vec![spec("bandwidth")]);
    });
    assert!(
        rendered.contains("Selected specification · bandwidth"),
        "the selection still names the requirement it was made on: {rendered}"
    );

    // The selected requirement is gone, so nothing is selected — the row that
    // took its place must not inherit the selection.
    let rendered = render_with(SimulationPage::Specifications, 1400.0, |app| {
        let id = plan_id(app);
        app.state.workbench.selected_specification = Some("gain".to_owned());
        app.state
            .workspace
            .replace_active_specs(id, vec![spec("bandwidth")]);
    });
    assert!(
        !rendered.contains("Selected specification · bandwidth"),
        "a removed requirement must never hand its selection to another row: {rendered}"
    );
    assert!(
        rendered.contains("none selected"),
        "the record card reports that nothing resolves: {rendered}"
    );
}

/// A capture policy an engineer can read but not change is a report, not a
/// control. The Save & streaming page totals these exact fields, so each one
/// has to be editable through a validated transaction that moves the plan
/// revision.
#[test]
fn a_saved_output_capture_policy_can_be_changed_after_it_is_authored() {
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };

    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("test instance has a stable plan")
        .id();
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "vout",
        "V(out)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .expect("valid saved output");
    let output_id = output.id;
    let original_revision = output.revision;
    app.state
        .workspace
        .add_saved_output(plan_id, output.clone())
        .expect("the plan accepts a valid output");

    let mut replacement = output.clone();
    replacement.save_policy = SavedOutputPolicy::FailureDiagnosticsOnly;
    replacement.stored_precision = SavedOutputPrecision::DisplayCacheWithFullSourcePrecision;
    replacement.streaming = SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation;
    let revision = app
        .state
        .workspace
        .replace_saved_output(plan_id, output_id, replacement)
        .expect("a valid capture contract is accepted");
    assert_ne!(
        revision, original_revision,
        "an accepted edit must advance the output's own revision"
    );
    let stored = app
        .state
        .workspace
        .active_plan_data(plan_id)
        .expect("payload")
        .saved_outputs
        .iter()
        .find(|candidate| candidate.id == output_id)
        .expect("the output survives the edit")
        .clone();
    assert_eq!(
        stored.save_policy,
        SavedOutputPolicy::FailureDiagnosticsOnly
    );
    assert_eq!(
        stored.stored_precision,
        SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
    );
    assert_eq!(
        stored.streaming,
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
    );
    assert_eq!(
        stored.name, "vout",
        "editing the capture policy must not rename the output"
    );

    // Re-committing the same contract is a semantic no-op: it must not spend a
    // revision, or every repaint that re-selects the same option would.
    let unchanged = app
        .state
        .workspace
        .replace_saved_output(plan_id, output_id, stored.clone())
        .expect("an unchanged contract is accepted");
    assert_eq!(
        unchanged, revision,
        "re-committing an identical contract must not advance the revision"
    );

    // An edit that collides with another output's name is refused whole.
    let other = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "vin",
        "V(in)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .expect("valid saved output");
    app.state
        .workspace
        .add_saved_output(plan_id, other)
        .expect("the plan accepts a second output");
    let mut collision = stored;
    collision.name = "VIN".to_owned();
    assert!(
        app.state
            .workspace
            .replace_saved_output(plan_id, output_id, collision)
            .is_err(),
        "a name that collides case-insensitively with another output must be refused"
    );
    assert_eq!(
        app.state
            .workspace
            .active_plan_data(plan_id)
            .expect("payload")
            .saved_outputs
            .iter()
            .find(|candidate| candidate.id == output_id)
            .expect("the output is untouched by the refused edit")
            .name,
        "vout"
    );
}

/// An output's name and expression are the two things most likely to be wrong
/// after authoring — a typo'd node, a signal renamed in the schematic. Both
/// have to be fixable without deleting and re-authoring the contract.
#[test]
fn a_saved_output_can_be_renamed_and_its_expression_rewritten() {
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };

    let mut app = RSpiceApp::test_instance();
    let plan_id = plan_id(&app);
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "vout",
        "V(otu)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .expect("valid saved output");
    let output_id = output.id;
    app.state
        .workspace
        .add_saved_output(plan_id, output.clone())
        .expect("the plan accepts it");

    let mut corrected = output;
    corrected.name = "v_out".to_owned();
    corrected.source_expression = "V(out)".to_owned();
    app.state
        .workspace
        .replace_saved_output(plan_id, output_id, corrected)
        .expect("a corrected contract is accepted");

    let stored = app
        .state
        .workspace
        .active_plan_data(plan_id)
        .expect("payload")
        .saved_outputs
        .iter()
        .find(|candidate| candidate.id == output_id)
        .expect("the output keeps its identity across a rename")
        .clone();
    assert_eq!(stored.name, "v_out");
    assert_eq!(stored.source_expression, "V(out)");
    assert_eq!(
        stored.save_policy,
        SavedOutputPolicy::EveryAcceptedPoint,
        "correcting the expression must not disturb the capture policy"
    );
}

/// The composed run space decides how many points a dispatch executes, so a
/// change to it has to move the plan revision. Otherwise an authorized
/// preflight could be followed by a sweep change and a dispatch that ran a
/// different run space than the one that was checked.
#[test]
fn a_run_space_change_moves_the_plan_revision() {
    let mut app = RSpiceApp::test_instance();
    let before = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();
    let id = app.state.sim_setup.corner.run_set.dimensions[2].id.clone();
    let transaction = crate::simulation::run_set::dispatch(
        &mut app.state.sim_setup.corner.run_set,
        crate::simulation::run_set::RunSetAction::SetValues {
            id,
            text: "-40\n0\n25\n85\n125".to_owned(),
        },
        1,
    );
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);
    app.state
        .sim_setup
        .commit_active_plan_configuration_change("Run set · set-dimension-values".to_owned())
        .expect("a run-space change commits as a configuration change");
    let after = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();
    assert_ne!(
        before, after,
        "a run-space change must advance the plan revision so preflight is invalidated"
    );
}

/// An expression an engineer cannot change is a create-only registry.
#[test]
fn a_design_variable_expression_can_be_edited_in_place() {
    use crate::state::{
        DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableScope,
        DesignVariableSweepEligibility,
    };

    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let variable = DesignVariable::new(
        "rload",
        "1kohm",
        DesignVariableQuantity::Resistance,
        DesignVariableScope::Testbench,
        "load resistance",
        None,
        DesignVariableSweepEligibility::NestedSweepAndOptimization,
        DesignVariableOverridePolicy::ExplicitTestLocalOverride,
    )
    .expect("valid design variable");
    let variable_id = variable.id;
    let original_revision = variable.revision;
    app.state
        .workspace
        .add_design_variable(plan_id, variable)
        .expect("the plan accepts a valid variable");

    let revision = app
        .state
        .workspace
        .update_design_variable_expression(plan_id, variable_id, "2kohm")
        .expect("a valid expression is accepted");
    assert_ne!(
        revision, original_revision,
        "an accepted edit must advance the variable's own revision"
    );
    let stored = app
        .state
        .workspace
        .active_plan_data(plan_id)
        .expect("payload")
        .design_variables
        .iter()
        .find(|candidate| candidate.id == variable_id)
        .expect("the variable survives the edit");
    assert_eq!(stored.expression, "2kohm");
    assert_eq!(
        stored.name, "rload",
        "editing the expression must not rename the variable"
    );
}

/// A variable's typed contract — quantity, scope, sweep role, override policy
/// and bounds — decides what a sweep may do with it and what preflight may
/// reject. All of it has to be reachable after authoring, and an unbounded
/// variable has to be able to become bounded.
#[test]
fn a_design_variable_contract_can_be_retyped_and_bounded_after_it_is_authored() {
    use crate::state::{
        DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableRange,
        DesignVariableScope, DesignVariableSweepEligibility,
    };

    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let variable = DesignVariable::new(
        "rload",
        "1kohm",
        DesignVariableQuantity::Resistance,
        DesignVariableScope::Testbench,
        "load resistance",
        None,
        DesignVariableSweepEligibility::FixedParameter,
        DesignVariableOverridePolicy::InheritOwnerOnly,
    )
    .expect("valid design variable");
    let variable_id = variable.id;
    app.state
        .workspace
        .add_design_variable(plan_id, variable.clone())
        .expect("the plan accepts a valid variable");
    assert!(
        variable.allowed_range.is_none(),
        "the variable starts unbounded"
    );

    let mut replacement = variable.clone();
    replacement.sweep_eligibility = DesignVariableSweepEligibility::NestedSweepAndOptimization;
    replacement.override_policy = DesignVariableOverridePolicy::ExplicitTestLocalOverride;
    replacement.scope = DesignVariableScope::Project;
    replacement.allowed_range = Some(DesignVariableRange {
        minimum: "500ohm".to_owned(),
        maximum: "2kohm".to_owned(),
    });
    app.state
        .workspace
        .replace_design_variable(plan_id, variable_id, replacement)
        .expect("a valid contract is accepted");

    let stored = app
        .state
        .workspace
        .active_plan_data(plan_id)
        .expect("payload")
        .design_variables
        .iter()
        .find(|candidate| candidate.id == variable_id)
        .expect("the variable survives the edit")
        .clone();
    assert_eq!(
        stored.sweep_eligibility,
        DesignVariableSweepEligibility::NestedSweepAndOptimization
    );
    assert_eq!(
        stored.override_policy,
        DesignVariableOverridePolicy::ExplicitTestLocalOverride
    );
    assert_eq!(stored.scope, DesignVariableScope::Project);
    assert_eq!(
        stored
            .allowed_range
            .as_ref()
            .map(|range| (range.minimum.as_str(), range.maximum.as_str())),
        Some(("500ohm", "2kohm")),
        "a variable that was authored unbounded must be able to gain a bound"
    );
    assert_eq!(
        stored.expression, "1kohm",
        "retyping the contract must not disturb the expression"
    );

    // Clearing both bounds is a real choice: it makes the variable unbounded
    // again rather than leaving a half-open range preflight cannot check.
    let mut unbounded = stored;
    unbounded.allowed_range = None;
    app.state
        .workspace
        .replace_design_variable(plan_id, variable_id, unbounded)
        .expect("removing the bound is accepted");
    assert!(
        app.state
            .workspace
            .active_plan_data(plan_id)
            .expect("payload")
            .design_variables
            .iter()
            .find(|candidate| candidate.id == variable_id)
            .expect("the variable survives")
            .allowed_range
            .is_none()
    );
}

#[test]
fn ledger_columns_tile_their_row_without_a_gap_or_an_overhang() {
    let row = Rect::from_min_size(egui::Pos2::ZERO, vec2(640.0, 28.0));
    for fractions in [
        vec![0.36, 0.34, 0.30],
        vec![0.18, 0.52, 0.30],
        vec![0.20, 0.13, 0.28, 0.17, 0.22],
    ] {
        let columns = column_rects(row, &fractions);
        assert_eq!(columns.len(), fractions.len());
        assert_eq!(columns[0].left(), row.left());
        assert_eq!(
            columns[columns.len() - 1].right(),
            row.right(),
            "the last column must absorb the remainder so no strip of the row is unclaimed"
        );
        assert!(columns.iter().all(|column| column.width() > 0.0));
        assert!(
            columns
                .windows(2)
                .all(|pair| pair[0].right() == pair[1].left())
        );
    }
}

/// The ledger's job is to say what a run resolves to, so an analysis that
/// carries its own bound has to be reported at that bound and not at the plan's.
/// Both values are shown side by side: the divergence is the fact, and one
/// number alone cannot state it.
#[test]
fn the_ledger_reports_the_effective_value_of_an_authored_override() {
    use crate::simulation::plan::{
        AnalysisDraft, AnalysisKind, AnalysisNumericOverride, NumericOverrideOption,
    };

    let rendered = render_with(SimulationPage::Solver, 1400.0, |app| {
        app.state.sim_setup.options.itl4 = 6;
        let Ok(plan) = app.state.sim_setup.stable_analysis_plan_mut() else {
            return;
        };
        let Some(id) = plan
            .instances()
            .iter()
            .find(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_)))
            .map(|instance| instance.id())
        else {
            return;
        };
        let mut record = AnalysisNumericOverride::default();
        record
            .set(AnalysisKind::Transient, NumericOverrideOption::Itl4, "137")
            .expect("a transient takes timesteps");
        plan.set_numeric_override(id, Some(record))
            .expect("the plan accepts a bound the transient can use");
    });

    assert!(
        rendered.contains("EFFECTIVE VALUE") && rendered.contains("PRESET VALUE"),
        "the ledger must separate what the plan states from what the run resolves to:\n{rendered}"
    );
    assert_eq!(
        rendered.matches("Iterations per step").count(),
        2,
        "the plan row and the analysis's own row must both appear:\n{rendered}"
    );
    assert!(
        rendered.contains("137"),
        "the ledger reported the preset instead of the value the run will use:\n{rendered}"
    );
    assert_eq!(
        rendered.matches("analysis override").count(),
        2,
        "exactly one row may be attributed to the analysis:\n{rendered}"
    );
}

/// An override the analysis's solve would never read must be refused, not
/// stored. A stored-and-ignored bound is indistinguishable from one that works,
/// and the ledger would then report a policy no run resolves to.
///
/// The record is built against a transient and then offered to an AC sweep,
/// because that is the shape a restored project or a retyped analysis takes:
/// the authoring gate is not the only place the pairing has to hold.
#[test]
fn an_override_the_analysis_cannot_use_is_refused_and_leaves_the_plan_unchanged() {
    use crate::simulation::plan::{AnalysisKind, AnalysisNumericOverride, NumericOverrideOption};

    let mut app = RSpiceApp::test_instance();
    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the test instance has a stable plan");
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("an AC sweep inserts");

    let mut record = AnalysisNumericOverride::default();
    record
        .set(AnalysisKind::Transient, NumericOverrideOption::Itl4, "12")
        .expect("a transient takes timesteps");

    let before = serde_json::to_vec(&plan).expect("the plan serializes");
    let error = plan
        .set_numeric_override(ac, Some(record))
        .expect_err("an AC sweep never takes a timestep");
    let after = serde_json::to_vec(&plan).expect("the plan serializes");

    assert_eq!(
        before, after,
        "a refused override must leave the plan byte-for-byte unchanged"
    );
    assert!(
        error.to_string().contains("ITL4"),
        "the refusal must name the option it refused: {error}"
    );
}

/// Projects written before per-analysis numerics existed have to keep opening.
/// `AnalysisInstance` denies unknown fields, so the new one has to default when
/// absent — and stay absent on the way out, so re-saving an untouched plan does
/// not rewrite every instance.
#[test]
fn a_plan_written_before_per_analysis_numerics_still_loads() {
    use crate::simulation::plan::{
        AnalysisDraft, AnalysisKind, AnalysisNumericOverride, NumericOverrideOption, SimulationPlan,
    };

    let mut plan = SimulationPlan::new();
    let serialized = serde_json::to_string(&plan).expect("the plan serializes");
    assert!(
        !serialized.contains("numeric_override"),
        "an inheriting plan must serialize exactly as it did before the field existed"
    );
    let restored: SimulationPlan =
        serde_json::from_str(&serialized).expect("a plan without the field still loads");
    assert!(restored.instances()[0].numeric_override().is_none());

    let id = plan
        .instances()
        .iter()
        .find(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_)))
        .map(|instance| instance.id())
        .expect("a fresh plan holds one transient");
    let mut record = AnalysisNumericOverride::default();
    record
        .set(AnalysisKind::Transient, NumericOverrideOption::Trtol, "3")
        .expect("a transient has a truncation bound");
    plan.set_numeric_override(id, Some(record))
        .expect("the plan accepts it");
    let round_tripped: SimulationPlan =
        serde_json::from_str(&serde_json::to_string(&plan).expect("serializes"))
            .expect("an authored plan round trips");
    assert_eq!(
        round_tripped
            .instance(id)
            .and_then(|instance| instance.numeric_override())
            .and_then(|record| record.trtol()),
        Some(3.0)
    );
}

/// The scope control is only honest if its options come from the plan: an
/// option list with a literal point count, or a corner the run set does not
/// declare, would offer a scope no run can answer.
#[test]
fn the_applies_to_control_is_built_from_the_declared_run_set() {
    use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig, ProcessCorner};
    use crate::simulation::run_set::RunSetState;
    use crate::state::{SpecEntry, SpecPointScope};

    fn spec(scope: SpecPointScope) -> SpecEntry {
        SpecEntry {
            measurement: "gain".to_owned(),
            expression: "meas ac gain max V(out)".to_owned(),
            min: Some(10.0),
            max: None,
            unit: "dB".to_owned(),
            scope,
        }
    }

    fn six_point_run_set() -> RunSetState {
        RunSetState::from_corner_config(&CornerConfig {
            process_corners: vec![ProcessCorner::TT, ProcessCorner::SS],
            voltages: vec![1.0],
            temperatures: vec![-40.0, 27.0, 125.0],
            full_matrix: true,
            points: Vec::new(),
            base_analysis: CornerBaseAnalysis::Op,
        })
    }

    let rendered = render_with(SimulationPage::Specifications, 1400.0, |app| {
        let id = plan_id(app);
        app.state.sim_setup.corner.run_set = six_point_run_set();
        app.state.workbench.selected_specification = Some("gain".to_owned());
        app.state
            .workspace
            .replace_active_specs(id, vec![spec(SpecPointScope::AllPoints)]);
    });
    assert!(
        rendered.contains("Applies to"),
        "the record carries the scope control: {rendered}"
    );
    assert!(
        rendered.contains("All 6 PVT points"),
        "the point count is the run set's own, not a literal: {rendered}"
    );

    // A scope the run set cannot reach is standing validation, recomputed
    // every frame, so it belongs in the registry's own status line.
    let rendered = render_with(SimulationPage::Specifications, 1400.0, |app| {
        let id = plan_id(app);
        app.state.sim_setup.corner.run_set = six_point_run_set();
        app.state.workbench.selected_specification = Some("gain".to_owned());
        app.state.workspace.replace_active_specs(
            id,
            vec![spec(SpecPointScope::SelectedCorners {
                corners: vec!["FF".to_owned()],
            })],
        );
    });
    assert!(
        rendered.contains("1 scoped to corners the run set does not declare"),
        "the registry reports the unreachable scope: {rendered}"
    );
    assert!(
        rendered.contains("which the current run set does not declare"),
        "the selected record says which corner is unreachable: {rendered}"
    );

    // A corner the run set does declare is reachable and reported as nothing
    // more than a limit still awaiting evidence.
    let rendered = render_with(SimulationPage::Specifications, 1400.0, |app| {
        let id = plan_id(app);
        app.state.sim_setup.corner.run_set = six_point_run_set();
        app.state.workbench.selected_specification = Some("gain".to_owned());
        app.state.workspace.replace_active_specs(
            id,
            vec![spec(SpecPointScope::SelectedCorners {
                corners: vec!["SS".to_owned()],
            })],
        );
    });
    assert!(
        !rendered.contains("does not declare"),
        "a declared corner raises nothing: {rendered}"
    );
    assert!(
        rendered.contains("Corner SS only"),
        "the control shows the scope it holds: {rendered}"
    );
}

/// The scope is part of the requirement, so changing it has to move the plan
/// the same way every other specification edit does: one validated
/// transaction that leaves a receipt and invalidates preflight. A rejected
/// scope must leave the plan exactly as it was.
#[test]
fn a_scope_change_commits_as_a_plan_transaction_and_a_rejected_one_changes_nothing() {
    use crate::state::{SpecEntry, SpecPointScope};

    let mut app = RSpiceApp::test_instance();
    let id = plan_id(&app);
    app.state.workspace.replace_active_specs(
        id,
        vec![SpecEntry {
            measurement: "gain".to_owned(),
            expression: "meas ac gain max V(out)".to_owned(),
            min: Some(10.0),
            max: None,
            unit: "dB".to_owned(),
            scope: SpecPointScope::AllPoints,
        }],
    );
    let revision = |app: &RSpiceApp| {
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .revision()
    };
    let before = revision(&app);

    // Matched case-insensitively, exactly as the selection resolves it.
    super::page_specs::commit_scope(&mut app, "GAIN", SpecPointScope::Nominal);
    assert_eq!(
        app.state.workspace.active_specs(id)[0].scope,
        SpecPointScope::Nominal
    );
    assert_ne!(
        revision(&app),
        before,
        "an accepted edit moves the plan revision"
    );
    assert!(
        !app.state.workbench.analysis_lifecycle_status.is_empty(),
        "the transaction reports its receipt"
    );

    let after_accept = revision(&app);
    super::page_specs::commit_scope(
        &mut app,
        "gain",
        SpecPointScope::SelectedCorners {
            corners: Vec::new(),
        },
    );
    assert_eq!(
        app.state.workspace.active_specs(id)[0].scope,
        SpecPointScope::Nominal,
        "a rejected scope leaves the stored requirement untouched"
    );
    assert_eq!(
        revision(&app),
        after_accept,
        "a rejected edit does not move the plan revision"
    );
}
