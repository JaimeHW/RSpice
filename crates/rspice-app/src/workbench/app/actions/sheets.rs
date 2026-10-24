//! Sheet navigation and sheet lifecycle for the active cell view.
//!
//! Sheets are project-owned identities, so every operation here publishes one
//! reviewed design-management transaction against the live project rather than
//! editing a dialog draft. That is what lets a strip chip, a chord, a palette
//! row and the design manager act on the same sheet without three of them
//! holding a private copy of the catalog.
//!
//! Activation, and the transaction each edit publishes, are the model's own
//! operations ([`Model::activate_sheet`], [`Model::edit_sheet_catalog`]),
//! since other features bring a sheet forward too; this module states their
//! outcome on the status bar.

use crate::state::{
    CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDefinition, CrossSheetPortDirection,
    CrossSheetPortEndpoint, CrossSheetSignalType, DrawingSheetInheritance,
    DrawingSheetNewSheetPolicy, MoveBoundaryResolution, MoveSelectionRequest, PortDirection,
    PortDiscipline, ReorderPageNumbering, SchematicSheetFormat, SheetCatalog, SheetDefinition,
    SheetDeleteBehavior, SheetDeleteResolution, SheetId, SheetMoveConnectivityPlan,
    SheetPortPolicy, SheetTemplate,
};
use crate::workbench::app::dialogs::delete_sheet::PendingSheetDelete;
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::state::WorkbenchSession;

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
    state.model.active_sheet_catalog()
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

/// Show the open sheet's properties: with nothing selected the inspector is
/// the sheet's, so the selection is cleared, the inspector is opened where it
/// is closed, and it is scrolled to its Sheet section. Only an editable sheet
/// is shown so; anywhere else nothing changes.
pub(crate) fn reveal_sheet_properties(state: &mut AppState) {
    if reveal_sheet_properties_on(&mut state.model, &mut state.workbench) {
        state.sheets.reveal_sheet_section = true;
    }
}

/// [`reveal_sheet_properties`] up to the inspector's own Sheet section,
/// answering whether the sheet was shown, so the caller asks that section to
/// come into view.
pub(crate) fn reveal_sheet_properties_on(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
) -> bool {
    if !crate::workbench::app::drawing_sheet_edit::drawing_sheet_editable(workbench, model) {
        return false;
    }
    model.schematic.session.editor.selection.clear();
    model.schematic.session.editor.net_highlight.clear();
    workbench.inspector_visible = true;
    workbench.drawer = Some(crate::workbench::state::Drawer::Inspector);
    true
}

pub(in crate::workbench) fn active_sheet_id(state: &AppState) -> Option<SheetId> {
    state.model.active_sheet_id()
}

pub(in crate::workbench) fn sheet_count(state: &AppState) -> usize {
    active_sheet_catalog(state).map_or(0, |catalog| catalog.sheets().len())
}

/// Make one sheet the canvas the session is editing: [`Model::activate_sheet`],
/// stated on the status bar.
pub(in crate::workbench) fn activate_sheet(
    state: &mut AppState,
    id: SheetId,
) -> Result<String, String> {
    let outcome = state
        .model
        .activate_sheet(id, state.workbench.interaction.drag.let_go());
    state.workbench.report(outcome)
}

pub(in crate::workbench) fn next_sheet(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
) -> Result<String, String> {
    let outcome = model.step_sheet(1, workbench.interaction.drag.let_go());
    workbench.report(outcome)
}

pub(in crate::workbench) fn previous_sheet(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
) -> Result<String, String> {
    let outcome = model.step_sheet(-1, workbench.interaction.drag.let_go());
    workbench.report(outcome)
}

/// [`Model::go_to_sheet`], stated on the status bar. Page navigation reaches
/// it through [`next_sheet`] and [`previous_sheet`].
#[cfg(test)]
fn go_to_sheet(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    index: usize,
) -> Result<String, String> {
    let outcome = model.go_to_sheet(index, workbench.interaction.drag.let_go());
    workbench.report(outcome)
}

/// Create a sheet after the active one and enter it.
pub(in crate::workbench) fn new_sheet(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
) -> Result<String, String> {
    let after = model.active_sheet_id();
    new_sheet_after(model, workbench, after)
}

/// Create a sheet directly after `after` and enter it.
pub(in crate::workbench) fn new_sheet_after(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    after: Option<SheetId>,
) -> Result<String, String> {
    if model.active_sheet_catalog().is_none() {
        let outcome = second_sheet(model, workbench).map(|name| format!("Created sheet `{name}`."));
        return workbench.report(outcome);
    }
    match create(model, after) {
        Ok((id, name)) => {
            let _ = model.activate_sheet(id, workbench.interaction.drag.let_go());
            workbench.report(Ok(format!("Created sheet `{name}`.")))
        }
        Err(error) => workbench.report(Err(error)),
    }
}

/// Let go of any gesture under way and write the open drawing back to the
/// project, before an edit of the catalog reads it.
fn settle_drawing(model: &mut Model, workbench: &mut WorkbenchSession) {
    workbench.interaction.drag.cancel_schematic_drag(model);
    if let Some(prompt) = model.sync_active_schematic_to_workspace() {
        workbench.prompt(prompt);
    }
}

pub(in crate::workbench) fn rename_sheet(
    state: &mut AppState,
    id: SheetId,
    name: String,
) -> Result<String, String> {
    let name = name.trim().to_owned();
    let outcome = state
        .model
        .edit_sheet_catalog("rename sheet", |catalog| {
            let revision = catalog
                .find(id)
                .map(|sheet| sheet.revision())
                .ok_or_else(|| Model::NO_SUCH_SHEET.to_owned())?;
            catalog
                .rename_sheet(id, revision, name.clone())
                .map_err(|error| error.to_string())
        })
        .map(|_| format!("Renamed sheet to `{name}`."));
    state.workbench.report(outcome)
}

/// What [`delete_sheet`] would do, under the catalog's own delete policy.
pub(in crate::workbench) fn plan_delete(model: &Model, id: SheetId) -> SheetDeletePlan {
    let Some(catalog) = model.active_sheet_catalog() else {
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

/// Delete a sheet from a menu or a chord, which state no outcome beforehand.
///
/// The outcome is resolved first. A delete that would move objects onto the
/// neighbouring sheet waits for a confirmation naming how many and where;
/// any other outcome (an empty sheet, a refusal) is run, or reported, now.
pub(in crate::workbench) fn delete_sheet(
    pending: &mut Option<PendingSheetDelete>,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    id: SheetId,
) -> Result<String, String> {
    if let SheetDeletePlan::MovesObjects {
        objects,
        destination,
        destination_name,
    } = plan_delete(model, id)
    {
        let name = model.sheet_name(id);
        let message = format!("Deleting sheet `{name}` waits for confirmation.");
        *pending = Some(PendingSheetDelete {
            sheet: id,
            name,
            objects,
            destination,
            destination_name,
        });
        return Ok(message);
    }
    commit_delete_sheet(model, workbench, id)
}

/// Delete a sheet now, under the catalog's own delete policy.
///
/// For a caller that has already shown the outcome (the sheet strip's menu
/// row names it) or had it confirmed.
pub(in crate::workbench) fn commit_delete_sheet(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    id: SheetId,
) -> Result<String, String> {
    let name = model.sheet_name(id);
    let outcome = model
        .edit_sheet_catalog("delete sheet", |catalog| {
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
    let outcome = workbench.report(outcome);
    if outcome.is_ok() {
        // Deleting the active sheet moves activation inside the catalog's own
        // transaction, so the canvas is entering a sheet it never activated.
        model.enter_active_sheet();
    }
    outcome
}

pub(in crate::workbench) fn move_sheet(
    state: &mut AppState,
    id: SheetId,
    to_index: usize,
) -> Result<String, String> {
    let name = state.model.sheet_name(id);
    let outcome = state
        .model
        .edit_sheet_catalog("reorder sheets", |catalog| {
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
    state.workbench.report(outcome)
}

/// Move the retained instance selection onto `destination`, creating the typed
/// off-sheet ports every boundary net now needs.
pub(in crate::workbench) fn move_selection_to_sheet(
    state: &mut AppState,
    destination: SheetId,
) -> Result<String, String> {
    let outcome = move_selection(state, destination);
    state.workbench.report(outcome)
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
    let all_objects = live_object_ids(&state.model);
    let source = active_sheet_catalog(state)
        .and_then(|catalog| {
            connectivity
                .source_component_ids
                .first()
                .and_then(|object_id| catalog.sheet_for_object(*object_id))
                .or_else(|| catalog.active_sheet_id())
        })
        .ok_or_else(|| Model::NO_SHEET_CATALOG.to_owned())?;
    if source == destination {
        return Err("The selection already lives on that sheet.".to_owned());
    }
    let resolution = boundary_resolution(&connectivity, source, destination);
    let moved = connectivity.moved_object_ids.clone();
    let receipt = state
        .model
        .edit_sheet_catalog("move selection to sheet", move |catalog| {
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
        state.model.sheet_name(destination),
        receipt.created_port_ids.len()
    ))
}

/// Give a drawing with no sheet catalog its catalog: everything already
/// drawn becomes Sheet 1, as one undoable transaction. For a sheet workflow
/// used for the first time (Sheet formats…); a catalog is never created on
/// load. Does nothing where the catalog exists.
pub(in crate::workbench) fn ensure_sheet_catalog(
    model: &mut Model,
    workbench: &mut WorkbenchSession,
) -> Result<(), String> {
    if model.active_sheet_catalog().is_some() {
        return Ok(());
    }
    settle_drawing(model, workbench);
    let key = model.workspace.content.active_schematic_reference().key();
    let objects = live_object_ids(model);
    let before = (*model.workspace.content.design_management).clone();
    let mut candidate = before.clone();
    candidate
        .bootstrap_for_cell_view(&key, "Sheet 1", objects)
        .map_err(|error| error.to_string())?;
    model.commit_sheet_catalog("create sheet catalog", before, candidate)
}

/// A drawing with no sheet catalog is one sheet. Its first new sheet creates
/// the catalog with everything already drawn as Sheet 1, adds Sheet 2 after
/// it and enters it, as one transaction, so one undo gives back the single
/// drawing.
fn second_sheet(model: &mut Model, workbench: &mut WorkbenchSession) -> Result<String, String> {
    settle_drawing(model, workbench);
    let key = model.workspace.content.active_schematic_reference().key();
    let objects = live_object_ids(model);
    let before = (*model.workspace.content.design_management).clone();
    let mut candidate = before.clone();
    let first = candidate
        .bootstrap_for_cell_view(&key, "Sheet 1", objects)
        .map_err(|error| error.to_string())?;
    let settings = candidate.drawing_sheet_settings().clone();
    let catalog = candidate
        .sheet_catalog_mut(&key)
        .ok_or_else(|| Model::NO_SHEET_CATALOG.to_owned())?;
    let name = unique_sheet_name(catalog);
    let page_format = new_sheet_page_format(&settings, catalog)?;
    let page_number = first_free_page_number(catalog)?;
    let second = catalog
        .create_sheet_with_page_format(
            SheetDefinition {
                name: name.clone(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(page_number),
            },
            Some(first),
            page_format,
        )
        .map_err(|error| error.to_string())?;
    catalog
        .set_active(second)
        .map_err(|error| error.to_string())?;
    model.commit_sheet_catalog("create sheet", before, candidate)?;
    model.enter_active_sheet();
    Ok(name)
}

fn create(model: &mut Model, insert_after: Option<SheetId>) -> Result<(SheetId, String), String> {
    let settings = model
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .clone();
    let Some(catalog) = model.active_sheet_catalog() else {
        return Err(Model::NO_SHEET_CATALOG.to_owned());
    };
    let name = unique_sheet_name(catalog);
    let insert_after = insert_after.or_else(|| catalog.active_sheet_id());
    let page_format = new_sheet_page_format(&settings, catalog)?;
    let page_number = first_free_page_number(catalog)?;
    let created = name.clone();
    let id = model.edit_sheet_catalog("create sheet", move |catalog| {
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

fn live_object_ids(model: &Model) -> Vec<u64> {
    let schematic = &model.schematic;
    let mut ids = std::collections::BTreeSet::new();
    ids.extend(
        schematic
            .document()
            .components
            .iter()
            .map(|object| object.id),
    );
    ids.extend(schematic.document().wires.iter().map(|object| object.id));
    ids.extend(schematic.document().buses.iter().map(|object| object.id));
    ids.extend(schematic.document().bus_taps.iter().map(|object| object.id));
    ids.extend(
        schematic
            .document()
            .junctions
            .iter()
            .map(|object| object.id),
    );
    ids.extend(
        schematic
            .document()
            .net_labels
            .iter()
            .map(|object| object.id),
    );
    ids.extend(
        schematic
            .document()
            .design_notes
            .iter()
            .map(|object| object.id),
    );
    ids.extend(
        schematic
            .document()
            .documentation_shapes
            .iter()
            .map(|object| object.id),
    );
    ids.extend(schematic.document().probes.iter().map(|object| object.id));
    ids.into_iter().collect()
}

#[cfg(test)]
mod tests;
