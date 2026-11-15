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
//! a source through — are in [`rails`]. The model fixtures and the
//! frame-reading helpers they share with these — [`painted_runs`],
//! [`run_rect`], [`glyph_canvas`] — stay here, because a harness that reads
//! one painted frame belongs to no single tab. The designs built on a whole
//! application, and the panel harness that holds one, are with the cases that
//! drive a whole application.

#[cfg(test)]
use super::shelf::*;
use super::*;
use crate::state::{CellViewRef, ComponentType, SchematicState};

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod object_rows;
pub mod shelf;
#[cfg(test)]
mod shelf_search;

/// Give the project one more schematic master: the library cell view, and the
/// content the projection will materialize for it.
pub fn add_master(model: &mut Model, library: &str, cell: &str, schematic: SchematicState) {
    use crate::state::{Library, View, ViewType};

    if model.library_manager.get_library(library).is_none() {
        model.library_manager.add_library(Library::new(library));
    }
    let owner = model
        .library_manager
        .get_library_mut(library)
        .expect("the fixture library exists");
    let target = owner.get_or_create_cell(cell);
    if target.get_view("schematic").is_none() {
        target.add_view(View::new("schematic", ViewType::Schematic));
    }
    model.workspace.insert_schematic_editor(
        CellViewRef::new(library, cell, "schematic").key(),
        schematic,
    );
}

/// One placed hierarchical instance, under the name the design gives it.
pub fn placed(id: u64, name: &str, library: &str, cell: &str) -> crate::state::Component {
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

/// Everything one navigator surface paints, at the dock's real width.
#[cfg(not(target_arch = "wasm32"))]
pub fn painted_panel(ctx: &egui::Context, mut contents: impl FnMut(&mut Ui)) -> String {
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
pub fn painted_runs(output: &egui::FullOutput) -> Vec<(String, egui::Rect, egui::Color32)> {
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
pub fn painted_text(output: &egui::FullOutput) -> String {
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
pub fn run_rect(runs: &[(String, egui::Rect, egui::Color32)], text: &str) -> Option<egui::Rect> {
    runs.iter()
        .find(|(run, _, _)| run == text)
        .map(|(_, rect, _)| *rect)
}

/// One glyph rasterized alone in the 15 px slot the shelf paints it in.
#[cfg(test)]
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
pub fn port(
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

/// A press and a release at one point, which is what egui reads as a click.
#[cfg(not(target_arch = "wasm32"))]
pub fn click_events(at: egui::Pos2) -> Vec<egui::Event> {
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

/// One unmodified press.
#[cfg(not(target_arch = "wasm32"))]
pub fn key_event(key: egui::Key) -> egui::Event {
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
pub fn announced_name(node: &egui::accesskit::Node) -> Option<String> {
    node.label().or_else(|| node.value()).map(str::to_owned)
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
