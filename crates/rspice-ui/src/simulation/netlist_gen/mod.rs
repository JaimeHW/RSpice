//! Netlist Generator - Schematic to SPICE Netlist Conversion
//!
//! Commercial-grade netlist generation following Cadence Spectre conventions:
//! - Node connectivity extraction via wire tracing
//! - Ground detection (GND nets → node 0)
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

use crate::product::AnalysisInstanceId;
#[cfg(test)]
use crate::state::Wire;
use crate::state::{
    CellViewRef, Component, ComponentType, DesignVariable, DesignVariableScope, Point,
    SchematicState,
};

mod connectivity;
mod formatting;
mod header;
mod instances;
mod magnetics;
mod models;
mod subcircuits;

pub use subcircuits::HierarchySource;

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
    finish_generation(
        NetlistGenerator::new(schematic),
        schematic,
        analysis_lines,
        &[],
    )
}

/// Generate a netlist with project-cell hierarchy: placed cells whose
/// masters live in the workspace netlist as `.SUBCKT` definitions, so the
/// emitted deck is self-contained.
pub fn generate_netlist_hierarchical(
    schematic: &SchematicState,
    analysis_lines: &[String],
    hierarchy: &HierarchySource<'_>,
) -> NetlistResult {
    finish_generation(
        NetlistGenerator::with_hierarchy(schematic, hierarchy),
        schematic,
        analysis_lines,
        &[],
    )
}

/// Resolution context for project design variables. Scope matching uses exact
/// persisted identities; display labels never participate in execution.
#[derive(Debug, Clone, Copy)]
pub struct DesignVariableNetlistContext<'a> {
    pub active_cell: &'a CellViewRef,
    pub analysis_instances: &'a [AnalysisInstanceId],
}

/// Generate a self-contained hierarchical deck with the exact design
/// variables applicable to this cell/view and run set.
pub fn generate_netlist_hierarchical_with_variables(
    schematic: &SchematicState,
    analysis_lines: &[String],
    hierarchy: &HierarchySource<'_>,
    variables: &[DesignVariable],
    context: DesignVariableNetlistContext<'_>,
) -> NetlistResult {
    let parameter_lines = match design_variable_parameter_lines(variables, context) {
        Ok(lines) => lines,
        Err(errors) => {
            let mut result = finish_generation(
                NetlistGenerator::with_hierarchy(schematic, hierarchy),
                schematic,
                analysis_lines,
                &[],
            );
            result.errors.extend(errors);
            return result;
        }
    };
    finish_generation(
        NetlistGenerator::with_hierarchy(schematic, hierarchy),
        schematic,
        analysis_lines,
        &parameter_lines,
    )
}

/// Canonical parameter lines sorted by case-insensitive SPICE identifier.
/// Reordering equivalent project rows therefore cannot perturb executable
/// source or its content digest.
pub fn design_variable_parameter_lines(
    variables: &[DesignVariable],
    context: DesignVariableNetlistContext<'_>,
) -> Result<Vec<String>, Vec<String>> {
    let mut errors = Vec::new();
    let mut ordered = BTreeMap::<String, &DesignVariable>::new();
    for (index, variable) in variables.iter().enumerate() {
        if let Err(error) = variable.validate() {
            errors.push(format!("design variable {} is invalid: {error}", index + 1));
            continue;
        }
        let canonical = variable.name.to_ascii_lowercase();
        if ordered.insert(canonical, variable).is_some() {
            errors.push(format!(
                "design variable '{}' duplicates another case-insensitive parameter name",
                variable.name
            ));
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(ordered
        .into_values()
        .filter(|variable| design_variable_applies(variable, context))
        .map(DesignVariable::netlist_statement)
        .collect())
}

fn design_variable_applies(
    variable: &DesignVariable,
    context: DesignVariableNetlistContext<'_>,
) -> bool {
    match &variable.scope {
        DesignVariableScope::Project => true,
        // The payload owner is the active named simulation plan/testbench.
        // Presentation view type is not execution authority.
        DesignVariableScope::Testbench => true,
        DesignVariableScope::SelectedCell { cell } => cell == context.active_cell,
        DesignVariableScope::SelectedAnalysis { analysis_id } => {
            context.analysis_instances.contains(analysis_id)
        }
    }
}

fn finish_generation(
    mut generator: NetlistGenerator<'_>,
    schematic: &SchematicState,
    analysis_lines: &[String],
    parameter_lines: &[String],
) -> NetlistResult {
    let netlist = generator.generate_with_analysis_and_parameters(analysis_lines, parameter_lines);

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
// Design net summary (navigation UIs)
//=============================================================================

/// One electrical net of the open schematic, summarized for navigation:
/// the rail's Nets segment, cross-probe highlighting, search.
#[derive(Debug, Clone)]
pub struct DesignNet {
    /// SPICE name ("0", a label/port name, or autonamed `netN`).
    pub name: String,
    /// Component terminals on this net.
    pub pin_count: usize,
    /// `true` when the net is an interface port of the cell.
    pub is_port: bool,
    /// Wires belonging to this net (for canvas highlighting).
    pub wire_ids: Vec<u64>,
}

/// Live net summary: connectivity + ports + labels + ground, no instance
/// generation. Cheap enough to recompute on topology change; callers cache
/// by `topology_version`.
pub fn design_nets(schematic: &SchematicState) -> Vec<DesignNet> {
    let mut generator = NetlistGenerator::new(schematic);
    collect_design_nets(schematic, &mut generator)
}

/// Live net summary with project hierarchy/symbol resolution enabled.
pub fn design_nets_with_hierarchy(
    schematic: &SchematicState,
    hierarchy: &HierarchySource<'_>,
) -> Vec<DesignNet> {
    let mut generator = NetlistGenerator::with_hierarchy(schematic, hierarchy);
    collect_design_nets(schematic, &mut generator)
}

fn collect_design_nets(
    schematic: &SchematicState,
    generator: &mut NetlistGenerator<'_>,
) -> Vec<DesignNet> {
    generator.extract_nets();
    generator.apply_interface_ports();
    generator.apply_net_labels();
    generator.identify_ground();

    let ports: HashSet<String> = schematic
        .interface_ports()
        .iter()
        .map(|port| port.name.to_ascii_lowercase())
        .collect();

    let mut pin_counts: HashMap<usize, usize> = HashMap::new();
    for component in &schematic.components {
        for (_, position) in generator.component_terminal_positions(component) {
            if let Some(net) = generator.net_at(position) {
                *pin_counts.entry(net.id).or_default() += 1;
            }
        }
    }
    let mut wires: HashMap<usize, Vec<u64>> = HashMap::new();
    for wire in &schematic.wires {
        if let Some(first) = wire.points.first()
            && let Some(net) = generator.net_at(*first)
        {
            wires.entry(net.id).or_default().push(wire.id);
        }
    }

    let mut nets: Vec<DesignNet> = generator
        .nets()
        .iter()
        .map(|net| {
            let name = net.spice_name();
            DesignNet {
                is_port: ports.contains(&name.to_ascii_lowercase()),
                pin_count: pin_counts.get(&net.id).copied().unwrap_or(0),
                wire_ids: wires.remove(&net.id).unwrap_or_default(),
                name,
            }
        })
        .collect();

    // Reading order: interface ports, then named nets, then autonamed.
    let autonamed = |name: &str| {
        name.strip_prefix("net")
            .is_some_and(|n| n.chars().all(|c| c.is_ascii_digit()))
    };
    nets.sort_by(|a, b| {
        (!a.is_port, autonamed(&a.name), a.name.to_ascii_lowercase()).cmp(&(
            !b.is_port,
            autonamed(&b.name),
            b.name.to_ascii_lowercase(),
        ))
    });
    nets
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
    /// Project cell masters for hierarchical generation (`.SUBCKT`
    /// emission and instance terminal resolution). `None` keeps the
    /// flat, single-schematic behavior.
    hierarchy: Option<&'a HierarchySource<'a>>,
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
            hierarchy: None,
        }
    }

    /// Create a generator that resolves placed project cells through the
    /// given hierarchy source.
    pub fn with_hierarchy(
        schematic: &'a SchematicState,
        hierarchy: &'a HierarchySource<'a>,
    ) -> Self {
        let mut generator = Self::new(schematic);
        generator.hierarchy = Some(hierarchy);
        generator
    }

    /// Consume the generated lines (subcircuit-body assembly).
    pub(self) fn take_lines(&mut self) -> Vec<String> {
        std::mem::take(&mut self.lines)
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
        self.generate_with_analysis_and_parameters(analysis_lines, &[])
    }

    fn generate_with_analysis_and_parameters(
        &mut self,
        analysis_lines: &[String],
        parameter_lines: &[String],
    ) -> String {
        self.reset_generation_state();

        // Phase 1: Extract node connectivity
        self.extract_nets();

        // Phase 1a: Interface ports name their nets first — the port list
        // is the cell's contract, so it wins label conflicts.
        self.apply_interface_ports();

        // Phase 1b: Fold user net labels into the nets (names + same-name
        // connections). Runs before ground identification so the ground
        // symbol always wins the node-0 assignment.
        self.apply_net_labels();

        // Phase 2: Identify ground
        self.identify_ground();

        // Phase 3: Generate header
        self.generate_header();

        // Phase 3b: Emit typed project parameters before any instance or
        // subcircuit can reference them.
        if !parameter_lines.is_empty() {
            self.lines.push("* Design variables".to_owned());
            self.lines.extend(parameter_lines.iter().cloned());
            self.lines.push(String::new());
        }

        // Phase 4: Generate include directives for library-bound instances
        self.generate_library_view_includes();

        // Phase 4b: Project-cell .SUBCKT definitions (hierarchical mode) —
        // before instances so every definition precedes its first use.
        self.generate_subcircuit_definitions();

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
    pub(super) fn component_terminal_positions(
        &self,
        component: &Component,
    ) -> Vec<(String, Point)> {
        let resolved_symbol = component
            .library_cell
            .as_ref()
            .and_then(|binding| self.hierarchy?.resolved_symbol_for(binding));
        component.terminal_positions_resolved(resolved_symbol.as_ref())
    }

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
    format!("{}", crate::common::time_compat::unix_epoch().as_secs())
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, CellViewRef, Library, LibraryCellInstance, LibraryManager, NetLabel, PortDirection,
        PortSpec, SymbolDocument, SymbolPin, View, ViewType,
    };
    use std::collections::HashMap;

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    fn library_with_authored_amp_symbol() -> (LibraryManager, HashMap<String, SchematicState>) {
        let mut libraries = LibraryManager::new();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));

        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
            ],
            ..SymbolDocument::default()
        };
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);

        let mut buffers = HashMap::new();
        let mut master = SchematicState::default();
        for (idx, name) in ["IN", "OUT"].iter().enumerate() {
            let id = master.add_component(ComponentType::Port, Point::new(idx as i32 * 40, 0));
            master
                .components
                .iter_mut()
                .find(|component| component.id == id)
                .expect("port component")
                .value = (*name).to_owned();
        }
        buffers.insert(CellViewRef::new("work", "amp", "schematic").key(), master);
        (libraries, buffers)
    }

    fn authored_amp_instance() -> Component {
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding)
    }

    fn variable(name: &str, expression: &str, scope: DesignVariableScope) -> DesignVariable {
        DesignVariable::new(
            name,
            expression,
            crate::state::DesignVariableQuantity::Resistance,
            scope,
            "netlist fixture",
            None,
            crate::state::DesignVariableSweepEligibility::FixedParameter,
            crate::state::DesignVariableOverridePolicy::InheritOwnerOnly,
        )
        .expect("fixture variable is valid")
    }

    #[test]
    fn design_variable_parameter_lines_are_canonical_and_plan_scoped() {
        let active_cell = CellViewRef::default_top();
        let other_cell = CellViewRef::new("user", "other", "schematic");
        let selected_analysis = AnalysisInstanceId::new();
        let variables = vec![
            variable("ZLOAD", "20 kohm", DesignVariableScope::Project),
            variable("ALOAD", "10 kohm", DesignVariableScope::Testbench),
            variable(
                "CELL_ONLY",
                "30 kohm",
                DesignVariableScope::SelectedCell { cell: other_cell },
            ),
            variable(
                "ANALYSIS_ONLY",
                "40 kohm",
                DesignVariableScope::SelectedAnalysis {
                    analysis_id: selected_analysis,
                },
            ),
        ];
        let lines = design_variable_parameter_lines(
            &variables,
            DesignVariableNetlistContext {
                active_cell: &active_cell,
                analysis_instances: &[selected_analysis],
            },
        )
        .unwrap();
        assert_eq!(
            lines,
            [
                ".param ALOAD=1.00000000000000000e4",
                ".param ANALYSIS_ONLY=4.00000000000000000e4",
                ".param ZLOAD=2.00000000000000000e4",
            ]
        );
    }

    #[test]
    fn design_nets_with_hierarchy_counts_authored_symbol_pin_positions() {
        let (libraries, buffers) = library_with_authored_amp_symbol();
        let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);
        let mut schematic = SchematicState::default();
        schematic.components.push(authored_amp_instance());
        schematic
            .wires
            .push(Wire::segment(2, Point::new(60, 40), Point::new(40, 40)));
        schematic
            .wires
            .push(Wire::segment(3, Point::new(170, 70), Point::new(190, 70)));
        schematic
            .net_labels
            .push(NetLabel::new(4, Point::new(40, 40), "vin"));
        schematic
            .net_labels
            .push(NetLabel::new(5, Point::new(190, 70), "vout"));

        let nets = design_nets_with_hierarchy(&schematic, &hierarchy);
        let pin_counts: HashMap<_, _> = nets
            .iter()
            .map(|net| (net.name.as_str(), net.pin_count))
            .collect();

        assert_eq!(pin_counts.get("vin"), Some(&1));
        assert_eq!(pin_counts.get("vout"), Some(&1));
    }

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
            result
                .netlist
                .lines()
                .any(|l| { l.starts_with('R') && l.split_whitespace().any(|tok| tok == "out") }),
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
        state
            .net_labels
            .push(NetLabel::new(1, Point::new(20, 0), "bus"));
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

    #[test]
    fn strict_net_policy_preserves_case_distinct_connectivity() {
        let mut state = SchematicState::default();
        state
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .wires
            .push(Wire::new(2, vec![Point::new(0, 100), Point::new(40, 100)]));
        state
            .net_labels
            .push(NetLabel::new(1, Point::new(20, 0), "DATA"));
        state
            .net_labels
            .push(NetLabel::new(2, Point::new(20, 100), "data"));

        let mut generator = NetlistGenerator::new(&state);
        generator.generate();

        assert_ne!(
            generator.net_at(Point::new(0, 0)).map(|net| net.id),
            generator.net_at(Point::new(0, 100)).map(|net| net.id)
        );
    }

    #[test]
    fn relaxed_net_policy_merges_case_insensitively() {
        let mut state = SchematicState::default();
        state.document_policy.net_naming = crate::state::NetNamingPolicy::SpiceCompatibleRelaxed;
        state
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .wires
            .push(Wire::new(2, vec![Point::new(0, 100), Point::new(40, 100)]));
        state
            .net_labels
            .push(NetLabel::new(1, Point::new(20, 0), "DATA"));
        state
            .net_labels
            .push(NetLabel::new(2, Point::new(20, 100), "data"));

        let mut generator = NetlistGenerator::new(&state);
        generator.generate();

        assert_eq!(
            generator.net_at(Point::new(0, 0)).map(|net| net.id),
            generator.net_at(Point::new(0, 100)).map(|net| net.id)
        );
    }

    /// A label naming a net "gnd" maps to SPICE node 0.
    #[test]
    fn gnd_label_maps_to_node_zero() {
        let mut state = SchematicState::default();
        state
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .net_labels
            .push(NetLabel::new(1, Point::new(20, 0), "GND"));

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
