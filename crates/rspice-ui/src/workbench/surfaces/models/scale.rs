//! What this workspace costs on a corpus the size of a real PDK.
//!
//! Every page here derives what it shows from the whole model corpus on every
//! frame. That is correct — nothing is cached behind a stale key — but it means
//! the cost of a frame is set by the corpus, not by what fits on screen. This
//! module builds a corpus at production scale and pins the per-frame work to
//! the visible rows rather than the corpus size.
//!
//! The assertions are counters and widget counts, not wall-clock. A timing
//! assertion in CI is a coin flip on a loaded machine, and the thing that
//! actually goes wrong here is a quantity — a re-derivation per model, a row
//! painted per model — which a counter states exactly.

use std::path::PathBuf;

use crate::product::{ContentDigest, ModelSourceId};
use crate::state::model_library::{
    DeviceModel, ModelDefinitionMetadata, ModelLibrary, ModelSourceAuthority, ModelSourceContent,
    ModelSourcePin, ModelType, ProjectModelDefinition, ProjectModelRevisionDefinition,
};
use crate::state::{Component, ComponentType, Point};
use crate::workbench::RSpiceApp;
use crate::workbench::state::ModelsPage;

/// Libraries a project at the top of its range binds at once.
pub(super) const LIBRARIES: usize = 40;
/// Models in each of them.
pub(super) const MODELS_PER_LIBRARY: usize = 125;
/// Libraries the project itself owns — the ones every page must authenticate.
///
/// A project owning four whole model libraries is already past what a real one
/// does; the remaining thirty-six are bound PDK sources, which is the shape a
/// production project actually has.
pub(super) const PROJECT_LIBRARIES: usize = 4;
/// Instances on the active schematic, which every "used by" column consults.
pub(super) const COMPONENTS: usize = 400;
/// Symbol cellviews in the design library, which the Symbols page registers.
pub(super) const SYMBOLS: usize = 600;

/// Total models the fixture holds.
pub(super) const MODELS: usize = LIBRARIES * MODELS_PER_LIBRARY;

/// An application holding a model corpus at production scale.
pub(super) fn large_corpus_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.library_manager.clear();
    app.state.model_library_manager.clear();

    for index in 0..PROJECT_LIBRARIES {
        app.state
            .model_library_manager
            .add_library(project_owned_library(index));
    }
    for index in PROJECT_LIBRARIES..LIBRARIES {
        app.state
            .model_library_manager
            .add_library(bound_pdk_library(index));
    }

    // The Symbols page registers one row per symbol cellview, which is its own
    // corpus: a technology library ships a symbol for every device it offers.
    app.state
        .library_manager
        .add_library(crate::state::Library::new("tech_symbols"));
    let symbol_library = app
        .state
        .library_manager
        .get_library_mut("tech_symbols")
        .expect("symbol library was just added");
    for index in 0..SYMBOLS {
        let mut cell = crate::state::Cell::new(format!("sym{index}"));
        cell.add_view(device_symbol_view());
        symbol_library.add_cell(cell);
    }

    app.state.workspace.ensure_active_buffer();
    let mut schematic = app
        .state
        .workspace
        .active_schematic()
        .cloned()
        .unwrap_or_default();
    for index in 0..COMPONENTS {
        let mut component = Component::new(
            index as u64 + 1,
            ComponentType::Nmos,
            Point::new(index as i32 % 40, index as i32 / 40),
        );
        component.name = format!("M{index}");
        // Instances spread across the corpus rather than all naming one model,
        // so a per-model scan of the schematic cannot be shortened by an early
        // match on the first row.
        let library = index % LIBRARIES;
        component.value = if library < PROJECT_LIBRARIES {
            owned_model_name(library, index % MODELS_PER_LIBRARY)
        } else {
            pdk_model_name(library, index % MODELS_PER_LIBRARY)
        };
        schematic.components.push(component);
    }
    app.state.workspace.save_active_schematic(&schematic);
    app
}

fn owned_model_name(library: usize, model: usize) -> String {
    format!("owned{library}_m{model}")
}

/// A symbol cellview carrying real drawn artwork.
///
/// An empty view is not the shape that costs: a cellview with no stored
/// document deserializes nothing, and the page's cost is the JSON in the
/// metadata of a symbol somebody actually drew. Four terminals with a body is
/// what a device symbol in a technology library is.
fn device_symbol_view() -> crate::state::View {
    use crate::state::{PortDirection, PortSpec, SymbolDocument};

    let ports = ["D", "G", "S", "B"]
        .into_iter()
        .map(|name| PortSpec {
            name: name.to_owned(),
            direction: PortDirection::InOut,
        })
        .collect::<Vec<_>>();
    let mut view = crate::state::View::new("symbol", crate::state::ViewType::Symbol);
    SymbolDocument::generated_from_ports(&ports)
        .store_in_view(&mut view)
        .expect("a generated device symbol is storable");
    view
}

/// Device families a binned PDK library declares.
pub(super) const FAMILIES: [&str; 5] = ["nch", "pch", "nch_hvt", "pch_hvt", "rppolywo"];

/// A binned card's name, in the `family.bin` form the engine resolves by.
fn pdk_model_name(library: usize, model: usize) -> String {
    format!(
        "{}{library}.{}",
        FAMILIES[model % FAMILIES.len()],
        model / FAMILIES.len() + 1
    )
}

/// A project-owned library whose every model resolves through the retained
/// closure — the path the qualification page and the model editor take.
fn project_owned_library(index: usize) -> ModelLibrary {
    let name = format!("owned{index}");
    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    let seed = ProjectModelDefinition {
        name: owned_model_name(index, 0),
        spice_type: "NMOS".to_owned(),
        description: "Project-owned card".to_owned(),
        numeric_parameters: [
            ("level".to_owned(), 1.0),
            ("vth0".to_owned(), 0.42),
            ("kp".to_owned(), 0.000_11),
        ]
        .into_iter()
        .collect(),
        string_parameters: std::collections::BTreeMap::new(),
    };
    manager
        .create_project_model(&name, &seed)
        .expect("seed the project-owned library");
    let mut library = manager.get_library(&name).expect("seeded library").clone();
    let root = library.root_path.clone().expect("project source root");
    let seed_model = library.models.values().next().expect("seed model").clone();
    let seed_metadata = library
        .model_definition_metadata
        .values()
        .next()
        .expect("seed metadata")
        .clone();

    let mut source = library.source_contents[0].bytes.clone();
    for model_index in 1..MODELS_PER_LIBRARY {
        let mut model = seed_model.clone();
        model.name = owned_model_name(index, model_index);
        model.file_path = Some(root.clone());
        let (definition, metadata) = revision(&model, &seed_metadata);
        source.extend_from_slice(
            definition
                .canonical_source()
                .expect("clone renders canonical source")
                .as_bytes(),
        );
        library
            .model_definition_metadata
            .insert(model.name.clone(), metadata);
        library.add_model(model);
    }

    rebind_source(&mut library, root, source);
    library
}

/// A model's revision definition with the manager-owned identity cleared.
///
/// The seed's identity names the seed's own digest; a clone that kept it would
/// be rejected as bound to a different revision, which is exactly what should
/// happen to a real clone. Clearing it is how a model that has never been
/// separately published presents itself.
fn revision(
    model: &DeviceModel,
    seed_metadata: &ModelDefinitionMetadata,
) -> (ProjectModelRevisionDefinition, ModelDefinitionMetadata) {
    let mut metadata = seed_metadata.clone();
    metadata.source_identity = None;
    let definition = ProjectModelRevisionDefinition::new(
        ProjectModelDefinition::from_device_model(model),
        metadata.clone(),
    );
    (definition, metadata)
}

/// Repin a library to source bytes assembled here rather than by the manager.
fn rebind_source(library: &mut ModelLibrary, root: PathBuf, source: Vec<u8>) {
    use sha2::{Digest as _, Sha256};

    let digest = ContentDigest::from_bytes(Sha256::digest(source.as_slice()).into());
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision,
        ..
    } = library.source_authority
    else {
        panic!("the seeded library is project-owned")
    };
    library.source_authority = ModelSourceAuthority::ProjectOwned {
        source_id,
        revision,
        digest,
    };
    library.source_closure = vec![ModelSourcePin {
        path: root.clone(),
        digest,
    }];
    library.source_contents = vec![ModelSourceContent {
        path: root,
        bytes: source,
    }];
}

/// A bound PDK library: pinned bytes the project retained, not project-owned.
///
/// Its cards are binned the way a foundry library really is — `nch7.1`,
/// `nch7.2`, … tiling the L axis — because that is what decides which cards the
/// Bins page compares against each other. Adjacent bins share a boundary, so a
/// correct audit finds nothing here.
fn bound_pdk_library(index: usize) -> ModelLibrary {
    let mut library = ModelLibrary::new(format!("pdk{index}"));
    library.pdk_name = format!("foundry{index}");
    library.technology_node = "28nm".to_owned();
    let root = PathBuf::from(format!("/pdk/{index}/models.lib"));
    library.root_path = Some(root.clone());
    let mut source = String::new();
    for model_index in 0..MODELS_PER_LIBRARY {
        let name = pdk_model_name(index, model_index);
        let bin = model_index / FAMILIES.len();
        let mut model = DeviceModel::new(&name, ModelType::Nmos);
        model.file_path = Some(root.clone());
        model.description = format!("{name} corner card");
        model.spice_level = Some(54);
        model.vth0 = Some(0.4);
        model.l_min = Some(2.8e-8 * (bin + 1) as f64);
        model.l_max = Some(2.8e-8 * (bin + 2) as f64);
        model.w_min = Some(1.0e-7);
        model.w_max = Some(1.0e-4);
        model.parameters = [
            ("level".to_owned(), 54.0),
            ("vth0".to_owned(), 0.4),
            ("u0".to_owned(), 0.02),
            ("tox".to_owned(), 1.8e-9),
        ]
        .into_iter()
        .collect();
        source.push_str(&format!(".model {name} NMOS ( LEVEL=54 VTH0=0.4 )\n"));
        library.add_model(model);
    }
    let bytes = source.into_bytes();
    let digest = {
        use sha2::{Digest as _, Sha256};
        ContentDigest::from_bytes(Sha256::digest(bytes.as_slice()).into())
    };
    library.source_authority = ModelSourceAuthority::RetainedImport {
        source_id: ModelSourceId::new(),
        digest,
    };
    library.source_closure = vec![ModelSourcePin {
        path: root.clone(),
        digest,
    }];
    library.source_contents = vec![ModelSourceContent { path: root, bytes }];
    library
}

/// Render one page of the workspace and report what the frame produced.
pub(super) fn render_page(app: &mut RSpiceApp, page: ModelsPage, size: egui::Vec2) -> FrameReport {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    app.state.workbench.models_page = page;
    let input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
        ..Default::default()
    };
    // Fonts build on the first pass and sizing settles on the second; the
    // third is the steady-state frame this measures.
    let mut output = None;
    let mut authentications = 0;
    let mut symbol_parses = 0;
    let mut consumer_index_builds = 0;
    let mut library_serializations = 0;
    let mut drift_hashes = 0;
    for _ in 0..3 {
        crate::workbench::documents::model_editor::CLOSURE_AUTHENTICATIONS.with(|count| {
            count.set(0);
        });
        crate::state::SYMBOL_VIEW_PARSES.with(|count| count.set(0));
        super::manager::CONSUMER_INDEX_BUILDS.with(|count| count.set(0));
        crate::state::CATALOG_LIBRARY_SERIALIZATIONS.with(|count| count.set(0));
        super::manager::SOURCE_DRIFT_HASHES.with(|count| count.set(0));
        output = Some(ctx.run_ui(input(), |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| super::show(ui, app));
        }));
        authentications =
            crate::workbench::documents::model_editor::CLOSURE_AUTHENTICATIONS.with(|c| c.get());
        symbol_parses = crate::state::SYMBOL_VIEW_PARSES.with(std::cell::Cell::get);
        consumer_index_builds = super::manager::CONSUMER_INDEX_BUILDS.with(std::cell::Cell::get);
        library_serializations =
            crate::state::CATALOG_LIBRARY_SERIALIZATIONS.with(std::cell::Cell::get);
        drift_hashes = super::manager::SOURCE_DRIFT_HASHES.with(std::cell::Cell::get);
    }
    let output = output.expect("three passes");
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("models accessibility tree")
        .nodes;
    FrameReport {
        widgets: nodes.len(),
        authentications,
        symbol_parses,
        consumer_index_builds,
        library_serializations,
        drift_hashes,
    }
}

pub(super) struct FrameReport {
    /// Widgets the frame built. A virtualized table builds the rows on screen;
    /// one that is not builds a widget per row in the corpus.
    pub(super) widgets: usize,
    /// Retained closures authenticated during the frame.
    pub(super) authentications: usize,
    /// Symbol cellviews deserialized during the frame.
    pub(super) symbol_parses: usize,
    /// Consumer indexes built during the frame.
    pub(super) consumer_index_builds: usize,
    /// Model libraries serialized whole during the frame.
    pub(super) library_serializations: usize,
    /// Source blobs hashed for drift during the frame.
    pub(super) drift_hashes: usize,
}

/// Widgets a page may build at this viewport, whatever the corpus holds.
///
/// Generous — this is not a layout assertion, it is the difference between "a
/// screenful" and "the corpus". The fixture holds 5,000 models, so a page that
/// builds a widget per model lands two orders of magnitude above this.
const WIDGET_CEILING: usize = 500;

#[test]
fn every_page_builds_a_screenful_not_a_corpus() {
    let mut app = large_corpus_app();
    assert_eq!(app.state.model_library_manager.total_model_count(), MODELS);
    for page in ModelsPage::ALL {
        let report = render_page(&mut app, page, egui::vec2(1600.0, 1000.0));
        assert!(
            report.widgets <= WIDGET_CEILING,
            "{} built {} widgets for a {MODELS}-model corpus; its tables are \
             enumerating the corpus rather than the rows on screen",
            page.label(),
            report.widgets
        );
    }
}

#[test]
fn a_frame_authenticates_each_retained_closure_at_most_once() {
    // Authenticating digests every retained byte. Resolving models one at a
    // time re-authenticates per model, which made the qualification page cost
    // one hash of the library per model in it.
    let mut app = large_corpus_app();
    for page in ModelsPage::ALL {
        let report = render_page(&mut app, page, egui::vec2(1600.0, 1000.0));
        assert!(
            report.authentications <= PROJECT_LIBRARIES,
            "{} authenticated {} closures in one frame; there are only \
             {PROJECT_LIBRARIES} project-owned libraries to authenticate",
            page.label(),
            report.authentications
        );
    }
}

#[test]
fn a_repainted_page_deserializes_no_symbol_it_already_read() {
    // Deriving a symbol registry row deserializes the cellview's typed contract
    // and its artwork, and the Symbols page derives every row in the corpus
    // before the table narrows to the rows on screen. Nothing on screen changes
    // between two frames over an untouched corpus, so the second frame must not
    // repeat the work — and the fixture ships a symbol per device the way a
    // technology library does, which is where the cost is.
    let mut app = large_corpus_app();
    for page in ModelsPage::ALL {
        // The first frame on a page is the one allowed to read the corpus; the
        // report describes the third.
        let report = render_page(&mut app, page, egui::vec2(1600.0, 1000.0));
        assert_eq!(
            report.symbol_parses,
            0,
            "{} deserialized {} symbol cellviews on a frame that changed nothing; \
             the registry is being rebuilt from the corpus rather than read back",
            page.label(),
            report.symbol_parses
        );
    }
}

#[test]
fn a_repainted_page_serializes_no_library_to_decide_a_cache_is_still_good() {
    // The Bins & geometry page presents an engine model-bin inspection of the
    // exact prepared deck, which is far too expensive to redo per frame and is
    // therefore cached behind an input digest. That digest used to include the
    // execution catalogue digest, which serializes every library in the corpus
    // through `serde_json::Value` — one node per model, per parameter, and per
    // byte of every retained source file. Computing the key cost more than the
    // work it was guarding, so the frame paid the corpus anyway and the cache
    // bought nothing. A cache key has to be cheaper than its miss.
    let mut app = large_corpus_app();
    for page in ModelsPage::ALL {
        let report = render_page(&mut app, page, egui::vec2(1600.0, 1000.0));
        assert_eq!(
            report.library_serializations,
            0,
            "{} serialized {} model libraries whole on a frame that changed nothing; \
             deciding whether a cached answer is still good is walking the corpus",
            page.label(),
            report.library_serializations
        );
    }
}

#[test]
fn a_frame_builds_at_most_one_consumer_index() {
    // The index is one pass over every placed instance in the design. Which
    // model an instance names does not depend on who is asking, so a frame that
    // builds it twice has walked the schematic twice for one answer.
    let mut app = large_corpus_app();
    for page in ModelsPage::ALL {
        let report = render_page(&mut app, page, egui::vec2(1600.0, 1000.0));
        assert!(
            report.consumer_index_builds <= 1,
            "{} built {} consumer indexes in one frame; every build walks all \
             {COMPONENTS} placed instances",
            page.label(),
            report.consumer_index_builds
        );
    }
}

#[test]
fn a_painted_frame_hashes_no_source_bytes_at_all() {
    // Source drift is decided by rehashing every retained byte, which for this
    // fixture is the whole corpus and for a real foundry import is megabytes.
    // The result is identical whether it is decided once per import or sixty
    // times a second, so nothing about the painted frame can reveal the
    // difference — the counter is the only witness. The first frame on a page
    // is allowed to arm the scan; the report describes the third.
    let mut app = large_corpus_app();
    for page in ModelsPage::ALL {
        let report = render_page(&mut app, page, egui::vec2(1600.0, 1000.0));
        assert_eq!(
            report.drift_hashes,
            0,
            "{} hashed {} source blobs on a frame that changed nothing; drift is \
             decided on events — a workspace opening, an import completing, a \
             reader asking — and never in the paint path",
            page.label(),
            report.drift_hashes
        );
    }
}

#[test]
#[ignore = "measures wall-clock; run by hand with --ignored --nocapture"]
fn report_page_cost_at_production_scale() {
    use std::io::Write as _;

    let stderr = std::io::stderr();
    let mut report_output = stderr.lock();
    let build = std::time::Instant::now();
    let mut app = large_corpus_app();
    writeln!(
        report_output,
        "fixture: {MODELS} models in {LIBRARIES} libraries, built in {:?}",
        build.elapsed()
    )
    .expect("write scale qualification report");
    // Reported separately because it is the one derivation a page cannot
    // narrow to what is on screen: the metric strip totals every model.
    let summaries = std::time::Instant::now();
    let count = super::qualification_summaries(&app).len();
    writeln!(
        report_output,
        "qualification_summaries: {count} summaries in {:?}",
        summaries.elapsed()
    )
    .expect("write scale qualification report");
    for width in [2560.0, 1600.0, 1000.0] {
        writeln!(report_output, "--- {width}px ---").expect("write scale qualification report");
        for page in ModelsPage::ALL {
            let start = std::time::Instant::now();
            let report = render_page(&mut app, page, egui::vec2(width, width * 0.625));
            writeln!(
                report_output,
                "{:<18} {:>8.1} ms/frame  {:>6} widgets  {:>3} authentications  \
                 {:>5} symbol parses  {:>2} consumer indexes  {:>3} library serializations",
                page.label(),
                start.elapsed().as_secs_f64() * 1000.0 / 3.0,
                report.widgets,
                report.authentications,
                report.symbol_parses,
                report.consumer_index_builds,
                report.library_serializations,
            )
            .expect("write scale qualification report");
        }
    }
}
