//! What a screen reader is told about the application, scene by scene,
//! written out so two builds can be compared line for line.
//!
//! Moving code between files and containers must not change what the reader
//! is shown or in what order. The AccessKit tree records both: every widget a
//! frame lays out, its name, its state and where it sits. Each scene here runs
//! whole application frames through `eframe::App::ui`, the frame eframe runs,
//! and writes the last frame's tree to its own text file, one node per line in
//! tree order. A scene that opens a menu, the palette or More tools then has
//! the pointer rest on or press a node of that tree, as a reader does, and
//! runs frames again. A build is unchanged when its files match the files of the
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

use egui::accesskit::{Node, NodeId, Role, TreeUpdate};
use egui::{Event, Modifiers, PointerButton, Pos2, Rect, Vec2};

use super::veriloga_design_tests::{add_veriloga_cell, open_in_design};
use super::{AppState, RSpiceApp};
use crate::state::{
    AnalysisResult, AnalysisType, Cell, CellViewRef, Point, RetainedTrace, SimulationRun,
    SimulationRunLifecycle, SymbolDocument, SymbolEditorMetadata, SymbolShape, View, ViewType,
};
use crate::workbench::documents::code_workspace::CodeWorkspacePage;
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

/// The application menus, by the names their triggers carry, in the bar's
/// order.
const APPLICATION_MENUS: [&str; 11] = [
    "File",
    "Edit",
    "View",
    "Design",
    "Simulate",
    "Results",
    "Verify",
    "Models",
    "Automation",
    "Window",
    "Help",
];

/// The command palette's search scopes, by the names their chips carry.
const PALETTE_SCOPES: [&str; 11] = [
    "All",
    "Navigate",
    "Commands",
    "Design",
    "Simulation",
    "Results",
    "Verification",
    "Models",
    "Automation",
    "Signals",
    "Help",
];

type Setup = Box<dyn Fn(&mut RSpiceApp)>;
type Holds = Box<dyn Fn(&RSpiceApp) -> bool>;
type Shows = Box<dyn Fn(&TreeUpdate) -> bool>;

/// What a reader does once a scene's frames have run: the pointer comes to
/// rest over the node of `role` named `label` in the last frame's tree, and
/// presses `press` there when it is set.
#[derive(Clone, Copy)]
struct Gesture {
    role: Role,
    label: &'static str,
    press: Option<PointerButton>,
}

impl Gesture {
    const fn click(role: Role, label: &'static str) -> Self {
        Self {
            role,
            label,
            press: Some(PointerButton::Primary),
        }
    }

    const fn secondary_click(role: Role, label: &'static str) -> Self {
        Self {
            role,
            label,
            press: Some(PointerButton::Secondary),
        }
    }

    const fn hover(role: Role, label: &'static str) -> Self {
        Self {
            role,
            label,
            press: None,
        }
    }
}

/// One state of the application to dump.
struct Scene {
    name: String,
    size: Vec2,
    /// Puts a fresh test instance into the scene's state.
    setup: Setup,
    /// What the reader does after the setup's frames, in order; each gesture
    /// is followed by frames of its own.
    gestures: Vec<Gesture>,
    /// Whether the scene is still the state it is named for after its frames;
    /// a scene that quietly fell back to another would dump the wrong tree.
    holds: Holds,
    /// Whether the last frame's main tree shows the state the scene is named
    /// for, such as an open menu, which lives in egui's memory rather than in
    /// the application.
    shows: Shows,
    /// Draws secondary windows as native viewports rather than embedded in the
    /// main one.
    native_viewports: bool,
    /// Writes what a row tells a reader beyond its name: whether it is
    /// greyed, why, its key and whether it is expanded.
    detailed: bool,
}

impl Scene {
    fn new(name: impl Into<String>, size: Vec2, setup: impl Fn(&mut RSpiceApp) + 'static) -> Self {
        Self {
            name: name.into(),
            size,
            setup: Box::new(setup),
            gestures: Vec::new(),
            holds: Box::new(|_| true),
            shows: Box::new(|_| true),
            native_viewports: false,
            detailed: false,
        }
    }

    fn holding(mut self, holds: impl Fn(&RSpiceApp) -> bool + 'static) -> Self {
        self.holds = Box::new(holds);
        self
    }

    /// The scene after `gesture`, with every row written in detail.
    fn then(mut self, gesture: Gesture) -> Self {
        self.gestures.push(gesture);
        self.detailed = true;
        self
    }

    fn showing(mut self, shows: impl Fn(&TreeUpdate) -> bool + 'static) -> Self {
        self.shows = Box::new(shows);
        self
    }
}

/// The node of `role` named `label` in `tree`.
fn find_node<'a>(tree: &'a TreeUpdate, role: Role, label: &str) -> Option<&'a Node> {
    tree.nodes
        .iter()
        .map(|(_, node)| node)
        .find(|node| node.role() == role && node.label() == Some(label))
}

/// Whether `tree` shows the menu `trigger` opens: the trigger expanded and its
/// menu, named after it, in the tree.
fn menu_is_open(tree: &TreeUpdate, trigger: &str) -> bool {
    find_node(tree, Role::MenuItem, trigger).is_some_and(|node| node.is_expanded() == Some(true))
        && find_node(tree, Role::Menu, &format!("{trigger} menu")).is_some()
}

/// Whether `tree` shows the submenu the row `label` opens.
fn submenu_is_open(tree: &TreeUpdate, label: &str) -> bool {
    tree.nodes.iter().any(|(_, node)| {
        node.role() == Role::MenuItem
            && node.label() == Some(label)
            && node.is_expanded() == Some(true)
    })
}

/// A drawing open in Design: the page the application menus are read over.
fn open_design(app: &mut RSpiceApp) {
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
}

/// The scenes that open what a reader opens on top of a page: each
/// application menu and its submenu, the compact menu and each of its
/// submenus, the source editor's context menu, the command palette in each
/// scope, and the toolbar's More tools.
fn opened_scenes() -> Vec<Scene> {
    let mut scenes = Vec::new();
    for menu in APPLICATION_MENUS {
        scenes.push(
            Scene::new(
                format!("menu-{}", menu.to_lowercase()),
                DESKTOP,
                open_design,
            )
            .then(Gesture::click(Role::MenuItem, menu))
            .showing(move |tree| menu_is_open(tree, menu)),
        );
    }
    scenes.push(
        Scene::new("menu-design-arrange", DESKTOP, open_design)
            .then(Gesture::click(Role::MenuItem, "Design"))
            .then(Gesture::hover(Role::MenuItem, "Arrange"))
            .showing(|tree| menu_is_open(tree, "Design") && submenu_is_open(tree, "Arrange")),
    );
    // Edit and View hold rows of their own while a source editor is the
    // document.
    for menu in ["Edit", "View"] {
        scenes.push(
            Scene::new(
                format!("menu-{}-source-editor", menu.to_lowercase()),
                DESKTOP,
                |app| {
                    app.state.open_project_session();
                    open_a_veriloga_view(&mut app.state);
                },
            )
            .then(Gesture::click(Role::MenuItem, menu))
            .holding(|app| app.state.workbench.active_source_editor.is_some())
            .showing(move |tree| menu_is_open(tree, menu)),
        );
    }
    scenes.push(
        Scene::new("menu-compact", PHONE, |app| on_a_phone(app, None))
            .then(Gesture::click(Role::MenuItem, "Menu"))
            .showing(|tree| menu_is_open(tree, "Menu")),
    );
    for menu in APPLICATION_MENUS {
        scenes.push(
            Scene::new(
                format!("menu-compact-{}", menu.to_lowercase()),
                PHONE,
                |app| on_a_phone(app, None),
            )
            .then(Gesture::click(Role::MenuItem, "Menu"))
            .then(Gesture::hover(Role::MenuItem, menu))
            .showing(move |tree| menu_is_open(tree, "Menu") && submenu_is_open(tree, menu)),
        );
    }
    scenes.push(
        Scene::new("menu-source-editor", DESKTOP, |app| {
            app.state.open_project_session();
            open_a_veriloga_view(&mut app.state);
        })
        .then(Gesture::secondary_click(
            Role::MultilineTextInput,
            "Verilog-A source editor",
        ))
        .showing(|tree| find_node(tree, Role::Menu, "Source editor menu").is_some()),
    );
    for scope in PALETTE_SCOPES {
        let mut scene = Scene::new(
            format!("palette-{}", scope.to_lowercase()),
            DESKTOP,
            move |app| {
                open_design(app);
                app.state.workbench.command_palette.open_in_scope(scope);
            },
        )
        .holding(|app| app.state.workbench.command_palette.open)
        .showing(|tree| find_node(tree, Role::ListBox, "Matching commands").is_some());
        scene.detailed = true;
        scenes.push(scene);
    }
    scenes.push(
        Scene::new("toolbar-overflow", PHONE, |app| on_a_phone(app, None))
            .then(Gesture::click(Role::Button, "More tools"))
            // A phone's tool row has no slot for Undo, so it is a row of
            // More tools, which says whether it is the armed tool.
            .showing(|tree| {
                find_node(tree, Role::Button, "Undo").is_some_and(|node| node.toggled().is_some())
            }),
    );
    scenes
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
    for (view_type, open) in [
        (ViewType::Symbol, open_a_symbol_view as fn(&mut AppState)),
        (ViewType::VerilogA, open_a_veriloga_view),
    ] {
        let name = format!("{view_type:?}").to_lowercase();
        scenes.push(
            Scene::new(format!("design-{name}-view"), DESKTOP, move |app| {
                app.state.open_project_session();
                open(&mut app.state);
            })
            .holding(move |app| {
                app.state.workbench.workspace == Workspace::Design
                    && app.state.model.workspace.content.active_view_type() == view_type
            }),
        );
    }
    scenes.push(
        Scene::new("netlist-automation-page", DESKTOP, |app| {
            app.state.open_project_session();
            app.state.workbench.activate(Workspace::Netlist);
            app.state.code.code_workspace.page = CodeWorkspacePage::Automation;
        })
        .holding(|app| {
            app.state.workbench.workspace == Workspace::Netlist
                && app.state.code.code_workspace.page == CodeWorkspacePage::Automation
        }),
    );
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
    scenes.extend(opened_scenes());
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

/// A symbol cellview open in Design, holding one circle so its canvas has a
/// shape to draw.
fn open_a_symbol_view(state: &mut AppState) {
    let library = state.model.workspace.content.active_view.library.clone();
    let mut cell = Cell::new("buffer");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    state
        .model
        .library_manager
        .get_library_mut(&library)
        .expect("the open project has its active view's library")
        .add_cell(cell);
    state.open_workspace_view(CellViewRef::new(&library, "buffer", "symbol"));
    let document = SymbolDocument {
        body: vec![SymbolShape::Circle {
            center: Point::origin(),
            radius: 10,
        }],
        ..Default::default()
    };
    state
        .store_active_symbol_editor_bundle(
            &document,
            &SymbolEditorMetadata::for_document(&document),
        )
        .expect("the symbol view takes its document");
    state.workbench.activate(Workspace::Design);
}

/// A Verilog-A cellview open in Design, its one-file bundle in the editor.
fn open_a_veriloga_view(state: &mut AppState) {
    let (reference, _) = add_veriloga_cell(state, "gain", "model.va", "gain");
    open_in_design(state, &reference);
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
    let mut frames = Frames {
        ctx: &ctx,
        app: &mut app,
        frame: &mut frame,
        size: scene.size,
        next: 0,
    };
    let mut trees = frames.run(Vec::new());
    for gesture in &scene.gestures {
        let bounds = find_node(&trees[0].1, gesture.role, gesture.label)
            .and_then(Node::bounds)
            .unwrap_or_else(|| {
                panic!(
                    "scene {} has no {:?} named {:?} to reach",
                    scene.name, gesture.role, gesture.label
                )
            });
        // The frame laid the node out in `f32` points.
        let at = Pos2::new(
            ((bounds.x0 + bounds.x1) / 2.0) as f32,
            ((bounds.y0 + bounds.y1) / 2.0) as f32,
        );
        let mut events = vec![Event::PointerMoved(at)];
        if let Some(button) = gesture.press {
            for pressed in [true, false] {
                events.push(Event::PointerButton {
                    pos: at,
                    button,
                    pressed,
                    modifiers: Modifiers::NONE,
                });
            }
        }
        trees = frames.run(events);
    }
    assert!(
        (scene.holds)(&app),
        "scene {} is no longer the state it is named for after its frames",
        scene.name
    );
    assert!(
        (scene.shows)(&trees[0].1),
        "scene {}'s tree does not show the state it is named for",
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

/// A scene's application and the frames it has run so far.
struct Frames<'a> {
    ctx: &'a egui::Context,
    app: &'a mut RSpiceApp,
    frame: &'a mut eframe::Frame,
    size: Vec2,
    /// The index of the next frame, which sets its time.
    next: u32,
}

impl Frames<'_> {
    /// Run [`FRAMES`] frames, the first of them carrying `events`, and return
    /// the last one's trees.
    fn run(&mut self, mut events: Vec<Event>) -> Vec<(String, TreeUpdate)> {
        let mut trees = Vec::new();
        for index in self.next..self.next + FRAMES {
            VIEWPORT_TREES.with_borrow_mut(Vec::clear);
            let output = self.ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, self.size)),
                    time: Some(f64::from(index) * FRAME_SECONDS),
                    focused: true,
                    events: std::mem::take(&mut events),
                    ..Default::default()
                },
                |ui| eframe::App::ui(self.app, ui, self.frame),
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
        self.next += FRAMES;
        trees
    }
}

/// A bound rounded to the nearest half point, so subpixel layout noise is not
/// a difference.
fn half_point(value: f64) -> f64 {
    (value * 2.0).round() / 2.0 + 0.0
}

fn describe(node: &Node, detailed: bool) -> String {
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
    if detailed {
        if node.is_disabled() {
            line.push_str(" disabled");
        }
        if let Some(expanded) = node.is_expanded() {
            write!(line, " expanded={expanded}").expect("a String takes any write");
        }
        if let Some(shortcut) = node.keyboard_shortcut() {
            write!(line, " shortcut={shortcut:?}").expect("a String takes any write");
        }
        if let Some(description) = node.description() {
            write!(line, " description={description:?}").expect("a String takes any write");
        }
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
fn write_tree(into: &mut String, name: &str, tree: &TreeUpdate, detailed: bool) -> usize {
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
        writeln!(into, "{}{}", "  ".repeat(depth), describe(node, detailed))
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
            writeln!(into, "unreached {}", describe(node, detailed))
                .expect("a String takes any write");
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
            nodes += write_tree(&mut text, &name, &tree, scene.detailed);
        }
        assert!(nodes > 0, "scene {} has an empty tree", scene.name);
        let path = dir.join(format!("{}.txt", scene.name));
        std::fs::write(&path, text)
            .unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
    }
}
