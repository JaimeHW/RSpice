//! Tests for the models surface's evidence rules.
//!
//! The central assertion is that evidence is never invented: a non-project
//! model receives no synthetic qualification evidence, and a configured
//! correlation requires current approved evidence before it counts.

use egui::Rect;

use super::*;
use crate::workbench::state::ModelsPage;

#[test]
fn model_tabs_match_the_mockup_taxonomy() {
    assert_eq!(
        ModelsPage::ALL.map(ModelsPage::label),
        [
            "Models",
            "Symbols & CDF",
            "Corners & sections",
            "Bins & geometry",
            "Include graph",
            "Qualification",
        ]
    );
}

/// Every page's actions stay inside its own title band.
///
/// The band is one row now rather than two — the mockup's `.model-section-bar`
/// carries the title, the live meta and the action cluster on one line — so its
/// bounds are stated as the composition's own constants rather than as the two
/// magic numbers that described the stacked pair this replaces. The claim is
/// unchanged: the action is inside the page bar and clear of the tab strip
/// above it, at every width down to 561 px, where the actions squeeze the
/// title rather than wrapping out of the band.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn complete_models_surface_keeps_action_pages_inside_the_title_band() {
    let band_top = f64::from(manager::PAGE_TABS_H);
    let band_bottom = band_top + f64::from(manager::SECTION_BAR_H);
    for (page, label) in [
        (ModelsPage::Symbols, "Create symbol"),
        (ModelsPage::Include, "Export manifest"),
    ] {
        for width in [1_431.0, 820.0, 720.0, 561.0] {
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            ctx.enable_accesskit();
            let mut app = RSpiceApp::test_instance();
            app.state.workbench.models_page = page;
            app.state.library_manager.clear();
            app.state.model_library_manager.clear();

            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width, 560.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE)
                        .show(ctx, |ui| show(ui, &mut app));
                },
            );

            let nodes = output
                .platform_output
                .accesskit_update
                .expect("models accessibility tree")
                .nodes;
            let bounds = nodes
                .iter()
                .find(|(_, node)| {
                    node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
                })
                .and_then(|(_, node)| node.bounds())
                .unwrap_or_else(|| panic!("missing {label} action"));
            assert!(
                bounds.y1 <= band_bottom,
                "{label} escaped the models title band (ends at {band_bottom}) on {page:?} at {width}: {bounds:?}"
            );
            assert!(
                bounds.y0 >= band_top,
                "{label} overlapped the page tabs (they end at {band_top}) on {page:?} at {width}: {bounds:?}"
            );
            assert!(
                !output.shapes.is_empty(),
                "{page:?} produced no visible body composition at {width}"
            );
        }
    }
}

#[test]
fn qualification_tab_uses_the_mockup_contract_label() {
    assert_eq!(ModelsPage::Qualification.label(), "Qualification");
    assert_eq!(
        Command::ModelsPage(ModelsPage::Qualification).stable_id(),
        "model-qualification"
    );
}

#[test]
fn qualification_evidence_set_digest_is_order_independent_and_suite_qualified() {
    let first = crate::product::ContentDigest::from_bytes([0x11; 32]);
    let second = crate::product::ContentDigest::from_bytes([0x22; 32]);
    let mut one = vec![("dc".to_owned(), 3, first)];
    let one_label = qualification_evidence_contract_digest(&mut one).expect("single digest label");
    assert!(one_label.starts_with("dc@3 · "));

    let mut forward = vec![
        ("transient".to_owned(), 4, second),
        ("dc".to_owned(), 3, first),
    ];
    let mut reverse = forward.iter().cloned().rev().collect::<Vec<_>>();
    let forward_label =
        qualification_evidence_contract_digest(&mut forward).expect("aggregate digest");
    let reverse_label =
        qualification_evidence_contract_digest(&mut reverse).expect("aggregate digest");
    assert_eq!(forward_label, reverse_label);
    assert!(forward_label.starts_with("2 suites · "));
}

#[test]
fn qualification_domain_projection_never_invents_oracle_provenance() {
    let mut quantities = BTreeSet::new();
    quantities.insert("v(out)".to_owned());
    let domains = qualification_domain_summaries(BTreeMap::from([(
        QualificationDomain::Ac,
        QualificationDomainAccumulator {
            vectors: 2,
            references: 2,
            quantities,
            tolerance_contracts: BTreeMap::from([(
                qualification_tolerance_key(1.0e-6, 0.005),
                qualification_tolerance_label(1.0e-6, 0.005),
            )]),
            evidenced_vectors: 1,
            passing_vectors: 1,
            open_dispositions: 0,
        },
    )]));

    assert_eq!(domains.len(), 1);
    assert_eq!(domains[0].domain, QualificationDomain::Ac);
    assert_eq!(domains[0].reference_coverage, "2 refs · 1 quantity");
    assert_eq!(domains[0].disposition, "1 without evidence");
    assert!(
        !domains[0]
            .reference_coverage
            .to_ascii_lowercase()
            .contains("vendor")
    );
    assert!(
        !domains[0]
            .reference_coverage
            .to_ascii_lowercase()
            .contains("oracle")
    );
}

#[test]
fn qualification_domain_projection_preserves_distinct_tolerance_contracts() {
    let domains = qualification_domain_summaries(BTreeMap::from([(
        QualificationDomain::Dc,
        QualificationDomainAccumulator {
            vectors: 2,
            references: 2,
            quantities: BTreeSet::from(["v(out)".to_owned()]),
            tolerance_contracts: BTreeMap::from([
                (
                    qualification_tolerance_key(1.0001e-6, 0.0),
                    qualification_tolerance_label(1.0001e-6, 0.0),
                ),
                (
                    qualification_tolerance_key(1.0002e-6, 0.0),
                    qualification_tolerance_label(1.0002e-6, 0.0),
                ),
            ]),
            evidenced_vectors: 0,
            passing_vectors: 0,
            open_dispositions: 0,
        },
    )]));

    assert_eq!(
        qualification_tolerance_label(1.0001e-6, 0.0),
        qualification_tolerance_label(1.0002e-6, 0.0)
    );
    assert_eq!(domains[0].tolerance, "2 declared contracts · varies");
}

#[test]
fn project_model_without_suites_is_truthfully_unqualified() {
    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    let definition = crate::state::model_library::ProjectModelDefinition {
        name: "nch_owned".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Project-owned qualification fixture".to_owned(),
        numeric_parameters: std::collections::BTreeMap::from([
            ("level".to_owned(), 1.0),
            ("vth0".to_owned(), 0.48),
        ]),
        string_parameters: std::collections::BTreeMap::new(),
    };
    app.state
        .model_library_manager
        .create_project_model("owned-models", &definition)
        .expect("create project model");

    let summary = qualification_summaries(&app)
        .into_iter()
        .find(|summary| summary.model == "nch_owned")
        .expect("qualification summary");

    assert!(summary.source_error.is_none());
    assert_eq!(summary.gate, QualificationGate::Unqualified);
    assert_eq!(summary.suites, 0);
    assert_eq!(summary.vectors, 0);
    assert_eq!(summary.passing_vectors, 0);
    assert!(summary.evidence_digest.is_none());

    app.state
        .model_library_manager
        .select_library(&summary.library)
        .expect("the fixture library is loaded");
    app.state.workbench.selected_model = Some(summary.model.clone());
    assert_eq!(
        qualification_action_block_reason(&app, Some(&summary), QualificationPageAction::RunSuite)
            .as_deref(),
        Some("Author at least one executable qualification suite first")
    );
    assert_eq!(
        qualification_action_block_reason(
            &app,
            Some(&summary),
            QualificationPageAction::ReviewVectors
        ),
        None
    );
    execute_qualification_action(&mut app, QualificationPageAction::ReviewVectors);
    assert_eq!(
        app.state.workbench.model_editor.active_section,
        ModelEditorSection::Tests
    );
    assert!(app.state.workbench.model_editor.qualification_plan_open);
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        SurfaceId::ModelEditor
    );

    let editor = &mut app.state.workbench.model_editor;
    editor.begin_qualification_suite();
    let authoring = &mut editor.qualification_authoring;
    authoring.suite_id = "dc-op".to_owned();
    authoring.suite_name = "DC operating point".to_owned();
    authoring.vector_id = "nominal".to_owned();
    authoring.vector_name = "Nominal bias".to_owned();
    authoring.executable_input =
        "V1 out 0 1\nR1 out 0 1k\nMbind 0 0 0 0 nch_owned\n.op\n.end\n".to_owned();
    authoring.quantity = "v(out)".to_owned();
    authoring.probe_target = "out".to_owned();
    authoring.expected = "1".to_owned();
    authoring.absolute_tolerance = "1e-9".to_owned();
    authoring.relative_tolerance = "1e-9".to_owned();
    assert!(
        editor.commit_qualification_suite(),
        "{:?}",
        editor.qualification_authoring.error
    );

    let working_summary = qualification_summaries(&app)
        .into_iter()
        .find(|summary| summary.model == "nch_owned")
        .expect("working qualification summary");
    assert_eq!(working_summary.suites, 1);
    assert_eq!(working_summary.vectors, 1);
    assert!(
        working_summary
            .source_revision
            .ends_with("· working qualification")
    );
    assert_eq!(
        qualification_action_block_reason(
            &app,
            Some(&working_summary),
            QualificationPageAction::RunSuite
        ),
        None
    );
}

#[test]
fn engine_owned_model_is_exempt_and_never_receives_synthetic_qualification_evidence() {
    let app = RSpiceApp::test_instance();
    let library = ModelLibrary::new("built-in");
    let model = DeviceModel::new(
        "builtin_resistor",
        crate::state::model_library::ModelType::Resistor,
    );

    let closure = model_editor::verify_project_library_closure(&library, &library.name);
    let summary = qualification_model_summary(&app, &library, &model, closure.as_ref());

    // No evidence is invented for it — and no failure is either: the source
    // resolution that cannot succeed for a compiled-in card is never run, so
    // its error never becomes this model's verdict.
    assert!(summary.source_error.is_none());
    assert_eq!(summary.gate, QualificationGate::EngineOwned);
    assert_eq!(summary.source_revision, "engine-owned");
    assert_eq!(summary.vectors, 0);
    assert_eq!(summary.passing_vectors, 0);
    assert!(summary.evidence_digest.is_none());
}

/// The exemption is for engine-owned cards alone. A library that claims a
/// source and cannot produce one is still a blocked gate: that is a project
/// defect, and softening it would be the release gate failing open.
#[test]
fn a_library_that_should_be_source_owned_and_is_not_stays_blocked() {
    let app = RSpiceApp::test_instance();
    let mut library = ModelLibrary::new("vendor-pdk");
    library.source_authority = crate::state::model_library::ModelSourceAuthority::External;
    let model = DeviceModel::new("nch", crate::state::model_library::ModelType::Nmos);

    let closure = model_editor::verify_project_library_closure(&library, &library.name);
    let summary = qualification_model_summary(&app, &library, &model, closure.as_ref());

    assert!(summary.source_error.is_some());
    assert_eq!(summary.gate, QualificationGate::Blocked);
    assert_eq!(summary.source_revision, "not source-owned");
}

/// A project nobody has touched must not open on a red release gate.
///
/// Every family in the compiled-in foundation library is engine-owned, and the
/// page derived their verdict from a source resolution that cannot succeed for
/// a card with no source — so a fresh project reported all of them as blocked
/// release-gate failures, which is RSpice shipping red against its own library.
#[test]
fn a_fresh_project_reports_no_blocked_release_gates() {
    let app = RSpiceApp::test_instance();
    let summaries = qualification_summaries(&app);
    assert!(
        !summaries.is_empty(),
        "a fresh project loads the compiled-in foundation library"
    );

    let blocked = summaries
        .iter()
        .filter(|summary| summary.gate == QualificationGate::Blocked)
        .map(|summary| format!("{}/{}", summary.library, summary.model))
        .collect::<Vec<_>>();
    assert!(
        blocked.is_empty(),
        "a fresh project reported release-gate failures: {}",
        blocked.join(", ")
    );
    for summary in &summaries {
        assert_eq!(
            summary.gate,
            QualificationGate::EngineOwned,
            "{}/{} is compiled in and is not a gate subject",
            summary.library,
            summary.model
        );
        assert!(summary.source_error.is_none());
    }

    // A reading surface sees the same rung, and never "the gate could not be
    // read" — which is the fact that would make the Simulation Studio refuse a
    // release on evidence nobody can open.
    let facts = model_gate_facts(&app);
    assert_eq!(facts.len(), summaries.len());
    assert!(
        facts
            .iter()
            .all(|fact| !fact.unreadable && fact.gate == QualificationGate::EngineOwned)
    );

    // The rail groups them under their own band rather than mixing them into
    // the gate's own subjects.
    let rows = qualification_rail_rows(&summaries);
    assert_eq!(rows.len(), summaries.len() + 1);
    assert!(matches!(
        rows.first(),
        Some(QualificationRailRow::Band(label)) if *label == ENGINE_OWNED_BAND
    ));

    // And the workflows state the exemption instead of asking for a project
    // revision the user has no way to select.
    let selected = summaries.first().expect("a foundation family");
    for action in [
        QualificationPageAction::RunSuite,
        QualificationPageAction::ReviewVectors,
        QualificationPageAction::ReviewReleaseBinding,
        QualificationPageAction::CompareRelease,
    ] {
        let reason = qualification_action_block_reason(&app, Some(selected), action)
            .expect("an engine-owned selection blocks every qualification workflow");
        assert!(
            reason.contains("exempt from the source-owned release gate"),
            "{action:?} blocked for the wrong reason: {reason}"
        );
    }
}

#[test]
fn configured_correlation_requires_current_approved_evidence_for_qualification() {
    use crate::state::model_library::{
        CorrelationDatasetClass, CorrelationDatasetRevision, CorrelationSuite,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    let definition = crate::state::model_library::ProjectModelDefinition {
        name: "nch_correlated".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Correlation handoff fixture".to_owned(),
        numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0), ("vth0".to_owned(), 0.48)]),
        string_parameters: BTreeMap::new(),
    };
    app.state
        .model_library_manager
        .create_project_model("owned-models", &definition)
        .unwrap();
    let resolved = model_editor::resolve_project_model_for_editor(
        &app.state.model_library_manager,
        "owned-models",
        "nch_correlated",
    )
    .unwrap();
    let source = ModelSourceEvidenceBinding::try_new_project_bound(
        "nch_correlated",
        resolved.source_id,
        resolved.model_digest,
        resolved.model_revision,
    )
    .unwrap();
    let dataset = CorrelationDatasetRevision::try_from_csv(
        "bench",
        crate::product::ObjectRevision::INITIAL,
        "Bench",
        CorrelationDatasetClass::BenchMeasurement,
        "lab",
        "lot-1",
        "fixture-1",
        "calibration-1",
        "bench.csv",
        b"id,quantity,value,unit\np1,V(out),1,V\n".to_vec(),
        None,
    )
    .unwrap();
    let suite = CorrelationSuite::try_new(
        "bench-correlation",
        crate::product::ObjectRevision::INITIAL,
        "Bench correlation",
        "model-owner",
        source.clone(),
        vec![dataset],
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let correlation = ModelCorrelationState::try_new(vec![suite], Vec::new()).unwrap();
    let library = app
        .state
        .model_library_manager
        .get_library("owned-models")
        .unwrap();
    let model = library.models.get("nch_correlated").unwrap();
    let closure = model_editor::verify_project_library_closure(library, &library.name);
    let mut summary = qualification_model_summary(&app, library, model, closure.as_ref());
    summary.gate = QualificationGate::Qualified;

    apply_correlation_qualification_contract(&mut summary, Some(&correlation), Some(&source));

    assert_eq!(summary.gate, QualificationGate::Review);
    assert_eq!(
        summary.correlation_status,
        "0/1 current suite approvals retained"
    );
    assert!(summary.correlation_evidence_digest.is_none());
}

/// No page ends in the shell's own canvas.
///
/// This workspace is one continuous document surface: the mockup's
/// `.models-view` sits on the panel colour from the tab strip to the bottom
/// edge and separates its regions with one-pixel dividers. A page whose content
/// stopped short exposed the charcoal canvas underneath, which reads as a page
/// that failed to render rather than as a page with room left.
///
/// The probe is a point in the bottom band of the viewport: the panel had
/// better be painted there, on every page, at a document width where the
/// detail columns are at their widest.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn no_models_page_ends_in_unpainted_canvas() {
    fn covers(shape: &egui::epaint::Shape, probe: egui::Pos2, fill: egui::Color32) -> bool {
        match shape {
            egui::epaint::Shape::Rect(rect) => rect.fill == fill && rect.rect.contains(probe),
            egui::epaint::Shape::Vec(shapes) => {
                shapes.iter().any(|shape| covers(shape, probe, fill))
            }
            _ => false,
        }
    }

    // Every catalog scope, not only the default: the shelf shipped 250 px
    // short with this guard green because the probe only ever saw the
    // project scope's composition.
    use crate::workbench::state::ModelsCatalogScope;
    const SCOPES: [ModelsCatalogScope; 3] = [
        ModelsCatalogScope::Project,
        ModelsCatalogScope::InstalledPacks,
        ModelsCatalogScope::RSpiceLibrary,
    ];
    for page in ModelsPage::ALL {
        for scope in SCOPES {
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let mut app = RSpiceApp::test_instance();
            app.state.workbench.models_page = page;
            app.state.workbench.models_view.catalog_scope = scope;
            let size = egui::vec2(1_180.0, 900.0);
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE)
                        .show(ctx, |ui| show(ui, &mut app));
                },
            );
            let panel = crate::ui::tokens::Tokens::get(&ctx).color.bg_panel;
            let probe = egui::pos2(size.x * 0.5, size.y - 4.0);
            assert!(
                output
                    .shapes
                    .iter()
                    .any(|clipped| covers(&clipped.shape, probe, panel)),
                "{page:?} under {scope:?} left the document surface unpainted at {probe:?}"
            );
            if page != ModelsPage::Models {
                // Only the Models page reads the scope; one pass is evidence
                // enough for the rest.
                break;
            }
        }
    }
}

/// Write every page of the workspace to a PNG so its design can be reviewed.
///
/// The default state renders the compiled-in catalog; per-page populated and
/// exception states live beside the pages that own them (`corners::tests`,
/// `symbols::tests`, `bins::tests`, `qualification_page::tests`) and the hub's
/// trust rungs in `manager/raster.rs`. This test is the whole-workspace sweep
/// a reviewer starts from. Renders go to `RSPICE_RASTER_DIR` (default: the
/// system temp directory); read them for layout, not wording — the
/// rasterizer's own header says why.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_every_page_for_review() {
    use crate::workbench::state::ModelsCatalogScope;
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    let mut render = |slug: &str, page: ModelsPage, scope: ModelsCatalogScope| {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.models_page = page;
        app.state.workbench.models_view.catalog_scope = scope;
        let canvas = crate::ui::raster::render(egui::vec2(1_180.0, 900.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| show(ui, &mut app));
        });
        let path = directory.join(format!("models-{slug}.png"));
        let height = canvas.content_height().max(200);
        std::fs::write(&path, canvas.png(height)).expect("write png");
        writeln!(report, "wrote {}", path.display()).ok();
    };
    for (index, page) in ModelsPage::ALL.into_iter().enumerate() {
        let slug = page
            .label()
            .to_ascii_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect::<String>();
        render(
            &format!("{index:02}-{slug}"),
            page,
            ModelsCatalogScope::Project,
        );
    }
    render(
        "00b-rspice-library",
        ModelsPage::Models,
        ModelsCatalogScope::RSpiceLibrary,
    );
    render(
        "00b-installed-packs",
        ModelsPage::Models,
        ModelsCatalogScope::InstalledPacks,
    );
}

/// The Models page's detail panes are one row of equal columns that reaches the
/// panel's bottom edge.
///
/// The mockup's `.model-detail-body` is a single grid row of `minmax(0, 1fr)`
/// columns; the surface used to split the pane 36 / 64 and let the cards stop
/// at their own content, which left a band of the container's hairline colour
/// under the shortest of them and a strip of unpainted canvas under all four.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_model_detail_panes_are_one_row_of_equal_columns_filling_the_document() {
    fn walk(shape: &egui::epaint::Shape, fill: egui::Color32, out: &mut Vec<Rect>) {
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

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.models_page = ModelsPage::Models;
    let size = egui::vec2(1_180.0, 900.0);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| show(ui, &mut app));
        },
    );

    let panel = crate::ui::tokens::Tokens::get(&ctx).color.bg_panel;
    let mut rects = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, panel, &mut rects);
    }
    // The four detail panes are the panel-filled boxes that end at the bottom
    // of the document without spanning its whole width — the one that does is
    // the document surface itself, laid down before any page painted on it.
    let mut panes = rects
        .iter()
        .filter(|rect| (rect.bottom() - size.y).abs() <= 1.0 && rect.width() < size.x - 1.0)
        .copied()
        .collect::<Vec<_>>();
    panes.sort_by(|left, right| left.left().total_cmp(&right.left()));
    assert_eq!(
        panes.len(),
        4,
        "a document wider than 1100 px lays the four panes out in one row: {panes:?}"
    );
    let first = panes[0];
    for pane in &panes {
        assert!(
            (pane.top() - first.top()).abs() <= 1.0
                && (pane.height() - first.height()).abs() <= 1.0,
            "the detail panes are not one row of equal height: {panes:?}"
        );
    }
    assert!(
        (panes[0].left() - 0.0).abs() <= 1.0 && (panes[3].right() - size.x).abs() <= 1.0,
        "the row does not span the document column: {panes:?}"
    );
    assert!(
        first.height() >= 170.0,
        "a pane is drawn shorter than the mockup's 170 px minimum: {first:?}"
    );
}
