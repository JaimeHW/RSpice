//! SPICE naming rules.
//!
//! Rules about what a name *means* in a deck, as opposed to how it is spelled
//! in any particular vendor's dialect. Everything here is a leaf: it depends
//! on nothing else in the crate, because everything else depends on it.
//!
//! These predicates used to live in `compat`, beside the LTspice RAW reader.
//! That put a module documented as "compatibility readers" underneath the
//! solver, the device models, the circuit store and the parser — eight
//! modules taking a dependency on a foreign-format reader to ask whether a
//! node is ground. The rule is not a compatibility shim; it is part of what
//! SPICE means.

/// Return true when `name` is the canonical SPICE ground node.
///
/// Dialect-specific aliases are normalized during netlist elaboration.
/// Keeping this low-level predicate literal prevents an ordinary node named
/// `GND` from silently collapsing to zero when Xyce `REPLACEGROUND` is
/// disabled.
pub(crate) fn is_spice_ground_name(name: &str) -> bool {
    name.trim() == "0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_ground_is_node_zero() {
        assert!(is_spice_ground_name("0"));
        // Surrounding whitespace is incidental to tokenization, not part of
        // the name.
        assert!(is_spice_ground_name(" 0 "));
    }

    #[test]
    fn ground_aliases_are_not_canonical_ground() {
        // The whole point of keeping this literal: these are ordinary nodes
        // unless elaboration has been told to fold them, and folding them
        // here would do it even when Xyce REPLACEGROUND is off.
        for alias in ["GND", "gnd", "GND!", "0.0", "00", "ground"] {
            assert!(
                !is_spice_ground_name(alias),
                "{alias} must not be treated as canonical ground"
            );
        }
    }
}
