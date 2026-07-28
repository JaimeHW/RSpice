//! Reference output parsing and comparison for ngspice regression tests.

use super::*;
#[path = "reference/datasets.rs"]
mod datasets;
#[path = "reference/live.rs"]
mod live;
#[path = "reference/measures.rs"]
mod measures;
#[path = "reference/op.rs"]
mod op;
#[path = "reference/probes.rs"]
mod probes;
#[path = "reference/tables.rs"]
mod tables;
#[path = "reference/tolerances.rs"]
mod tolerances;
#[path = "reference/transfer_pz.rs"]
mod transfer_pz;
