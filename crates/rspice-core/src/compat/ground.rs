//! Shared SPICE ground-name handling.

/// Return true when `name` is the canonical SPICE ground node.
///
/// Dialect-specific aliases are normalized during netlist elaboration. Keeping
/// this low-level predicate literal prevents an ordinary node named `GND` from
/// silently collapsing to zero when Xyce `REPLACEGROUND` is disabled.
pub(crate) fn is_spice_ground_name(name: &str) -> bool {
    name.trim() == "0"
}
