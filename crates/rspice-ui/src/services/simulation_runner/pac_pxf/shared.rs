pub(super) fn normalize_pac_node_name(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 3
        && trimmed
            .get(0..2)
            .map(|prefix| prefix.eq_ignore_ascii_case("V("))
            .unwrap_or(false)
        && trimmed.ends_with(')')
    {
        return trimmed[2..trimmed.len() - 1].trim().to_string();
    }
    trimmed.to_string()
}

pub(super) fn resolve_pac_output_node_with_abort(
    result: &rspice_core::analysis::advanced::pac::PacResult,
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
use super::super::error::ensure_not_aborted;
