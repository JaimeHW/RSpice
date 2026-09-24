//! Schematic editing operations.
//!
//! Every mutation a user can make to a schematic, grouped by what it acts
//! on: wires and their junctions, buses, connections, placement previews,
//! clipboard and array replication, movement and stretch, instance
//! replacement, and the armed-tool lifecycle. Each operation leaves the
//! topology consistent, so callers never repair it afterwards.

mod array_ops;
mod bus_ops;
mod clipboard_ops;
mod connection_ops;
mod interface_repair_ops;
mod junction_labels;
mod movement_ops;
mod preview_ops;
mod replace_ops;
mod stretch_ops;
mod tool_ops;
mod wire_management;
mod wire_operations;
