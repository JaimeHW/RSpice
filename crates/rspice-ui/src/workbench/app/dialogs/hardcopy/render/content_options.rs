//! The section panels of the hardcopy studio: what to publish and how.
//!
//! Each function owns exactly one section of the main page and draws into the
//! editor column the rail selected. None of them refreshes the preview
//! themselves — the body compares the whole draft before and after the frame,
//! so moving a control from one section to another cannot quietly stop it
//! invalidating the plan. Options a target cannot honour are disabled with the
//! reason attached rather than offered and then refused: transparency needs a
//! vector artifact, color needs a printer that reports it, and a raster
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
                    _ => "Manual rows and columns".to_owned(),
                };
                egui::ComboBox::from_id_salt("hardcopy-pagination")
                    .selected_text(selected)
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut tiling_kind, 0, automatic);
                        ui.selectable_value(&mut tiling_kind, 1, "Single page");
                        ui.selectable_value(&mut tiling_kind, 2, "Manual rows and columns");
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
        true,
        "",
    ) {
        action = BodyAction::CustomPaper;
    }
    action
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
    if draft.format == OutputFormat::PdfA {
        draft.embed_fonts = true;
        draft.searchable_text = true;
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
        true,
        "",
    ) {
        action = BodyAction::PrintMapping;
    }
    if draft.workflow != HardcopyWorkflow::Export {
        ui.add_space(6.0);
        let (enabled, hint) = printer_properties_availability();
        if action_row(
            ui,
            "Printer properties…",
            "tray, duplex, resolution and collation",
            enabled,
            hint,
        ) {
            action = BodyAction::PrinterProperties;
        }
    }
    action
}

/// Where the driver dialog can be reached from, and what to say where it
/// cannot. The browser has its own print dialog and no driver boundary to
/// offer; other desktops have one RSpice does not yet speak to.
#[cfg(target_os = "windows")]
const fn printer_properties_availability() -> (bool, &'static str) {
    (true, "")
}

#[cfg(all(not(target_os = "windows"), target_arch = "wasm32"))]
const fn printer_properties_availability() -> (bool, &'static str) {
    (
        false,
        "Printer capabilities are selected in the browser print dialog.",
    )
}

#[cfg(all(not(target_os = "windows"), not(target_arch = "wasm32")))]
const fn printer_properties_availability() -> (bool, &'static str) {
    (
        false,
        "System printer driver properties are available on Windows desktop.",
    )
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
                // silently reset it.
                let dpi = draft.format.raster_dpi().unwrap_or(DEFAULT_RASTER_DPI);
                for (format, label) in [
                    (OutputFormat::PdfVector, "PDF · vector"),
                    (OutputFormat::PdfA, "PDF/A · vector"),
                    (OutputFormat::SvgVector, "SVG · vector"),
                    (OutputFormat::Png { dpi }, "PNG · raster"),
                    (OutputFormat::Tiff { dpi }, "TIFF · raster"),
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
    let mut fonts = draft.embed_fonts && draft.searchable_text;
    ui.add_enabled_ui(draft.format.is_vector(), |ui| {
        if ui
            .checkbox(&mut fonts, "Embed fonts and preserve searchable text")
            .changed()
        {
            draft.embed_fonts = fonts;
            draft.searchable_text = fonts;
        }
    });
    if !draft.format.is_vector() {
        draft.embed_fonts = false;
        draft.searchable_text = false;
    }
    ui.add_space(6.0);
    panel_note(
        ui,
        match draft.format {
            OutputFormat::PdfA => {
                "PDF/A requires embedded fonts and searchable text; the archive contract sets both."
            }
            format if format.is_vector() => {
                "Embedded fonts make the artifact self-contained and its text selectable."
            }
            _ => {
                "A raster target has no text to embed or search; the option applies to vector formats only."
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
    let ceiling = draft
        .preview_plan
        .as_ref()
        .and_then(|plan| max_raster_dpi(plan, draft.format));
    let committed = draft.format.raster_dpi().unwrap_or(DEFAULT_RASTER_DPI);

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
        let committing = response.lost_focus();
        if committing {
            match parse_raster_dpi(&draft.raster_dpi_draft, ceiling) {
                Ok(dpi) => draft.format = draft.format.with_raster_dpi(dpi),
                Err(_) => draft.raster_dpi_draft = committed.to_string(),
            }
        }

        let (message, tone) = raster_resolution_note(&draft.raster_dpi_draft, ceiling, committed);
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

fn raster_resolution_note(text: &str, ceiling: Option<u16>, committed: u16) -> (String, NoteTone) {
    let warn: NoteTone = |ui| ui.visuals().warn_fg_color;
    let dim: NoteTone = |ui| Tokens::get(ui.ctx()).color.text_dim;
    match parse_raster_dpi(text, ceiling) {
        Err(reason) => (reason, warn),
        Ok(_) => match ceiling {
            // The committed value can outlive the page it was valid for: a
            // wider media or a coarser tiling lowers the ceiling underneath it.
            Some(ceiling) if committed > ceiling => (
                format!("{committed} dpi no longer fits this page — up to {ceiling} dpi"),
                warn,
            ),
            Some(ceiling) => (format!("up to {ceiling} dpi for this page"), dim),
            None => (format!("{MIN_RASTER_DPI}–{MAX_RASTER_DPI} dpi"), dim),
        },
    }
}
