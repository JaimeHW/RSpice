//! Choosing what to print and how: content selection and render options.
//!
//! Both forms compare their inputs before and after each frame and refresh the
//! preview only when something actually changed, so the preview stays exactly
//! in step with the settings without re-rendering on every repaint. Page
//! ranges are validated against the resolved content, so a range that selects
//! nothing is reported rather than silently producing an empty job.

use super::*;

pub(super) fn content_pagination(
    ui: &mut Ui,
    draft: &mut HardcopyDialogState,
) -> Option<BodyAction> {
    let before = content_inputs(draft);
    let mut action = None;
    Frame::NONE
        .inner_margin(Margin {
            left: 11,
            right: 11,
            top: 0,
            bottom: 10,
        })
        .show(ui, |ui| {
            section_header(ui, "Content and pagination", None);
            ui.spacing_mut().item_spacing.y = 0.0;
            let active_kind = draft.source.as_ref().map(|source| source.document_kind());
            let active_key = draft
                .resolved_document
                .as_ref()
                .map(|source| source.source_key().to_owned());
            let (narrow_field, wide_field) = form_grid_widths(ui);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 11.0;
                ui.allocate_ui_with_layout(
                    vec2(narrow_field, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        field(ui, "Document type", |ui| {
                            egui::ComboBox::from_id_salt("hardcopy-document-type")
                                .selected_text(
                                    active_kind.map_or("No active type", document_kind_label),
                                )
                                .width(ui.available_width())
                                .show_ui(ui, |ui| {
                                    for kind in [
                        crate::hardcopy::HardcopyDocumentKind::SchematicOrSymbol,
                        crate::hardcopy::HardcopyDocumentKind::LayoutWithLayerLegend,
                        crate::hardcopy::HardcopyDocumentKind::PlotOrWorksheet,
                        crate::hardcopy::HardcopyDocumentKind::Report,
                    ] {
                        let candidate = draft.source_candidates.iter().find(|candidate| {
                            candidate.document_kind == kind && candidate.availability.is_available()
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
                            action = Some(BodyAction::SelectSource { source_key, scope });
                        } else if candidate.is_none() {
                            response.on_disabled_hover_text(
                                "No authenticated retained document of this type is available.",
                            );
                        }
                    }
                                });
                        });
                    },
                );
                ui.allocate_ui_with_layout(
                    vec2(wide_field, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
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
                                            && (!active
                                                || active_key.as_deref()
                                                    != Some(source_key.as_str()))
                                        {
                                            action = Some(BodyAction::SelectSource {
                                                source_key,
                                                scope,
                                            });
                                        } else if choice.is_none() {
                                            response.on_disabled_hover_text(
                                "This document family does not retain that publication extent.",
                            );
                                        }
                                    }
                                    let named_sets = named_source_choices(
                                        &draft.source_candidates,
                                        active_candidate,
                                    );
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
                                            let active = active_key.as_deref()
                                                == Some(source_key.as_str())
                                                && active_scope == Some(&scope);
                                            if ui
                                                .add(egui::Button::selectable(
                                                    active,
                                                    format!("Named print set · {name}"),
                                                ))
                                                .clicked()
                                                && !active
                                            {
                                                action = Some(BodyAction::SelectSource {
                                                    source_key,
                                                    scope,
                                                });
                                            }
                                        }
                                    }
                                });
                        });
                    },
                );
            });
            ui.spacing_mut().item_spacing.y = 6.0;
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 11.0;
                ui.allocate_ui_with_layout(
                    vec2(narrow_field, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        field(ui, "Paper", |ui| {
                            let selected = paper_label(&draft.paper);
                            egui::ComboBox::from_id_salt("hardcopy-paper")
                                .selected_text(selected)
                                .width(ui.available_width())
                                .show_ui(ui, |ui| {
                                    for paper in [
                                        StandardPaper::Letter,
                                        StandardPaper::A4,
                                        StandardPaper::A3,
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
                        });
                    },
                );
                ui.allocate_ui_with_layout(
                    vec2(wide_field, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
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
                    },
                );
            });
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 11.0;
                ui.allocate_ui_with_layout(
                    vec2(narrow_field, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
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
                                            egui::TextEdit::singleline(
                                                &mut draft.custom_scale_percent,
                                            )
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
                    },
                );
                ui.allocate_ui_with_layout(
                    vec2(wide_field, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
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
                                    ui.selectable_value(
                                        &mut tiling_kind,
                                        2,
                                        "Manual rows and columns",
                                    );
                                    if tiling_kind == 2 {
                                        ui.separator();
                                        ui.label("Manual page grid");
                                        ui.horizontal(|ui| {
                                            ui.add(
                                                egui::TextEdit::singleline(
                                                    &mut draft.manual_columns,
                                                )
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
                    },
                );
            });
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 11.0;
                ui.allocate_ui_with_layout(
                    vec2(narrow_field, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        length_field(ui, "Tile overlap", &mut draft.overlap, draft.display_unit);
                    },
                );
                ui.allocate_ui_with_layout(
                    vec2(wide_field, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
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
                    },
                );
            });
        });
    if draft.format == OutputFormat::NativePrinter
        && (before.paper != draft.paper || before.orientation != draft.orientation)
    {
        reconcile_native_printer_job(draft);
    }
    if content_inputs(draft) != before {
        draft.refresh_preview();
    }
    action
}

pub(super) fn rendering_options(ui: &mut Ui, draft: &mut HardcopyDialogState) {
    let before = rendering_inputs(draft);
    Frame::NONE
        .inner_margin(Margin {
            left: 11,
            right: 11,
            top: 0,
            bottom: 10,
        })
        .show(ui, |ui| {
            section_header(ui, "Rendering and identity", None);
            ui.spacing_mut().item_spacing.y = 0.0;
            let old_format = draft.format;
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
                        for (format, label) in [
                            (OutputFormat::PdfVector, "PDF · vector"),
                            (OutputFormat::PdfA, "PDF/A · vector"),
                            (OutputFormat::SvgVector, "SVG · vector"),
                            (OutputFormat::Png { dpi: 600 }, "PNG · 600 dpi"),
                            (OutputFormat::Tiff { dpi: 600 }, "TIFF · 600 dpi"),
                        ] {
                            ui.selectable_value(&mut draft.format, format, label);
                        }
                    });
            });
            ui.spacing_mut().item_spacing.y = 6.0;
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
            if draft.background == BackgroundMode::Transparent
                && (!draft.format.is_vector()
                    || matches!(
                        draft.format,
                        OutputFormat::NativePrinter | OutputFormat::BrowserPrintDocument
                    ))
            {
                draft.background = BackgroundMode::White;
            }
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
        });
    if rendering_inputs(draft) != before {
        draft.refresh_preview();
    }
}
