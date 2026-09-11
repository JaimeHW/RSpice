//! The event-value domains attached to circuit node identities.
//!
//! The legacy deck route uses one node number for an electrical node and its
//! converted event representations. Digital and real values already live in
//! separate scheduler maps; this table retains both domains instead of losing
//! their types when deciding which matrix rows also identify event nets.

use crate::NodeId;

/// Event representations attached to one deck node identity.
///
/// `DigitalAndReal` records two distinct event nets, not an implicit digital /
/// real conversion or one resolver for unlike values. The existing auto-bridge
/// route may connect both representations to the same electrical node. Keeping
/// both marks is order independent and preserves that route until the design
/// graph allocates separate event and electrical identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) enum NetKind {
    /// No event connection is registered for this node.
    #[default]
    Continuous,
    /// Four-state digital values with drive strength.
    Digital,
    /// Real-valued event data.
    Real,
    /// Separate digital and real event representations share this node ID.
    DigitalAndReal,
}

impl NetKind {
    pub(crate) fn is_discrete(self) -> bool {
        match self {
            Self::Continuous => false,
            Self::Digital | Self::Real | Self::DigitalAndReal => true,
        }
    }

    #[cfg(feature = "veriloga")]
    pub(crate) fn description(self) -> &'static str {
        match self {
            Self::Continuous => "electrical",
            Self::Digital => "four-state digital",
            Self::Real => "real-valued",
            Self::DigitalAndReal => "separate four-state digital and real-valued",
        }
    }

    fn union(self, other: Self) -> Self {
        match (self, other) {
            (Self::Continuous, kind) | (kind, Self::Continuous) => kind,
            (Self::Digital, Self::Digital) => Self::Digital,
            (Self::Real, Self::Real) => Self::Real,
            (Self::Digital, Self::Real)
            | (Self::Real, Self::Digital)
            | (Self::DigitalAndReal, _)
            | (_, Self::DigitalAndReal) => Self::DigitalAndReal,
        }
    }
}

/// Net kinds by node ID, with `Continuous` as the unrecorded default.
///
/// Only non-continuous nets take a slot, so a purely analog circuit carries an
/// empty table and every lookup answers from the default.
///
/// XSPICE connections and the circuit HDL graph own these identities. After
/// a ground remap, rebuilding the table from their updated node numbering
/// preserves every event kind and drops the former ground identity.
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

    /// Add an event representation without overwriting an earlier port's type.
    ///
    /// Ground is the voltage reference rather than a net of its own, so it
    /// never takes a kind: a code model tying an event port to node `0` leaves
    /// the table untouched.
    pub(crate) fn register(&mut self, node: NodeId, kind: NetKind) {
        if node == 0 {
            return;
        }
        if node >= self.by_node.len() {
            self.by_node.resize(node + 1, NetKind::Continuous);
        }
        self.by_node[node] = self.by_node[node].union(kind);
    }

    /// Every discrete-valued net, in ascending node order.
    pub(crate) fn discrete_nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.by_node
            .iter()
            .enumerate()
            .filter_map(|(node, kind)| match kind {
                NetKind::Digital | NetKind::Real | NetKind::DigitalAndReal => Some(node),
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
                circuit.net_kinds.kind(node).is_discrete(),
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

    /// A deck with no explicit node `0` gets its ground chosen after every
    /// element is in place, and that choice shifts every higher node ID down by
    /// one. The recorded kinds are keyed by node ID, so they have to follow the
    /// renumbering or they describe the wrong nets: before this was fixed the
    /// table marked the analog output net discrete and the digital net
    /// continuous, which is exactly backwards.
    #[test]
    fn event_net_identity_survives_a_late_auto_ground_remap() {
        let circuit = build(
            "* no explicit ground, so `ref` is auto-selected after elaboration\n\
             vin in ref pulse(0 1 0 1p 1p 1n 2n)\n\
             r1 in ref 1k\n\
             a_adc [in] [dig] adc\n\
             .model adc adc_bridge (in_low=0.4 in_high=0.6)\n\
             a_dac [dig] [out] dac\n\
             .model dac dac_bridge (out_low=0 out_high=1)\n\
             r2 out ref 1k\n\
             .tran 100p 2n\n\
             .end\n",
        );

        // `ref` became ground, so it is no longer a net of its own.
        assert_eq!(circuit.get_node_by_name("ref"), Some(0));
        let dig = circuit
            .get_node_by_name("dig")
            .expect("the deck names `dig`");
        let out = circuit
            .get_node_by_name("out")
            .expect("the deck names `out`");
        // The remap moved `dig` down onto the ID `out` used to hold, which is
        // what made a shifted table point at the wrong net.
        assert!(
            dig < out,
            "the remap is expected to renumber `dig` below `out`"
        );

        assert_eq!(
            circuit.net_kinds.discrete_nodes().collect::<Vec<NodeId>>(),
            vec![dig],
            "only the bridged digital net is discrete after the remap"
        );
        assert!(circuit.is_discrete_net(dig));
        assert!(!circuit.is_discrete_net(out));
        for name in ["in", "out"] {
            let node = circuit.get_node_by_name(name).expect("deck names the net");
            assert_eq!(circuit.net_kinds.kind(node), NetKind::Continuous);
        }
        assert_views_agree(&circuit);
    }

    #[test]
    fn typed_event_representations_survive_bridge_planning_and_ground_remap() {
        for ground in ["0", "ref"] {
            for reverse in [false, true] {
                let digital = "a_digital [mix] converted dtr";
                let real = "a_real mix observed rg";
                let cards = if reverse {
                    format!("{real}\n{digital}")
                } else {
                    format!("{digital}\n{real}")
                };
                let circuit = build(&format!(
                    "* distinct event representations of one loaded electrical node\n\
                     v1 mix {ground} dc 1.5\n\
                     r1 mix {ground} 1k\n\
                     {cards}\n\
                     .model dtr d_to_real\n\
                     .model rg real_gain\n\
                     .end\n"
                ));
                assert_eq!(circuit.get_node_by_name(ground), Some(0));
                for (name, expected) in [
                    ("mix", NetKind::DigitalAndReal),
                    ("converted", NetKind::Real),
                    ("observed", NetKind::Real),
                ] {
                    let node = circuit.get_node_by_name(name).unwrap();
                    assert_eq!(circuit.net_kinds.kind(node), expected, "{name}");
                }
                // Each input representation still gets its own physical
                // conversion. Neither registration order overwrites the other.
                for model in ["adc_bridge", "v_to_real"] {
                    assert_eq!(
                        circuit
                            .xspice_instances
                            .iter()
                            .filter(|instance| instance.model_name() == model)
                            .count(),
                        1
                    );
                }
                assert_views_agree(&circuit);
            }
        }
    }

    #[test]
    fn recorded_kinds_read_back_and_ground_is_never_a_net() {
        let mut kinds = NetKinds::default();
        kinds.register(0, NetKind::Digital);
        kinds.register(4, NetKind::Digital);
        kinds.register(2, NetKind::Digital);
        kinds.register(2, NetKind::Real);
        kinds.register(3, NetKind::Digital);

        assert_eq!(kinds.kind(0), NetKind::Continuous);
        assert_eq!(kinds.kind(2), NetKind::DigitalAndReal);
        assert_eq!(kinds.kind(3), NetKind::Digital);
        assert_eq!(kinds.kind(4), NetKind::Digital);
        // Past the recorded range, and so continuous by default.
        assert_eq!(kinds.kind(99), NetKind::Continuous);
        assert_eq!(
            kinds.discrete_nodes().collect::<Vec<NodeId>>(),
            vec![2, 3, 4]
        );
    }
}
