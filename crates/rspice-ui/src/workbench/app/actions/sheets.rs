//! Sheet navigation and sheet lifecycle for the active cell view.
//!
//! Sheets are project-owned identities, so every operation here publishes one
//! reviewed design-management transaction against the live project rather than
//! editing a dialog draft. That is what lets a strip chip, a chord, a palette
//! row and the design manager act on the same sheet without three of them
//! holding a private copy of the catalog.
//!
//! Activation is the exception that proves the rule: it is navigation, not
//! authoring, so it syncs the live schematic into its buffer first, then
//! retires selection and net highlighting — stale identities from the sheet
//! being left must never reach a command run on the sheet being entered.

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDefinition, CrossSheetPortDirection,
    CrossSheetPortEndpoint, CrossSheetSignalType, DesignManagementCatalog, DrawingSheetInheritance,
    DrawingSheetNewSheetPolicy, MoveBoundaryResolution, MoveSelectionRequest, PortDirection,
    PortDiscipline, ReorderPageNumbering, SchematicSheetFormat, SheetCatalog, SheetDefinition,
    SheetDeleteBehavior, SheetDeleteResolution, SheetId, SheetMoveConnectivityPlan,
    SheetPortPolicy, SheetTemplate,
};
use crate::workbench::app_state::{AppState, DesignManagementHistoryEntry};

const NO_CATALOG: &str = "The active cell view has no governed sheet catalog.";
const NO_SHEET: &str = "That sheet is no longer part of the governed catalog.";

/// One sheet as the navigation surfaces present it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) struct SheetEntry {
    pub(in crate::workbench) id: SheetId,
    pub(in crate::workbench) name: String,
    pub(in crate::workbench) page: u32,
}

/// What deleting a sheet would do, resolved before anything is committed so a
/// confirmation can state the outcome instead of describing the button.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) enum SheetDeletePlan {
    LastSheet,
    Empty,
    Blocked {
        objects: usize,
    },
    MovesObjects {
        objects: usize,
        destination: SheetId,
        destination_name: String,
    },
}

pub(in crate::workbench) fn active_sheet_catalog(state: &AppState) -> Option<&SheetCatalog> {
    state
        .workspace
        .design_management
        .sheet_catalog(&state.workspace.active_schematic_reference().key())
}

pub(in crate::workbench) fn sheet_entries(state: &AppState) -> Vec<SheetEntry> {
    let Some(catalog) = active_sheet_catalog(state) else {
        return Vec::new();
    };
    catalog
        .sheets()
        .iter()
        .enumerate()
        .map(|(index, sheet)| SheetEntry {
            id: sheet.id(),
            name: sheet.name().to_owned(),
            page: catalog
                .page_number_and_count(sheet.id())
                .map_or_else(|| index as u32 + 1, |(page, _)| page),
        })
        .collect()
}

pub(in crate::workbench) fn active_sheet_id(state: &AppState) -> Option<SheetId> {
    active_sheet_catalog(state).and_then(SheetCatalog::active_sheet_id)
}

pub(in crate::workbench) fn sheet_count(state: &AppState) -> usize {
    active_sheet_catalog(state).map_or(0, |catalog| catalog.sheets().len())
}

/// Make one sheet the canvas the session is editing.
///
/// Re-activating the sheet already active is inert: it publishes nothing, so
/// a chip redrawn every frame cannot advance the project revision.
pub(in crate::workbench) fn activate_sheet(
    state: &mut AppState,
    id: SheetId,
) -> Result<String, String> {
    let name = state_sheet_name(state, id);
    let outcome = activate(state, id, name);
    report(state, outcome)
}

pub(in crate::workbench) fn next_sheet(state: &mut AppState) -> Result<String, String> {
    step(state, 1)
}

pub(in crate::workbench) fn previous_sheet(state: &mut AppState) -> Result<String, String> {
    step(state, -1)
}

pub(in crate::workbench) fn go_to_sheet(
    state: &mut AppState,
    index: usize,
) -> Result<String, String> {
    let Some(id) = active_sheet_catalog(state)
        .and_then(|catalog| catalog.sheets().get(index))
        .map(|sheet| sheet.id())
    else {
        return report(state, Err(NO_SHEET.to_owned()));
    };
    activate_sheet(state, id)
}

/// Create a sheet after the active one and enter it.
pub(in crate::workbench) fn new_sheet(state: &mut AppState) -> Result<String, String> {
    let after = active_sheet_id(state);
    new_sheet_after(state, after)
}

/// Create a sheet directly after `after` and enter it.
pub(in crate::workbench) fn new_sheet_after(
    state: &mut AppState,
    after: Option<SheetId>,
) -> Result<String, String> {
    match create(state, after) {
        Ok((id, name)) => {
            let _ = activate(state, id, name.clone());
            report(state, Ok(format!("Created sheet `{name}`.")))
        }
        Err(error) => report(state, Err(error)),
    }
}

pub(in crate::workbench) fn rename_sheet(
    state: &mut AppState,
    id: SheetId,
    name: String,
) -> Result<String, String> {
    let name = name.trim().to_owned();
    let outcome = edit_catalog(state, "rename sheet", |catalog| {
        let revision = catalog
            .find(id)
            .map(|sheet| sheet.revision())
            .ok_or_else(|| NO_SHEET.to_owned())?;
        catalog
            .rename_sheet(id, revision, name.clone())
            .map_err(|error| error.to_string())
    })
    .map(|_| format!("Renamed sheet to `{name}`."));
    report(state, outcome)
}

/// What [`delete_sheet`] would do, under the catalog's own delete policy.
pub(in crate::workbench) fn plan_delete(state: &AppState, id: SheetId) -> SheetDeletePlan {
    let Some(catalog) = active_sheet_catalog(state) else {
        return SheetDeletePlan::LastSheet;
    };
    if catalog.sheets().len() < 2 {
        return SheetDeletePlan::LastSheet;
    }
    let objects = catalog.objects_on_sheet(id).count();
    if objects == 0 {
        return SheetDeletePlan::Empty;
    }
    match catalog.settings().delete_behavior {
        SheetDeleteBehavior::BlockWhileReferenced => SheetDeletePlan::Blocked { objects },
        SheetDeleteBehavior::MoveReferencesToReviewedReplacement => {
            match replacement_sheet(catalog, id) {
                Some(destination) => SheetDeletePlan::MovesObjects {
                    objects,
                    destination,
                    destination_name: catalog
                        .find(destination)
                        .map_or_else(String::new, |sheet| sheet.name().to_owned()),
                },
                None => SheetDeletePlan::Blocked { objects },
            }
        }
    }
}

pub(in crate::workbench) fn delete_sheet(
    state: &mut AppState,
    id: SheetId,
) -> Result<String, String> {
    let name = state_sheet_name(state, id);
    let outcome = edit_catalog(state, "delete sheet", |catalog| {
        let resolution = match catalog.settings().delete_behavior {
            SheetDeleteBehavior::BlockWhileReferenced => SheetDeleteResolution::BlockIfNotEmpty,
            SheetDeleteBehavior::MoveReferencesToReviewedReplacement => {
                match replacement_sheet(catalog, id) {
                    Some(destination) => SheetDeleteResolution::MoveObjectsTo(destination),
                    None => SheetDeleteResolution::BlockIfNotEmpty,
                }
            }
        };
        catalog
            .delete_sheet(catalog.revision(), id, resolution)
            .map_err(|error| error.to_string())
    })
    .map(|receipt| {
        if receipt.moved_object_ids.is_empty() {
            format!("Deleted sheet `{name}`.")
        } else {
            format!(
                "Deleted sheet `{name}` and moved {} object(s) to the adjacent sheet.",
                receipt.moved_object_ids.len()
            )
        }
    });
    let outcome = report(state, outcome);
    if outcome.is_ok() {
        // Deleting the active sheet moves activation inside the catalog's own
        // transaction, so the canvas is entering a sheet it never activated.
        enter_active_sheet(state);
    }
    outcome
}

pub(in crate::workbench) fn move_sheet(
    state: &mut AppState,
    id: SheetId,
    to_index: usize,
) -> Result<String, String> {
    let name = state_sheet_name(state, id);
    let outcome = edit_catalog(state, "reorder sheets", |catalog| {
        catalog
            .move_sheet(
                catalog.revision(),
                id,
                to_index,
                ReorderPageNumbering::UpdatePrintPageNumbers,
            )
            .map_err(|error| error.to_string())
    })
    .map(|_| format!("Moved sheet `{name}` to position {}.", to_index + 1));
    report(state, outcome)
}

/// Move the retained instance selection onto `destination`, creating the typed
/// off-sheet ports every boundary net now needs.
pub(in crate::workbench) fn move_selection_to_sheet(
    state: &mut AppState,
    destination: SheetId,
) -> Result<String, String> {
    let outcome = move_selection(state, destination);
    report(state, outcome)
}

/// The explicit boundary-port contract for one sheet move.
///
/// Both the strip's "Move selection here" and the design manager's reviewed
/// Move-selection page publish through this, so a boundary net receives the
/// same typed contract whichever surface asked for the move.
pub(in crate::workbench) fn boundary_resolution(
    plan: &SheetMoveConnectivityPlan,
    source: SheetId,
    destination: SheetId,
) -> MoveBoundaryResolution {
    let ports = plan
        .boundaries
        .iter()
        .map(|boundary| CrossSheetPortDefinition {
            net_name: boundary.net_name.clone(),
            first: CrossSheetPortEndpoint {
                sheet_id: source,
                anchor: CrossSheetPortAnchor::WirePoint {
                    wire_id: boundary.stationary_wire_id,
                    point: boundary.stationary_point,
                },
            },
            second: CrossSheetPortEndpoint {
                sheet_id: destination,
                anchor: CrossSheetPortAnchor::ComponentTerminal {
                    component_id: boundary.moved_component_id,
                    terminal_name: boundary.moved_terminal_name.clone(),
                },
            },
            direction: match boundary.direction {
                PortDirection::In => CrossSheetPortDirection::Input,
                PortDirection::Out => CrossSheetPortDirection::Output,
                PortDirection::InOut => CrossSheetPortDirection::InOut,
                PortDirection::Supply => CrossSheetPortDirection::Supply,
            },
            signal_type: match (boundary.direction, boundary.discipline) {
                (PortDirection::Supply, _) => CrossSheetSignalType::Power,
                (_, PortDiscipline::Logic) => CrossSheetSignalType::Logic,
                _ => CrossSheetSignalType::Analog,
            },
            discipline: match boundary.discipline {
                PortDiscipline::Electrical => CrossSheetDiscipline::Electrical,
                PortDiscipline::Logic => CrossSheetDiscipline::Logic,
                PortDiscipline::Wreal => CrossSheetDiscipline::Wreal,
                PortDiscipline::Thermal => CrossSheetDiscipline::Thermal,
            },
        })
        .collect::<Vec<_>>();
    if ports.is_empty() {
        MoveBoundaryResolution::VerifiedNoBoundaryNets
    } else {
        MoveBoundaryResolution::ExplicitPorts { ports }
    }
}

fn move_selection(state: &mut AppState, destination: SheetId) -> Result<String, String> {
    let connectivity =
        crate::workbench::app::dialogs::hierarchy::create::selected_component_sheet_move_plan(
            state,
        )?;
    let all_objects = live_object_ids(state);
    let source = active_sheet_catalog(state)
        .and_then(|catalog| {
            connectivity
                .source_component_ids
                .first()
                .and_then(|object_id| catalog.sheet_for_object(*object_id))
                .or_else(|| catalog.active_sheet_id())
        })
        .ok_or_else(|| NO_CATALOG.to_owned())?;
    if source == destination {
        return Err("The selection already lives on that sheet.".to_owned());
    }
    let resolution = boundary_resolution(&connectivity, source, destination);
    let moved = connectivity.moved_object_ids.clone();
    let receipt = edit_catalog(state, "move selection to sheet", move |catalog| {
        let active = catalog.active_sheet_id();
        catalog
            .reconcile_object_assignments(catalog.revision(), all_objects, active)
            .map_err(|error| error.to_string())?;
        catalog
            .move_selection(MoveSelectionRequest {
                expected_catalog_revision: catalog.revision(),
                object_ids: moved,
                destination_sheet_id: destination,
                boundary_resolution: resolution,
            })
            .map_err(|error| error.to_string())
    })?;
    Ok(format!(
        "Moved {} object(s) to `{}` with {} explicit boundary port(s).",
        receipt.object_ids.len(),
        state_sheet_name(state, destination),
        receipt.created_port_ids.len()
    ))
}

fn step(state: &mut AppState, delta: isize) -> Result<String, String> {
    let Some(catalog) = active_sheet_catalog(state) else {
        return report(state, Err(NO_CATALOG.to_owned()));
    };
    let count = catalog.sheets().len();
    if count < 2 {
        return report(
            state,
            Err("The active cell view has a single sheet.".to_owned()),
        );
    }
    let current = catalog
        .active_sheet_id()
        .and_then(|active| {
            catalog
                .sheets()
                .iter()
                .position(|sheet| sheet.id() == active)
        })
        .unwrap_or_default();
    // Wrapping is the surface's choice, not the catalog's: `next_sheet` stops
    // at the ends so the durable order stays a line rather than a ring.
    let index = (current as isize + delta).rem_euclid(count as isize) as usize;
    go_to_sheet(state, index)
}

fn activate(state: &mut AppState, id: SheetId, name: String) -> Result<String, String> {
    if active_sheet_id(state) == Some(id) {
        return Ok(format!("Sheet `{name}` is already active."));
    }
    // Objects authored on the sheet being left take their membership from the
    // sheet that is still active, so the buffer is synchronized first.
    state.sync_active_schematic_to_workspace();
    edit_catalog(state, "activate sheet", |catalog| {
        catalog.set_active(id).map_err(|error| error.to_string())
    })?;
    enter_active_sheet(state);
    Ok(format!("Sheet `{name}` is active."))
}

/// Retire everything that named the sheet just left and frame the new one.
fn enter_active_sheet(state: &mut AppState) {
    state.schematic.selection.clear();
    state.schematic.net_highlight.clear();
    state.schematic.needs_drawing_sheet_fit = true;
    state.schematic.needs_fit = false;
}

fn create(
    state: &mut AppState,
    insert_after: Option<SheetId>,
) -> Result<(SheetId, String), String> {
    let settings = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .clone();
    let Some(catalog) = active_sheet_catalog(state) else {
        return Err(NO_CATALOG.to_owned());
    };
    let name = unique_sheet_name(catalog);
    let insert_after = insert_after.or_else(|| catalog.active_sheet_id());
    let page_format = new_sheet_page_format(&settings, catalog)?;
    let page_number = first_free_page_number(catalog)?;
    let created = name.clone();
    let id = edit_catalog(state, "create sheet", move |catalog| {
        catalog
            .create_sheet_with_page_format(
                SheetDefinition {
                    name: created,
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(page_number),
                },
                insert_after,
                page_format,
            )
            .map_err(|error| error.to_string())
    })?;
    Ok((id, name))
}

/// Resolve the drawing sheet a newly created sheet is printed on, under the
/// project's own new-sheet policy. `Ask` has no reviewer here, so it uses the
/// remembered explicit format the design manager would have offered first.
fn new_sheet_page_format(
    settings: &crate::state::DrawingSheetProjectSettings,
    catalog: &SheetCatalog,
) -> Result<SchematicSheetFormat, String> {
    let current = catalog
        .active()
        .map(|sheet| {
            sheet
                .page_format()
                .try_update(|format| {
                    format.inheritance = DrawingSheetInheritance::Explicit;
                })
                .map(|format| format.without_project_owned_title_values())
                .map_err(|error| error.to_string())
        })
        .transpose()?;
    Ok(match settings.new_sheet_policy {
        DrawingSheetNewSheetPolicy::ProjectDefault => settings.default_format.clone(),
        DrawingSheetNewSheetPolicy::MatchCurrent => {
            current.unwrap_or_else(|| settings.default_format.clone())
        }
        DrawingSheetNewSheetPolicy::Ask => settings
            .remember_last_explicit_format
            .then(|| settings.last_explicit_format.clone())
            .flatten()
            .or(current)
            .unwrap_or_else(|| settings.default_format.clone()),
    })
}

fn first_free_page_number(catalog: &SheetCatalog) -> Result<u32, String> {
    let used = catalog
        .sheets()
        .iter()
        .filter_map(|sheet| sheet.definition().explicit_page_number)
        .collect::<std::collections::BTreeSet<_>>();
    (1..=u32::MAX)
        .find(|page| !used.contains(page))
        .ok_or_else(|| "No free sheet page number remains.".to_owned())
}

/// `Sheet N`, where `N` is the lowest ordinal the catalog has not claimed.
/// Sheet names are unique case-insensitively, so a generated name has to
/// avoid the ones already authored rather than count the sheets.
fn unique_sheet_name(catalog: &SheetCatalog) -> String {
    let mut ordinal = catalog.sheets().len() + 1;
    loop {
        let candidate = format!("Sheet {ordinal}");
        if !catalog
            .sheets()
            .iter()
            .any(|sheet| sheet.name().eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
        ordinal += 1;
    }
}

/// Where a deleted sheet's objects go: the sheet before it, or the one after
/// it when the sheet being removed is first.
fn replacement_sheet(catalog: &SheetCatalog, id: SheetId) -> Option<SheetId> {
    catalog
        .previous_sheet(id)
        .or_else(|| catalog.next_sheet(id))
}

fn state_sheet_name(state: &AppState, id: SheetId) -> String {
    active_sheet_catalog(state)
        .and_then(|catalog| catalog.find(id))
        .map_or_else(|| id.to_string(), |sheet| sheet.name().to_owned())
}

fn live_object_ids(state: &AppState) -> Vec<u64> {
    let schematic = &state.schematic;
    let mut ids = std::collections::BTreeSet::new();
    ids.extend(schematic.components.iter().map(|object| object.id));
    ids.extend(schematic.wires.iter().map(|object| object.id));
    ids.extend(schematic.buses.iter().map(|object| object.id));
    ids.extend(schematic.bus_taps.iter().map(|object| object.id));
    ids.extend(schematic.junctions.iter().map(|object| object.id));
    ids.extend(schematic.net_labels.iter().map(|object| object.id));
    ids.extend(schematic.design_notes.iter().map(|object| object.id));
    ids.extend(
        schematic
            .documentation_shapes
            .iter()
            .map(|object| object.id),
    );
    ids.extend(schematic.probes.iter().map(|object| object.id));
    ids.into_iter().collect()
}

/// Apply one edit to the active cell view's sheet catalog and publish the
/// result as a single project transaction, so undo restores the whole change.
fn edit_catalog<T>(
    state: &mut AppState,
    description: &str,
    edit: impl FnOnce(&mut SheetCatalog) -> Result<T, String>,
) -> Result<T, String> {
    let key = state.workspace.active_schematic_reference().key();
    let before = state.workspace.design_management.clone();
    let mut candidate = before.clone();
    let catalog = candidate
        .sheet_catalog_mut(&key)
        .ok_or_else(|| NO_CATALOG.to_owned())?;
    let outcome = edit(catalog)?;
    commit(state, description, before, candidate)?;
    Ok(outcome)
}

fn commit(
    state: &mut AppState,
    description: &str,
    before: DesignManagementCatalog,
    candidate: DesignManagementCatalog,
) -> Result<(), String> {
    candidate.validate().map_err(|error| error.to_string())?;
    let schematic_tx = state.prepare_design_management_schematic_transaction(&candidate)?;
    let owner = state.workspace.active_schematic_reference();
    let committed_revision = state
        .workspace
        .replace_design_management(candidate)
        .map_err(|error| error.to_string())?;
    state.apply_design_management_schematic_transaction(&schematic_tx);
    let after = state.workspace.design_management.clone();
    state.record_design_management_transaction(DesignManagementHistoryEntry {
        description: description.to_owned(),
        owner,
        before,
        after,
        before_schematics: schematic_tx.before,
        after_schematics: schematic_tx.after,
        committed_revision,
    });
    Ok(())
}

fn report(state: &mut AppState, outcome: Result<String, String>) -> Result<String, String> {
    match &outcome {
        Ok(message) => state.push_user_message(ConsoleMessage::info(message.clone())),
        Err(error) => state.push_user_message(ConsoleMessage::warning(error.clone())),
    }
    outcome
}

#[cfg(test)]
mod tests;
