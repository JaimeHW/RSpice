//! Node resolution shared by PAC and PXF.
//!
//! Both name their output node the same way, and both must resolve it
//! against the flattened deck rather than the schematic.

use rspice_simulation::periodic::normalize_pac_node_name;

pub(super) fn resolve_pac_output_node_with_abort(
    result: &rspice_core::analysis::pac::PacResult,
    requested: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<usize>> {
    ensure_not_aborted(abort)?;
    let trimmed = requested.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let resolved = result
        .node_index(trimmed)
        .or_else(|| result.node_index(&normalize_pac_node_name(trimmed)));
    ensure_not_aborted(abort)?;
    Ok(resolved)
}
use rspice_core::abort_signal::AbortSignal;

use super::super::ServiceRunResult;
use rspice_simulation::error::ensure_not_aborted;
