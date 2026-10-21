//! The circuit store as the topology a generated Verilog-A instance is built
//! into.
//!
//! `device::veriloga_builtins` names the nets, branches and private-node
//! bookkeeping it needs through `BuiltinTopology` and never the store itself,
//! so `device` stays below `circuit`. Each operation forwards to the store's
//! own method or field.

use super::CircuitData;
use crate::NodeId;
use crate::device::veriloga_builtins::BuiltinTopology;
use rspice_veriloga_runtime::GeneratedSimulationParameters;

impl BuiltinTopology for CircuitData {
    fn get_or_create_node(&mut self, name: &str) -> NodeId {
        CircuitData::get_or_create_node(self, name)
    }

    fn allocate_branch(&mut self) -> NodeId {
        CircuitData::allocate_branch(self)
    }

    fn record_compact_model_stamp_reach(
        &mut self,
        private: &[NodeId],
        dc_rows: impl IntoIterator<Item = NodeId>,
        read: impl IntoIterator<Item = NodeId>,
    ) {
        CircuitData::record_compact_model_stamp_reach(self, private, dc_rows, read);
    }

    fn add_compact_model_private_nodes(&mut self, nodes: impl IntoIterator<Item = NodeId>) {
        self.compact_model_private_nodes.extend(nodes);
    }

    fn generated_simulation_parameters(&self) -> &GeneratedSimulationParameters {
        &self.generated_simulation_parameters
    }
}

#[cfg(all(test, feature = "veriloga-model-vbic13-4t"))]
mod tests {
    use super::CircuitData;
    use crate::device::veriloga_builtins::{builtins, instantiate_builtin_scoped};

    #[test]
    fn optional_internal_connections_preserve_generated_node_slots() {
        let optional = ["dt", "cx", "ci", "bx", "bi", "ei"];
        for count in 0..=optional.len() {
            let mut circuit = CircuitData::new();
            let mut terminals = ["c", "b", "e", "0"].map(str::to_string).to_vec();
            terminals.extend((0..count).map(|index| {
                if index == 2 {
                    "0".to_string()
                } else {
                    format!("exposed_{index}")
                }
            }));
            let instance = instantiate_builtin_scoped(
                "vbic13_4t",
                "Q1",
                &terminals,
                &[],
                &crate::netlist::ParamContext::new(),
                &mut circuit,
                &optional,
                4,
            )
            .unwrap()
            .unwrap();
            let descriptor = builtins::descriptor("vbic13_4t").unwrap();
            assert_eq!(instance.nodes.len(), descriptor.total_node_count);
            assert_eq!(
                instance.terminal_currents().len(),
                descriptor.terminals.len()
            );
            for (name, node) in instance.internal_nodes() {
                let internal_name = format!("Q1.__{name}.internal");
                let position = optional.iter().position(|candidate| *candidate == name);
                if let Some(index) = position.filter(|index| *index < count) {
                    let expected = if index == 2 {
                        0
                    } else {
                        circuit
                            .get_node_by_name(&format!("exposed_{index}"))
                            .unwrap()
                    };
                    assert_eq!(
                        node, expected,
                        "binding {name} with {count} optional connections"
                    );
                    assert!(circuit.get_node_by_name(&internal_name).is_none());
                } else {
                    assert_eq!(Some(node), circuit.get_node_by_name(&internal_name));
                }
            }
        }
    }

    #[test]
    fn invalid_optional_connections_fail_before_allocating_nodes() {
        for (optional, count) in [
            (&["dt", "dt"][..], 4),
            (&["not_a_model_node"][..], 4),
            (&["dt"][..], 6),
            (&[][..], 5),
            (&["dt"][..], 3),
        ] {
            let mut circuit = CircuitData::new();
            let initial_nodes = circuit.num_nodes();
            let terminals = (0..count)
                .map(|index| format!("n{index}"))
                .collect::<Vec<_>>();
            let error = instantiate_builtin_scoped(
                "vbic13_4t",
                "Q1",
                &terminals,
                &[],
                &crate::netlist::ParamContext::new(),
                &mut circuit,
                optional,
                count,
            )
            .unwrap_err();
            assert!(error.to_string().contains("vbic13_4t"));
            assert_eq!(circuit.num_nodes(), initial_nodes);
        }
    }
}
