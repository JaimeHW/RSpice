//! DC stamping functions
//!
//! Functions for stamping DC values into the matrix and RHS vector.

use super::{Engine, SimulationError};
use crate::solver::{ComplexMatrix, StaticMatrix};
use crate::{CircuitData, NodeId, Value};

impl Engine {
    /// Add a numerical conditioning term to every MNA equation except private
    /// non-electrical DAE state rows. Unlike circuit GMIN, this intentionally
    /// covers branch-current equations so inductive and controlled-source
    /// systems retain the conditioning contract used by continuation/startup.
    pub(in crate::engine) fn stamp_matrix_conditioning_diagonal(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        matrix_size: usize,
        value: Value,
    ) {
        if value == 0.0 {
            return;
        }
        let node_count = circuit.num_nodes().min(matrix_size);
        for index in 0..matrix_size {
            if index >= node_count || !circuit.is_non_electrical_state_matrix_index(index) {
                matrix.add(index, index, value);
            }
        }
    }

    /// [`Self::stamp_matrix_conditioning_diagonal`] on the node rows alone.
    ///
    /// A pseudo-transient stage projects every ideal source's nodes back onto
    /// the source's value after each iterate. A conditioning term on a branch
    /// row turns `V = E` into `V = E - value * I`, which the projection then
    /// breaks by `value * I`, and the stage's pseudo-capacitors draw amperes
    /// through a supply: the KCL error that leaves beside the supply exceeds
    /// the residual tolerance, at every stage. The stage anchor leaves branch
    /// rows out for the same reason.
    pub(in crate::engine) fn stamp_node_conditioning_diagonal(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        matrix_size: usize,
        value: Value,
    ) {
        if value == 0.0 {
            return;
        }
        for index in 0..circuit.num_nodes().min(matrix_size) {
            if !circuit.is_non_electrical_state_matrix_index(index) {
                matrix.add(index, index, value);
            }
        }
    }

    /// Add the nodal GMIN floor to the node-voltage equations that take it
    /// ([`CircuitData::receives_nodal_gmin`]). Private DAE state rows share the
    /// matrix's nodal prefix but are not voltages, and a compact model's
    /// private nodes are the model's own; neither receives circuit GMIN.
    pub(in crate::engine) fn stamp_nodal_gmin(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        gmin: Value,
    ) {
        if gmin == 0.0 {
            // The floor is what closes a pure event net's placeholder row at
            // DC, so the attempt without one has to close it instead. Without
            // it the row is empty, and the system would be singular for a net
            // no analog equation can reach. Any other assembly at a zero floor
            // pins these rows itself.
            if circuit.zero_dc_nodal_floor {
                for row in circuit.xspice_event_node_matrix_rows() {
                    if circuit.event_only_net_kind(row + 1).is_some() {
                        matrix.add(row, row, 1.0);
                    }
                }
            }
            Self::stamp_settled_node_gmin(circuit, matrix);
            Self::stamp_held_node_gmin(circuit, matrix);
            return;
        }
        for index in 0..circuit.num_nodes() {
            if circuit.receives_nodal_gmin(index) {
                matrix.add(index, index, gmin);
            }
        }
        circuit.stamp_resistive_island_floor(matrix, gmin);
    }

    /// Install GMIN on each settled compact-model node alone
    /// ([`Self::settle_compact_model_private_nodes`]) in an assembly that
    /// stamps no nodal floor.
    ///
    /// A settled node takes the floor like any other node, and no equation
    /// reaches it, so without the floor its row was empty: every operating
    /// point solved without the floor first was singular there and went back
    /// to the floor whole, moving every node only leakage holds. Spectre
    /// installs GMIN on the node that has no DC path alone ("Gmin installed
    /// to provide path"), which keeps the node at the 0 V its balanced flows
    /// give it and leaves the rest of the circuit without a floor.
    fn stamp_settled_node_gmin(circuit: &CircuitData, matrix: &mut StaticMatrix) {
        let Some(settled) = &circuit.settled_compact_model_private_nodes else {
            return;
        };
        if circuit.settled_node_gmin == 0.0 {
            return;
        }
        for &node in settled {
            matrix.add(node - 1, node - 1, circuit.settled_node_gmin);
        }
    }

    /// Install GMIN on each node the floor-free operating point found with
    /// no DC path ([`CircuitData::dc_floor_held_nodes`]), alone, in an
    /// assembly that stamps no nodal floor: the conductance Spectre installs
    /// "to provide path" on such a node, which leaves every other node
    /// without a floor.
    fn stamp_held_node_gmin(circuit: &CircuitData, matrix: &mut StaticMatrix) {
        if circuit.settled_node_gmin == 0.0 {
            return;
        }
        for &row in &circuit.dc_floor_held_nodes {
            matrix.add(row, row, circuit.settled_node_gmin);
        }
    }

    /// The diagonal a settled node carries in an assembly at nodal floor
    /// `floor`: the floor, or the GMIN installed on it where there is none.
    fn settled_node_diagonal(circuit: &CircuitData, floor: Value) -> Value {
        if floor == 0.0 {
            circuit.settled_node_gmin
        } else {
            floor
        }
    }

    /// Add a homotopy GMIN stage `gmin` whose final floor is `floor`: the
    /// nodes that take the floor take the whole stage, as
    /// [`Self::stamp_nodal_gmin`] stamps it, and a compact model's private
    /// node takes the part of the stage above the floor
    /// ([`CircuitData::receives_homotopy_gmin`]). At the floor itself the
    /// private nodes take nothing, so the last stage is the circuit's own
    /// system.
    pub(in crate::engine) fn stamp_nodal_gmin_stage(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        gmin: Value,
        floor: Value,
    ) {
        Self::stamp_nodal_gmin(circuit, matrix, gmin);
        let above_floor = gmin - floor.max(0.0);
        if above_floor <= 0.0 || circuit.compact_model_private_nodes.is_empty() {
            return;
        }
        for index in 0..circuit.num_nodes() {
            if circuit.is_compact_model_private_node(index) && circuit.receives_homotopy_gmin(index)
            {
                matrix.add(index, index, above_floor);
            }
        }
    }

    /// Final DC nodal gmin floor.
    ///
    /// B3SOI floating-body currents can be in the e-18 A range at a valid DC
    /// operating point. Xyce/ngspice apply their BSIMSOI GMIN inside the device
    /// load; adding a second simulator-level final shunt visibly deforms those
    /// roots. Keep explicit homotopy GMIN steps unchanged, but solve the final
    /// native BSIM3-SOI DC system without an extra nodal floor. An authored
    /// global RSHUNT likewise makes every electrical nodal diagonal physical;
    /// retaining a parallel numerical floor would systematically perturb the
    /// user-selected resistance even though it is no longer needed for nodal
    /// rank regularization. An enforced winding IC must also satisfy its
    /// fixed-current network without a numerical shunt absorbing a mismatch.
    pub(in crate::engine) fn dc_nodal_gmin_floor(&self, circuit: &CircuitData) -> Value {
        let gmin = self.config.convergence_config.gmin_target.max(0.0);
        if circuit.zero_dc_nodal_floor
            || circuit.has_b3soi_devices()
            || circuit.has_global_shunt()
            || circuit.inductors.has_dc_initial_conditions()
        {
            0.0
        } else {
            gmin
        }
    }

    /// A prescribed winding current removes the inductor's DC short. The
    /// remaining network must determine a finite voltage without a numerical
    /// shunt absorbing a mismatch between fixed currents. Continuation may
    /// still regularize trial systems; the accepted constrained bias may not.
    pub(in crate::engine) fn transient_operating_point_nodal_gmin_floor(
        &self,
        circuit: &CircuitData,
    ) -> Value {
        if circuit.inductors.has_operating_point_initial_conditions() {
            0.0
        } else {
            self.dc_nodal_gmin_floor(circuit)
        }
    }

    /// The event rows an assembly must pin: rows of pure event nets that no
    /// analog stamp reached. Every analysis shares this rule so that a DC,
    /// transient and small-signal assembly of the same circuit agree on which
    /// placeholder rows exist.
    ///
    /// `is_deficient` is the assembled matrix's own emptiness report for one
    /// row, so a row an analog device did stamp is never a candidate. It is
    /// asked of each event row just before that row is visited.
    pub(in crate::engine) fn visit_unconstrained_xspice_event_rows(
        circuit: &CircuitData,
        is_deficient: impl Fn(usize) -> bool,
        mut pin: impl FnMut(usize),
    ) {
        for row in circuit.xspice_event_node_matrix_rows() {
            let deficient = is_deficient(row);
            Self::debug_assert_event_row_unstamped(circuit, row, deficient);
            if deficient {
                pin(row);
            }
        }
    }

    /// The result namespace drops the voltage of every net
    /// `event_only_net_kind` claims, on the grounds that its row is one of
    /// these placeholders. If such a row carried an analog stamp after all,
    /// the two would disagree: an author would lose `V()` of a net the solver
    /// really does solve. The `.OPTIONS RSHUNT` shunt is the one stamp that
    /// reaches every row by construction and is not a claim about this net,
    /// so it is excluded rather than reported.
    #[inline]
    fn debug_assert_event_row_unstamped(circuit: &CircuitData, row: usize, deficient: bool) {
        debug_assert!(
            deficient
                || circuit.global_shunt_conductance != 0.0
                || circuit.event_only_net_kind(row + 1).is_none(),
            "row {row} belongs to a net classified event-only but carries an analog \
             stamp; the pinned placeholder rows and the published analog namespace \
             have drifted apart"
        );
    }

    /// Pin only numerically empty event rows that carry no analog RHS.
    ///
    /// Event values live in the event runtimes, but their stable node IDs
    /// share the circuit node namespace. A pure event net therefore owns an
    /// otherwise empty MNA placeholder row. Pinning that placeholder to zero
    /// restores matrix rank without adding conductance to any physical row or
    /// hiding an analog source connected to an event net.
    ///
    /// Each event row's emptiness is read from that row alone, just before it
    /// is pinned. Pinning writes only its own diagonal, so every row reads as
    /// a sweep of the whole matrix taken before the first pin would, without
    /// paying for that sweep at every assembly.
    pub(in crate::engine) fn pin_unconstrained_xspice_event_rows(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
    ) {
        for row in circuit.xspice_event_node_matrix_rows() {
            let deficient = matrix.row_is_deficient(row);
            Self::debug_assert_event_row_unstamped(circuit, row, deficient);
            if deficient && rhs.get(row).is_some_and(|value| *value == 0.0) {
                matrix.add(row, row, 1.0);
                rhs[row] = 0.0;
            }
        }
    }

    /// Settle, at the first operating-point assembly, which of a compact
    /// model's own nodes no equation reaches, and refuse one that is left
    /// without a DC path.
    ///
    /// A compact model's private node takes no final GMIN floor
    /// ([`CircuitData::receives_nodal_gmin`]). The model may still leave
    /// one's voltage out of every equation: BSIM-SOI 4.7 with `tnodeout=1`
    /// and six terminals ties `b` as its temperature node and never drives
    /// its unconnected `t`, whose row holds only the incidence of a thermal
    /// branch that carries no flow. A node whose column and right-hand side
    /// are empty is the twin of a pure event net's placeholder row
    /// ([`Self::pin_unconstrained_xspice_event_rows`]): no other unknown
    /// depends on its voltage, so a diagonal there cannot move any of them,
    /// and it decides only the node's own voltage, from the flows the model
    /// sends into it. Such a node takes the floor again, which reads 0 V
    /// when those flows balance, as before the model's nodes lost the floor;
    /// this is Spectre's practice for a node with no DC path ("Gmin installed
    /// to provide path"). The floor rather than an identity row keeps the
    /// node's transient equation as it was, should the model reach it only
    /// through `ddt()`. A node the model reaches only through `ddt()` is
    /// therefore settled too, where strictly it has no DC operating point.
    ///
    /// A private node whose row is empty while its column or its right-hand
    /// side is not has no DC path either, but its voltage reaches other
    /// equations, so no value is canonical: the operating point refuses and
    /// names it, as it refuses a deck node with no DC path. So does one
    /// whose column is empty but whose right-hand side is not, a current the
    /// model drives into a node it gives no path.
    ///
    /// A refusal holds only where the emptiness is structural: no equation
    /// of the model that is not a sum of `ddt()` terms has a stamp location
    /// in the row ([`CircuitData::compact_model_dc_driven_nodes`]), or none
    /// reads the column ([`CircuitData::compact_model_read_nodes`]). A row a
    /// conditional stamp leaves empty at this bias alone, `I(x) <+ (V(c) >
    /// 0.5) ? V(x) / R : 0` at `V(c) = 0`, is neither settled nor refused: the
    /// solve goes on, and the GMIN continuation carries it to the bias where
    /// the stamp holds.
    pub(in crate::engine) fn settle_compact_model_private_nodes(
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &[Value],
        floor: Value,
    ) -> Result<(), SimulationError> {
        if circuit.settled_compact_model_private_nodes.is_some() {
            return Self::release_restamped_private_nodes(circuit, matrix, rhs, floor);
        }
        circuit.floating_compact_model_islands.clear();
        circuit.settled_compact_model_terminal_nets.clear();
        let mut settled = Vec::new();
        let mut nodes: Vec<NodeId> = circuit
            .compact_model_private_nodes
            .iter()
            .copied()
            .filter(|&node| {
                let index = node - 1;
                circuit.receives_homotopy_gmin(index)
                    && (matrix.row_is_deficient(index) || matrix.column_is_deficient(index))
            })
            .collect();
        nodes.sort_unstable();
        for node in nodes {
            let index = node - 1;
            let unread = matrix.column_is_deficient(index);
            if unread && rhs.get(index).is_some_and(|&value| value == 0.0) {
                circuit.compact_model_private_nodes.remove(&node);
                settled.push(node);
                let diagonal = Self::settled_node_diagonal(circuit, floor);
                if diagonal != 0.0 {
                    matrix.add(index, index, diagonal);
                }
                continue;
            }
            // Empty here is a fact about this bias only. A refusal needs the
            // emptiness to hold at every bias: no stamp location of the model
            // reads the node, or none drives its row at DC.
            let structurally_empty = if unread {
                !circuit.compact_model_read_nodes.contains(&node)
            } else {
                !circuit.compact_model_dc_driven_nodes.contains(&node)
            };
            if structurally_empty {
                circuit.settled_compact_model_private_nodes = Some(settled);
                return Err(Self::private_node_without_dc_path(circuit, node, unread));
            }
        }
        Self::settle_floating_compact_model_islands(circuit, matrix, floor, &mut settled);
        Self::settle_unconnected_terminal_nets(circuit, matrix, rhs, floor, &mut settled);
        circuit.settled_compact_model_private_nodes = Some(settled);
        Ok(())
    }

    /// Settle each sub-network of a compact model's private nodes that no
    /// DC equation ties to the rest of the circuit or to ground, holding
    /// every node of it through GMIN.
    ///
    /// VBIC 4T et (`vbic_4T_et_cf.va`) reads a zero series resistance as an
    /// open branch, so its default card leaves `cx`, `ci`, `bx`, `bi`, `ei`,
    /// `bp` and `si` joined to one another through its junctions and to
    /// nothing else. Each row of such a sub-network has entries, so no single
    /// node settles, yet every current in it is a function of its own
    /// branch voltages: its block of the system carries the common mode in
    /// its null space, its rows sum to zero, and the operating point is
    /// singular. Spectre installs GMIN on a node with no DC path ("Gmin
    /// installed to provide path") at setup, so every analysis sees it; the
    /// sub-network then reads the 0 V its balanced flows give it, and the
    /// post-condition ([`Self::check_settled_compact_model_private_nodes`])
    /// refuses one whose flows do not balance. Xyce warns for a netlist node
    /// with no DC path and leaves a device's internal nodes to the device.
    ///
    /// A sub-network qualifies on this assembly's values: two or more private
    /// nodes joined by nonzero entries, no nonzero entry between any of them
    /// and a node outside, and every row summing to zero within rounding (a
    /// ground conductance on any node leaves its row sum at that
    /// conductance). A single private node is the caller's: it settles only
    /// with an empty row or column. A stamp that later ties a sub-network out
    /// releases it ([`Self::release_restamped_private_nodes`]).
    fn settle_floating_compact_model_islands(
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        floor: Value,
        settled: &mut Vec<NodeId>,
    ) {
        let size = circuit.num_nodes().min(circuit.matrix_size());
        let mut members: Vec<usize> = circuit
            .compact_model_private_nodes
            .iter()
            .map(|&node| node - 1)
            .filter(|&index| index < size && circuit.receives_homotopy_gmin(index))
            .collect();
        if members.len() < 2 {
            return;
        }
        members.sort_unstable();
        let islands = Self::floating_islands_among(matrix, size, &members, |_| 0.0);
        if islands.is_empty() {
            return;
        }
        let diagonal = Self::settled_node_diagonal(circuit, floor);
        let node_names = circuit.node_names_sorted();
        for island in islands {
            let names: Vec<&str> = island
                .iter()
                .map(|&index| node_names.get(index).map_or("?", String::as_str))
                .collect();
            log::info!(
                "Nodes {} of a compact model have no DC path to the rest of the circuit; \
                 holding each through GMIN = {:e} S.",
                names.join(", "),
                circuit.settled_node_gmin
            );
            let nodes: Vec<NodeId> = island.iter().map(|&index| index + 1).collect();
            for &node in &nodes {
                circuit.compact_model_private_nodes.remove(&node);
                settled.push(node);
                if diagonal != 0.0 {
                    matrix.add(node - 1, node - 1, diagonal);
                }
            }
            circuit.floating_compact_model_islands.push(nodes);
        }
    }

    /// The sub-networks of `members` (sorted zero-based rows, all below
    /// `size`) that this assembly's values leave without a DC path: joined
    /// by nonzero entries, with no nonzero entry to a row or column outside
    /// them, and each row summing to `diagonal(row)` within rounding, where
    /// `diagonal` is what the caller itself installed on that row. Each has
    /// two or more rows, sorted.
    fn floating_islands_among(
        matrix: &StaticMatrix,
        size: usize,
        members: &[usize],
        diagonal: impl Fn(usize) -> Value,
    ) -> Vec<Vec<usize>> {
        fn root(parent: &mut [usize], mut slot: usize) -> usize {
            while parent[slot] != slot {
                parent[slot] = parent[parent[slot]];
                slot = parent[slot];
            }
            slot
        }
        const OUTSIDE: usize = usize::MAX;
        let mut slot_of = vec![OUTSIDE; size];
        for (slot, &row) in members.iter().enumerate() {
            slot_of[row] = slot;
        }
        let slot = |index: usize| slot_of.get(index).copied().unwrap_or(OUTSIDE);
        let mut parent: Vec<usize> = (0..members.len()).collect();
        let mut tied_out = vec![false; members.len()];
        // NaN compares unequal to zero, so it ties as any other value does.
        for (row, col, value) in matrix.stored_entries() {
            if value == 0.0 {
                continue;
            }
            match (slot(row), slot(col)) {
                (OUTSIDE, OUTSIDE) => {}
                (a, OUTSIDE) => tied_out[a] = true,
                (OUTSIDE, b) => tied_out[b] = true,
                (a, b) => {
                    let (a, b) = (root(&mut parent, a), root(&mut parent, b));
                    if a != b {
                        parent[a] = b;
                    }
                }
            }
        }
        let mut sum = vec![0.0; members.len()];
        let mut magnitude = vec![0.0; members.len()];
        for (row, col, value) in matrix.stored_entries() {
            let (a, b) = (slot(row), slot(col));
            if a != OUTSIDE && b != OUTSIDE {
                sum[a] += value;
                magnitude[a] += value.abs();
            }
        }
        // A row whose entries cancel to the last bits of their magnitude
        // sums to zero; one that carries a ground conductance sums to it.
        let tolerance = Value::EPSILON.sqrt();
        let mut groups: std::collections::BTreeMap<usize, (Vec<usize>, bool)> =
            std::collections::BTreeMap::new();
        for (slot, &row) in members.iter().enumerate() {
            let group = groups
                .entry(root(&mut parent, slot))
                .or_insert_with(|| (Vec::new(), true));
            group.0.push(row);
            let balanced = (sum[slot] - diagonal(row)).abs()
                <= tolerance * magnitude[slot].max(Value::MIN_POSITIVE);
            group.1 &= !tied_out[slot] && balanced;
        }
        groups
            .into_values()
            .filter(|(rows, floating)| *floating && rows.len() >= 2)
            .map(|(rows, _)| rows)
            .collect()
    }

    /// Settle each compact model's terminal net that no equation reaches in
    /// this assembly: an empty row, column and right-hand side, on a deck
    /// node the DC ground-path walk has not already held
    /// ([`Self::hold_nodes_without_dc_path`]).
    ///
    /// VBIC 4T et's thermal terminal `dt` carries `I(rth) <+ V(rth) / RTH`
    /// only when `RTH > 0`; the default card's `RTH = 0` contributes zero,
    /// and with no power in the device the net on that terminal has no
    /// equation. The walk cannot see that, since a generated model's
    /// terminals conduct for it. The net takes GMIN alone, as Spectre's
    /// "Gmin installed to provide path" does, and the post-condition refuses
    /// it if a current the model later drives into it finds no path. A stamp
    /// that later reaches it releases it to the deck's nodes.
    fn settle_unconnected_terminal_nets(
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &[Value],
        floor: Value,
        settled: &mut Vec<NodeId>,
    ) {
        #[allow(unused_mut)]
        let mut nets: Vec<NodeId> = Vec::new();
        #[cfg(feature = "veriloga-builtins-base")]
        for instance in circuit.generated_veriloga_devices().iter() {
            let terminals = instance.terminal_currents().len().min(instance.nodes.len());
            nets.extend(&instance.nodes[..terminals]);
        }
        #[cfg(feature = "veriloga")]
        for device in circuit.veriloga_devices().iter() {
            nets.extend(
                (0..device.num_terminals()).map(|terminal| device.node_for_terminal(terminal)),
            );
        }
        if nets.is_empty() {
            return;
        }
        nets.retain(|&node| {
            let index = node.wrapping_sub(1);
            node > 0
                && index < circuit.num_nodes().min(circuit.matrix_size())
                && circuit.receives_nodal_gmin(index)
                && circuit.event_only_net_kind(node).is_none()
                && circuit
                    .dc_floating_component_for_matrix_row(index)
                    .is_none()
                && matrix.row_is_deficient(index)
                && matrix.column_is_deficient(index)
                && rhs.get(index).is_some_and(|&value| value == 0.0)
        });
        nets.sort_unstable();
        nets.dedup();
        let diagonal = Self::settled_node_diagonal(circuit, floor);
        for node in nets {
            log::info!(
                "Node {} on a compact model's terminal has no DC path; holding it alone \
                 through GMIN = {:e} S.",
                Self::compact_model_node_name(circuit, node),
                circuit.settled_node_gmin
            );
            settled.push(node);
            circuit.settled_compact_model_terminal_nets.push(node);
            if diagonal != 0.0 {
                matrix.add(node - 1, node - 1, diagonal);
            }
        }
    }

    /// Every operating-point assembly after the one that settled them: a
    /// settled node whose column or right-hand side a stamp now writes is no
    /// longer unconnected.
    ///
    /// The first assembly reads one bias. A stamp the model writes only
    /// under `if (V(c) > 0.5)` is absent at `V(c) = 0`, so the node settled
    /// there, and the floor then decides its voltage: 0 V, whatever the stamp
    /// says once it appears. Where the node then has an equation and a
    /// reader, its row and its column, it is the model's own again:
    /// it leaves the settled set, gives back the floor in this assembly and
    /// every later one, and this assembly's solve is the system with the
    /// stamp. Where it has only one of them, no value is canonical, and the
    /// operating point refuses, naming the node.
    ///
    /// Only settled nodes are read, and only the locations already stamped
    /// in this assembly; a circuit with none settled pays nothing.
    fn release_restamped_private_nodes(
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &[Value],
        floor: Value,
    ) -> Result<(), SimulationError> {
        let diagonal = Self::settled_node_diagonal(circuit, floor);
        let Some(mut settled) = circuit.settled_compact_model_private_nodes.take() else {
            return Ok(());
        };
        if settled.is_empty() {
            circuit.settled_compact_model_private_nodes = Some(settled);
            return Ok(());
        }
        let island_nodes = Self::release_tied_islands(circuit, matrix, diagonal, &mut settled);
        let mut released = Vec::new();
        let mut refused = None;
        settled.retain(|&node| {
            if refused.is_some() || island_nodes.contains(&node) {
                return true;
            }
            // The node settled on an empty column and right-hand side; its
            // row may hold what it held then, a branch incidence with no
            // flow, as BSIM-SOI's unconnected thermal node does.
            let index = node - 1;
            let column_quiet = matrix.column_is_deficient_beyond_diagonal(index, diagonal);
            if column_quiet && rhs.get(index).is_none_or(|&value| value == 0.0) {
                return true;
            }
            if column_quiet || matrix.row_is_deficient_beyond_diagonal(index, diagonal) {
                refused = Some(node);
                return true;
            }
            released.push(node);
            false
        });
        circuit.settled_compact_model_private_nodes = Some(settled);
        for &node in &released {
            // A terminal net goes back to the deck's nodes, which take the
            // floor; any other node is the model's own again.
            if let Some(position) = circuit
                .settled_compact_model_terminal_nets
                .iter()
                .position(|&net| net == node)
            {
                circuit
                    .settled_compact_model_terminal_nets
                    .swap_remove(position);
            } else {
                circuit.compact_model_private_nodes.insert(node);
            }
            if diagonal != 0.0 {
                matrix.add(node - 1, node - 1, -diagonal);
            }
        }
        match refused {
            Some(node) => Err(SimulationError::Circuit(format!(
                "a stamp on {} appeared after it was pinned as unconnected",
                Self::compact_model_node_name(circuit, node)
            ))),
            None => Ok(()),
        }
    }

    /// Release each held sub-network
    /// ([`Self::settle_floating_compact_model_islands`]) that a stamp in this
    /// assembly ties to a node outside it or to ground: its nodes are the
    /// model's own again and give back the GMIN this assembly installed.
    /// Returns the nodes of the sub-networks still held. A circuit with none
    /// held pays nothing.
    fn release_tied_islands(
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        diagonal: Value,
        settled: &mut Vec<NodeId>,
    ) -> std::collections::HashSet<NodeId> {
        if circuit.floating_compact_model_islands.is_empty() {
            return std::collections::HashSet::new();
        }
        let size = circuit.num_nodes().min(circuit.matrix_size());
        let mut members: Vec<usize> = circuit
            .floating_compact_model_islands
            .iter()
            .flatten()
            .map(|&node| node - 1)
            .collect();
        members.sort_unstable();
        let floating: std::collections::HashSet<NodeId> =
            Self::floating_islands_among(matrix, size, &members, |_| diagonal)
                .into_iter()
                .flatten()
                .map(|index| index + 1)
                .collect();
        let (held, tied): (Vec<_>, Vec<_>) =
            std::mem::take(&mut circuit.floating_compact_model_islands)
                .into_iter()
                .partition(|island| island.iter().all(|node| floating.contains(node)));
        for node in tied.into_iter().flatten() {
            settled.retain(|&settled_node| settled_node != node);
            circuit.compact_model_private_nodes.insert(node);
            if diagonal != 0.0 {
                matrix.add(node - 1, node - 1, -diagonal);
            }
        }
        let held_nodes = held.iter().flatten().copied().collect();
        circuit.floating_compact_model_islands = held;
        held_nodes
    }

    /// The post-condition of [`Self::settle_compact_model_private_nodes`]: a
    /// settled node reads 0 V within the voltage tolerance, so the flows the
    /// model sends into it balance. One that reads anything else is the
    /// floor's answer for a current with no path, `V = -I / g`, and refuses.
    pub(in crate::engine) fn check_settled_compact_model_private_nodes(
        &self,
        circuit: &CircuitData,
        solution: &[Value],
    ) -> Result<(), SimulationError> {
        let tolerance = self.config.convergence_config.voltage_abstol;
        let Some(settled) = &circuit.settled_compact_model_private_nodes else {
            return Ok(());
        };
        match settled.iter().copied().find(|&node| {
            solution
                .get(node - 1)
                .is_some_and(|value| !value.is_finite() || value.abs() > tolerance)
        }) {
            Some(node)
                if circuit
                    .floating_compact_model_islands
                    .iter()
                    .any(|island| island.contains(&node)) =>
            {
                Err(SimulationError::Circuit(format!(
                    "node {} has no DC path: it belongs to a sub-network of its compact model \
                     that nothing ties to the rest of the circuit, and only GMIN would set its \
                     voltages",
                    Self::compact_model_node_name(circuit, node)
                )))
            }
            Some(node) => Err(Self::private_node_without_dc_path(circuit, node, true)),
            None => Ok(()),
        }
    }

    fn private_node_without_dc_path(
        circuit: &CircuitData,
        node: NodeId,
        unread: bool,
    ) -> SimulationError {
        let name = Self::compact_model_node_name(circuit, node);
        let reason = if unread {
            "its compact model drives a current into it and gives that current no path"
        } else {
            "no equation of its compact model constrains it at DC, yet its voltage reaches \
             the model's other equations"
        };
        SimulationError::Circuit(format!("node {name} has no DC path: {reason}"))
    }

    fn compact_model_node_name(circuit: &CircuitData, node: NodeId) -> String {
        circuit
            .node_names_sorted()
            .get(node - 1)
            .cloned()
            .unwrap_or_else(|| format!("#{node}"))
    }

    /// The small-signal twin of [`Engine::pin_unconstrained_xspice_event_rows`].
    ///
    /// A pure event net carries no small signal: its value is frozen at the
    /// operating point the event runtimes settled, exactly as Spectre and
    /// ngspice freeze a digital net through an `.ac` sweep. Every small-signal
    /// assembly therefore closes an otherwise empty event placeholder row with
    /// the real identity equation `v = 0`, which restores rank without
    /// contributing conductance or susceptance to any physical row.
    ///
    /// The stamp is deliberately real: pole-zero analysis reads the dynamic
    /// descriptor out of the imaginary part alone, so a pinned row stays a
    /// pure algebraic constraint and introduces no spurious pole.
    pub(in crate::engine) fn pin_unconstrained_xspice_event_rows_small_signal(
        circuit: &CircuitData,
        matrix: &mut ComplexMatrix,
    ) {
        if circuit.xspice_event_node_matrix_rows().next().is_none() {
            return;
        }
        let deficient_rows = matrix.deficient_rows();
        Self::visit_unconstrained_xspice_event_rows(
            circuit,
            |row| deficient_rows.binary_search(&row).is_ok(),
            |row| {
                matrix.add_real(row, row, 1.0);
            },
        );
    }

    /// Conductance of the `.OPTIONS RSHUNT` node-to-ground shunt, or zero when
    /// the deck did not ask for one.
    pub(in crate::engine) fn nodal_shunt_conductance(&self) -> Value {
        self.config
            .rshunt
            .map_or(0.0, |resistance| 1.0 / resistance)
    }

    /// Stamp all DC values into matrix using O(1) direct stamping
    pub(crate) fn stamp_dc_direct(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        gmin: Value,
    ) {
        // Add GMIN only to node-voltage equations (not branch-current equations).
        Self::stamp_nodal_gmin(circuit, matrix, gmin);

        // Use the optimized direct stamping from CircuitData
        circuit.stamp_dc_direct(matrix, rhs);
    }

    /// Stamp with scaled sources for source stepping
    pub(crate) fn stamp_dc_scaled(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        gmin: Value,
        scale: Value,
    ) {
        // Add GMIN only to node-voltage equations (not branch-current equations).
        Self::stamp_nodal_gmin(circuit, matrix, gmin);

        circuit.stamp_dc_direct_scaled(matrix, rhs, scale);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::Netlist;

    #[test]
    fn a_zero_nodal_floor_closes_a_pure_event_row_and_no_analog_one() {
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("digital");
        circuit.get_or_create_node("analog");
        circuit.xspice_has_event_driven_devices = true;
        circuit
            .net_kinds
            .register(1, crate::circuit::NetKind::Digital);
        circuit.set_analog_touched_nodes(&std::collections::BTreeSet::from([2]));
        circuit.zero_dc_nodal_floor = true;
        let mut matrix = StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (1, 1, 0.0)]).unwrap();

        Engine::stamp_nodal_gmin(&circuit, &mut matrix, 0.0);

        assert_eq!(matrix.deficient_rows(), vec![1]);
    }

    #[test]
    fn only_empty_zero_rhs_xspice_event_rows_are_pinned() {
        let mut circuit = CircuitData::new();
        circuit.get_or_create_node("digital");
        circuit.xspice_has_event_driven_devices = true;
        circuit
            .net_kinds
            .register(1, crate::circuit::NetKind::Digital);
        let mut matrix = StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).unwrap();
        let mut rhs = [0.0];

        Engine::pin_unconstrained_xspice_event_rows(&circuit, &mut matrix, &mut rhs);
        assert!(matrix.deficient_rows().is_empty());

        matrix.clear_values();
        rhs[0] = 1.0;
        Engine::pin_unconstrained_xspice_event_rows(&circuit, &mut matrix, &mut rhs);
        assert_eq!(matrix.deficient_rows(), vec![0]);

        matrix.clear_values();
        matrix.add(0, 0, 2.0);
        rhs[0] = 0.0;
        Engine::pin_unconstrained_xspice_event_rows(&circuit, &mut matrix, &mut rhs);
        rhs[0] = 4.0;
        assert_eq!(matrix.solve(&rhs).unwrap(), vec![2.0]);
    }

    #[test]
    fn physical_rshunt_is_additive_with_numerical_gmin_and_not_source_scaled() {
        let netlist = Netlist::parse(
            "additive RSHUNT stamp\n\
             I1 0 out 2\n\
             .OPTIONS RSHUNT=2k\n\
             .END\n",
        )
        .expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);

        let gmin = 2.5e-4;
        let residual = matrix.with_probe_values(|probe, rhs| {
            engine.stamp_dc_scaled(&circuit, probe, rhs, gmin, 0.25);
            probe.residual_vector(&[1.0], rhs).expect("residual forms")
        });

        // GMIN=250 uS and RSHUNT=500 uS add to 750 uS. The 2 A source is
        // stepped to 0.5 A, while the physical shunt is deliberately not.
        assert!((residual[0] + 0.5 - 7.5e-4).abs() <= 1.0e-15);
    }

    #[test]
    fn physical_rshunt_excludes_private_non_electrical_state_rows() {
        let engine = Engine::default();
        let mut circuit = CircuitData::new();
        let electrical = circuit.get_or_create_node("electrical");
        let private_state = circuit.get_or_create_node("private_state");
        circuit.global_shunt_conductance = 2.0e-3;
        circuit.non_electrical_state_nodes.insert(private_state);
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);

        let residual = matrix.with_probe_values(|probe, rhs| {
            circuit.stamp_dc_direct(probe, rhs);
            probe
                .residual_vector(&[1.0, 1.0], rhs)
                .expect("residual forms")
        });
        assert_eq!(residual[electrical - 1].to_bits(), 2.0e-3_f64.to_bits());
        assert_eq!(residual[private_state - 1], 0.0);
    }

    #[test]
    fn physical_rshunt_replaces_the_final_numerical_nodal_floor() {
        let netlist = Netlist::parse(
            "physical RSHUNT final floor\n\
             I1 0 out 1m\n\
             .OPTIONS RSHUNT=1e9\n\
             .END\n",
        )
        .expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");

        assert!(engine.config.convergence_config.gmin_target > 0.0);
        assert_eq!(engine.dc_nodal_gmin_floor(&circuit), 0.0);
    }
}
