//! Deterministic Xyce-compatible device load planning.
//!
//! Xyce 7.10 instantiates device masters and their instances from a reversed
//! breadth-first traversal of its bipartite voltage/device graph.  Device
//! vector loads subsequently preserve that master/instance order, while
//! independent sources retain their inter-family instantiation order.  This
//! module records that logical order independently of RSpice's family SoAs so
//! physical DAE vector loading can remain allocation-free and reproducible.

use super::CircuitData;
use crate::NodeId;
use std::collections::{HashMap, VecDeque};

/// An indexed reference into one of the existing circuit device stores.
///
/// Additional families can be added as their direct `Q/F/B` loaders become
/// available; the plan never owns or duplicates device model behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum XyceDeviceRef {
    Resistor(usize),
    VoltageSource(usize),
    CurrentSource(usize),
    Core(usize),
    CoreGroup(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum XyceDeviceFamily {
    Resistor,
    VoltageSource,
    CurrentSource,
    Core,
}

impl XyceDeviceRef {
    #[inline]
    fn family(self) -> XyceDeviceFamily {
        match self {
            Self::Resistor(_) => XyceDeviceFamily::Resistor,
            Self::VoltageSource(_) => XyceDeviceFamily::VoltageSource,
            Self::CurrentSource(_) => XyceDeviceFamily::CurrentSource,
            Self::Core(_) | Self::CoreGroup(_) => XyceDeviceFamily::Core,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum XyceIndependentSourceRef {
    Voltage(usize),
    Current(usize),
}

/// One device vertex in Xyce's insertion-ordered topology graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct XyceTopologyDevice {
    pub(crate) device: XyceDeviceRef,
    pub(crate) terminals: Box<[NodeId]>,
}

impl XyceTopologyDevice {
    pub(crate) fn new(device: XyceDeviceRef, terminals: impl Into<Box<[NodeId]>>) -> Self {
        Self {
            device,
            terminals: terminals.into(),
        }
    }

    pub(crate) fn two_terminal(device: XyceDeviceRef, node_pos: NodeId, node_neg: NodeId) -> Self {
        Self {
            device,
            terminals: Box::new([node_pos, node_neg]),
        }
    }
}

/// Explicit compatibility contract for Xyce implementation-defined graph
/// root selection.
///
/// Xyce 7.10 obtains its root from `std::unordered_map::begin()`, whose order
/// is not portable.  The MSVC build used as RSpice's live 7.10 reference
/// starts at the first inserted graph vertex.  Naming that behavior prevents
/// an accidental claim that one traversal is universal across every Xyce
/// binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum XyceTopologyCompatibility {
    V7_10MsvcFirstInserted,
}

impl Default for XyceTopologyCompatibility {
    fn default() -> Self {
        Self::V7_10MsvcFirstInserted
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TopologyVertex {
    Node(NodeId),
    Device(XyceDeviceRef),
}

/// Frozen traversal products consumed by hot DAE vector loaders.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct XyceLoadPlan {
    ordered_devices: Vec<XyceDeviceRef>,
    master_order: Vec<XyceDeviceFamily>,
    independent_sources: Vec<XyceIndependentSourceRef>,
    current_sources: Vec<usize>,
    cores: Vec<usize>,
    core_groups: Vec<usize>,
}

impl XyceLoadPlan {
    pub(crate) fn build(
        devices: &[XyceTopologyDevice],
        compatibility: XyceTopologyCompatibility,
    ) -> Self {
        if devices.is_empty() {
            return Self::default();
        }

        let mut vertices = Vec::<TopologyVertex>::new();
        let mut adjacency = Vec::<Vec<usize>>::new();
        let mut node_vertices = HashMap::<NodeId, usize>::new();

        for entry in devices {
            let mut terminal_vertices = Vec::with_capacity(entry.terminals.len());
            for &node in entry.terminals.iter() {
                let vertex = if let Some(&existing) = node_vertices.get(&node) {
                    existing
                } else {
                    let index = vertices.len();
                    vertices.push(TopologyVertex::Node(node));
                    adjacency.push(Vec::new());
                    node_vertices.insert(node, index);
                    index
                };
                if !terminal_vertices.contains(&vertex) {
                    terminal_vertices.push(vertex);
                }
            }

            let device_vertex = vertices.len();
            vertices.push(TopologyVertex::Device(entry.device));
            adjacency.push(Vec::with_capacity(terminal_vertices.len()));
            for terminal_vertex in terminal_vertices {
                adjacency[terminal_vertex].push(device_vertex);
                adjacency[device_vertex].push(terminal_vertex);
            }
        }

        let root = match compatibility {
            XyceTopologyCompatibility::V7_10MsvcFirstInserted => 0,
        };
        let mut visited = vec![false; vertices.len()];
        let mut traversal = Vec::with_capacity(vertices.len());
        let mut queue = VecDeque::with_capacity(vertices.len());

        // Xyce circuits are normally connected through their reference node.
        // Continue in insertion order for a disconnected forest so malformed
        // or intentionally isolated decks still receive a total, stable plan.
        for component_root in
            std::iter::once(root).chain((0..vertices.len()).filter(|&index| index != root))
        {
            if visited[component_root] {
                continue;
            }
            visited[component_root] = true;
            queue.push_back(component_root);
            while let Some(vertex) = queue.pop_front() {
                traversal.push(vertex);
                for &neighbor in &adjacency[vertex] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        let ordered_devices = traversal
            .into_iter()
            .rev()
            .filter_map(|vertex| match vertices[vertex] {
                TopologyVertex::Node(_) => None,
                TopologyVertex::Device(device) => Some(device),
            })
            .collect::<Vec<_>>();

        let mut master_order = Vec::new();
        let mut independent_sources = Vec::new();
        let mut current_sources = Vec::new();
        let mut cores = Vec::new();
        let mut core_groups = Vec::new();
        for &device in &ordered_devices {
            let family = device.family();
            if !master_order.contains(&family) {
                master_order.push(family);
            }
            match device {
                XyceDeviceRef::VoltageSource(index) => {
                    independent_sources.push(XyceIndependentSourceRef::Voltage(index));
                }
                XyceDeviceRef::CurrentSource(index) => {
                    independent_sources.push(XyceIndependentSourceRef::Current(index));
                    current_sources.push(index);
                }
                XyceDeviceRef::Core(index) => cores.push(index),
                XyceDeviceRef::CoreGroup(index) => core_groups.push(index),
                XyceDeviceRef::Resistor(_) => {}
            }
        }

        Self {
            ordered_devices,
            master_order,
            independent_sources,
            current_sources,
            cores,
            core_groups,
        }
    }

    pub(crate) fn ordered_devices(&self) -> &[XyceDeviceRef] {
        &self.ordered_devices
    }

    pub(crate) fn master_order(&self) -> &[XyceDeviceFamily] {
        &self.master_order
    }

    pub(crate) fn independent_sources(&self) -> &[XyceIndependentSourceRef] {
        &self.independent_sources
    }

    pub(crate) fn current_sources(&self) -> &[usize] {
        &self.current_sources
    }

    pub(crate) fn cores(&self) -> &[usize] {
        &self.cores
    }

    pub(crate) fn core_groups(&self) -> &[usize] {
        &self.core_groups
    }
}

impl CircuitData {
    /// Record one Xyce topology device at its parser-defined insertion point.
    /// Device models remain owned by their existing family stores; this is an
    /// index-only scheduling record.
    pub(crate) fn record_xyce_topology_device(&mut self, device: XyceDeviceRef) {
        self.xyce_topology_order.push(device);
    }

    /// Freeze Xyce's reversed-BFT load order after all node remapping and
    /// synthesized Core registration are complete.
    pub(crate) fn finalize_xyce_load_plan(&mut self) -> Result<(), String> {
        let mut topology = Vec::with_capacity(self.xyce_topology_order.len());
        for &device in &self.xyce_topology_order {
            let terminals = match device {
                XyceDeviceRef::Resistor(index) => {
                    let stamp = self.resistors.stamps.get(index).ok_or_else(|| {
                        format!("Xyce load plan references missing resistor index {index}")
                    })?;
                    Box::new([stamp.pp.row, stamp.nn.row]) as Box<[NodeId]>
                }
                XyceDeviceRef::VoltageSource(index) => Box::new([
                    *self.voltage_sources.node_pos.get(index).ok_or_else(|| {
                        format!("Xyce load plan references missing voltage source index {index}")
                    })?,
                    *self.voltage_sources.node_neg.get(index).ok_or_else(|| {
                        format!("Xyce load plan references missing voltage source index {index}")
                    })?,
                ]),
                XyceDeviceRef::CurrentSource(index) => Box::new([
                    *self.current_sources.node_pos.get(index).ok_or_else(|| {
                        format!("Xyce load plan references missing current source index {index}")
                    })?,
                    *self.current_sources.node_neg.get(index).ok_or_else(|| {
                        format!("Xyce load plan references missing current source index {index}")
                    })?,
                ]),
                XyceDeviceRef::Core(index) => {
                    let binding = self.jiles_atherton_inductors.get(index).ok_or_else(|| {
                        format!("Xyce load plan references missing Core index {index}")
                    })?;
                    let inductor_index = binding.inductor_index;
                    Box::new([
                        *self.inductors.node_pos.get(inductor_index).ok_or_else(|| {
                            format!("Xyce Core {index} references missing winding")
                        })?,
                        *self.inductors.node_neg.get(inductor_index).ok_or_else(|| {
                            format!("Xyce Core {index} references missing winding")
                        })?,
                    ])
                }
                XyceDeviceRef::CoreGroup(index) => {
                    let group = self.xyce_core_groups.get(index).ok_or_else(|| {
                        format!("Xyce load plan references missing Core group index {index}")
                    })?;
                    let mut terminals = Vec::with_capacity(group.windings.len() * 2);
                    for winding in &group.windings {
                        terminals.push(
                            *self
                                .inductors
                                .node_pos
                                .get(winding.inductor_index)
                                .ok_or_else(|| {
                                    format!("Xyce Core group {index} references missing winding")
                                })?,
                        );
                        terminals.push(
                            *self
                                .inductors
                                .node_neg
                                .get(winding.inductor_index)
                                .ok_or_else(|| {
                                    format!("Xyce Core group {index} references missing winding")
                                })?,
                        );
                    }
                    terminals.into_boxed_slice()
                }
            };
            topology.push(XyceTopologyDevice::new(device, terminals));
        }
        self.xyce_load_plan =
            XyceLoadPlan::build(&topology, XyceTopologyCompatibility::V7_10MsvcFirstInserted);
        Ok(())
    }

    pub(crate) fn xyce_load_plan(&self) -> &XyceLoadPlan {
        &self.xyce_load_plan
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_terminal(
        device: XyceDeviceRef,
        node_pos: NodeId,
        node_neg: NodeId,
    ) -> XyceTopologyDevice {
        XyceTopologyDevice::two_terminal(device, node_pos, node_neg)
    }

    #[test]
    fn bh_star_preserves_xyce_reverse_bft_source_order() {
        let devices = vec![
            two_terminal(XyceDeviceRef::Resistor(0), 1, 0),
            two_terminal(XyceDeviceRef::CurrentSource(0), 1, 0),
            two_terminal(XyceDeviceRef::CurrentSource(1), 1, 0),
            two_terminal(XyceDeviceRef::CurrentSource(2), 1, 0),
            // Xyce appends the synthesized YMIN Core after scanning L/K cards.
            two_terminal(XyceDeviceRef::Core(0), 1, 0),
        ];
        let plan = XyceLoadPlan::build(&devices, XyceTopologyCompatibility::default());

        assert_eq!(plan.current_sources(), &[2, 1, 0]);
        assert_eq!(
            plan.ordered_devices(),
            &[
                XyceDeviceRef::Core(0),
                XyceDeviceRef::CurrentSource(2),
                XyceDeviceRef::CurrentSource(1),
                XyceDeviceRef::CurrentSource(0),
                XyceDeviceRef::Resistor(0),
            ]
        );
    }

    #[test]
    fn non_star_reversed_bft_is_not_reverse_netlist_order() {
        let devices = vec![
            // The first edge introduces node 2, but the next device reached
            // through it is deeper than the later resistor attached directly
            // to root node 1. Breadth-first order therefore differs from deck
            // order before the traversal is reversed.
            two_terminal(XyceDeviceRef::Resistor(0), 1, 2),
            two_terminal(XyceDeviceRef::CurrentSource(0), 2, 3),
            two_terminal(XyceDeviceRef::Resistor(1), 1, 0),
            two_terminal(XyceDeviceRef::CurrentSource(1), 3, 0),
        ];
        let plan = XyceLoadPlan::build(&devices, XyceTopologyCompatibility::default());
        let plain_reverse = devices
            .iter()
            .rev()
            .map(|entry| entry.device)
            .collect::<Vec<_>>();

        assert_ne!(plan.ordered_devices(), plain_reverse);
        assert_eq!(plan.current_sources(), &[1, 0]);
    }

    #[test]
    fn interleaved_independent_sources_keep_one_shared_order() {
        let devices = vec![
            two_terminal(XyceDeviceRef::VoltageSource(0), 1, 0),
            two_terminal(XyceDeviceRef::CurrentSource(0), 1, 0),
            two_terminal(XyceDeviceRef::VoltageSource(1), 1, 0),
            two_terminal(XyceDeviceRef::CurrentSource(1), 1, 0),
        ];
        let plan = XyceLoadPlan::build(&devices, XyceTopologyCompatibility::default());

        assert_eq!(
            plan.independent_sources(),
            &[
                XyceIndependentSourceRef::Current(1),
                XyceIndependentSourceRef::Voltage(1),
                XyceIndependentSourceRef::Current(0),
                XyceIndependentSourceRef::Voltage(0),
            ]
        );
        assert_eq!(
            plan.master_order(),
            &[
                XyceDeviceFamily::CurrentSource,
                XyceDeviceFamily::VoltageSource
            ]
        );
    }

    #[test]
    fn disconnected_components_receive_a_total_deterministic_order() {
        let devices = vec![
            two_terminal(XyceDeviceRef::CurrentSource(0), 1, 0),
            two_terminal(XyceDeviceRef::CurrentSource(1), 3, 2),
        ];
        let first = XyceLoadPlan::build(&devices, XyceTopologyCompatibility::default());
        let second = XyceLoadPlan::build(&devices, XyceTopologyCompatibility::default());

        assert_eq!(first, second);
        assert_eq!(first.current_sources().len(), 2);
    }
}
