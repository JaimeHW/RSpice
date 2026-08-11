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

#[test]
fn a_page_never_claims_a_run_space_it_cannot_resolve() {
    let rendered = render(SimulationPage::RunSet, 1200.0);
    assert!(
        rendered.contains("single reference point") || rendered.contains("corner sweep enabled"),
        "the run-set page did not state which composition is active:\n{rendered}"
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
        itl2: 133,
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
/// resolving to the plan policy has to be visible on it. The value itself is
/// owned by the analysis form; this page reports the divergence.
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
    app.state.sim_setup.corner.enable_temp_sweep = true;
    app.state
        .sim_setup
        .commit_active_plan_configuration_change("Updated the temperature axis.".to_owned())
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
