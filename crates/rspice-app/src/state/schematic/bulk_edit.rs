//! Editor selection after a validated design-owned bulk edit.
use crate::state::SchematicState;
pub(crate) use rspice_design::schematic::owned::bulk_edit::{
    SelectionBulkProperty, SelectionBulkUnsetBehavior, mutate_component, parameter_key_from_filter,
    validate_bulk_value,
};
use std::collections::BTreeSet;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SelectionBulkReceipt {
    pub(crate) changed: usize,
}
pub(crate) fn apply_bulk_edit(
    schematic: &mut SchematicState,
    target_ids: &BTreeSet<u64>,
    property: SelectionBulkProperty,
    value: &str,
    unset: SelectionBulkUnsetBehavior,
    current_property: Option<&str>,
) -> Result<SelectionBulkReceipt, String> {
    let edit = rspice_design::schematic::owned::bulk_edit::apply_bulk_edit(
        &mut schematic.design,
        target_ids,
        property,
        value,
        unset,
        current_property,
    )?;
    if edit.value.is_empty() {
        return Ok(SelectionBulkReceipt { changed: 0 });
    }
    schematic.finish_document_edit(edit.committed);
    if !edit.committed {
        return Err("The bulk-edit transaction did not produce a document change.".to_owned());
    }
    schematic.session.editor.selection.clear();
    for id in edit.value {
        schematic.session.editor.selection.select_component(id);
    }
    Ok(SelectionBulkReceipt {
        changed: schematic.session.editor.selection.components.len(),
    })
}

/// Replace each expected instance by its edited candidate as one undo step.
/// Returns how many instances changed.
pub(crate) fn replace_components(
    schematic: &mut SchematicState,
    edits: Vec<(crate::state::Component, crate::state::Component)>,
    description: &str,
) -> Result<usize, String> {
    if schematic.session.read_only {
        return Err("The active schematic is read-only.".to_owned());
    }
    let edit = rspice_design::schematic::owned::bulk_edit::replace_components(
        &mut schematic.design,
        edits,
        description,
    )?;
    schematic.finish_document_edit(edit.committed);
    Ok(edit.value)
}
