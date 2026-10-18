//! A title block as sheets, defaults, presets and packages have written it.
//!
//! The product has retired some of what it once wrote, and every sheet and
//! package that names it still loads: the `compact` block reads as the
//! sheet's standard block.

use super::*;

/// A title block as it was written, which names the retired `compact` block
/// among the templates.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DrawingSheetTitleBlockWire {
    template: DrawingSheetTitleBlockTemplateWire,
    anchor: DrawingSheetTitleBlockAnchor,
    rotation: DrawingSheetTitleBlockRotation,
    offset_x_um: i64,
    offset_y_um: i64,
    scale: DrawingSheetScale,
    #[serde(default)]
    managed_template: Option<DrawingSheetManagedTemplateSnapshot>,
    fields: BTreeMap<DrawingSheetTitleFieldId, DrawingSheetTitleFieldState>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum DrawingSheetTitleBlockTemplateWire {
    /// Retired: a 130 mm block that predated ISO 7200's and ASME Y14.1's
    /// forms.
    Compact,
    #[serde(alias = "wide")]
    Standard,
    OrganizationManaged,
    None,
    #[serde(rename = "asme-y14-1")]
    AsmeY141,
}

impl DrawingSheetTitleBlockWire {
    /// The title block as the product draws it: a block written as the
    /// retired compact one is `convention`'s standard block, ISO 7200's or
    /// ASME Y14.1's, its field values and choices kept.
    pub(super) fn into_title_block(
        self,
        convention: DrawingSheetFrameConvention,
    ) -> DrawingSheetTitleBlock {
        let template = match self.template {
            DrawingSheetTitleBlockTemplateWire::Compact => {
                DrawingSheetTitleBlockTemplate::for_convention(convention)
            }
            DrawingSheetTitleBlockTemplateWire::Standard => {
                DrawingSheetTitleBlockTemplate::Standard
            }
            DrawingSheetTitleBlockTemplateWire::OrganizationManaged => {
                DrawingSheetTitleBlockTemplate::OrganizationManaged
            }
            DrawingSheetTitleBlockTemplateWire::None => DrawingSheetTitleBlockTemplate::None,
            DrawingSheetTitleBlockTemplateWire::AsmeY141 => {
                DrawingSheetTitleBlockTemplate::AsmeY141
            }
        };
        DrawingSheetTitleBlock {
            template,
            anchor: self.anchor,
            rotation: self.rotation,
            offset_x_um: self.offset_x_um,
            offset_y_um: self.offset_y_um,
            scale: self.scale,
            managed_template: self.managed_template,
            fields: self.fields,
        }
    }
}

impl<'de> Deserialize<'de> for DrawingSheetTitleBlock {
    /// A title block read without the sheet it belongs to: the retired
    /// compact block reads as ISO 7200's. A sheet's own reads as its
    /// format's standard block (see [`SchematicSheetFormat`]).
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        DrawingSheetTitleBlockWire::deserialize(deserializer)
            .map(|wire| wire.into_title_block(DrawingSheetFrameConvention::Iso5457))
    }
}
