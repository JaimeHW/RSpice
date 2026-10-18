//! Canonical title-block projection into hardcopy scene primitives.
//!
//! Managed logos and field layout are resolved here so every publication
//! backend consumes identical geometry. The block always stands upright.

use super::*;

impl SemanticSceneCompiler<'_> {
    pub(super) fn drawing_sheet_title_block(
        &mut self,
        format: &SchematicSheetFormat,
        template: DrawingSheetTitleBlockTemplate,
        automatic_values: &BTreeMap<DrawingSheetTitleFieldId, String>,
        block: DrawingSheetRect,
        stroke: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        self.primitives.push(ScenePrimitive::Rect {
            rect: self.drawing_sheet_rect(block)?,
            stroke: Some(stroke),
            fill: None,
        });
        // The outline is drawn at `stroke`; each rule inside it, and a
        // managed logo's artwork, keeps its standard's proportion to the
        // outline, whatever width a print mapping gives the outline.
        let outline_um = template.outline_um();
        let rule_of = |width_um: u64| StrokeStyle {
            width: Length::from_micrometres(stroke.width.micrometres() * width_um / outline_um),
            ..stroke
        };
        let rule = rule_of(rspice_design_model::design_management::DRAWING_SHEET_TITLE_RULE_UM);
        // The form the canvas and Page Setup draw too: every rule and every
        // cell's width come from the one layout, so a field is cut short on
        // paper exactly where it is on screen.
        let geometry = format
            .geometry()
            .map_err(|error| conversion_error(format!("drawing-sheet geometry: {error}")))?;
        let layout =
            rspice_design_model::design_management::drawing_sheet_title_layout(format, &geometry)
                .ok_or_else(|| conversion_error("title block has no authored grid"))?;
        let at_x = |x_um: u64| {
            i64::try_from(x_um)
                .ok()
                .and_then(|x_um| block.x_um.checked_add(x_um))
                .ok_or_else(|| conversion_error("drawing-sheet title-block X geometry overflow"))
        };
        let at_y = |y_um: u64| {
            i64::try_from(y_um)
                .ok()
                .and_then(|y_um| block.y_um.checked_add(y_um))
                .ok_or_else(|| conversion_error("drawing-sheet title-block Y geometry overflow"))
        };
        let logo = format.title_block_logo(template);
        let logo_reserved_width_um = layout.grid_x_um;
        let field_grid_x = at_x(layout.grid_x_um)?;
        if let Some(logo) = logo {
            self.primitives.push(ScenePrimitive::Line {
                from: self.drawing_sheet_point(field_grid_x, block.y_um)?,
                to: self.drawing_sheet_point(field_grid_x, block.y_um + block.height_um as i64)?,
                stroke: rule,
            });
            let inset = 3_000u64
                .min(logo_reserved_width_um / 10)
                .min(block.height_um / 10);
            let logo_width = logo_reserved_width_um.saturating_sub(inset.saturating_mul(2));
            let logo_height = block.height_um.saturating_sub(inset.saturating_mul(2));
            let basis = u64::from(
                rspice_design_model::design_management::DRAWING_SHEET_MANAGED_LOGO_COORDINATE_BASIS,
            );
            for primitive in logo.primitives() {
                let points = primitive
                    .points()
                    .iter()
                    .map(|point| {
                        let x_offset =
                            inset + logo_width.saturating_mul(u64::from(point.x())) / basis;
                        let y_offset =
                            inset + logo_height.saturating_mul(u64::from(point.y())) / basis;
                        self.drawing_sheet_point(
                            block.x_um + x_offset as i64,
                            block.y_um + y_offset as i64,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                self.primitives.push(ScenePrimitive::Polyline {
                    points,
                    closed: primitive.closed(),
                    stroke: rule,
                    fill: primitive.filled().then_some(SceneFill::solid(stroke.color)),
                });
            }
        }
        let fields = rspice_design_model::design_management::resolve_drawing_sheet_title_fields(
            format,
            automatic_values,
        );
        // A block with no field to show is plotted as its outline alone, as
        // the canvas draws it, rather than with fields the sheet does not
        // carry.
        if fields.is_empty() {
            return Ok(());
        }
        // Each cell's own rules, each drawn once at its own width.
        for cell_rule in &layout.rules {
            self.primitives.push(ScenePrimitive::Line {
                from: self
                    .drawing_sheet_point(at_x(cell_rule.from_x_um)?, at_y(cell_rule.from_y_um)?)?,
                to: self.drawing_sheet_point(at_x(cell_rule.to_x_um)?, at_y(cell_rule.to_y_um)?)?,
                stroke: rule_of(cell_rule.width_um),
            });
        }
        // Every line sits on the seat the canvas also places it on. The
        // canvas centres a line on that point while publication places a
        // baseline on it, so each is converted to a baseline here and a
        // field's ink lands in the same place on screen and on paper. Each is
        // set at the em whose capitals stand at its ISO 3098 height, as the
        // canvas sets it. A caption is the form's own wording and nothing
        // else: ISO 7200 marks no field as filled in by the system, and a
        // reader of the print has no use for knowing. An empty value plots
        // nothing: the cell is a box to fill in.
        for line in layout.lettering(&fields) {
            self.add_text_rotated(
                self.drawing_sheet_point(
                    at_x(line.x_um)?,
                    at_y(line.seat_um + drawing_sheet_mono_baseline_offset_um(line.lettering_um))?,
                )?,
                &line.text,
                if line.emphasised {
                    SceneFont::MonospaceMedium
                } else {
                    SceneFont::Monospace
                },
                drawing_sheet_mono_em_um(line.lettering_um),
                stroke.color,
                SceneTextRotation::Upright,
            )?;
        }
        Ok(())
    }
}
