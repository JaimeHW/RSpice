//! Authored node name to solver node index, for an unsolved netlist.
//!
//! `.SENS` and `.PZ` name their ports the way a deck does — `out`, `2`, `gnd`
//! — while the runners take the index the elaborated circuit assigned. Turning
//! one into the other means elaborating the deck and applying its ground
//! policy, which is real work with a real contract: which spellings mean node
//! zero, whether a bare integer is an index or a name, and what happens when
//! the name is absent.
//!
//! Every frontend had its own copy of that contract, so the browser API simply
//! refused both cards. This is the single implementation they all call.

use std::collections::HashMap;

use crate::abort_signal::AbortSignal;
use crate::netlist::GroundPolicy;
use crate::{Netlist, engine::Engine, engine::SimulationError};

/// Authored node names of one elaborated netlist.
///
/// Build it once and resolve every port of a card against it: elaborating a
/// large hierarchical deck per port would repeat the expensive half of the
/// work for each name.
#[derive(Debug, Clone)]
pub struct NodeResolver {
    indices: HashMap<String, usize>,
    ground: GroundPolicy,
}

impl NodeResolver {
    /// Elaborate `netlist` and record its node namespace.
    ///
    /// Elaboration is real work on a hierarchical deck, so it runs under the
    /// caller's abort source: cancelling a run that is still resolving a
    /// card's ports must stop it, not only one already inside a solver.
    pub fn build_with_abort(
        engine: &Engine,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        let names = circuit.node_names_sorted();
        let mut indices = HashMap::new();
        indices
            .try_reserve(names.len())
            .map_err(|_| SimulationError::Circuit("node name index".to_owned()))?;
        for (index, name) in names.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            indices.insert(name.to_ascii_uppercase(), index + 1);
        }
        Ok(Self {
            indices,
            ground: netlist.ground_policy(),
        })
    }

    /// Resolve one authored node name.
    ///
    /// The deck's own ground policy decides which spellings are node zero, a
    /// bare integer is taken as the index it spells, and anything else must
    /// name a node the elaborated circuit actually has. `role` names the card
    /// port in the failure so an operator knows which of a `.PZ` card's four
    /// ports was wrong.
    pub fn resolve(&self, node: &str, role: &str) -> Result<usize, SimulationError> {
        let node = node.trim();
        if node.is_empty() {
            return Err(SimulationError::Netlist(format!(
                "{role} names no node; a port must be spelled out"
            )));
        }
        if self.ground.is_ground(node) {
            return Ok(0);
        }
        if let Ok(index) = node.parse::<usize>() {
            return Ok(index);
        }
        self.indices
            .get(&node.to_ascii_uppercase())
            .copied()
            .ok_or_else(|| {
                SimulationError::Netlist(format!(
                    "{role} names node '{node}', which the elaborated circuit does not contain"
                ))
            })
    }

    /// Resolve an optional reference port, mapping ground onto `None`.
    ///
    /// A voltage probe referenced to ground is a single-ended probe, and the
    /// runners spell that as an absent reference rather than as node zero.
    pub fn resolve_reference(
        &self,
        node: Option<&str>,
        role: &str,
    ) -> Result<Option<usize>, SimulationError> {
        match node {
            Some(node) => {
                let index = self.resolve(node, role)?;
                Ok((index != 0).then_some(index))
            }
            None => Ok(None),
        }
    }
}

impl Engine {
    /// Resolve one authored node name against this deck's elaborated circuit.
    ///
    /// Convenience for a caller with a single name to resolve. A caller with
    /// several — every port of a `.PZ` card — should build a [`NodeResolver`]
    /// once instead, because each call here elaborates the deck again.
    pub fn resolve_node_with_abort(
        &self,
        netlist: &Netlist,
        node: &str,
        role: &str,
        abort: &dyn AbortSignal,
    ) -> Result<usize, SimulationError> {
        NodeResolver::build_with_abort(self, netlist, abort)?.resolve(node, role)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{ImmediateAbort, NoAbort};
    use crate::engine::SimulationConfig;

    fn deck() -> Netlist {
        Netlist::parse(
            "Node resolution\n\
             V1 in 0 DC 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .op\n\
             .end\n",
        )
        .expect("deck parses")
    }

    #[test]
    fn authored_names_resolve_case_insensitively_and_ground_is_node_zero() {
        let engine = Engine::new(SimulationConfig::default());
        let netlist = deck();
        let resolver =
            NodeResolver::build_with_abort(&engine, &netlist, &NoAbort).expect("resolver builds");
        let out = resolver.resolve("out", ".PZ output").expect("out resolves");
        assert_eq!(resolver.resolve("OUT", ".PZ output").expect("case"), out);
        assert_ne!(out, 0);
        assert_eq!(resolver.resolve("0", ".PZ reference").expect("ground"), 0);
    }

    #[test]
    fn an_unknown_node_is_a_typed_failure_naming_the_card_port() {
        let engine = Engine::new(SimulationConfig::default());
        let netlist = deck();
        let resolver =
            NodeResolver::build_with_abort(&engine, &netlist, &NoAbort).expect("resolver builds");
        let error = resolver
            .resolve("nowhere", ".SENS output")
            .expect_err("an absent node must fail closed");
        let message = error.to_string();
        assert!(message.contains(".SENS output"), "{message}");
        assert!(message.contains("nowhere"), "{message}");
    }

    #[test]
    fn a_ground_reference_becomes_a_single_ended_probe() {
        let engine = Engine::new(SimulationConfig::default());
        let netlist = deck();
        let resolver =
            NodeResolver::build_with_abort(&engine, &netlist, &NoAbort).expect("resolver builds");
        assert_eq!(
            resolver
                .resolve_reference(Some("0"), ".SENS reference")
                .expect("ground reference"),
            None
        );
        assert!(
            resolver
                .resolve_reference(Some("out"), ".SENS reference")
                .expect("named reference")
                .is_some()
        );
        assert_eq!(
            resolver
                .resolve_reference(None, ".SENS reference")
                .expect("absent reference"),
            None
        );
    }

    /// Which spellings mean node zero is the deck's own ground policy, not a
    /// fixed list this module keeps. An ngspice-dialect deck aliases `GND`;
    /// `0` is ground under every policy.
    #[test]
    fn the_decks_own_ground_policy_decides_which_names_are_node_zero() {
        let engine = Engine::new(SimulationConfig::default());
        let aliased = Netlist::parse(
            "Aliased ground\n\
             V1 in gnd DC 1\n\
             R1 in out 1k\n\
             R2 out gnd 1k\n\
             .op\n\
             .end\n",
        )
        .expect("deck parses");
        let resolver =
            NodeResolver::build_with_abort(&engine, &aliased, &NoAbort).expect("resolver builds");
        assert_eq!(
            aliased.ground_policy().canonical_node("GND"),
            "0",
            "this deck's policy aliases GND"
        );
        for name in ["GND", "gnd", "0"] {
            assert_eq!(
                resolver
                    .resolve(name, ".PZ reference")
                    .expect("ground alias"),
                0,
                "{name} is ground under this deck's policy"
            );
        }

        let plain = deck();
        let resolver =
            NodeResolver::build_with_abort(&engine, &plain, &NoAbort).expect("resolver builds");
        assert_eq!(resolver.resolve("0", ".PZ reference").expect("zero"), 0);
    }

    #[test]
    fn building_the_resolver_honours_its_abort_source() {
        let engine = Engine::new(SimulationConfig::default());
        let error = NodeResolver::build_with_abort(&engine, &deck(), &ImmediateAbort)
            .expect_err("an aborted elaboration must not produce a namespace");
        assert!(matches!(error, SimulationError::Aborted), "{error}");
    }
}
