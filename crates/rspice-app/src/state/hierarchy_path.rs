//! Canonical hierarchy paths from the portable application value crate.
//!
//! Probe-expression rewriting stays with project reference preparation because
//! it also recognizes application-owned current-probe syntax.

pub use rspice_app_types::hierarchy_path::{
    HierarchyPathError, InstancePath, InstancePathPattern, MAX_INSTANCE_PATH_BYTES,
    MAX_INSTANCE_PATH_DEPTH, PatternSegment, ProbeTarget,
};

mod probe_rewrite;
pub(crate) use probe_rewrite::remap_instance_probes_many;
