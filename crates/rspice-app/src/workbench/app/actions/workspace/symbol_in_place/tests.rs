//! Editing a symbol in place, driven through real egui frames: the symbol
//! stands where its instance does, the pointer edits it in symbol
//! coordinates, and leaving with edits asks first.

use egui::{Color32, Context, Pos2, Rect, Vec2, vec2};

use crate::schematic::view::schematic_symbol_context;
use crate::state::{
    Cell, CellViewRef, ComponentType, LibraryCellInstance, Point, PortSpec, Rotation,
    SchematicState, SymbolDocument, SymbolEditorMetadata, SymbolPin, SymbolShape, View, ViewType,
    Wire,
};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::Workspace;
use rspice_schematic_editor::symbol_editor::session::SymbolSelection;
use rspice_schematic_editor::view::coordinates::viewport_from_camera;

const SCREEN: Vec2 = Vec2::new(640.0, 480.0);
const PAN: (f64, f64) = (60.0, 40.0);
const ZOOM: f64 = 2.0;

/// The `amp` symbol for the interface `ports` declares: a square body and
/// three pins. Each pin is the only one 10, 20 or 30 off an axis, so no
/// orientation puts one where another stood.
fn amp_symbol(ports: &[PortSpec]) -> SymbolDocument {
    let corners = [(-20, -20), (20, -20), (20, 20), (-20, 20)];
    let positions = [(-40, -10), (40, 20), (30, -40)];
    SymbolDocument {
        body: vec![SymbolShape::Polyline {
            points: corners.map(|(x, y)| Point::new(x, y)).to_vec(),
            closed: true,
        }],
        pins: ports
            .iter()
            .zip(positions)
            .map(|(port, (x, y))| {
                SymbolPin::new(&port.name, port.direction, Some(Point::new(x, y)))
            })
            .collect(),
        ..Default::default()
    }
}

/// The `amp` schematic, declaring ports A, B and C.
fn amp_schematic() -> SchematicState {
    let mut schematic = SchematicState::default();
    for (index, name) in ["A", "B", "C"].into_iter().enumerate() {
        let id = schematic.add_component(ComponentType::Port, Point::new(0, 40 * index as i32));
        schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .unwrap()
            .value = name.to_owned();
    }
    schematic
}

struct Fixture {
    ctx: Context,
    app: RSpiceApp,
    canvas: Rect,
    time: f64,
    parent: CellViewRef,
    symbol: CellViewRef,
    instances: Vec<u64>,
}

impl Fixture {
    /// A parent schematic placing the `amp` symbol once per entry, each at
    /// its position, turned and mirrored as the entry says.
    fn new(placements: &[(Point, Rotation, bool)]) -> Self {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        app.state.workbench.activate(Workspace::Design);
        app.state.schematic = SchematicState::default();
        let parent = app.state.workspace.content.active_view.clone();
        let symbol = CellViewRef::new(&parent.library, "amp", "symbol");
        let mut cell = Cell::new(&symbol.cell);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cell.add_view(View::new(&symbol.view, ViewType::Symbol));
        app.state
            .library_manager
            .get_library_mut(&symbol.library)
            .unwrap()
            .add_cell(cell);
        let schematic = amp_schematic();
        let ports = schematic.document().interface_ports();
        app.state.workspace.insert_schematic_editor(
            CellViewRef::new(&symbol.library, &symbol.cell, "schematic").key(),
            schematic,
        );
        app.state.open_workspace_view(symbol.clone());
        let document = amp_symbol(&ports);
        app.state
            .store_active_symbol_editor_bundle(
                &document,
                &SymbolEditorMetadata::for_document(&document),
            )
            .unwrap();
        app.state.open_workspace_view(parent.clone());
        app.state.workspace.content.close_view(&symbol);
        assert_eq!(app.state.workspace.content.active_view, parent);

        let mut instances = Vec::new();
        for &(position, rotation, mirror_h) in placements {
            let id = app.state.schematic.add_library_cell_component(
                position,
                LibraryCellInstance::new(&symbol.library, &symbol.cell, &symbol.view),
            );
            let component = app
                .state
                .schematic
                .document_mut_for_test()
                .components
                .iter_mut()
                .find(|component| component.id == id)
                .unwrap();
            component.rotation = rotation;
            component.mirror_h = mirror_h;
            instances.push(id);
        }
        app.state.schematic.recalculate_runtime_state();
        app.state.schematic.clear_undo_history();
        app.state.schematic.session.is_dirty = false;
        let editor = &mut app.state.schematic.session.editor;
        editor.needs_fit = false;
        editor.needs_drawing_sheet_fit = false;
        editor.pan = PAN;
        editor.zoom = ZOOM;
        app.state.sync_active_schematic_to_workspace();
        let mut fixture = Self {
            ctx,
            app,
            canvas: Rect::NOTHING,
            time: 0.0,
            parent,
            symbol,
            instances,
        };
        fixture.frame(vec![]);
        fixture
    }

    fn frame(&mut self, events: Vec<egui::Event>) -> egui::FullOutput {
        self.time += 0.02;
        let Self {
            ctx,
            app,
            canvas,
            time,
            ..
        } = self;
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SCREEN)),
                events,
                focused: true,
                time: Some(*time),
                ..Default::default()
            },
            |ui| {
                app.state.reconcile_schematic_drag(ui.ctx(), false);
                app.handle_shortcuts(ui.ctx());
                *canvas = ui.available_rect_before_wrap();
                if app.state.workspace.content.active_view_type() == ViewType::Symbol {
                    crate::schematic::symbol_editor::show(ui, &mut app.state, None);
                } else {
                    crate::schematic::view::render_schematic_view(ui, &mut app.state, None);
                }
                app.state.reconcile_schematic_drag(ui.ctx(), true);
            },
        )
    }

    /// Select `instance` on the parent and edit its symbol in place.
    fn enter(&mut self, instance: usize) {
        let id = self.instances[instance];
        let editor = &mut self.app.state.schematic.session.editor;
        editor.selection.select_only_component(id);
        Command::EditSymbolInPlace.execute(&mut self.app);
        assert_eq!(self.app.state.workspace.content.active_view, self.symbol);
        assert!(self.app.state.symbol_in_place().is_some());
        self.frame(vec![]);
    }

    /// Where the parent draws `instance`'s pins, by name.
    fn terminals(&self, instance: usize) -> Vec<(String, Point)> {
        let id = self.instances[instance];
        let state = &self.app.state;
        let component = state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .unwrap();
        schematic_symbol_context(state).named_terminal_points(component)
    }

    /// Where a parent-sheet point lands on screen, through the parent's
    /// camera as the session entered with it.
    fn screen(&self, world: Point) -> Pos2 {
        viewport_from_camera(PAN, ZOOM, self.canvas, 1.0).schematic_to_screen(world)
    }

    fn press_drag_release(&mut self, from: Pos2, to: Pos2) {
        let button = |pos, pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        self.frame(vec![egui::Event::PointerMoved(from), button(from, true)]);
        self.frame(vec![egui::Event::PointerMoved(from + vec2(8.0, 0.0))]);
        self.frame(vec![egui::Event::PointerMoved(to)]);
        self.frame(vec![button(to, false)]);
        self.frame(vec![]);
    }

    /// Press `key` and let go of it, a frame each.
    fn key(&mut self, key: egui::Key) {
        for pressed in [true, false] {
            self.frame(vec![egui::Event::Key {
                key,
                physical_key: Some(key),
                pressed,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }]);
        }
        self.frame(vec![]);
    }

    fn symbol_pin(&self, name: &str) -> Option<Point> {
        let document = self.app.state.load_symbol_document(&self.symbol).unwrap();
        document.pin(name).and_then(|pin| pin.position)
    }

    fn leaving(&self) -> bool {
        self.app
            .state
            .symbol_in_place()
            .is_some_and(|session| session.leaving.is_some())
    }

    fn on_parent(&self) -> bool {
        self.app.state.workspace.content.active_view == self.parent
            && self.app.state.symbol_in_place().is_none()
    }
}

/// The centre of the `accent` ink within ten pixels of `around`, and how
/// many pixels of it there are.
fn accent_centroid(
    canvas: &crate::ui::raster::Canvas,
    accent: Color32,
    around: Pos2,
) -> (Pos2, usize) {
    let near = |pixel: Color32| {
        pixel
            .to_array()
            .iter()
            .zip(accent.to_array())
            .take(3)
            .all(|(a, b)| a.abs_diff(b) <= 40)
    };
    let (mut sum, mut count) = (Vec2::ZERO, 0usize);
    let (cx, cy) = (around.x.round() as i64, around.y.round() as i64);
    for y in cy - 10..=cy + 10 {
        for x in cx - 10..=cx + 10 {
            let (Ok(px), Ok(py)) = (usize::try_from(x), usize::try_from(y)) else {
                continue;
            };
            if canvas.pixel(px, py).is_some_and(near) {
                sum += vec2(x as f32 + 0.5, y as f32 + 0.5);
                count += 1;
            }
        }
    }
    (Pos2::ZERO + sum / count.max(1) as f32, count)
}

/// Every pin of the symbol is drawn on the pin of the instance it is edited
/// through, at each quarter turn, mirrored and not: the pads the canvas
/// paints land within a pixel of where the parent connects that pin.
#[test]
fn in_place_pins_land_on_the_instance_pins_in_every_orientation() {
    let mut failures = Vec::new();
    for rotation in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
        for mirror_h in [false, true] {
            let mut fixture = Fixture::new(&[(Point::new(100, 80), rotation, mirror_h)]);
            let terminals = fixture.terminals(0);
            assert_eq!(terminals.len(), 3, "the parent resolves the amp symbol");
            fixture.enter(0);
            let mut selection = SymbolSelection::default();
            for name in ["A", "B", "C"] {
                selection.toggle_pin(name);
            }
            fixture.app.state.ui.symbol.editor.set_selection(selection);
            fixture.frame(vec![]);
            let output = fixture.frame(vec![]);
            let accent = crate::ui::tokens::Tokens::get(&fixture.ctx).color.accent;
            let canvas = crate::ui::raster::rasterize(&fixture.ctx, SCREEN, &output);
            for (name, world) in &terminals {
                let expected = fixture.screen(*world);
                let (centre, count) = accent_centroid(&canvas, accent, expected);
                if count < 8 || (centre - expected).length() > 1.0 {
                    failures.push(format!(
                        "{rotation:?} mirrored={mirror_h} pin {name}: pad at {centre:?} \
                         ({count} px), instance pin at {expected:?}"
                    ));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// A pin dragged in place lands where the pointer let go of it, read back
/// through the instance's transform into symbol coordinates, and the parent
/// wire it was attached to stays where it was.
#[test]
fn a_pin_dragged_in_place_lands_under_the_pointer_in_symbol_coordinates() {
    let position = Point::new(100, 80);
    let mut fixture = Fixture::new(&[(position, Rotation::R90, true)]);
    let start = fixture
        .terminals(0)
        .into_iter()
        .find(|(name, _)| name == "A")
        .unwrap()
        .1;
    fixture.app.state.schematic.document_mut_for_test().wires =
        vec![Wire::new(1, vec![start, start + Point::new(0, 60)])];
    fixture.app.state.schematic.recalculate_runtime_state();
    fixture.app.state.sync_active_schematic_to_workspace();
    let document = fixture.app.state.schematic.document();
    let wires = document.wires.clone();
    let instance = document
        .components
        .iter()
        .find(|component| component.id == fixture.instances[0])
        .unwrap();
    let target = Point::new(-40, 20);
    // The symbol's origin is (0, 0), so a symbol point turns about it.
    let world = position + instance.transform_point(target);
    let (from, to) = (fixture.screen(start), fixture.screen(world));
    fixture.enter(0);
    fixture.press_drag_release(from, to);

    assert_eq!(fixture.symbol_pin("A"), Some(target));
    let parent = fixture
        .app
        .state
        .workspace
        .content
        .schematic_buffers
        .get(&fixture.parent.key())
        .unwrap();
    assert_eq!(parent.document().wires, wires, "no parent wire moved");
}

/// Save publishes the edit to the master: a second instance, turned the
/// other way, connects the moved pin where the edit put it.
#[test]
fn saving_in_place_updates_every_instance_of_the_symbol() {
    let (first, second) = (Point::new(100, 80), Point::new(200, 80));
    let mut fixture = Fixture::new(&[
        (first, Rotation::R0, false),
        (second, Rotation::R180, false),
    ]);
    let revision = |fixture: &Fixture| {
        let document = fixture
            .app
            .state
            .load_symbol_document(&fixture.symbol)
            .unwrap();
        fixture
            .app
            .state
            .load_symbol_editor_metadata(&fixture.symbol, &document)
            .unwrap()
            .revision
    };
    let before = revision(&fixture);
    fixture.enter(0);
    let from = fixture.screen(first + Point::new(-40, -10));
    let to = fixture.screen(first + Point::new(-40, 20));
    fixture.press_drag_release(from, to);
    assert_eq!(fixture.symbol_pin("A"), Some(Point::new(-40, 20)));

    fixture.key(egui::Key::Escape);
    assert!(fixture.leaving(), "Escape with an edit asks first");
    // Save, then the save dialog's own Save.
    fixture.key(egui::Key::Enter);
    assert!(fixture.app.state.ui.symbol.editor.save_dialog_open);
    // The revision note the save dialog asks for, typed into its field.
    fixture.frame(vec![egui::Event::Text("Move pin A".into())]);
    fixture.key(egui::Key::Enter);

    assert!(fixture.on_parent(), "the save finished leaving");
    assert!(revision(&fixture) > before, "the save published a revision");
    let terminal = fixture
        .terminals(1)
        .into_iter()
        .find(|(name, _)| name == "A")
        .unwrap()
        .1;
    assert_eq!(terminal, second + Point::new(40, -20));
}

/// Escape and Return leave at once when nothing changed. With an edit,
/// Escape asks; Cancel stays with the edit kept, and Discard puts the
/// symbol back as it was before leaving.
#[test]
fn leaving_in_place_asks_only_when_the_symbol_has_unsaved_edits() {
    let position = Point::new(100, 80);
    let mut fixture = Fixture::new(&[(position, Rotation::R0, false)]);
    fixture.enter(0);
    fixture.key(egui::Key::Escape);
    assert!(fixture.on_parent(), "Escape with nothing changed leaves");
    assert_eq!(
        fixture
            .app
            .state
            .schematic
            .session
            .editor
            .selection
            .single_component(),
        Some(fixture.instances[0]),
        "the instance is selected again"
    );
    assert!(
        !fixture
            .app
            .state
            .workspace
            .content
            .open_views
            .iter()
            .any(|open| open.reference == fixture.symbol),
        "the symbol tab the session opened is closed"
    );

    fixture.enter(0);
    let empty = fixture.screen(position + Point::new(0, 100));
    fixture.press_drag_release(empty, empty);
    fixture.key(egui::Key::Enter);
    assert!(fixture.on_parent(), "Return with nothing changed leaves");

    fixture.enter(0);
    let from = fixture.screen(position + Point::new(-40, -10));
    let to = fixture.screen(position + Point::new(-40, 20));
    fixture.press_drag_release(from, to);
    assert_eq!(fixture.symbol_pin("A"), Some(Point::new(-40, 20)));
    fixture.key(egui::Key::Escape);
    assert!(fixture.leaving(), "Escape with an edit asks");
    fixture.key(egui::Key::Escape);
    assert!(!fixture.leaving(), "Escape in the question cancels it");
    assert!(fixture.app.state.symbol_in_place().is_some(), "and stays");
    assert_eq!(fixture.symbol_pin("A"), Some(Point::new(-40, 20)));

    fixture.key(egui::Key::Escape);
    assert!(fixture.leaving());
    fixture.app.state.discard_and_leave_symbol_in_place();
    assert!(fixture.on_parent(), "Discard leaves");
    assert_eq!(fixture.symbol_pin("A"), Some(Point::new(-40, -10)));
}
