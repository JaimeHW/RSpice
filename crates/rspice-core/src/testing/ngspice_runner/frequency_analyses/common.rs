use super::*;

impl TestRunner {
    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
    // Frequency Point Generation
    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

    #[inline]
    pub(in crate::testing::ngspice_runner) fn optional_probe_node(name: &str) -> Option<String> {
        let trimmed = name.trim();
        if trimmed.is_empty() || trimmed == "0" || trimmed.eq_ignore_ascii_case("gnd") {
            None
        } else {
            Some(trimmed.to_string())
        }
    }

    pub(in crate::testing::ngspice_runner) fn resolve_circuit_node_index(
        circuit: &crate::circuit::Circuit,
        node: &str,
        role: &str,
    ) -> Result<usize, String> {
        circuit
            .get_node_by_name(node)
            .ok_or_else(|| format!("Unknown {role} node '{node}'"))
    }

    pub(in crate::testing::ngspice_runner) fn resolve_optional_circuit_node_index(
        circuit: &crate::circuit::Circuit,
        node: Option<&str>,
        role: &str,
    ) -> Result<Option<usize>, String> {
        match node {
            Some(name) => Ok(Some(Self::resolve_circuit_node_index(circuit, name, role)?)),
            None => Ok(None),
        }
    }
}
