//! Schematic-domain services shared by the app's edit workflows.

mod conductor_routes;
pub(in crate::workbench) use rspice_app_state::workbench::app::schematic::edit_authority;
pub(in crate::workbench) mod hierarchy_reference_impact;
pub(in crate::workbench) mod instance_catalog;
pub(in crate::workbench) mod named_net;
pub(in crate::workbench) mod placement;
