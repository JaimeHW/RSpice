//! What acts on the schematic selection: the immediate edit commands, the
//! armed Move and Stretch tools, and the array, rename, bulk-edit and
//! shared-properties forms.

pub(in crate::workbench) mod array;
pub(crate) mod bulk_edit;
pub(in crate::workbench) mod bulk_filter;
pub(in crate::workbench) mod commands;
pub(crate) mod move_selection;
pub(crate) mod rename;
pub(crate) mod shared_properties;
pub(crate) mod stretch;
