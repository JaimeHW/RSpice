//! Where a design-check location is drawn, resolved from the model alone.
//!
//! The canvas, the console and the published rule findings all send the
//! author to one place for one location, so the resolver reads only what the
//! model persists and the review markers the drawing shows.

use rspice_design::drc::DrcLocation;
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;
use rspice_schematic_editor::view::design_view::DesignView;
use rspice_schematic_editor::view::violations as markers;

use crate::diagnostics::LogAnchor;
use crate::state::CellViewRef;
use crate::workbench::app_state::model::Model;

/// The drawing in front of the reader, from the model and the review markers
/// it shows.
pub fn model_design_view(
    model: &Model,
    review_markers: SchematicReviewMarkerVisibility,
) -> DesignView<'_> {
    let key = model.workspace.content.active_schematic_reference().key();
    DesignView {
        document: model.schematic.document(),
        canvas_cache: model.schematic.canvas_cache(),
        sheet_catalog: model
            .workspace
            .content
            .design_management
            .sheet_catalog(&key),
        review_markers,
    }
}

/// Where one finding location in `cell_view` points, when it points anywhere.
///
/// The drawing in front of the reader is asked through its live editor, and
/// any other cell view through the working copy the workspace holds for it.
pub fn model_location_anchor(
    model: &Model,
    review_markers: SchematicReviewMarkerVisibility,
    cell_view: &CellViewRef,
    location: &DrcLocation,
) -> Option<LogAnchor> {
    if let DrcLocation::SymbolPin {
        reference,
        pin_name,
        point,
    } = location
    {
        return Some(LogAnchor::Symbol {
            reference: reference.clone(),
            pin_name: pin_name.clone(),
            point: *point,
        });
    }

    let key = cell_view.key();
    let placed =
        if key.eq_ignore_ascii_case(&model.workspace.content.active_schematic_reference().key()) {
            markers::placement(&model_design_view(model, review_markers), location)?
        } else {
            let (_, schematic) = model
                .workspace
                .content
                .schematic_buffers
                .iter()
                .find(|(buffer, _)| buffer.eq_ignore_ascii_case(&key))?;
            let view = DesignView {
                document: schematic.document(),
                canvas_cache: None,
                sheet_catalog: model
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog(&key),
                review_markers,
            };
            markers::placement(&view, location)?
        };
    let (component, wire) = match location {
        DrcLocation::Component { id, .. } => (Some(*id), None),
        DrcLocation::Wire { id } => (None, Some(*id)),
        _ => (None, None),
    };
    Some(LogAnchor::Schematic {
        cell_view: cell_view.clone(),
        x: placed.point.x,
        y: placed.point.y,
        component,
        wire,
        sheet: placed.sheet,
    })
}
