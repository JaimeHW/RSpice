//! Where an imported drawing lies on its sheet.
//!
//! The source program's coordinates run anywhere, so the drawing is moved,
//! whole and by whole grid steps, into the middle of the sheet's drawing
//! area clear of the title block. The project's sheet holds it when it fits
//! there; otherwise the drawing gets its own sheet, the smallest standard size
//! of the project sheet's series that holds it.

use rspice_design::schematic::document::SchematicDocument;
use rspice_design::symbol_resolver::SymbolResolver;
use rspice_design_model::design_management::{
    AuthoredDrawingSheetSize, DrawingSheetStandard, DrawingSheetStandardSeries,
    SchematicSheetFormat,
};
use rspice_design_model::{Point, SchematicPageOrientation};
use rspice_schematic_editor::view::drawing_sheet::{DrawingSheetGeometry, WorldRect};
use rspice_schematic_editor::view::symbol_context::SchematicSymbolContext;

use crate::workbench::app_state::Model;

/// The clearance between the drawing and the edge of the area it is placed
/// in, in world units: 10 mm.
const CLEARANCE: f64 = 40.0;

/// A sheet the drawing was given in place of the project's, with every
/// object the sheet holds.
pub(super) struct OwnSheet {
    pub format: SchematicSheetFormat,
    pub objects: Vec<u64>,
}

/// Moves `document` onto the project's sheet, or onto the smallest larger
/// standard sheet that holds it, and returns that sheet when it is not the
/// project's. A drawing too large for every standard sheet goes on the
/// largest in the project's orientation, its top left corner in the drawing
/// area's.
pub(super) fn place_on_sheet(model: &Model, document: &mut SchematicDocument) -> Option<OwnSheet> {
    let resolver = SymbolResolver::new(
        model.library_manager.catalog(),
        &model.workspace.content.schematic_buffers,
    );
    let (min_x, min_y, max_x, max_y) =
        SchematicSymbolContext::new(document, None, &resolver, 0).content_bounds(document)?;
    let drawing = WorldRect {
        min_x: f64::from(min_x),
        min_y: f64::from(min_y),
        max_x: f64::from(max_x),
        max_y: f64::from(max_y),
    };
    let project = &model
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .default_format;
    let candidates = std::iter::once(project.clone()).chain(larger_sheets(project));
    let mut largest = None;
    for (index, format) in candidates.enumerate() {
        let geometry = DrawingSheetGeometry::from_format(&format);
        if let Some(area) = holding_area(&geometry, drawing) {
            move_drawing(document, centring_step(drawing, area, document.grid_size));
            return (index > 0).then(|| OwnSheet {
                format,
                objects: object_ids(document),
            });
        }
        if format.orientation == project.orientation {
            largest = Some((format, geometry.drawing_area));
        }
    }
    let (format, area) = largest?;
    let corner = WorldRect {
        min_x: area.min_x + CLEARANCE,
        min_y: area.min_y + CLEARANCE,
        max_x: area.min_x + CLEARANCE + (drawing.max_x - drawing.min_x),
        max_y: area.min_y + CLEARANCE + (drawing.max_y - drawing.min_y),
    };
    move_drawing(document, centring_step(drawing, corner, document.grid_size));
    (format != *project).then(|| OwnSheet {
        format,
        objects: object_ids(document),
    })
}

/// The standard sheets of `project`'s series larger than it, smallest
/// first, each in the project's orientation and then the other.
fn larger_sheets(project: &SchematicSheetFormat) -> impl Iterator<Item = SchematicSheetFormat> {
    let series = match &project.authored_size {
        AuthoredDrawingSheetSize::Standard { standard } => standard.series(),
        AuthoredDrawingSheetSize::Custom { snapshot } => snapshot
            .started_from
            .unwrap_or(DrawingSheetStandardSeries::Iso),
    };
    let area = |(width, height): (u64, u64)| u128::from(width) * u128::from(height);
    let project_area = area(project.portrait_dimensions_um());
    let mut standards = DrawingSheetStandard::ALL
        .into_iter()
        .filter(|standard| standard.series() == series)
        .filter(|standard| area(standard.portrait_dimensions_um()) > project_area)
        .collect::<Vec<_>>();
    standards.sort_by_key(|standard| area(standard.portrait_dimensions_um()));
    let orientations = match project.orientation {
        SchematicPageOrientation::Landscape => [
            SchematicPageOrientation::Landscape,
            SchematicPageOrientation::Portrait,
        ],
        SchematicPageOrientation::Portrait => [
            SchematicPageOrientation::Portrait,
            SchematicPageOrientation::Landscape,
        ],
    };
    let project = project.clone();
    standards.into_iter().flat_map(move |standard| {
        let project = project.clone();
        orientations.into_iter().map(move |orientation| {
            let mut format = SchematicSheetFormat::from_standard(standard, orientation);
            format.display_unit = project.display_unit;
            format
        })
    })
}

/// The part of the sheet's drawing area clear of its title block that holds
/// `drawing` with the clearance all round: the band above the block, or the
/// one beside it, whichever leaves the drawing more room.
fn holding_area(geometry: &DrawingSheetGeometry, drawing: WorldRect) -> Option<WorldRect> {
    let area = geometry.drawing_area;
    let regions = match geometry.title_block {
        None => vec![area],
        Some(block) => vec![
            WorldRect {
                max_y: block.min_y,
                ..area
            },
            WorldRect {
                max_x: block.min_x,
                ..area
            },
        ],
    };
    let width = drawing.max_x - drawing.min_x + 2.0 * CLEARANCE;
    let height = drawing.max_y - drawing.min_y + 2.0 * CLEARANCE;
    regions
        .into_iter()
        .filter(|region| {
            region.max_x - region.min_x >= width && region.max_y - region.min_y >= height
        })
        .max_by(|a, b| {
            let room = |region: &WorldRect| {
                ((region.max_x - region.min_x) - width).min((region.max_y - region.min_y) - height)
            };
            room(a).total_cmp(&room(b))
        })
}

/// The whole-grid step that brings the middle of `drawing` to the middle of
/// `area`.
fn centring_step(drawing: WorldRect, area: WorldRect, grid: i32) -> Point {
    let grid = f64::from(grid.max(1));
    let step = |from_min: f64, from_max: f64, to_min: f64, to_max: f64| {
        let shift = (to_min + to_max - from_min - from_max) / 2.0;
        // A step is at most the sheet's size, far inside `i32`.
        ((shift / grid).round() * grid) as i32
    };
    Point::new(
        step(drawing.min_x, drawing.max_x, area.min_x, area.max_x),
        step(drawing.min_y, drawing.max_y, area.min_y, area.max_y),
    )
}

/// Moves every object of `document` by `delta`. Connections name objects,
/// not places, so they hold as they are.
fn move_drawing(document: &mut SchematicDocument, delta: Point) {
    let shift = |point: &mut Point| {
        point.x = point.x.saturating_add(delta.x);
        point.y = point.y.saturating_add(delta.y);
    };
    for component in &mut document.components {
        shift(&mut component.pos);
    }
    for wire in &mut document.wires {
        wire.translate(delta.x, delta.y);
    }
    for bus in &mut document.buses {
        bus.translate(delta);
    }
    for tap in &mut document.bus_taps {
        tap.translate(delta);
    }
    for note in &mut document.design_notes {
        note.translate(delta);
    }
    for shape in &mut document.documentation_shapes {
        shape.translate(delta);
    }
    for probe in &mut document.probes {
        shift(&mut probe.position);
    }
    for label in &mut document.net_labels {
        shift(&mut label.pos);
    }
    for junction in &mut document.junctions {
        shift(&mut junction.pos);
    }
}

/// Every object a sheet of `document` holds.
fn object_ids(document: &SchematicDocument) -> Vec<u64> {
    let mut ids = std::collections::BTreeSet::new();
    ids.extend(document.components.iter().map(|object| object.id));
    ids.extend(document.wires.iter().map(|object| object.id));
    ids.extend(document.buses.iter().map(|object| object.id));
    ids.extend(document.bus_taps.iter().map(|object| object.id));
    ids.extend(document.junctions.iter().map(|object| object.id));
    ids.extend(document.net_labels.iter().map(|object| object.id));
    ids.extend(document.design_notes.iter().map(|object| object.id));
    ids.extend(document.documentation_shapes.iter().map(|object| object.id));
    ids.extend(document.probes.iter().map(|object| object.id));
    ids.into_iter().collect()
}
