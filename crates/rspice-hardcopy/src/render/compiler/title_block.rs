//! Canonical title-block projection into hardcopy scene primitives.
//!
//! Rotation, clipping, managed logos, and field layout are resolved here so
//! every publication backend consumes identical geometry.

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
        let rotation = format.title_block.rotation;
        let authored_block = authored_title_block_rect(block, rotation)?;
        let transform = |this: &Self, x: i64, y: i64| {
            this.drawing_sheet_title_block_point(x, y, authored_block, rotation)
        };
        let text_rotation = match rotation {
            DrawingSheetTitleBlockRotation::Upright => SceneTextRotation::Upright,
            DrawingSheetTitleBlockRotation::Clockwise90 => SceneTextRotation::Clockwise90,
            DrawingSheetTitleBlockRotation::CounterClockwise90 => {
                SceneTextRotation::CounterClockwise90
            }
        };
        let logo = format.title_block_logo(template);
        let logo_reserved_width_um = logo.map_or(0, |logo| logo.reserved_width_um());
        let field_grid_x = authored_block
            .x_um
            .checked_add(i64::try_from(logo_reserved_width_um).map_err(|_| {
                conversion_error("drawing-sheet managed logo width exceeds coordinate range")
            })?)
            .ok_or_else(|| conversion_error("drawing-sheet managed logo position overflow"))?;
        let field_grid_width = authored_block
            .width_um
            .checked_sub(logo_reserved_width_um)
            .ok_or_else(|| conversion_error("drawing-sheet managed logo exceeds title block"))?;
        if let Some(logo) = logo {
            self.primitives.push(ScenePrimitive::Line {
                from: transform(self, field_grid_x, authored_block.y_um)?,
                to: transform(
                    self,
                    field_grid_x,
                    authored_block.y_um + authored_block.height_um as i64,
                )?,
                stroke,
            });
            let inset = 3_000u64
                .min(logo_reserved_width_um / 10)
                .min(authored_block.height_um / 10);
            let logo_width = logo_reserved_width_um.saturating_sub(inset.saturating_mul(2));
            let logo_height = authored_block
                .height_um
                .saturating_sub(inset.saturating_mul(2));
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
                        transform(
                            self,
                            authored_block.x_um + x_offset as i64,
                            authored_block.y_um + y_offset as i64,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                self.primitives.push(ScenePrimitive::Polyline {
                    points,
                    closed: primitive.closed(),
                    stroke,
                    fill: primitive.filled().then_some(SceneFill::solid(stroke.color)),
                });
            }
        }
        let rows = format
            .title_block_rows(template)
            .map(|rows| rows as u64)
            .ok_or_else(|| conversion_error("title block has no authored grid"))?;
        let fields = rspice_design_model::design_management::resolve_drawing_sheet_title_fields(format, automatic_values)
            .into_iter()
            .map(|field| {
                (
                    field.id.display_label().to_owned(),
                    field.value,
                    field.authority
                        == rspice_design_model::design_management::DrawingSheetTitleFieldValueAuthority::Automatic,
                )
            })
            .collect::<Vec<_>>();
        // A block with no field to show is plotted as its outline alone, as
        // the canvas draws it, rather than with fields the sheet does not
        // carry.
        if fields.is_empty() {
            return Ok(());
        }
        let visible_field_count = fields.len();
        let columns = fields.len().div_ceil(rows as usize).max(1) as u64;
        let row_height = authored_block.height_um / rows;
        let cell_width = field_grid_width / columns;
        for row in 1..rows {
            let y = authored_block.y_um + (authored_block.height_um * row / rows) as i64;
            self.primitives.push(ScenePrimitive::Line {
                from: transform(self, field_grid_x, y)?,
                to: transform(self, field_grid_x + field_grid_width as i64, y)?,
                stroke,
            });
        }
        for index in 0..fields.len() {
            let row = index as u64 / columns;
            let column = index as u64 % columns;
            if column > 0 {
                let x = field_grid_x + (cell_width * column) as i64;
                let top = authored_block.y_um + (row_height * row) as i64;
                self.primitives.push(ScenePrimitive::Line {
                    from: transform(self, x, top)?,
                    to: transform(self, x, top + row_height as i64)?,
                    stroke,
                });
            }
        }
        // Every cell in the grid is the same width, so the budget and the
        // lettering are the block's, not each field's.
        let geometry = format
            .geometry()
            .map_err(|error| conversion_error(format!("drawing-sheet geometry: {error}")))?;
        let capacity = |lettering_um| {
            rspice_design_model::design_management::drawing_sheet_title_cell_capacity(
                format,
                &geometry,
                visible_field_count,
                lettering_um,
            )
            .ok_or_else(|| conversion_error("title block has no authored cell capacity"))
        };
        let lettering = rspice_design_model::design_management::DRAWING_SHEET_TITLE_BLOCK_LETTERING;
        let value_chars = capacity(lettering.data_um)?;
        let caption_chars = capacity(lettering.caption_um)?;
        for (index, (label, value, automatic)) in fields.into_iter().enumerate() {
            let row = index as u64 / columns;
            let column = index as u64 % columns;
            let label = if automatic {
                format!("• {label}")
            } else {
                label
            };
            // Caption and value sit on the shared fractions of the cell that
            // the canvas also places them on. The canvas centres a line on
            // that point while publication places a baseline on it, so each
            // is converted to a baseline here and a field's ink lands in the
            // same place on screen and on paper. Each is set at the em whose
            // capitals stand at its ISO 3098 height, as the canvas sets it.
            let at = |(numerator, denominator): (u64, u64)| {
                (row_height * numerator / denominator) as i64
            };
            let text_x = field_grid_x
                + (cell_width * column) as i64
                + DRAWING_SHEET_TITLE_CELL_PADDING_UM as i64;
            let row_y = authored_block.y_um + (row_height * row) as i64;
            self.add_text_rotated(
                transform(
                    self,
                    text_x,
                    row_y
                        + at(DRAWING_SHEET_TITLE_CAPTION_POSITION)
                        + drawing_sheet_mono_baseline_offset_um(lettering.caption_um) as i64,
                )?,
                &truncate_title_block_text(&label, caption_chars),
                SceneFont::Monospace,
                drawing_sheet_mono_em_um(lettering.caption_um),
                stroke.color,
                text_rotation,
            )?;
            self.add_text_rotated(
                transform(
                    self,
                    text_x,
                    row_y
                        + at(DRAWING_SHEET_TITLE_VALUE_POSITION)
                        + drawing_sheet_mono_baseline_offset_um(lettering.data_um) as i64,
                )?,
                &truncate_title_block_text(&value, value_chars),
                SceneFont::MonospaceMedium,
                drawing_sheet_mono_em_um(lettering.data_um),
                stroke.color,
                text_rotation,
            )?;
        }
        Ok(())
    }
}

pub(in crate::render) fn authored_title_block_rect(
    block: DrawingSheetRect,
    rotation: DrawingSheetTitleBlockRotation,
) -> Result<DrawingSheetRect, HardcopyRenderError> {
    if rotation == DrawingSheetTitleBlockRotation::Upright {
        return Ok(block);
    }
    let block_width = i64::try_from(block.width_um)
        .map_err(|_| conversion_error("drawing-sheet title-block width overflow"))?;
    let block_height = i64::try_from(block.height_um)
        .map_err(|_| conversion_error("drawing-sheet title-block height overflow"))?;
    let x_twice = block
        .x_um
        .checked_mul(2)
        .and_then(|value| value.checked_add(block_width))
        .and_then(|value| value.checked_sub(block_height))
        .ok_or_else(|| conversion_error("drawing-sheet title-block X geometry overflow"))?;
    let y_twice = block
        .y_um
        .checked_mul(2)
        .and_then(|value| value.checked_add(block_height))
        .and_then(|value| value.checked_sub(block_width))
        .ok_or_else(|| conversion_error("drawing-sheet title-block Y geometry overflow"))?;
    if x_twice % 2 != 0 || y_twice % 2 != 0 {
        return Err(conversion_error(
            "drawing-sheet title-block geometry requires half-micrometre coordinates",
        ));
    }
    Ok(DrawingSheetRect {
        x_um: x_twice / 2,
        y_um: y_twice / 2,
        width_um: block.height_um,
        height_um: block.width_um,
    })
}

pub(in crate::render) fn zone_alpha_label(index: u8) -> String {
    // Match canvas and preview: engineering drawing zones omit ambiguous
    // letters and fall back to the numeric ordinal once the alphabet ends.
    const LETTERS: &[u8] = b"ABCDEFGHJKLMNPRSTUVWXY";
    LETTERS.get(usize::from(index)).map_or_else(
        || (usize::from(index) + 1).to_string(),
        |letter| char::from(*letter).to_string(),
    )
}

pub(in crate::render) fn midpoint_coordinate(
    start: i64,
    end: i64,
    context: &'static str,
) -> Result<i64, HardcopyRenderError> {
    end.checked_sub(start)
        .and_then(|distance| start.checked_add(distance / 2))
        .ok_or_else(|| conversion_error(format!("{context} coordinate overflow")))
}

pub(in crate::render) fn truncate_title_block_text(value: &str, max_chars: usize) -> String {
    let count = value.chars().count();
    if count <= max_chars {
        return value.to_owned();
    }
    let keep = max_chars.saturating_sub(1);
    let mut truncated = value.chars().take(keep).collect::<String>();
    truncated.push('…');
    truncated
}
