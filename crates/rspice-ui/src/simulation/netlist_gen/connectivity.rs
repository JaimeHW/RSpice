//! Net naming for the generated deck.
//!
//! Assigns a name to every net the schematic implies: the label a designer
//! wrote where there is one, and a stable generated name where there is not.

use super::*;

impl<'a> NetlistGenerator<'a> {
    //-------------------------------------------------------------------------

    /// Extract electrical nets from wire connectivity.
    ///
    /// The graph holds only meaningful nodes — wire vertices, component
    /// terminals, junctions, and net-label anchors — and chains each wire
    /// segment through the candidates that lie on it. Connectivity is
    /// identical to enumerating every coordinate along every segment (a
    /// point matters only if some other geometry references it), at a
    /// fraction of the cost and memory.
    pub(super) fn extract_nets(&mut self) {
        let bus_analysis =
            crate::schematic::bus_connectivity::analyze_bus_connectivity(self.schematic);
        for diagnostic in &bus_analysis.diagnostics {
            use crate::schematic::bus_connectivity::BusDiagnosticKind;
            let destination = if diagnostic.kind == BusDiagnosticKind::UnnamedBus {
                &mut self.warnings
            } else {
                &mut self.errors
            };
            destination.push(diagnostic.message.clone());
        }

        // Seeded nodes: always nets, even when isolated (a floating
        // terminal must still get its own SPICE node).
        let mut point_graph: HashMap<Point, HashSet<Point>> = HashMap::new();
        for wire in &self.schematic.wires {
            for point in &wire.points {
                point_graph.entry(*point).or_default();
            }
        }
        for component in &self.schematic.components {
            for (_, terminal_pos) in self.component_terminal_positions(component) {
                point_graph.entry(terminal_pos).or_default();
            }
        }

        // Chain candidates: points that join a segment when they lie on
        // it. Labels and junctions are candidates but not seeds — a label
        // floating in empty space must NOT mint a net (it warns instead).
        let mut by_row: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut by_col: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut candidate_points: HashSet<Point> = HashSet::new();
        {
            let mut add_candidate = |p: Point| {
                by_row.entry(p.y).or_default().push(p.x);
                by_col.entry(p.x).or_default().push(p.y);
                candidate_points.insert(p);
            };
            for point in point_graph.keys() {
                add_candidate(*point);
            }
            for junction in &self.schematic.junctions {
                add_candidate(junction.pos);
            }
            for label in &self.schematic.net_labels {
                add_candidate(label.pos);
            }
            for binding in &bus_analysis.scalar_taps {
                add_candidate(binding.point);
            }
        }
        for xs in by_row.values_mut() {
            xs.sort_unstable();
            xs.dedup();
        }
        for ys in by_col.values_mut() {
            ys.sort_unstable();
            ys.dedup();
        }

        fn link(graph: &mut HashMap<Point, HashSet<Point>>, a: Point, b: Point) {
            if a != b {
                graph.entry(a).or_default().insert(b);
                graph.entry(b).or_default().insert(a);
            }
        }

        for wire in &self.schematic.wires {
            for seg in wire.points.windows(2) {
                let (a, b) = (seg[0], seg[1]);
                if a.y == b.y {
                    // Horizontal: chain through candidates on this row.
                    let (x0, x1) = (a.x.min(b.x), a.x.max(b.x));
                    let mut prev = Point::new(x0, a.y);
                    if let Some(xs) = by_row.get(&a.y) {
                        let lo = xs.partition_point(|&x| x < x0);
                        let hi = xs.partition_point(|&x| x <= x1);
                        for &x in &xs[lo..hi] {
                            let p = Point::new(x, a.y);
                            link(&mut point_graph, prev, p);
                            prev = p;
                        }
                    }
                    link(&mut point_graph, prev, Point::new(x1, a.y));
                } else if a.x == b.x {
                    // Vertical: chain through candidates on this column.
                    let (y0, y1) = (a.y.min(b.y), a.y.max(b.y));
                    let mut prev = Point::new(a.x, y0);
                    if let Some(ys) = by_col.get(&a.x) {
                        let lo = ys.partition_point(|&y| y < y0);
                        let hi = ys.partition_point(|&y| y <= y1);
                        for &y in &ys[lo..hi] {
                            let p = Point::new(a.x, y);
                            link(&mut point_graph, prev, p);
                            prev = p;
                        }
                    }
                    link(&mut point_graph, prev, Point::new(a.x, y1));
                } else {
                    // Any-angle conductors have the same exact attachment
                    // semantics as Manhattan conductors. Terminals, labels,
                    // junctions, bus-tap endpoints, and another wire's
                    // endpoint on this segment all split its electrical
                    // chain. Interior/interior crossings stay disconnected
                    // unless an explicit junction is authored there.
                    let segment = crate::state::WireSegment::new(a, b);
                    let mut points = candidate_points
                        .iter()
                        .copied()
                        .filter(|point| segment.contains_point(*point))
                        .collect::<Vec<_>>();
                    points.sort_unstable_by_key(|point| {
                        let dx = i128::from(point.x) - i128::from(a.x);
                        let dy = i128::from(point.y) - i128::from(a.y);
                        dx * dx + dy * dy
                    });
                    points.dedup();
                    for pair in points.windows(2) {
                        link(&mut point_graph, pair[0], pair[1]);
                    }
                }
            }
        }

        // Flood-fill to find connected components (nets)
        let mut visited: HashSet<Point> = HashSet::new();
        let mut net_id = 1;

        // Deterministic traversal is critical because auto-generated node labels
        // (net1, net2, ...) depend on discovery order. HashMap key iteration is
        // intentionally randomized, so sort points first to keep names stable
        // across repeated runs of the same schematic.
        let mut all_points: Vec<Point> = point_graph.keys().copied().collect();
        all_points.sort_by_key(|point| (point.x, point.y));

        for start_point in all_points {
            if visited.contains(&start_point) {
                continue;
            }

            let mut net = Net::new(net_id);
            let mut stack = vec![start_point];

            while let Some(point) = stack.pop() {
                if visited.insert(point) {
                    net.add_point(point);
                    self.point_to_net.insert(point, net_id);

                    if let Some(neighbors) = point_graph.get(&point) {
                        for neighbor in neighbors {
                            if !visited.contains(neighbor) {
                                stack.push(*neighbor);
                            }
                        }
                    }
                }
            }

            if !net.points.is_empty() {
                self.nets.push(net);
                net_id += 1;
            }
        }

        // Vector nets project onto scalar nodes here, before any name is
        // resolved, so a scalar tap and a vector port cannot each mint their
        // own node for one conductor.
        self.expand_vector_nets();
    }

    //-------------------------------------------------------------------------
    // Phase 1a: Interface Ports
    //-------------------------------------------------------------------------

    /// Fold interface ports into the extracted nets.
    ///
    /// A port names its net exactly like a label, and same-name ports
    /// connect their nets. This runs before `apply_net_labels` so the
    /// interface name wins label conflicts — the port list is the cell's
    /// contract. An unnamed port is an error: the interface cannot contain
    /// an anonymous pin.
    pub(super) fn apply_interface_ports(&mut self) {
        let mut name_to_net: HashMap<String, usize> = HashMap::new();
        for component in &self.schematic.components {
            if component.kind != ComponentType::Port {
                continue;
            }
            let Some(spec) = component.port_spec() else {
                self.errors.push(format!(
                    "Unnamed interface port at ({}, {}) — name it or remove it",
                    component.pos.x, component.pos.y
                ));
                continue;
            };
            let Some((_, terminal)) = self
                .component_terminal_positions(component)
                .into_iter()
                .next()
            else {
                continue;
            };
            let Some(&net_id) = self.point_to_net.get(&terminal) else {
                continue;
            };
            // A vector port names no scalar node. Its conductors are the
            // projected bits of the vector net it joins, which already carry
            // the deck names the header declares; writing `DATA[7:0]` onto the
            // terminal's own net would put the authored delimiters — which no
            // probe survives — into the emitted deck.
            if spec.vector().is_some() {
                continue;
            }

            if let Err(error) =
                validate_net_name(&spec.name, self.schematic.document_policy.net_naming)
            {
                self.errors.push(format!(
                    "Invalid interface port name '{}': {error}",
                    spec.name
                ));
                continue;
            }
            let key = net_name_key(&spec.name);
            match name_to_net.get(&key) {
                // Same port name elsewhere: one interface pin, one net.
                Some(&primary) if primary != net_id => self.merge_nets(primary, net_id),
                _ => {
                    name_to_net.insert(key, net_id);
                    if let Some(net) = self.nets.iter_mut().find(|n| n.id == net_id) {
                        net.label = Some(spec.name.clone());
                    }
                }
            }
        }
    }

    //-------------------------------------------------------------------------
    // Phase 1b: Net Labels
    //-------------------------------------------------------------------------

    /// Fold user net labels into the extracted nets.
    ///
    /// A label names the net beneath it, and labels sharing a name connect
    /// their nets (standard schematic-label semantics). Conflicts keep the
    /// first label and warn; a label off any wire or terminal warns.
    pub(super) fn apply_net_labels(&mut self) {
        // Typed scalar aliases are the electrical contract, so resolve them
        // before ordinary labels. A conflicting free-form label is a blocking
        // error rather than a warning that silently changes DATA[n] into an
        // unrelated scalar node name.
        let mut typed_bindings =
            crate::schematic::bus_connectivity::analyze_bus_connectivity(self.schematic)
                .scalar_taps;
        typed_bindings.sort_by_key(|binding| binding.tap_id);

        // Seed with names already assigned (interface ports run first), so
        // a label matching a port name connects to the port's net instead
        // of minting a same-named twin.
        let mut name_to_net: HashMap<String, usize> = self
            .nets
            .iter()
            .filter_map(|net| {
                net.label
                    .as_ref()
                    .map(|label| (net_name_key(label), net.id))
            })
            .collect();
        // The authored spelling is what a user could have written, so it is what
        // gets validated, keyed and reported; the deck spelling is what the net
        // is actually named, because `<3>` does not survive a probe.
        for binding in &typed_bindings {
            let name = binding.member_name.as_str();
            let deck_name = binding.deck_name.as_str();
            if let Err(error) = validate_net_name(name, self.schematic.document_policy.net_naming) {
                self.errors
                    .push(format!("Invalid typed bus member \"{name}\": {error}"));
                continue;
            }
            let Some(&net_id) = self.point_to_net.get(&binding.point) else {
                self.errors.push(format!(
                    "Typed bus member \"{name}\" at ({}, {}) is not on a scalar net",
                    binding.point.x, binding.point.y
                ));
                continue;
            };
            if let Some(existing) = self.net(net_id).and_then(|net| net.label.as_deref())
                && !net_names_equal(existing, name)
                && !net_names_equal(existing, deck_name)
            {
                let existing = self.display_net_name(existing);
                self.errors.push(format!(
                    "Typed bus member \"{name}\" conflicts with net name \"{existing}\""
                ));
                continue;
            }

            // The bit this tap selects may already exist as a projected node of
            // its bus. Both spellings are looked up so the tap merges into that
            // node instead of creating a second one under the same deck name.
            let key = net_name_key(name);
            let deck_key = net_name_key(deck_name);
            let effective_net_id = match name_to_net
                .get(&key)
                .or_else(|| name_to_net.get(&deck_key))
                .copied()
            {
                Some(primary) if primary != net_id => {
                    self.merge_nets(primary, net_id);
                    primary
                }
                _ => net_id,
            };
            name_to_net.insert(key, effective_net_id);
            name_to_net.insert(deck_key, effective_net_id);
            if let Some(net) = self.nets.iter_mut().find(|net| net.id == effective_net_id) {
                net.label = Some(deck_name.to_owned());
            }
        }

        let mut labels: Vec<_> = self.schematic.net_labels.iter().collect();
        labels.sort_by_key(|label| label.id);
        for label in labels {
            let authored_name = label.name.trim();
            if authored_name.is_empty() {
                continue;
            }
            if let Err(error) =
                validate_net_name(authored_name, self.schematic.document_policy.net_naming)
            {
                self.errors
                    .push(format!("Invalid net label \"{authored_name}\": {error}"));
                continue;
            }
            let promoted_name = self
                .hierarchy
                .and_then(|hierarchy| hierarchy.canonical_global_label(authored_name));
            let name = promoted_name.as_deref().unwrap_or(authored_name);
            let Some(&net_id) = self.point_to_net.get(&label.pos) else {
                self.warnings.push(format!(
                    "Net label \"{authored_name}\" at ({}, {}) is not on a wire or terminal",
                    label.pos.x, label.pos.y
                ));
                continue;
            };

            let typed_conflict = typed_bindings.iter().find_map(|binding| {
                (self.point_to_net.get(&binding.point) == Some(&net_id)
                    && !net_names_equal(&binding.member_name, name))
                .then_some(binding.member_name.as_str())
            });
            if let Some(typed_name) = typed_conflict {
                self.errors.push(format!(
                    "Net label \"{name}\" conflicts with typed bus member \"{typed_name}\""
                ));
                continue;
            }

            let key = net_name_key(name);
            match name_to_net.get(&key) {
                // Same name on another net: connect them.
                Some(&primary) if primary != net_id => self.merge_nets(primary, net_id),
                _ => {
                    name_to_net.insert(key, net_id);
                    // A tap already named this net in the deck spelling, and a
                    // label that disagreed with the member errored above. This
                    // one agrees, so it must not rewrite the node's identity
                    // back into the authored delimiters.
                    if typed_bindings
                        .iter()
                        .any(|binding| self.point_to_net.get(&binding.point) == Some(&net_id))
                    {
                        continue;
                    }
                    let existing = self.net(net_id).and_then(|net| net.label.clone());
                    match existing {
                        Some(existing) if !net_names_equal(&existing, name) => {
                            let existing = self.display_net_name(&existing);
                            self.warnings.push(format!(
                                "Net carries conflicting labels \"{existing}\" and \
                                 \"{name}\"; keeping \"{existing}\""
                            ));
                        }
                        _ => {
                            if let Some(net) = self.nets.iter_mut().find(|n| n.id == net_id) {
                                net.label = Some(name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    /// Merge net `loser` into `winner`: fold its points, repoint the lookup,
    /// and drop it from the net list.
    fn merge_nets(&mut self, winner: usize, loser: usize) {
        if winner == loser {
            return;
        }
        let Some(index) = self.nets.iter().position(|n| n.id == loser) else {
            return;
        };
        let loser_net = self.nets.remove(index);
        for net_id in self.point_to_net.values_mut() {
            if *net_id == loser {
                *net_id = winner;
            }
        }
        if let Some(net) = self.nets.iter_mut().find(|n| n.id == winner) {
            net.merge(&loser_net);
        }
    }

    //-------------------------------------------------------------------------
    // Phase 2: Ground Identification
    //-------------------------------------------------------------------------

    /// Identify ground net from Ground components
    pub(super) fn identify_ground(&mut self) {
        // EVERY ground symbol binds its net to node 0 — standard schematic
        // semantics where separate ground symbols are the same electrical
        // node even without a drawn wire between them.
        for component in &self.schematic.components {
            if component.kind == ComponentType::Ground {
                let terminals = self.component_terminal_positions(component);
                if let Some((_, terminal_pos)) = terminals.first()
                    && let Some(&net_id) = self.point_to_net.get(terminal_pos)
                {
                    if self.ground_net.is_none() {
                        self.ground_net = Some(net_id);
                    }
                    if let Some(net) = self.nets.iter_mut().find(|n| n.id == net_id) {
                        net.label = Some("0".to_string());
                    }
                }
            }
        }

        // If no explicit ground, create one at net 0 if it exists
        if !self.nets.is_empty() && self.ground_net.is_none() {
            // Warn: no ground found (in real use, this would be an error)
        }
    }

    //-------------------------------------------------------------------------
    // Phase 3: Header Generation
}

/// Net identity folds ASCII case because the emitted deck does: SPICE node
/// names are case-insensitive, so `Out` and `out` are one node in the
/// simulation whatever the document's naming policy says. That policy is an
/// authoring-syntax rule — which characters a name may contain — and never a
/// statement about which nets are the same net.
fn net_name_key(name: &str) -> String {
    name.to_ascii_lowercase()
}

/// Identity again, for a direct comparison: see [`net_name_key`].
fn net_names_equal(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
}

fn validate_net_name(
    name: &str,
    policy: crate::state::NetNamingPolicy,
) -> Result<(), &'static str> {
    crate::state::NetLabel::validate_name(name, policy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{NetLabel, NetNamingPolicy, Point, SchematicState, Wire};

    #[test]
    fn net_identity_folds_case_under_every_policy() {
        assert_eq!(net_name_key("Out"), net_name_key("out"));
        assert!(net_names_equal("Out", "out"));

        for policy in [
            NetNamingPolicy::StrictCaseSensitive,
            NetNamingPolicy::SpiceCompatibleRelaxed,
        ] {
            let mut schematic = SchematicState::default();
            schematic.document_policy.net_naming = policy;
            schematic
                .wires
                .push(Wire::segment(1, Point::new(0, 0), Point::new(40, 0)));
            schematic
                .wires
                .push(Wire::segment(2, Point::new(0, 40), Point::new(40, 40)));
            schematic
                .net_labels
                .push(NetLabel::new(1, Point::new(20, 0), "Out"));
            schematic
                .net_labels
                .push(NetLabel::new(2, Point::new(20, 40), "out"));

            let nets = crate::simulation::netlist_gen::design_nets(&schematic);
            assert_eq!(
                nets.len(),
                1,
                "{policy:?}: the deck joins `Out` and `out` into one node, so the drawing must \
                 show one net"
            );
            assert_eq!(nets[0].name, "Out");
            let mut wire_ids = nets[0].wire_ids.clone();
            wire_ids.sort_unstable();
            assert_eq!(wire_ids, vec![1, 2]);
        }
    }
}
