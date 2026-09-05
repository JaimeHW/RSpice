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

use std::any::Any;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use crate::product::AnalysisInstanceId;
#[cfg(test)]
use crate::state::Wire;
use crate::state::{
    CellViewRef, Component, ComponentType, DesignVariable, DesignVariableScope, InstancePath,
    Point, SchematicState,
};

mod connectivity;
pub(crate) mod extraction;
mod formatting;
mod header;
mod instances;
mod magnetics;
mod master_index;
mod models;
mod subcircuits;
mod vector_display;
mod vector_names;
mod vector_nets;
mod xspice;

pub(crate) use instances::independent_source_card;
pub use master_index::{EmissionRow, NetlistDefect};
use master_index::{MasterIndex, validate_occurrence_interface};
pub use subcircuits::HierarchySource;
pub(crate) use vector_display::bus_notations;
pub(crate) use vector_names::deck_bit_name;

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

    /// Any errors that prevent simulation. Every typed defect below is also
    /// rendered here, so a consumer that only reads strings still sees it.
    pub errors: Vec<String>,

    /// Typed defects, for surfaces that attach a repair to the kind rather
    /// than matching on the text of an error.
    pub defects: Vec<NetlistDefect>,

    /// Which master each occurrence was emitted against, for every occurrence
    /// in the deck. This is what lets a receipt name the executed hierarchy
    /// without re-deriving it from the netlist text.
    pub emission_map: Vec<EmissionRow>,
}

/// Generate a flat SPICE netlist from a schematic.
///
/// Execution always goes through [`generate_netlist_hierarchical`], which
/// resolves placed project cells into `.SUBCKT` definitions. This flat form
/// exists for the tests that exercise the generator without a workspace.
#[cfg(test)]
pub fn generate_netlist(schematic: &SchematicState) -> NetlistResult {
    finish_generation(NetlistGenerator::new(schematic), schematic, &[], &[])
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
        defects: generator.defects.clone(),
        emission_map: generator.emission_map.clone(),
    }
}

//=============================================================================
// Design net summary (navigation UIs)
//=============================================================================

/// Electrical class of a design net.
///
/// The class is derived from declared design intent — the resolved ground
/// net, a declared `dir=supply` interface port, or a power net label — and
/// never guessed from the spelling of a name alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetClass {
    /// The simulation reference node.
    Ground,
    /// A declared supply/power rail.
    Supply,
    /// Any other conductor.
    Signal,
}

impl NetClass {
    /// Lower-case vocabulary shared with the inspector and the navigator.
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Ground => "ground",
            Self::Supply => "supply",
            Self::Signal => "signal",
        }
    }
}

/// One component terminal bound to a net.
#[derive(Debug, Clone)]
pub struct NetTerminal {
    /// Owning component, for selection and cross-probing.
    pub component_id: u64,
    /// Instance reference designator as drawn.
    pub reference: String,
    /// Terminal name on that instance.
    pub pin: String,
}

/// One electrical net of the open schematic, summarized for navigation:
/// the rail's Nets segment, the net inspector, cross-probe highlighting,
/// search.
#[derive(Debug, Clone)]
pub struct DesignNet {
    /// SPICE name ("0", a label/port name, or autonamed `netN`).
    pub name: String,
    /// `true` when the name comes from an authored label, interface port, or
    /// ground symbol rather than the netlister's isolated-node fallback.
    pub authored_name: bool,
    /// Electrical class of the conductor.
    pub class: NetClass,
    /// Component terminals on this net, in document order.
    pub terminals: Vec<NetTerminal>,
    /// Declared direction when the net is an interface port of the cell.
    pub port: Option<crate::state::PortDirection>,
    /// Wires belonging to this net (for canvas highlighting).
    pub wire_ids: Vec<u64>,
}

impl DesignNet {
    /// Component terminals bound to this net.
    pub fn pin_count(&self) -> usize {
        self.terminals.len()
    }

    /// `true` when the net is an interface port of the cell.
    pub const fn is_port(&self) -> bool {
        self.port.is_some()
    }
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

/// Net summary of one cell view of a frozen design projection, extracted once
/// and then retained by the projection itself.
///
/// The projection keeps the slot type-erased so the design model never names
/// a generator type; the downcast back to [`DesignNet`] belongs here, where
/// the type is owned. A cell view the projection does not carry has no nets
/// rather than an error: the projection is the authority on which views exist.
pub fn projection_nets(
    libraries: &crate::state::LibraryManager,
    projection: &crate::state::workspace::DesignProjection,
    cell_view_key: &str,
) -> Arc<Vec<DesignNet>> {
    let extract = || -> Arc<Vec<DesignNet>> {
        Arc::new(
            projection
                .schematic_buffers()
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(cell_view_key))
                .map(|(_, schematic)| {
                    design_nets_with_hierarchy(
                        schematic,
                        &HierarchySource::from_design_projection(libraries, projection),
                    )
                })
                .unwrap_or_default(),
        )
    };
    projection
        .memo_nets(cell_view_key, || extract() as Arc<dyn Any + Send + Sync>)
        .downcast::<Vec<DesignNet>>()
        .unwrap_or_else(|_| extract())
}

/// Resolved terminal names for every placed component, in the same order and
/// with the same authored-symbol authority used by hierarchical netlisting.
///
/// Publication uses this alongside the hierarchy-aware net summary so
/// disconnected pins remain visible without inventing names or falling back
/// to generic pin numbers when an authored symbol is available.
pub(crate) fn component_pin_names_with_hierarchy(
    schematic: &SchematicState,
    hierarchy: &HierarchySource<'_>,
) -> HashMap<u64, Vec<String>> {
    let generator = NetlistGenerator::with_hierarchy(schematic, hierarchy);
    schematic
        .components
        .iter()
        .map(|component| {
            (
                component.id,
                generator
                    .component_terminal_positions(component)
                    .into_iter()
                    .map(|(name, _)| name)
                    .collect(),
            )
        })
        .collect()
}

fn collect_design_nets(
    schematic: &SchematicState,
    generator: &mut NetlistGenerator<'_>,
) -> Vec<DesignNet> {
    generator.extract_connectivity();

    let ports: HashMap<String, crate::state::PortDirection> = schematic
        .interface_ports()
        .iter()
        .map(|port| (port.name.to_ascii_lowercase(), port.direction))
        .collect();

    // Terminals are collected in document order so the inspector's
    // connectivity table reads the same way twice for the same drawing.
    let mut terminals: HashMap<usize, Vec<NetTerminal>> = HashMap::new();
    for component in &schematic.components {
        for (pin, position) in generator.component_terminal_positions(component) {
            if let Some(net) = generator.net_at(position) {
                terminals.entry(net.id).or_default().push(NetTerminal {
                    component_id: component.id,
                    reference: component.name.clone(),
                    pin,
                });
            }
        }
    }
    // A power label anywhere on the net declares it a supply rail.
    let mut power_labelled: HashSet<usize> = HashSet::new();
    for label in &schematic.net_labels {
        if label.is_power_net()
            && !label.is_ground()
            && let Some(net) = generator.net_at(label.pos)
        {
            power_labelled.insert(net.id);
        }
    }
    let ground_net = generator.ground_net_id();

    let mut nets: Vec<DesignNet> = generator
        .nets()
        .iter()
        .map(|net| {
            let name = net.spice_name();
            let authored_name = net.label.is_some();
            let port = ports.get(&name.to_ascii_lowercase()).copied();
            let class = if ground_net == Some(net.id) || name == "0" {
                NetClass::Ground
            } else if port == Some(crate::state::PortDirection::Supply)
                || power_labelled.contains(&net.id)
            {
                NetClass::Supply
            } else {
                NetClass::Signal
            };
            DesignNet {
                authored_name,
                class,
                port,
                terminals: terminals.remove(&net.id).unwrap_or_default(),
                wire_ids: net.wires.clone(),
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
        (
            !a.is_port(),
            autonamed(&a.name),
            a.name.to_ascii_lowercase(),
        )
            .cmp(&(
                !b.is_port(),
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

    /// Conductors belonging to this net, for a finding or a highlight that has
    /// to name the drawn object rather than the node.
    pub wires: Vec<u64>,
}

impl Net {
    /// Create a new empty net
    pub fn new(id: usize) -> Self {
        Self {
            id,
            points: HashSet::new(),
            label: None,
            wires: Vec::new(),
        }
    }

    /// Add a point to this net
    pub fn add_point(&mut self, point: Point) {
        self.points.insert(point);
    }

    /// Merge another net into this one
    pub fn merge(&mut self, other: &Net) {
        self.points.extend(&other.points);
        self.wires.extend(&other.wires);
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
    /// Exact path of `schematic` inside a frozen configuration plan.
    /// Legacy generation keeps the design root and ignores it.
    hierarchy_path: InstancePath,
    /// Master identity for the whole deck, published by the definition pass
    /// and shared with every nested generator so an X-line and the definition
    /// it refers to are named by one authority.
    masters: Option<std::rc::Rc<MasterIndex<'a>>>,
    /// Typed defects raised while generating.
    defects: Vec<NetlistDefect>,
    /// Which master each occurrence was emitted against.
    emission_map: Vec<EmissionRow>,
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
            hierarchy_path: InstancePath::root(),
            masters: None,
            defects: Vec::new(),
            emission_map: Vec::new(),
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

    /// A generator for one master's body, at the occurrence the deck emits it
    /// at, sharing the deck's one master index.
    fn with_master_index(
        schematic: &'a SchematicState,
        hierarchy: &'a HierarchySource<'a>,
        hierarchy_path: InstancePath,
        masters: std::rc::Rc<MasterIndex<'a>>,
    ) -> Self {
        let mut generator = Self::with_hierarchy(schematic, hierarchy);
        generator.hierarchy_path = hierarchy_path;
        generator.masters = Some(masters);
        generator
    }

    /// The path of a placed instance below this generator's own.
    ///
    /// An instance the hierarchy grammar cannot name has no path in the plan,
    /// so this reports rather than inventing one: emitting the cell under a
    /// repaired name would bind the deck to an instance the design does not
    /// contain.
    fn child_hierarchy_path(&self, component: &Component) -> Result<InstancePath, String> {
        self.hierarchy_path
            .child(&component.name)
            .map_err(|error| error.to_string())
    }

    /// The frozen resolved binding when a plan governs this instance.
    ///
    /// The plan wins wherever it speaks: falling back to the placed binding for
    /// an instance the plan resolved would make validation and executable bytes
    /// disagree. An instance the plan does not carry — a cell view checked on
    /// its own, or an occurrence that never resolved — reports through the
    /// placed binding, which is what names the master that is missing.
    fn effective_library_binding<'b>(
        &'b self,
        component: &'b Component,
    ) -> Result<Option<&'b crate::state::LibraryCellInstance>, String> {
        let Some(placed) = component.library_cell.as_ref() else {
            return Ok(None);
        };
        let Some(hierarchy) = self.hierarchy else {
            return Ok(Some(placed));
        };
        let path = self.child_hierarchy_path(component)?;
        let Some(resolved) = hierarchy.execution_binding(&path) else {
            return Ok(Some(placed));
        };
        resolved.materialized_binding().map(Some).ok_or_else(|| {
            format!(
                "configuration execution plan did not materialize instance '{}' at {}",
                component.name, path
            )
        })
    }

    /// Consume the generated lines (subcircuit-body assembly).
    pub(self) fn take_lines(&mut self) -> Vec<String> {
        std::mem::take(&mut self.lines)
    }

    /// Generate the netlist body with no analysis or parameter cards.
    /// Production always goes through `finish_generation`, which supplies
    /// both; the generator's own tests use this bare form.
    #[cfg(test)]
    pub fn generate(&mut self) -> String {
        self.generate_with_analysis_and_parameters(&[], &[])
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
        self.defects.clear();
        self.emission_map.clear();
        self.masters = None;
    }

    /// Read-only warnings collected during generation.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Read-only errors collected during generation.
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn generate_with_analysis_and_parameters(
        &mut self,
        analysis_lines: &[String],
        parameter_lines: &[String],
    ) -> String {
        self.reset_generation_state();

        // Phase 1: Adopt the design's one connectivity extraction — geometry,
        // interface ports, labels, typed bus members and ground, already
        // resolved against the single label-winner rule.
        self.extract_connectivity();

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

        // Phase 6b: Verilog-A source registration is deck-global. Nested
        // hierarchy generators may discover the same source independently;
        // publish each exact identity once before any subcircuit definition.
        self.hoist_veriloga_directives();

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
        extraction::terminal_positions(component, self.hierarchy)
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
}
//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Cell, CellViewRef,
        ConnectivityAliasGroup, ConnectivityContract, ConnectivityPolicy, DesignNote,
        DesignNoteKind, DialectAliasCatalog, DocumentationShape, DocumentationShapeGeometry,
        GlobalAliasComparisonPolicy, GlobalNetPromotionPolicy, Library, LibraryCellInstance,
        LibraryManager, NetLabel, PortDirection, PortSpec, SymbolDocument, SymbolPin,
        TechnologyGlobalNetCatalog, View, ViewType,
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
            .map(|net| (net.name.as_str(), net.pin_count()))
            .collect();

        assert_eq!(pin_counts.get("vin"), Some(&1));
        assert_eq!(pin_counts.get("vout"), Some(&1));
    }

    /// A label sitting on a wire names that net in the netlist.
    #[test]
    fn net_label_names_the_node() {
        let mut state = SchematicState::default();
        crate::workbench::examples::load_example("RC Lowpass Filter", &mut state);

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

    #[test]
    fn any_angle_segment_interior_attachments_share_the_generated_net() {
        let mut state = SchematicState::default();
        state
            .wires
            .push(Wire::segment(1, Point::new(0, 0), Point::new(40, 40)));
        state
            .wires
            .push(Wire::segment(2, Point::new(20, 20), Point::new(20, 60)));
        state
            .net_labels
            .push(NetLabel::new(3, Point::new(30, 30), "diag"));

        let mut generator = NetlistGenerator::new(&state);
        generator.generate();
        let diagonal = generator
            .net_at(Point::new(0, 0))
            .expect("diagonal endpoint has a net")
            .id;
        assert_eq!(
            generator.net_at(Point::new(20, 60)).map(|net| net.id),
            Some(diagonal),
            "a wire endpoint on a diagonal interior is an electrical T attachment"
        );
        assert_eq!(
            generator.net_at(Point::new(30, 30)).map(|net| net.id),
            Some(diagonal),
            "a label on a diagonal interior names that net"
        );
        assert_eq!(
            generator.net_at(Point::new(30, 30)).unwrap().spice_name(),
            "diag"
        );
    }

    #[test]
    fn diagonal_crossings_require_an_explicit_junction() {
        let crossing = Point::new(20, 20);
        let mut state = SchematicState::default();
        state
            .wires
            .push(Wire::segment(1, Point::new(0, 0), Point::new(40, 40)));
        state
            .wires
            .push(Wire::segment(2, Point::new(0, 40), Point::new(40, 0)));

        let mut disconnected = NetlistGenerator::new(&state);
        disconnected.generate();
        assert_ne!(
            disconnected.net_at(Point::new(0, 0)).map(|net| net.id),
            disconnected.net_at(Point::new(0, 40)).map(|net| net.id),
            "an unmarked interior crossing is not a connection"
        );

        state.add_junction(crossing);
        let mut connected = NetlistGenerator::new(&state);
        connected.generate();
        assert_eq!(
            connected.net_at(Point::new(0, 0)).map(|net| net.id),
            connected.net_at(Point::new(0, 40)).map(|net| net.id),
            "an explicit junction connects both diagonal segments"
        );
    }

    #[test]
    fn transient_source_formatter_uses_the_typed_property_contract() {
        let schematic = SchematicState::default();
        let generator = NetlistGenerator::new(&schematic);

        let mut voltage_pulse =
            Component::new(1, ComponentType::VoltageSourcePulse, Point::origin())
                .with_name_value("VPULSE", "0.2");
        voltage_pulse.params = "v2=1.8 td=3n tr=4n tf=5n pw=6n per=7n".to_owned();
        assert_eq!(
            generator.format_source_value(&voltage_pulse),
            "PULSE(0.2 1.8 3n 4n 5n 6n 7n)"
        );

        let mut current_pulse =
            Component::new(2, ComponentType::CurrentSourcePulse, Point::origin())
                .with_name_value("IPULSE", "2m");
        current_pulse.params = "i2=5m td=8n tr=9n tf=10n pw=11n per=12n".to_owned();
        assert_eq!(
            generator.format_source_value(&current_pulse),
            "PULSE(2m 5m 8n 9n 10n 11n 12n)"
        );

        let mut current_sin = Component::new(3, ComponentType::CurrentSourceSin, Point::origin())
            .with_name_value("ISIN", "3m");
        current_sin.params = "ia=4m freq=5Meg td=6n theta=7 phase=8".to_owned();
        assert_eq!(
            generator.format_source_value(&current_sin),
            "SIN(3m 4m 5Meg 6n 7 8)"
        );

        let mut current_exp = Component::new(4, ComponentType::CurrentSourceExp, Point::origin())
            .with_name_value("IEXP", "1m");
        current_exp.params = "i2=9m td1=1u tau1=2u td2=3u tau2=4u".to_owned();
        assert_eq!(
            generator.format_source_value(&current_exp),
            "EXP(1m 9m 1u 2u 3u 4u)"
        );
    }

    /// Positional tails are emitted only as far as the last field that carries
    /// information, so adding PHASE to the pulse sheet cannot append ` 0` to
    /// every deck RSpice has ever written.
    #[test]
    fn optional_waveform_tails_appear_only_when_they_are_set() {
        let schematic = SchematicState::default();
        let generator = NetlistGenerator::new(&schematic);

        let mut pulse = Component::new(1, ComponentType::VoltageSourcePulse, Point::origin())
            .with_name_value("VPULSE", "0");
        pulse.params = "v2=1.8 td=0 tr=1n tf=1n pw=5n per=10n phase=90".to_owned();
        assert_eq!(
            generator.format_source_value(&pulse),
            "PULSE(0 1.8 0 1n 1n 5n 10n 90)"
        );

        let mut sffm = Component::new(2, ComponentType::CurrentSourceSffm, Point::origin())
            .with_name_value("ISFFM", "0");
        sffm.params = "va=1m fc=1Meg mdi=3 fm=2k".to_owned();
        assert_eq!(
            generator.format_source_value(&sffm),
            "SFFM(0 1m 1Meg 3 2k)",
            "an unset delay and phases leave the classic five-argument form"
        );
        sffm.params = "va=1m fc=1Meg mdi=3 fm=2k phasec=45".to_owned();
        assert_eq!(
            generator.format_source_value(&sffm),
            "SFFM(0 1m 1Meg 3 2k 0 0 45)",
            "a set carrier phase drags its positional predecessors along"
        );
    }

    /// The app spelled `SFFM`'s modulating frequency `fs` and the engine spells
    /// it `fm`. The rename is a load-time migration, so a project authored
    /// under either spelling emits the same bytes and neither is a second
    /// parameter the emitter has to know about.
    #[test]
    fn an_sffm_source_authored_under_either_spelling_emits_the_same_card() {
        let schematic = SchematicState::default();
        let generator = NetlistGenerator::new(&schematic);

        let mut authored = Component::new(2, ComponentType::VoltageSourceSffm, Point::origin())
            .with_name_value("V2", "0");
        authored.params = "va=1m fc=1Meg mdi=3 fs=2k".to_owned();
        let legacy: Component =
            ron::from_str(&ron::ser::to_string(&authored).expect("serialize")).expect("decode");

        let mut current = Component::new(2, ComponentType::VoltageSourceSffm, Point::origin())
            .with_name_value("V2", "0");
        current.params = "va=1m fc=1Meg mdi=3 fm=2k".to_owned();

        assert_eq!(
            generator.format_source_value(&legacy),
            generator.format_source_value(&current)
        );
        assert_eq!(
            generator.format_source_value(&legacy),
            "SFFM(0 1m 1Meg 3 2k)"
        );
    }

    /// AM and PAT are the two families the engine has always parsed but no
    /// schematic component could produce.
    #[test]
    fn modulated_and_pattern_sources_emit_their_engine_spelling() {
        let schematic = SchematicState::default();
        let generator = NetlistGenerator::new(&schematic);

        let mut am = Component::new(1, ComponentType::VoltageSourceAm, Point::origin())
            .with_name_value("VAM", "0");
        am.params = "vmo=1 vma=0.5 fm=1k fc=1Meg".to_owned();
        assert_eq!(generator.format_source_value(&am), "AM(0 1 0.5 1k 1Meg)");

        let mut pat = Component::new(2, ComponentType::VoltageSourcePat, Point::origin())
            .with_name_value("VPAT", "1.8");
        pat.params = "vlo=0 td=1n tr=100p tf=100p tsample=2n data=1011".to_owned();
        assert_eq!(
            generator.format_source_value(&pat),
            "PAT(1.8 0 1n 100p 100p 2n b1011)",
            "a bit string typed without its leading B is normalized, not rejected"
        );

        pat.params.push_str(" repeat_count=-1");
        assert_eq!(
            generator.format_source_value(&pat),
            "PAT(1.8 0 1n 100p 100p 2n b1011 R=-1)"
        );

        let mut noise = Component::new(3, ComponentType::VoltageSourceNoise, Point::origin())
            .with_name_value("VNOISE", "10n");
        noise.params = "nt=1u nalpha=0 namp=0 dc=0".to_owned();
        assert_eq!(
            generator.format_source_value(&noise),
            "DC 0 TRNOISE(10n 1u 0 0)"
        );
    }

    #[test]
    fn transient_source_formatter_preserves_legacy_waveform_literals_once() {
        let schematic = SchematicState::default();
        let generator = NetlistGenerator::new(&schematic);
        let pulse = Component::new(1, ComponentType::VoltageSourcePulse, Point::origin())
            .with_name_value("VIN", "pulse(0 1.8 0 1n 1n 5n 10n)");

        assert_eq!(
            generator.format_source_value(&pulse),
            "pulse(0 1.8 0 1n 1n 5n 10n)"
        );
    }

    #[test]
    fn design_notes_never_change_generated_spice_or_connectivity() {
        let mut baseline = SchematicState::default();
        crate::workbench::examples::load_example("RC Lowpass Filter", &mut baseline);
        let expected = generate_netlist(&baseline);
        let mut documented = baseline;
        documented.design_notes.push(
            DesignNote::new(
                90_001,
                Point::new(25, 30),
                DesignNoteKind::PropertyDisplay,
                "${component_count} components",
            )
            .unwrap(),
        );

        let actual = generate_netlist(&documented);
        assert_eq!(actual.netlist, expected.netlist);
        assert_eq!(actual.nets.len(), expected.nets.len());
        for (name, expected_points) in &expected.nets {
            let mut expected_points = expected_points.clone();
            expected_points.sort_by_key(|point| (point.x, point.y));
            let mut actual_points = actual.nets.get(name).cloned().unwrap_or_default();
            actual_points.sort_by_key(|point| (point.x, point.y));
            assert_eq!(actual_points, expected_points, "net {name}");
        }
        assert_eq!(actual.warnings, expected.warnings);
        assert_eq!(actual.errors, expected.errors);
    }

    #[test]
    fn all_documentation_shapes_leave_spice_and_connectivity_byte_for_byte_unchanged() {
        let mut baseline = SchematicState::default();
        crate::workbench::examples::load_example("RC Lowpass Filter", &mut baseline);
        let expected = generate_netlist(&baseline);
        let mut documented = baseline;
        documented.documentation_shapes = [
            DocumentationShapeGeometry::Rectangle {
                first: Point::new(-80, -40),
                opposite: Point::new(20, 30),
            },
            DocumentationShapeGeometry::Line {
                start: Point::new(-25, 70),
                end: Point::new(65, 105),
            },
            DocumentationShapeGeometry::Polygon {
                points: vec![
                    Point::new(100, -20),
                    Point::new(170, 10),
                    Point::new(145, 85),
                    Point::new(80, 45),
                ],
            },
            DocumentationShapeGeometry::Arc {
                start: Point::new(-100, 160),
                through: Point::new(-50, 110),
                end: Point::new(0, 160),
            },
            DocumentationShapeGeometry::Callout {
                tip: Point::new(90, 150),
                elbow: Point::new(130, 125),
                box_corner: Point::new(230, 190),
            },
        ]
        .into_iter()
        .enumerate()
        .map(|(index, geometry)| {
            DocumentationShape::new(90_100 + index as u64, geometry)
                .expect("documentation shape fixture is valid")
        })
        .collect();

        let actual = generate_netlist(&documented);
        assert_eq!(actual.netlist.as_bytes(), expected.netlist.as_bytes());
        assert_eq!(actual.nets.len(), expected.nets.len());
        for (name, expected_points) in &expected.nets {
            let mut expected_points = expected_points.clone();
            expected_points.sort_by_key(|point| (point.x, point.y));
            let mut actual_points = actual
                .nets
                .get(name)
                .unwrap_or_else(|| panic!("missing generated net {name}"))
                .clone();
            actual_points.sort_by_key(|point| (point.x, point.y));
            assert_eq!(actual_points, expected_points, "net {name}");
        }
        assert_eq!(actual.point_to_net, expected.point_to_net);
        assert_eq!(actual.net_segments.len(), expected.net_segments.len());
        for (name, expected_segments) in &expected.net_segments {
            let canonicalize = |segments: &[(Point, Point)]| {
                let mut segments: Vec<_> = segments
                    .iter()
                    .map(|&(first, second)| {
                        if (first.x, first.y) <= (second.x, second.y) {
                            (first, second)
                        } else {
                            (second, first)
                        }
                    })
                    .collect();
                segments.sort_by_key(|(first, second)| (first.x, first.y, second.x, second.y));
                segments
            };
            let actual_segments = actual
                .net_segments
                .get(name)
                .unwrap_or_else(|| panic!("missing generated net segments for {name}"));
            assert_eq!(
                canonicalize(actual_segments),
                canonicalize(expected_segments),
                "net segments for {name}"
            );
        }
        assert_eq!(actual.warnings, expected.warnings);
        assert_eq!(actual.errors, expected.errors);
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

    fn add_scalar_bus_tap(
        state: &mut SchematicState,
        bus_id: u64,
        tap_id: u64,
        bus_y: i32,
        wire_id: u64,
        wire_y: i32,
        member: &str,
    ) {
        let bus = Bus::segment(
            bus_id,
            Point::new(0, bus_y),
            Point::new(40, bus_y),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            tap_id,
            &bus,
            Point::new(20, bus_y),
            Point::new(20, wire_y),
            BusSlice::parse(member).unwrap(),
            if wire_y < bus_y {
                BusTapOrientation::Up
            } else {
                BusTapOrientation::Down
            },
        )
        .unwrap();
        state.buses.push(bus);
        state.bus_taps.push(tap);
        state.wires.push(Wire::segment(
            wire_id,
            Point::new(0, wire_y),
            Point::new(40, wire_y),
        ));
    }

    #[test]
    fn scalar_bus_tap_applies_exact_member_name_and_accepts_matching_label() {
        let mut state = SchematicState::default();
        add_scalar_bus_tap(&mut state, 10, 11, -20, 12, 0, "DATA[3]");
        state
            .net_labels
            .push(NetLabel::new(13, Point::new(30, 0), "DATA[3]"));

        let mut generator = NetlistGenerator::new(&state);
        generator.generate();

        // The label agrees with the tap, so it is not a conflict — and agreeing
        // does not rewrite the node back into delimiters the engine drops.
        assert!(generator.errors().is_empty(), "{:?}", generator.errors());
        assert_eq!(
            generator
                .net_at(Point::new(20, 0))
                .map(|net| net.spice_name()),
            Some("DATA#3".to_owned())
        );
    }

    #[test]
    fn scalar_bus_tap_conflicting_free_form_label_is_blocking() {
        let mut state = SchematicState::default();
        add_scalar_bus_tap(&mut state, 20, 21, -20, 22, 0, "DATA[3]");
        state
            .net_labels
            .push(NetLabel::new(23, Point::new(30, 0), "FOO"));

        let result = generate_netlist(&state);

        assert!(result.errors.iter().any(|error| {
            error.contains("FOO") && error.contains("DATA[3]") && error.contains("conflicts")
        }));
    }

    #[test]
    fn identical_scalar_bus_members_merge_disjoint_wire_nets() {
        let mut state = SchematicState::default();
        add_scalar_bus_tap(&mut state, 30, 31, -20, 32, 0, "DATA[3]");
        add_scalar_bus_tap(&mut state, 40, 41, 80, 42, 100, "DATA[3]");

        let mut generator = NetlistGenerator::new(&state);
        generator.generate();

        assert!(generator.errors().is_empty(), "{:?}", generator.errors());
        assert_eq!(
            generator.net_at(Point::new(0, 0)).map(|net| net.id),
            generator.net_at(Point::new(0, 100)).map(|net| net.id)
        );
    }

    #[test]
    fn different_scalar_bus_members_remain_distinct() {
        let mut state = SchematicState::default();
        add_scalar_bus_tap(&mut state, 50, 51, -20, 52, 0, "DATA[3]");
        add_scalar_bus_tap(&mut state, 60, 61, 80, 62, 100, "DATA[4]");

        let mut generator = NetlistGenerator::new(&state);
        generator.generate();

        assert!(generator.errors().is_empty(), "{:?}", generator.errors());
        assert_ne!(
            generator.net_at(Point::new(0, 0)).map(|net| net.id),
            generator.net_at(Point::new(0, 100)).map(|net| net.id)
        );
    }

    #[test]
    fn multi_bit_bus_tap_never_enters_scalar_spice_connectivity() {
        let source = Bus::segment(
            70,
            Point::new(0, 0),
            Point::new(40, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let destination = Bus::segment(
            71,
            Point::new(0, 20),
            Point::new(40, 20),
            Some(BusDeclaration::parse("DATA[3:0]").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            72,
            &source,
            Point::new(20, 0),
            Point::new(20, 20),
            BusSlice::parse("DATA[3:0]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut state = SchematicState::default();
        state.buses = vec![source, destination];
        state.bus_taps.push(tap);

        let result = generate_netlist(&state);

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(result.nets.keys().all(|name| !name.starts_with("DATA[")));
        assert!(result.point_to_net.is_empty());
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

    fn netlist_for(components: Vec<Component>) -> String {
        let mut state = SchematicState::default();
        state.components = components;
        generate_netlist(&state).netlist
    }

    /// The authored text of a foundation card, read from the library rather
    /// than restated here: a test that spells out `BF=100` is a third place
    /// the value lives and the first place it goes stale.
    fn foundation_card(name: &str) -> &'static str {
        rspice_core::library::foundation_card_source(name).expect("foundation card is embedded")
    }

    /// A bare diode placement must produce an executable deck: the foundation
    /// junction card, referenced from the instance line and carried verbatim so
    /// the deck stands on its own.
    #[test]
    fn bare_diode_gets_the_foundation_junction_model_card() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::Diode, Point::origin()).with_name_value("D1", ""),
        ]);
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("D1 ") && l.ends_with(" RSPICE_DIODE")),
            "diode line should reference the foundation card:\n{netlist}"
        );
        assert!(
            netlist.contains(foundation_card("RSPICE_DIODE")),
            "{netlist}"
        );
    }

    /// The card written for an unbound device is the library's own text, and
    /// one card serves every placement of the family. Minting a card per
    /// instance is what let a schematic's bare transistor mean something the
    /// model library had never heard of.
    #[test]
    fn unbound_devices_of_one_family_share_the_foundation_card() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::NpnBjt, Point::origin()).with_name_value("Q1", ""),
            Component::new(2, ComponentType::NpnBjt, Point::new(200, 0)).with_name_value("Q2", ""),
            Component::new(3, ComponentType::PnpBjt, Point::new(400, 0)).with_name_value("Q3", ""),
        ]);
        for instance in ["Q1", "Q2"] {
            assert!(
                netlist
                    .lines()
                    .any(|l| l.starts_with(&format!("{instance} ")) && l.ends_with(" RSPICE_NPN")),
                "{netlist}"
            );
        }
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("Q3 ") && l.ends_with(" RSPICE_PNP")),
            "{netlist}"
        );
        assert_eq!(
            netlist
                .lines()
                .filter(|l| l.trim_start().to_ascii_uppercase().starts_with(".MODEL "))
                .count(),
            2,
            "two families placed, two cards expected:\n{netlist}"
        );
        assert!(netlist.contains(foundation_card("RSPICE_NPN")), "{netlist}");
        assert!(netlist.contains(foundation_card("RSPICE_PNP")), "{netlist}");
    }

    /// An explicit diode model binding is trusted verbatim with no card.
    #[test]
    fn explicit_diode_model_is_trusted_without_a_generated_card() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::Diode, Point::origin()).with_name_value("D1", "BAV99"),
        ]);
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("D1 ") && l.ends_with(" BAV99")),
            "{netlist}"
        );
        assert!(!netlist.contains(".MODEL"), "{netlist}");
    }

    /// MOSFET model overrides must reach the deck instead of being replaced
    /// by a foundation card — the model-driven path for advanced compact
    /// models depends on this.
    #[test]
    fn mosfet_explicit_model_override_is_honored() {
        let mut mos =
            Component::new(1, ComponentType::Nmos, Point::origin()).with_name_value("M1", "");
        mos.params = "model=psp_nch w=1u l=100n".to_owned();
        let netlist = netlist_for(vec![mos]);
        let line = netlist
            .lines()
            .find(|l| l.starts_with("M1 "))
            .expect("mosfet line");
        assert!(line.contains(" psp_nch"), "{netlist}");
        assert!(line.contains("w=1u"), "{netlist}");
        assert!(!line.contains("model="), "{netlist}");
        assert!(!netlist.contains("RSPICE_NMOS"), "{netlist}");
    }

    /// JFET model overrides are honored the same way.
    #[test]
    fn jfet_explicit_model_override_is_honored() {
        let mut jfet =
            Component::new(1, ComponentType::Njfet, Point::origin()).with_name_value("J1", "");
        jfet.params = "model=J2N5484".to_owned();
        let netlist = netlist_for(vec![jfet]);
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("J1 ") && l.ends_with(" J2N5484")),
            "{netlist}"
        );
        assert!(!netlist.contains("RSPICE_NJFET"), "{netlist}");
    }

    /// VDMOS placements emit a real power-MOSFET model card (they used to
    /// fall through the catch-all with no card at all).
    #[test]
    fn vdmos_emits_instance_and_model_card() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::NVdmos, Point::origin()).with_name_value("M1", ""),
            Component::new(2, ComponentType::PVdmos, Point::new(200, 0)).with_name_value("M2", ""),
        ]);
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("M1 ") && l.ends_with(" RSPICE_NVDMOS")),
            "{netlist}"
        );
        assert!(
            netlist.contains(foundation_card("RSPICE_NVDMOS")),
            "{netlist}"
        );
        assert!(
            netlist.contains(foundation_card("RSPICE_PVDMOS")),
            "{netlist}"
        );
    }

    /// H/F elements reference a controlling V source by name; the schematic's
    /// control pins become a synthesized 0 V sense source.
    #[test]
    fn current_controlled_sources_synthesize_a_sense_source() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::Ccvs, Point::origin()).with_name_value("H1", "100"),
            Component::new(2, ComponentType::Cccs, Point::new(200, 0)).with_name_value("F1", "2.5"),
        ]);
        let h_line = netlist
            .lines()
            .find(|l| l.starts_with("H1 "))
            .expect("ccvs line");
        assert!(h_line.contains(" VSENSE_H1 100"), "{netlist}");
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("VSENSE_H1 ") && l.ends_with(" 0")),
            "{netlist}"
        );
        let f_line = netlist
            .lines()
            .find(|l| l.starts_with("F1 "))
            .expect("cccs line");
        assert!(f_line.contains(" VSENSE_F1 2.5"), "{netlist}");
        // Exactly name + 2 nodes + control + gain: no trailing tokens, the
        // core rejects any tail after the gain.
        assert_eq!(h_line.split_whitespace().count(), 5, "{netlist}");
    }

    /// The random-telegraph tail reaches the engine, positionally, and the
    /// engine reads back exactly what the sheet was given.
    ///
    /// RTSAM/RTSCAPT/RTSEMT are the fifth, sixth and seventh positional
    /// arguments of the card (`netlist/parser/source_specs.rs:578-582`), and
    /// the transient lowers them into a real two-state telegraph rather than
    /// storing and ignoring them (`engine/transient/noise.rs:213-281`).
    #[test]
    fn noise_source_emits_the_rts_tail_the_engine_reads_back() {
        let mut noise = Component::new(1, ComponentType::VoltageSourceNoise, Point::origin())
            .with_name_value("V1", "10n");
        noise.params = "nt=1u nalpha=0 namp=0 dc=0 rtsam=5m rtscapt=2u rtsemt=3u".to_owned();
        let netlist = netlist_for(vec![noise]);
        let card = netlist
            .lines()
            .find(|line| line.starts_with("V1 "))
            .unwrap_or_else(|| panic!("{netlist}"));
        assert!(
            card.ends_with("DC 0 TRNOISE(10n 1u 0 0 5m 2u 3u)"),
            "{card}"
        );

        let parsed =
            rspice_core::netlist::parse_netlist(&netlist).expect("engine must accept the card");
        let spec = parsed
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                rspice_core::netlist::ElementKind::VoltageSource(spec) if element.name == "V1" => {
                    Some(spec)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("{netlist}"));
        // The card carries a DC level as well as the waveform, so the engine
        // reads it back as a `DcTransient` wrapping the TRNOISE spec.
        let rspice_core::netlist::SourceSpec::DcTransient { transient, .. } = spec else {
            panic!("a TRNOISE card with a DC level parses as a DC + transient pair: {spec:?}");
        };
        assert!(
            matches!(
                transient.as_ref(),
                rspice_core::netlist::SourceSpec::TrNoise {
                    rts_amplitude,
                    rts_capture,
                    rts_emit,
                    ..
                } if *rts_amplitude == 5e-3 && *rts_capture == 2e-6 && *rts_emit == 3e-6
            ),
            "{transient:?}"
        );
    }

    /// An RTS tail with one mean time still at zero is a hard parse error, not
    /// a silently disabled telegraph (`source_specs.rs:606-615`). The sheet
    /// refuses the same pair before it can be emitted — see
    /// `half_an_rts_dwell_pair_is_refused_and_neither_half_is` — and this is
    /// the pin that the two agree about which pairs are refusable.
    #[test]
    fn a_half_authored_rts_dwell_pair_is_refused_by_the_engine() {
        let mut noise = Component::new(1, ComponentType::VoltageSourceNoise, Point::origin())
            .with_name_value("V1", "10n");
        noise.params = "nt=1u rtsam=5m rtscapt=2u rtsemt=0".to_owned();
        let netlist = netlist_for(vec![noise]);
        let error = rspice_core::netlist::parse_netlist(&netlist)
            .expect_err("the engine refuses half an RTS dwell pair")
            .to_string();
        assert!(
            error.contains("capture and emission mean times"),
            "{error}\n{netlist}"
        );
    }

    /// An RTS tail nobody authored stays off the card entirely, so a project
    /// written before these fields existed emits exactly the deck it always
    /// did.
    #[test]
    fn an_unauthored_rts_tail_stays_off_the_card() {
        let mut noise = Component::new(1, ComponentType::VoltageSourceNoise, Point::origin())
            .with_name_value("V1", "10n");
        noise.params = "nt=1u nalpha=0 namp=0 dc=0".to_owned();
        let netlist = netlist_for(vec![noise]);
        let card = netlist
            .lines()
            .find(|line| line.starts_with("V1 "))
            .unwrap_or_else(|| panic!("{netlist}"));
        assert!(card.ends_with("DC 0 TRNOISE(10n 1u 0 0)"), "{card}");
        rspice_core::netlist::parse_netlist(&netlist).expect("engine must accept the card");
    }

    /// The noise source emits a TRNOISE spec, not a bare DC value.
    #[test]
    fn noise_current_source_emits_trnoise() {
        let mut noise = Component::new(1, ComponentType::CurrentSourceNoise, Point::origin())
            .with_name_value("I1", "2n");
        noise.params = "nt=0.5u".to_owned();
        let netlist = netlist_for(vec![noise]);
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("I1 ") && l.ends_with("DC 0 TRNOISE(2n 0.5u 0 0)")),
            "{netlist}"
        );
    }

    /// The saturable inductor binds a generated Jiles-Atherton CORE card.
    #[test]
    fn saturable_inductor_emits_core_model_card() {
        let mut sat = Component::new(1, ComponentType::SaturableInductor, Point::origin())
            .with_name_value("L1", "2m");
        sat.params = "ms=1.6 n=50".to_owned();
        let netlist = netlist_for(vec![sat]);
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("L1 ") && l.ends_with(" 2m core_L1")),
            "{netlist}"
        );
        let card = netlist
            .lines()
            .find(|l| l.starts_with(".MODEL core_L1 CORE"))
            .expect("core card");
        assert!(card.contains("MS=1.6"), "{netlist}");
        assert!(card.contains("N=50"), "{netlist}");
    }

    /// XSPICE blocks emit shaped ports plus a .MODEL card whose type is the
    /// registered code-model name (the cards used to be missing entirely).
    #[test]
    fn xspice_gate_emits_vector_ports_and_model_card() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::XspiceAndGate, Point::origin())
                .with_name_value("A1", ""),
        ]);
        let line = netlist
            .lines()
            .find(|l| l.starts_with("A1 "))
            .expect("gate line");
        assert!(line.starts_with("A1 ["), "{netlist}");
        assert!(line.contains("] ["), "{netlist}");
        assert!(line.ends_with(" a1_model"), "{netlist}");
        // The gate declares no required parameters, so an untouched block
        // restates none of the code model's defaults back at it.
        assert!(netlist.contains(".MODEL a1_model d_and\n"), "{netlist}");
    }

    /// The limiter's required output limits always reach its card, carrying the
    /// code model's own defaults rather than a value invented by the emitter —
    /// the property sheet shows the same two numbers.
    #[test]
    fn xspice_limiter_card_carries_required_limits() {
        let mut limiter = Component::new(1, ComponentType::XspiceLimiter, Point::origin())
            .with_name_value("A1", "");
        limiter.params = "out_upper_limit=5".to_owned();
        let netlist = netlist_for(vec![limiter]);
        assert!(
            netlist.contains(
                ".MODEL a1_model limit (out_lower_limit=-1000000000000 out_upper_limit=5)"
            ),
            "{netlist}"
        );
    }

    /// Sequential blocks null their unconnected optional set/reset ports and
    /// the SR latch carries its mandatory enable terminal.
    #[test]
    fn xspice_sequential_blocks_null_optional_ports() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::XspiceDFlipFlop, Point::origin())
                .with_name_value("A1", ""),
            Component::new(2, ComponentType::XspiceSrLatch, Point::new(200, 0))
                .with_name_value("A2", ""),
        ]);
        let dff = netlist
            .lines()
            .find(|l| l.starts_with("A1 "))
            .expect("dff line");
        assert!(dff.contains(" null null "), "{netlist}");
        assert!(netlist.contains(".MODEL a1_model d_dff"), "{netlist}");
        let latch = netlist
            .lines()
            .find(|l| l.starts_with("A2 "))
            .expect("latch line");
        // s r en null null q qbar model = 8 tokens + name
        assert_eq!(latch.split_whitespace().count(), 9, "{netlist}");
        assert!(netlist.contains(".MODEL a2_model d_srlatch"), "{netlist}");
    }

    /// MESFETs emit the Z element with a generated NMF/PMF card.
    #[test]
    fn mesfet_emits_z_element_and_model_card() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::Nmesfet, Point::origin()).with_name_value("Z1", ""),
            Component::new(2, ComponentType::Pmesfet, Point::new(200, 0)).with_name_value("Z2", ""),
        ]);
        let line = netlist
            .lines()
            .find(|l| l.starts_with("Z1 "))
            .expect("mesfet line");
        assert_eq!(line.split_whitespace().count(), 5, "{netlist}");
        assert!(line.ends_with(" RSPICE_NMESFET"), "{netlist}");
        assert!(
            netlist.contains(foundation_card("RSPICE_NMESFET")),
            "{netlist}"
        );
        assert!(
            netlist.contains(foundation_card("RSPICE_PMESFET")),
            "{netlist}"
        );
    }

    /// The current-controlled switch's sense-coil pins become a
    /// synthesized 0 V sense source, referenced by name on the W line.
    #[test]
    fn iswitch_synthesizes_its_sense_source() {
        let mut switch =
            Component::new(1, ComponentType::ISwitch, Point::origin()).with_name_value("W1", "");
        switch.params = "it=2m".to_owned();
        let netlist = netlist_for(vec![switch]);
        let line = netlist
            .lines()
            .find(|l| l.starts_with("W1 "))
            .expect("switch line");
        assert!(line.contains(" VSENSE_W1 isw_W1"), "{netlist}");
        // name + 2 nodes + control source + model: the core rejects tails.
        assert_eq!(line.split_whitespace().count(), 5, "{netlist}");
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("VSENSE_W1 ") && l.ends_with(" 0")),
            "{netlist}"
        );
        assert!(
            netlist.contains(".MODEL isw_W1 CSW (IT=2m IH=0 RON=1 ROFF=1meg)"),
            "{netlist}"
        );
    }

    /// The lossy line emits an O element with an LTRA card (G=0) by
    /// default and a TXL card when selected.
    #[test]
    fn lossy_line_emits_ltra_or_txl_card() {
        let mut ltra = Component::new(1, ComponentType::LossyTransmissionLine, Point::origin())
            .with_name_value("O1", "");
        ltra.params = "r=12.45 l=8.972n c=0.468p len=16".to_owned();
        let mut txl = Component::new(2, ComponentType::LossyTransmissionLine, Point::new(200, 0))
            .with_name_value("O2", "");
        txl.params = "kind=txl g=1u".to_owned();
        let netlist = netlist_for(vec![ltra, txl]);
        let line = netlist
            .lines()
            .find(|l| l.starts_with("O1 "))
            .expect("ltra line");
        assert_eq!(line.split_whitespace().count(), 6, "{netlist}");
        assert!(line.ends_with(" ltra_O1"), "{netlist}");
        assert!(
            netlist.contains(".MODEL ltra_O1 LTRA (R=12.45 L=8.972n C=0.468p G=0 LEN=16)"),
            "{netlist}"
        );
        assert!(
            netlist.contains(".MODEL txl_O2 TXL (R=1 L=250n G=1u C=100p LENGTH=1)"),
            "{netlist}"
        );
    }

    /// The coupled line emits six nodes plus a CPL card with the three
    /// upper-triangle matrices and the length.
    #[test]
    fn coupled_line_emits_cpl_card() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::CoupledTransmissionLine, Point::origin())
                .with_name_value("P1", ""),
        ]);
        let line = netlist
            .lines()
            .find(|l| l.starts_with("P1 "))
            .expect("cpl line");
        // name + 6 nodes + model
        assert_eq!(line.split_whitespace().count(), 8, "{netlist}");
        assert!(line.ends_with(" cpl_P1"), "{netlist}");
        assert!(
            netlist.contains(
                ".MODEL cpl_P1 CPL\n+ R = (0.1 0 0.1)\n+ L = (380n 60n 380n)\n+ C = (120p -12p 120p)\n+ G = (0 0 0)\n+ LENGTH = 0.1"
            ),
            "{netlist}"
        );
    }

    /// The memristor emits the Xyce YMEMRISTOR element with a TEAM card.
    #[test]
    fn memristor_emits_ymemristor_and_team_card() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::Memristor, Point::origin()).with_name_value("MR1", ""),
        ]);
        let line = netlist
            .lines()
            .find(|l| l.starts_with("YMEMRISTOR MR1 "))
            .expect("memristor line");
        assert!(line.ends_with(" mem_MR1"), "{netlist}");
        // A device left at its defaults restates none of them: the sheet's
        // defaults are the engine's, so writing them out could only ever drift
        // away from the values the engine would have used anyway.
        assert!(
            netlist.contains(".MODEL mem_MR1 MEMRISTOR (LEVEL=2)"),
            "{netlist}"
        );
    }

    /// Every TEAM field the sheet offers reaches the card, and IVRELATION —
    /// the memristor's only instance parameter — reaches the instance line.
    #[test]
    fn memristor_carries_its_configured_team_parameters() {
        let mut memristor =
            Component::new(1, ComponentType::Memristor, Point::origin()).with_name_value("MR1", "");
        memristor.params =
            "ron=100 roff=10k xon=1n xoff=5n wt=2 alphaon=4 wc=2p ivrelation=1".to_owned();
        let netlist = netlist_for(vec![memristor]);

        assert!(
            netlist.contains(
                ".MODEL mem_MR1 MEMRISTOR (LEVEL=2 RON=100 ROFF=10k XON=1n XOFF=5n \
                 ALPHAON=4 WT=2 WC=2p)"
            ),
            "{netlist}"
        );
        assert!(
            netlist
                .lines()
                .any(|line| line.starts_with("YMEMRISTOR MR1 ") && line.ends_with(" IVRELATION=1")),
            "{netlist}"
        );
    }

    /// A B line accepts M, TC1, and TC2 and rejects the deck on anything else,
    /// so the emitter forwards exactly those three and nothing more.
    #[test]
    fn behavioral_source_carries_its_multiplier_and_temperature_coefficients() {
        let mut plain = Component::new(1, ComponentType::BehavioralSource, Point::origin())
            .with_name_value("B1", "I=V(in)/100");
        plain.params = String::new();
        let netlist = netlist_for(vec![plain]);
        assert!(
            netlist
                .lines()
                .any(|line| line.starts_with("B1 ") && line.ends_with("I=V(in)/100")),
            "{netlist}"
        );

        let mut configured = Component::new(1, ComponentType::BehavioralSource, Point::origin())
            .with_name_value("B1", "I=V(in)/100");
        configured.params = "m=4 tc1=1m tc2=2u".to_owned();
        let netlist = netlist_for(vec![configured]);
        assert!(
            netlist
                .lines()
                .any(|line| line.ends_with("I=V(in)/100 M=4 TC1=1m TC2=2u")),
            "{netlist}"
        );
    }

    /// TD and F/NL are alternative specifications of the same lossless line
    /// and the engine takes TD as authoritative when both are present, so a
    /// card must never carry both — the reference frequency would be an input
    /// the editor accepts and the solver ignores.
    #[test]
    fn lossless_line_accepts_either_delay_or_electrical_length() {
        let mut delayed = Component::new(1, ComponentType::TransmissionLine, Point::origin())
            .with_name_value("T1", "");
        delayed.params = "z0=75 td=2n".to_owned();
        let netlist = netlist_for(vec![delayed]);
        assert!(
            netlist
                .lines()
                .any(|line| line.starts_with("T1 ") && line.ends_with("Z0=75 TD=2n")),
            "{netlist}"
        );

        let mut electrical = Component::new(1, ComponentType::TransmissionLine, Point::origin())
            .with_name_value("T1", "");
        electrical.params = "z0=50 td=2n f=1G nl=0.25".to_owned();
        let netlist = netlist_for(vec![electrical]);
        assert!(
            netlist
                .lines()
                .any(|line| line.starts_with("T1 ") && line.ends_with("Z0=50 F=1G NL=0.25")),
            "a set reference frequency replaces the delay rather than joining it: {netlist}"
        );
    }

    /// The RF port always carries Z0 (which also selects the port parse
    /// branch); DC/AC excitation is appended only when set.
    #[test]
    fn rf_port_emits_port_element() {
        let plain =
            Component::new(1, ComponentType::RfPort, Point::origin()).with_name_value("P1", "");
        let mut driven =
            Component::new(2, ComponentType::RfPort, Point::new(200, 0)).with_name_value("P2", "");
        driven.params = "port=2 z0=75 ac_mag=1".to_owned();
        let netlist = netlist_for(vec![plain, driven]);
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("P1 ") && l.ends_with("PORT=1 Z0=50")),
            "{netlist}"
        );
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("P2 ") && l.ends_with("PORT=2 Z0=75 AC 1")),
            "{netlist}"
        );
    }

    /// A driven port's phase rides behind its magnitude — and only behind it,
    /// since a bare phase has nothing to shift.
    #[test]
    fn rf_port_ac_phase_follows_its_magnitude() {
        let mut shifted =
            Component::new(1, ComponentType::RfPort, Point::origin()).with_name_value("P1", "");
        shifted.params = "port=1 z0=50 ac_mag=1 ac_phase=90".to_owned();
        let netlist = netlist_for(vec![shifted]);
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("P1 ") && l.ends_with("PORT=1 Z0=50 AC 1 90")),
            "{netlist}"
        );

        let mut phase_only =
            Component::new(1, ComponentType::RfPort, Point::origin()).with_name_value("P1", "");
        phase_only.params = "port=1 z0=50 ac_phase=90".to_owned();
        let netlist = netlist_for(vec![phase_only]);
        assert!(
            netlist
                .lines()
                .any(|l| l.starts_with("P1 ") && l.ends_with("PORT=1 Z0=50")),
            "an unmagnitudinal phase is not an excitation: {netlist}"
        );
    }

    fn pwl_file_source(params: &str) -> Component {
        let mut source = Component::new(1, ComponentType::VoltageSourcePwlFile, Point::origin())
            .with_name_value("V1", "");
        source.params = params.to_owned();
        source
    }

    /// The card must be spelled the way the netlist reader accepts it, quotes
    /// and all, and every modifier the sheet exposes must survive the trip.
    #[test]
    fn pwl_file_source_emits_the_readers_spelling() {
        let netlist = netlist_for(vec![pwl_file_source(
            "file=wave.csv td=1u r=0 tscale=2 vscale=3 toffset=1n voffset=0.5",
        )]);
        let card = netlist
            .lines()
            .find(|line| line.starts_with("V1 "))
            .unwrap_or_else(|| panic!("{netlist}"));
        assert!(
            card.ends_with(
                "PWL FILE=\"wave.csv\" TD=1u R=0 TSCALE=2 VSCALE=3 TOFFSET=1n VOFFSET=0.5"
            ),
            "{card}"
        );
        rspice_core::netlist::parse_netlist(&netlist).expect("engine must accept the card");
    }

    /// An untouched modifier has no business on the card: TSCALE and VSCALE are
    /// unset at one, the offsets and delay at zero, and R when it is blank.
    #[test]
    fn unset_pwl_file_modifiers_stay_off_the_card() {
        let netlist = netlist_for(vec![pwl_file_source(
            "file=wave.csv td=0 r= tscale=1 vscale=1 toffset=0 voffset=0",
        )]);
        let card = netlist
            .lines()
            .find(|line| line.starts_with("V1 "))
            .unwrap_or_else(|| panic!("{netlist}"));
        assert!(card.ends_with("PWL FILE=\"wave.csv\""), "{card}");
        rspice_core::netlist::parse_netlist(&netlist).expect("engine must accept the card");
    }

    /// A source with no file selected cannot run, and saying so beats emitting
    /// a card the engine will reject with a path the user never typed.
    #[test]
    fn a_pwl_file_source_without_a_file_blocks_the_run() {
        let mut state = SchematicState::default();
        state.components = vec![pwl_file_source("td=1u")];
        let result = generate_netlist(&state);
        assert!(
            result
                .errors
                .iter()
                .any(|error| error.contains("V1") && error.contains("no data file")),
            "{:?}",
            result.errors
        );
    }

    /// An absolute reference is checkable, so a missing file stops the run
    /// here rather than deep inside the engine's circuit build.
    #[test]
    fn a_missing_pwl_data_file_blocks_the_run() {
        let absent = std::env::temp_dir().join("rspice-no-such-waveform-9c1f.csv");
        let mut state = SchematicState::default();
        state.components = vec![pwl_file_source(&format!("file={}", absent.display()))];
        let result = generate_netlist(&state);
        assert!(
            result
                .errors
                .iter()
                .any(|error| error.contains("V1") && error.contains("cannot read data file")),
            "{:?}",
            result.errors
        );
    }

    /// Substrate and thermal BJTs emit 4/5 nodes; the thermal variant
    /// binds a native VBIC card so the dT terminal is solved.
    #[test]
    fn substrate_and_thermal_bjts_emit_extra_nodes() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::NpnBjt4, Point::origin()).with_name_value("Q1", ""),
            Component::new(2, ComponentType::PnpBjt5, Point::new(200, 0)).with_name_value("Q2", ""),
        ]);
        let q1 = netlist
            .lines()
            .find(|l| l.starts_with("Q1 "))
            .expect("4T bjt line");
        assert_eq!(q1.split_whitespace().count(), 6, "{netlist}");
        assert!(q1.ends_with(" RSPICE_NPN"), "{netlist}");
        assert!(netlist.contains(foundation_card("RSPICE_NPN")), "{netlist}");
        let q2 = netlist
            .lines()
            .find(|l| l.starts_with("Q2 "))
            .expect("5T bjt line");
        assert_eq!(q2.split_whitespace().count(), 7, "{netlist}");
        // The thermal placement gets the VBIC card, not the Gummel-Poon one:
        // only the LEVEL=4 family solves the dT terminal it carries.
        assert!(q2.ends_with(" RSPICE_PNP_THERMAL"), "{netlist}");
        assert!(
            netlist.contains(foundation_card("RSPICE_PNP_THERMAL")),
            "{netlist}"
        );
    }

    /// SOI MOSFETs emit five nodes with a partially-depleted BSIMSOI card,
    /// and W/L always reach the instance line (the BSIMSOI internal
    /// geometry defaults do not converge on a bare card).
    #[test]
    fn soi_mosfet_emits_five_nodes_and_bsimsoi_card() {
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::NmosSoi, Point::origin()).with_name_value("M1", ""),
        ]);
        let line = netlist
            .lines()
            .find(|l| l.starts_with("M1 "))
            .expect("soi line");
        // name + 5 nodes + model + w= + l=
        assert_eq!(line.split_whitespace().count(), 9, "{netlist}");
        assert!(line.contains(" RSPICE_NMOS_SOI "), "{netlist}");
        assert!(line.contains("w=1u"), "{netlist}");
        assert!(line.contains("l=180n"), "{netlist}");
        assert!(
            netlist.contains(foundation_card("RSPICE_NMOS_SOI")),
            "{netlist}"
        );
    }

    /// A placed K coupling references its windings by name.
    #[test]
    fn placed_k_coupling_emits_coupling_line() {
        let mut coupling = Component::new(3, ComponentType::CoupledInductor, Point::new(400, 0))
            .with_name_value("K1", "0.9");
        coupling.params = "inductors=\"L1 L2\"".to_owned();
        let netlist = netlist_for(vec![
            Component::new(1, ComponentType::Inductor, Point::origin()).with_name_value("L1", "1u"),
            Component::new(2, ComponentType::Inductor, Point::new(200, 0))
                .with_name_value("L2", "1u"),
            coupling,
        ]);
        assert!(
            netlist.lines().any(|l| l.trim() == "K1 L1 L2 0.9"),
            "{netlist}"
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

    #[test]
    fn technology_global_catalog_changes_the_executable_node_authority() {
        let libraries = LibraryManager::new();
        let mut schematic = SchematicState::default();
        schematic
            .wires
            .push(Wire::segment(1, Point::new(0, 0), Point::new(40, 0)));
        schematic
            .net_labels
            .push(NetLabel::new(2, Point::new(0, 0), "VCC"));
        let mut buffers = HashMap::new();
        buffers.insert("user/top/schematic".to_owned(), schematic.clone());
        let contract = ConnectivityContract {
            policy: ConnectivityPolicy {
                global_promotion: GlobalNetPromotionPolicy::TechnologyDefinedOnly,
                ..ConnectivityPolicy::default()
            },
            technology_global_nets: Some(TechnologyGlobalNetCatalog {
                authority: "demo-pdk@1.0".to_owned(),
                nets: vec![ConnectivityAliasGroup {
                    canonical_name: "VDD".to_owned(),
                    aliases: vec!["VCC".to_owned()],
                }],
            }),
            ..ConnectivityContract::default()
        };
        contract.validate().unwrap();
        let hierarchy =
            HierarchySource::from_workspace(&libraries, &buffers).with_connectivity(&contract);

        let result = generate_netlist_hierarchical(&schematic, &[], &hierarchy);

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(result.netlist.contains(".GLOBAL VDD"), "{}", result.netlist);
        assert!(result.nets.contains_key("VDD"), "{:?}", result.nets);
        assert!(!result.nets.contains_key("VCC"), "{:?}", result.nets);
    }

    #[test]
    fn dialect_alias_catalog_merges_explicit_global_declarations() {
        let libraries = LibraryManager::new();
        let mut schematic = SchematicState::default();
        schematic
            .wires
            .push(Wire::segment(1, Point::new(0, 0), Point::new(40, 0)));
        schematic
            .wires
            .push(Wire::segment(2, Point::new(0, 40), Point::new(40, 40)));
        schematic
            .net_labels
            .push(NetLabel::new(3, Point::new(0, 0), "VCC!"));
        schematic
            .net_labels
            .push(NetLabel::new(4, Point::new(0, 40), "VDD!"));
        let mut buffers = HashMap::new();
        buffers.insert("user/top/schematic".to_owned(), schematic.clone());
        let contract = ConnectivityContract {
            policy: ConnectivityPolicy {
                alias_comparison: GlobalAliasComparisonPolicy::DialectCompatibility,
                ..ConnectivityPolicy::default()
            },
            dialect_aliases: Some(DialectAliasCatalog {
                authority: "commercial-spice-2026".to_owned(),
                groups: vec![ConnectivityAliasGroup {
                    canonical_name: "VDD".to_owned(),
                    aliases: vec!["VCC".to_owned()],
                }],
            }),
            ..ConnectivityContract::default()
        };
        contract.validate().unwrap();
        let hierarchy =
            HierarchySource::from_workspace(&libraries, &buffers).with_connectivity(&contract);

        let result = generate_netlist_hierarchical(&schematic, &[], &hierarchy);

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(
            result.netlist.contains(".GLOBAL VDD!"),
            "{}",
            result.netlist
        );
        assert_eq!(result.nets.len(), 1, "{:?}", result.nets);
        assert!(result.nets.contains_key("VDD!"), "{:?}", result.nets);
    }
}
