//! The Simulation Studio setup pages drawn over the whole application: every
//! route renders its own heading without an identity clash, the page's own
//! subject reaches the screen, and the record editors, ledgers, filters and
//! handoffs paint what the plan holds. The other page sweeps draw through the
//! render helpers here.

use super::stage::show_on_stage;
use crate::state::SimulationPage;
use crate::workbench::surfaces::simulate::page_outputs;
use crate::workbench::surfaces::simulate::page_tests::plan_id;
use crate::workbench::{AppState, RSpiceApp};
use egui::{Rect, vec2};

/// Render one page and collect the text it painted.
pub(super) fn render(page: SimulationPage, width: f32) -> String {
    render_with(page, width, |_| {})
}

/// The rendered viewport's height.
///
/// Every page draws inside a `ScrollArea`, which paints only what is visible,
/// so a page taller than this frame is collected in part and a test asserting
/// on its lower half fails for a reason that has nothing to do with the page.
/// The measure is therefore taller than the tallest page rather than tuned to
/// a particular one: what a test wants to see is the whole page.
pub(super) const RENDER_VIEWPORT_HEIGHT: f32 = 2600.0;

/// Render one page over an app the caller has seeded first.
///
/// A registry page shows its record editor only once something is selected, so
/// a test that wants to see those controls has to put a record in the plan and
/// select it before the frame runs.
pub(super) fn render_with(
    page: SimulationPage,
    width: f32,
    seed: impl FnOnce(&mut RSpiceApp),
) -> String {
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
    app.state.simulate.simulation_page = page;
    seed(&mut app);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                vec2(width, RENDER_VIEWPORT_HEIGHT),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    show_on_stage(ui, &mut app);
                });
        },
    );
    let mut rendered = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut rendered);
    }
    rendered
}

/// A Studio page is named once, by its title.
///
/// Every one of the nine carried a sentence under that title explaining what
/// the page was for, in the words of whoever built it — the lecture subtitle,
/// which says nothing the controls beneath it do not already show. The
/// fragments below are what each page used to say.
#[test]
fn no_studio_page_paints_a_sentence_under_its_title() {
    // The Analyses route is headed by its plan's name, every other route by
    // its page's.
    let app = RSpiceApp::test_instance();
    let plan = app.state.model.sim_setup.active_plan_name();
    for (page, lecture) in [
        (
            SimulationPage::Analyses,
            "Order the plan, configure each analysis",
        ),
        (SimulationPage::Excitations, "See every source the design"),
        (SimulationPage::Variables, "Own the typed parameters"),
        (SimulationPage::Outputs, "Own the expressions the run saves"),
        (
            SimulationPage::Specifications,
            "Own the limits a result is judged",
        ),
        (SimulationPage::RunSet, "Compose the run space from process"),
        (SimulationPage::Models, "Own the ordered model closure"),
        (SimulationPage::Solver, "Own what counts as converged"),
        (SimulationPage::Save, "Own what is stored"),
    ] {
        let rendered = render(page, 1200.0);
        let title = match page {
            SimulationPage::Analyses => plan.as_str(),
            _ => page.title(),
        };
        assert!(
            rendered.contains(title),
            "{page:?} must still paint its title:\n{rendered}"
        );
        assert!(
            !rendered.contains(lecture),
            "{page:?} still lectures under its title:\n{rendered}"
        );
    }
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
        let id = plan_id(&app.state.model);
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
            .model
            .workspace
            .content
            .add_saved_output(id, output)
            .expect("the plan accepts it");
        app.state.simulate.selected_saved_output = Some("vout".to_owned());
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
        let id = plan_id(&app.state.model);
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
            .model
            .workspace
            .content
            .add_design_variable(id, variable)
            .expect("the plan accepts it");
        app.state.simulate.selected_design_variable = Some("rload".to_owned());
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

/// Every field of `SimulationOptions` that reaches the engine has an editor on
/// this page, because this page is the only editor there is. The bypass voltage
/// floor and the model reference temperature were reachable solely through a
/// modal that could not be opened; they are checked by name so a layout change
/// cannot quietly strand them again.
#[test]
fn the_solver_page_carries_the_options_that_had_no_other_editor() {
    let rendered = render(SimulationPage::Solver, 1200.0);
    for fragment in [
        "BSIMSOI device bypass",
        "Bypass relative bound",
        "Bypass voltage floor",
        "Temperature reference",
        "Simulation temperature · TEMP",
        "Model reference temperature · TNOM",
    ] {
        assert!(
            rendered.contains(fragment),
            "the solver page did not state {fragment:?}:\n{rendered}"
        );
    }
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
        let Ok(plan) = app.state.model.sim_setup.stable_analysis_plan_mut() else {
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
        let Ok(plan) = app.state.model.sim_setup.stable_analysis_plan_mut() else {
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
        overridden.contains(crate::simulation::plan::AnalysisKind::Transient.label()),
        "the override row must name which analysis diverged, by the name the rest of the \
         product shows it under rather than by its directive code:\n{overridden}"
    );
    assert!(
        overridden.contains("5n"),
        "the ledger must state the value the analysis actually resolves to:\n{overridden}"
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
        let id = plan_id(&app.state.model);
        app.state.simulate.selected_specification = Some("bandwidth".to_owned());
        app.state
            .model
            .workspace
            .content
            .replace_active_specs(id, vec![spec("bandwidth")]);
    });
    assert!(
        rendered.contains("Selected specification · bandwidth"),
        "the selection still names the requirement it was made on: {rendered}"
    );

    // The selected requirement is gone, so nothing is selected — the row that
    // took its place must not inherit the selection.
    let rendered = render_with(SimulationPage::Specifications, 1400.0, |app| {
        let id = plan_id(&app.state.model);
        app.state.simulate.selected_specification = Some("gain".to_owned());
        app.state
            .model
            .workspace
            .content
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
        app.state.model.sim_setup.options.itl4 = 6;
        let Ok(plan) = app.state.model.sim_setup.stable_analysis_plan_mut() else {
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
            .set_for_instance(
                AnalysisKind::Transient,
                crate::simulation::plan::SolverOwnership::NONE,
                NumericOverrideOption::Itl4,
                "137",
            )
            .expect("a transient takes timesteps");
        plan.set_numeric_override(id, Some(record))
            .expect("the plan accepts a bound the transient can use");
    });

    assert!(
        rendered.contains("Effective value") && rendered.contains("Preset value"),
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

/// The scope control is only honest if its options come from the plan: an
/// option list with a literal point count, or a corner the run set does not
/// declare, would offer a scope no run can answer.
#[test]
fn the_applies_to_control_is_built_from_the_declared_run_set() {
    use crate::product::ProcessCorner;
    use crate::state::{SpecEntry, SpecPointScope};
    use rspice_simulation_contract::corner_config::{CornerBaseAnalysis, CornerConfig};
    use rspice_simulation_contract::run_set::RunSetState;

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
        crate::simulation::controller::test_execution::pvt::from_corner_config(&CornerConfig {
            process_corners: vec![ProcessCorner::TT, ProcessCorner::SS],
            voltages: vec![1.0],
            supply_source_names: Vec::new(),
            temperatures: vec![-40.0, 27.0, 125.0],
            full_matrix: true,
            points: Vec::new(),
            base_analysis: CornerBaseAnalysis::Op,
        })
    }

    let rendered = render_with(SimulationPage::Specifications, 1400.0, |app| {
        let id = plan_id(&app.state.model);
        app.state.model.sim_setup.run_set = six_point_run_set();
        app.state.simulate.selected_specification = Some("gain".to_owned());
        app.state
            .model
            .workspace
            .content
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
        let id = plan_id(&app.state.model);
        app.state.model.sim_setup.run_set = six_point_run_set();
        app.state.simulate.selected_specification = Some("gain".to_owned());
        app.state.model.workspace.content.replace_active_specs(
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
        let id = plan_id(&app.state.model);
        app.state.model.sim_setup.run_set = six_point_run_set();
        app.state.simulate.selected_specification = Some("gain".to_owned());
        app.state.model.workspace.content.replace_active_specs(
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

/// The active plan's identity, for seeds that hold only the session state.
fn seeded_plan_id(state: &AppState) -> crate::product::SimulationPlanId {
    state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance has a stable plan")
        .id()
}

/// Three saved outputs whose names, expressions and kinds differ, so a filter
/// has something to discriminate on.
fn seed_three_outputs(state: &mut AppState) {
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };

    let id = seeded_plan_id(state);
    for (kind, name, expression) in [
        (SavedOutputKind::RawVoltageOrCurrent, "vout", "V(out)"),
        (SavedOutputKind::RawVoltageOrCurrent, "vsupply", "V(vdd)"),
        (
            SavedOutputKind::DerivedExpression,
            "gain_ac",
            "db20(V(out))",
        ),
    ] {
        let output = SavedOutput::new(
            kind,
            name,
            expression,
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("valid saved output");
        state
            .model
            .workspace
            .content
            .add_saved_output(id, output)
            .expect("the plan accepts it");
    }
}

/// A filter is only worth drawing if it changes what the table shows. This
/// asserts the narrowing itself — the rows the query excludes stop being
/// painted — and that the plan behind them is untouched, because a filter that
/// removed outputs would change what a run captures.
#[test]
fn the_output_filter_narrows_the_rendered_rows_and_not_the_plan() {
    let unfiltered = render_with(SimulationPage::Outputs, 1200.0, |app| {
        seed_three_outputs(&mut app.state);
    });
    for name in ["vout", "vsupply", "gain_ac"] {
        assert!(
            unfiltered.contains(name),
            "the unfiltered registry paints every saved output:\n{unfiltered}"
        );
    }

    let filtered = render_with(SimulationPage::Outputs, 1200.0, |app| {
        seed_three_outputs(&mut app.state);
        app.state.simulate.saved_output_filter = "db20".to_owned();
    });
    // Scoped to the registry card, which ends at its closing note: the storage
    // forecast below it is a total over every output the plan holds and is
    // deliberately not filtered.
    let table = filtered
        .split_once("Status is resolved against")
        .map_or(filtered.as_str(), |(registry, _)| registry);
    assert!(
        table.contains("gain_ac"),
        "the row whose expression matches has to survive:\n{filtered}"
    );
    for hidden in ["vout", "vsupply"] {
        assert!(
            !table.contains(hidden),
            "{hidden} does not match `db20` and must not be painted:\n{filtered}"
        );
    }
    assert!(
        filtered.contains("showing 1 of 3"),
        "the head has to say how much of the registry survived:\n{filtered}"
    );

    let mut app = RSpiceApp::test_instance();
    seed_three_outputs(&mut app.state);
    let id = plan_id(&app.state.model);
    let before = app.state.model.workspace.content.plan_data(id).cloned();
    app.state.simulate.saved_output_filter = "db20".to_owned();
    assert_eq!(
        app.state.model.workspace.content.plan_data(id).cloned(),
        before,
        "a filter is a view: it must not move one byte of plan-owned data"
    );
}

/// "You have no outputs" and "your filter matched none of them" are different
/// facts with different fixes. A registry that reported the second as the
/// first would tell an engineer their filtered view was the whole plan.
#[test]
fn an_output_filter_matching_nothing_is_distinguishable_from_an_empty_registry() {
    let empty = render_with(SimulationPage::Outputs, 1200.0, |_| {});
    // What an empty registry means is the selection mode's answer, and the
    // default mode saves a bounded synthesized set rather than nothing.
    assert!(
        empty.contains("No saved outputs")
            && empty.contains(&page_outputs::empty_registry_outcome(
                crate::state::OutputSelectionMode::default(),
                0
            )),
        "a plan with no outputs states what its selection mode will save:\n{empty}"
    );
    assert!(
        !empty.contains("No matches"),
        "an unfiltered empty registry is not a failed match:\n{empty}"
    );

    let narrowed = render_with(SimulationPage::Outputs, 1200.0, |app| {
        seed_three_outputs(&mut app.state);
        app.state.simulate.saved_output_filter = "no such signal".to_owned();
    });
    assert!(
        narrowed.contains("No matches"),
        "a filter that matched nothing has to say so:\n{narrowed}"
    );
    assert!(
        narrowed.contains("3 saved"),
        "and the registry's head still counts the outputs it holds:\n{narrowed}"
    );
    assert!(
        !narrowed.contains("the run stores nothing"),
        "the outputs are all still captured, so this must not read as an empty plan:\n{narrowed}"
    );
}

/// Two specifications: one the dataset measured and failed, one it never
/// measured at all, so both the text filter and the evidence class have
/// something to discriminate on.
fn seed_two_specifications(state: &mut AppState) {
    use crate::state::{
        AnalysisResult, AnalysisResultProvenance, AnalysisType, SimulationRun, SpecEntry,
        SpecPointScope,
    };

    let id = seeded_plan_id(state);
    state.model.workspace.content.replace_active_specs(
        id,
        vec![
            SpecEntry {
                measurement: "gain_dc".to_owned(),
                expression: "db20(V(out))".to_owned(),
                min: Some(40.0),
                max: None,
                unit: "dB".to_owned(),
                scope: SpecPointScope::AllPoints,
            },
            SpecEntry {
                measurement: "bandwidth_3db".to_owned(),
                expression: "bw(V(out))".to_owned(),
                min: Some(1.0),
                max: None,
                unit: "Hz".to_owned(),
                scope: SpecPointScope::AllPoints,
            },
        ],
    );

    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "ac")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain_dc", 31.0)])
            .with_provenance(
                AnalysisResultProvenance::new(
                    crate::product::AnalysisInstanceId::new(),
                    crate::product::ObjectRevision::INITIAL,
                    crate::product::ContentDigest::from_bytes([0x5a; 32]),
                    Vec::new(),
                )
                .expect("valid test provenance"),
            ),
    );
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.view.active_run_idx = Some(0);
}

/// The text filter matches over the cells the table paints, so a surviving row
/// is always one whose reason for surviving is on screen.
#[test]
fn the_specification_filter_narrows_the_table_over_the_cells_it_paints() {
    let filtered = render_with(SimulationPage::Specifications, 1200.0, |app| {
        seed_two_specifications(&mut app.state);
        app.state.simulate.specification_filter = "bw(".to_owned();
    });
    assert!(
        filtered.contains("bandwidth_3db"),
        "the row whose definition matches has to survive:\n{filtered}"
    );
    assert!(
        !filtered.contains("gain_dc"),
        "the row that matches nothing must not be painted:\n{filtered}"
    );
    assert!(
        filtered.contains("showing 1 of 2"),
        "the head reports how much of the registry survived:\n{filtered}"
    );
}

/// The class the head counts and the class the filter keeps are one
/// classification. A row counted as failing and then hidden by "Failing only"
/// would mean the page held two disagreeing opinions about the same evidence.
#[test]
fn the_evidence_class_keeps_exactly_the_rows_the_head_counted() {
    use crate::workbench::state::SpecificationEvidenceFilter;

    let unfiltered = render_with(SimulationPage::Specifications, 1200.0, |app| {
        seed_two_specifications(&mut app.state);
    });
    assert!(
        unfiltered.contains(SpecificationEvidenceFilter::All.label()),
        "the class control paints the class the registry is currently showing:\n{unfiltered}"
    );

    let failing = render_with(SimulationPage::Specifications, 1200.0, |app| {
        seed_two_specifications(&mut app.state);
        app.state.simulate.specification_evidence_filter = SpecificationEvidenceFilter::Failing;
    });
    assert!(
        failing.contains(SpecificationEvidenceFilter::Failing.label()),
        "and it paints the narrowed class once one is picked:\n{failing}"
    );
    assert!(
        failing.contains("1 fail · 1 without results"),
        "the head counts one of each over the whole registry:\n{failing}"
    );
    assert!(
        failing.contains("gain_dc"),
        "the measured-and-failed row is the one `Failing only` keeps:\n{failing}"
    );
    assert!(
        !failing.contains("bandwidth_3db"),
        "a limit no measurement answers is not a failure:\n{failing}"
    );

    let unmapped = render_with(SimulationPage::Specifications, 1200.0, |app| {
        seed_two_specifications(&mut app.state);
        app.state.simulate.specification_evidence_filter =
            SpecificationEvidenceFilter::WithoutEvidence;
    });
    assert!(
        unmapped.contains("bandwidth_3db") && !unmapped.contains("gain_dc"),
        "`Without evidence` keeps exactly the limit the dataset never answered:\n{unmapped}"
    );

    let partial = render_with(SimulationPage::Specifications, 1200.0, |app| {
        seed_two_specifications(&mut app.state);
        app.state.simulate.specification_evidence_filter = SpecificationEvidenceFilter::Partial;
    });
    assert!(
        partial.contains("No specification matches the filter"),
        "no limit here was judged on several points, and the table has to say so rather than \
         read as a plan with no requirements:\n{partial}"
    );
    assert!(
        !partial.contains("nothing judges this plan's results"),
        "the requirements are still there:\n{partial}"
    );
}

/// Each hand-off has to land on the surface its label names. A button that
/// armed a dock behind the page the reader is still looking at is the dead
/// control this program exists to remove.
#[test]
fn every_expression_editor_handoff_opens_the_surface_it_names() {
    use page_outputs::EditorHandoff;

    let mut app = RSpiceApp::test_instance();
    EditorHandoff::Calculator.dispatch(&mut crate::workbench::features::cx(&mut app));
    // The shell answers the hand-off once the page has drawn.
    app.after_contribution();
    assert!(
        app.state.results.waveform_calculator_dialog,
        "the calculator hand-off opens the calculator"
    );

    let mut app = RSpiceApp::test_instance();
    seed_two_specifications(&mut app.state);
    EditorHandoff::UnitDiagnostics.dispatch(&mut crate::workbench::features::cx(&mut app));
    // The shell answers the hand-off once the page has drawn.
    app.after_contribution();
    assert!(
        app.state.results.expression_diagnostics_dialog,
        "the diagnostics hand-off opens the diagnostics window"
    );

    let mut app = RSpiceApp::test_instance();
    seed_two_specifications(&mut app.state);
    EditorHandoff::ScalarMeasurement.dispatch(&mut crate::workbench::features::cx(&mut app));
    // The shell answers the hand-off once the page has drawn.
    app.after_contribution();
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::Results,
        "Add measurement draws over the result viewer, so the hand-off has to arrive there \
         rather than leave the reader on this page"
    );
    assert!(
        matches!(
            app.state.results.result_document_dialog,
            Some(rspice_results_ui::document_dialogs::ResultDocumentDialog::Measurement(_))
        ),
        "the hand-off opens Add measurement"
    );
}

/// A hand-off the plan cannot run has to say why on the page, not only in a
/// tooltip the reader has to guess is there.
#[test]
fn a_dataset_backed_handoff_states_its_condition_before_a_run_exists() {
    use page_outputs::EditorHandoff;

    let rendered = render_with(SimulationPage::Outputs, 1200.0, |app| {
        seed_three_outputs(&mut app.state);
        app.state.simulate.selected_saved_output = Some("vout".to_owned());
        assert!(!app.state.model.simulation.retained.has_results());
    });

    for action in EditorHandoff::ALL {
        assert!(
            rendered.contains(action.label()),
            "the {} hand-off must reach the screen:\n{rendered}",
            action.label()
        );
    }
    assert!(
        rendered.contains("read a retained dataset"),
        "the card has to state why the dataset-backed hand-offs are disabled:\n{rendered}"
    );
    assert_eq!(
        EditorHandoff::ALL
            .iter()
            .filter(|action| action.needs_dataset())
            .count(),
        2,
        "exactly the measurement author and the diagnostics window read a dataset"
    );
    assert!(
        !EditorHandoff::Calculator.needs_dataset(),
        "the calculator composes against the plan, so it is never gated on a run"
    );
}

/// The variables page states its unit and out-of-bounds rules rather than
/// offering a choice the engine does not implement.
#[test]
fn the_variables_page_states_the_rules_it_enforces_instead_of_offering_them() {
    use crate::state::{
        DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableRange,
        DesignVariableScope, DesignVariableSweepEligibility,
    };

    let rendered = render_with(SimulationPage::Variables, 1200.0, |app| {
        let id = plan_id(&app.state.model);
        let variable = DesignVariable::new(
            "rload",
            "1kohm",
            DesignVariableQuantity::Resistance,
            DesignVariableScope::Testbench,
            "load resistance",
            Some(DesignVariableRange {
                minimum: "100ohm".to_owned(),
                maximum: "10kohm".to_owned(),
            }),
            DesignVariableSweepEligibility::FixedParameter,
            DesignVariableOverridePolicy::InheritOwnerOnly,
        )
        .expect("valid design variable");
        app.state
            .model
            .workspace
            .content
            .add_design_variable(id, variable)
            .expect("the plan accepts it");
        app.state.simulate.selected_design_variable = Some("rload".to_owned());
    });

    for stated in ["Unit policy", "Out of bounds"] {
        assert!(
            rendered.contains(stated),
            "the page has to state its {stated} rule:\n{rendered}"
        );
    }
    assert!(
        rendered.contains("never converted"),
        "strict unit checking is the rule, and coercion is not offered:\n{rendered}"
    );
    assert!(
        rendered.contains("never clamped into it"),
        "a bounded variable is never moved into range:\n{rendered}"
    );
    assert!(
        !rendered.contains("enforced at preflight"),
        "the bound is refused on more paths than preflight, and the note now says so:\n{rendered}"
    );
}
