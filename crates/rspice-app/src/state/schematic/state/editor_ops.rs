//! Schematic editing operations.
//!
//! Every mutation a user can make to a schematic, grouped by what it acts
//! on: wires and their junctions, buses, connections, placement previews,
//! clipboard and array replication, movement and stretch, instance
//! replacement, and the armed-tool lifecycle. Each operation leaves the
//! topology consistent, so callers never repair it afterwards.

mod array_ops;
mod binding_ops;
mod bus_ops;
mod clipboard_ops;
mod connection_ops;
mod connectivity_repair;
mod interface_repair_ops;
mod junction_labels;
mod layout_ops;
mod model_binding_ops;
mod movement_ops;
mod probe_ops;
mod property_ops;
mod replace_ops;
mod sheet_topology;
mod stretch_ops;
mod symbol_ops;
mod tool_ops;
mod wire_management;
mod wire_operations;
