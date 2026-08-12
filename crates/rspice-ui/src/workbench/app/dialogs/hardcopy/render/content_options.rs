//! The section panels of the hardcopy studio: what to publish and how.
//!
//! Each function owns exactly one section of the main page and draws into the
//! editor column the rail selected. None of them refreshes the preview
//! themselves — the body compares the whole draft before and after the frame,
//! so moving a control from one section to another cannot quietly stop it
//! invalidating the plan. Options a target cannot honour are disabled with the
//! reason attached rather than offered and then refused: transparency needs a
//! vector artifact, color needs a printer that reports it, searchable text
//! needs the typefaces that draw it inside the artifact, and a raster
//! resolution is bounded by what this host can afford for the chosen page.

use super::*;

/// 01 — which retained document, and how much of it.
pub(super) fn source_panel(ui: &mut Ui, draft: &mut HardcopyDialogState) -> BodyAction {
    let mut action = BodyAction::None;
    let active_kind = draft.source.as_ref().map(|source| source.document_kind());
    let active_key = draft
        .resolved_document
        .as_ref()
        .map(|source| source.source_key().to_owned());
    form_row(ui, |row| {
        row.narrow(|ui| {
            field(ui, "Document type", |ui| {
                egui::ComboBox::from_id_salt("hardcopy-document-type")
                    .selected_text(active_kind.map_or("No active type", document_kind_label))
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for kind in [
                            crate::hardcopy::HardcopyDocumentKind::SchematicOrSymbol,
                            crate::hardcopy::HardcopyDocumentKind::LayoutWithLayerLegend,
                            crate::hardcopy::HardcopyDocumentKind::PlotOrWorksheet,
                            crate::hardcopy::HardcopyDocumentKind::Report,
                        ] {
                            let candidate = draft.source_candidates.iter().find(|candidate| {
                                candidate.document_kind == kind
                                    && candidate.availability.is_available()
                            });
                            let active = active_kind == Some(kind);
                            let response = ui.add_enabled(
                                candidate.is_some(),
                                egui::Button::selectable(active, document_kind_label(kind)),
                            );
                            if response.clicked()
                                && let Some(candidate) = candidate
                                && active_key.as_deref() != Some(candidate.source_key.as_str())
                                && let Some((source_key, scope)) = source_choice_for_active_extent(
                                    &draft.source_candidates,
                                    Some(candidate),
                                )
                            {
                                action = BodyAction::SelectSource { source_key, scope };
                            } else if candidate.is_none() {
                                response.on_disabled_hover_text(
                                    "No authenticated retained document of this type is available.",
                                );
                            }
                        }
                    });
            });
        });
        row.wide(|ui| {
            let active_scope = draft.source.as_ref().map(|source| source.scope());
            let active_candidate = active_key.as_deref().and_then(|key| {
                draft
                    .source_candidates
                    .iter()
                    .find(|candidate| candidate.source_key == key)
            });
            field(ui, "Scope", |ui| {
                let selected = active_scope.map_or("No active extent", scope_label);
                egui::ComboBox::from_id_salt("hardcopy-scope")
                    .selected_text(selected)
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        let choices = [
                            (
                                active_scope.map_or("Active document", scope_label),
                                source_choice_for_active_extent(
                                    &draft.source_candidates,
                                    active_candidate,
                                ),
                            ),
                            (
                                "Selection",
                                source_choice_for_scope(
                                    &draft.source_candidates,
                                    active_candidate,
                                    crate::hardcopy::HardcopyScope::Selection,
                                ),
                            ),
                            (
                                "All sheets / panes",
                                source_choice_for_scope(
                                    &draft.source_candidates,
                                    active_candidate,
                                    crate::hardcopy::HardcopyScope::AllSheetsOrPanes,
                                ),
                            ),
                        ];
                        for (label, choice) in choices {
                            let active = choice.as_ref().is_some_and(|(key, scope)| {
                                active_key.as_deref() == Some(key.as_str())
                                    && active_scope == Some(scope)
                            });
                            let response = ui.add_enabled(
                                choice.is_some(),
                                egui::Button::selectable(active, label),
                            );
                            if response.clicked()
                                && let Some((source_key, scope)) = choice.clone()
                                && (!active || active_key.as_deref() != Some(source_key.as_str()))
                            {
                                action = BodyAction::SelectSource { source_key, scope };
                            } else if choice.is_none() {
                                response.on_disabled_hover_text(
                                    "This document family does not retain that publication extent.",
                                );
                            }
                        }
                        let named_sets =
                            named_source_choices(&draft.source_candidates, active_candidate);
                        if named_sets.is_empty() {
                            ui.add_enabled(
                                false,
                                egui::Button::selectable(false, "Named print set"),
                            )
                            .on_disabled_hover_text(
                                "No validated project-owned named print set is configured.",
                            );
                        } else {
                            for (source_key, scope, name) in named_sets {
                                let active = active_key.as_deref() == Some(source_key.as_str())
                                    && active_scope == Some(&scope);
                                if ui
                                    .add(egui::Button::selectable(
                                        active,
                                        format!("Named print set · {name}"),
                                    ))
                                    .clicked()
                                    && !active
                                {
                                    action = BodyAction::SelectSource { source_key, scope };
                                }
                            }
                        }
                    });
            });
        });
    });
    ui.add_space(12.0);
    let available = draft
        .source_candidates
        .iter()
        .filter(|candidate| candidate.availability.is_available())
        .count();
    fact_list(
        ui,
        &draft.source.as_ref().map_or_else(
            || vec![("Document", "no active publication source".to_owned())],
            |source| {
                vec![
                    ("Document", source.display_name().to_owned()),
                    ("Revision", format!("r{}", source.revision().get())),
                    ("Content digest", short_digest(&source.content_digest())),
                    (
                        "Retained sources",
                        format!("{available} available of {}", draft.source_candidates.len()),
                    ),
                ]
            },
        ),
    );
    ui.add_space(10.0);
    panel_note(
        ui,
        "Identity, revision and content digest are sealed when this workflow opens. Publication resolves them again and refuses rather than publishing a document that changed underneath the preview.",
    );
    action
}

/// 02 — physical media, scale and tiling.
pub(super) fn page_panel(ui: &mut Ui, draft: &mut HardcopyDialogState) -> BodyAction {
    let mut action = BodyAction::None;
    let schematic_sheet_output = schematic_sheet_relationship(draft).is_some();
    form_row(ui, |row| {
        row.narrow(|ui| {
            field(
                ui,
                if schematic_sheet_output {
                    "Output media"
                } else {
                    "Paper"
                },
                |ui| {
                    let selected = paper_label(&draft.paper);
                    egui::ComboBox::from_id_salt("hardcopy-paper")
                        .selected_text(selected)
                        .width(ui.available_width())
                        .show_ui(ui, |ui| {
                            if schematic_sheet_output {
                                ui.selectable_value(
                                    &mut draft.paper,
                                    PaperDraft::MatchAuthoredSheets,
                                    "Match each authored sheet",
                                )
                                .on_hover_text(
                                    "Use each governed schematic sheet's exact authored physical size and orientation. Mixed sheet sets retain per-page media.",
                                );
                                ui.separator();
                            }
                            for paper in [
                                StandardPaper::Letter,
                                StandardPaper::Legal,
                                StandardPaper::A4,
                                StandardPaper::A3,
                                StandardPaper::Tabloid,
                            ] {
                                ui.selectable_value(
                                    &mut draft.paper,
                                    PaperDraft::Standard(paper),
                                    standard_paper_label(paper),
                                );
                            }
                            if ui
                                .selectable_label(
                                    matches!(draft.paper, PaperDraft::Custom { .. }),
                                    "Custom…",
                                )
                                .clicked()
                            {
                                draft.paper = custom_from_current(draft);
                            }
                        });
                },
            );
        });
        row.wide(|ui| {
            enum_combo(
                ui,
                "Orientation",
                "hardcopy-orientation",
                &mut draft.orientation,
                &[
                    (Orientation::Landscape, "Landscape"),
                    (Orientation::Portrait, "Portrait"),
                    (Orientation::AutomaticPerPage, "Automatic per page"),
                ],
            );
        });
    });
    form_row(ui, |row| {
        row.narrow(|ui| {
            let mut scale_kind = scale_key(draft.scale);
            field(ui, "Scale", |ui| {
                let selected = if scale_kind == 2 {
                    format!("Custom · {}%", draft.custom_scale_percent)
                } else if let Some(resolved) = resolved_scale(draft) {
                    // The two fit modes derive the number the drawing is
                    // published at, and the closed control is where a derived
                    // number stops being a silent one. The list below still
                    // names the modes in full.
                    resolved
                } else {
                    scale_label(scale_kind).to_owned()
                };
                egui::ComboBox::from_id_salt("hardcopy-scale")
                    .selected_text(selected)
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for key in 0..4 {
                            ui.selectable_value(&mut scale_kind, key, scale_label(key));
                        }
                        if scale_kind == 2 {
                            ui.separator();
                            ui.label("Custom percent");
                            ui.add(
                                egui::TextEdit::singleline(&mut draft.custom_scale_percent)
                                    .desired_width(120.0),
                            );
                        }
                    });
            });
            draft.scale = match scale_kind {
                0 => ScaleMode::FitPrintableArea,
                1 => ScaleMode::EngineeringOneToOne,
                2 => ScaleMode::CustomPercent {
                    hundredths_percent: 10_000,
                },
                _ => ScaleMode::FitWidth,
            };
        });
        row.wide(|ui| {
            let mut tiling_kind = tiling_key(draft.tiling);
            field(ui, "Tiled pages", |ui| {
                let automatic = draft.preview_plan.as_ref().map_or_else(
                    || "Automatic".to_owned(),
                    |plan| {
                        if plan.pagination().sections().is_empty() {
                            format!(
                                "Automatic · {} × {} pages",
                                plan.pagination().columns(),
                                plan.pagination().rows()
                            )
                        } else {
                            format!(
                                "Automatic · {} section-aware pages",
                                plan.pagination().pages().len()
                            )
                        }
                    },
                );
                let selected = match tiling_kind {
                    0 => automatic.clone(),
                    1 => "Single page".to_owned(),
                    _ => manual_label(draft),
                };
                egui::ComboBox::from_id_salt("hardcopy-pagination")
                    .selected_text(selected)
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut tiling_kind, 0, automatic);
                        ui.selectable_value(&mut tiling_kind, 1, "Single page")
                            .on_hover_text(
                                "Refuse to publish rather than spread this drawing over more than one sheet. Choose the fit-to-printable-area scale to make it fit.",
                            );
                        ui.selectable_value(&mut tiling_kind, 2, "Manual rows and columns")
                            .on_hover_text(
                                "Publish on exactly this grid. Asking for more sheets than the drawing needs widens every seam past the tile overlap below.",
                            );
                        if tiling_kind == 2 {
                            ui.separator();
                            ui.label("Manual page grid");
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::TextEdit::singleline(&mut draft.manual_columns)
                                        .desired_width(54.0)
                                        .hint_text("Columns"),
                                );
                                ui.label("×");
                                ui.add(
                                    egui::TextEdit::singleline(&mut draft.manual_rows)
                                        .desired_width(54.0)
                                        .hint_text("Rows"),
                                );
                            });
                        }
                    });
            });
            draft.tiling = match tiling_kind {
                0 => TilingMode::Automatic,
                1 => TilingMode::SinglePage,
                _ => TilingMode::Manual {
                    columns: 2,
                    rows: 1,
                },
            };
        });
    });
    form_row(ui, |row| {
        row.narrow(|ui| {
            length_field(ui, "Tile overlap", &mut draft.overlap, draft.display_unit);
        });
        row.wide(|ui| {
            let row_height = ui.spacing().interact_size.y * 2.0 + 5.0;
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), row_height),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.checkbox(
                        &mut draft.registration_marks,
                        "Registration marks and page coordinates",
                    );
                },
            );
        });
    });
    ui.add_space(8.0);
    let unit = match draft.display_unit {
        LengthUnit::Inches => "in",
        LengthUnit::Millimetres => "mm",
    };
    if action_row(
        ui,
        "Custom paper and margins…",
        &format!(
            "margins {}/{}/{}/{} {unit} · bleed {} {unit}",
            draft.margin_top,
            draft.margin_right,
            draft.margin_bottom,
            draft.margin_left,
            draft.bleed
        ),
    ) {
        action = BodyAction::CustomPaper;
    }
    action
}

/// The closed reading of a scale mode that derives its own percentage. Kept
/// short because the control has to say it inside one line of the narrow
/// column; the list it opens carries the full names.
fn resolved_scale(draft: &HardcopyDialogState) -> Option<String> {
    let mode = match draft.scale {
        ScaleMode::FitPrintableArea => "Fit page",
        ScaleMode::FitWidth => "Fit width",
        ScaleMode::EngineeringOneToOne | ScaleMode::CustomPercent { .. } => return None,
    };
    let tenths = draft
        .preview_plan
        .as_ref()?
        .pagination()
        .scale()
        .hundredths_percent()
        / 10;
    Some(format!("{mode} · {}.{}%", tenths / 10, tenths % 10))
}

/// The grid the fields ask for, and the seam it actually leaves. A grid wider
/// than the drawing needs spends the surplus on overlap, so the tile-overlap
/// field beside this one states a floor and this states what reaches the paper.
fn manual_label(draft: &HardcopyDialogState) -> String {
    let grid = format!("Manual · {} × {}", draft.manual_columns, draft.manual_rows);
    let Some(seam) = draft
        .preview_plan
        .as_ref()
        .filter(|_| matches!(draft.tiling, TilingMode::Manual { .. }))
        .and_then(|plan| plan.pagination().tile_overlap())
    else {
        return format!("{grid} pages");
    };
    let unit = match draft.display_unit {
        LengthUnit::Inches => "in",
        LengthUnit::Millimetres => "mm",
    };
    format!(
        "{grid} · seam {} {unit}",
        format_length_local(seam, draft.display_unit)
    )
}

/// 03 — how much of the authored drawing sheet is drawn.
pub(super) fn drawing_sheet_panel(ui: &mut Ui, draft: &mut HardcopyDialogState) {
    let Some(has_outside_content) = schematic_sheet_relationship(draft) else {
        return;
    };
    panel_note(
        ui,
        "These controls affect Print and Export only. They never rewrite the authored drawing sheet.",
    );
    ui.add_space(9.0);

    form_row(ui, |row| {
        row.narrow(|ui| {
            field(ui, "Extent", |ui| {
                egui::ComboBox::from_id_salt("hardcopy-schematic-extent")
                    .selected_text(match draft.schematic_extent {
                        SchematicHardcopyExtent::AuthoredDrawingSheet => "Authored drawing sheet",
                        SchematicHardcopyExtent::CompleteSchematicContent => {
                            "Complete schematic content"
                        }
                    })
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut draft.schematic_extent,
                            SchematicHardcopyExtent::AuthoredDrawingSheet,
                            "Authored drawing sheet",
                        );
                        ui.selectable_value(
                            &mut draft.schematic_extent,
                            SchematicHardcopyExtent::CompleteSchematicContent,
                            "Complete schematic content",
                        );
                    });
            });
        });
        row.wide(|ui| {
            let complete =
                draft.schematic_extent == SchematicHardcopyExtent::CompleteSchematicContent;
            if complete {
                draft.outside_sheet_content = OutsideSheetContentPolicy::ExtendOutput;
            }
            field(ui, "Outside-sheet content", |ui| {
                let selected = match draft.outside_sheet_content {
                    OutsideSheetContentPolicy::Ask => "Ask when content crosses sheet",
                    OutsideSheetContentPolicy::ClipToAuthoredSheet => {
                        "Clip to authored drawing sheet"
                    }
                    OutsideSheetContentPolicy::ExtendOutput => "Extend output to include content",
                };
                ui.add_enabled_ui(!complete, |ui| {
                    egui::ComboBox::from_id_salt("hardcopy-outside-sheet-content")
                        .selected_text(selected)
                        .width(ui.available_width())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut draft.outside_sheet_content,
                                OutsideSheetContentPolicy::Ask,
                                "Ask when content crosses sheet",
                            );
                            ui.selectable_value(
                                &mut draft.outside_sheet_content,
                                OutsideSheetContentPolicy::ClipToAuthoredSheet,
                                "Clip to authored drawing sheet",
                            );
                            ui.selectable_value(
                                &mut draft.outside_sheet_content,
                                OutsideSheetContentPolicy::ExtendOutput,
                                "Extend output to include content",
                            );
                        });
                });
            });
        });
    });

    ui.add_space(10.0);
    ui.columns(2, |columns| {
        columns[0].checkbox(&mut draft.crop_marks, "Crop marks");
        columns[1].checkbox(
            &mut draft.include_sheet_paper,
            "Include paper and printable boundary",
        );
        columns[0].checkbox(&mut draft.include_sheet_border, "Include border");
        columns[1].checkbox(&mut draft.include_sheet_title_block, "Include title block");
        columns[0].checkbox(&mut draft.include_sheet_zones, "Include reference zones");
        columns[1].checkbox(&mut draft.include_schematic_grid, "Include schematic grid");
    });

    let status = if has_outside_content {
        match draft.schematic_extent {
            SchematicHardcopyExtent::CompleteSchematicContent => {
                "Outside content is included by the complete-content extent."
            }
            SchematicHardcopyExtent::AuthoredDrawingSheet => match draft.outside_sheet_content {
                OutsideSheetContentPolicy::Ask => {
                    "Outside content is present. Choose Clip to publish only the authored sheet, or Extend to retain every resolved object."
                }
                OutsideSheetContentPolicy::ClipToAuthoredSheet => {
                    "Outside content is clipped exactly at the authored drawing-sheet boundary."
                }
                OutsideSheetContentPolicy::ExtendOutput => {
                    "The output extent expands to retain every resolved object."
                }
            },
        }
    } else {
        "All resolved schematic content is within the authored drawing sheet."
    };
    ui.add_space(10.0);
    panel_note(ui, status);
}

/// 04 — format, resolution and color.
pub(super) fn output_panel(ui: &mut Ui, draft: &mut HardcopyDialogState) -> BodyAction {
    let mut action = BodyAction::None;
    let old_format = draft.format;
    if draft.format.raster_dpi().is_some() {
        form_row(ui, |row| {
            row.wide(|ui| output_format_field(ui, draft));
            row.narrow(|ui| raster_resolution_field(ui, draft));
        });
    } else {
        output_format_field(ui, draft);
    }
    if old_format != draft.format && !draft.format.is_vector() {
        draft.searchable_text = false;
    }
    // The archive format is the one case where the contract makes the font
    // decision itself. It says nothing about outlining, so searchable text
    // stays the operator's.
    if draft.format == OutputFormat::PdfA {
        draft.embed_fonts = true;
    }

    let color_supported = draft.format != OutputFormat::NativePrinter
        || draft
            .printer_capabilities
            .as_ref()
            .is_some_and(|capabilities| capabilities.supports_color());
    if !color_supported
        && matches!(
            draft.color_mapping,
            ColorMapping::PrintSafeEngineeringPalette | ColorMapping::ScreenColors
        )
    {
        draft.color_mapping = ColorMapping::GrayscaleWithDashMarkerRedundancy;
    }
    form_row(ui, |row| {
        row.wide(|ui| {
            field(ui, "Color mapping", |ui| {
                egui::ComboBox::from_id_salt("hardcopy-color")
                    .selected_text(color_mapping_label(draft.color_mapping))
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for (value, label, needs_color) in [
                            (
                                ColorMapping::PrintSafeEngineeringPalette,
                                "Print-safe engineering palette",
                                true,
                            ),
                            (ColorMapping::ScreenColors, "Screen colors", true),
                            (
                                ColorMapping::GrayscaleWithDashMarkerRedundancy,
                                "Grayscale with dash/marker redundancy",
                                false,
                            ),
                            (ColorMapping::Monochrome, "Monochrome", false),
                        ] {
                            let response = ui.add_enabled(
                                color_supported || !needs_color,
                                egui::Button::selectable(draft.color_mapping == value, label),
                            );
                            if response.clicked() {
                                draft.color_mapping = value;
                            } else if needs_color && !color_supported {
                                response.on_disabled_hover_text(
                                    "The selected printer reports monochrome output only.",
                                );
                            }
                        }
                    });
            });
        });
        row.narrow(|ui| {
            field(ui, "Background", |ui| {
                let transparent_available = draft.format.is_vector()
                    && !matches!(
                        draft.format,
                        OutputFormat::NativePrinter | OutputFormat::BrowserPrintDocument
                    );
                egui::ComboBox::from_id_salt("hardcopy-background")
                    .selected_text(background_label(draft.background))
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut draft.background, BackgroundMode::White, "White");
                        let transparent = ui.add_enabled(
                            transparent_available,
                            egui::Button::selectable(
                                draft.background == BackgroundMode::Transparent,
                                "Transparent · vector export",
                            ),
                        );
                        if transparent.clicked() {
                            draft.background = BackgroundMode::Transparent;
                        } else if !transparent_available {
                            transparent.on_disabled_hover_text(
                                "Transparent background requires a vector artifact target.",
                            );
                        }
                        ui.selectable_value(
                            &mut draft.background,
                            BackgroundMode::WorkspaceBackground,
                            "Workspace background",
                        );
                    });
            });
        });
    });
    if draft.background == BackgroundMode::Transparent
        && (!draft.format.is_vector()
            || matches!(
                draft.format,
                OutputFormat::NativePrinter | OutputFormat::BrowserPrintDocument
            ))
    {
        draft.background = BackgroundMode::White;
    }
    ui.add_space(4.0);
    ui.checkbox(&mut draft.soft_proof, "Soft-proof print-safe colors");

    ui.add_space(10.0);
    if action_row(
        ui,
        "Layer, trace, and marker mapping…",
        "dash, marker, hatch and label redundancy",
    ) {
        action = BodyAction::PrintMapping;
    }
    if draft.workflow != HardcopyWorkflow::Export && printer_properties_reachable() {
        ui.add_space(6.0);
        if action_row(
            ui,
            "Printer properties…",
            "tray, duplex, resolution and collation",
        ) {
            action = BodyAction::PrinterProperties;
        }
    }
    action
}

/// Whether this host has a driver boundary the dialog can open at all. The
/// browser selects tray, duplex and collation in its own print dialog, and the
/// other desktops have a driver RSpice does not yet speak to — so on both there
/// is nothing behind this row. Where it leads nowhere it is absent rather than
/// dimmed: a row advertising capabilities the host does not have reads as a
/// feature withheld instead of one that was never there.
const fn printer_properties_reachable() -> bool {
    cfg!(target_os = "windows")
}

fn output_format_field(ui: &mut Ui, draft: &mut HardcopyDialogState) {
    let output_label = if draft.workflow == HardcopyWorkflow::Export {
        "Output format"
    } else {
        "Printer / target"
    };
    field(ui, output_label, |ui| {
        egui::ComboBox::from_id_salt("hardcopy-output")
            .selected_text(format_label(draft.format))
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                #[cfg(target_os = "windows")]
                if draft.workflow != HardcopyWorkflow::Export {
                    ui.selectable_value(
                        &mut draft.format,
                        OutputFormat::NativePrinter,
                        "System printer",
                    );
                }
                #[cfg(target_arch = "wasm32")]
                if draft.workflow != HardcopyWorkflow::Export {
                    ui.selectable_value(
                        &mut draft.format,
                        OutputFormat::BrowserPrintDocument,
                        "Browser print dialog",
                    );
                }
                #[cfg(all(not(target_arch = "wasm32"), not(target_os = "windows")))]
                if draft.workflow != HardcopyWorkflow::Export {
                    ui.selectable_value(
                        &mut draft.format,
                        OutputFormat::BrowserPrintDocument,
                        "Print-ready document handoff",
                    );
                }
                // The raster entries carry whatever resolution is currently in
                // play, so leaving PNG for SVG and coming back does not
                // silently reset it — each bounded by what this host can raster
                // for that format on this page, because the two formats charge
                // different bytes per pixel and neither ceiling is the default
                // this list would otherwise commit.
                let dpi = draft.format.raster_dpi().unwrap_or(DEFAULT_RASTER_DPI);
                let png = bounded_raster(draft, OutputFormat::Png { dpi });
                let tiff = bounded_raster(draft, OutputFormat::Tiff { dpi });
                for (format, label) in [
                    (OutputFormat::PdfVector, "PDF · vector"),
                    (OutputFormat::PdfA, "PDF/A · vector"),
                    (OutputFormat::SvgVector, "SVG · vector"),
                    (png, "PNG · raster"),
                    (tiff, "TIFF · raster"),
                ] {
                    ui.selectable_value(&mut draft.format, format, label);
                }
            });
    });
}

/// 05 — what every published page carries beyond the drawing itself.
pub(super) fn identity_panel(ui: &mut Ui, draft: &mut HardcopyDialogState) {
    ui.checkbox(
        &mut draft.include_legends,
        "Include trace, layer, net, and marker legends",
    );
    ui.checkbox(
        &mut draft.include_header,
        "Project, revision, sheet, date, and page header",
    );
    ui.checkbox(
        &mut draft.include_provenance,
        "Result manifest, model digest, and run provenance footer",
    );
    let mut watermark = !matches!(draft.watermark, Watermark::None);
    if ui
        .checkbox(&mut watermark, "Draft / confidential watermark")
        .changed()
    {
        draft.watermark = if watermark {
            Watermark::Draft
        } else {
            Watermark::None
        };
    }
    ui.add_space(10.0);
    // Two capabilities, one dependency: the contract accepts embedded fonts on
    // their own, refuses searchable text without them, and refuses searchable
    // text on a raster target at all. Each is offered where it is legal and
    // disabled with the reason where it is not, so no combination the contract
    // admits is unreachable and none it refuses is offered.
    let vector = draft.format.is_vector();
    let archival = draft.format == OutputFormat::PdfA;
    ui.columns(2, |columns| {
        let embed = columns[0].add_enabled(
            vector && !archival,
            egui::Checkbox::new(&mut draft.embed_fonts, "Embed fonts"),
        );
        if !vector {
            embed.on_disabled_hover_text("A raster target carries no typeface to embed.");
        } else if archival {
            embed.on_disabled_hover_text(
                "PDF/A refuses an artifact whose typefaces are not inside it.",
            );
        }
        let searchable = columns[1].add_enabled(
            vector && draft.embed_fonts,
            egui::Checkbox::new(&mut draft.searchable_text, "Searchable text"),
        );
        if !vector {
            searchable.on_disabled_hover_text("A raster target has no text to keep live.");
        } else if !draft.embed_fonts {
            searchable.on_disabled_hover_text(
                "Text a reader can select needs the typefaces that draw it inside the artifact.",
            );
        }
    });
    if !vector {
        draft.embed_fonts = false;
        draft.searchable_text = false;
    } else if !draft.embed_fonts {
        draft.searchable_text = false;
    }
    ui.add_space(6.0);
    panel_note(
        ui,
        match draft.format {
            OutputFormat::PdfA => {
                "PDF/A embeds its typefaces by contract. Searchable text is still a choice: without it the letters are published as outlines."
            }
            OutputFormat::SvgVector => {
                "SVG has no outlined-text form here, so a page carrying any text is refused unless it stays searchable."
            }
            format if format.is_vector() => {
                "Embedded fonts make the artifact self-contained; searchable text keeps its letters selectable rather than drawn as outlines."
            }
            _ => {
                "A raster target has no text to embed or search; both apply to vector formats only."
            }
        },
    );
}

fn panel_note(ui: &mut Ui, text: &str) {
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .size(tokens::FS_0)
                .color(Tokens::get(ui.ctx()).color.text_dim),
        )
        .wrap(),
    );
}

/// Resolution for the raster formats, bounded by what this host can actually
/// rasterise for the chosen page.
///
/// The ceiling is not a constant and not a guess: `max_raster_dpi` bisects the
/// publisher's own budget predicates for the current pagination and format, so
/// the number offered here and the number the renderer will accept cannot
/// disagree. It moves with the page — changing media or tiling can pull it
/// under a resolution already typed, which the field then reports rather than
/// silently rewriting. A browser's raster budget is a quarter of the desktop's,
/// so this is also where the two hosts visibly differ instead of one of them
/// offering a setting that always fails.
fn raster_resolution_field(ui: &mut Ui, draft: &mut HardcopyDialogState) {
    if draft.format.raster_dpi().is_none() {
        return;
    }
    let ceiling = raster_dpi_ceiling(draft);

    field(ui, "Resolution", |ui| {
        let visuals = ui.style().visuals.widgets.inactive;
        let available_width = ui.available_width();
        let control_height = ui.spacing().interact_size.y;
        let response = Frame::NONE
            .fill(visuals.bg_fill)
            .stroke(visuals.bg_stroke)
            .corner_radius(visuals.corner_radius)
            .inner_margin(Margin::symmetric(6, 0))
            .show(ui, |ui| {
                ui.set_min_size(vec2((available_width - 12.0).max(76.0), control_height));
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let suffix_width = 32.0;
                    let response = ui.add_sized(
                        vec2(
                            (ui.available_width() - suffix_width).max(48.0),
                            control_height,
                        ),
                        egui::TextEdit::singleline(&mut draft.raster_dpi_draft)
                            .id(raster_resolution_id())
                            .font(egui::TextStyle::Monospace)
                            .frame(Frame::NONE),
                    );
                    let suffix = ui.allocate_ui_with_layout(
                        vec2(suffix_width, control_height),
                        Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |ui| {
                            ui.label(
                                egui::RichText::new("dpi")
                                    .monospace()
                                    .color(ui.visuals().weak_text_color()),
                            );
                        },
                    );
                    ui.painter().vline(
                        suffix.response.rect.left(),
                        suffix.response.rect.y_range().shrink(4.0),
                        visuals.bg_stroke,
                    );
                    response
                })
                .inner
            })
            .inner;

        // Commit on Enter or on leaving the field. Never per keystroke: `60`
        // is a legal prefix of `600` and an order of magnitude away from it.
        if response.lost_focus() {
            commit_raster_resolution(draft, ceiling);
        }

        let (message, tone) = raster_resolution_note(&draft.raster_dpi_draft, ceiling);
        ui.add(
            egui::Label::new(
                egui::RichText::new(message)
                    .size(tokens::FS_0)
                    .color(tone(ui)),
            )
            .wrap(),
        );
    });
}

/// The resolution field's fixed identity, so whether it is being edited can be
/// read on the passes where the section that draws it is no longer on screen.
fn raster_resolution_id() -> egui::Id {
    egui::Id::new("hardcopy-raster-resolution")
}

/// What this page and this host will accept for the format in the draft, or the
/// contract's own range while no plan has compiled.
fn raster_dpi_ceiling(draft: &HardcopyDialogState) -> Option<u16> {
    raster_dpi_ceiling_for(draft, draft.format)
}

/// The same question asked of a format the draft does not currently hold, which
/// is what the target list has to ask before it offers one.
fn raster_dpi_ceiling_for(draft: &HardcopyDialogState, format: OutputFormat) -> Option<u16> {
    draft
        .preview_plan
        .as_ref()
        .and_then(|plan| max_raster_dpi(plan, format))
}

/// The resolution in force for a raster target: the one the format carries,
/// never above what this host can raster for the page in front of it.
fn resolution_in_force(format: OutputFormat, ceiling: Option<u16>) -> u16 {
    let dpi = format.raster_dpi().unwrap_or(DEFAULT_RASTER_DPI);
    ceiling.map_or(dpi, |ceiling| dpi.min(ceiling))
}

/// `format` carrying a resolution this host can actually raster for it here.
fn bounded_raster(draft: &HardcopyDialogState, format: OutputFormat) -> OutputFormat {
    format.with_raster_dpi(resolution_in_force(
        format,
        raster_dpi_ceiling_for(draft, format),
    ))
}

/// Take what the field holds into the draft. An entry that names no resolution
/// this page admits reverts to the one in force, because the field states the
/// committed value whenever it is not being typed into — and the value it
/// reverts to is bounded, since reverting to a resolution the publisher would
/// itself refuse is no refusal at all.
fn commit_raster_resolution(draft: &mut HardcopyDialogState, ceiling: Option<u16>) {
    let dpi = parse_raster_dpi(&draft.raster_dpi_draft, ceiling).unwrap_or_else(|_| {
        let dpi = resolution_in_force(draft.format, ceiling);
        draft.raster_dpi_draft = dpi.to_string();
        dpi
    });
    draft.format = draft.format.with_raster_dpi(dpi);
}

/// Settle a resolution the operator typed and then walked away from.
///
/// The field's text outlives the widget that edits it. Selecting another
/// section, or the page instead of the setup, replaces what is drawn in the
/// same pass the choice is made, so the field is never drawn again to notice
/// its editing session ended — and a publication started from wherever the
/// operator landed would carry the previously committed resolution while the
/// field still showed the typed one. The body settles it once, on the path
/// every pass takes, rather than from the surfaces a layout may decide not to
/// draw.
///
/// The ceiling settles on the same path and for the same kind of reason. It
/// belongs to the page rather than to the field, so media, orientation, scale
/// and tiling all move it under a resolution that was legal when it was
/// committed. Bringing the committed value back inside it here is what keeps
/// the rail summary, the estimate strip and the enabled primary describing a
/// publication this host will accept, rather than one it refuses a render round
/// trip later.
pub(super) fn settle_raster_resolution(ui: &Ui, draft: &mut HardcopyDialogState) {
    if draft.format.raster_dpi().is_none() {
        return;
    }
    let ceiling = raster_dpi_ceiling(draft);
    draft.format = bounded_raster(draft, draft.format);
    if ui
        .ctx()
        .memory(|memory| memory.has_focus(raster_resolution_id()))
    {
        return;
    }
    commit_raster_resolution(draft, ceiling);
}

/// Accept a resolution only if both the contract and this host admit it.
pub(super) fn parse_raster_dpi(text: &str, ceiling: Option<u16>) -> Result<u16, String> {
    let dpi: u16 = text
        .trim()
        .parse()
        .map_err(|_| "enter a whole number of dots per inch".to_owned())?;
    if dpi < MIN_RASTER_DPI {
        return Err(format!(
            "{MIN_RASTER_DPI} dpi is the lowest this contract accepts"
        ));
    }
    if dpi > ceiling.unwrap_or(MAX_RASTER_DPI) {
        return Err(match ceiling {
            Some(ceiling) => format!("this page cannot be rastered above {ceiling} dpi here"),
            None => format!("{MAX_RASTER_DPI} dpi is the highest this contract accepts"),
        });
    }
    Ok(dpi)
}

type NoteTone = fn(&Ui) -> egui::Color32;

/// What the field says under itself. Only the text being typed can be out of
/// range: the committed value is settled inside the ceiling on every pass, so
/// the note states the bound rather than reporting the draft against it.
fn raster_resolution_note(text: &str, ceiling: Option<u16>) -> (String, NoteTone) {
    let warn: NoteTone = |ui| ui.visuals().warn_fg_color;
    let dim: NoteTone = |ui| Tokens::get(ui.ctx()).color.text_dim;
    match parse_raster_dpi(text, ceiling) {
        Err(reason) => (reason, warn),
        Ok(_) => match ceiling {
            Some(ceiling) => (format!("up to {ceiling} dpi for this page"), dim),
            None => (format!("{MIN_RASTER_DPI}–{MAX_RASTER_DPI} dpi"), dim),
        },
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    use egui::accesskit::Role;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::hardcopy::sources::HardcopySourceIdentity;
    use crate::hardcopy::{HardcopyDocumentId, HardcopyPlanId, HardcopyScope};
    use crate::product::ObjectRevision;
    use crate::state::SchematicSheetFormat;
    use crate::workbench::hardcopy_adapters::render::{
        HardcopyPublicationTimestamp, HardcopySceneMetadata,
    };
    use crate::workbench::hardcopy_adapters::sources::resolve_blank_schematic_sheet_with_format;

    /// The save picker the export reaches once the plan is sealed. It refuses a
    /// destination so nothing is written, and counts the calls so a test can
    /// tell "the export ran and carried this plan" from "the export never got
    /// past the plan".
    struct RefusedDestination(Arc<AtomicUsize>);

    impl crate::workbench::workflows::export_workflow::ExportWorkflowIo for RefusedDestination {
        fn show_save_dialog(
            &self,
            _config: crate::workbench::workflows::export_workflow::SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(None)
        }

        fn write_text_file(&self, _path: &Path, _contents: &str) -> Result<(), String> {
            Ok(())
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    struct Studio {
        ctx: Context,
        app: RSpiceApp,
        exports: Arc<AtomicUsize>,
        viewport: Vec2,
    }

    impl Studio {
        /// The Export workflow open on the active cell view, settled onto a
        /// 150 dpi raster target: a resolution the operator can be seen to
        /// change, an order of magnitude clear of this page's ceiling.
        fn raster(viewport: Vec2) -> Self {
            let ctx = Context::default();
            ctx.enable_accesskit();
            crate::ui::Theme::default().apply(&ctx);
            let exports = Arc::new(AtomicUsize::new(0));
            let mut app = RSpiceApp::test_instance();
            app.export_workflow_io = Box::new(RefusedDestination(exports.clone()));
            let reference = app.state.workspace.active_view.clone();
            app.state.workbench.documents.activate(
                crate::workbench::state::WorkspaceDocumentId::CellView(reference),
            );
            publish::open_hardcopy_workflow(&mut app, HardcopyWorkflow::Export);
            let mut studio = Self {
                ctx,
                app,
                exports,
                viewport,
            };
            studio.settle();
            studio.app.state.dialogs.hardcopy.section = HardcopySection::Output;
            studio.app.state.dialogs.hardcopy.region = HardcopyRegion::Setup;
            studio.app.state.dialogs.hardcopy.format = OutputFormat::Png { dpi: 150 };
            studio.app.state.dialogs.hardcopy.raster_dpi_draft = "150".to_owned();
            studio.app.state.dialogs.hardcopy.embed_fonts = false;
            studio.app.state.dialogs.hardcopy.searchable_text = false;
            studio.app.state.dialogs.hardcopy.refresh_preview();
            studio.settle();
            studio
        }

        /// The desktop composition: rail, editor and preview all on screen.
        fn wide() -> Self {
            Self::raster(vec2(1_440.0, 900.0))
        }

        /// A phone-width surface, which is below `STUDIO_SPLIT_WIDTH` and so
        /// shows the setup or the page but never both.
        fn narrow() -> Self {
            Self::raster(vec2(390.0, 844.0))
        }

        fn pass(&mut self, events: Vec<egui::Event>) -> egui::FullOutput {
            let app = &mut self.app;
            let viewport = self.viewport;
            self.ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, viewport)),
                    events,
                    ..Default::default()
                },
                |ui| app.render_hardcopy_dialog(ui),
            )
        }

        /// The source resolves on a worker and the exact page raster lands a
        /// pass after the plan it belongs to, so a measurement is only
        /// meaningful once both have happened.
        fn settle(&mut self) -> egui::FullOutput {
            for _ in 0..400 {
                let _ = self.pass(Vec::new());
                if self.app.state.dialogs.hardcopy.preview.is_some() {
                    let _ = self.pass(Vec::new());
                    return self.pass(Vec::new());
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            panic!("the hardcopy studio never resolved a source and an exact page preview");
        }

        fn control(&mut self, role: Role, label: &str) -> Rect {
            let output = self.pass(Vec::new());
            controls(&output)
                .into_iter()
                .find(|(found, name, _)| *found == role && name == label)
                .unwrap_or_else(|| panic!("the studio drew no {role:?} labelled {label:?}"))
                .2
        }

        fn field(&mut self) -> Rect {
            let output = self.pass(Vec::new());
            controls(&output)
                .into_iter()
                .find(|(role, _, _)| *role == Role::TextInput)
                .expect("the output section draws the resolution field")
                .2
        }

        /// Put the caret in the resolution field and replace what it holds.
        fn type_resolution(&mut self, text: &str) {
            let field = self.field();
            let caret = egui::pos2(field.right() - 4.0, field.center().y);
            let _ = self.pass(press(caret));
            let _ = self.pass(release(caret));
            let _ = self.pass(vec![
                egui::Event::Key {
                    key: egui::Key::A,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::COMMAND,
                },
                egui::Event::Text(text.to_owned()),
            ]);
            assert_eq!(
                self.app.state.dialogs.hardcopy.raster_dpi_draft, text,
                "the field did not take the typed resolution"
            );
        }

        /// The resolution of the plan the export sealed. `current_plan` hands
        /// the publication the dialog's own preview plan, so this is the plan
        /// that ran — and the export is only credited if the workflow reached
        /// its destination picker rather than refusing before it.
        fn published_resolution(&self) -> Option<u16> {
            assert_eq!(
                self.exports.load(Ordering::SeqCst),
                1,
                "the export never reached its destination: {:?}",
                self.app.state.dialogs.hardcopy.error
            );
            self.app
                .state
                .dialogs
                .hardcopy
                .preview_plan
                .as_ref()
                .and_then(|plan| plan.setup().render().format().raster_dpi())
        }
    }

    fn controls(output: &egui::FullOutput) -> Vec<(Role, String, Rect)> {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("the probe enables AccessKit, so every pass carries a tree")
            .nodes
            .iter()
            .filter_map(|(_, node)| {
                let bounds = node.bounds()?;
                let rect = Rect::from_min_max(
                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                );
                rect.is_finite().then(|| {
                    (
                        node.role(),
                        node.label()
                            .or_else(|| node.value())
                            .unwrap_or_default()
                            .to_owned(),
                        rect,
                    )
                })
            })
            .collect()
    }

    fn press(pos: egui::Pos2) -> Vec<egui::Event> {
        vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ]
    }

    fn release(pos: egui::Pos2) -> Vec<egui::Event> {
        vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }]
    }

    fn click(pos: egui::Pos2) -> Vec<egui::Event> {
        let mut events = press(pos);
        events.extend(release(pos));
        events
    }

    /// A click delivered inside one pass — a press and a release between two
    /// repaints, which is what an ordinary click on a settled surface is. The
    /// dialog draws its body before its command row, so the field has to notice
    /// it is no longer being edited within the same pass that publishes.
    #[test]
    fn a_resolution_typed_and_published_in_one_gesture_publishes_what_it_shows() {
        let mut studio = Studio::wide();
        studio.type_resolution("300");
        let button = studio.control(Role::Button, "Export hardcopy");
        let _ = studio.pass(click(button.center()));

        assert_eq!(studio.published_resolution(), Some(300));
        assert_eq!(studio.app.state.dialogs.hardcopy.raster_dpi_draft, "300");
    }

    /// The same gesture spread over two passes, which is what a click looks
    /// like when the surface repaints between the press and the release.
    #[test]
    fn a_resolution_published_by_a_click_spanning_two_passes_publishes_what_it_shows() {
        let mut studio = Studio::wide();
        studio.type_resolution("300");
        let button = studio.control(Role::Button, "Export hardcopy");
        let _ = studio.pass(press(button.center()));
        let _ = studio.pass(release(button.center()));

        assert_eq!(studio.published_resolution(), Some(300));
        assert_eq!(studio.app.state.dialogs.hardcopy.raster_dpi_draft, "300");
    }

    /// The rail replaces the section in the pass it is clicked, so the field is
    /// never drawn again to see that its editing session ended. Publishing from
    /// the section the operator landed on used to seal the previous resolution
    /// while the field still held the typed one.
    #[test]
    fn a_resolution_left_behind_by_the_rail_is_still_the_one_published() {
        let mut studio = Studio::wide();
        studio.type_resolution("300");
        let rail = studio.control(Role::Button, "Identity");
        let _ = studio.pass(click(rail.center()));
        assert_eq!(
            studio.app.state.dialogs.hardcopy.section,
            HardcopySection::Identity,
            "the rail did not leave the output section"
        );

        let button = studio.control(Role::Button, "Export hardcopy");
        let _ = studio.pass(click(button.center()));

        assert_eq!(studio.published_resolution(), Some(300));
        assert_eq!(studio.app.state.dialogs.hardcopy.raster_dpi_draft, "300");
    }

    /// Below `STUDIO_SPLIT_WIDTH` the surface shows the setup or the page and
    /// never both, so choosing the page replaces the whole editor — the section
    /// panels included — in the pass the switch is clicked. Publishing from
    /// there used to seal the previous resolution while the field held the
    /// typed one, and no section panel was left running to notice.
    #[test]
    fn a_resolution_left_behind_by_the_region_switch_is_still_the_one_published() {
        let mut studio = Studio::narrow();
        studio.type_resolution("300");
        let preview = studio.control(Role::Button, "Exact preview");
        let _ = studio.pass(click(preview.center()));
        assert_eq!(
            studio.app.state.dialogs.hardcopy.region,
            HardcopyRegion::Preview,
            "the switch did not leave the setup region"
        );

        let button = studio.control(Role::Button, "Export hardcopy");
        let _ = studio.pass(click(button.center()));

        assert_eq!(studio.published_resolution(), Some(300));
        assert_eq!(studio.app.state.dialogs.hardcopy.raster_dpi_draft, "300");
    }

    /// Settling belongs on the path every pass takes, not on the surfaces a
    /// composition may choose not to draw. Whatever the rail, the tab strip and
    /// the region switch select — in either composition — a pass has to leave
    /// the draft holding the resolution the field is showing.
    #[test]
    fn no_section_or_region_can_strand_a_typed_resolution() {
        for mut studio in [Studio::wide(), Studio::narrow()] {
            let composition = studio.viewport;
            for section in HardcopySection::ALL {
                for region in [HardcopyRegion::Setup, HardcopyRegion::Preview] {
                    studio.app.state.dialogs.hardcopy.format = OutputFormat::Png { dpi: 150 };
                    studio.app.state.dialogs.hardcopy.raster_dpi_draft = "300".to_owned();
                    studio.app.state.dialogs.hardcopy.section = section;
                    studio.app.state.dialogs.hardcopy.region = region;
                    let _ = studio.pass(Vec::new());
                    assert_eq!(
                        studio.app.state.dialogs.hardcopy.format.raster_dpi(),
                        Some(300),
                        "{composition:?} {section:?} {region:?} left a typed resolution uncommitted"
                    );
                }
            }
        }
    }

    /// The largest page a standard media offers, which is what pulls the
    /// ceiling well under both the contract's range and the default a raster
    /// target would otherwise commit.
    fn ceiling_bound_media(studio: &mut Studio) -> u16 {
        studio.app.state.dialogs.hardcopy.paper = PaperDraft::Standard(StandardPaper::A0);
        studio.app.state.dialogs.hardcopy.refresh_preview();
        let _ = studio.pass(Vec::new());
        let plan = studio
            .app
            .state
            .dialogs
            .hardcopy
            .preview_plan
            .clone()
            .expect("A0 is a legal page");
        let ceiling = max_raster_dpi(&plan, OutputFormat::Png { dpi: 300 })
            .expect("an A0 page rasters at some resolution");
        assert!(
            ceiling < DEFAULT_RASTER_DPI,
            "this page rasters to {ceiling} dpi, which is not under the default a target commits"
        );
        ceiling
    }

    /// A raster target is selected here and refused nowhere else until the
    /// render round trip, so the entries the list offers carry the resolution
    /// they would commit — not the default, which on a browser's raster budget
    /// is above the ceiling for an ordinary Letter page. The two formats charge
    /// different bytes per pixel, so each entry is bounded by its own ceiling
    /// rather than by one number for both.
    #[test]
    fn the_target_list_offers_no_resolution_the_page_it_is_on_refuses() {
        let mut studio = Studio::wide();
        let ceiling = ceiling_bound_media(&mut studio);
        let draft = &studio.app.state.dialogs.hardcopy;

        let png = bounded_raster(
            draft,
            OutputFormat::Png {
                dpi: DEFAULT_RASTER_DPI,
            },
        );
        let tiff = bounded_raster(
            draft,
            OutputFormat::Tiff {
                dpi: DEFAULT_RASTER_DPI,
            },
        );
        assert_eq!(png, OutputFormat::Png { dpi: ceiling });
        assert!(
            tiff.raster_dpi().is_some_and(|dpi| dpi < ceiling),
            "TIFF holds a third more bytes per pixel and cannot share PNG's ceiling: {tiff:?}"
        );
        // A resolution already under the ceiling is offered untouched, and a
        // vector target has no resolution to bound.
        assert_eq!(
            bounded_raster(draft, OutputFormat::Png { dpi: 72 }),
            OutputFormat::Png { dpi: 72 }
        );
        assert_eq!(
            bounded_raster(draft, OutputFormat::SvgVector),
            OutputFormat::SvgVector
        );
    }

    /// The ceiling belongs to the page, so every control that changes the page
    /// moves it — under a resolution that was legal when it was committed. The
    /// draft has to be brought back inside it before anything is published from
    /// it, and what is published has to be what the surface was reporting.
    #[test]
    fn a_page_that_lowers_the_ceiling_lowers_the_resolution_committed_with_it() {
        let mut studio = Studio::wide();
        studio.app.state.dialogs.hardcopy.format = OutputFormat::Png {
            dpi: DEFAULT_RASTER_DPI,
        };
        studio.app.state.dialogs.hardcopy.raster_dpi_draft = DEFAULT_RASTER_DPI.to_string();
        studio.app.state.dialogs.hardcopy.refresh_preview();
        let _ = studio.pass(Vec::new());
        assert_eq!(
            studio.app.state.dialogs.hardcopy.format,
            OutputFormat::Png {
                dpi: DEFAULT_RASTER_DPI
            },
            "the default resolution is legal on the page the studio opened on"
        );

        let ceiling = ceiling_bound_media(&mut studio);
        assert_eq!(
            studio.app.state.dialogs.hardcopy.format,
            OutputFormat::Png { dpi: ceiling },
            "the wider media left a resolution behind that this host cannot raster"
        );
        assert_eq!(
            output_summary(studio.app.state.dialogs.hardcopy.format),
            format!("PNG · {ceiling} dpi")
        );

        let button = studio.control(Role::Button, "Export hardcopy");
        let _ = studio.pass(click(button.center()));
        assert_eq!(studio.published_resolution(), Some(ceiling));
    }

    /// An entry that names no resolution cannot be published, so an unfocused
    /// field states the one in force rather than a number that will not be used.
    #[test]
    fn a_field_nobody_is_editing_states_the_resolution_in_force() {
        let mut studio = Studio::wide();
        studio.app.state.dialogs.hardcopy.raster_dpi_draft = "3oo".to_owned();
        studio.app.state.dialogs.hardcopy.section = HardcopySection::Page;
        let _ = studio.pass(Vec::new());

        assert_eq!(studio.app.state.dialogs.hardcopy.raster_dpi_draft, "150");
        assert_eq!(
            studio.app.state.dialogs.hardcopy.format,
            OutputFormat::Png { dpi: 150 }
        );
    }

    /// A resolution being typed is not a resolution yet: `60` is a legal prefix
    /// of `600`, so nothing commits while the caret is still in the field.
    #[test]
    fn a_resolution_still_being_typed_commits_nothing() {
        let mut studio = Studio::wide();
        studio.type_resolution("1200");
        assert_eq!(
            studio.app.state.dialogs.hardcopy.format,
            OutputFormat::Png { dpi: 150 },
            "the field committed while it was still being typed into"
        );
        let _ = studio.pass(Vec::new());
        assert_eq!(
            studio.app.state.dialogs.hardcopy.format,
            OutputFormat::Png { dpi: 150 },
            "the field committed while it still held the caret"
        );
    }

    /// A blank drawing sheet is the smallest source that resolves into a real
    /// scene, which is what makes the font settings observable in an artifact
    /// rather than only in the draft.
    fn sheet_draft() -> HardcopyDialogState {
        let identity = HardcopySourceIdentity::try_new(
            "hardcopy-font-capabilities",
            HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(0x4834_0101)).unwrap(),
            ObjectRevision::INITIAL,
            "Font capabilities",
        )
        .unwrap();
        let sheet = resolve_blank_schematic_sheet_with_format(
            identity,
            HardcopyScope::CurrentSheet,
            Some(&SchematicSheetFormat::default()),
        )
        .unwrap();
        let mut draft = HardcopyDialogState::default();
        draft
            .open_resolved(HardcopyWorkflow::Export, sheet, None)
            .expect("blank drawing sheet opens");
        draft.format = OutputFormat::SvgVector;
        draft.refresh_preview();
        draft
    }

    fn metadata(lettered: bool) -> HardcopySceneMetadata {
        let mut metadata =
            HardcopySceneMetadata::try_new("Font capabilities", "RSpice hardcopy tests").unwrap();
        metadata.set_publication_timestamp(
            HardcopyPublicationTimestamp::try_new(2026, 8, 11, 9, 0, 0).unwrap(),
        );
        if lettered {
            metadata
                .set_header_lines(vec!["Font capabilities".to_owned()])
                .unwrap();
        }
        metadata
    }

    fn artifact(draft: &HardcopyDialogState, metadata: HardcopySceneMetadata) -> Vec<u8> {
        let setup = draft.build_setup().expect("the draft is a legal setup");
        let document = draft.resolved_document.as_ref().expect("resolved source");
        let extent = document
            .content_extent_for_setup(setup.schematic())
            .expect("resolved extent");
        let plan = crate::hardcopy::HardcopyPlan::compile_with_id(
            HardcopyPlanId::try_from_uuid(uuid::Uuid::from_u128(0x4834_0102)).unwrap(),
            document.authority().clone(),
            setup,
            extent,
        )
        .expect("the draft compiles a plan");
        HardcopyRenderer::render_resolved(&plan, document, metadata)
            .expect("the plan publishes")
            .parts()
            .iter()
            .flat_map(|part| part.bytes().to_vec())
            .collect()
    }

    /// Drive the identity panel the way an operator does, and return what the
    /// two font controls left in the draft.
    struct FontControls {
        ctx: Context,
        draft: HardcopyDialogState,
    }

    impl FontControls {
        fn new(draft: HardcopyDialogState) -> Self {
            let ctx = Context::default();
            ctx.enable_accesskit();
            crate::ui::Theme::default().apply(&ctx);
            Self { ctx, draft }
        }

        fn pass(&mut self, events: Vec<egui::Event>) -> egui::FullOutput {
            let draft = &mut self.draft;
            self.ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(620.0, 420.0))),
                    events,
                    ..Default::default()
                },
                |ui| identity_panel(ui, draft),
            )
        }

        fn click_checkbox(&mut self, label: &str) {
            let output = self.pass(Vec::new());
            let rect = controls(&output)
                .into_iter()
                .find(|(role, name, _)| *role == Role::CheckBox && name == label)
                .unwrap_or_else(|| panic!("the identity panel draws no checkbox {label:?}"))
                .2;
            let _ = self.pass(click(rect.center()));
        }

        fn state(&mut self) -> (bool, bool) {
            let _ = self.pass(Vec::new());
            (self.draft.embed_fonts, self.draft.searchable_text)
        }
    }

    /// A row that leads nowhere is not drawn. The driver dialog exists on one
    /// host; on the browser, tray, duplex and collation belong to its own print
    /// dialog, and a dimmed row advertising them there reads as a capability
    /// withheld rather than one the host never had. The rest of the section is
    /// held to the same standard as the sheet section, which is absent rather
    /// than inert when there is no authored sheet.
    #[test]
    fn the_printer_properties_row_is_drawn_only_where_it_leads_somewhere() {
        let mut draft = sheet_draft();
        draft.workflow = HardcopyWorkflow::Print;
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(620.0, 620.0))),
                ..Default::default()
            },
            |ui| {
                let _ = output_panel(ui, &mut draft);
            },
        );
        let row = |wanted: &str| {
            controls(&output)
                .into_iter()
                .any(|(role, name, _)| role == Role::Button && name == wanted)
        };
        assert_eq!(row("Printer properties…"), printer_properties_reachable());
        // The gate is the driver boundary, not the section: the mapping flow is
        // reachable everywhere and its row is always there to prove the panel
        // was drawn at all.
        assert!(row("Layer, trace, and marker mapping…"));
    }

    /// `RenderSetup::validate` accepts embedded fonts on their own, refuses
    /// searchable text without them, and refuses searchable text on a raster
    /// target — three legal vector states, not two. The single checkbox this
    /// replaced could reach two of them and read a third back as unchecked.
    #[test]
    fn embedded_fonts_and_searchable_text_are_two_decisions() {
        let mut controls = FontControls::new(sheet_draft());
        controls.draft.embed_fonts = false;
        controls.draft.searchable_text = false;

        controls.click_checkbox("Searchable text");
        assert_eq!(
            controls.state(),
            (false, false),
            "searchable text was offered without the typefaces it needs"
        );

        controls.click_checkbox("Embed fonts");
        assert_eq!(
            controls.state(),
            (true, false),
            "embedding fonts alone is a state the contract admits"
        );
        assert!(controls.draft.build_setup().is_ok());

        controls.click_checkbox("Searchable text");
        assert_eq!(controls.state(), (true, true));
        assert!(controls.draft.build_setup().is_ok());

        controls.click_checkbox("Embed fonts");
        assert_eq!(
            controls.state(),
            (false, false),
            "dropping the typefaces left text the contract refuses to keep live"
        );
        assert!(controls.draft.build_setup().is_ok());
    }

    /// A raster target has neither capability, and the archive format has one
    /// of them by contract. Both are stated at the control rather than accepted
    /// and refused on publication.
    #[test]
    fn a_target_that_cannot_honour_a_font_capability_does_not_offer_it() {
        let mut controls = FontControls::new(sheet_draft());
        controls.draft.format = OutputFormat::Png { dpi: 300 };
        controls.draft.embed_fonts = true;
        controls.draft.searchable_text = true;
        assert_eq!(
            controls.state(),
            (false, false),
            "a raster target kept a font capability it cannot carry"
        );
        controls.click_checkbox("Embed fonts");
        controls.click_checkbox("Searchable text");
        assert_eq!(controls.state(), (false, false));

        controls.draft.format = OutputFormat::PdfA;
        controls.draft.embed_fonts = true;
        controls.draft.searchable_text = true;
        controls.click_checkbox("Embed fonts");
        assert_eq!(
            controls.state(),
            (true, true),
            "PDF/A let its typefaces be dropped"
        );
        controls.click_checkbox("Searchable text");
        assert_eq!(
            controls.state(),
            (true, false),
            "PDF/A pinned a decision its contract leaves open"
        );
        // Leaving that decision open is only honest if the writer agrees: an
        // archive whose letters are outlined has to publish, and has to have
        // outlined them.
        let archive = artifact(&controls.draft, metadata(true));
        let parsed = lopdf::Document::load_mem(&archive).expect("valid PDF/A");
        let pages = parsed.get_pages().keys().copied().collect::<Vec<_>>();
        assert!(
            !parsed
                .extract_text(&pages)
                .unwrap_or_default()
                .contains("Font capabilities"),
            "the archive kept text live that the control said it would outline"
        );
    }

    /// Comparing whole artifacts would prove nothing: both vector writers stamp
    /// the plan digest, so anything that reaches the sealed setup changes the
    /// bytes. Each control is checked against the specific property it governs
    /// with the other one held still.
    #[test]
    fn each_font_control_reaches_the_artifact_on_its_own() {
        // An SVG cannot outline text, so dropping the typefaces is only legal
        // once the page carries none.
        let unlettered = || {
            let mut draft = sheet_draft();
            draft.include_sheet_paper = false;
            draft.include_sheet_border = false;
            draft.include_sheet_title_block = false;
            draft.include_sheet_zones = false;
            draft.include_schematic_grid = false;
            draft.crop_marks = false;
            draft.embed_fonts = false;
            draft.searchable_text = false;
            draft
        };
        let mut linked = FontControls::new(unlettered());
        assert_eq!(linked.state(), (false, false));
        let mut embedded = FontControls::new(unlettered());
        embedded.click_checkbox("Embed fonts");
        assert_eq!(embedded.state(), (true, false));

        assert!(
            String::from_utf8_lossy(&artifact(&embedded.draft, metadata(false)))
                .contains("@font-face"),
            "embedding fonts wrote no typeface into the artifact"
        );
        assert!(
            !String::from_utf8_lossy(&artifact(&linked.draft, metadata(false)))
                .contains("@font-face"),
            "the artifact carries a typeface nobody asked it to embed"
        );

        // Searchable text is the PDF writer's outlining decision, so it is read
        // back the way a reader would: by extracting the text.
        let lettered = || {
            let mut draft = sheet_draft();
            draft.format = OutputFormat::PdfVector;
            draft.embed_fonts = true;
            draft.searchable_text = false;
            draft
        };
        let mut outlined = FontControls::new(lettered());
        assert_eq!(outlined.state(), (true, false));
        let mut searchable = FontControls::new(lettered());
        searchable.click_checkbox("Searchable text");
        assert_eq!(searchable.state(), (true, true));

        let extract = |bytes: &[u8]| {
            let parsed = lopdf::Document::load_mem(bytes).expect("valid PDF");
            let pages = parsed.get_pages().keys().copied().collect::<Vec<_>>();
            parsed.extract_text(&pages).unwrap_or_default()
        };
        assert!(
            extract(&artifact(&searchable.draft, metadata(true))).contains("Font capabilities"),
            "a searchable PDF has no extractable text"
        );
        assert!(
            !extract(&artifact(&outlined.draft, metadata(true))).contains("Font capabilities"),
            "outlining text left extractable text behind"
        );
    }
}
