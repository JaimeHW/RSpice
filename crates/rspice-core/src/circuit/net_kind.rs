//! What kind of quantity each circuit net carries.
//!
//! A net's kind is a property of the net, not of the analysis reading it: one
//! event-driven port makes its net discrete for the matrix assembler, the
//! shunt pass and the event scheduler at the same time. Keeping the answer
//! next to the node table means those consumers agree by construction instead
//! of each carrying its own node list.

use crate::NodeId;

/// The quantity a net carries.
///
/// `Discrete` currently covers every XSPICE event connection, digital and
/// real-valued alike. Separating the real-valued ones into their own variant
/// is an addition here; the matches that decide something per kind are written
/// without a wildcard arm so that addition surfaces as a compile error at each
/// of them rather than as a silent fallthrough.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) enum NetKind {
    /// An analog net whose voltage the MNA system solves for.
    #[default]
    Continuous,
    /// An event-driven net whose value lives in the event scheduler.
    Discrete,
}

/// Net kinds by node ID, with `Continuous` as the unrecorded default.
///
/// Only non-continuous nets take a slot, so a purely analog circuit carries an
/// empty table and every lookup answers from the default. The table is never
/// truncated to the node count: node IDs shift when a late ground reference is
/// chosen, and the recorded kinds keep the identities they were given.
#[derive(Debug, Clone, Default)]
pub(crate) struct NetKinds {
    by_node: Vec<NetKind>,
}

impl NetKinds {
    /// The kind of one net. Nodes with no recorded kind are continuous.
    #[inline]
    pub(crate) fn kind(&self, node: NodeId) -> NetKind {
        self.by_node.get(node).copied().unwrap_or_default()
    }

    /// Record a net's kind.
    ///
    /// Ground is the voltage reference rather than a net of its own, so it
    /// never takes a kind: a code model tying an event port to node `0` leaves
    /// the table untouched.
    pub(crate) fn set(&mut self, node: NodeId, kind: NetKind) {
        if node == 0 {
            return;
        }
        if node >= self.by_node.len() {
            self.by_node.resize(node + 1, NetKind::Continuous);
        }
        self.by_node[node] = kind;
    }

    /// Every discrete-valued net, in ascending node order.
    pub(crate) fn discrete_nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.by_node
            .iter()
            .enumerate()
            .filter_map(|(node, kind)| match kind {
                NetKind::Discrete => Some(node),
                NetKind::Continuous => None,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CircuitData, Engine, Netlist, SimulationConfig};

    fn build(deck: &str) -> CircuitData {
        let netlist = Netlist::parse(deck).expect("deck parses");
        Engine::new(SimulationConfig::default())
            .build_circuit(&netlist)
            .expect("circuit builds")
    }

    /// The derived view and the discriminant have to answer the same question,
    /// node by node, or a consumer's choice of route would change its answer.
    fn assert_views_agree(circuit: &CircuitData) {
        let discrete: Vec<NodeId> = circuit.net_kinds.discrete_nodes().collect();
        let rows: Vec<usize> = circuit.xspice_event_node_matrix_rows().collect();
        assert_eq!(
            rows,
            discrete.iter().map(|node| node - 1).collect::<Vec<usize>>()
        );
        for node in 0..=circuit.num_nodes() {
            assert_eq!(
                circuit.net_kinds.kind(node) == NetKind::Discrete,
                discrete.contains(&node),
                "node {node} disagrees between the discriminant and the view"
            );
            assert_eq!(
                circuit.is_discrete_net(node),
                discrete.contains(&node),
                "node {node} disagrees with the membership accessor"
            );
        }
        assert!(
            !discrete.contains(&0),
            "ground took a discrete kind; it is the voltage reference, not a net"
        );
    }

    #[test]
    fn every_net_of_a_pure_analog_deck_is_continuous() {
        let circuit = build(
            "analog only\n\
             V1 in 0 DC 1\n\
             R1 in mid 1k\n\
             C1 mid 0 1n\n\
             R2 mid 0 1k\n\
             .op\n\
             .end\n",
        );

        assert!(circuit.num_nodes() >= 2);
        for node in 0..=circuit.num_nodes() {
            assert_eq!(circuit.net_kinds.kind(node), NetKind::Continuous);
        }
        assert_eq!(circuit.net_kinds.discrete_nodes().count(), 0);
        assert_views_agree(&circuit);
    }

    #[test]
    fn event_nets_survive_circuit_build_as_discrete() {
        let circuit = build(
            "* an analog net bridged to a digital one and back\n\
             vin in 0 pulse(0 1 0 1p 1p 1n 2n)\n\
             r1 in 0 1k\n\
             a_adc [in] [dig] adc\n\
             .model adc adc_bridge (in_low=0.4 in_high=0.6)\n\
             a_dac [dig] [out] dac\n\
             .model dac dac_bridge (out_low=0 out_high=1)\n\
             r2 out 0 1k\n\
             .tran 100p 2n\n\
             .end\n",
        );

        let discrete: Vec<NodeId> = circuit.net_kinds.discrete_nodes().collect();
        let dig = circuit
            .get_node_by_name("dig")
            .expect("the deck names `dig`");
        assert_eq!(
            discrete,
            vec![dig],
            "only the bridged digital net is discrete"
        );

        // The analog nets either side of the bridges stay continuous.
        for name in ["in", "out"] {
            let node = circuit.get_node_by_name(name).expect("deck names the net");
            assert_eq!(circuit.net_kinds.kind(node), NetKind::Continuous);
        }
        assert_views_agree(&circuit);
    }

    #[test]
    fn recorded_kinds_read_back_and_ground_is_never_a_net() {
        let mut kinds = NetKinds::default();
        kinds.set(0, NetKind::Discrete);
        kinds.set(4, NetKind::Discrete);
        kinds.set(2, NetKind::Discrete);
        kinds.set(2, NetKind::Continuous);
        kinds.set(3, NetKind::Discrete);

        assert_eq!(kinds.kind(0), NetKind::Continuous);
        assert_eq!(kinds.kind(2), NetKind::Continuous);
        assert_eq!(kinds.kind(3), NetKind::Discrete);
        assert_eq!(kinds.kind(4), NetKind::Discrete);
        // Past the recorded range, and so continuous by default.
        assert_eq!(kinds.kind(99), NetKind::Continuous);
        assert_eq!(kinds.discrete_nodes().collect::<Vec<NodeId>>(), vec![3, 4]);
    }
}
