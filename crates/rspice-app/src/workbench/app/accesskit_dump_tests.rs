//! What a screen reader is told about the application, scene by scene,
//! written out so two builds can be compared line for line.
//!
//! Moving code between files and containers must not change what the reader
//! is shown or in what order. The AccessKit tree records both: every widget a
//! frame lays out, its name, its state and where it sits. Each scene here runs
//! whole application frames through `eframe::App::ui`, the frame eframe runs,
//! and writes the last frame's tree to its own text file, one node per line in
//! tree order. A build is unchanged when its files match the files of the
//! build before it. Bounds are rounded to half a point and the UUIDs a new
//! session mints are written as `<uuid>`, so two runs of one build match.
//!
//! The files go to `app-split/accesskit/` under the target directory, never
//! into the repository, and the test is ignored so it runs only when asked:
//!
//! ```text
//! cargo test -p rspice-app --lib -- --ignored accesskit_dump
//! ```

// Refactor scaffolding: deleted when the split completes.

use std::cell::RefCell;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use egui::accesskit::{Node, NodeId, TreeUpdate};
use egui::{Pos2, Rect, Vec2};

use super::RSpiceApp;
use crate::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle,
};
use crate::workbench::state::{Drawer, Workspace};
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

/// A desktop window, and a phone held upright.
const DESKTOP: Vec2 = Vec2::new(1440.0, 900.0);
const PHONE: Vec2 = Vec2::new(390.0, 844.0);

/// Frames run per scene. The tree is read from the last one, after layout has
/// settled and every panel animation has finished.
const FRAMES: u32 = 6;
const FRAME_SECONDS: f64 = 0.25;

/// The surfaces a route shows in place of the active workspace.
const ROUTE_SURFACES: [SurfaceId; 5] = [
    SurfaceId::ReportAuthoring,
    SurfaceId::ModelEditor,
    SurfaceId::ModelCorrelation,
    SurfaceId::PdkTechnologyAdmin,
    SurfaceId::LibraryCellviewManager,
];

type Setup = Box<dyn Fn(&mut RSpiceApp)>;
type Holds = Box<dyn Fn(&RSpiceApp) -> bool>;

/// One state of the application to dump.
struct Scene {
    name: String,
    size: Vec2,
    /// Puts a fresh test instance into the scene's state.
    setup: Setup,
    /// Whether the scene is still the state it is named for after its frames;
    /// a scene that quietly fell back to another would dump the wrong tree.
    holds: Holds,
    /// Draws secondary windows as native viewports rather than embedded in the
    /// main one.
    native_viewports: bool,
}

impl Scene {
    fn new(name: impl Into<String>, size: Vec2, setup: impl Fn(&mut RSpiceApp) + 'static) -> Self {
        Self {
            name: name.into(),
            size,
            setup: Box::new(setup),
            holds: Box::new(|_| true),
            native_viewports: false,
        }
    }

    fn holding(mut self, holds: impl Fn(&RSpiceApp) -> bool + 'static) -> Self {
        self.holds = Box::new(holds);
        self
    }
}

fn scenes() -> Vec<Scene> {
    let mut scenes = Vec::new();
    for workspace in Workspace::ALL {
        let name = format!("{workspace:?}").to_lowercase();
        scenes.push(
            Scene::new(
                format!("workspace-{name}-no-project"),
                DESKTOP,
                move |app| {
                    app.state.close_project_session();
                    app.state.workbench.activate(workspace);
                },
            )
            .holding(|app| !app.state.platform.project_lifecycle.is_open()),
        );
        scenes.push(
            Scene::new(
                format!("workspace-{name}-open-project"),
                DESKTOP,
                move |app| {
                    app.state.open_project_session();
                    app.state.workbench.activate(workspace);
                },
            )
            .holding(move |app| {
                app.state.platform.project_lifecycle.is_open()
                    && app.state.workbench.workspace == workspace
            }),
        );
    }
    scenes.push(
        Scene::new("split-results", DESKTOP, |app| {
            app.state.open_project_session();
            app.state.workbench.activate(Workspace::Design);
            app.state.workbench.split_with_results = true;
            retain_one_run(app);
        })
        .holding(|app| {
            app.state.workbench.results_split_visible(
                app.state.platform.project_lifecycle.is_open(),
                app.state
                    .model
                    .simulation
                    .retained
                    .has_retained_result_dataset(),
            )
        }),
    );
    for surface in ROUTE_SURFACES {
        let name = format!("{surface:?}").to_lowercase();
        scenes.push(
            Scene::new(format!("route-{name}"), DESKTOP, move |app| {
                app.state.open_project_session();
                app.state
                    .workbench
                    .navigate(SurfaceRoute::surface(surface), RouteTransitionSource::User)
                    .unwrap_or_else(|error| panic!("{surface:?} opens: {error}"));
            })
            .holding(move |app| app.state.workbench.current_route().surface_id() == surface),
        );
    }
    scenes.push(Scene::new("phone", PHONE, |app| on_a_phone(app, None)));
    for drawer in [Drawer::Navigator, Drawer::Inspector, Drawer::Workspaces] {
        let name = format!("{drawer:?}").to_lowercase();
        scenes.push(
            Scene::new(format!("phone-drawer-{name}"), PHONE, move |app| {
                on_a_phone(app, Some(drawer));
            })
            .holding(move |app| app.state.workbench.drawer == Some(drawer)),
        );
    }
    scenes.push(
        Scene::new("full-screen-presentation", DESKTOP, |app| {
            app.state.open_project_session();
            app.state.workbench.activate(Workspace::Design);
            app.state.enter_full_screen_presentation();
        })
        .holding(|app| app.state.workbench.full_screen_presentation),
    );
    scenes.push(
        Scene::new("secondary-window-embedded", DESKTOP, open_a_second_window)
            .holding(has_a_second_window),
    );
    let mut native =
        Scene::new("secondary-window", DESKTOP, open_a_second_window).holding(has_a_second_window);
    native.native_viewports = true;
    scenes.push(native);
    scenes
}

/// One completed run with one retained transient trace, stamped with a fixed
/// time so nothing it shows depends on when the dump ran.
fn retain_one_run(app: &mut RSpiceApp) {
    let mut run = SimulationRun::new(7);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.timestamp = 1_767_225_600.0;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "tran").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 2.0], "#ffbd2e"),
        ]),
    );
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
}

fn on_a_phone(app: &mut RSpiceApp, drawer: Option<Drawer>) {
    app.state.workbench.coarse_pointer = true;
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    app.state.workbench.drawer = drawer;
}

fn open_a_second_window(app: &mut RSpiceApp) {
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    let session = &mut app.state.workbench.window_session;
    let layout = session
        .state(session.primary())
        .map(|primary| primary.layout)
        .unwrap_or_default();
    session.create_window("Second window", Workspace::Results, layout, false);
}

fn has_a_second_window(app: &RSpiceApp) -> bool {
    !app.state
        .workbench
        .window_session
        .secondary_window_ids()
        .is_empty()
}

thread_local! {
    /// The trees the native viewports of the frame being run produced.
    static VIEWPORT_TREES: RefCell<Vec<TreeUpdate>> = const { RefCell::new(Vec::new()) };
}

/// Draw each native viewport the way a desktop backend does: its own pass
/// inside the main one, whose tree is kept for the dump.
fn render_native_viewports() {
    egui::Context::set_immediate_viewport_renderer(|ctx, mut viewport| {
        let size = viewport
            .builder
            .inner_size
            .unwrap_or(Vec2::new(1200.0, 800.0));
        let time = ctx.input(|input| input.time);
        let output = ctx.run_ui(
            egui::RawInput {
                viewport_id: viewport.ids.this,
                viewports: std::iter::once((viewport.ids.this, egui::ViewportInfo::default()))
                    .collect(),
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
                time: Some(time),
                ..Default::default()
            },
            |ui| (viewport.viewport_ui_cb)(ui),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            VIEWPORT_TREES.with_borrow_mut(|trees| trees.push(update));
        }
    });
}

/// The scene's last frame: the main viewport's tree, then each native
/// viewport's.
fn run(scene: &Scene) -> Vec<(String, TreeUpdate)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    if scene.native_viewports {
        ctx.set_embed_viewports(false);
        render_native_viewports();
    }
    let mut app = RSpiceApp::test_instance();
    app.symbol_library = crate::schematic::symbols::SymbolLibrary::load_embedded().ok();
    (scene.setup)(&mut app);
    let mut frame = eframe::Frame::_new_kittest();
    let mut trees = Vec::new();
    for index in 0..FRAMES {
        VIEWPORT_TREES.with_borrow_mut(Vec::clear);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, scene.size)),
                time: Some(f64::from(index) * FRAME_SECONDS),
                focused: true,
                ..Default::default()
            },
            |ui| eframe::App::ui(&mut app, ui, &mut frame),
        );
        trees = vec![(
            "main".to_owned(),
            output
                .platform_output
                .accesskit_update
                .expect("a frame with AccessKit on publishes its tree"),
        )];
        for (index, tree) in VIEWPORT_TREES.take().into_iter().enumerate() {
            trees.push((format!("native viewport {}", index + 1), tree));
        }
    }
    assert!(
        (scene.holds)(&app),
        "scene {} is no longer the state it is named for after its frames",
        scene.name
    );
    if scene.native_viewports {
        assert!(
            trees.len() > 1,
            "scene {} drew no native viewport",
            scene.name
        );
    }
    trees
}

/// A bound rounded to the nearest half point, so subpixel layout noise is not
/// a difference.
fn half_point(value: f64) -> f64 {
    (value * 2.0).round() / 2.0 + 0.0
}

fn describe(node: &Node) -> String {
    let mut line = format!("{:?}", node.role());
    if let Some(label) = node.label() {
        write!(line, " label={label:?}").expect("a String takes any write");
    }
    if let Some(value) = node.value() {
        write!(line, " value={value:?}").expect("a String takes any write");
    }
    if let Some(toggled) = node.toggled() {
        write!(line, " toggled={toggled:?}").expect("a String takes any write");
    }
    if let Some(selected) = node.is_selected() {
        write!(line, " selected={selected}").expect("a String takes any write");
    }
    if let Some(bounds) = node.bounds() {
        write!(
            line,
            " @ {:.1},{:.1} {:.1},{:.1}",
            half_point(bounds.x0),
            half_point(bounds.y0),
            half_point(bounds.x1),
            half_point(bounds.y1)
        )
        .expect("a String takes any write");
    }
    without_uuids(&line)
}

/// `text` with each UUID written as `<uuid>`. A new session mints its own,
/// so one names the same object differently on every run.
fn without_uuids(text: &str) -> String {
    const GROUPS: [usize; 5] = [8, 4, 4, 4, 12];
    const LENGTH: usize = 36;
    let bytes = text.as_bytes();
    let uuid_at = |start: usize| {
        let mut at = start;
        for (index, digits) in GROUPS.into_iter().enumerate() {
            if index > 0 {
                if bytes.get(at) != Some(&b'-') {
                    return false;
                }
                at += 1;
            }
            for _ in 0..digits {
                if !bytes.get(at).is_some_and(u8::is_ascii_hexdigit) {
                    return false;
                }
                at += 1;
            }
        }
        true
    };
    let mut written = String::with_capacity(text.len());
    let mut at = 0;
    while let Some(character) = text[at..].chars().next() {
        if uuid_at(at) {
            written.push_str("<uuid>");
            at += LENGTH;
        } else {
            written.push(character);
            at += character.len_utf8();
        }
    }
    written
}

/// `tree` one node per line, depth first from its root, indented by depth.
/// A node the root does not reach is listed after the tree, in update order.
fn write_tree(into: &mut String, name: &str, tree: &TreeUpdate) -> usize {
    let nodes = tree
        .nodes
        .iter()
        .map(|(id, node)| (*id, node))
        .collect::<std::collections::HashMap<NodeId, &Node>>();
    let root = tree.tree.as_ref().expect("a full tree names its root").root;
    writeln!(into, "viewport {name}: {} nodes", tree.nodes.len())
        .expect("a String takes any write");
    let mut reached = std::collections::HashSet::new();
    let mut pending = vec![(root, 0_usize)];
    while let Some((id, depth)) = pending.pop() {
        let Some(node) = nodes.get(&id) else {
            writeln!(into, "{}<missing node>", "  ".repeat(depth))
                .expect("a String takes any write");
            continue;
        };
        if !reached.insert(id) {
            continue;
        }
        writeln!(into, "{}{}", "  ".repeat(depth), describe(node))
            .expect("a String takes any write");
        pending.extend(
            node.children()
                .iter()
                .rev()
                .map(|child| (*child, depth + 1)),
        );
    }
    for (id, node) in &tree.nodes {
        if !reached.contains(id) {
            writeln!(into, "unreached {}", describe(node)).expect("a String takes any write");
        }
    }
    tree.nodes.len()
}

/// `app-split/accesskit/` under the target directory this build uses.
fn dump_dir() -> PathBuf {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .map_or_else(|| workspace.join("target"), |dir| workspace.join(dir));
    target.join("app-split").join("accesskit")
}

#[test]
#[ignore = "writes the AccessKit dump that two builds are compared by"]
fn accesskit_dump() {
    let dir = dump_dir();
    if dir.exists() {
        std::fs::remove_dir_all(&dir)
            .unwrap_or_else(|error| panic!("clear {}: {error}", dir.display()));
    }
    std::fs::create_dir_all(&dir)
        .unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    for scene in &scenes() {
        let mut text = format!(
            "scene {} at {}x{}\n",
            scene.name, scene.size.x, scene.size.y
        );
        let mut nodes = 0;
        for (name, tree) in run(scene) {
            nodes += write_tree(&mut text, &name, &tree);
        }
        assert!(nodes > 0, "scene {} has an empty tree", scene.name);
        let path = dir.join(format!("{}.txt", scene.name));
        std::fs::write(&path, text)
            .unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
    }
}
