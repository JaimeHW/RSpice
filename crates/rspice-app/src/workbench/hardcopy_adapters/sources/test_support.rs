//! Test-only resolution helpers for retained hardcopy source contracts.

use super::*;

#[cfg(test)]
pub(crate) fn resolve_retained_hardcopy_source(
    state: &AppState,
    source_key: &str,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let descriptors = enumerate_retained_hardcopy_sources(state);
    let descriptor = descriptors
        .iter()
        .find(|descriptor| descriptor.source_key == source_key)
        .ok_or_else(|| HardcopySourceError::SourceNotRetained(source_key.to_owned()))?;
    if let RetainedHardcopySourceAvailability::Unavailable { reason } = &descriptor.availability {
        return Err(HardcopySourceError::UnavailableRetainedSource {
            source_key: source_key.to_owned(),
            reason: reason.clone(),
        });
    }
    if !descriptor.supports_scope(&scope) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    if let Some(source_set) = state.workspace.hardcopy_source_set(source_key) {
        return resolve_retained_hardcopy_source_set(state, source_set);
    }

    let project_id = state.workspace.project.id();
    let design_key = format!(
        "project:{}:cell-view:{}",
        project_id.as_uuid(),
        state.workspace.active_key()
    );
    if matches!(
        state.workspace.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        let active_key = state.workspace.active_key();
        if let Some(catalog) = state.workspace.design_management.sheet_catalog(&active_key)
            && let Some(sheet) = catalog
                .sheets()
                .iter()
                .find(|sheet| format!("{design_key}:sheet:{}", sheet.id()) == source_key)
        {
            let base_identity = active_cell_view_identity(state)?;
            let resolver =
                SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers);
            let identity = schematic_sheet_identity(&base_identity, sheet)?;
            return resolve_schematic_source(SchematicHardcopySource {
                identity,
                schematic: &state.schematic,
                expected_topology_version: state.schematic.topology_version(),
                symbol_resolver: Some(&resolver),
                sheet_catalog: Some(catalog),
                sheet_id: Some(sheet.id()),
                project_default_drawing_sheet: Some(
                    &state
                        .workspace
                        .design_management
                        .drawing_sheet_settings()
                        .default_format,
                ),
                project_title_block_field_values: Some(
                    &state
                        .workspace
                        .design_management
                        .drawing_sheet_settings()
                        .title_block_field_values,
                ),
                scope,
            });
        }
    }
    if source_key == design_key {
        let identity = active_cell_view_identity(state)?;
        return match state.workspace.active_view_type() {
            ViewType::Schematic | ViewType::Testbench => {
                let resolver =
                    SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers);
                if matches!(scope, HardcopyScope::AllSheetsOrPanes) {
                    let active_key = state.workspace.active_key();
                    let sheet_catalog = state
                        .workspace
                        .design_management
                        .sheet_catalog(&active_key)
                        .ok_or_else(|| {
                            HardcopySourceError::InvalidSheetPartition(
                                "all-sheets scope has no governed sheet catalog".to_owned(),
                            )
                        })?;
                    return resolve_all_schematic_sheets(SchematicSheetSetHardcopySource {
                        identity,
                        schematic: &state.schematic,
                        expected_topology_version: state.schematic.topology_version(),
                        symbol_resolver: Some(&resolver),
                        sheet_catalog,
                        project_default_drawing_sheet: &state
                            .workspace
                            .design_management
                            .drawing_sheet_settings()
                            .default_format,
                        project_title_block_field_values: &state
                            .workspace
                            .design_management
                            .drawing_sheet_settings()
                            .title_block_field_values,
                    });
                }
                let active_key = state.workspace.active_key();
                let sheet_catalog = matches!(scope, HardcopyScope::CurrentSheet)
                    .then(|| state.workspace.design_management.sheet_catalog(&active_key))
                    .flatten();
                let sheet_id = sheet_catalog.and_then(SheetCatalog::active_sheet_id);
                let identity = if let (Some(catalog), Some(sheet_id)) = (sheet_catalog, sheet_id) {
                    let sheet = catalog.find(sheet_id).ok_or_else(|| {
                        HardcopySourceError::InvalidSheetPartition(format!(
                            "active sheet {sheet_id} is not retained"
                        ))
                    })?;

                    schematic_sheet_identity(&identity, sheet)?
                } else {
                    identity
                };
                resolve_schematic_source(SchematicHardcopySource {
                    identity,
                    schematic: &state.schematic,
                    expected_topology_version: state.schematic.topology_version(),
                    symbol_resolver: Some(&resolver),
                    sheet_catalog,
                    sheet_id,
                    project_default_drawing_sheet: matches!(
                        scope,
                        HardcopyScope::CurrentSheet | HardcopyScope::ActiveDocument
                    )
                    .then_some(
                        &state
                            .workspace
                            .design_management
                            .drawing_sheet_settings()
                            .default_format,
                    ),
                    project_title_block_field_values: Some(
                        &state
                            .workspace
                            .design_management
                            .drawing_sheet_settings()
                            .title_block_field_values,
                    ),
                    scope,
                })
            }
            ViewType::Symbol => {
                let document = state
                    .load_active_symbol_document()
                    .map_err(HardcopySourceError::StaleActiveDocumentAuthority)?;
                resolve_symbol_source(SymbolHardcopySource {
                    identity,
                    document: &document,
                    selection: None,
                    scope,
                })
            }
            view_type => Err(HardcopySourceError::UnsupportedDocument(format!(
                "active design view type {view_type:?} has no semantic hardcopy adapter"
            ))),
        };
    }

    if let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) =
        state.workbench.documents.active(Workspace::Results)
        && let Some((document, page, pane)) =
            active_visualization_document_pane(state, *document_id)
        && source_key == visualization_document_pane_source_key(project_id, document.id(), pane.id)
    {
        return resolve_visualization_document_source(
            source_key.to_owned(),
            project_id,
            document,
            page.id,
            pane.id,
            matches!(scope, HardcopyScope::AllSheetsOrPanes),
            scope,
        );
    }

    if let Some(run) = state.simulation.active_run() {
        let results_key = format!(
            "project:{}:result-dataset:{}",
            project_id.as_uuid(),
            run.dataset_id
        );
        if source_key == results_key {
            require_active_result_document(state, run.dataset_id)?;
            return resolve_results_quick_view_source(ResultsQuickViewHardcopySource {
                source_key: source_key.to_owned(),
                project_id,
                state,
                scope,
            });
        }
    }

    if let Some(pane) = state
        .workbench
        .visualization_studio
        .panes
        .iter()
        .find(|pane| {
            format!(
                "project:{}:visualization-pane:{}",
                project_id.as_uuid(),
                pane.id
            ) == source_key
        })
    {
        if matches!(scope, HardcopyScope::AllSheetsOrPanes) {
            let mut resolved = resolve_all_studio_panes(
                project_id,
                &state.workbench.visualization_studio,
                &state.simulation,
            )?;
            // The transient all-panes definition is addressed through the
            // selected retained pane descriptor, so commit-time revalidation
            // must keep that stable dialog key.
            resolved.source_key = source_key.to_owned();
            return Ok(resolved);
        }
        let pane_id = pane.id;
        return resolve_active_studio_pane_source(ActiveStudioPaneHardcopySource {
            source_key: source_key.to_owned(),
            project_id,
            studio: &state.workbench.visualization_studio,
            simulation: &state.simulation,
            pane_id,
            scope,
        });
    }

    if let Some(document_id) = state.workbench.report_authoring.selected_document {
        let report_key = format!("project:{}:report:{}", project_id.as_uuid(), document_id);
        if source_key == report_key {
            let document = state
                .workspace
                .report_documents
                .iter()
                .find(|document| document.id() == document_id)
                .ok_or_else(|| HardcopySourceError::SourceNotRetained(source_key.to_owned()))?;
            return report_inventory::resolve(state, document, scope);
        }
    }

    Err(HardcopySourceError::SourceNotRetained(
        source_key.to_owned(),
    ))
}

#[cfg(test)]
pub(crate) fn resolve_retained_hardcopy_source_set(
    state: &AppState,
    source_set: &HardcopySourceSet,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    resolve_hardcopy_source_set_with(source_set, |member| {
        resolve_retained_hardcopy_source(state, member.source_key(), member.scope().clone())
    })
}
