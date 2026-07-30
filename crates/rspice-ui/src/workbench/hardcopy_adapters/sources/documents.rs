//! Resolving one document kind to a printable source.
//!
//! Schematics, symbols, plots, and reports each become a semantic page a
//! different way, but all four obey the same rule: the page is built from the
//! saved document, not from what is currently on screen, and a document that
//! cannot supply the content is refused rather than printed blank. A blank
//! sheet is only ever produced where the schematic genuinely has one.

use std::collections::BTreeMap;

use crate::state::DrawingSheetTitleFieldId;

use super::*;

pub(super) fn resolve_blank_schematic_sheet(
    identity: HardcopySourceIdentity,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    resolve_blank_schematic_sheet_with_format(identity, scope, None)
}

pub(crate) fn resolve_blank_schematic_sheet_with_format(
    identity: HardcopySourceIdentity,
    scope: HardcopyScope,
    drawing_sheet: Option<&SchematicSheetFormat>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    resolve_blank_schematic_sheet_with_format_and_project_values(
        identity,
        scope,
        drawing_sheet,
        None,
    )
}

pub(super) fn resolve_blank_schematic_sheet_with_format_and_project_values(
    identity: HardcopySourceIdentity,
    scope: HardcopyScope,
    drawing_sheet: Option<&SchematicSheetFormat>,
    project_title_block_field_values: Option<&BTreeMap<DrawingSheetTitleFieldId, String>>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if let Some(project_values) = project_title_block_field_values {
        crate::state::validate_project_drawing_sheet_title_field_values(project_values)
            .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    }
    if !matches!(scope, HardcopyScope::CurrentSheet) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    let semantic = SemanticSchematic {
        view_path: identity.source_key.clone(),
        drawing_sheet: drawing_sheet.cloned(),
        drawing_sheet_title_values: drawing_sheet.map_or_else(BTreeMap::new, |format| {
            drawing_sheet_title_values(&identity, format, None, project_title_block_field_values)
        }),
        grid_pitch_units: 10,
        components: Vec::new(),
        wires: Vec::new(),
        buses: Vec::new(),
        bus_taps: Vec::new(),
        junctions: Vec::new(),
        net_labels: Vec::new(),
        design_notes: Vec::new(),
        documentation_shapes: Vec::new(),
    };
    let digest = canonical_digest(b"rspice-hardcopy-blank-schematic-sheet-v1", &semantic)?;
    let bounds = drawing_sheet.map_or_else(
        || {
            SemanticBounds::try_new(
                SemanticPoint::new(0, 0),
                SemanticPoint::new(
                    BLANK_SCHEMATIC_SHEET_WIDTH_UM,
                    BLANK_SCHEMATIC_SHEET_HEIGHT_UM,
                ),
            )
        },
        authored_sheet_bounds,
    )?;
    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        scope,
        HardcopySemanticDocument::Schematic(semantic),
        bounds,
    )
}

/// Resolve every governed schematic sheet in exact catalog order. Each sheet
/// is filtered independently before its authority is pinned into the
/// aggregate, so an assignment can never leak into a neighboring page.
pub fn resolve_all_schematic_sheets(
    source: SchematicSheetSetHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_project_default_drawing_sheet(source.project_default_drawing_sheet)?;
    crate::state::validate_project_drawing_sheet_title_field_values(
        source.project_title_block_field_values,
    )
    .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    source
        .sheet_catalog
        .validate()
        .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    if source.sheet_catalog.sheets().is_empty() {
        return Err(HardcopySourceError::InvalidSourceSet(
            "all-sheets scope requires at least one governed sheet".to_owned(),
        ));
    }
    if source.schematic.topology_version() != source.expected_topology_version {
        return Err(HardcopySourceError::StaleSchematic {
            expected: source.expected_topology_version,
            actual: source.schematic.topology_version(),
        });
    }

    let aggregate_source_key = source.identity.source_key.clone();
    let mut resolved_sheets = Vec::with_capacity(source.sheet_catalog.sheets().len());
    for sheet in source.sheet_catalog.sheets() {
        let sheet_identity = schematic_sheet_identity(&source.identity, sheet)?;
        if schematic_has_objects_on_sheet(source.schematic, source.sheet_catalog, sheet.id()) {
            resolved_sheets.push(resolve_schematic_source(SchematicHardcopySource {
                identity: sheet_identity,
                schematic: source.schematic,
                expected_topology_version: source.expected_topology_version,
                symbol_resolver: source.symbol_resolver,
                sheet_catalog: Some(source.sheet_catalog),
                sheet_id: Some(sheet.id()),
                project_default_drawing_sheet: Some(source.project_default_drawing_sheet),
                project_title_block_field_values: Some(source.project_title_block_field_values),
                scope: HardcopyScope::CurrentSheet,
            })?);
        } else {
            let format = effective_governed_sheet_format(
                sheet.page_format(),
                source.project_default_drawing_sheet,
            );
            resolved_sheets.push(
                resolve_blank_schematic_sheet_with_format_and_project_values(
                    sheet_identity,
                    HardcopyScope::CurrentSheet,
                    Some(&format),
                    Some(source.project_title_block_field_values),
                )?,
            );
        }
    }

    let members = resolved_sheets
        .iter()
        .map(HardcopySourceSetMember::from_resolved)
        .collect::<Result<Vec<_>, _>>()?;
    let mut set_identity_material = b"rspice-hardcopy-all-schematic-sheets-v1:".to_vec();
    set_identity_material.extend_from_slice(source.identity.document_id.as_uuid().as_bytes());
    let source_set = HardcopySourceSet::try_new(
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(
            &source.identity.document_id.as_uuid(),
            &set_identity_material,
        ))
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::new(source.sheet_catalog.revision())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        compact_display(
            &format!("{} · All sheets", source.identity.display_name),
            "All schematic sheets",
        ),
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::AllSheetsOrPanes,
        members,
    )?;
    let mut resolved_sheets = resolved_sheets.into_iter();
    let mut resolved = resolve_hardcopy_source_set_with(&source_set, |expected| {
        let actual = resolved_sheets.next().ok_or_else(|| {
            HardcopySourceError::SourceNotRetained(expected.source_key().to_owned())
        })?;
        if actual.source_key() != expected.source_key() {
            return Err(HardcopySourceError::StaleSourceSetMember {
                source_key: expected.source_key().to_owned(),
            });
        }
        Ok(actual)
    })?;
    // `AllSheetsOrPanes` is a transient scope of the owning design
    // descriptor, not a separately persisted named set.
    resolved.source_key = aggregate_source_key;
    Ok(resolved)
}

pub fn resolve_schematic_source(
    source: SchematicHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if let Some(project_default) = source.project_default_drawing_sheet {
        validate_project_default_drawing_sheet(project_default)?;
    }
    if let Some(project_values) = source.project_title_block_field_values {
        crate::state::validate_project_drawing_sheet_title_field_values(project_values)
            .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    }
    if source.schematic.topology_version() != source.expected_topology_version {
        return Err(HardcopySourceError::StaleSchematic {
            expected: source.expected_topology_version,
            actual: source.schematic.topology_version(),
        });
    }
    if !matches!(
        &source.scope,
        HardcopyScope::Selection | HardcopyScope::CurrentSheet | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    let selection_only = matches!(&source.scope, HardcopyScope::Selection);
    if selection_only && source.schematic.selection.is_empty() {
        return Err(HardcopySourceError::EmptySelection);
    }
    if selection_only && !source.schematic.selection.probes.is_empty() {
        return Err(HardcopySourceError::ProbeSelectionUnsupported);
    }
    let governed_sheet = match (source.sheet_catalog, source.sheet_id) {
        (Some(catalog), Some(sheet_id)) if matches!(source.scope, HardcopyScope::CurrentSheet) => {
            catalog
                .validate()
                .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
            if catalog.find(sheet_id).is_none() {
                return Err(HardcopySourceError::InvalidSheetPartition(format!(
                    "sheet {sheet_id} is not retained in the supplied catalog"
                )));
            }
            Some((catalog, sheet_id))
        }
        (None, None) => None,
        _ => {
            return Err(HardcopySourceError::InvalidSheetPartition(
                "sheet catalog and sheet identity must be supplied together only for CurrentSheet"
                    .to_owned(),
            ));
        }
    };
    let object_is_in_scope = |object_id: u64| {
        governed_sheet.is_none_or(|(catalog, sheet_id)| {
            // The canvas's existing governed-sheet contract assigns legacy
            // unassigned objects to the active sheet. Reusing it here makes
            // every object publish on exactly one sheet without leakage.
            catalog
                .sheet_for_object(object_id)
                .or(catalog.active_sheet_id())
                == Some(sheet_id)
        })
    };

    let drawing_sheet = governed_sheet
        .and_then(|(catalog, sheet_id)| catalog.find(sheet_id))
        .map(|sheet| {
            source.project_default_drawing_sheet.map_or_else(
                || sheet.page_format().clone(),
                |project_default| {
                    effective_governed_sheet_format(sheet.page_format(), project_default)
                },
            )
        })
        .or_else(|| {
            matches!(
                source.scope,
                HardcopyScope::CurrentSheet | HardcopyScope::ActiveDocument
            )
            .then(|| source.project_default_drawing_sheet.cloned())
            .flatten()
        });
    let mut semantic = SemanticSchematic {
        view_path: governed_sheet.map_or_else(
            || source.identity.source_key.clone(),
            |(_, sheet_id)| format!("{}:sheet:{sheet_id}", source.identity.source_key),
        ),
        drawing_sheet_title_values: drawing_sheet.as_ref().map_or_else(BTreeMap::new, |format| {
            drawing_sheet_title_values(
                &source.identity,
                format,
                governed_sheet,
                source.project_title_block_field_values,
            )
        }),
        drawing_sheet,
        grid_pitch_units: source.schematic.grid_size.max(1),
        components: Vec::new(),
        wires: Vec::new(),
        buses: Vec::new(),
        bus_taps: Vec::new(),
        junctions: Vec::new(),
        net_labels: Vec::new(),
        design_notes: Vec::new(),
        documentation_shapes: Vec::new(),
    };
    for component in &source.schematic.components {
        if !object_is_in_scope(component.id) {
            continue;
        }
        if selection_only
            && !source
                .schematic
                .selection
                .components
                .contains(&component.id)
        {
            continue;
        }
        let (resolved_symbol, symbol_source) =
            resolve_component_symbol(component, source.symbol_resolver)?;
        semantic.components.push(SemanticComponent {
            component: component.clone(),
            resolved_symbol,
            symbol_source,
        });
    }
    let selected_wire_ids = || {
        source
            .schematic
            .selection
            .wires
            .iter()
            .copied()
            .chain(
                source
                    .schematic
                    .selection
                    .wire_segments
                    .iter()
                    .map(|selection| selection.wire_id),
            )
            .chain(
                source
                    .schematic
                    .selection
                    .wire_vertices
                    .iter()
                    .map(|selection| selection.wire_id),
            )
            .collect::<std::collections::HashSet<_>>()
    };
    let selected_wire_ids = selection_only.then(selected_wire_ids);
    semantic.wires.extend(
        source
            .schematic
            .wires
            .iter()
            .filter(|wire| {
                object_is_in_scope(wire.id)
                    && selected_wire_ids
                        .as_ref()
                        .is_none_or(|selected| selected.contains(&wire.id))
            })
            .cloned(),
    );
    semantic.buses.extend(
        source
            .schematic
            .buses
            .iter()
            .filter(|bus| {
                object_is_in_scope(bus.id)
                    && (!selection_only || source.schematic.selection.buses.contains(&bus.id))
            })
            .cloned(),
    );
    semantic.bus_taps.extend(
        source
            .schematic
            .bus_taps
            .iter()
            .filter(|tap| {
                object_is_in_scope(tap.id)
                    && (!selection_only || source.schematic.selection.bus_taps.contains(&tap.id))
            })
            .cloned(),
    );
    semantic.junctions.extend(
        source
            .schematic
            .junctions
            .iter()
            .filter(|junction| {
                object_is_in_scope(junction.id)
                    && (!selection_only
                        || source
                            .schematic
                            .selection
                            .junctions
                            .iter()
                            .any(|selection| selection.pos == junction.pos))
            })
            .copied(),
    );
    semantic.net_labels.extend(
        source
            .schematic
            .net_labels
            .iter()
            .filter(|label| {
                object_is_in_scope(label.id)
                    && (!selection_only
                        || source.schematic.selection.net_labels.contains(&label.id))
            })
            .cloned(),
    );
    semantic.design_notes.extend(
        source
            .schematic
            .design_notes
            .iter()
            .filter(|note| {
                object_is_in_scope(note.id)
                    && (!selection_only
                        || source.schematic.selection.design_notes.contains(&note.id))
            })
            .cloned(),
    );
    semantic.documentation_shapes.extend(
        source
            .schematic
            .documentation_shapes
            .iter()
            .filter(|shape| {
                object_is_in_scope(shape.id)
                    && (!selection_only
                        || source
                            .schematic
                            .selection
                            .documentation_shapes
                            .contains(&shape.id))
            })
            .cloned(),
    );
    let content_empty = semantic_is_empty(&semantic);
    if content_empty && (selection_only || semantic.drawing_sheet.is_none()) {
        return Err(HardcopySourceError::EmptyContent);
    }

    let bounds = if content_empty {
        authored_sheet_bounds(
            semantic
                .drawing_sheet
                .as_ref()
                .expect("empty authored sheet was validated above"),
        )?
    } else {
        let content_bounds = schematic_bounds(&semantic)?;
        semantic
            .drawing_sheet
            .as_ref()
            .map_or(Ok(content_bounds), |format| {
                authored_sheet_bounds(format)
                    .map(|sheet_bounds| union_bounds(content_bounds, sheet_bounds))
            })?
    };
    let digest = canonical_digest(b"rspice-hardcopy-schematic-v1", &semantic)?;
    finish_resolved(
        source.identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        source.scope,
        HardcopySemanticDocument::Schematic(semantic),
        bounds,
    )
}

fn effective_governed_sheet_format(
    sheet_format: &SchematicSheetFormat,
    project_default: &SchematicSheetFormat,
) -> SchematicSheetFormat {
    match sheet_format.inheritance {
        crate::state::DrawingSheetInheritance::ProjectDefault => {
            project_default.with_target_sheet_title_fields(sheet_format)
        }
        crate::state::DrawingSheetInheritance::Explicit
        | crate::state::DrawingSheetInheritance::UserDefault => sheet_format.clone(),
    }
}

fn validate_project_default_drawing_sheet(
    format: &SchematicSheetFormat,
) -> Result<(), HardcopySourceError> {
    format
        .validate()
        .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    if format.inheritance != crate::state::DrawingSheetInheritance::ProjectDefault {
        return Err(HardcopySourceError::InvalidSheetPartition(
            "project drawing-sheet default has non-default inheritance".to_owned(),
        ));
    }
    Ok(())
}

fn drawing_sheet_title_values(
    identity: &HardcopySourceIdentity,
    format: &SchematicSheetFormat,
    governed: Option<(&SheetCatalog, SheetId)>,
    project_title_block_field_values: Option<&BTreeMap<DrawingSheetTitleFieldId, String>>,
) -> BTreeMap<DrawingSheetTitleFieldId, String> {
    let mut values = BTreeMap::new();
    let source_view = identity
        .source_key
        .split(":sheet:")
        .next()
        .unwrap_or(identity.source_key.as_str());
    let project = source_view
        .split(['/', '\\', ':'])
        .find(|part| !part.trim().is_empty())
        .unwrap_or("Project");
    let (sheet_title, page) = governed.map_or_else(
        || (identity.display_name.clone(), "1 / 1".to_owned()),
        |(catalog, sheet_id)| {
            let index = catalog
                .sheets()
                .iter()
                .position(|sheet| sheet.id() == sheet_id)
                .unwrap_or(0);
            let title = catalog.find(sheet_id).map_or_else(
                || identity.display_name.clone(),
                |sheet| sheet.name().to_owned(),
            );
            (title, format!("{} / {}", index + 1, catalog.sheets().len()))
        },
    );
    values.insert(DrawingSheetTitleFieldId::Project, project.to_owned());
    values.insert(DrawingSheetTitleFieldId::CellView, source_view.to_owned());
    values.insert(DrawingSheetTitleFieldId::SheetTitle, sheet_title);
    values.insert(DrawingSheetTitleFieldId::Page, page);
    values.insert(
        DrawingSheetTitleFieldId::Revision,
        identity.revision.get().to_string(),
    );
    values.insert(
        DrawingSheetTitleFieldId::Format,
        format.authored_size.label().to_owned(),
    );
    values.insert(
        DrawingSheetTitleFieldId::Scale,
        match format.title_block.scale {
            crate::state::DrawingSheetScale::NotToScale => "NTS".to_owned(),
            crate::state::DrawingSheetScale::Ratio {
                drawing_units,
                reality_units,
            } => format!("{drawing_units}:{reality_units}"),
        },
    );
    values.insert(
        DrawingSheetTitleFieldId::Date,
        crate::state::automatic_drawing_sheet_date_utc(),
    );
    if let Some(project_values) = project_title_block_field_values {
        for id in DrawingSheetTitleFieldId::PROJECT_OWNED {
            if let Some(value) = project_values.get(&id) {
                values.insert(id, value.clone());
            }
        }
    }
    values
}

pub fn resolve_symbol_source(
    source: SymbolHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if !matches!(
        &source.scope,
        HardcopyScope::Selection | HardcopyScope::CurrentSheet | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    let document = if matches!(&source.scope, HardcopyScope::Selection) {
        let selection = source
            .selection
            .ok_or(HardcopySourceError::EmptySelection)?;
        selected_symbol_document(source.document, selection)?
    } else {
        source.document.clone()
    };
    let bounds = symbol_bounds(&document)?;
    let digest = canonical_digest(b"rspice-hardcopy-symbol-v1", &document)?;
    finish_resolved(
        source.identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        source.scope,
        HardcopySemanticDocument::Symbol(document),
        bounds,
    )
}

pub fn resolve_plot_source(
    source: PlotHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source.source_key, SOURCE_KEY_LIMIT)?;
    validate_label("display name", &source.display_name, DISPLAY_NAME_LIMIT)?;
    if !matches!(
        &source.scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    if source.scene.traces().is_empty() {
        return Err(HardcopySourceError::UnretainedResult(
            "the active pane has no retained visible trace samples".to_owned(),
        ));
    }
    let plot_width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let x_range = source.scene.x_range();
    let y_range = source.scene.y_range();
    let x_span = x_range.maximum - x_range.minimum;
    let y_span = y_range.maximum - y_range.minimum;
    if !x_span.is_finite() || !y_span.is_finite() || x_span <= 0.0 || y_span <= 0.0 {
        return Err(HardcopySourceError::InvalidResultRange);
    }
    let mut traces = Vec::with_capacity(source.scene.traces().len());
    for trace in source.scene.traces() {
        if trace.points().is_empty() {
            return Err(HardcopySourceError::UnretainedResult(format!(
                "visible trace `{}` has no retained samples",
                trace.label()
            )));
        }
        let source_points = trace
            .points()
            .iter()
            .map(|point| (point.x(), point.y()))
            .collect::<Vec<_>>();
        let paths = clipped_plot_paths(
            &source_points,
            x_range.minimum,
            x_range.maximum,
            y_range.minimum,
            y_range.maximum,
            plot_width,
            plot_height,
        )?;
        traces.push(SemanticPlotTrace {
            trace_id: trace.trace_id().get(),
            label: trace.label().to_owned(),
            paths,
            source_samples: source_points
                .iter()
                .map(|(x, y)| (x.to_bits(), y.to_bits()))
                .collect(),
        });
    }
    let semantic = SemanticPlot {
        viewer: ResultViewer::Waves,
        page_id: source.scene.page_id().get(),
        pane_id: source.scene.pane_id().get(),
        traces,
        markers: Vec::new(),
        annotations: Vec::new(),
    };
    let identity = HardcopySourceIdentity::try_new(
        source.source_key,
        HardcopyDocumentId::try_from_uuid(source.scene.document_id().as_uuid())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        source.scene.revision(),
        source.display_name,
    )?;
    finish_resolved(
        identity,
        source.scene.source_digest(),
        HardcopyDocumentKind::PlotOrWorksheet,
        source.scope,
        HardcopySemanticDocument::Plot(semantic),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(PLOT_WIDTH_UM, PLOT_HEIGHT_UM),
        )?,
    )
}

pub(super) fn validate_frozen_report_png(
    block_id: ReportBlockId,
    artifact: &FrozenReportArtifact,
) -> Result<(u32, u32), HardcopySourceError> {
    if artifact.media_type() != "image/png" {
        return Err(HardcopySourceError::UnsupportedAuthenticatedReportBlock {
            block_id,
            kind: "frozen plot figure",
            reason: format!(
                "unsupported artifact media type `{}`; expected image/png",
                artifact.media_type()
            ),
        });
    }
    let computed = ContentDigest::from_bytes(Sha256::digest(artifact.payload()).into());
    if computed != artifact.content_digest() {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} artifact digest does not authenticate its payload"
        )));
    }
    if artifact.payload().len() > crate::results::visualization_raster::MAX_RASTER_ARTIFACT_BYTES {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} exceeds the PNG artifact byte limit"
        )));
    }
    if !png_has_exact_terminal_iend(artifact.payload()) {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} has malformed chunks or bytes after IEND"
        )));
    }

    let decoder = png::Decoder::new(std::io::Cursor::new(artifact.payload()));
    let mut reader = decoder.read_info().map_err(|error| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} is not a valid PNG: {error}"
        ))
    })?;
    let header = reader.info();
    let width = header.width;
    let height = header.height;
    if !(crate::results::visualization_raster::MIN_RASTER_DIMENSION
        ..=crate::results::visualization_raster::MAX_RASTER_DIMENSION)
        .contains(&width)
        || !(crate::results::visualization_raster::MIN_RASTER_DIMENSION
            ..=crate::results::visualization_raster::MAX_RASTER_DIMENSION)
            .contains(&height)
    {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} dimensions {width}x{height} are outside the governed raster bounds"
        )));
    }
    let pixels = usize::try_from(width)
        .ok()
        .and_then(|width| {
            usize::try_from(height)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .ok_or_else(|| {
            HardcopySourceError::InvalidReportSource(format!(
                "frozen plot block {block_id} dimensions overflow"
            ))
        })?;
    if pixels > crate::results::visualization_raster::MAX_RASTER_PIXELS {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} exceeds the governed pixel limit"
        )));
    }
    if header.color_type != png::ColorType::Rgb
        || header.bit_depth != png::BitDepth::Eight
        || header.trns.is_some()
        || header.animation_control.is_some()
    {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} must be a single-frame opaque RGB8 PNG"
        )));
    }
    let expected_bytes = pixels.checked_mul(3).ok_or_else(|| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded byte count overflowed"
        ))
    })?;
    let output_size = reader.output_buffer_size().ok_or_else(|| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} has no bounded decoded size"
        ))
    })?;
    if output_size != expected_bytes {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded byte count does not match RGB8 dimensions"
        )));
    }
    let mut decoded = Vec::new();
    decoded.try_reserve_exact(output_size).map_err(|_| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded buffer allocation failed"
        ))
    })?;
    decoded.resize(output_size, 0);
    let frame = reader.next_frame(&mut decoded).map_err(|error| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} PNG payload failed full decode: {error}"
        ))
    })?;
    if frame.width != width
        || frame.height != height
        || frame.color_type != png::ColorType::Rgb
        || frame.bit_depth != png::BitDepth::Eight
        || frame.buffer_size() != expected_bytes
    {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded frame contradicts its authenticated header"
        )));
    }
    Ok((width, height))
}

pub(super) fn png_has_exact_terminal_iend(payload: &[u8]) -> bool {
    const SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if payload.len() < SIGNATURE.len() || &payload[..8] != SIGNATURE {
        return false;
    }
    let mut offset = 8usize;
    let mut saw_ihdr = false;
    while offset < payload.len() {
        let Some(header_end) = offset.checked_add(8) else {
            return false;
        };
        if header_end > payload.len() {
            return false;
        }
        let length = u32::from_be_bytes(
            payload[offset..offset + 4]
                .try_into()
                .expect("four-byte PNG length"),
        ) as usize;
        let chunk_type = &payload[offset + 4..offset + 8];
        if !saw_ihdr {
            if chunk_type != b"IHDR" {
                return false;
            }
            saw_ihdr = true;
        }
        let Some(chunk_end) = header_end
            .checked_add(length)
            .and_then(|end| end.checked_add(4))
        else {
            return false;
        };
        if chunk_end > payload.len() {
            return false;
        }
        if chunk_type == b"IEND" {
            return length == 0 && chunk_end == payload.len();
        }
        offset = chunk_end;
    }
    false
}

pub fn resolve_report_source(
    source: ReportHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source.source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &source.scope,
        HardcopyScope::CompleteReport | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    source
        .document
        .validate()
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
    let record = source
        .document
        .revision_record(source.document.id(), source.document.revision())
        .ok_or_else(|| HardcopySourceError::UnretainedReportRevision(source.document.revision()))?;
    if record.snapshot().pages().is_empty() {
        return Err(HardcopySourceError::EmptyContent);
    }
    let mut authenticated_references = Vec::new();
    let mut figures = Vec::new();
    let mut linked_figures = Vec::new();
    let mut contains_linked_reference = false;
    for block in record
        .snapshot()
        .pages()
        .iter()
        .flat_map(|page| page.sections())
        .flat_map(|section| section.blocks())
    {
        let Some(reference) = block.kind().reference() else {
            continue;
        };
        if let ReportBlockKind::PlotFigure(figure) = block.kind() {
            match reference {
                ReportReferenceMode::Frozen { artifact, .. } => {
                    let (width_pixels, height_pixels) =
                        validate_frozen_report_png(block.id(), artifact)?;
                    figures.push(SemanticReportFigure {
                        block_id: block.id(),
                        artifact_digest: artifact.content_digest(),
                        media_type: artifact.media_type().to_owned(),
                        payload: artifact.payload().to_vec(),
                        width_pixels,
                        height_pixels,
                        caption: figure.caption.clone(),
                        alternative_text: figure.alternative_text.clone(),
                        sizing: figure.sizing,
                    });
                }
                ReportReferenceMode::Linked { .. } => linked_figures.push(block.id()),
            }
        }
        contains_linked_reference |= matches!(reference, ReportReferenceMode::Linked { .. });
        authenticated_references.push(SemanticReportReference {
            block_id: block.id(),
            reference: reference.clone(),
        });
    }
    let empty_inventory = ReportReferenceInventory::default();
    let inventory = match (contains_linked_reference, source.reference_inventory) {
        (true, None) => {
            return Err(HardcopySourceError::ReportReferenceInventoryRequired);
        }
        (_, Some(inventory)) => inventory,
        (false, None) => &empty_inventory,
    };
    inventory
        .validate()
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
    let audit = source
        .document
        .audit_references(inventory)
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
    if audit.entries.len() != authenticated_references.len() {
        return Err(HardcopySourceError::InvalidReportSource(
            "reference audit does not cover every referenced report block".to_owned(),
        ));
    }
    for entry in &audit.entries {
        if !matches!(
            entry.currentness,
            ReportReferenceCurrentness::Current | ReportReferenceCurrentness::Frozen
        ) {
            return Err(HardcopySourceError::UnauthenticatedReportReference {
                block_id: entry.block_id,
                currentness: entry.currentness,
            });
        }
    }
    if let Some(block_id) = linked_figures.first().copied() {
        return Err(HardcopySourceError::UnsupportedAuthenticatedReportBlock {
            block_id,
            kind: "linked plot figure",
            reason:
                "the source inventory authenticates identity but supplies no exact semantic or raster figure payload"
                    .to_owned(),
        });
    }
    let page_count = i64::try_from(record.snapshot().pages().len())
        .map_err(|_| HardcopySourceError::CoordinateOverflow)?;
    let height = REPORT_PAGE_HEIGHT_UM
        .checked_mul(page_count)
        .and_then(|value| {
            REPORT_PAGE_GAP_UM
                .checked_mul(page_count.saturating_sub(1))
                .and_then(|gaps| value.checked_add(gaps))
        })
        .ok_or(HardcopySourceError::CoordinateOverflow)?;
    let identity = HardcopySourceIdentity::try_new(
        source.source_key,
        HardcopyDocumentId::try_from_uuid(source.document.id().as_uuid())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        source.document.revision(),
        source.document.title(),
    )?;
    finish_resolved(
        identity,
        record.snapshot_digest(),
        HardcopyDocumentKind::Report,
        source.scope,
        HardcopySemanticDocument::Report(SemanticReport {
            pages: record.snapshot().pages().to_vec(),
            authenticated_references,
            figures,
        }),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, height),
        )?,
    )
}
