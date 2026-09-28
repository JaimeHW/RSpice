//! Materialize authored sheets, variants and reference annotation for execution.

use crate::state::{DesignManagementCatalog, SchematicState};
use std::collections::{BTreeMap, HashMap, HashSet};

impl SchematicState {
    pub(crate) fn materialize_design_management_schematic(
        &self,
        design_management: &DesignManagementCatalog,
        cell_view_key: &str,
    ) -> Result<SchematicState, crate::state::DesignManagementError> {
        design_management.validate()?;
        let source = self;
        let mut projected = source.clone();

        if let Some(catalog) = design_management.sheet_catalog(cell_view_key) {
            let offsets = catalog
                .sheets()
                .iter()
                .enumerate()
                .map(|(index, sheet)| {
                    let ordinal = i32::try_from(index).unwrap_or(i32::MAX);
                    (
                        sheet.id(),
                        crate::state::Point::new(ordinal.saturating_mul(1_000_000), 0),
                    )
                })
                .collect::<HashMap<_, _>>();
            let offset_for = |object_id: u64| {
                design_management
                    .sheet_for_object_or_active(cell_view_key, object_id)
                    .and_then(|sheet_id| offsets.get(&sheet_id).copied())
                    .unwrap_or_else(crate::state::Point::origin)
            };

            for component in &mut projected.document.components {
                component.pos = translated_point(component.pos, offset_for(component.id))?;
            }
            for wire in &mut projected.document.wires {
                let delta = offset_for(wire.id);
                for point in &mut wire.points {
                    *point = translated_point(*point, delta)?;
                }
            }
            for bus in &mut projected.document.buses {
                let delta = offset_for(bus.id);
                for point in &mut bus.points {
                    *point = translated_point(*point, delta)?;
                }
            }
            for tap in &mut projected.document.bus_taps {
                let delta = offset_for(tap.id);
                tap.bus_point = translated_point(tap.bus_point, delta)?;
                tap.connection_point = translated_point(tap.connection_point, delta)?;
            }
            for junction in &mut projected.document.junctions {
                junction.pos = translated_point(junction.pos, offset_for(junction.id))?;
            }
            for label in &mut projected.document.net_labels {
                label.pos = translated_point(label.pos, offset_for(label.id))?;
            }
            for note in &mut projected.document.design_notes {
                note.pos = translated_point(note.pos, offset_for(note.id))?;
            }
            for shape in &mut projected.document.documentation_shapes {
                let delta = offset_for(shape.id);
                let (minimum, maximum) = shape.bounds();
                let _ = translated_point(minimum, delta)?;
                let _ = translated_point(maximum, delta)?;
                shape.translate(delta);
            }

            for contract in catalog.cross_sheet_ports() {
                for endpoint in [&contract.definition().first, &contract.definition().second] {
                    if catalog.sheet_for_object(endpoint.object_id()) != Some(endpoint.sheet_id) {
                        return Err(crate::state::DesignManagementError::MissingReference {
                            domain: "cross-sheet anchor sheet assignment",
                            identity: endpoint.object_id().to_string(),
                        });
                    }
                    let delta = offsets.get(&endpoint.sheet_id).copied().ok_or_else(|| {
                        crate::state::DesignManagementError::MissingReference {
                            domain: "cross-sheet port sheet",
                            identity: endpoint.sheet_id.to_string(),
                        }
                    })?;
                    let anchor = projected_cross_sheet_anchor(source, &projected, endpoint, delta)?;
                    // A materialized crossing is exactly what an authored
                    // off-sheet connector is, so it carries the contract's
                    // direction rather than reading as a plain local name.
                    let next_id = projected.next_id();
                    projected
                        .document
                        .net_labels
                        .push(crate::state::NetLabel::off_sheet(
                            next_id,
                            anchor,
                            contract.definition().net_name.clone(),
                            contract.definition().direction,
                        ));
                }
            }
        }

        let active_variant = design_management
            .variants()
            .active()
            .map(|variant| design_management.variants().resolve(variant.id()))
            .transpose()?;
        if let Some(resolved) = &active_variant {
            let mut do_not_populate = HashSet::new();
            for component in &projected.document.components {
                if matches!(
                    resolved.override_for(cell_view_key, component.id)?,
                    Some(crate::state::VariantObjectOverride::DoNotPopulate { .. })
                ) {
                    do_not_populate.insert(component.id);
                }
            }
            projected
                .document
                .components
                .retain(|component| !do_not_populate.contains(&component.id));
            projected
                .document
                .connections
                .retain(|connection| !do_not_populate.contains(&connection.component_id));
        }

        // Old project files can retain an approved journal before its names
        // were written into the buffers. Resolve all original component names
        // together so swaps and structural references retain their targets.
        // Annotation describes the authored device; substitutions below can
        // turn that primitive into a cell with a different emitted prefix.
        let annotation = design_management.annotation();
        let names = if annotation.journal().is_empty() {
            BTreeMap::new()
        } else {
            let sources = projected
                .document
                .components
                .iter()
                .map(|component| {
                    Ok((
                        crate::state::SchematicObjectKey::new(cell_view_key, component.id)?,
                        component.name.as_str(),
                    ))
                })
                .collect::<Result<Vec<_>, crate::state::DesignManagementError>>()?;
            annotation
                .projected_reference_assignments(sources)?
                .into_iter()
                .map(|(object, name)| (object.object_id(), name))
                .collect()
        };
        if !names.is_empty() {
            projected.document.components =
                projected
                    .prepare_component_renames(&names)
                    .map_err(|reason| {
                        crate::state::DesignManagementError::InvalidAnnotationProjection {
                            cell_view_key: cell_view_key.to_owned(),
                            reason,
                        }
                    })?;
        }

        if let Some(resolved) = &active_variant {
            for component in &mut projected.document.components {
                let Some(override_value) = resolved.override_for(cell_view_key, component.id)?
                else {
                    continue;
                };
                match override_value {
                    crate::state::VariantObjectOverride::DoNotPopulate { .. } => {
                        // Removed before reference preparation.
                    }
                    crate::state::VariantObjectOverride::Substitute { replacement } => {
                        crate::state::params_string::validate_parameter_text(&component.params)
                            .map_err(|reason| {
                                crate::state::DesignManagementError::InvalidReplacementParameters {
                                    object: crate::state::SchematicObjectKey::new(
                                        cell_view_key,
                                        component.id,
                                    )
                                    .expect("validated override owner"),
                                    reason,
                                }
                            })?;
                        let prior = component.library_cell.take();
                        let mut binding = crate::state::LibraryCellInstance::new(
                            replacement.library.clone(),
                            replacement.cell.clone(),
                            replacement.view.clone(),
                        );
                        if let Some(prior) = prior {
                            binding.terminal_order = prior.terminal_order;
                            binding.terminal_dirs = prior.terminal_dirs;
                            binding.interface_bound = prior.interface_bound;
                        }
                        binding.model_section.clone_from(&replacement.model_section);
                        component.kind = crate::state::ComponentType::CellInstance;
                        component.library_cell = Some(binding);
                        if let Some(value) = &replacement.value_override {
                            component.value.clone_from(value);
                        }
                    }
                }
            }
        }
        projected.recalculate_runtime_state();
        Ok(projected)
    }
}

fn translated_point(
    point: crate::state::Point,
    delta: crate::state::Point,
) -> Result<crate::state::Point, crate::state::DesignManagementError> {
    Ok(crate::state::Point::new(
        point
            .x
            .checked_add(delta.x)
            .ok_or(crate::state::DesignManagementError::NumericRange(
                "materialized sheet x coordinate",
            ))?,
        point
            .y
            .checked_add(delta.y)
            .ok_or(crate::state::DesignManagementError::NumericRange(
                "materialized sheet y coordinate",
            ))?,
    ))
}

/// The world point one named terminal of a placed instance sits at, read from
/// the instance's own durable placement and bound interface.
///
/// A bound cell instance names its terminals twice: by interface port where
/// the binding carries one, and by ordinal position otherwise. A contract may
/// have been authored against either spelling, so both are matched — against
/// the same transformed point in both cases, because placement, rotation and
/// mirror are what put the terminal there.
fn component_terminal_point(
    component: &crate::state::Component,
    terminal_name: &str,
) -> Option<crate::state::Point> {
    let interface = component.instance_pin_layout();
    component
        .terminal_positions()
        .into_iter()
        .enumerate()
        .find(|(index, (label, _))| {
            label.eq_ignore_ascii_case(terminal_name)
                || interface.get(*index).is_some_and(|(port, _)| {
                    port.as_deref()
                        .is_some_and(|port| port.eq_ignore_ascii_case(terminal_name))
                })
        })
        .map(|(_, (_, point))| point)
}

/// Resolve one typed cross-sheet endpoint against the authored topology and
/// then project it into the endpoint sheet's execution namespace. A wire
/// point must still lie on its retained conductor; a component terminal must
/// still sit on one. Stale contracts fail before DRC or netlisting rather
/// than silently connecting a label to a component origin.
///
/// Both endpoints resolve from durable design data alone. The runtime
/// connection cache is stripped on save and rebuilt only for the buffer the
/// session has open, so a contract that resolved through it failed for every
/// cell view that was not the active one.
fn projected_cross_sheet_anchor(
    source: &SchematicState,
    projected: &SchematicState,
    endpoint: &crate::state::CrossSheetPortEndpoint,
    delta: crate::state::Point,
) -> Result<crate::state::Point, crate::state::DesignManagementError> {
    let authored_point = match &endpoint.anchor {
        crate::state::CrossSheetPortAnchor::WirePoint { wire_id, point } => {
            let wire = source
                .document
                .wires
                .iter()
                .find(|wire| wire.id == *wire_id)
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet wire anchor",
                    identity: wire_id.to_string(),
                })?;
            if !wire.contains_point(*point) {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet wire anchor point",
                    identity: format!("{}@{},{}", wire_id, point.x, point.y),
                });
            }
            *point
        }
        crate::state::CrossSheetPortAnchor::ComponentTerminal {
            component_id,
            terminal_name,
        } => {
            let component = source
                .document
                .components
                .iter()
                .find(|component| component.id == *component_id)
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component anchor",
                    identity: component_id.to_string(),
                })?;
            let point = component_terminal_point(component, terminal_name).ok_or_else(|| {
                crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component terminal",
                    identity: format!("{}:{}", component_id, terminal_name),
                }
            })?;
            if !source
                .document
                .wires
                .iter()
                .any(|wire| wire.contains_point(point))
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component terminal connection",
                    identity: format!("{}:{}", component_id, terminal_name),
                });
            }
            point
        }
    };
    let anchor = translated_point(authored_point, delta)?;
    match &endpoint.anchor {
        crate::state::CrossSheetPortAnchor::WirePoint { wire_id, .. } => {
            if !projected
                .document
                .wires
                .iter()
                .any(|wire| wire.id == *wire_id && wire.contains_point(anchor))
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "projected cross-sheet wire anchor",
                    identity: wire_id.to_string(),
                });
            }
        }
        crate::state::CrossSheetPortAnchor::ComponentTerminal { component_id, .. } => {
            if !projected
                .document
                .components
                .iter()
                .any(|component| component.id == *component_id)
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "projected cross-sheet component anchor",
                    identity: component_id.to_string(),
                });
            }
        }
    }
    Ok(anchor)
}

impl SchematicState {
    pub(crate) fn apply_variant_replacement(
        &mut self,
        prepared: rspice_design::schematic::component::PreparedVariantReplacement,
    ) {
        prepared.apply_to(&mut self.document);
    }
}
