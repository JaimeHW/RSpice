//! Core engine type and shared orchestration helpers.

use super::{
    SimulationConfig, SimulationConfigOverrides, SimulationError, resolve_simulation_config,
};
use crate::netlist::{ElementKind, SubcircuitDef};
use crate::{Netlist, Value};
use std::collections::{HashMap, HashSet};
/// Main simulation engine
pub struct Engine {
    pub(crate) config: SimulationConfig,
}

impl Engine {
    fn validated_startup_netlist<'a>(
        &self,
        netlist: &'a Netlist,
    ) -> Option<std::borrow::Cow<'a, Netlist>> {
        if netlist.startup_directives.is_empty() {
            return Some(std::borrow::Cow::Borrowed(netlist));
        }
        let mut validated = netlist.clone();
        match crate::netlist::validate_startup_directives(&mut validated) {
            Ok(()) => Some(std::borrow::Cow::Owned(validated)),
            Err(error) => {
                // Circuit construction owns the typed fatal error path. Hint
                // collection remains infallible for established internal API
                // callers, but never applies unvalidated sidecar entries.
                log::debug!("startup validation failed before hint collection: {error}");
                None
            }
        }
    }

    /// Create a new engine with the given configuration
    pub fn new(config: SimulationConfig) -> Self {
        Self { config }
    }

    /// Get a reference to the simulation configuration
    pub fn config(&self) -> &SimulationConfig {
        &self.config
    }

    /// Resolve effective simulation configuration for a specific netlist.
    ///
    /// Applies `.OPTIONS` on top of the engine's base configuration.
    pub(crate) fn resolved_for_netlist(&self, netlist: &Netlist) -> Self {
        let resolved = resolve_simulation_config(
            &self.config,
            Some(&netlist.options),
            &SimulationConfigOverrides::default(),
        );
        Self::new(resolved)
    }

    /// Refuse analyses that require native BSIM4 charge equations when a
    /// card selects a charge model not yet ported. DC evaluation remains
    /// valid for these cards because BSIM4's DC path is capmod-independent.
    pub(crate) fn ensure_supported_bsim4_dynamic_charges(
        circuit: &crate::CircuitData,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        for dev in &circuit.bsim4v8.devices {
            let model = &dev.core.model;
            if !(0..=2).contains(&model.cap_mod) {
                return Err(SimulationError::Circuit(format!(
                    "{analysis} analysis requires native BSIM4 charge model equations; BSIM4 '{}' \
                     selects CAPMOD={} (only CAPMOD=0, 1, or 2 is implemented for charge-based analyses; \
                     DC operating point is supported)",
                    dev.name, model.cap_mod
                )));
            }
            if !model.cvcharge_mod_supported_for_charges() {
                return Err(SimulationError::Circuit(format!(
                    "{analysis} analysis requires native BSIM4 charge model equations; BSIM4 '{}' \
                     selects CVCHARGEMOD={} (only integer CVCHARGEMOD=0, 1, 2, or 3 is implemented for \
                     charge-based analyses; DC operating point is supported)",
                    dev.name, model.cvcharge_mod_value
                )));
            }
        }
        Ok(())
    }

    fn unsupported_b3soi_capmod_error(
        analysis: &str,
        family: &str,
        name: &str,
        selected: i32,
        supported: &str,
    ) -> SimulationError {
        SimulationError::Circuit(format!(
            "{analysis} analysis requires native {family} charge model equations; {family} '{name}' \
             selects CAPMOD={selected} (only {supported} is implemented for charge-based \
             analyses; DC operating point is supported)"
        ))
    }

    /// Refuse analyses that require B3SOI charge equations when a card selects
    /// a charge model not yet ported. DC evaluation remains valid because the
    /// SOI DC paths are capmod-independent.
    pub(crate) fn ensure_supported_b3soi_dynamic_charges(
        circuit: &crate::CircuitData,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        for dev in &circuit.b3soi_fd.devices {
            if dev.charges_suppressed() {
                continue;
            }
            if dev.model.cap_mod != 2 && dev.model.cap_mod != 3 {
                return Err(Self::unsupported_b3soi_capmod_error(
                    analysis,
                    "B3SOIFD",
                    &dev.name,
                    dev.model.cap_mod,
                    "CAPMOD=2 or 3",
                ));
            }
        }
        for dev in &circuit.b3soi.devices {
            if dev.charges_suppressed() {
                continue;
            }
            if dev.model.cap_mod != 2 && dev.model.cap_mod != 3 {
                return Err(Self::unsupported_b3soi_capmod_error(
                    analysis,
                    "B3SOIDD",
                    &dev.name,
                    dev.model.cap_mod,
                    "CAPMOD=2 or 3",
                ));
            }
        }
        for dev in &circuit.b3soi_pd.devices {
            if dev.charges_suppressed() {
                continue;
            }
            if dev.model.cap_mod != 2 && dev.model.cap_mod != 3 {
                return Err(Self::unsupported_b3soi_capmod_error(
                    analysis,
                    "B3SOIPD",
                    &dev.name,
                    dev.model.cap_mod,
                    "CAPMOD=2 or 3",
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn ensure_supported_ekv3_dynamic_charges(
        circuit: &crate::CircuitData,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        for dev in &circuit.ekv3s.devices {
            if analysis.eq_ignore_ascii_case("Noise") {
                continue;
            }
            return Err(SimulationError::Circuit(format!(
                "{analysis} analysis requires native EKV3 dynamic charge/stability equations; \
                 EKV3 '{}' is supported for the VA-Models source-backed LEVEL=301 \
                 NMOS150 DC operating-point/DC sweep slice and the Xyce-validated LEVEL=301 \
                 NMOS150 VANOISE small-signal/noise slice. AC, transient, STB, and pole-zero \
                 remain fail-closed until validated against Xyce dynamic oracles",
                dev.name
            )));
        }
        Ok(())
    }

    pub(crate) fn ensure_supported_dynamic_charges(
        circuit: &crate::CircuitData,
        analysis: &str,
    ) -> Result<(), SimulationError> {
        Self::ensure_supported_bsim4_dynamic_charges(circuit, analysis)?;
        Self::ensure_supported_b3soi_dynamic_charges(circuit, analysis)?;
        Self::ensure_supported_ekv3_dynamic_charges(circuit, analysis)
    }

    pub(crate) fn ensure_supported_ac_dynamic_charges(
        circuit: &crate::CircuitData,
    ) -> Result<(), SimulationError> {
        Self::ensure_supported_bsim4_dynamic_charges(circuit, "AC")?;
        Self::ensure_supported_b3soi_dynamic_charges(circuit, "AC")?;
        Self::ensure_supported_ekv3_dynamic_charges(circuit, "AC")
    }

    pub(crate) fn ensure_supported_transient_dynamic_charges(
        circuit: &crate::CircuitData,
    ) -> Result<(), SimulationError> {
        Self::ensure_supported_bsim4_dynamic_charges(circuit, "Transient")?;
        Self::ensure_supported_b3soi_dynamic_charges(circuit, "Transient")?;
        Self::ensure_supported_ekv3_dynamic_charges(circuit, "Transient")
    }

    pub(crate) fn node_lookup_candidates(netlist: &Netlist, node_name: &str) -> Vec<String> {
        let canonical = netlist.ground_policy().canonical_node(node_name);
        if canonical == "0" {
            return vec![canonical.to_string()];
        }
        let mut candidates = Vec::new();
        Self::push_unique_node_lookup_candidate(&mut candidates, node_name.to_string());

        let normalized = Self::normalize_hierarchical_node_name(node_name).replace(':', ".");
        Self::push_unique_node_lookup_candidate(&mut candidates, normalized);

        if let Some(resolved) = Self::resolve_hierarchical_node_name(netlist, node_name) {
            Self::push_unique_node_lookup_candidate(&mut candidates, resolved);
        }

        candidates
    }

    pub(crate) fn resolve_hierarchical_node_name(
        netlist: &Netlist,
        node_name: &str,
    ) -> Option<String> {
        let normalized = Self::normalize_hierarchical_node_name(node_name);
        Self::resolve_hierarchical_node_name_with_delimiter(netlist, &normalized, ':').or_else(
            || Self::resolve_hierarchical_node_name_with_delimiter(netlist, &normalized, '.'),
        )
    }

    fn resolve_hierarchical_node_name_with_delimiter(
        netlist: &Netlist,
        normalized: &str,
        delimiter: char,
    ) -> Option<String> {
        let parts = normalized.split(delimiter).collect::<Vec<_>>();
        if parts.len() < 2 {
            return None;
        }
        let delimiter = delimiter.to_string();

        for instance_count in (1..parts.len()).rev() {
            if parts[..instance_count].iter().any(|part| part.is_empty()) {
                continue;
            }

            let local_node = parts[instance_count..].join(&delimiter);
            if local_node.is_empty() {
                continue;
            }

            if let Some(resolved) = Self::resolve_hierarchical_node_parts(
                netlist,
                &parts[..instance_count],
                &local_node,
            ) {
                return Some(resolved);
            }
        }

        None
    }

    fn resolve_hierarchical_node_parts(
        netlist: &Netlist,
        instance_names: &[&str],
        local_node: &str,
    ) -> Option<String> {
        let mut elements = netlist.elements.as_slice();
        let root_subcircuits = netlist.subcircuits.as_slice();
        let mut subcircuits = root_subcircuits;
        let mut prefix = String::new();
        let mut node_map: HashMap<String, String> = HashMap::new();

        for instance_name in instance_names {
            let instance = elements.iter().find(|element| {
                element.name.eq_ignore_ascii_case(instance_name)
                    && matches!(element.kind, ElementKind::Subcircuit { .. })
            })?;
            let ElementKind::Subcircuit { subckt_name, .. } = &instance.kind else {
                return None;
            };
            let subckt = Self::find_subcircuit_definition(subcircuits, subckt_name)
                .or_else(|| Self::find_subcircuit_definition(root_subcircuits, subckt_name))?;

            let mut child_node_map = HashMap::new();
            for (port, external_node) in subckt.ports.iter().zip(instance.nodes.iter()) {
                child_node_map.insert(
                    Self::normalize_hierarchical_node_name(port),
                    Self::remap_hierarchical_node_name(
                        external_node,
                        &prefix,
                        &node_map,
                        &netlist.global_nodes,
                    ),
                );
            }

            prefix = if prefix.is_empty() {
                instance.name.clone()
            } else {
                format!("{}.{}", prefix, instance.name)
            };
            node_map = child_node_map;
            elements = subckt.elements.as_slice();
            subcircuits = subckt.nested_subcircuits.as_slice();
        }

        Some(Self::remap_hierarchical_node_name(
            local_node,
            &prefix,
            &node_map,
            &netlist.global_nodes,
        ))
    }

    fn startup_node_id(
        netlist: &Netlist,
        circuit: &crate::CircuitData,
        node_name: &str,
    ) -> Option<usize> {
        Self::node_lookup_candidates(netlist, node_name)
            .into_iter()
            .find_map(|candidate| circuit.get_node_by_name(&candidate))
    }

    fn push_unique_node_lookup_candidate(candidates: &mut Vec<String>, candidate: String) {
        if !candidates
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(&candidate))
        {
            candidates.push(candidate);
        }
    }

    fn normalize_hierarchical_node_name(node_name: &str) -> String {
        node_name
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase()
    }

    fn find_subcircuit_definition<'a>(
        subcircuits: &'a [SubcircuitDef],
        name: &str,
    ) -> Option<&'a SubcircuitDef> {
        for subckt in subcircuits {
            if subckt.name.eq_ignore_ascii_case(name) {
                return Some(subckt);
            }
            if let Some(nested) = Self::find_subcircuit_definition(&subckt.nested_subcircuits, name)
            {
                return Some(nested);
            }
        }
        None
    }

    fn remap_hierarchical_node_name(
        node: &str,
        prefix: &str,
        node_map: &HashMap<String, String>,
        global_nodes: &HashSet<String>,
    ) -> String {
        if node == "0" {
            return "0".to_string();
        }

        if global_nodes.contains(&node.to_ascii_uppercase()) {
            return node.to_string();
        }

        if let Some(mapped) = node_map.get(&Self::normalize_hierarchical_node_name(node)) {
            return mapped.clone();
        }

        if prefix.is_empty() {
            node.to_string()
        } else {
            format!("{prefix}.{node}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hierarchical_node_lookup_resolves_nested_subcircuit_port() {
        let netlist = Netlist::parse(
            "\
nested hierarchical lookup
X2 4 5 IC_SubSubckt
.SUBCKT IC_Subckt in out
R1 in mid 10
C1 mid out 1u
.ENDS
.SUBCKT IC_SubSubckt in out
R1 in a 1
X1 a b IC_Subckt
R2 b out 1
.ENDS
.END
",
        )
        .expect("test deck parses");

        let resolved = Engine::resolve_hierarchical_node_name(&netlist, "X2:X1:out")
            .expect("hierarchical node resolves");

        assert_eq!(resolved.to_ascii_lowercase(), "x2.b");
    }

    #[test]
    fn hierarchical_node_lookup_resolves_dot_delimited_subcircuit_port() {
        let netlist = Netlist::parse(
            "\
dot-delimited hierarchical lookup
X2 4 5 IC_Subckt
.SUBCKT IC_Subckt in out
R1 in mid 10
C1 mid out 1u
.ENDS
.END
",
        )
        .expect("test deck parses");

        let resolved = Engine::resolve_hierarchical_node_name(&netlist, "X2.out")
            .expect("dot-delimited hierarchical node resolves");

        assert_eq!(resolved, "5");
    }

    #[test]
    fn hierarchical_node_lookup_preserves_colon_suffix_local_nodes() {
        let netlist = Netlist::parse(
            "\
colon local node lookup
X1 2 0 sub1
.SUBCKT sub1 a b
V1 1: 0 1
V2 : 0 1
X2 3 0 sub2
.ENDS
.SUBCKT sub2 c d
V1 1: 0 1
V2 : 0 1
.ENDS
.END
",
        )
        .expect("test deck parses");

        assert_eq!(
            Engine::resolve_hierarchical_node_name(&netlist, "X1:1:")
                .expect("colon-suffix local node resolves"),
            "X1.1:"
        );
        assert_eq!(
            Engine::resolve_hierarchical_node_name(&netlist, "X1::")
                .expect("literal colon local node resolves"),
            "X1.:"
        );
        assert_eq!(
            Engine::resolve_hierarchical_node_name(&netlist, "X1:X2:1:")
                .expect("nested colon-suffix local node resolves"),
            "X1.X2.1:"
        );
        assert_eq!(
            Engine::resolve_hierarchical_node_name(&netlist, "X1:X2::")
                .expect("nested literal colon local node resolves"),
            "X1.X2.:"
        );
    }

    #[test]
    fn hierarchical_startup_directives_apply_to_flattened_node_ids() {
        let netlist = Netlist::parse(
            "\
hierarchical startup directives
V1 1 0 0
X2 1 2 IC_SubSubckt
RLOAD 2 0 1k
.NODESET V(X2:X1:out)=0.1
.IC V(X2:X1:out)=0.25
.SUBCKT IC_Subckt in out
R1 in mid 10
C1 mid out 1u
.ENDS
.SUBCKT IC_SubSubckt in out
R1 in a 1
X1 a b IC_Subckt
R2 b out 1
.ENDS
.END
",
        )
        .expect("test deck parses");
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let flattened_node = circuit
            .get_node_by_name("X2.B")
            .expect("flattened internal node exists");

        let ic_hints = engine.collect_initial_condition_hints(&netlist, &circuit);
        assert_eq!(ic_hints, vec![(flattened_node, 0.25)]);

        let voltage_hints = engine.collect_node_voltage_hints(&netlist, &circuit);
        assert_eq!(voltage_hints, vec![(flattened_node, 0.25)]);
    }

    #[test]
    fn dot_delimited_subcircuit_port_initial_condition_targets_connected_node() {
        let netlist = Netlist::parse(
            "\
dot-delimited hierarchical startup directive
V1 1 0 0
X2 1 3 RC
CLOAD 3 0 1u
.IC V(X2.B)=0.5
.SUBCKT RC a b
R1 a mid 10
C1 mid b 1u
.ENDS
.END
",
        )
        .expect("test deck parses");
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let connected_node = circuit
            .get_node_by_name("3")
            .expect("connected top-level node exists");

        let ic_hints = engine.collect_initial_condition_hints(&netlist, &circuit);
        assert_eq!(ic_hints, vec![(connected_node, 0.5)]);
    }

    #[test]
    fn infallible_hint_collectors_never_apply_failed_startup_validation() {
        let mut netlist = Netlist::parse(
            "invalid startup hints\n\
             V1 1 0 1\n\
             .IC V(1)=0.25\n\
             .NODESET V(1)=0.75\n\
             .OP\n\
             .END\n",
        )
        .expect("default/ngspice mode permits both startup modes");
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("permissive-mode circuit builds");
        netlist
            .params
            .set_expression_dialect(crate::netlist::ExpressionDialect::Xyce);

        assert!(
            engine
                .collect_initial_condition_hints(&netlist, &circuit)
                .is_empty()
        );
        assert!(
            engine
                .collect_node_voltage_hints(&netlist, &circuit)
                .is_empty()
        );
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(SimulationConfig::default())
    }
}

impl Engine {
    fn scoped_startup_directives(
        &self,
        netlist: &Netlist,
    ) -> (
        Vec<crate::netlist::InitialCondition>,
        Vec<crate::netlist::NodeSet>,
    ) {
        if netlist.subcircuits.is_empty() {
            return (Vec::new(), Vec::new());
        }

        match crate::netlist::flatten_netlist_with_models(netlist) {
            Ok(flattened) => (
                flattened.scoped_initial_conditions,
                flattened.scoped_node_sets,
            ),
            Err(error) => {
                log::debug!(
                    "could not collect scoped startup directives from flattened netlist: {error}"
                );
                (Vec::new(), Vec::new())
            }
        }
    }

    /// Collect node-voltage hints from .NODESET and .IC directives.
    ///
    /// .IC entries override .NODESET entries for the same node.
    pub(crate) fn collect_node_voltage_hints(
        &self,
        netlist: &Netlist,
        circuit: &crate::CircuitData,
    ) -> Vec<(usize, Value)> {
        let Some(validated) = self.validated_startup_netlist(netlist) else {
            return Vec::new();
        };
        let netlist = validated.as_ref();
        let mut by_node: Vec<Option<Value>> = vec![None; circuit.num_nodes() + 1];
        let (scoped_initial_conditions, scoped_node_sets) = self.scoped_startup_directives(netlist);

        for nodeset in netlist.node_sets.iter().chain(scoped_node_sets.iter()) {
            if !nodeset.voltage.is_finite() {
                continue;
            }
            if let Some(node_id) = Self::startup_node_id(netlist, circuit, &nodeset.node)
                && node_id > 0
                && node_id <= circuit.num_nodes()
            {
                by_node[node_id] = Some(nodeset.voltage);
            }
        }

        for ic in netlist
            .initial_conditions
            .iter()
            .chain(scoped_initial_conditions.iter())
        {
            if !ic.voltage.is_finite() {
                continue;
            }
            if let Some(node_id) = Self::startup_node_id(netlist, circuit, &ic.node)
                && node_id > 0
                && node_id <= circuit.num_nodes()
            {
                by_node[node_id] = Some(ic.voltage);
            }
        }

        by_node
            .into_iter()
            .enumerate()
            .skip(1)
            .filter_map(|(node_id, voltage)| voltage.map(|v| (node_id, v)))
            .collect()
    }

    /// Collect node-voltage initial conditions from .IC directives only.
    pub(crate) fn collect_initial_condition_hints(
        &self,
        netlist: &Netlist,
        circuit: &crate::CircuitData,
    ) -> Vec<(usize, Value)> {
        let Some(validated) = self.validated_startup_netlist(netlist) else {
            return Vec::new();
        };
        let netlist = validated.as_ref();
        let (scoped_initial_conditions, _) = self.scoped_startup_directives(netlist);
        netlist
            .initial_conditions
            .iter()
            .chain(scoped_initial_conditions.iter())
            .filter_map(|ic| {
                if !ic.voltage.is_finite() {
                    return None;
                }
                let node_id = Self::startup_node_id(netlist, circuit, &ic.node)?;
                if node_id == 0 || node_id > circuit.num_nodes() {
                    return None;
                }
                Some((node_id, ic.voltage))
            })
            .collect()
    }

    /// Apply .IC node overrides to an operating-point solution vector.
    ///
    /// Returns the number of nodes overridden.
    pub(crate) fn apply_initial_condition_overrides(
        &self,
        netlist: &Netlist,
        circuit: &crate::CircuitData,
        solution: &mut [Value],
    ) -> usize {
        let hints = self.collect_initial_condition_hints(netlist, circuit);
        let mut applied = 0usize;

        for (node_id, voltage) in hints {
            let idx = node_id - 1;
            if idx < solution.len() {
                solution[idx] = voltage;
                applied += 1;
            }
        }

        applied
    }

    /// Solve the DC operating point with optional node-voltage hints.
    #[cfg(test)]
    pub(crate) fn solve_dc_operating_point(
        &self,
        netlist: &Netlist,
        circuit: &mut crate::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        self.solve_dc_operating_point_with_abort(
            netlist,
            circuit,
            matrix,
            &crate::abort_signal::NoAbort,
        )
    }

    /// Solve the DC operating point with optional node-voltage hints and abort support.
    pub(crate) fn solve_dc_operating_point_with_abort(
        &self,
        netlist: &Netlist,
        circuit: &mut crate::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        abort: &dyn crate::abort_signal::AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        if circuit.has_nonlinear_devices() || !circuit.generic_switches.is_empty() {
            let hints = self.collect_node_voltage_hints(netlist, circuit);
            if hints.is_empty() {
                self.solve_nonlinear_with_node_hints_and_abort(circuit, matrix, &[], abort)
            } else {
                self.solve_nonlinear_with_node_hints_and_abort(circuit, matrix, &hints, abort)
            }
        } else {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            self.solve_linear(circuit, matrix)
        }
    }
}
