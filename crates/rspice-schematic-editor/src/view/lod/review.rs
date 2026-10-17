//! Review renders of the canvas level of detail: the same small circuit at
//! the zooms the design's proof table names, dark and light. `#[ignore]`d:
//! the PNGs are for a person to look at and measure.

use egui::{Pos2, Rect, Vec2};
use rspice_design::schematic::{
    bus::Bus, component::Component, component_display::SchematicParameterLabelVisibility,
    component_type::ComponentType, net_label::NetLabel, probe::SchematicProbe,
};
use rspice_design_model::{Point, bus::BusDeclaration};

use crate::symbols::SymbolLibrary;
use crate::view::{
    drawing::{
        ProbeVisualStatus, draw_bus, draw_component, draw_conductor, draw_junction, draw_probe,
    },
    lod::Emphasis,
    net_labels::draw_net_label,
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};

pub(crate) const ZOOMS: [f32; 6] = [0.25, 0.32, 0.5, 1.0, 2.0, 4.0];
pub(crate) const CELL: Vec2 = Vec2::new(230.0, 300.0);
/// The world point each cell centres on: the whole circuit while it fits,
/// the T-junction and its label once zoomed in.
fn focus(zoom: f32) -> Point {
    if zoom < 2.0 {
        Point::new(30, 80)
    } else {
        Point::new(-5, 10)
    }
}

pub(crate) fn cell_viewport(index: usize, zoom: f32) -> Viewport {
    let bounds = Rect::from_min_size(Pos2::new(index as f32 * CELL.x, 0.0), CELL);
    let focus = focus(zoom);
    Viewport {
        offset: Pos2::new(
            CELL.x * 0.5 - focus.x as f32 * zoom,
            CELL.y * 0.5 - focus.y as f32 * zoom,
        ),
        zoom,
        bounds,
    }
}

/// The circuit: a T-junction at the origin, an unmarked crossing at x = 60,
/// a net label, a resistor, a bus and a probe.
pub(crate) fn paint_circuit(painter: &egui::Painter, viewport: &Viewport) {
    let library = SymbolLibrary::load_embedded().expect("symbol library");
    let context = SchematicSymbolContext::default();
    let through = [Point::new(-60, 0), Point::new(120, 0)];
    let stub = [Point::new(0, 0), Point::new(0, 60)];
    let crossing = [Point::new(60, -40), Point::new(60, 60)];
    let selected = [Point::new(-60, 100), Point::new(120, 100)];
    let selected_stub = [Point::new(90, 100), Point::new(90, 150)];
    draw_conductor(painter, viewport, &through, false, None);
    draw_conductor(painter, viewport, &stub, false, None);
    draw_conductor(painter, viewport, &crossing, false, None);
    draw_conductor(painter, viewport, &selected, true, None);
    draw_conductor(painter, viewport, &selected_stub, true, None);
    draw_junction(
        painter,
        viewport,
        Point::new(0, 0),
        false,
        false,
        Emphasis::Plain,
    );
    draw_junction(
        painter,
        viewport,
        Point::new(90, 100),
        false,
        false,
        Emphasis::Raised,
    );

    let mut resistor = Component::new(1, ComponentType::Resistor, Point::new(0, 100));
    resistor.name = "R1".to_owned();
    resistor.value = "10k".to_owned();
    draw_component(
        painter,
        viewport,
        &resistor,
        false,
        Some(&library),
        &context,
        SchematicParameterLabelVisibility::NamesAndValues,
    );
    let label = NetLabel::new(2, Point::new(-40, 0), "VOUT");
    draw_net_label(painter, viewport, &label, false, false, false);
    let bus = Bus::segment(
        3,
        Point::new(-60, 200),
        Point::new(120, 200),
        Some(BusDeclaration::parse("D[7:0]").expect("declaration")),
    )
    .expect("bus");
    draw_bus(painter, viewport, &bus, false);
    let probe = SchematicProbe::new(4, Point::new(60, 60), "V(OUT)", None).expect("probe");
    draw_probe(
        painter,
        viewport,
        &probe,
        ProbeVisualStatus::Materialized,
        false,
        false,
    );
}

pub(crate) fn write_png(name: &str, canvas: &rspice_ui_kit::raster::Canvas, height: usize) {
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let path = directory.join(format!("{name}.png"));
    std::fs::write(&path, canvas.png(height)).expect("write review png");
    eprintln!("wrote {}", path.display());
}

#[test]
#[ignore = "writes review PNGs to RSPICE_RASTER_DIR"]
fn render_level_of_detail_review() {
    let size = Vec2::new(CELL.x * ZOOMS.len() as f32, CELL.y);
    for (mode, theme) in themes() {
        let canvas = rspice_ui_kit::raster::render_themed(theme, size, |ui, _| {
            let palette = rspice_ui_kit::tokens::active_palette();
            for (index, zoom) in ZOOMS.into_iter().enumerate() {
                let viewport = cell_viewport(index, zoom);
                let painter = ui.painter().with_clip_rect(viewport.bounds.shrink(1.0));
                painter.rect_filled(viewport.bounds.shrink(1.0), 0.0, palette.canvas_bg);
                paint_circuit(&painter, &viewport);
                ui.painter().text(
                    viewport.bounds.left_top() + Vec2::new(4.0, 4.0),
                    egui::Align2::LEFT_TOP,
                    format!("z {zoom}"),
                    egui::FontId::monospace(10.0),
                    palette.text_dim,
                );
            }
        });
        write_png(
            &format!("lod-{mode}-{}", review_stage()),
            &canvas,
            CELL.y as usize,
        );
    }
}

/// The canvas in its dark and its light palette.
pub(crate) fn themes() -> [(&'static str, rspice_ui_kit::Theme); 2] {
    [
        ("dark", rspice_ui_kit::Theme::default()),
        (
            "light",
            rspice_ui_kit::Theme {
                mode: rspice_ui_kit::Mode::Light,
                canvas_theme: rspice_ui_kit::EngineeringCanvasTheme::Light,
                ..rspice_ui_kit::Theme::default()
            },
        ),
    ]
}

/// `before` or `after`, from `RSPICE_REVIEW_STAGE`, so one test writes both
/// sides of a change without overwriting the other.
pub(crate) fn review_stage() -> String {
    std::env::var("RSPICE_REVIEW_STAGE").unwrap_or_else(|_| "now".to_owned())
}
