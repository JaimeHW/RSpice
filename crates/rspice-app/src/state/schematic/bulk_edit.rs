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
    schematic.session.selection.clear();
    for id in edit.value {
        schematic.session.selection.select_component(id);
    }
    Ok(SelectionBulkReceipt {
        changed: schematic.session.selection.components.len(),
    })
}
