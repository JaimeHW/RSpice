//! A personal drawing-sheet preset captured into a project.
//!
//! An authored sheet references only project-owned formats, so a personal
//! preset is copied into the project before a sheet uses it. Every surface
//! that lets a sheet use a personal preset captures through this one
//! operation, so the preset lands in the project the same way whichever
//! surface asked.

use super::Model;
use crate::state::{
    AuthoredDrawingSheetSize, DesignManagementCatalog, DrawingSheetInheritance, DrawingSheetPreset,
    DrawingSheetPresetScope, SchematicSheetFormat,
};
use uuid::Uuid;

impl Model {
    /// Publish a project-owned copy of the personal preset `personal` into
    /// `candidate`, or return the project preset already drawing the same
    /// sheet.
    pub(crate) fn capture_personal_preset_into_project(
        candidate: &mut DesignManagementCatalog,
        personal: &DrawingSheetPreset,
    ) -> Result<DrawingSheetPreset, String> {
        if personal.scope != DrawingSheetPresetScope::User {
            return Err("Only a personal preset requires project capture.".to_owned());
        }
        if let Some(existing) = candidate
            .drawing_sheet_settings()
            .presets
            .iter()
            .find(|preset| same_preset_content(&preset.format, &personal.format))
        {
            return Ok(existing.clone());
        }
        let visible = candidate.drawing_sheet_settings().presets.clone();
        let id = if visible
            .iter()
            .any(|preset| preset.id.eq_ignore_ascii_case(&personal.id))
        {
            format!("custom-{}", Uuid::new_v4().simple())
        } else {
            personal.id.clone()
        };
        let name = Self::unique_preset_copy_name(&personal.name, &visible);
        let mut format = personal
            .format
            .try_update(|draft| {
                draft.inheritance = DrawingSheetInheritance::Explicit;
                if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size {
                    snapshot.preset_id = Some(id.clone());
                    snapshot.name.clone_from(&name);
                    snapshot.source_preset_unavailable = false;
                }
            })
            .map_err(|error| error.to_string())?;
        format.inheritance = DrawingSheetInheritance::Explicit;
        let captured = DrawingSheetPreset {
            id,
            name,
            scope: DrawingSheetPresetScope::Project,
            format: format.as_reusable_drawing_sheet_preset(),
        };
        candidate
            .publish_drawing_sheet_preset(candidate.revision(), captured.clone())
            .map_err(|error| error.to_string())?;
        Ok(captured)
    }

    /// `base`, or the first `base 2`, `base 3`, ... that no preset in
    /// `visible` is named, ignoring case.
    pub(crate) fn unique_preset_copy_name(base: &str, visible: &[DrawingSheetPreset]) -> String {
        if !visible
            .iter()
            .any(|preset| preset.name.eq_ignore_ascii_case(base))
        {
            return base.to_owned();
        }
        (2_u32..)
            .map(|suffix| format!("{base} {suffix}"))
            .find(|candidate| {
                !visible
                    .iter()
                    .any(|preset| preset.name.eq_ignore_ascii_case(candidate))
            })
            .expect("a bounded preset catalog always has a free display suffix")
    }
}

fn same_preset_content(left: &SchematicSheetFormat, right: &SchematicSheetFormat) -> bool {
    let normalize = |format: &SchematicSheetFormat| {
        format
            .try_update(|draft| {
                draft.inheritance = DrawingSheetInheritance::Explicit;
                if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size {
                    snapshot.preset_id = None;
                    snapshot.name = "content comparison".to_owned();
                    snapshot.source_preset_unavailable = false;
                }
            })
            .ok()
    };
    normalize(left) == normalize(right)
}
