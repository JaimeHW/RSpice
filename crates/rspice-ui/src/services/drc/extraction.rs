//! Extracting checkable data from a schematic.
//!
//! Binds every placed component to the design's one connectivity extraction and
//! reads the canonical property schema for each. The rules receive node names
//! the deck will actually write, so a finding quotes what the engine sees.

use super::checker::{
    DrcChecker, DrcConfig, append_case_collision_violations, append_off_sheet_connector_violations,
    append_vector_width_violations, diagnostic_location,
};
use super::input::{ComponentInfo, ParameterRangeIssue, PinInfo};
use super::netlist_gen::HierarchySource;
use super::netlist_gen::extraction::{
    ConnectivityDiagnosticKind, ExtractedConnectivity, ExtractedTerminal,
};
use super::types::{DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
use crate::state::{Component, ComponentType, PropertyDefinition, PropertyRegistry, PropertyValue};

/// Resolve the design once, and bind every placed component to it.
pub(super) fn extract_checked_design(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
) -> (Vec<ComponentInfo>, ExtractedConnectivity) {
    let connectivity = super::netlist_gen::extraction::extract(schematic, Some(hierarchy));
    let components = extract_components(schematic, &connectivity, |comp| {
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

fn extract_components(
    schematic: &crate::state::SchematicState,
    connectivity: &ExtractedConnectivity,
    mut component_known_for: impl FnMut(&Component) -> Option<bool>,
) -> Vec<ComponentInfo> {
    let mut components = Vec::with_capacity(schematic.components.len());
    let property_registry = PropertyRegistry::new();

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
        let declared_output_pins: std::collections::HashSet<String> = comp
            .library_cell
            .as_ref()
            .and_then(|binding| binding.interface())
            .into_iter()
            .flatten()
            .filter(|port| port.direction == crate::state::PortDirection::Out)
            .map(|port| port.name)
            .collect();

        for terminal in terminals {
            let is_output = matches!(
                comp.kind,
                ComponentType::VoltageSource
                    | ComponentType::VoltageSourceAc
                    | ComponentType::VoltageSourcePulse
                    | ComponentType::VoltageSourceSin
                    | ComponentType::VoltageSourceExp
                    | ComponentType::VoltageSourceSffm
                    | ComponentType::VoltageSourceAm
                    | ComponentType::VoltageSourcePat
                    | ComponentType::VoltageSourceNoise
                    | ComponentType::VoltageSourcePwl
                    | ComponentType::VoltageSourcePwlFile
            ) && terminal.pin == "+"
                || declared_output_pins.contains(&terminal.pin);

            pins.push(PinInfo {
                name: terminal.pin.clone(),
                net_name: terminal.net_name.clone(),
                is_output,
                point: terminal.point,
                attached: terminal.attached,
            });
        }

        let is_voltage_source = matches!(
            comp.kind,
            ComponentType::VoltageSource
                | ComponentType::VoltageSourceAc
                | ComponentType::VoltageSourcePulse
                | ComponentType::VoltageSourceSin
                | ComponentType::VoltageSourceExp
                | ComponentType::VoltageSourceSffm
                | ComponentType::VoltageSourceAm
                | ComponentType::VoltageSourcePat
                | ComponentType::VoltageSourceNoise
                | ComponentType::VoltageSourcePwl
        );

        let is_current_source = matches!(
            comp.kind,
            ComponentType::CurrentSource
                | ComponentType::CurrentSourceAc
                | ComponentType::CurrentSourcePulse
                | ComponentType::CurrentSourceSin
                | ComponentType::CurrentSourceExp
                | ComponentType::CurrentSourceSffm
                | ComponentType::CurrentSourceAm
                | ComponentType::CurrentSourcePat
                | ComponentType::CurrentSourceNoise
                | ComponentType::CurrentSourcePwl
                | ComponentType::CurrentSourcePwlFile
        );
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

fn effective_component_properties<'a>(
    component: &Component,
    registry: &'a PropertyRegistry,
) -> Option<(
    &'a crate::state::PropertySheet,
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
        crate::properties::property_bridge::collect_properties_from_component(component, registry),
    ))
}

fn missing_parameters(component: &Component, registry: &PropertyRegistry) -> Vec<String> {
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
                    crate::state::PropertyType::Number | crate::state::PropertyType::Expression
                )
                && value.trim().is_empty()
        }
        PropertyValue::Number { value, .. } => value.is_nan(),
        PropertyValue::Enum { .. } | PropertyValue::Boolean(_) => false,
    }
}

fn out_of_range_parameters(
    component: &Component,
    registry: &PropertyRegistry,
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
    crate::properties::tabbed_dialog::parse_expression_source(
        &unconstrained,
        source,
        crate::quantity::QuantityPresentationPolicy::default(),
        crate::quantity::UiNumberLocale::default(),
    )
    .ok()
    .and_then(|value| value.as_number())
    .filter(|value| value.is_finite())
}

/// Run a configured DRC check with project-cell symbol resolution enabled.
pub fn run_drc_check_with_hierarchy_and_config(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
    config: DrcConfig,
) -> DrcResult {
    let start = crate::time_compat::Instant::now();
    let (components, connectivity) = extract_checked_design(schematic, hierarchy);
    let severity_overrides = config.severity_overrides.clone();
    let connectivity_policy = config.connectivity.clone();
    let mut checker = DrcChecker::with_config(config);
    let mut result = checker.check(&components, &connectivity);
    append_connectivity_diagnostics(&connectivity, &mut result, &severity_overrides);
    append_vector_width_violations(
        &connectivity,
        &connectivity_policy,
        &mut result,
        &severity_overrides,
    );
    append_case_collision_violations(schematic, &mut result, &severity_overrides);
    append_off_sheet_connector_violations(schematic, &mut result, &severity_overrides);
    result.completed = true;
    result.duration_ms = start.elapsed().as_millis() as u64;
    result
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

#[cfg(test)]
mod tests;
