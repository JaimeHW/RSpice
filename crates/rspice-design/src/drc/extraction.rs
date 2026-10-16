//! Map one extracted design into rule inputs and assemble its complete report.
//!
//! Binds every placed component to the design's one connectivity extraction and
//! reads the canonical property schema for each. The rules receive node names
//! the deck will actually write, so a finding quotes what the engine sees.

use super::checker::{
    DrcChecker, DrcConfig, append_case_collision_violations, append_off_sheet_connector_violations,
    append_vector_width_violations, component_identity, diagnostic_location,
};
use super::input::{BrokenReference, ComponentInfo, ParameterRangeIssue, PinInfo};
use super::source_loops::{self, GROUND_KEY};
use super::types::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
use crate::connectivity::{ConnectivityDiagnosticKind, ExtractedConnectivity};
use crate::hierarchy::{ConfigurationExecutionBinding, HierarchySource};
use crate::properties::PropertyCatalog;
use crate::properties::PropertyDefinition;
use crate::properties::PropertyValue;
use crate::schematic::component::{Component, LibraryCellInstance};
use crate::schematic::component_references::{InstanceReference, resolve_instance_reference};
use crate::schematic::component_type::ComponentType;
use crate::schematic::document::SchematicDocument;
use crate::schematic::port::{PortDirection, PortSpec};
use rspice_app_types::hierarchy_path::InstancePath;
use rspice_design_model::cell_view::CellViewRef;
use std::collections::{BTreeMap, HashMap, HashSet};

mod walk;
use walk::{InstanceGraph, MAX_REPORTED_OCCURRENCE_PATHS, Occurrences, ReachedView, SUBJECT};

/// How much of the instantiated hierarchy one check states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckReach {
    /// The subject and every cellview its instances reach. A finding below
    /// the subject is stated at the occurrences that reach it.
    Hierarchy,
    /// The subject's own findings. The hierarchy below is still walked for
    /// the two questions no single cellview can answer — whether the design
    /// reaches ground, and whether a cycle runs through the subject — but
    /// nothing found inside another cellview is restated. A caller that checks
    /// every cellview of a project asks this of each, so each is checked once.
    Cellview,
}

/// Resolve the design once, and bind every placed component to it.
///
/// The caller hands over a drawing rather than an occurrence, so a cell
/// instance's master is judged from its placed binding — except where a
/// configuration may bind it elsewhere, which this has no occurrence path to
/// ask about and so does not judge.
pub fn extract_checked_design(
    schematic: &impl AsRef<SchematicDocument>,
    hierarchy: &HierarchySource<'_>,
) -> (Vec<ComponentInfo>, ExtractedConnectivity) {
    let globals = global_nodes(hierarchy);
    extract_view(
        schematic.as_ref(),
        hierarchy,
        &ViewContext {
            plan_path: None,
            globals: &globals,
        },
    )
}

/// What one cellview's extraction knows about where it stands.
struct ViewContext<'g> {
    /// The cellview's occurrence path in the plan's own terms, when the walk
    /// follows the plan, which then decides what each instance binds.
    plan_path: Option<&'g InstancePath>,
    /// The deck's global node names, folded.
    globals: &'g HashSet<String>,
}

impl ViewContext<'_> {
    /// The plan's path of one instance placed in this cellview.
    fn occurrence(&self, component: &Component) -> Option<InstancePath> {
        self.plan_path?.child(&component.name).ok()
    }
}

/// Every global node the deck declares, folded, so a rule can tell a node that
/// continues into other cellviews from one that ends here.
fn global_nodes(hierarchy: &HierarchySource<'_>) -> HashSet<String> {
    hierarchy
        .global_net_names()
        .into_iter()
        .map(|name| name.to_ascii_lowercase())
        .collect()
}

fn extract_view(
    schematic: &SchematicDocument,
    hierarchy: &HierarchySource<'_>,
    context: &ViewContext<'_>,
) -> (Vec<ComponentInfo>, ExtractedConnectivity) {
    let connectivity = crate::connectivity::extract_with_hierarchy(schematic, Some(hierarchy));
    let components = extract_components(schematic, &connectivity, hierarchy, context);
    (components, connectivity)
}

/// Whether a binding carries the text the deck writes for it — a source, a
/// netlist template, or a compiled built-in — rather than naming a schematic
/// whose contents the deck emits.
fn writes_its_own_text(binding: &LibraryCellInstance) -> bool {
    binding.source_path.is_some()
        || binding.netlist_template.is_some()
        || binding.is_executable_builtin()
}

/// Whether a cell instance resolves to something the deck can write.
fn component_known(
    component: &Component,
    hierarchy: &HierarchySource<'_>,
    context: &ViewContext<'_>,
) -> Option<bool> {
    if component.kind != ComponentType::CellInstance {
        return Some(true);
    }
    let Some(binding) = component.library_cell.as_ref() else {
        return Some(false);
    };
    if context.plan_path.is_some() {
        // The plan decides what this occurrence binds, and it is asked: an
        // occurrence it could not materialize is one the deck cannot write.
        return context.occurrence(component).map(|path| {
            hierarchy
                .execution_binding(&path)
                .and_then(ConfigurationExecutionBinding::materialized_binding)
                .is_some()
        });
    }
    if writes_its_own_text(binding) {
        return Some(true);
    }
    if hierarchy.has_execution_plan() {
        // A configuration may bind this instance to another master, and there
        // is no occurrence path here to ask the plan with. Judging the placed
        // binding instead would judge a master the configuration may have
        // replaced.
        return None;
    }
    Some(hierarchy.schematic_master_for_binding(binding).is_some())
}

/// Check the subject cellview and every cellview its instances reach.
///
/// The hierarchy is walked, not merely consulted. Checking the subject alone
/// let a hierarchical design pass with a short, an open input or a missing
/// parameter one level down, which is the one failure a checker must not have:
/// a report that says the circuit is sound when it is not.
///
/// Each distinct cellview is extracted and checked once, however many times it
/// is placed, and its findings are then stated at the occurrence paths that
/// reached it, so `X1/M3` and `X2/M3` read as the two terminals they are.
///
/// The subject's own cellview is recognized when the caller hands over one of
/// the very buffers `hierarchy` indexes; a caller holding a copy should name
/// it with [`run_check_of_cellview`]. Where the hierarchy's execution plan is
/// rooted at the subject, the walk descends where the deck does.
pub fn run_check_with_hierarchy(
    schematic: &impl AsRef<SchematicDocument>,
    hierarchy: &HierarchySource<'_>,
    config: DrcConfig,
) -> DrcResult {
    let subject = schematic.as_ref();
    check_subject(
        subject,
        hierarchy.cellview_of(subject),
        hierarchy,
        config,
        CheckReach::Hierarchy,
    )
}

/// Check one named cellview, stating as much of the hierarchy below it as
/// `reach` asks for.
///
/// Naming the subject is what lets the walk close a cycle at it: an instance
/// that binds the subject's own cellview is reported as the recursion it is,
/// rather than reached as a second copy whose findings repeat the subject's.
pub fn run_check_of_cellview(
    schematic: &impl AsRef<SchematicDocument>,
    reference: &CellViewRef,
    hierarchy: &HierarchySource<'_>,
    config: DrcConfig,
    reach: CheckReach,
) -> DrcResult {
    check_subject(
        schematic.as_ref(),
        Some(reference.clone()),
        hierarchy,
        config,
        reach,
    )
}

fn check_subject(
    subject: &SchematicDocument,
    reference: Option<CellViewRef>,
    hierarchy: &HierarchySource<'_>,
    config: DrcConfig,
    reach: CheckReach,
) -> DrcResult {
    let start = web_time::Instant::now();
    let graph = InstanceGraph::walk(subject, reference, hierarchy);
    let globals = global_nodes(hierarchy);
    let occurrences = graph.occurrences();

    // The ground rule is the one question no single cellview can answer, so it
    // is held back until the whole walk has been extracted.
    let mut view_config = config.clone();
    view_config.check_missing_ground = false;

    // A cellview below the subject is read only where something here asks
    // about it: a hierarchy report states its findings, and the ground rule
    // needs its evidence.
    let read_below = reach == CheckReach::Hierarchy || config.check_missing_ground;
    let mut reports = Vec::with_capacity(graph.views.len());
    let mut drives = GlobalDrives::default();
    let mut touches = GlobalTouches::default();
    let mut design_reaches_ground = false;
    let mut subject_inputs = None;
    for (index, view) in graph.views.iter().enumerate() {
        if index != SUBJECT && !read_below {
            break;
        }
        if occurrences[index].paths.is_empty() {
            // Every occurrence of it is past the hierarchy's limits, which the
            // resolver reports at the parent: the deck writes none of it, and
            // no finding inside it has a place to be stated.
            if reach == CheckReach::Hierarchy {
                reports.push(DrcResult::new());
            }
            continue;
        }
        let context = ViewContext {
            plan_path: view.plan_path.as_ref(),
            globals: &globals,
        };
        let (components, connectivity) = extract_view(view.document, hierarchy, &context);
        design_reaches_ground |= reaches_ground(&components);
        if index == SUBJECT || reach == CheckReach::Hierarchy {
            let mut report =
                check_one_view(view.document, &view_config, &components, &connectivity);
            graph.append_recursive_findings(
                index,
                reach,
                &view_config.severity_overrides,
                &mut report,
            );
            reports.push(report);
        }
        if reach == CheckReach::Hierarchy && config.check_shorted_outputs {
            drives.collect(index, &components, &globals);
        }
        if reach == CheckReach::Hierarchy && config.check_floating_nodes {
            touches.collect(index, &components);
        }
        if index == SUBJECT {
            subject_inputs = Some((components, connectivity));
        }
    }

    let mut result = DrcResult::new();
    let mut next_id = 0;

    // The subject's own findings are stated as they always were: they are in
    // the drawing the caller named, and wrapping them in an empty path would
    // say they are somewhere else.
    for finding in reports[SUBJECT].violations() {
        result.add_violation(restated(finding, next_id, None, None));
        next_id += 1;
    }

    // Node 0 is global, so a ground symbol anywhere in the walked hierarchy
    // grounds the design above it: the rule is asked once, over the union of
    // what every reached cellview binds, and answered on the subject. Asking
    // it per cellview instead would fail nearly every correct subcircuit,
    // because a reusable cell takes its reference from the parent through a
    // port or a global and owns no node-0 symbol of its own — while a
    // top-level design that reaches no reference anywhere cannot solve at all.
    if config.check_missing_ground && !design_reaches_ground {
        let (components, connectivity) = subject_inputs
            .as_ref()
            .expect("the subject is always the first cellview checked");
        let ground_only = DrcConfig {
            check_floating_nodes: false,
            check_unconnected_pins: false,
            check_missing_ground: true,
            check_duplicate_names: false,
            check_component_parameters: false,
            check_unknown_components: false,
            check_shorted_outputs: false,
            ..config.clone()
        };
        let ground = DrcChecker::with_config(ground_only).check(components, connectivity);
        for finding in ground.violations() {
            result.add_violation(restated(finding, next_id, None, None));
            next_id += 1;
        }
    }

    if reach == CheckReach::Hierarchy {
        for index in (SUBJECT + 1)..graph.views.len() {
            if occurrences[index].paths.is_empty() {
                continue;
            }
            let reference = graph.views[index]
                .reference
                .as_ref()
                .expect("a reached cellview is named by the binding that reached it");
            let (paths, note) = statement(&occurrences[index]);
            for path in paths {
                for finding in reports[index].violations() {
                    result.add_violation(restated(
                        finding,
                        next_id,
                        Some((path, reference)),
                        note.as_deref(),
                    ));
                    next_id += 1;
                }
            }
        }
        for mut finding in drives.findings(&occurrences, &graph.views) {
            if let Some(severity) = config.severity_overrides.get(&finding.violation_type) {
                finding.severity = *severity;
            }
            finding.id = next_id;
            result.add_violation(finding);
            next_id += 1;
        }
        // A global touched once has its one terminal in exactly one
        // occurrence, and the finding is stated there.
        for (view, mut finding) in touches.floating(&occurrences, config.min_connections) {
            if let Some(severity) = config.severity_overrides.get(&finding.violation_type) {
                finding.severity = *severity;
            }
            let occurrence = graph.views[view].reference.as_ref().and_then(|reference| {
                (view != SUBJECT).then(|| (&occurrences[view].paths[0], reference))
            });
            result.add_violation(restated(&finding, next_id, occurrence, None));
            next_id += 1;
        }
    }

    result.completed = true;
    result.duration_ms = start.elapsed().as_millis() as u64;
    result
}

/// The occurrence paths a cellview's findings are stated at, and what the
/// reader is told about the occurrences that are not listed.
///
/// Past [`MAX_REPORTED_OCCURRENCE_PATHS`] a finding is stated once, with the
/// exact count. Below it, an occurrence whose path exceeds the hierarchy's
/// limits has no path to be stated at; it is counted rather than dropped, so a
/// report never lists fewer places than the design has without saying so.
fn statement(occurrence: &Occurrences) -> (&[InstancePath], Option<String>) {
    let count = occurrence.count_label();
    if occurrence.saturated || occurrence.count > MAX_REPORTED_OCCURRENCE_PATHS as u64 {
        let first = &occurrence.paths[..1];
        let note = format!("{count} occurrences; reported once, at {}", first[0]);
        return (first, Some(note));
    }
    let listed = occurrence.paths.len() as u64;
    let note = (listed < occurrence.count).then(|| {
        let unlisted = occurrence.count - listed;
        format!(
            "{count} occurrences; {unlisted} {} no instance path within the hierarchy limits",
            if unlisted == 1 { "has" } else { "have" }
        )
    });
    (&occurrence.paths, note)
}

/// Restate one cellview's finding in the merged report.
///
/// `occurrence` is `None` for the subject and names the path and cellview
/// otherwise. `note` is set only where the finding stands for occurrences the
/// report does not list.
fn restated(
    finding: &DrcViolation,
    id: usize,
    occurrence: Option<(&InstancePath, &CellViewRef)>,
    note: Option<&str>,
) -> DrcViolation {
    let DrcViolation {
        violation_type,
        severity,
        mut message,
        location,
        related_items,
        ..
    } = finding.clone();
    let Some((path, reference)) = occurrence else {
        return DrcViolation {
            id,
            violation_type,
            severity,
            message,
            location,
            related_items,
        };
    };
    if let Some(note) = note {
        message = format!("{message} ({note})");
    }
    DrcViolation {
        id,
        violation_type,
        severity,
        message,
        location: DrcLocation::InsideInstance {
            path: path.clone(),
            view: reference.clone(),
            inner: Box::new(location),
        },
        related_items,
    }
}

/// Whether this cellview binds the design's reference node to anything other
/// than a ground symbol's own terminal.
///
/// This is exactly the predicate [`DrcChecker::check`]'s ground rule applies to
/// its own net map: a ground-named node is evidence only once some terminal
/// that is not a drawn ground symbol's stands on it. Node 0 is global, so the
/// evidence is about the whole design wherever in the hierarchy it is found.
fn reaches_ground(components: &[ComponentInfo]) -> bool {
    components
        .iter()
        .filter(|component| !component.is_ground_symbol)
        .any(|component| {
            component
                .pins
                .iter()
                .any(|pin| crate::schematic::ground_names::is_ground_reference(&pin.net_name))
        })
}

fn extract_components(
    schematic: &SchematicDocument,
    connectivity: &ExtractedConnectivity,
    hierarchy: &HierarchySource<'_>,
    context: &ViewContext<'_>,
) -> Vec<ComponentInfo> {
    let mut components = Vec::with_capacity(schematic.components.len());
    let property_registry = PropertyCatalog::new();

    // Terminals arrive in placement order and, within a component, in the
    // order the symbol declares its pins. Grouping preserves both.
    let mut bound: HashMap<u64, Vec<&crate::connectivity::ExtractedTerminal>> = HashMap::new();
    for terminal in &connectivity.terminals {
        bound
            .entry(terminal.component_id)
            .or_default()
            .push(terminal);
    }

    for comp in &schematic.components {
        let terminals = bound.get(&comp.id).map(Vec::as_slice).unwrap_or_default();
        let mut pins = Vec::with_capacity(terminals.len());
        let declared_directions =
            declared_port_directions(live_interface(comp, hierarchy, context));

        for terminal in terminals {
            let source_drive_terminal = is_voltage_source(comp.kind) && terminal.pin == "+";
            let direction = if source_drive_terminal {
                // An independent voltage source binds no cell interface, and
                // its `+` terminal is the whole declaration it has: that
                // terminal is what drives the node. This case is checked first
                // so a source keeps its drive terminal whatever else a binding
                // might say about a pin spelled `+`.
                Some(PortDirection::Out)
            } else {
                // A symbol pin matches its interface port whatever the case.
                declared_directions
                    .get(&terminal.pin.to_ascii_lowercase())
                    .copied()
            };

            pins.push(PinInfo {
                name: terminal.pin.clone(),
                net_name: terminal.net_name.clone(),
                direction,
                point: terminal.point,
                attached: terminal.attached,
                optional: false,
                on_global: context
                    .globals
                    .contains(&terminal.net_name.to_ascii_lowercase()),
            });
        }
        mark_optional_inputs(comp, &mut pins);

        let is_voltage_source = is_voltage_source(comp.kind);
        let is_current_source = is_current_source(comp.kind);
        let reference_required = !comp.kind.spice_prefix().is_empty();
        // The reference is validated as written, surrounding blanks and all:
        // the netlister writes it as written.
        let reference_error = (reference_required && !comp.name.trim().is_empty())
            .then(|| comp.validate_reference_designator(&comp.name).err())
            .flatten();

        components.push(ComponentInfo {
            id: comp.id,
            name: comp.name.clone(),
            component_type: comp.kind.spice_prefix().to_string(),
            pins,
            is_voltage_source,
            is_current_source,
            is_ground_symbol: comp.kind == ComponentType::Ground,
            reference_required,
            reference_error,
            deck_name: reference_required.then(|| deck_name(comp)).flatten(),
            is_interface_port: comp.port_spec().is_some(),
            component_known: component_known(comp, hierarchy, context),
            missing_parameters: missing_parameters(comp, &property_registry),
            out_of_range_parameters: out_of_range_parameters(comp, &property_registry),
            broken_references: broken_references(comp, &schematic.components),
        });
    }

    components
}

/// The references one instance makes to others by name that the deck cannot
/// write as a single element.
///
/// A current-controlled source names its controlling element. The netlister
/// resolves that reference with [`resolve_instance_reference`] and writes the
/// emitted name it finds, so this asks the same question of the same function.
fn broken_references(component: &Component, cellview: &[Component]) -> Vec<BrokenReference> {
    if !matches!(component.kind, ComponentType::Ccvs | ComponentType::Cccs) {
        return Vec::new();
    }
    let params = crate::parameters::parse_params_string(&component.params);
    let Some(reference) = params
        .get("vref")
        .map(|reference| reference.trim())
        .filter(|reference| !reference.is_empty())
    else {
        return Vec::new();
    };
    let ambiguous = match resolve_instance_reference(cellview, reference) {
        InstanceReference::Names(_) => return Vec::new(),
        InstanceReference::Ambiguous => true,
        InstanceReference::Unresolved => false,
    };
    vec![BrokenReference {
        parameter: "vref".to_owned(),
        reference: reference.to_owned(),
        ambiguous,
    }]
}

/// The name the deck writes one instance under, or `None` when it cannot
/// write the instance at all.
///
/// This is the netlister's own naming, not a second account of it: an unnamed
/// primitive is written `<prefix><id>`, and one whose name starts with the
/// wrong letter is written with the prefix in front. A cell instance is also
/// addressed by its occurrence path, so a reference that is not a path segment
/// — an empty one among them — leaves the deck nothing to write it under. And
/// the written name has to be one the engine's element-head parser reads as a
/// name: a blank, a delimiter such as `(` `,` `=` `{`, a quote or a character
/// outside ASCII would split or break the card it heads. That is judged by the
/// parser's own rule, so the two cannot disagree about which names run.
fn deck_name(component: &Component) -> Option<String> {
    if component.kind == ComponentType::CellInstance
        && InstancePath::root().child(&component.name).is_err()
    {
        return None;
    }
    let name = component.emitted_instance_name();
    rspice_core::netlist::lexer::is_xyce_device_name(&name).then_some(name)
}

/// The interface the deck binds one instance to, read live.
///
/// The placed binding keeps a copy of its master's interface from when it was
/// placed, and the master may have changed since. Where the plan can be asked,
/// the binding it materialized at this occurrence carries the interface the
/// deck uses; otherwise a schematic master's own ports are read. Only a binding
/// that writes its own text — a source, a template, a compiled built-in, each
/// validated against its definition when it is netlisted — answers with what
/// it carries.
fn live_interface(
    component: &Component,
    hierarchy: &HierarchySource<'_>,
    context: &ViewContext<'_>,
) -> Option<Vec<PortSpec>> {
    let binding = component.library_cell.as_ref()?;
    if let Some(path) = context.occurrence(component)
        && let Some(materialized) = hierarchy
            .execution_binding(&path)
            .and_then(ConfigurationExecutionBinding::materialized_binding)
    {
        return materialized.interface();
    }
    if !writes_its_own_text(binding)
        && let Some(master) = hierarchy.schematic_master_for_binding(binding)
    {
        return Some(master.interface_ports());
    }
    binding.interface()
}

/// Mark the inputs a compiled XSPICE built-in may leave open.
///
/// The authority is the frozen port contract the netlister itself emits from:
/// a port that allows `null` and that nothing meets is written as `null`, so
/// such a port is not an undriven input. A port with any one of its terminals
/// attached is written as nodes, and its open terminals stay open pins.
fn mark_optional_inputs(component: &Component, pins: &mut [PinInfo]) {
    let Some(binding) = component.library_cell.as_ref() else {
        return;
    };
    let Some(contract) = binding.builtin_xspice.as_ref() else {
        return;
    };
    for port in contract.ports.iter().filter(|port| port.null_allowed) {
        let names = port
            .terminals
            .iter()
            .filter_map(|terminal| binding.terminal_order.get(*terminal))
            .map(|name| name.to_ascii_lowercase())
            .collect::<HashSet<_>>();
        let open = pins
            .iter()
            .filter(|pin| names.contains(&pin.name.to_ascii_lowercase()))
            .all(|pin| !pin.attached);
        if !open {
            continue;
        }
        for pin in pins
            .iter_mut()
            .filter(|pin| names.contains(&pin.name.to_ascii_lowercase()))
        {
            pin.optional = true;
        }
    }
}

/// Every direction an interface declares, keyed by folded pin name.
///
/// A primitive binds no interface and therefore declares nothing: the map is
/// empty, every one of its terminals reads as undeclared, and an open resistor
/// leg is never judged as an input nothing drives.
///
/// A drawn interface port's own `dir=` is deliberately not read here. That
/// declaration is written from the parent's side of the symbol, so inside the
/// master an `out` port is where the cellview's own circuitry arrives — reading
/// it here would make a port a driver within the very cellview that drives it.
fn declared_port_directions(interface: Option<Vec<PortSpec>>) -> HashMap<String, PortDirection> {
    let mut declared = HashMap::new();
    for port in interface.into_iter().flatten() {
        // One name declared twice keeps the `Out` claim rather than whichever
        // declaration was read last, which is the reading the driver rules
        // have always had.
        match declared.entry(port.name.to_ascii_lowercase()) {
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(port.direction);
            }
            std::collections::hash_map::Entry::Occupied(mut slot)
                if port.direction == PortDirection::Out =>
            {
                slot.insert(port.direction);
            }
            std::collections::hash_map::Entry::Occupied(_) => {}
        }
    }
    declared
}

fn effective_component_properties<'a>(
    component: &Component,
    registry: &'a PropertyCatalog,
) -> Option<(
    &'a crate::properties::PropertySheet,
    std::collections::HashMap<String, PropertyValue>,
)> {
    if component.kind == ComponentType::CellInstance {
        // Cell-instance CDF is transaction-scoped and cannot be reconstructed
        // from the built-in registry. Reporting against a previous or generic
        // master's schema would be a false positive.
        return None;
    }
    let sheet = registry.get(component.kind)?;
    Some((
        sheet,
        crate::properties::component::collect_properties_with_sheet(component, Some(sheet)),
    ))
}

fn missing_parameters(component: &Component, registry: &PropertyCatalog) -> Vec<String> {
    let Some((sheet, values)) = effective_component_properties(component, registry) else {
        return Vec::new();
    };
    let mut missing = sheet
        .iter()
        .filter(|definition| definition.name != "name" && definition.required)
        .filter_map(|definition| {
            let value = values
                .get(&definition.name)
                .unwrap_or(&definition.default_value);
            property_value_is_missing(definition, value).then(|| definition.display_name.clone())
        })
        .collect::<Vec<_>>();
    missing.sort();
    missing.dedup();
    missing
}

fn property_value_is_missing(definition: &PropertyDefinition, value: &PropertyValue) -> bool {
    match value {
        PropertyValue::String(value) => value.trim().is_empty(),
        PropertyValue::Expression(value) => {
            definition.required
                && matches!(
                    definition.prop_type,
                    crate::properties::PropertyType::Number
                        | crate::properties::PropertyType::Expression
                )
                && value.trim().is_empty()
        }
        PropertyValue::Number { value, .. } => value.is_nan(),
        PropertyValue::Enum { .. } | PropertyValue::Boolean(_) => false,
    }
}

fn out_of_range_parameters(
    component: &Component,
    registry: &PropertyCatalog,
) -> Vec<ParameterRangeIssue> {
    let Some((sheet, values)) = effective_component_properties(component, registry) else {
        return Vec::new();
    };
    let mut issues = sheet
        .iter()
        .filter(|definition| definition.min_value.is_some() || definition.max_value.is_some())
        .filter_map(|definition| {
            let value = values
                .get(&definition.name)
                .unwrap_or(&definition.default_value);
            let numeric = exact_numeric_constant(definition, value)?;
            let below = definition
                .min_value
                .is_some_and(|minimum| numeric < minimum);
            let above = definition
                .max_value
                .is_some_and(|maximum| numeric > maximum);
            (below || above).then(|| ParameterRangeIssue {
                name: definition.name.clone(),
                display_name: definition.display_name.clone(),
                value: numeric,
                min: definition.min_value,
                max: definition.max_value,
            })
        })
        .collect::<Vec<_>>();
    issues.sort_by(|left, right| left.name.cmp(&right.name));
    issues
}

fn exact_numeric_constant(definition: &PropertyDefinition, value: &PropertyValue) -> Option<f64> {
    if let PropertyValue::Number { value, .. } = value {
        return value.is_finite().then_some(*value);
    }
    let PropertyValue::Expression(source) = value else {
        return None;
    };

    // Reuse the production property editor parser (quantity policy,
    // expression grammar, and unit normalization) while temporarily removing
    // the range itself. This reveals a definite numeric constant without
    // relying on validator error strings; symbolic expressions remain unknown.
    let mut unconstrained = definition.clone();
    unconstrained.min_value = None;
    unconstrained.max_value = None;
    crate::properties::value::parse_expression_source(
        &unconstrained,
        source,
        rspice_app_types::quantity::QuantityPresentationPolicy::default(),
        rspice_app_types::quantity::UiNumberLocale::default(),
    )
    .ok()
    .and_then(|value| value.as_number())
    .filter(|value| value.is_finite())
}

/// Render the bus-family diagnostics the extraction raised.
///
/// These are findings about what the drawing declares rather than about what a
/// rule computes, so the checker does not recompute them: the netlister already
/// refused the deck over the same text, and one message keeps the two reports
/// from describing the same defect differently.
fn append_connectivity_diagnostics(
    connectivity: &ExtractedConnectivity,
    result: &mut DrcResult,
    severity_overrides: &std::collections::HashMap<DrcViolationType, DrcSeverity>,
) {
    let mut next_id = result.total_count();
    for diagnostic in &connectivity.diagnostics {
        let violation_type = match diagnostic.kind {
            ConnectivityDiagnosticKind::MalformedBus => DrcViolationType::MalformedBus,
            ConnectivityDiagnosticKind::UnnamedBus => DrcViolationType::UnnamedBus,
            ConnectivityDiagnosticKind::BusRangeConflict => DrcViolationType::BusRangeConflict,
            ConnectivityDiagnosticKind::DanglingBusTap => DrcViolationType::DanglingBusTap,
            ConnectivityDiagnosticKind::MixedBusTap => DrcViolationType::MixedBusTap,
            // The checker raises the orphan label itself, under the gate that
            // governs attachment findings. A width mismatch is raised by
            // `append_vector_width_violations`, which is where the project's
            // width policy decides its severity. A name the deck cannot carry
            // blocks emission and has no electrical rule of its own.
            ConnectivityDiagnosticKind::OrphanNetLabel
            | ConnectivityDiagnosticKind::VectorWidthMismatch
            | ConnectivityDiagnosticKind::NetNaming => continue,
        };
        let mut violation = DrcViolation::new(
            next_id,
            violation_type,
            diagnostic.message.clone(),
            diagnostic_location(&diagnostic.anchor),
        );
        if let Some(severity) = severity_overrides.get(&violation_type) {
            violation.severity = *severity;
        }
        result.add_violation(violation);
        next_id += 1;
    }
}

/// Whether this type is an independent voltage source — one the rules count
/// as driving the net it stands on.
///
/// The two predicates are a pair, and they are written out here rather than
/// asked of `ComponentType::is_source` because that one also admits the
/// dependent sources and the behavioural one, which drive a net through an
/// expression and are not what a source-to-source rule is about.
///
/// Being a pair is the thing that has to hold: `VoltageSourcePwlFile` was
/// missing from this side while `CurrentSourcePwlFile` was present on the
/// other, so a voltage source driven from a PWL file was invisible to every
/// rule that asks whether a net has a driver — no comment claimed it, and no
/// rule wants it. `every_independent_source_is_claimed_by_exactly_one_side`
/// in `tests` is what keeps the next family from reopening it.
const fn is_voltage_source(kind: ComponentType) -> bool {
    matches!(
        kind,
        ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm
            | ComponentType::VoltageSourceAm
            | ComponentType::VoltageSourcePat
            | ComponentType::VoltageSourceNoise
            | ComponentType::VoltageSourceRandom
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourcePwlFile
    )
}

/// Whether this type is an independent current source. See
/// [`is_voltage_source`] for why the pair is written out.
const fn is_current_source(kind: ComponentType) -> bool {
    matches!(
        kind,
        ComponentType::CurrentSource
            | ComponentType::CurrentSourceAc
            | ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourceSffm
            | ComponentType::CurrentSourceAm
            | ComponentType::CurrentSourcePat
            | ComponentType::CurrentSourceNoise
            | ComponentType::CurrentSourceRandom
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourcePwlFile
    )
}

/// Check one cellview with its authoritative connectivity and component facts.
/// Both inputs come from the same extraction pass and must describe the same
/// document.
fn check_one_view(
    schematic: &SchematicDocument,
    config: &DrcConfig,
    components: &[ComponentInfo],
    connectivity: &ExtractedConnectivity,
) -> DrcResult {
    let severity_overrides = &config.severity_overrides;
    let mut checker = DrcChecker::with_config(config.clone());
    let mut result = checker.check(components, connectivity);
    append_connectivity_diagnostics(connectivity, &mut result, severity_overrides);
    append_vector_width_violations(
        connectivity,
        &config.connectivity,
        &mut result,
        severity_overrides,
    );
    append_case_collision_violations(schematic, &mut result, severity_overrides);
    append_off_sheet_connector_violations(schematic, &mut result, severity_overrides);
    result
}

/// Drivers on the deck's global nodes, gathered across the whole walk.
///
/// A global is one node in the deck however many cellviews name it, and every
/// occurrence of a cellview places its drivers on it again. Ideal sources that
/// close a loop through global nodes, or two declared outputs on one global,
/// conflict wherever they are drawn — and the per-cellview rules cannot see the
/// conflict when its parts sit in different cellviews, or in two occurrences
/// of one.
///
/// Ground is node 0 everywhere, but it is the reference, not a contested net:
/// an output tied to ground in a cell placed twice is not two outputs fighting
/// over one node. For sources it is a node like any other, as it is in the
/// engine's graph: a source from a global to ground in a cell placed twice is
/// a loop through the global and ground.
#[derive(Default)]
struct GlobalDrives {
    /// Every independent voltage source in the walk, in walk order.
    sources: Vec<SourceBranch>,
    /// Declared outputs standing on a global node other than ground, by folded
    /// node name.
    outputs: BTreeMap<String, GlobalDriven>,
}

/// One independent voltage source, as a branch of the deck's source graph.
struct SourceBranch {
    view: usize,
    identity: String,
    /// Its `+` and `-` nodes as the cellview spells them.
    nodes: [String; 2],
    /// For each of the two, the folded name of the node every cellview shares
    /// — a global, or ground — or `None` for a node of the cellview's own.
    shared: [Option<String>; 2],
}

/// A node of the deck's source graph.
#[derive(PartialEq, Eq, Hash)]
enum SourceNode<'a> {
    /// A global or ground: one node across the whole design.
    Shared(&'a str),
    /// A node of one copy of one cellview.
    Own { view: usize, copy: u8, net: &'a str },
}

/// Terminals on each global node, counted across the whole walk.
///
/// A cellview's own floating-node rule takes a terminal on a global as
/// connected, because the global continues into every other cellview that
/// names it. Whether anything else does name it is a question about the whole
/// design, so the walk counts every terminal on each global at every
/// occurrence of the cellview that holds it. A global the design touches fewer
/// times than a node needs is floating, wherever its one terminal is drawn.
#[derive(Default)]
struct GlobalTouches {
    /// By folded node name.
    nodes: BTreeMap<String, GlobalTouched>,
}

#[derive(Default)]
struct GlobalTouched {
    /// The terminals on the node in each cellview that touches it, with the
    /// node as that cellview spells it.
    touches: Vec<(usize, u64, String)>,
    /// An interface port stands on the node somewhere, so it continues into a
    /// parent the walk does not see.
    continues: bool,
}

impl GlobalTouches {
    fn collect(&mut self, view: usize, components: &[ComponentInfo]) {
        let mut in_view = BTreeMap::<String, (u64, String)>::new();
        for component in components {
            for pin in component.pins.iter().filter(|pin| pin.on_global) {
                let key = pin.net_name.to_ascii_lowercase();
                if component.is_interface_port {
                    self.nodes.entry(key.clone()).or_default().continues = true;
                }
                in_view
                    .entry(key)
                    .or_insert_with(|| (0, pin.net_name.clone()))
                    .0 += 1;
            }
        }
        for (key, (terminals, node)) in in_view {
            self.nodes
                .entry(key)
                .or_default()
                .touches
                .push((view, terminals, node));
        }
    }

    /// Each global the design touches fewer than `min_connections` times, at
    /// the cellview that touches it first.
    fn floating(
        &self,
        occurrences: &[Occurrences],
        min_connections: usize,
    ) -> Vec<(usize, DrcViolation)> {
        let mut findings = Vec::new();
        for touched in self.nodes.values() {
            let Some((view, _, node)) = touched.touches.first() else {
                continue;
            };
            if touched.continues || crate::schematic::ground_names::is_ground_reference(node) {
                continue;
            }
            let total = touched
                .touches
                .iter()
                .map(|(view, terminals, _)| terminals.saturating_mul(occurrences[*view].count))
                .fold(0u64, u64::saturating_add);
            if total >= min_connections as u64 {
                continue;
            }
            findings.push((
                *view,
                DrcViolation::new(
                    0,
                    DrcViolationType::FloatingNode,
                    format!("Global node '{node}' has only {total} connection(s) in the design"),
                    DrcLocation::Node {
                        net_name: node.clone(),
                    },
                ),
            ));
        }
        findings
    }
}

/// What drives one global node, or one pair of them, across the walk.
struct GlobalDriven {
    /// The nodes as the drawing first spelled them, which is what a finding
    /// quotes.
    nodes: Vec<String>,
    drivers: Vec<GlobalDriver>,
}

/// One driver on a global node: the cellview it is drawn in, and what it is
/// called there.
struct GlobalDriver {
    view: usize,
    identity: String,
}

impl GlobalDrives {
    fn collect(&mut self, view: usize, components: &[ComponentInfo], globals: &HashSet<String>) {
        let global = |net: &str| {
            if source_loops::node_key(net) == GROUND_KEY {
                Some(GROUND_KEY.to_owned())
            } else {
                let folded = net.to_ascii_lowercase();
                globals.contains(&folded).then_some(folded)
            }
        };
        for component in components {
            let identity = component_identity(component);
            if component.is_voltage_source {
                let terminal = |name: &str| {
                    component
                        .pins
                        .iter()
                        .find(|pin| pin.name == name)
                        .map(|pin| pin.net_name.as_str())
                };
                let (Some(plus), Some(minus)) = (terminal("+"), terminal("-")) else {
                    continue;
                };
                self.sources.push(SourceBranch {
                    view,
                    identity,
                    nodes: [plus.to_owned(), minus.to_owned()],
                    shared: [global(plus), global(minus)],
                });
                continue;
            }
            // One driver per component and node, as the cellview's own rule
            // counts: two output pins of one instance on one global are that
            // instance's business, not two drivers fighting.
            let mut driven = BTreeMap::new();
            for pin in component.pins.iter().filter(|pin| pin.drives()) {
                if let Some(key) = global(&pin.net_name).filter(|key| key != GROUND_KEY) {
                    driven.entry(key).or_insert_with(|| pin.net_name.clone());
                }
            }
            for (key, node) in driven {
                self.outputs
                    .entry(key)
                    .or_insert_with(|| GlobalDriven {
                        nodes: vec![node],
                        drivers: Vec::new(),
                    })
                    .drivers
                    .push(GlobalDriver {
                        view,
                        identity: identity.clone(),
                    });
            }
        }
    }

    /// The conflicts that span more than one occurrence.
    ///
    /// Drivers that all sit in one occurrence of one cellview are that
    /// cellview's own rules' business, and are stated there.
    fn findings(
        &self,
        occurrences: &[Occurrences],
        views: &[ReachedView<'_>],
    ) -> Vec<DrcViolation> {
        let mut findings = self.source_loops(occurrences, views);
        for driven in self.outputs.values() {
            let Some((total, listed)) = conflict(&driven.drivers, occurrences) else {
                continue;
            };
            findings.push(
                DrcViolation::new(
                    0,
                    DrcViolationType::ShortedOutputs,
                    format!(
                        "Global node '{}' has {total} declared output drivers across the \
                         hierarchy: {}",
                        driven.nodes[0],
                        listed.join(", ")
                    ),
                    driven.location(occurrences, views),
                )
                .with_related(listed),
            );
        }
        findings
    }

    /// The loops the walk's sources close that no one cellview's rule sees.
    ///
    /// A cellview placed more than once puts its sources in the deck once per
    /// occurrence, each copy on nodes of its own except the shared ones. Two
    /// copies show every loop the repetition closes, because every further
    /// copy joins the shared nodes exactly as the second does; `conflict`
    /// then counts and lists all of them. A loop inside one copy of one
    /// cellview is that cellview's own rule's, and is stated there.
    fn source_loops(
        &self,
        occurrences: &[Occurrences],
        views: &[ReachedView<'_>],
    ) -> Vec<DrcViolation> {
        let mut copies = Vec::new();
        let mut branches = Vec::new();
        for (index, source) in self.sources.iter().enumerate() {
            let repeated = occurrences[source.view].count > 1;
            for copy in 0..=u8::from(repeated) {
                copies.push((index, copy));
                branches.push([0, 1].map(|end| match &source.shared[end] {
                    Some(key) => SourceNode::Shared(key),
                    None => SourceNode::Own {
                        view: source.view,
                        copy,
                        net: &source.nodes[end],
                    },
                }));
            }
        }

        let mut findings = Vec::new();
        let mut stated = HashSet::new();
        for members in source_loops::closed_loops(&branches) {
            let (first, first_copy) = copies[members[0]];
            let first_view = self.sources[first].view;
            if members.iter().all(|&member| {
                let (source, copy) = copies[member];
                self.sources[source].view == first_view && copy == first_copy
            }) {
                continue;
            }
            // Further copies close the same loop again through the same
            // sources, and `conflict` already lists every occurrence of each.
            let mut sources = members
                .iter()
                .map(|&member| copies[member].0)
                .collect::<Vec<_>>();
            sources.sort_by_key(|&source| (self.sources[source].view, source));
            sources.dedup();
            if !stated.insert(sources.clone()) {
                continue;
            }
            let drivers = sources
                .iter()
                .map(|&source| GlobalDriver {
                    view: self.sources[source].view,
                    identity: self.sources[source].identity.clone(),
                })
                .collect::<Vec<_>>();
            let Some((total, listed)) = conflict(&drivers, occurrences) else {
                continue;
            };
            // The loop is stated at a global the stated cellview's sources
            // stand on: it is a node that cellview draws, and the one the loop
            // leaves the cellview through.
            let global = sources
                .iter()
                .map(|&source| &self.sources[source])
                .filter(|source| source.view == drivers[0].view)
                .flat_map(|source| source.nodes.iter().zip(&source.shared))
                .find_map(|(node, shared)| {
                    shared
                        .as_deref()
                        .is_some_and(|key| key != GROUND_KEY)
                        .then_some(node)
                });
            let (through, node) = match global {
                Some(node) => ("global node", node.clone()),
                None => ("node", self.sources[sources[0]].nodes[0].clone()),
            };
            let driven = GlobalDriven {
                nodes: vec![node],
                drivers,
            };
            findings.push(
                DrcViolation::new(
                    0,
                    DrcViolationType::SourceToSource,
                    format!(
                        "{total} voltage sources form a closed loop through {through} '{}' \
                         across the hierarchy: {}",
                        driven.nodes[0],
                        listed.join(", ")
                    ),
                    driven.location(occurrences, views),
                )
                .with_related(listed),
            );
        }
        findings
    }
}

impl GlobalDriven {
    /// Where a conflict on this node is stated.
    ///
    /// Where the subject draws one of the drivers, the node is the subject's
    /// own, and that driver is something the subject can change. Where every
    /// driver is drawn below it, there is nothing in the subject's drawing to
    /// rewire: the conflict is stated inside the first driver's occurrence,
    /// which is where a reader goes to resolve it.
    fn location(&self, occurrences: &[Occurrences], views: &[ReachedView<'_>]) -> DrcLocation {
        let node = DrcLocation::Node {
            net_name: self.nodes[0].clone(),
        };
        let first = &self.drivers[0];
        if self.drivers.iter().any(|driver| driver.view == SUBJECT) {
            return node;
        }
        let Some(reference) = views[first.view].reference.clone() else {
            return node;
        };
        DrcLocation::InsideInstance {
            path: occurrences[first.view].paths[0].clone(),
            view: reference,
            inner: Box::new(node),
        }
    }
}

/// How many drivers one global conflict has in the deck, and the occurrences a
/// reader is shown, or `None` where there is no conflict this rule owns.
fn conflict(
    drivers: &[GlobalDriver],
    occurrences: &[Occurrences],
) -> Option<(String, Vec<String>)> {
    let mut total = 0u64;
    let mut saturated = false;
    for driver in drivers {
        let occurrence = &occurrences[driver.view];
        let (sum, overflowed) = total.overflowing_add(occurrence.count);
        total = if overflowed { u64::MAX } else { sum };
        saturated |= overflowed || occurrence.saturated;
    }
    if total <= 1 {
        return None;
    }
    let one_occurrence = drivers
        .iter()
        .all(|driver| driver.view == drivers[0].view && occurrences[driver.view].count == 1);
    if one_occurrence {
        return None;
    }

    let mut listed = Vec::new();
    for driver in drivers {
        for path in &occurrences[driver.view].paths {
            if listed.len() == MAX_REPORTED_OCCURRENCE_PATHS {
                break;
            }
            listed.push(if path.segments().is_empty() {
                driver.identity.clone()
            } else {
                format!("{path}/{}", driver.identity)
            });
        }
    }
    let shown = listed.len() as u64;
    if saturated || shown < total {
        listed.push(if saturated {
            "and more".to_owned()
        } else {
            format!("and {} more", total - shown)
        });
    }
    let total = if saturated {
        format!("at least {total}")
    } else {
        total.to_string()
    };
    Some((total, listed))
}

#[cfg(test)]
mod tests;
