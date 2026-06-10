use super::*;

impl<'a> NetlistGenerator<'a> {
    //-------------------------------------------------------------------------

    /// Extract electrical nets from wire connectivity
    pub(super) fn extract_nets(&mut self) {
        // Build point graph from wires
        let mut point_graph: HashMap<Point, HashSet<Point>> = HashMap::new();

        // Add all wire segments to the graph
        for wire in &self.schematic.wires {
            self.add_wire_to_graph(wire, &mut point_graph);
        }

        // Add component terminals to graph
        for component in &self.schematic.components {
            for (_, terminal_pos) in component.terminal_positions() {
                point_graph.entry(terminal_pos).or_default();
            }
        }

        // Connect terminals to wires at matching points
        for component in &self.schematic.components {
            for (_, terminal_pos) in component.terminal_positions() {
                // Check if any wire passes through this terminal
                for wire in &self.schematic.wires {
                    if wire.contains_point(terminal_pos) {
                        // Connect terminal to wire endpoints
                        for point in &wire.points {
                            if *point != terminal_pos {
                                point_graph.entry(terminal_pos).or_default().insert(*point);
                                point_graph.entry(*point).or_default().insert(terminal_pos);
                            }
                        }
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
    }

    /// Add wire points and connections to the graph
    fn add_wire_to_graph(&self, wire: &Wire, graph: &mut HashMap<Point, HashSet<Point>>) {
        // Connect consecutive points
        for i in 0..wire.points.len() {
            let point = wire.points[i];
            graph.entry(point).or_default();

            // Connect to previous point
            if i > 0 {
                let prev = wire.points[i - 1];
                graph.entry(point).or_default().insert(prev);
                graph.entry(prev).or_default().insert(point);

                // Also add all points along the segment (for T-junctions)
                self.add_segment_points(prev, point, graph);
            }
        }
    }

    /// Add intermediate points along a wire segment for T-junction detection
    fn add_segment_points(
        &self,
        start: Point,
        end: Point,
        graph: &mut HashMap<Point, HashSet<Point>>,
    ) {
        // Only handle orthogonal segments
        if start.x == end.x {
            // Vertical segment
            let (min_y, max_y) = if start.y < end.y {
                (start.y, end.y)
            } else {
                (end.y, start.y)
            };
            for y in min_y..=max_y {
                let p = Point::new(start.x, y);
                graph.entry(p).or_default();
                // Connect to segment endpoints
                if p != start {
                    graph.entry(p).or_default().insert(start);
                    graph.entry(start).or_default().insert(p);
                }
                if p != end {
                    graph.entry(p).or_default().insert(end);
                    graph.entry(end).or_default().insert(p);
                }
            }
        } else if start.y == end.y {
            // Horizontal segment
            let (min_x, max_x) = if start.x < end.x {
                (start.x, end.x)
            } else {
                (end.x, start.x)
            };
            for x in min_x..=max_x {
                let p = Point::new(x, start.y);
                graph.entry(p).or_default();
                // Connect to segment endpoints
                if p != start {
                    graph.entry(p).or_default().insert(start);
                    graph.entry(start).or_default().insert(p);
                }
                if p != end {
                    graph.entry(p).or_default().insert(end);
                    graph.entry(end).or_default().insert(p);
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

        let mut name_to_net: HashMap<String, usize> = HashMap::new();
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
                    let existing = self
                        .net(net_id)
                        .and_then(|net| net.label.clone());
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
                let terminals = component.terminal_positions();
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
