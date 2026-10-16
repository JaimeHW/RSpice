//! What acts on the schematic selection: the immediate edit commands, the
//! armed Move and Stretch tools, and the array, rename and bulk-edit forms.

pub(in crate::workbench) mod array;
pub(in crate::workbench) mod bulk_edit;
pub(in crate::workbench) mod commands;
pub(in crate::workbench) mod move_selection;
pub(in crate::workbench) mod rename;
pub(in crate::workbench) mod stretch;

#[cfg(test)]
mod tests;
