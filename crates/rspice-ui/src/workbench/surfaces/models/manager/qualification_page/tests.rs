//! What the qualification matrix is allowed to claim, and where it stops.
//!
//! The page's whole subject is coverage, so the assertions here are about the
//! difference between an absence and a zero: a family that never wrote a
//! transient vector and a family whose transient vectors all fail are two
//! different findings, and a matrix that spelled them the same way would be
//! worse than the badge it replaced.

use super::*;

use super::super::super::ENGINE_OWNED_BAND;
use crate::state::model_library::{ModelLibraryManager, ProjectModelDefinition};
use crate::workbench::documents::model_editor::{
    QualificationAuthoringAnalysis, QualificationAuthoringProbe, QualificationAuthoringSample,
};
use crate::workbench::state::ModelsPage;

/// The document column in the shell at a 1680 px window, and a viewport tall
/// enough to show the composition the page is measured against.
const PAGE_SIZE: [f32; 2] = [1_180.0, 900.0];

/// A project owning one model with retained DC, AC/charge and transient
/// suites — and deliberately no noise suite.
///
/// Authored through the model editor's own transaction rather than by writing
/// a `ModelQualificationState` by hand: a suite is only counted when its
/// vectors bind the *exact* resolved source revision, and a fixture that
/// assembled that binding itself would be testing the fixture.
fn source_owned_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager = ModelLibraryManager::new();
    let definition = ProjectModelDefinition {
        name: "nch_owned".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Project-owned qualification fixture".to_owned(),
        numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0), ("vth0".to_owned(), 0.48)]),
        string_parameters: BTreeMap::new(),
    };
    app.state
        .model_library_manager
        .create_project_model("owned-models", &definition)
        .expect("create project model");

    let summary = qualification_summaries(&app)
        .into_iter()
        .find(|summary| summary.model == "nch_owned")
        .expect("the project-owned family is summarized");
    app.state
        .model_library_manager
        .select_library(&summary.library)
        .expect("the fixture library is loaded");
    app.state.workbench.selected_model = Some(summary.model.clone());
    execute_qualification_action(&mut app, QualificationPageAction::ReviewVectors);

    let editor = &mut app.state.workbench.model_editor;

    editor.begin_qualification_suite();
    {
        let fields = &mut editor.qualification_authoring;
        fields.suite_id = "dc-op".to_owned();
        fields.suite_name = "DC operating point".to_owned();
        fields.vector_id = "nominal".to_owned();
        fields.vector_name = "Nominal bias".to_owned();
        fields.executable_input =
            "V1 out 0 1\nR1 out 0 1k\nMbind 0 0 0 0 nch_owned\n.op\n.end\n".to_owned();
        fields.quantity = "v(out)".to_owned();
        fields.probe_target = "out".to_owned();
        fields.expected = "1".to_owned();
        fields.absolute_tolerance = "1e-9".to_owned();
        fields.relative_tolerance = "1e-9".to_owned();
    }
    assert!(
        editor.commit_qualification_suite(),
        "{:?}",
        editor.qualification_authoring.error
    );

    editor.begin_qualification_suite();
    {
        let fields = &mut editor.qualification_authoring;
        fields.suite_id = "ac-cv".to_owned();
        fields.suite_name = "AC and capacitance".to_owned();
        fields.vector_id = "ac-nominal".to_owned();
        fields.vector_name = "Nominal small-signal".to_owned();
        fields.analysis = QualificationAuthoringAnalysis::AcSweep;
        fields.frequencies = "1e3, 1e4".to_owned();
        fields.probe = QualificationAuthoringProbe::AcEffectiveCapacitance;
        fields.probe_target = "V1".to_owned();
        fields.excitation_magnitude = "1".to_owned();
        fields.sample = QualificationAuthoringSample::FirstFrequencyPoint;
        fields.executable_input =
            "V1 out 0 DC 0 AC 1\nR1 out 0 1k\nMbind 0 0 0 0 nch_owned\n.end\n".to_owned();
        fields.quantity = "ceff(V1)".to_owned();
        fields.expected = "0".to_owned();
        fields.absolute_tolerance = "1e-15".to_owned();
        fields.relative_tolerance = "0".to_owned();
    }
    assert!(
        editor.commit_qualification_suite(),
        "{:?}",
        editor.qualification_authoring.error
    );

    editor.begin_qualification_suite();
    {
        let fields = &mut editor.qualification_authoring;
        fields.suite_id = "transient".to_owned();
        fields.suite_name = "Transient".to_owned();
        fields.vector_id = "transient-nominal".to_owned();
        fields.vector_name = "Nominal waveform".to_owned();
        fields.executable_input = "V1 out 0 DC 1\nMbind out out 0 0 nch_owned\n.end\n".to_owned();
        fields.analysis = QualificationAuthoringAnalysis::Transient;
        fields.transient_stop_time = "1e-6".to_owned();
        fields.transient_max_step = "1e-7".to_owned();
        fields.quantity = "v(out)".to_owned();
        fields.probe = QualificationAuthoringProbe::TransientNodeVoltage;
        fields.probe_target = "out".to_owned();
        fields.sample = QualificationAuthoringSample::LastTimePoint;
        fields.expected = "1".to_owned();
        fields.absolute_tolerance = "1e-8".to_owned();
        fields.relative_tolerance = "1e-8".to_owned();
    }
    assert!(
        editor.commit_qualification_suite(),
        "{:?}",
        editor.qualification_authoring.error
    );

    app.state.workbench.models_page = ModelsPage::Qualification;
    app
}

fn owned_summary(app: &RSpiceApp) -> QualificationModelSummary {
    qualification_summaries(app)
        .into_iter()
        .find(|summary| summary.model == "nch_owned")
        .expect("the project-owned family is summarized")
}

fn themed_context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

/// The header cannot describe a column the rows do not fill, and the columns
/// cannot describe an analysis a vector cannot name.
#[test]
fn the_matrix_columns_name_every_analysis_domain_and_tile_the_track() {
    let domain_headings = MATRIX_COLUMNS
        [DOMAIN_COLUMN..DOMAIN_COLUMN + QualificationDomain::ALL.len()]
        .iter()
        .map(|(label, _)| *label)
        .collect::<Vec<_>>();
    assert_eq!(
        domain_headings,
        QualificationDomain::ALL
            .map(QualificationDomain::column_label)
            .to_vec(),
        "a domain column heading has drifted from the domain it counts"
    );
    assert_eq!(MATRIX_COLUMNS[REFERENCES_COLUMN].0, "REFERENCES");
    assert_eq!(MATRIX_COLUMNS[GATE_COLUMN].0, "GATE");
    let track = MATRIX_COLUMNS
        .iter()
        .map(|(_, fraction)| fraction)
        .sum::<f32>();
    assert!(
        (track - 1.0).abs() <= 1.0e-4,
        "the columns take {track} of the track rather than all of it"
    );
}

/// A domain the family never wrote a vector against reads as absent, and one
/// it did reads as its own counts.
///
/// This is the whole reason the page is a matrix. "0 / 0" against an
/// undeclared domain is a coverage claim nobody made.
#[test]
fn an_undeclared_domain_reads_absent_and_a_declared_one_reads_its_own_counts() {
    let app = source_owned_app();
    let summary = owned_summary(&app);
    assert_eq!(summary.suites, 3, "the fixture retains three suites");
    let ctx = themed_context();
    let t = Tokens::get(&ctx);

    for domain in [
        QualificationDomain::Dc,
        QualificationDomain::Ac,
        QualificationDomain::Transient,
    ] {
        let (cell, color) = domain_cell(&summary, domain, &t);
        assert_eq!(
            cell, "0 / 1",
            "{domain:?} retains one vector and no passing evidence"
        );
        assert_eq!(
            color, t.color.warn,
            "{domain:?} declares a vector nothing has passed and is not toned as a pass"
        );
    }

    let (noise, color) = domain_cell(&summary, QualificationDomain::Noise, &t);
    assert_eq!(
        noise, "—",
        "a domain the family never declared must not read as a count"
    );
    assert_eq!(color, t.color.text_faint);
    assert!(
        summary.domain(QualificationDomain::Noise).is_none(),
        "the projection invented a noise domain the suites do not hold"
    );
}

/// The gate exemption is the family's own row, not a colour on it.
#[test]
fn an_engine_owned_family_states_the_exemption_in_every_cell() {
    let app = RSpiceApp::test_instance();
    let summaries = qualification_summaries(&app);
    let summary = summaries
        .iter()
        .find(|summary| !summary.gate.is_gate_subject())
        .expect("a fresh project holds the engine-owned families");
    let ctx = themed_context();
    let t = Tokens::get(&ctx);
    for domain in QualificationDomain::ALL {
        assert_eq!(domain_cell(summary, domain, &t).0, "—");
    }
    assert_eq!(gate_detail(summary), "compiled in");
    assert_eq!(gate_color(summary.gate, &t), t.color.text_dim);
}

/// Every cell a sighted reader can see is announced, because none of them is a
/// widget: the row is painted, and the row's node is the only thing a screen
/// reader has.
#[test]
fn a_matrix_row_announces_every_domain_it_paints() {
    let app = source_owned_app();
    let announced = row_announcement(&owned_summary(&app));
    for domain in QualificationDomain::ALL {
        assert!(
            announced.contains(domain.label()),
            "the row says nothing about {domain:?}: {announced}"
        );
    }
    assert!(
        announced.contains("not declared"),
        "the undeclared noise domain is announced as a count: {announced}"
    );
    assert!(announced.contains("nch_owned") && announced.contains("owned-models"));
}

/// The engine-owned band is a full-width matrix row now, so its sentence fits.
///
/// On the 300-pixel rail this replaced, the band read "Engine-owned · exempt
/// from the source-owned re…" — the clause that says what the exemption *is*
/// was the half that got cut.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_engine_owned_band_states_its_whole_exemption() {
    fn painted(shape: &egui::epaint::Shape, wanted: &str) -> bool {
        match shape {
            egui::epaint::Shape::Text(text) => text.galley.text() == wanted,
            egui::epaint::Shape::Vec(shapes) => shapes.iter().any(|shape| painted(shape, wanted)),
            _ => false,
        }
    }

    let ctx = themed_context();
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.models_page = ModelsPage::Qualification;
    let size = egui::vec2(PAGE_SIZE[0], PAGE_SIZE[1]);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| super::super::show(ui, &mut app));
        },
    );
    assert!(
        output
            .shapes
            .iter()
            .any(|clipped| painted(&clipped.shape, ENGINE_OWNED_BAND)),
        "the engine-owned band was painted truncated or not at all"
    );
}

/// The body is two tracks that reach the panel's bottom edge.
///
/// The page used to stop about five hundred pixels short of it, leaving a band
/// of bare panel under a matrix that had simply run out of rows — which reads
/// as a page that failed to render rather than as room.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_qualification_body_fills_the_document_in_two_tracks() {
    fn walk(shape: &egui::epaint::Shape, fill: Color32, out: &mut Vec<egui::Rect>) {
        match shape {
            egui::epaint::Shape::Rect(rect) if rect.fill == fill => out.push(rect.rect),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, fill, out);
                }
            }
            _ => {}
        }
    }

    let ctx = themed_context();
    let mut app = source_owned_app();
    let size = egui::vec2(PAGE_SIZE[0], PAGE_SIZE[1]);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| super::super::show(ui, &mut app));
        },
    );

    let panel = Tokens::get(&ctx).color.bg_panel;
    let mut rects = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, panel, &mut rects);
    }
    // The panes are the panel-filled boxes that end at the bottom of the
    // document without spanning its whole width — the one that does is the
    // document surface itself, laid down before any page painted on it.
    let mut panes = rects
        .iter()
        .filter(|rect| (rect.bottom() - size.y).abs() <= 1.5 && rect.width() < size.x - 1.0)
        .copied()
        .collect::<Vec<_>>();
    panes.sort_by(|left, right| left.left().total_cmp(&right.left()));
    panes.dedup_by(|left, right| (left.left() - right.left()).abs() <= 0.5);
    assert_eq!(
        panes.len(),
        2,
        "the body is the matrix and the contract rail, both reaching the bottom: {panes:?}"
    );
    assert!(
        panes[0].left() <= 1.0 && (panes[1].right() - size.x).abs() <= 1.0,
        "the two tracks do not span the document column: {panes:?}"
    );
    assert!(
        panes[0].width() > panes[1].width(),
        "the matrix is the wider track: {panes:?}"
    );
    assert!(
        panes[0].height() >= DETAIL_PANE_MIN_H,
        "the matrix is drawn shorter than the mockup's 170 px minimum: {:?}",
        panes[0]
    );
}

/// Both per-family workflows stay reachable from the matrix's selection strip.
///
/// Not a wiring test — [`qualification_action_block_reason`] has those — but a
/// guard that the composition change did not strand a route: the strip is the
/// only place either action is authored now.
#[test]
fn the_selection_strip_keeps_both_family_workflows_reachable() {
    let app = source_owned_app();
    let summary = owned_summary(&app);
    assert_eq!(
        qualification_action_block_reason(
            &app,
            Some(&summary),
            QualificationPageAction::ReviewVectors
        ),
        None,
        "a family with retained vectors can be reviewed"
    );
    let engine_owned = qualification_summaries(&app)
        .into_iter()
        .find(|summary| !summary.gate.is_gate_subject());
    if let Some(engine_owned) = engine_owned {
        assert!(
            qualification_action_block_reason(
                &app,
                Some(&engine_owned),
                QualificationPageAction::ReviewVectors
            )
            .is_some_and(|reason| reason.contains("exempt")),
            "an exempt family's blocked action must say why rather than vanish"
        );
    }
}

/// Renders of the page for a human to look at, in both states it has: a fresh
/// project holding only engine-owned families, and a project that owns a
/// model with retained suites.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_qualification_page() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    // One application at a time, and each one dropped before the next is
    // built: an `RSpiceApp` is a whole session by value, and two of them held
    // in a loop's array overflowed the test thread's stack before the first
    // render started.
    {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.models_page = ModelsPage::Qualification;
        let path = write_render(&directory, "engine-owned", &mut app);
        writeln!(report, "wrote {}", path.display()).ok();
    }
    {
        let mut app = source_owned_app();
        let path = write_render(&directory, "source-owned", &mut app);
        writeln!(report, "wrote {}", path.display()).ok();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn write_render(
    directory: &std::path::Path,
    slug: &str,
    app: &mut RSpiceApp,
) -> std::path::PathBuf {
    let canvas =
        crate::ui::raster::render(egui::vec2(PAGE_SIZE[0], PAGE_SIZE[1]), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| super::super::show(ui, app));
        });
    let path = directory.join(format!("egui-05-qualification-{slug}.png"));
    let height = canvas.content_height().max(200);
    std::fs::write(&path, canvas.png(height)).expect("write png");
    path
}
