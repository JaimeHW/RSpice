//! Printing a sheet's frame.
//!
//! The frame — border, zone band, references and marks — is laid out once by
//! the sheet model, which the canvas paints from too, so the plot prints
//! exactly the frame the canvas shows. Each piece goes out at the width the
//! standard gives it unless a print mapping entry for the frame layer says
//! otherwise.

use super::*;
use rspice_design_model::design_management::{
    DrawingSheetFrameLayer, DrawingSheetFrameMark, DrawingSheetGeometry,
};

/// The print-mapping layer every piece of the frame belongs to.
const FRAME_LAYER: &str = "layer:drawing-sheet-frame";

impl SemanticSceneCompiler<'_> {
    /// Print the frame artwork the schematic output asks for, and return the
    /// frame layer's stroke at the frame line's width for the legend.
    pub(super) fn drawing_sheet_frame(
        &mut self,
        format: &SchematicSheetFormat,
        geometry: &DrawingSheetGeometry,
    ) -> Result<StrokeStyle, HardcopyRenderError> {
        let output = self.schematic_output;
        let bordered = format.border != DrawingSheetBorderTemplate::None;
        let artwork = format.frame_artwork(geometry);
        for item in &artwork.items {
            let included = match item.layer {
                DrawingSheetFrameLayer::Border
                | DrawingSheetFrameLayer::CentringMarks
                | DrawingSheetFrameLayer::FoldingMarks
                | DrawingSheetFrameLayer::SizeDesignation => output.includes_border() && bordered,
                DrawingSheetFrameLayer::Zones | DrawingSheetFrameLayer::ZoneLabels => {
                    output.includes_zones()
                }
                DrawingSheetFrameLayer::RegistrationMarks | DrawingSheetFrameLayer::TrimMarks => {
                    output.crop_marks()
                }
            };
            if included {
                self.drawing_sheet_frame_mark(&item.mark)?;
            }
        }
        Ok(self.frame_stroke(format.frame_convention().frame_line_um()))
    }

    fn frame_stroke(&self, width_um: u64) -> StrokeStyle {
        self.mapped_stroke(
            PrintObjectKind::Layer,
            FRAME_LAYER,
            StrokeStyle {
                color: SemanticColor::Foreground,
                width: Length::from_micrometres(width_um),
                pattern: StrokePattern::Solid,
                series_index: None,
                exact_dash: None,
                exact_dot_spacing: None,
            },
        )
    }

    fn drawing_sheet_frame_mark(
        &mut self,
        mark: &DrawingSheetFrameMark,
    ) -> Result<(), HardcopyRenderError> {
        let ink = self.frame_stroke(1).color;
        match mark {
            DrawingSheetFrameMark::Line { from, to, width_um } => {
                self.primitives.push(ScenePrimitive::Line {
                    from: self.drawing_sheet_point(from.0, from.1)?,
                    to: self.drawing_sheet_point(to.0, to.1)?,
                    stroke: self.frame_stroke(*width_um),
                });
            }
            DrawingSheetFrameMark::Rect { rect, width_um } => {
                self.primitives.push(ScenePrimitive::Rect {
                    rect: self.drawing_sheet_rect(*rect)?,
                    stroke: Some(self.frame_stroke(*width_um)),
                    fill: None,
                });
            }
            DrawingSheetFrameMark::FilledRect { rect } => {
                self.primitives.push(ScenePrimitive::Rect {
                    rect: self.drawing_sheet_rect(*rect)?,
                    stroke: None,
                    fill: Some(SceneFill::solid(ink)),
                });
            }
            DrawingSheetFrameMark::FilledPolygon { points } => {
                let points = points
                    .iter()
                    .map(|point| self.drawing_sheet_point(point.0, point.1))
                    .collect::<Result<Vec<_>, _>>()?;
                self.primitives.push(ScenePrimitive::Polyline {
                    points,
                    closed: true,
                    stroke: self.frame_stroke(1),
                    fill: Some(SceneFill::solid(ink)),
                });
            }
            DrawingSheetFrameMark::Text {
                centre,
                text,
                lettering_um,
            } => {
                // Lettered at the canvas's height and centred where the
                // canvas centres it, the baseline converted from the line's
                // centre.
                let baseline_um = drawing_sheet_mono_baseline_offset_um(*lettering_um) as i64;
                self.add_text_placed(
                    self.drawing_sheet_point(centre.0, centre.1 + baseline_um)?,
                    text,
                    SceneFont::Monospace,
                    drawing_sheet_mono_em_um(*lettering_um),
                    ink,
                    (TextAnchor::Middle, SceneTextRotation::Upright),
                )?;
            }
        }
        Ok(())
    }
}
