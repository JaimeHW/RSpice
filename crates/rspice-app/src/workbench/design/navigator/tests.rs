//! What the design navigator claims about the design it is reading.
//!
//! The hierarchy rows are the load-bearing ones: an unfolded node builds its
//! children and a folded one does not, a master that repeats on its own
//! ancestry is an inert leaf, and an occurrence click lands the session on
//! that occurrence. The rest hold the mockup geometry, the section folds and
//! the keyboard's walk along the rails.
//!
//! The shelf tab's own cases are in [`shelf`] and the whole-design rail
//! contracts — the interface, the RF ports and every occurrence a run reaches
//! a source through — are in [`rails`]. The fixtures and the frame-reading
//! helpers they share with these — [`painted_runs`], [`run_rect`],
//! [`glyph_canvas`] — stay here, because a harness that reads one painted
//! frame belongs to no single tab.

use super::shelf::*;
use super::*;
use crate::state::{CellViewRef, ComponentType, SchematicState};
use crate::workbench::RSpiceApp;
use crate::workbench::state::Workspace;

#[cfg(not(target_arch = "wasm32"))]
mod object_rows;
pub(crate) mod shelf;
mod shelf_search;

/// Give the project one more schematic master: the library cell view, and the
/// content the projection will materialize for it.
pub(crate) fn add_master(
    state: &mut crate::workbench::app_state::AppState,
    library: &str,
    cell: &str,
    schematic: SchematicState,
) {
    use crate::state::{Library, View, ViewType};

    if state.model.library_manager.get_library(library).is_none() {
        state
            .model
            .library_manager
            .add_library(Library::new(library));
    }
    let owner = state
        .model
        .library_manager
        .get_library_mut(library)
        .expect("the fixture library exists");
    let target = owner.get_or_create_cell(cell);
    if target.get_view("schematic").is_none() {
        target.add_view(View::new("schematic", ViewType::Schematic));
    }
    state.model.workspace.insert_schematic_editor(
        CellViewRef::new(library, cell, "schematic").key(),
        schematic,
    );
}

/// One placed hierarchical instance, under the name the design gives it.
pub(crate) fn placed(id: u64, name: &str, library: &str, cell: &str) -> crate::state::Component {
    let mut component = crate::state::Component::new(
        id,
        ComponentType::CellInstance,
        crate::state::Point::new(40, 40),
    )
    .with_library_cell(crate::state::LibraryCellInstance::new(
        library,
        cell,
        "schematic",
    ));
    component.name = name.to_owned();
    component
}

/// A design that recurses. The root places `XA` of `work/child`, and
/// `work/child` places a leaf beside a second instance of the root cell — so
/// `/XA/XLOOP` is the occurrence the resolver refuses to descend.
pub(crate) fn recursive_design() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let root = app.state.model.workspace.content.active_view.clone();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(placed(101, "XA", "work", "child"));
    app.state.sync_active_schematic_to_workspace();

    let mut child = SchematicState::default();
    let components = &mut child.document_mut_for_test().components;
    components.push(placed(201, "XLEAF", "work", "leaf"));
    components.push(placed(202, "XLOOP", &root.library, &root.cell));
    add_master(&mut app.state, "work", "child", child);
    add_master(&mut app.state, "work", "leaf", SchematicState::default());
    app
}

/// A design whose active cell view holds two sheets: the named source `VIN`
/// on the sheet the session is on, `VBIAS` on the one it is not.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn two_sheet_named_signals() -> RSpiceApp {
    use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};

    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let components = &mut app.state.model.schematic.document_mut_for_test().components;
    for (id, name) in [(301, "VIN"), (302, "VBIAS")] {
        let mut source = crate::state::Component::new(
            id,
            ComponentType::VoltageSource,
            crate::state::Point::new(20, 20),
        );
        source.name = name.to_owned();
        components.push(source);
    }
    app.state.sync_active_schematic_to_workspace();

    let workspace = &mut app.state.model.workspace.content;
    let key = workspace.active_schematic_reference().key();
    let first = workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Input stage", [301, 302])
        .expect("the fixture cell view takes a sheet catalog");
    let catalog = workspace
        .design_management
        .sheet_catalog_mut(&key)
        .expect("the catalog was just bootstrapped");
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Output stage".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("a second sheet");
    catalog
        .assign_objects(catalog.revision(), second, [302])
        .expect("the off-sheet source takes its assignment");
    catalog
        .set_active(first)
        .expect("the first sheet is active");
    app
}

/// Everything one navigator surface paints, at the dock's real width.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn painted_panel(ctx: &egui::Context, mut contents: impl FnMut(&mut Ui)) -> String {
    painted_text(&ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.set_width(260.0);
                contents(ui);
            });
    }))
}

#[test]
fn design_tabs_keep_the_mockup_horizontal_inset() {
    let outer = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(260.0, 33.0));
    let content = panel_tabs_content_rect(outer);

    assert_eq!(content.left(), 8.0);
    assert_eq!(content.right(), 252.0);
}

#[test]
fn design_tabs_flex_from_their_label_widths_like_the_mockup() {
    let widths = flexible_tab_widths(239.0, [59.0, 95.0]);
    assert!((widths[0] - 101.5).abs() <= 0.001);
    assert!((widths[1] - 137.5).abs() <= 0.001);
    assert!((widths.iter().sum::<f32>() - 239.0).abs() <= 0.001);
}

#[test]
fn navigator_path_names_the_occurrence_and_the_master_bound_there() {
    let mut workspace = crate::state::ProjectWorkspace::default();
    let (occurrence, master, can_ascend) = navigator_path(&workspace);
    assert_eq!(occurrence, "/");
    assert_eq!(master, "user/top");
    assert!(!can_ascend);

    workspace.descend_into(
        "XAFE".to_owned(),
        crate::state::CellViewRef::new("user", "afe_core", "schematic"),
        crate::state::ViewType::Schematic,
    );
    let (occurrence, master, can_ascend) = navigator_path(&workspace);
    assert_eq!(occurrence, "/XAFE");
    assert_eq!(master, "user/afe_core");
    assert!(can_ascend);

    workspace.descend_into(
        "XBIAS".to_owned(),
        crate::state::CellViewRef::new("user", "bias", "schematic"),
        crate::state::ViewType::Schematic,
    );
    assert_eq!(navigator_path(&workspace).0, "/XAFE/XBIAS");
}

#[test]
fn raw_probe_targets_cover_scalar_differential_and_current_navigation() {
    assert_eq!(
        raw_probe_target("V(afe_out)"),
        Some(RawProbeTarget::Voltage {
            positive: "afe_out",
            negative: None,
        })
    );
    assert_eq!(
        raw_probe_target(" v(VREF) "),
        Some(RawProbeTarget::Voltage {
            positive: "VREF",
            negative: None,
        })
    );
    assert_eq!(
        raw_probe_target("V(out, in)"),
        Some(RawProbeTarget::Voltage {
            positive: "out",
            negative: Some("in"),
        })
    );
    assert_eq!(
        raw_probe_target("I(VDD)"),
        Some(RawProbeTarget::Current("VDD"))
    );
    assert_eq!(raw_probe_target("gain"), None);
    assert_eq!(raw_probe_target("V(out,)"), None);
}

/// Every run of text a frame painted: what it says, where it landed, and the
/// colour it was set in.
///
/// The rail is read from its shapes rather than its accessibility tree: a
/// section that resolves paints selectable rows and one that does not
/// paints a plain row, and only the shapes carry both. The colour is here
/// because a meta column that states a hazard has to be told apart from one
/// that states a count, and that difference is only ever a colour.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn painted_runs(output: &egui::FullOutput) -> Vec<(String, egui::Rect, egui::Color32)> {
    fn walk(shape: &egui::epaint::Shape, into: &mut Vec<(String, egui::Rect, egui::Color32)>) {
        match shape {
            egui::epaint::Shape::Text(painted) => into.push((
                painted.galley.job.text.clone(),
                egui::Rect::from_min_size(painted.pos, painted.galley.size()),
                painted.override_text_color.unwrap_or_else(|| {
                    painted
                        .galley
                        .job
                        .sections
                        .first()
                        .map_or(egui::Color32::PLACEHOLDER, |section| section.format.color)
                }),
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, into);
                }
            }
            _ => {}
        }
    }

    let mut runs = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut runs);
    }
    runs
}

/// Everything a painted frame carries, as text.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn painted_text(output: &egui::FullOutput) -> String {
    painted_runs(output)
        .into_iter()
        .fold(String::new(), |mut text, (run, _, _)| {
            text.push_str(&run);
            text.push('\n');
            text
        })
}

/// Where one painted run of a frame sits, for a gesture that has to land on
/// it.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn run_rect(
    runs: &[(String, egui::Rect, egui::Color32)],
    text: &str,
) -> Option<egui::Rect> {
    runs.iter()
        .find(|(run, _, _)| run == text)
        .map(|(_, rect, _)| *rect)
}

/// One glyph rasterized alone in the 15 px slot the shelf paints it in.
#[cfg(not(target_arch = "wasm32"))]
fn glyph_canvas(glyph: ShelfGlyph) -> crate::ui::raster::Canvas {
    crate::ui::raster::render(egui::vec2(24.0, 24.0), move |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                glyph.paint(
                    ui.painter(),
                    egui::Rect::from_center_size(egui::pos2(12.0, 12.0), egui::vec2(15.0, 15.0)),
                    egui::Color32::WHITE,
                );
            });
    })
}

// ---------------------------------------------------------- rails that fold

/// One interface pin, drawn as the typed port editor writes it.
pub(crate) fn port(
    id: u64,
    name: &str,
    netlist_order: Option<usize>,
    direction: PortDirection,
    documentation: &str,
) -> crate::state::Component {
    use crate::state::{PortContract, PortDiscipline, PortSignalType};

    let mut component = crate::state::Component::new(
        id,
        ComponentType::Port,
        crate::state::Point::new(20, 20 + id as i32),
    );
    component.value = name.to_owned();
    component.params = PortContract {
        direction,
        signal_type: PortSignalType::Analog,
        discipline: PortDiscipline::Electrical,
        netlist_order,
        documentation: documentation.to_owned(),
    }
    .encoded_params();
    component
}

/// A cell whose interface is declared out of document order: the pin the
/// contract puts second is the one drawn first.
pub(crate) fn interface_design() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let components = &mut app.state.model.schematic.document_mut_for_test().components;
    for (id, name, order, direction) in [
        (401, "BETA", 2, PortDirection::In),
        (402, "ALPHA", 1, PortDirection::In),
        (403, "GAMMA", 3, PortDirection::Out),
    ] {
        components.push(port(
            id,
            name,
            Some(order),
            direction,
            "carries the proving signal",
        ));
    }
    app.state.sync_active_schematic_to_workspace();
    app
}

/// A press and a release at one point, which is what egui reads as a click.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn click_events(at: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(at),
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        },
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        },
    ]
}

/// The navigator held open across frames, so a press and the frame that reads
/// it belong to one session.
///
/// A disclosure is only worth anything if it survives the frame it was set in,
/// and a case that wrote the persisted flag itself would prove nothing about
/// the band the reader actually presses.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct NavigatorPanel {
    pub(crate) ctx: egui::Context,
    pub(crate) app: RSpiceApp,
    /// The dock's height, which is what decides whether the rail scrolls.
    height: f32,
    /// Every control the frame announced: what it says, where it is, and the
    /// disclosure position it publishes.
    pub(crate) controls: Vec<(String, egui::Rect, Option<bool>)>,
    /// Every run of text the frame painted.
    pub(crate) runs: Vec<(String, egui::Rect, egui::Color32)>,
    /// What the keyboard is on, as the accessibility tree reports it.
    focus: Option<String>,
    /// Which traversal keys were still in the input once the panel had run:
    /// the whole of the panel's consumption contract, read from the outside.
    survived: Vec<egui::Key>,
}

/// The keys a rail may claim, and the one it must not.
#[cfg(not(target_arch = "wasm32"))]
const TRAVERSAL_KEYS: [egui::Key; 7] = [
    egui::Key::ArrowUp,
    egui::Key::ArrowDown,
    egui::Key::ArrowLeft,
    egui::Key::ArrowRight,
    egui::Key::Home,
    egui::Key::End,
    egui::Key::Escape,
];

/// One unmodified press.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn key_event(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

/// What one accessibility node says it is called.
///
/// `label` for a control and `value` for a `Label`-role node, because egui
/// files a label's text under the second of those and a reader asking only the
/// first would find the panel's plain rows nameless.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn announced_name(node: &egui::accesskit::Node) -> Option<String> {
    node.label().or_else(|| node.value()).map(str::to_owned)
}

#[cfg(not(target_arch = "wasm32"))]
impl NavigatorPanel {
    /// The panel at a height every rail fits in, so a case that is not about
    /// scrolling never has to think about it.
    pub(crate) fn open(app: RSpiceApp) -> Self {
        Self::open_within(app, 1600.0)
    }

    /// The panel in a dock of `height`, for the cases that are.
    pub(crate) fn open_within(app: RSpiceApp, height: f32) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut panel = Self {
            ctx,
            app,
            height,
            controls: Vec::new(),
            runs: Vec::new(),
            focus: None,
            survived: Vec::new(),
        };
        // Twice: the first pass builds the font set and the second lays out
        // against it, and a band measured before the fonts exist is not the
        // band the header ends up in.
        panel.pass(Vec::new());
        panel.pass(Vec::new());
        panel
    }

    /// One rendered pass, and what it published.
    pub(crate) fn pass(&mut self, events: Vec<egui::Event>) {
        let app = &mut self.app;
        let height = self.height;
        let mut survived = Vec::new();
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(260.0, height),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        ui.set_width(260.0);
                        navigator(ui, &mut app.design_cx());
                    });
                // Read once the panel has had the press: what is still in the
                // input is what the panel left for the canvas behind it.
                survived = TRAVERSAL_KEYS
                    .into_iter()
                    .filter(|key| {
                        ctx.ctx()
                            .input_mut(|input| input.consume_key(egui::Modifiers::NONE, *key))
                    })
                    .collect();
            },
        );
        self.survived = survived;
        self.runs = painted_runs(&output);
        let update = output.platform_output.accesskit_update;
        self.focus = update.as_ref().and_then(|update| {
            update
                .nodes
                .iter()
                .find(|(id, _)| *id == update.focus)
                .and_then(|(_, node)| announced_name(node))
        });
        self.controls = update
            .map(|update| {
                update
                    .nodes
                    .iter()
                    .filter_map(|(_, node)| {
                        let label = node.label()?.to_owned();
                        let bounds = node.bounds()?;
                        Some((
                            label,
                            egui::Rect::from_min_max(
                                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                            ),
                            node.is_expanded(),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();
    }

    /// One press, and the frame that settles what it moved.
    ///
    /// Two passes, because a fold moved from the keyboard is applied once the
    /// rail is painted — exactly as a press on the caret is — so the rows it
    /// discloses belong to the frame after. What the press left behind is
    /// carried across from the pass that carried the press.
    pub(crate) fn press(&mut self, key: egui::Key) {
        self.pass(vec![key_event(key)]);
        let survived = std::mem::take(&mut self.survived);
        self.pass(Vec::new());
        self.survived = survived;
    }

    /// Run the panel until whatever it set in motion has stopped moving.
    ///
    /// egui animates a scroll over the frames after the one that asked for it,
    /// so a case reading where a row ended up has to let those frames run.
    pub(crate) fn settle(&mut self) {
        for _ in 0..60 {
            self.pass(Vec::new());
        }
    }

    /// What the keyboard is on.
    pub(crate) fn focused(&self) -> Option<&str> {
        self.focus.as_deref()
    }

    /// Every run the last pass painted, one to a line.
    pub(crate) fn painted(&self) -> String {
        self.runs
            .iter()
            .map(|(run, _, _)| run.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Whether the panel left `key` for whatever stands behind it.
    pub(crate) fn survived(&self, key: egui::Key) -> bool {
        self.survived.contains(&key)
    }

    /// Put the keyboard in the filter field, through the route the Find
    /// command uses.
    pub(crate) fn focus_filter(&mut self) {
        self.app.state.workbench.outbox.push(FocusNavigatorSearch);
        self.pass(Vec::new());
    }

    /// Fold every rail this design draws.
    ///
    /// What is left is the panel's own bands, so the order a traversal has to
    /// produce is the order [`DESIGN_NAVIGATOR_SECTION_ORDER`] declares rather
    /// than whatever the fixture happens to place inside them.
    pub(crate) fn fold_every_section(&mut self) {
        for section in DESIGN_NAVIGATOR_SECTION_ORDER {
            let title = section.title();
            if self.band_if_present(title).is_some() {
                self.click(title);
            }
        }
    }

    /// The one band announcing `title`, or nothing when this design draws no
    /// such rail.
    ///
    /// Absence is an answer — the RF ports band is drawn only above a design
    /// that places a port — but a title announced *twice* never is, so that
    /// stays a failure either way.
    pub(crate) fn band_if_present(&self, title: &str) -> Option<(egui::Rect, Option<bool>)> {
        let hits = self
            .controls
            .iter()
            .filter(|(label, _, _)| label == title)
            .collect::<Vec<_>>();
        assert!(
            hits.len() <= 1,
            "expected at most one control announcing {title:?}, found {}; the rail announces: \
             {:#?}",
            hits.len(),
            self.controls
                .iter()
                .map(|(label, _, _)| label.as_str())
                .collect::<Vec<_>>()
        );
        hits.first().map(|(_, rect, expanded)| (*rect, *expanded))
    }

    /// The one band announcing `title`, which this design is expected to draw.
    pub(crate) fn band(&self, title: &str) -> (egui::Rect, Option<bool>) {
        self.band_if_present(title).unwrap_or_else(|| {
            panic!(
                "expected a control announcing {title:?}; the rail announces: {:#?}",
                self.controls
                    .iter()
                    .map(|(label, _, _)| label.as_str())
                    .collect::<Vec<_>>()
            )
        })
    }

    /// The disclosure position the band publishes to a screen reader.
    pub(crate) fn expanded(&self, title: &str) -> Option<bool> {
        self.band(title).1
    }

    /// Press the band, then settle the frame the press produced.
    pub(crate) fn click(&mut self, title: &str) {
        let at = self.band(title).0.center();
        self.pass(click_events(at));
        self.pass(Vec::new());
    }

    /// The count the section states, read from the band its own title is
    /// painted in rather than from the frame at large.
    pub(crate) fn stated_count(&self, title: &str) -> Option<String> {
        let heading = title.to_owned();
        let band = self
            .runs
            .iter()
            .find(|(run, _, _)| run.as_str() == heading)?
            .1;
        self.runs
            .iter()
            .find(|(run, rect, _)| {
                run.as_str() != heading && (rect.center().y - band.center().y).abs() <= 2.0
            })
            .map(|(run, _, _)| run.clone())
    }

    /// The accessible names of the controls `keep` admits, top to bottom.
    ///
    /// Ordered by where they were laid out rather than by the order the tree
    /// was published in: the accessibility update is a node set, and a rail's
    /// claim is about the order a reader meets its rows in.
    pub(crate) fn announced_rows(&self, keep: impl Fn(&str) -> bool) -> Vec<String> {
        let mut hits = self
            .controls
            .iter()
            .filter(|(label, _, _)| keep(label))
            .collect::<Vec<_>>();
        hits.sort_by(|(_, left, _), (_, right, _)| left.top().total_cmp(&right.top()));
        hits.into_iter()
            .map(|(label, _, _)| label.clone())
            .collect()
    }

    /// Every run one section painted under its own band.
    ///
    /// Read from the band down to the next one rather than from the frame at
    /// large, because the rails answer about one design and say the same words
    /// about it: a port named `ALPHA` puts `ALPHA` in the ports rail and in the
    /// nets rail both, and a search of the whole frame would call a folded
    /// section open on the strength of its neighbour.
    pub(crate) fn rows_under(&self, title: &str) -> Vec<String> {
        let band = self.band(title).0;
        let next = DESIGN_NAVIGATOR_SECTION_ORDER
            .iter()
            .map(|section| section.title())
            .filter(|other| *other != title)
            .filter_map(|other| self.band_if_present(other))
            .map(|(rect, _)| rect.top())
            .filter(|top| *top > band.top())
            .fold(f32::INFINITY, f32::min);
        self.runs
            .iter()
            .filter(|(_, rect, _)| rect.top() >= band.bottom() && rect.bottom() <= next)
            .map(|(run, _, _)| run.clone())
            .collect()
    }
}

/// The clear control's mark is painted geometry, not a character.
///
/// The mockup draws a `✕`. The control paints [`WorkbenchIcon::Close`]
/// instead, the crossing strokes every other dismissal in the workbench
/// paints, so the mark matches them whichever face holds the character.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_filter_clear_mark_paints_the_workbench_close_strokes() {
    let pixels = |canvas: &crate::ui::raster::Canvas| -> Vec<egui::Color32> {
        canvas
            .pixels_in(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(24.0, 24.0),
            ))
            .collect()
    };
    let tofu = pixels(&glyph_canvas(ShelfGlyph::Text("\u{2603}")));

    let mark = crate::ui::raster::render(egui::vec2(24.0, 24.0), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                WorkbenchIcon::Close.paint(
                    ui.painter(),
                    egui::Rect::from_center_size(egui::pos2(12.0, 12.0), egui::vec2(13.0, 13.0)),
                    egui::Color32::WHITE,
                );
            });
    });
    let background = mark.background();
    let painted = pixels(&mark);
    assert!(
        painted.iter().any(|pixel| *pixel != background),
        "the clear control paints no ink at all"
    );
    assert_ne!(
        painted, tofu,
        "and what it paints is not the replacement box"
    );
}
