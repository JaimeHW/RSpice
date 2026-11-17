//! The Verify surface over the whole application: each view named by its
//! label, a stored session reopening on its view, and the seeded pages' table
//! grammar.

use crate::workbench::RSpiceApp;
use rspice_app_simulate::workbench::surfaces::verify::toolbar::TOOLBAR_H;
use rspice_app_state::state::VerificationPage;

/// One name per view: the inspector's title and the view's toolbar title are
/// both the view's label, the name its menu row, navigator row and palette
/// entry carry too.
#[test]
fn the_inspector_and_the_toolbar_name_each_view_by_its_label() {
    use rspice_app_simulate::workbench::surfaces::verify::raster::{
        VerifyStage, page_input, page_pass,
    };
    use rspice_app_workbench::workbench::docks::inspector::header;

    for page in VerificationPage::NAVIGATION {
        let mut app = RSpiceApp::test_instance();
        app.open(page);
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut headings = Vec::new();
        for _ in 0..2 {
            let (output, _) = page_pass(&ctx, page_input(Vec::new()), |ui| {
                header(ui, &mut app);
                app.draw(ui);
            });
            headings = output
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::Heading)
                // The toolbar's heading carries its title as its value.
                .map(|(_, node)| {
                    (
                        node.level(),
                        node.label().or(node.value()).map(str::to_owned),
                    )
                })
                .collect::<Vec<_>>();
        }
        let label = Some(page.label().to_owned());
        assert!(
            headings.contains(&(Some(2), label.clone())),
            "{page:?}: the inspector's title among {headings:?}"
        );
        assert!(
            headings.contains(&(None, label)),
            "{page:?}: the toolbar's title among {headings:?}"
        );
    }
}

/// The workspace has one name. The rail, the document bar, the title, the
/// phone's tab bar and the surface catalog all read the workspace's label at
/// every width, and the navigator's header names what it lists, as every
/// other workspace's does.
#[test]
fn every_surface_that_names_the_workspace_reads_verify() {
    use crate::feature_tests::replay::Replay;
    use rspice_app_state::workbench::workspace::Workspace;
    use rspice_app_workbench::workbench::framework::host::{DocumentHost, Host};
    use rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId;

    assert_eq!(Workspace::Verify.label(), "Verify");
    assert_eq!(SurfaceId::Verify.label(), "Verify");
    for size in [
        egui::vec2(1440.0, 900.0),
        egui::vec2(820.0, 1180.0),
        egui::vec2(390.0, 844.0),
    ] {
        let mut replay = Replay::new().sized(size);
        replay.app.state.workbench.activate(Workspace::Verify);
        let controls = replay.frames(Vec::new());
        let old_name = controls
            .iter()
            .filter(|control| {
                [&control.label, &control.value, &control.description]
                    .iter()
                    .any(|text| text.contains("Verification"))
            })
            .map(|control| (control.role, control.label.as_str()))
            .collect::<Vec<_>>();
        assert!(old_name.is_empty(), "{size:?}: {old_name:?}");
        assert!(
            controls.iter().any(|control| control.label == "Verify"),
            "{size:?}: nothing names the open workspace"
        );
        let app = &replay.app;
        assert_eq!(app.title_cell(Workspace::Verify), "Verify");
        assert_eq!(app.navigator_section(Workspace::Verify).title, "Views");
        assert_eq!(
            app.state
                .workspace_documents(Workspace::Verify)
                .iter()
                .map(|document| document.label.as_str())
                .collect::<Vec<_>>(),
            ["Verify"]
        );
    }
}

/// A wide run keeps Corners' worst-case summary in view: in a 1440 x 900
/// window with both docks open, a twelve-point sweep scrolled to its last
/// point still shows Min and Max beside Specification and Limit, every one
/// of them inside the table.
#[test]
fn corners_keeps_its_summary_in_view_on_a_wide_run() {
    use rspice_app_simulate::workbench::surfaces::verify::raster::{
        PageGeometry, page_pass, seed_run_set,
    };
    use rspice_app_state::workbench::workspace::Workspace;

    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    seed_run_set(
        &mut app.state.model,
        &["TT", "SS", "FF"],
        &[1.62, 1.98],
        &[-40.0, 125.0],
    );
    app.state.workbench.activate(Workspace::Verify);
    app.state.simulate.verification_page = VerificationPage::Corners;
    let points = app
        .state
        .model
        .simulation
        .verification_sheet()
        .expect("the sweep is judged")
        .points
        .len();
    assert_eq!(points, 12);
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut frame = eframe::Frame::_new_kittest();
    let mut window = |app: &mut RSpiceApp| -> PageGeometry {
        let mut geometry = PageGeometry::default();
        for _ in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..Default::default()
            };
            geometry = page_pass(&ctx, input, |ui| eframe::App::ui(app, ui, &mut frame)).1;
        }
        geometry
    };
    let last = 4 + points - 1;
    let opened = window(&mut app);
    assert!(
        opened.one(&format!("table.head.{last}")).is_none(),
        "the run is wider than the page"
    );
    app.state.simulate.verification.corners_table.scroll_x = 1.0e5;
    let scrolled = window(&mut app);
    let table = scrolled.one("table.head").expect("the table's head");
    for (column, head) in [
        (0, "Specification"),
        (1, "Limit"),
        (2, "Min"),
        (3, "Max"),
        (last, "the last point"),
    ] {
        let cell = scrolled
            .one(&format!("table.head.{column}"))
            .unwrap_or_else(|| panic!("{head} scrolled out of view"));
        assert!(
            cell.left() >= table.left() - 0.5 && cell.right() <= table.right() + 0.5,
            "{head} at {cell:?} is outside the table at {table:?}"
        );
    }
}

/// An SOA rule's limit line in Results opens Verify's Checks view with that
/// rule's check selected, as a specification's limit line selects its
/// specification, showing every check again when the violations filter hid
/// it. A check the run does not hold is said, and selects nothing.
#[test]
fn an_soa_limit_line_selects_its_check_in_verify() {
    use super::verify_fixtures::soa_app;
    use rspice_app_simulate::workbench::surfaces::verify::raster::VerifyStage as _;
    use rspice_app_simulate::workbench::surfaces::verify::raster::{page_input, page_pass};
    use rspice_app_state::state::{AnalysisResultPayload, SoaParameterEvidence};
    use rspice_app_state::workbench::workspace::Workspace;
    use rspice_app_workbench::workbench::framework::requests::{OpenInVerify, SoaCheckRef};

    let mut app = soa_app();
    app.state.workbench.activate(Workspace::Results);
    app.state.simulate.verification.checks_violations_only = true;
    let analysis = app
        .state
        .model
        .simulation
        .active_run()
        .expect("the SOA run")
        .analyses
        .iter()
        .position(|analysis| {
            matches!(
                analysis.result_payload,
                Some(AnalysisResultPayload::Soa { .. })
            )
        })
        .expect("an SOA analysis");
    let follow = |app: &mut RSpiceApp, device: &str| {
        app.state.workbench.outbox.push(OpenInVerify {
            page: VerificationPage::Checks,
            specification: None,
            check: Some(SoaCheckRef {
                analysis,
                device: device.to_owned(),
                parameter: SoaParameterEvidence::DrainCurrent,
            }),
        });
        app.after_contribution();
        assert_eq!(app.state.workbench.workspace, Workspace::Verify);
        assert_eq!(
            app.state.simulate.verification_page,
            VerificationPage::Checks
        );
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut selected = Vec::new();
        for _ in 0..2 {
            let (output, _) = page_pass(&ctx, page_input(Vec::new()), |ui| app.draw(ui));
            selected = output
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes
                .iter()
                // A selected row is announced toggled on.
                .filter(|(_, node)| node.toggled() == Some(egui::accesskit::Toggled::True))
                .filter_map(|(_, node)| node.label().map(str::to_owned))
                .collect::<Vec<_>>();
        }
        selected
    };

    // M4's drain current passes, so the violations filter hid its row.
    let selected = follow(&mut app, "M4");
    assert!(!app.state.simulate.verification.checks_violations_only);
    assert!(
        matches!(selected.as_slice(), [row] if row.contains("M4") && row.contains("ID max")),
        "{selected:?}"
    );

    let selected = follow(&mut app, "M9");
    assert!(
        app.state
            .prompt_text()
            .is_some_and(|text| text.contains("M9")),
        "a check the run does not hold is said"
    );
    assert!(
        matches!(selected.as_slice(), [row] if row.contains("M4")),
        "and the selection stays: {selected:?}"
    );
}

/// The console's Measurements tab spells a corner run's numbers as Verify
/// does, each in its specification's unit, and names the point the sheet
/// solved each row at, so the gain of one corner reads apart from the next.
#[test]
fn the_console_spells_a_corner_run_as_verify_and_names_each_point() {
    use rspice_app_simulate::workbench::surfaces::verify::raster::seed_run_set;
    use rspice_app_workbench::workbench::docks::console::active_measurement_rows;
    use rspice_results::verification::{difference_text, format_value, limit_text};

    let mut app = RSpiceApp::test_instance();
    seed_run_set(
        &mut app.state.model,
        &["TT", "SS"],
        &[1.62, 1.8],
        &[27.0, 125.0],
    );
    let sheet = app
        .state
        .model
        .simulation
        .verification_sheet()
        .expect("the run is judged");
    let rows = active_measurement_rows(&app);
    assert_eq!(
        rows.len(),
        2 * sheet.points.len(),
        "gain and idd at each point"
    );
    let mut named = Vec::new();
    for row in &rows {
        let specification = sheet
            .rows
            .iter()
            .find(|specification| specification.measurement.eq_ignore_ascii_case(&row.name))
            .expect("each measurement has a specification");
        let point = sheet
            .points
            .iter()
            .position(|point| point.label == row.at)
            .unwrap_or_else(|| panic!("{:?} names a point of the sheet", row.at));
        let cell = &specification.cells[point];
        let value = cell.value.expect("the point measured it");
        let unit = specification.unit.as_str();
        assert_eq!(row.value, format_value(value, unit), "{row:?}");
        assert_eq!(
            Some(row.specification.as_str()),
            limit_text(&specification.comparison, unit).as_deref(),
            "{row:?}"
        );
        assert_eq!(
            row.margin,
            difference_text(cell.signed_margin.expect("judged"), value, unit),
            "{row:?}"
        );
        named.push((row.name.clone(), row.at.clone()));
    }
    named.sort();
    named.dedup();
    assert_eq!(named.len(), rows.len(), "no two rows read alike");

    let failing = rows
        .iter()
        .find(|row| row.failed)
        .expect("gain fails at the slow corner");
    assert_eq!(
        (
            failing.name.as_str(),
            failing.value.as_str(),
            failing.specification.as_str(),
            failing.margin.as_str(),
            failing.at.as_str(),
        ),
        (
            "gain",
            "39.40 dB",
            "\u{2265} 40 dB",
            "-0.60 dB",
            "SS 1.62 V 125 \u{b0}C"
        ),
        "the row reads as Verify's Specifications states it"
    );
}

/// A run whose limit was edited after it ran: the console's Measurements
/// table states what Verify's sheet says against the plan as it stands, and a
/// publication of the run keeps the verdict it was sealed with. The two
/// differ, which is the point: one is the live judgement, the other a record.
#[test]
fn the_console_reads_the_sheet_and_a_publication_keeps_the_sealed_verdict() {
    use crate::feature_tests::hardcopy::publication_reads;
    use rspice_app_hardcopy::workbench::hardcopy_adapters::publication_snapshot::build_publication_snapshot;
    use rspice_app_state::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use rspice_app_state::state::{
        AnalysisResult, AnalysisResultProvenance, AnalysisResultSourceDomain, AnalysisType,
        PreparedRunReceipt, PreparedRunReceiptInput, PreparedRunTaskReceipt,
        PreparedSourceCheckReceipt, SimulationRun, SpecEntry, SpecPointScope,
    };
    use rspice_app_workbench::workbench::docks::console::active_measurement_rows;
    use rspice_app_workbench::workbench::framework::feature::PublicationDraft;
    use rspice_results::specification::{PreparedSpecification, PreparedSpecificationPolicy};
    use rspice_results::specification_verdict::{SpecificationVerdict, SpecificationVerdictStatus};
    use rspice_results::verification::{CellStatus, LimitsSinceRun};

    let gain_at_least = |minimum: f64| SpecEntry {
        measurement: "gain".to_owned(),
        expression: String::new(),
        min: Some(minimum),
        max: None,
        unit: "dB".to_owned(),
        scope: SpecPointScope::AllPoints,
    };
    let mut app = RSpiceApp::test_instance();
    let model = &mut app.state.model;
    let plan_id = model
        .sim_setup
        .stable_analysis_plan()
        .expect("the project has a plan")
        .id();
    // Sealed against gain >= 40 dB, which the measured 39.4 dB fails.
    let source = AnalysisInstanceId::new();
    let receipt = PreparedRunReceipt::new(PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(plan_id),
        project_revision: ObjectRevision::INITIAL,
        prepared_snapshot_digest: ContentDigest::from_bytes([0x5a; 32]),
        source_content_digest: ContentDigest::from_bytes([0x41; 32]),
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes(
            [0x42; 32],
        )),
        project_model_sources: Vec::new(),
        specifications: vec![PreparedSpecification::new(gain_at_least(40.0)).expect("a limit")],
        specification_policy: PreparedSpecificationPolicy::default(),
        tasks: vec![
            PreparedRunTaskReceipt::new(
                source,
                ObjectRevision::INITIAL,
                Vec::new(),
                2,
                ContentDigest::from_bytes([2; 32]),
            )
            .expect("a task receipt"),
        ],
    })
    .expect("a run receipt");
    let mut run = SimulationRun::new_prepared(1, receipt);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 39.4)])
            .with_provenance(
                AnalysisResultProvenance::new(
                    source,
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([0x5a; 32]),
                    Vec::new(),
                )
                .expect("analysis provenance"),
            ),
    );
    model.simulation.retained.runs = vec![run].into();
    assert!(model.simulation.select_run(0));
    // The plan's limit, lowered after the run to gain >= 35 dB.
    model
        .workspace
        .content
        .replace_active_specs(plan_id, vec![gain_at_least(35.0)]);
    model
        .simulation
        .refresh_verification(&model.sim_setup, &model.workspace.content);

    let sheet = model
        .simulation
        .verification_sheet()
        .expect("the run is judged");
    assert_eq!(sheet.summary.limits, LimitsSinceRun::Edited);
    let point = sheet.point_of_analysis(0).expect("the analysis is a point");
    assert_eq!(sheet.rows[0].cells[point].status, CellStatus::Pass);
    let run = model.simulation.active_run().expect("the run is selected");
    let sealed = SpecificationVerdict::evaluate(
        run.prepared_receipt().expect("sealed").specifications(),
        run.analyses.iter().map(|analysis| &analysis.data),
    );
    assert_eq!(sealed[0].status(), SpecificationVerdictStatus::BoundFailure);

    let console = active_measurement_rows(&app);
    let [row] = console.as_slice() else {
        panic!("one measurement: {console:?}");
    };
    assert_eq!(
        (row.status, row.failed, row.specification.as_str()),
        ("PASS", false, "≥ 35 dB"),
        "the console states the sheet's verdict against the plan's limit"
    );

    let snapshot = build_publication_snapshot(
        publication_reads(&app.state),
        &PublicationDraft {
            title: "Edited limits".to_owned(),
            author_display: "Test".to_owned(),
            created_utc: "2026-11-16T00:00:00Z".to_owned(),
            license: rspice_publication_contract::ContentLicense::CcBy40,
            ..Default::default()
        },
    )
    .expect("the run publishes");
    let results = snapshot.results.expect("the run's results");
    let [published] = results.measurements.as_slice() else {
        panic!("one measurement: {:?}", results.measurements);
    };
    assert_eq!(
        (published.passed, published.spec_display.as_deref()),
        (Some(false), Some("≥ 40 dB")),
        "the publication keeps the verdict the run was sealed with"
    );
    assert_ne!(
        published.passed,
        Some(!row.failed),
        "the two must differ here"
    );
}

/// A stored session reopens on the Verify view it was left on, stored and
/// read back the way the application does it.
#[test]
fn a_stored_session_reopens_on_its_verify_view() {
    #[derive(Default)]
    struct MemoryStorage(std::collections::HashMap<String, String>);

    impl eframe::Storage for MemoryStorage {
        fn get_string(&self, key: &str) -> Option<String> {
            self.0.get(key).cloned()
        }

        fn set_string(&mut self, key: &str, value: String) {
            self.0.insert(key.to_owned(), value);
        }

        fn remove_string(&mut self, key: &str) {
            self.0.remove(key);
        }

        fn flush(&mut self) {}
    }

    for page in VerificationPage::NAVIGATION {
        let mut app = RSpiceApp::test_instance();
        app.state.simulate.verification_page = page;
        let mut storage = MemoryStorage::default();
        app.state.store_session(&mut storage);
        let restored = crate::workbench::app_state::AppState::restore_eframe_session(&storage)
            .expect("RSpice restores a session it just stored");
        assert_eq!(restored.simulate.verification_page, page);
    }
}

/// The shell's filter field narrows the open view's rows, so it names them:
/// specifications on the views listed by specification, checks on the rest.
#[test]
fn the_filter_field_names_the_rows_of_the_open_view() {
    use rspice_app_state::workbench::workspace::Workspace;
    use rspice_app_workbench::workbench::framework::host::Host;

    let mut app = RSpiceApp::test_instance();
    for (page, placeholder) in [
        (
            VerificationPage::Specifications,
            "Filter specifications\u{2026}",
        ),
        (VerificationPage::Corners, "Filter specifications\u{2026}"),
        (
            VerificationPage::MonteCarlo,
            "Filter specifications\u{2026}",
        ),
        (VerificationPage::Checks, "Filter checks\u{2026}"),
        (VerificationPage::Reference, "Filter checks\u{2026}"),
    ] {
        app.state.simulate.verification_page = page;
        assert_eq!(
            app.navigator_section(Workspace::Verify).placeholder,
            placeholder,
            "{page:?}"
        );
    }
    assert_eq!(
        app.navigator_section(Workspace::Simulate).placeholder,
        "Filter setup\u{2026}"
    );
}

/// Every seeded page opens with a 36 px toolbar whose title starts on the
/// panel inset, then a table of 24 px head and rows directly under it, with
/// every right-aligned column ending on one edge and nothing past the insets.
#[test]
fn seeded_verify_pages_follow_the_table_grammar() {
    use super::verify_fixtures::{corner_app, monte_carlo_app, soa_app};
    use rspice_app_simulate::workbench::surfaces::verify::raster::{PAGE_WIDTH, page_geometry};

    for (page, app) in [
        (VerificationPage::Specifications, corner_app()),
        (VerificationPage::MonteCarlo, monte_carlo_app()),
        (VerificationPage::Corners, corner_app()),
        (VerificationPage::Checks, soa_app()),
    ] {
        let geometry = page_geometry(app, page);
        let toolbar = geometry.one("toolbar").expect("toolbar");
        assert_eq!(toolbar.height(), TOOLBAR_H, "{page:?}");
        assert_eq!(toolbar.left(), 0.0, "{page:?}");
        assert_eq!(
            geometry.one("toolbar.title").expect("title").left(),
            10.0,
            "{page:?}"
        );
        let head = geometry.one("table.head").expect("table head");
        assert_eq!(head.height(), 24.0, "{page:?}");
        // Band rows naming runs of columns are the head's own top rows.
        let bands = geometry.all("table.band");
        assert_eq!(
            bands.first().map_or(head.top(), |band| band.top()),
            toolbar.bottom(),
            "{page:?}: nothing between toolbar and table"
        );
        if let Some(band) = bands.last() {
            assert_eq!(head.top(), band.bottom(), "{page:?}");
        }
        let rows = geometry.all("table.row");
        assert!(!rows.is_empty(), "{page:?}");
        assert!(rows.iter().all(|row| row.height() == 24.0), "{page:?}");
        assert_eq!(rows[0].top(), head.bottom(), "{page:?}");
        for (column, edges) in geometry.right_edges() {
            assert_eq!(
                edges.len(),
                1,
                "{page:?} {column}: numbers end on one edge, {edges:?}"
            );
        }
        assert_eq!(
            geometry.outside_insets(PAGE_WIDTH),
            Vec::<String>::new(),
            "{page:?}"
        );
        assert!(
            geometry.one("toolbar.summary").is_some(),
            "{page:?} states a summary"
        );
    }
}
