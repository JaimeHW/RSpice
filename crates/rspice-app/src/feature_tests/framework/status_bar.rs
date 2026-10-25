use crate::simulation::status::EngineAvailability;
use crate::simulation::status::{
    preflight_status, simulation_engine_status, simulation_progress_percent,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Problem;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::diagnostic_store::listing::DiagnosticCounts;
use crate::workbench::chrome::status_bar::design::{schematic_cursor_summary, selection_summary};
use crate::workbench::chrome::status_bar::results::{format_plot_zoom, results_view_summary};
use crate::workbench::chrome::status_bar::sheets::sheet_position_summary;
use crate::workbench::chrome::status_bar::*;
use crate::workbench::framework::host::StatusMark;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::Workspace;

/// The counts the bar states, read as the page reads them.
fn counts(app: &RSpiceApp) -> DiagnosticCounts {
    crate::workbench::app_state::diagnostic_store::listing::listing(&app.state).counts()
}

/// A rule separates two items. The last item on the left used to draw
/// one anyway, so a rule stood alone in front of the empty middle of the
/// bar. Every rule now has an item's ink within a gutter on both sides.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_status_rule_stands_between_two_items() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    let size = egui::vec2(1440.0, 900.0);
    let canvas = crate::ui::raster::render(size, |ui, _| {
        let layout = LayoutSpec::resolve(size.x, size.y, &app.state.workbench);
        show(ui, &mut app, layout);
    });
    let themed = egui::Context::default();
    crate::ui::Theme::default().apply(&themed);
    let border = Tokens::get(&themed).color.border;
    let y = size.y - 11.0;
    let row = (0..size.x as usize)
        .map(|x| {
            let rect = egui::Rect::from_min_size(egui::pos2(x as f32, y), egui::vec2(1.0, 1.0));
            canvas.pixels_in(rect).next().expect("pixel on the canvas")
        })
        .collect::<Vec<_>>();
    let fill = *row
        .iter()
        .max_by_key(|colour| row.iter().filter(|other| other == colour).count())
        .expect("a status row");
    let ink = |range: std::ops::Range<usize>| {
        row[range]
            .iter()
            .any(|colour| *colour != fill && *colour != border)
    };
    let rules = (1..row.len() - 1)
        .filter(|&x| row[x] == border)
        .collect::<Vec<_>>();
    assert!(!rules.is_empty(), "the bar draws its rules");
    for x in rules {
        assert!(
            ink(x.saturating_sub(14)..x) && ink(x + 1..(x + 15).min(row.len())),
            "the rule at x = {x} faces empty bar"
        );
    }
}

/// A prompt is the bar's answer to the gesture just made: drawn, and
/// announced politely, until it expires, and then gone from the bar and
/// from the state that held it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_bar_draws_the_prompt_and_drops_it_once_it_expires() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    app.state.prompt("The schematic is read-only.");

    let size = egui::vec2(1440.0, 900.0);
    for expired in [false, true] {
        if expired {
            app.state
                .workbench
                .prompt
                .as_mut()
                .expect("the prompt is still held")
                .issued = crate::time_compat::Instant::now()
                .checked_sub(crate::workbench::state::StatusPrompt::LIFETIME)
                .expect("the session clock reaches back one prompt lifetime");
        }
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            },
            |ui| {
                let layout = LayoutSpec::resolve(size.x, size.y, &app.state.workbench);
                show(ui, &mut app, layout);
            },
        );
        let announced: Vec<_> = output
            .platform_output
            .accesskit_update
            .expect("status bar accessibility tree")
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Status)
            .map(|(_, node)| (node.label().map(str::to_owned), node.live()))
            .collect();
        if expired {
            assert!(announced.is_empty(), "{announced:?}");
            assert!(app.state.workbench.prompt.is_none());
        } else {
            assert_eq!(
                announced,
                vec![(
                    Some("The schematic is read-only.".to_owned()),
                    Some(egui::accesskit::Live::Polite)
                )]
            );
        }
    }
}

/// The segment states the same count wherever the reader is standing.
///
/// It used to swap between the netlist summary and the schematic result
/// depending on the open workspace, so the bar reported a different
/// number of problems on the Netlist page than on the Design page in a
/// session that had both.
#[test]
fn the_checks_segment_counts_every_producer_from_one_list() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Netlist);
    let diagnostics = vec![
        crate::state::netlist_diagnostics::Diagnostic::error("bad card"),
        crate::state::netlist_diagnostics::Diagnostic::current(
            "rspice.test",
            "TEST-WARNING",
            crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning,
            "implicit option",
        ),
    ];
    app.state.code.netlist.diagnostics = std::sync::Arc::new(
        crate::state::netlist_diagnostics::NetlistDiagnosticCollection::try_new(diagnostics, "")
            .unwrap(),
    );

    assert_eq!(
        check_summary(counts(&app)).as_deref(),
        Some("Problems · 1 error · 1 advisory")
    );
    assert_eq!(check_problem(counts(&app)), Some(Problem::Failure));

    app.state.workbench.activate(Workspace::Design);
    assert_eq!(
        check_summary(counts(&app)).as_deref(),
        Some("Problems · 1 error · 1 advisory"),
        "the count cannot depend on which workspace is open"
    );
}

/// The checks segment is a problem count. A closed project, or checks
/// that have never run, put nothing on the bar; zero and zero is a count
/// with no mark; warnings and staleness earn a warning, errors a failure.
#[test]
fn the_checks_segment_marks_only_a_problem_and_is_absent_until_checks_run() {
    use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

    let violation = |kind| {
        DrcViolation::new(
            1,
            kind,
            "fixture",
            DrcLocation::NetLabel {
                name: "OUT".to_owned(),
            },
        )
    };
    let publish = |app: &mut RSpiceApp, result: DrcResult| {
        app.state
            .publish_active_design_check_result(result)
            .expect("the fixture publishes a current receipt");
    };
    let completed = |violation: Option<DrcViolation>| {
        let mut result = DrcResult::new();
        if let Some(violation) = violation {
            result.add_violation(violation);
        }
        result.completed = true;
        result
    };

    let mut app = RSpiceApp::test_instance();
    app.state.close_project_session();
    assert_eq!(check_summary(counts(&app)), None);
    assert_eq!(check_problem(counts(&app)), None);

    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    assert_eq!(
        check_summary(counts(&app)),
        None,
        "checks that never ran say nothing"
    );
    assert_eq!(check_problem(counts(&app)), None);

    publish(&mut app, completed(None));
    assert_eq!(
        check_summary(counts(&app)),
        None,
        "zero and zero puts nothing on the bar"
    );
    assert_eq!(check_problem(counts(&app)), None);

    publish(
        &mut app,
        completed(Some(violation(DrcViolationType::UnconnectedPin))),
    );
    assert_eq!(check_problem(counts(&app)), Some(Problem::Warning));

    publish(
        &mut app,
        completed(Some(violation(DrcViolationType::ShortedOutputs))),
    );
    assert_eq!(check_problem(counts(&app)), Some(Problem::Failure));
    assert_eq!(
        check_summary(counts(&app)).as_deref(),
        Some("Problems · 1 error · 0 advisories")
    );

    // An edit under the evidence leaves the finding standing and faint:
    // the count moves to the stale column, which is neither an error nor
    // an advisory about the design in front of the reader.
    app.state
        .model
        .workspace
        .content
        .connectivity
        .policy
        .width_mismatch = crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;
    assert_eq!(
        check_summary(counts(&app)).as_deref(),
        Some("Problems · 0 errors · 0 advisories · 1 stale")
    );
    assert_eq!(check_problem(counts(&app)), Some(Problem::Warning));
}

/// Findings whose inputs cannot be read are said to be that, not stale:
/// nothing is known to have moved under them.
#[test]
fn findings_that_cannot_be_evaluated_are_not_called_stale() {
    let counts = DiagnosticCounts {
        unavailable: 2,
        ..DiagnosticCounts::default()
    };
    assert_eq!(
        check_summary(counts).as_deref(),
        Some("Problems · 0 errors · 0 advisories · 2 unavailable")
    );
    assert_eq!(check_problem(counts), Some(Problem::Warning));
}

/// A note or a hint is counted, but it is not a warning and raises no
/// warning mark; it used to, because every advisory did.
#[test]
fn only_a_warning_raises_the_warning_mark() {
    let notes = DiagnosticCounts {
        advisories: 2,
        ..DiagnosticCounts::default()
    };
    assert_eq!(
        check_summary(notes).as_deref(),
        Some("Problems · 0 errors · 2 advisories")
    );
    assert_eq!(check_problem(notes), None);

    let warned = DiagnosticCounts {
        warnings: 1,
        ..notes
    };
    assert_eq!(check_problem(warned), Some(Problem::Warning));
}

/// The coordinate and selection segments sit side by side. They answer
/// different questions, so neither may borrow the other's sentence — with
/// no dataset open the bar used to print "No result dataset selected"
/// twice in a row.
#[test]
fn adjacent_result_segments_never_print_the_same_sentence() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Results);
    app.state.open_project_session();

    let coordinates = results_view_summary(&app, None);
    let selection = selection_summary(&app);
    assert_ne!(
        coordinates, selection,
        "two segments cannot report the same fact in the same words"
    );
    assert_eq!(coordinates, "No plotted interval");
    assert_eq!(selection, "No result dataset selected");
}

#[test]
fn the_zoom_chip_drops_the_decimal_only_once_magnification_is_deep() {
    assert_eq!(format_plot_zoom(1.0), "1×");
    assert_eq!(format_plot_zoom(2.5), "2.5×");
    assert_eq!(format_plot_zoom(9.94), "9.9×");
    assert_eq!(format_plot_zoom(10.0), "10×");
    assert_eq!(format_plot_zoom(137.4), "137×");
}

#[test]
fn dense_status_metadata_follows_the_mockup_nine_hundred_pixel_cutoff() {
    assert_eq!(
        status_visibility(900.0),
        StatusVisibility {
            coordinates: false,
            selection: false,
            platform: false,
        }
    );
    assert_eq!(
        status_visibility(901.0),
        StatusVisibility {
            coordinates: true,
            selection: true,
            platform: true,
        }
    );
}

#[test]
fn status_item_geometry_matches_the_mockup_contract() {
    assert_eq!(STATUS_FONT_SIZE, tokens::FS_1);
    assert_eq!(
        status_text_font(),
        theme::mono(tokens::FS_1, FontWeight::Regular)
    );
}

#[test]
fn status_groups_use_intrinsic_widths_and_space_between() {
    assert_eq!(
        status_group_allocation(420.0, 310.0, 1_440.0),
        StatusGroupAllocation {
            left: 420.0,
            right: 310.0,
            gap: 710.0,
        }
    );
}

#[test]
fn status_groups_compress_proportionally_only_when_they_overlap() {
    let allocation = status_group_allocation(600.0, 400.0, 900.0);
    assert!((allocation.left - 540.0).abs() < f32::EPSILON);
    assert!((allocation.right - 360.0).abs() < f32::EPSILON);
    assert_eq!(allocation.gap, 0.0);
}

#[test]
fn fractional_engine_progress_is_rendered_as_a_percentage() {
    assert_eq!(simulation_progress_percent(-1.0), 0);
    assert_eq!(simulation_progress_percent(0.375), 38);
    assert_eq!(simulation_progress_percent(1.5), 100);
}

#[test]
fn engine_status_follows_execution_lifecycle_not_worker_activity() {
    let mut simulation = crate::state::SimulationState::default();
    let identity = simulation
        .start_run()
        .execution_identity()
        .expect("current run has execution identity");
    simulation.execution.active_execution = Some(identity);
    simulation.execution.progress = 0.375;
    simulation.execution.is_running = false;

    assert_eq!(
        simulation_engine_status(&simulation, &EngineAvailability::Ready).as_deref(),
        Some("Engine preparing · 38%")
    );

    simulation.request_abort_active_run().unwrap();
    assert_eq!(
        simulation_engine_status(&simulation, &EngineAvailability::Ready).as_deref(),
        Some("Engine stopping · 38%")
    );
}

/// An idle engine that is ready is nothing to report, so the bar shows no
/// engine segment at all; every other idle state still names itself.
#[test]
fn idle_engine_status_says_nothing_while_the_engine_is_ready() {
    let simulation = crate::state::SimulationState::default();
    for (availability, expected) in [
        (EngineAvailability::Ready, None),
        (EngineAvailability::Starting, Some("Engine starting")),
        (EngineAvailability::Restartable, Some("Engine stopped")),
        (
            EngineAvailability::Unavailable("worker module missing".into()),
            Some("Engine unavailable"),
        ),
    ] {
        assert_eq!(
            simulation_engine_status(&simulation, &availability).as_deref(),
            expected
        );
    }
}

/// The engine segment's one dot means a run is live. Unavailability is a
/// failure mark, and no other state carries a mark at all.
#[test]
fn the_engine_mark_is_a_live_dot_only_while_a_run_is_in_progress() {
    let unavailable = EngineAvailability::Unavailable("worker module missing".into());
    assert_eq!(
        engine_status_mark(&unavailable, false),
        StatusMark::Problem(Problem::Failure)
    );
    assert_eq!(
        engine_status_mark(&unavailable, true),
        StatusMark::Problem(Problem::Failure)
    );
    assert_eq!(
        engine_status_mark(&EngineAvailability::Ready, true),
        StatusMark::Live
    );
    for availability in [
        EngineAvailability::Ready,
        EngineAvailability::Starting,
        EngineAvailability::Restartable,
    ] {
        assert_eq!(engine_status_mark(&availability, false), StatusMark::None);
    }
}

#[test]
fn backend_startup_and_failure_do_not_advertise_running_progress() {
    let mut simulation = crate::state::SimulationState::default();
    let identity = simulation.start_run().execution_identity().unwrap();
    simulation.execution.active_execution = Some(identity);
    simulation.execution.progress = 0.375;
    assert_eq!(
        simulation_engine_status(&simulation, &EngineAvailability::Starting).as_deref(),
        Some("Engine starting")
    );
    assert_eq!(
        simulation_engine_status(
            &simulation,
            &EngineAvailability::Unavailable("worker initialization failed".into())
        )
        .as_deref(),
        Some("Engine unavailable")
    );
}

/// The page segment is the reader's position in a multi-sheet drawing. It
/// follows activation because it projects the catalog rather than holding
/// a copy, and it is absent below two sheets because `p1/1` is not a
/// position.
#[test]
fn the_page_segment_appears_only_above_one_sheet_and_follows_activation() {
    use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};

    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let key = app
        .state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let first = app
        .state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input stage", [10])
        .expect("the fixture cell view takes a sheet catalog");
    assert_eq!(sheet_position_summary(&app.state.model), None);

    let catalog = app
        .state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("the catalog was just bootstrapped");
    for (page, name) in [(2, "Output stage"), (3, "Supplies"), (4, "Test points")] {
        catalog
            .create_sheet(
                SheetDefinition {
                    name: name.to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(page),
                },
                None,
            )
            .expect("a further sheet");
    }
    catalog
        .set_active(first)
        .expect("the first sheet is active");
    let second = catalog.sheets()[1].id();
    assert_eq!(
        sheet_position_summary(&app.state.model).map(|(segment, _)| segment),
        Some("p1/4".to_owned())
    );

    crate::workbench::app::sheets::activate_sheet(
        &mut app.state.model,
        &mut app.state.workbench,
        second,
    )
    .expect("the second sheet activates");
    assert_eq!(
        sheet_position_summary(&app.state.model),
        Some((
            "p2/4".to_owned(),
            "Sheet 2 of 4 \u{00b7} Output stage".to_owned()
        ))
    );
}

#[test]
fn schematic_coordinates_use_the_permanent_sheet_origin_and_status_context() {
    let mut state = crate::workbench::app_state::AppState::default();
    state.model.schematic.document_mut_for_test().grid_size = 10;
    assert_eq!(
        schematic_cursor_summary(&state, -14.0, -4.0),
        "x 0 · y 0 mm"
    );
    assert_eq!(
        schematic_cursor_summary(&state, -15.0, -4.0),
        "x -2.5 · y 0 mm · off sheet"
    );
}

/// A preflight pass names its stage, and counts once it knows what it
/// works through: the run's points while preparing, its decks while the
/// engine builds them. A run of one point is not counted.
#[test]
fn a_preflight_pass_states_its_stage_and_how_far_it_has_got() {
    use crate::simulation::controller::PreflightStage;
    use crate::simulation::execution::{DeckBuildProgress, PreparationProgress};

    let preparing = |prepared, total| {
        preflight_status(PreflightStage::Preparing(PreparationProgress {
            prepared,
            total,
        }))
    };
    let checking = |built, total| {
        preflight_status(PreflightStage::EngineChecking(DeckBuildProgress {
            built,
            total,
        }))
    };
    assert_eq!(preparing(0, 0), "Preparing");
    assert_eq!(preparing(1, 1), "Preparing");
    assert_eq!(preparing(3, 9), "Preparing · 3 of 9 points");
    assert_eq!(checking(0, 0), "Engine checking");
    assert_eq!(checking(0, 1), "Engine checking · 0 of 1 deck");
    assert_eq!(checking(2, 3), "Engine checking · 2 of 3 decks");
}
