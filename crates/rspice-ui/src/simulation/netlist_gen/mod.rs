//! Netlist Generator - Schematic to SPICE Netlist Conversion
//!
//! Commercial-grade netlist generation following Cadence Spectre conventions:
//! - Node connectivity extraction via wire tracing
//! - Ground detection (GND nets â†’ node 0)
//! - SPICE instance line generation for all component types
//! - Subcircuit and model statement generation
//! - Analysis command generation from UI configuration
//!
//! # Architecture
//!
//! The netlist generator works in phases:
//! 1. **Node extraction**: Build connectivity graph from wires and terminals
//! 2. **Node naming**: Assign node numbers/names (with ground = 0)
//! 3. **Instance generation**: Generate SPICE lines for each component
//! 4. **Analysis generation**: Generate analysis commands from config
//!
//! # Example
//!
//! ```rust,ignore
//! use rspice_ui::state::schematic::SchematicState;
//! use rspice_ui::app::simulation::netlist_gen::NetlistGenerator;
//!
//! let schematic = SchematicState::default();
//! let generator = NetlistGenerator::new(&schematic);
//! let netlist = generator.generate();
//! ```

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::state::{Component, ComponentType, Point, SchematicState};
#[cfg(test)]
use crate::state::Wire;

mod connectivity;
mod formatting;
mod header;
mod instances;
mod magnetics;
mod models;

//=============================================================================
// NetlistResult (Compatibility API)
//=============================================================================

/// Result of netlist generation (compatibility API)
///
/// This provides the same interface as the legacy netlist generator
/// for backward compatibility with existing code.
#[derive(Debug, Clone)]
pub struct NetlistResult {
    /// The generated SPICE netlist text
    pub netlist: String,

    /// Net name assignments for cross-probing: net_name -> [node points]
    pub nets: HashMap<String, Vec<Point>>,

    /// Node point to net name mapping for probe lookup. Holds the nets'
    /// node points (vertices, terminals, junctions, labels); probes
    /// between nodes resolve through `net_segments`.
    pub point_to_net: HashMap<Point, String>,

    /// Wire segments per net, for resolving probe points that fall on a
    /// segment between nodes.
    pub net_segments: HashMap<String, Vec<(Point, Point)>>,

    /// Any warnings during generation
    pub warnings: Vec<String>,

    /// Any errors that prevent simulation
    pub errors: Vec<String>,
}

/// Generate a SPICE netlist from a schematic (convenience function)
///
/// This provides the same API as the legacy netlist generator for
/// backward compatibility. For more control, use `NetlistGenerator` directly.
pub fn generate_netlist(schematic: &SchematicState) -> NetlistResult {
    generate_netlist_with_analysis(schematic, &[])
}

/// Generate a SPICE netlist from a schematic with explicit analysis directives.
pub fn generate_netlist_with_analysis(
    schematic: &SchematicState,
    analysis_lines: &[String],
) -> NetlistResult {
    let mut generator = NetlistGenerator::new(schematic);
    let netlist = generator.generate_with_analysis(analysis_lines);

    // Build the nets map from the generator's data
    let mut nets: HashMap<String, Vec<Point>> = HashMap::new();
    let mut point_to_net: HashMap<Point, String> = HashMap::new();

    for net in generator.nets() {
        let name = net.spice_name();
        let points: Vec<Point> = net.points.iter().copied().collect();
        for &p in &points {
            point_to_net.insert(p, name.clone());
        }
        nets.insert(name, points);
    }

    // A wire belongs to exactly one net; index its segments under that
    // net's name so probes between nodes resolve.
    let mut net_segments: HashMap<String, Vec<(Point, Point)>> = HashMap::new();
    for wire in &schematic.wires {
        let Some(name) = wire
            .points
            .first()
            .and_then(|first| point_to_net.get(first))
        else {
            continue;
        };
        net_segments
            .entry(name.clone())
            .or_default()
            .extend(wire.points.windows(2).map(|w| (w[0], w[1])));
    }

    NetlistResult {
        netlist,
        nets,
        point_to_net,
        net_segments,
        warnings: generator.warnings().to_vec(),
        errors: generator.errors().to_vec(),
    }
}

//=============================================================================
// Node Net
//=============================================================================

/// Represents an electrical net (set of connected points)

#[derive(Debug, Clone)]
pub struct Net {
    /// Unique identifier for this net
    pub id: usize,

    /// All grid points that belong to this net
    pub points: HashSet<Point>,

    /// User-assigned label (if any, from net label component)
    pub label: Option<String>,
}

impl Net {
    /// Create a new empty net
    pub fn new(id: usize) -> Self {
        Self {
            id,
            points: HashSet::new(),
            label: None,
        }
    }

    /// Check if this net contains a point
    pub fn contains(&self, point: Point) -> bool {
        self.points.contains(&point)
    }

    /// Add a point to this net
    pub fn add_point(&mut self, point: Point) {
        self.points.insert(point);
    }

    /// Merge another net into this one
    pub fn merge(&mut self, other: &Net) {
        self.points.extend(&other.points);
        if self.label.is_none() {
            self.label = other.label.clone();
        }
    }

    /// Get SPICE node name
    ///
    /// Returns:
    /// - "0" for ground nets
    /// - User label if provided
    /// - "net{id}" otherwise
    pub fn spice_name(&self) -> String {
        if let Some(ref label) = self.label {
            if label.eq_ignore_ascii_case("0")
                || label.eq_ignore_ascii_case("gnd")
                || label.eq_ignore_ascii_case("ground")
            {
                return "0".to_string();
            }
            return label.clone();
        }
        format!("net{}", self.id)
    }
}

//=============================================================================
// Netlist Generator
//=============================================================================

/// Commercial-grade SPICE netlist generator
///
/// Extracts node connectivity from schematic and generates standard SPICE netlist.
pub struct NetlistGenerator<'a> {
    /// Reference to the schematic
    schematic: &'a SchematicState,

    /// Extracted nets (connected node groups)
    nets: Vec<Net>,

    /// Map from grid point to net ID
    point_to_net: HashMap<Point, usize>,

    /// Ground net ID (if found)
    ground_net: Option<usize>,

    /// Generated netlist lines
    lines: Vec<String>,

    /// Model definitions needed
    models: BTreeMap<String, String>,

    /// Subcircuit definitions needed
    subcircuits: Vec<String>,
    /// Netlist generation warnings surfaced to simulation controller.
    warnings: Vec<String>,
    /// Netlist generation errors that should block simulation.
    errors: Vec<String>,
}

impl<'a> NetlistGenerator<'a> {
    /// Create a new netlist generator for the given schematic
    pub fn new(schematic: &'a SchematicState) -> Self {
        Self {
            schematic,
            nets: Vec::new(),
            point_to_net: HashMap::new(),
            ground_net: None,
            lines: Vec::new(),
            models: BTreeMap::new(),
            subcircuits: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Generate complete SPICE netlist
    ///
    /// Returns the netlist as a string.
    pub fn generate(&mut self) -> String {
        self.generate_with_analysis(&[])
    }

    fn reset_generation_state(&mut self) {
        self.nets.clear();
        self.point_to_net.clear();
        self.ground_net = None;
        self.lines.clear();
        self.models.clear();
        self.subcircuits.clear();
        self.warnings.clear();
        self.errors.clear();
    }

    /// Read-only warnings collected during generation.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Read-only errors collected during generation.
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    /// Generate netlist with custom analysis commands
    pub fn generate_with_analysis(&mut self, analysis_lines: &[String]) -> String {
        self.reset_generation_state();

        // Phase 1: Extract node connectivity
        self.extract_nets();

        // Phase 1b: Fold user net labels into the nets (names + same-name
        // connections). Runs before ground identification so the ground
        // symbol always wins the node-0 assignment.
        self.apply_net_labels();

        // Phase 2: Identify ground
        self.identify_ground();

        // Phase 3: Generate header
        self.generate_header();

        // Phase 4: Generate include directives for library-bound instances
        self.generate_library_view_includes();

        // Phase 5: Generate component instances
        self.generate_instances();

        // Phase 6: Add models if needed
        self.generate_models();

        // Phase 7: Add analysis commands (if requested)
        if !analysis_lines.is_empty() {
            self.lines.push(String::new());
            self.lines.push("* Analysis commands".to_string());
            for line in analysis_lines {
                self.lines.push(line.clone());
            }
        }

        // Phase 8: End statement
        self.lines.push(String::new());
        self.lines.push(".end".to_string());

        self.lines.join("\n")
    }
}

impl<'a> NetlistGenerator<'a> {
    pub fn nets(&self) -> &[Net] {
        &self.nets
    }

    /// Get net by ID
    pub fn net(&self, id: usize) -> Option<&Net> {
        self.nets.iter().find(|n| n.id == id)
    }

    /// Get net for a point. Node points resolve directly; points between
    /// nodes resolve through the wire segment they lie on.
    pub fn net_at(&self, point: Point) -> Option<&Net> {
        if let Some(&id) = self.point_to_net.get(&point) {
            return self.net(id);
        }
        for wire in &self.schematic.wires {
            if wire.contains_point(point) {
                let first = wire.points.first()?;
                let id = *self.point_to_net.get(first)?;
                return self.net(id);
            }
        }
        None
    }

    /// Get ground net ID
    pub fn ground_net_id(&self) -> Option<usize> {
        self.ground_net
    }

    /// Check if schematic has ground
    pub fn has_ground(&self) -> bool {
        self.ground_net.is_some()
    }
}
fn chrono_lite_timestamp() -> String {
    // Use system time for a basic timestamp
    use std::time::{SystemTime, UNIX_EPOCH};
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => format!("{}", d.as_secs()),
        Err(_) => "unknown".to_string(),
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::NetLabel;

    /// A label sitting on a wire names that net in the netlist.
    #[test]
    fn net_label_names_the_node() {
        let mut state = SchematicState::default();
        crate::common::examples::load_example("RC Lowpass Filter", &mut state);

        let result = generate_netlist(&state);
        assert!(
            result.nets.contains_key("out"),
            "the \"out\" label should name its net; got nets {:?}",
            result.nets.keys().collect::<Vec<_>>()
        );
        // The named node appears in instance lines (R1 ... out ...).
        assert!(
            result.netlist.lines().any(|l| {
                l.starts_with('R') && l.split_whitespace().any(|tok| tok == "out")
            }),
            "instance lines should reference the labeled node:\n{}",
            result.netlist
        );
    }

    /// Labels sharing a name connect otherwise-disjoint nets.
    #[test]
    fn same_name_labels_merge_nets() {
        let mut state = SchematicState::default();
        state
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .wires
            .push(Wire::new(2, vec![Point::new(0, 100), Point::new(40, 100)]));
        state.net_labels.push(NetLabel::new(1, Point::new(20, 0), "bus"));
        state
            .net_labels
            .push(NetLabel::new(2, Point::new(20, 100), "bus"));

        let mut generator = NetlistGenerator::new(&state);
        generator.generate();
        let bus_nets = generator
            .nets()
            .iter()
            .filter(|n| n.spice_name() == "bus")
            .count();
        assert_eq!(bus_nets, 1, "same-name labels should fuse into one net");
        assert_eq!(
            generator.net_at(Point::new(0, 0)).map(|n| n.id),
            generator.net_at(Point::new(40, 100)).map(|n| n.id),
            "both wires should resolve to the same net id"
        );
    }

    /// A label naming a net "gnd" maps to SPICE node 0.
    #[test]
    fn gnd_label_maps_to_node_zero() {
        let mut state = SchematicState::default();
        state
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(40, 0)]));
        state.net_labels.push(NetLabel::new(1, Point::new(20, 0), "GND"));

        let mut generator = NetlistGenerator::new(&state);
        generator.generate();
        assert_eq!(
            generator.net_at(Point::new(20, 0)).map(|n| n.spice_name()),
            Some("0".to_string())
        );
    }

    /// A floating label warns instead of silently vanishing.
    #[test]
    fn floating_label_warns() {
        let mut state = SchematicState::default();
        state
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .net_labels
            .push(NetLabel::new(1, Point::new(500, 500), "lost"));

        let mut generator = NetlistGenerator::new(&state);
        generator.generate();
        assert!(
            generator
                .warnings()
                .iter()
                .any(|w| w.contains("lost") && w.contains("not on a wire")),
            "expected a floating-label warning; got {:?}",
            generator.warnings()
        );
    }
}
