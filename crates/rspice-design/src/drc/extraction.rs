//! Map one extracted design into rule inputs and assemble its complete report.
//!
//! Binds every placed component to the design's one connectivity extraction and
//! reads the canonical property schema for each. The rules receive node names
//! the deck will actually write, so a finding quotes what the engine sees.

use super::checker::{
    DrcChecker, DrcConfig, append_case_collision_violations, append_off_sheet_connector_violations,
    append_vector_width_violations, diagnostic_location,
};
use super::input::{ComponentInfo, ParameterRangeIssue, PinInfo};
use super::types::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
use crate::connectivity::{ConnectivityDiagnosticKind, ExtractedConnectivity, ExtractedTerminal};
use crate::hierarchy::HierarchySource;
use crate::properties::PropertyCatalog;
use crate::properties::PropertyDefinition;
use crate::properties::PropertyValue;
use crate::schematic::component::Component;
use crate::schematic::component_type::ComponentType;
use crate::schematic::document::SchematicDocument;
use crate::schematic::port::PortDirection;
use rspice_app_types::hierarchy_path::InstancePath;
use rspice_design_model::cell_view::CellViewRef;

/// Resolve the design once, and bind every placed component to it.
pub fn extract_checked_design(
    schematic: &impl AsRef<SchematicDocument>,
    hierarchy: &HierarchySource<'_>,
) -> (Vec<ComponentInfo>, ExtractedConnectivity) {
    let connectivity = crate::connectivity::extract_with_hierarchy(schematic, Some(hierarchy));
    let components = extract_components(schematic.as_ref(), &connectivity, |comp| {
        if comp.kind != ComponentType::CellInstance {
            return Some(true);
        }
        let Some(binding) = comp.library_cell.as_ref() else {
            return Some(false);
        };
        if binding.source_path.is_some()
            || binding.netlist_template.is_some()
            || binding.is_executable_builtin()
        {
            return Some(true);
        }
        if hierarchy.has_execution_plan() {
            // Top-level DRC extraction has no instance path with which to
            // query an execution-plan rebind. Do not judge the placed
            // binding when the executable authority may select another
            // master.
            return None;
        }
        Some(hierarchy.schematic_master_for_binding(binding).is_some())
    });
    (components, connectivity)
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
pub fn run_check_with_hierarchy(
    schematic: &impl AsRef<SchematicDocument>,
    hierarchy: &HierarchySource<'_>,
    config: DrcConfig,
) -> DrcResult {
    let start = web_time::Instant::now();
    let subject = schematic.as_ref();
    let graph = InstanceGraph::walk(subject, hierarchy);

    // The ground rule is the one question no single cellview can answer, so it
    // is held back until the whole walk has been extracted.
    let mut view_config = config.clone();
    view_config.check_missing_ground = false;

    let mut reports = Vec::with_capacity(graph.views.len());
    let mut design_reaches_ground = false;
    let mut subject_inputs = None;
    for (index, view) in graph.views.iter().enumerate() {
        let (components, connectivity) = extract_checked_design(&view.document, hierarchy);
        design_reaches_ground |= reaches_ground(&components);
        let mut report = check_one_view(view.document, &view_config, &components, &connectivity);
        graph.append_recursive_findings(index, &view_config.severity_overrides, &mut report);
        if index == SUBJECT {
            subject_inputs = Some((components, connectivity));
        }
        reports.push(report);
    }

    let occurrences = graph.occurrences();
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

    for index in (SUBJECT + 1)..graph.views.len() {
        let reference = graph.views[index]
            .reference
            .as_ref()
            .expect("a reached cellview is named by the binding that reached it");
        let occurrence = &occurrences[index];
        let flooded = occurrence.count > MAX_REPORTED_OCCURRENCE_PATHS as u64;
        let paths = if flooded {
            &occurrence.paths[..1]
        } else {
            occurrence.paths.as_slice()
        };
        let count = flooded.then_some(occurrence.count);
        for path in paths {
            for finding in reports[index].violations() {
                result.add_violation(restated(finding, next_id, Some((path, reference)), count));
                next_id += 1;
            }
        }
    }

    result.completed = true;
    result.duration_ms = start.elapsed().as_millis() as u64;
    result
}

/// How many occurrence paths one cellview's findings are repeated over.
///
/// A cellview placed a few times is worth reporting at each place: the defect
/// may be acceptable at one occurrence and not at another, and the terminal in
/// `X1` is not the terminal in `X2`. Past that the repetition would bury every
/// other finding in the report, so the finding is stated once with its exact
/// occurrence count — which is a count the reader can act on, and not an
/// occurrence quietly dropped.
const MAX_REPORTED_OCCURRENCE_PATHS: usize = 8;

/// Restate one cellview's finding in the merged report.
///
/// `occurrence` is `None` for the subject and names the path and cellview
/// otherwise. `occurrence_count` is set only where the finding stands for more
/// occurrences than the report lists.
fn restated(
    finding: &DrcViolation,
    id: usize,
    occurrence: Option<(&InstancePath, &CellViewRef)>,
    occurrence_count: Option<u64>,
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
    if let Some(count) = occurrence_count {
        message = format!("{message} ({count} occurrences; reported once, at {path})");
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
    schematic: &crate::schematic::document::SchematicDocument,
    connectivity: &ExtractedConnectivity,
    mut component_known_for: impl FnMut(&Component) -> Option<bool>,
) -> Vec<ComponentInfo> {
    let mut components = Vec::with_capacity(schematic.components.len());
    let property_registry = PropertyCatalog::new();

    // Terminals arrive in placement order and, within a component, in the
    // order the symbol declares its pins. Grouping preserves both.
    let mut bound: std::collections::HashMap<u64, Vec<&ExtractedTerminal>> =
        std::collections::HashMap::new();
    for terminal in &connectivity.terminals {
        bound
            .entry(terminal.component_id)
            .or_default()
            .push(terminal);
    }

    for comp in &schematic.components {
        let terminals = bound.get(&comp.id).map(Vec::as_slice).unwrap_or_default();
        let mut pins = Vec::with_capacity(terminals.len());
        let declared_directions = declared_port_directions(comp);

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
                declared_directions.get(&terminal.pin).copied()
            };

            pins.push(PinInfo {
                name: terminal.pin.clone(),
                net_name: terminal.net_name.clone(),
                direction,
                point: terminal.point,
                attached: terminal.attached,
            });
        }

        let is_voltage_source = is_voltage_source(comp.kind);
        let is_current_source = is_current_source(comp.kind);
        let reference_required = !comp.kind.spice_prefix().is_empty();
        let reference_error = (reference_required && !comp.name.trim().is_empty())
            .then(|| comp.validate_reference_designator(comp.name.trim()).err())
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
            component_known: component_known_for(comp),
            missing_parameters: missing_parameters(comp, &property_registry),
            out_of_range_parameters: out_of_range_parameters(comp, &property_registry),
        });
    }

    components
}

/// Every direction the bound cell interface declares, keyed by the pin name the
/// symbol places.
///
/// A primitive binds no interface and therefore declares nothing: the map is
/// empty, every one of its terminals reads as undeclared, and an open resistor
/// leg is never judged as an input nothing drives.
///
/// A drawn interface port's own `dir=` is deliberately not read here. That
/// declaration is written from the parent's side of the symbol, so inside the
/// master an `out` port is where the cellview's own circuitry arrives — reading
/// it here would make a port a driver within the very cellview that drives it.
fn declared_port_directions(
    component: &Component,
) -> std::collections::HashMap<String, PortDirection> {
    let mut declared = std::collections::HashMap::new();
    for port in component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.interface())
        .into_iter()
        .flatten()
    {
        // One name declared twice keeps the `Out` claim rather than whichever
        // declaration was read last, which is the reading the driver rules
        // have always had.
        match declared.entry(port.name) {
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

/// The subject's index in the walk. It is the only cellview whose findings are
/// stated without an occurrence path.
const SUBJECT: usize = 0;

/// One cellview the walk reached.
struct ReachedView<'a> {
    /// The cellview the binding named, or `None` for the subject, which the
    /// caller names rather than a binding.
    reference: Option<CellViewRef>,
    document: &'a SchematicDocument,
    /// The first occurrence path that reached it, which is proof that at least
    /// one path can name it. The subject's is the root.
    first_path: InstancePath,
}

/// One placed instance, as an edge of the instantiated hierarchy.
struct InstanceEdge {
    parent: usize,
    child: usize,
    /// The instance reference as the drawing spells it: one path segment.
    instance: String,
}

/// One instance whose master is already one of its own enclosing cellviews.
struct RecursiveInstance {
    parent: usize,
    component_id: u64,
    component_name: String,
    repeated: CellViewRef,
    /// The enclosing cellviews from the repeated one down to the instance, as
    /// a reader can follow them.
    chain: String,
}

/// Exactly how many times one cellview occurs, and the first few paths that
/// reach it.
struct Occurrences {
    count: u64,
    paths: Vec<InstancePath>,
}

/// The instantiated hierarchy below one checked cellview.
///
/// The walk follows the drawing's placed bindings. It deliberately does not ask
/// a configuration's execution plan which view an occurrence resolves to: the
/// subject of a check is any cellview, not necessarily the design root, so a
/// path here is relative to that subject and is not a path the plan can be
/// queried with. Where a configuration may rebind a master, the per-cellview
/// type check already declines to judge rather than inventing a finding.
struct InstanceGraph<'a> {
    views: Vec<ReachedView<'a>>,
    edges: Vec<InstanceEdge>,
    recursive: Vec<RecursiveInstance>,
    /// View index by case-folded `library/cell/view`.
    ///
    /// The subject is absent from this map, because a caller hands over a
    /// document rather than the cellview it belongs to. An instance that binds
    /// the subject's own cellview therefore opens a second view of the same
    /// document instead of closing the cycle at the subject — and the cycle is
    /// then caught one level further down, where that view reaches itself. The
    /// walk still terminates and still reports, which is what the guard is for.
    index: std::collections::HashMap<String, usize>,
}

impl<'a> InstanceGraph<'a> {
    fn walk(subject: &'a SchematicDocument, hierarchy: &HierarchySource<'a>) -> Self {
        let mut graph = Self {
            views: vec![ReachedView {
                reference: None,
                document: subject,
                first_path: InstancePath::root(),
            }],
            edges: Vec::new(),
            recursive: Vec::new(),
            index: std::collections::HashMap::new(),
        };
        graph.descend(SUBJECT, hierarchy, &mut vec![SUBJECT]);
        graph
    }

    /// Add every instance of one cellview, descending into each master the
    /// first time it is reached.
    ///
    /// `enclosing` is the chain of views from the subject to this one. An
    /// instance whose master is already on it closes a cycle: it is reported
    /// and not followed, because there is no finite expansion to check.
    fn descend(
        &mut self,
        parent: usize,
        hierarchy: &HierarchySource<'a>,
        enclosing: &mut Vec<usize>,
    ) {
        let document = self.views[parent].document;
        let parent_path = self.views[parent].first_path.clone();
        let mut instances = document
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .collect::<Vec<_>>();
        instances.sort_by_key(|component| component.id);

        for instance in instances {
            let Some(binding) = instance.library_cell.as_ref() else {
                continue;
            };
            // A binding that carries its own netlist text or source is a leaf:
            // that text is what the deck writes, so there is no schematic below
            // it for a rule to read.
            if binding.source_path.is_some()
                || binding.netlist_template.is_some()
                || binding.is_executable_builtin()
            {
                continue;
            }
            let Some(master) = hierarchy.schematic_master_for_binding(binding) else {
                continue;
            };
            // A finding below this instance is named by its occurrence path, so
            // the instance's own reference has to be a path segment. An empty
            // or unusable reference is already an error on this very instance
            // from the reference rules, and the depth and length limits here
            // are the limits the hierarchy resolver stops at and reports; in
            // every case the design surface states the boundary, so the walk
            // does not invent a second finding for it.
            let Ok(child_path) = parent_path.child(instance.name.trim()) else {
                continue;
            };
            let reference = CellViewRef::new(&binding.library, &binding.cell, &binding.view);
            let key = reference.key().to_ascii_lowercase();

            match self.index.get(&key).copied() {
                Some(child) if enclosing.contains(&child) => {
                    let chain = self.enclosing_chain(enclosing, child, &reference);
                    self.recursive.push(RecursiveInstance {
                        parent,
                        component_id: instance.id,
                        component_name: instance.name.clone(),
                        repeated: reference,
                        chain,
                    });
                }
                Some(child) => {
                    self.edges.push(InstanceEdge {
                        parent,
                        child,
                        instance: instance.name.trim().to_owned(),
                    });
                }
                None => {
                    let child = self.views.len();
                    self.views.push(ReachedView {
                        reference: Some(reference),
                        document: master,
                        first_path: child_path,
                    });
                    self.index.insert(key, child);
                    self.edges.push(InstanceEdge {
                        parent,
                        child,
                        instance: instance.name.trim().to_owned(),
                    });
                    enclosing.push(child);
                    self.descend(child, hierarchy, enclosing);
                    enclosing.pop();
                }
            }
        }
    }

    /// The cycle as a reader follows it: from the repeated cellview down to the
    /// instance that reaches it again.
    fn enclosing_chain(
        &self,
        enclosing: &[usize],
        repeated: usize,
        reference: &CellViewRef,
    ) -> String {
        let from = enclosing
            .iter()
            .position(|view| *view == repeated)
            .unwrap_or(0);
        let mut chain = enclosing[from..]
            .iter()
            .filter_map(|view| self.views[*view].reference.as_ref())
            .map(CellViewRef::display_path)
            .collect::<Vec<_>>();
        chain.push(reference.display_path());
        chain.join(" -> ")
    }

    /// State every cycle found inside one cellview, in that cellview's own
    /// report.
    fn append_recursive_findings(
        &self,
        parent: usize,
        severity_overrides: &std::collections::HashMap<DrcViolationType, DrcSeverity>,
        result: &mut DrcResult,
    ) {
        for recursive in self
            .recursive
            .iter()
            .filter(|recursive| recursive.parent == parent)
        {
            let instance = if recursive.component_name.trim().is_empty() {
                format!("component #{}", recursive.component_id)
            } else {
                recursive.component_name.clone()
            };
            let mut violation = DrcViolation::new(
                result.total_count(),
                DrcViolationType::RecursiveInstantiation,
                format!(
                    "{instance} reaches {}, which already encloses it: {}",
                    recursive.repeated.display_path(),
                    recursive.chain
                ),
                DrcLocation::Component {
                    id: recursive.component_id,
                    name: recursive.component_name.clone(),
                },
            )
            .with_related(vec![instance, recursive.repeated.display_path()]);
            if let Some(severity) =
                severity_overrides.get(&DrcViolationType::RecursiveInstantiation)
            {
                violation.severity = *severity;
            }
            result.add_violation(violation);
        }
    }

    /// Exactly how often each cellview occurs, with the first few paths that
    /// reach it.
    ///
    /// The count is computed over the edges rather than by walking every
    /// occurrence, so a wide hierarchy costs nothing to count exactly even
    /// where enumerating its occurrences would be hopeless. Cycles were
    /// excluded as they were found, so what remains is acyclic and one pass in
    /// dependency order is enough.
    fn occurrences(&self) -> Vec<Occurrences> {
        let mut occurrences = self
            .views
            .iter()
            .map(|_| Occurrences {
                count: 0,
                paths: Vec::new(),
            })
            .collect::<Vec<_>>();
        occurrences[SUBJECT].count = 1;
        occurrences[SUBJECT].paths = vec![InstancePath::root()];

        let mut outgoing = vec![Vec::new(); self.views.len()];
        let mut remaining = vec![0usize; self.views.len()];
        for (index, edge) in self.edges.iter().enumerate() {
            outgoing[edge.parent].push(index);
            remaining[edge.child] += 1;
        }

        let mut ready = std::collections::VecDeque::from([SUBJECT]);
        while let Some(parent) = ready.pop_front() {
            for edge in &outgoing[parent] {
                let edge = &self.edges[*edge];
                occurrences[edge.child].count = occurrences[edge.child]
                    .count
                    .saturating_add(occurrences[parent].count);
                let mut extended = Vec::new();
                for path in &occurrences[parent].paths {
                    if occurrences[edge.child].paths.len() + extended.len()
                        >= MAX_REPORTED_OCCURRENCE_PATHS
                    {
                        break;
                    }
                    if let Ok(path) = path.child(&edge.instance) {
                        extended.push(path);
                    }
                }
                occurrences[edge.child].paths.extend(extended);
                remaining[edge.child] -= 1;
                if remaining[edge.child] == 0 {
                    ready.push_back(edge.child);
                }
            }
        }

        // Every reached cellview was reached by a path that parsed, so it is
        // named by at least that one however its later occurrences fare.
        for (index, occurrence) in occurrences.iter_mut().enumerate() {
            if occurrence.paths.is_empty() {
                occurrence.paths.push(self.views[index].first_path.clone());
            }
            occurrence.count = occurrence.count.max(1);
        }
        occurrences
    }
}

#[cfg(test)]
mod tests;
