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
        {
            let mut add_candidate = |p: Point| {
                by_row.entry(p.y).or_default().push(p.x);
                by_col.entry(p.x).or_default().push(p.y);
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
                    // Diagonal segments connect only at their endpoints,
                    // matching `Wire::contains_point` semantics.
                    link(&mut point_graph, a, b);
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

            let key = spec.name.to_ascii_lowercase();
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
        // Process in id order so conflict resolution is deterministic.
        let mut labels: Vec<_> = self.schematic.net_labels.iter().collect();
        labels.sort_by_key(|label| label.id);

        // Seed with names already assigned (interface ports run first), so
        // a label matching a port name connects to the port's net instead
        // of minting a same-named twin.
        let mut name_to_net: HashMap<String, usize> = self
            .nets
            .iter()
            .filter_map(|net| {
                net.label
                    .as_ref()
                    .map(|label| (label.to_ascii_lowercase(), net.id))
            })
            .collect();
        for label in labels {
            let name = label.name.trim();
            if name.is_empty() {
                continue;
            }
            let Some(&net_id) = self.point_to_net.get(&label.pos) else {
                self.warnings.push(format!(
                    "Net label \"{name}\" at ({}, {}) is not on a wire or terminal",
                    label.pos.x, label.pos.y
                ));
                continue;
            };

            match name_to_net.get(&name.to_ascii_lowercase()) {
                // Same name on another net: connect them.
                Some(&primary) if primary != net_id => self.merge_nets(primary, net_id),
                _ => {
                    name_to_net.insert(name.to_ascii_lowercase(), net_id);
                    let existing = self.net(net_id).and_then(|net| net.label.clone());
                    match existing {
                        Some(existing) if !existing.eq_ignore_ascii_case(name) => {
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
        for component in &self.schematic.components {
            if component.kind == ComponentType::Ground {
                // Find the net connected to this ground symbol
                let terminals = self.component_terminal_positions(component);
                if let Some((_, terminal_pos)) = terminals.first()
                    && let Some(&net_id) = self.point_to_net.get(terminal_pos)
                {
                    self.ground_net = Some(net_id);
                    // Update the net's label
                    if let Some(net) = self.nets.iter_mut().find(|n| n.id == net_id) {
                        net.label = Some("0".to_string());
                    }
                    return;
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
