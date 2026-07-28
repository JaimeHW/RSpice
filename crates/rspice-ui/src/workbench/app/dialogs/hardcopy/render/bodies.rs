//! The secondary hardcopy bodies: printer properties, custom paper, mapping.
//!
//! Each edits one part of the plan and refreshes the preview when it changes,
//! so what is shown always corresponds to the settings currently entered. The
//! custom-paper form works in the plan's exact micrometre units rather than
//! the display unit, so switching units never perturbs a stored size.

use super::*;

pub(super) fn printer_properties_body(ui: &mut Ui, draft: &mut HardcopyDialogState) -> BodyAction {
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    #[cfg(target_os = "windows")]
    let mut action = BodyAction::None;
    #[cfg(not(target_os = "windows"))]
    let action = BodyAction::None;
    dialog_split(ui, |ui, column| match column {
        0 => {
            #[cfg(target_os = "windows")]
            {
                let selected_name = draft
                    .printer_capabilities
                    .as_ref()
                    .map_or("No printer selected", |value| value.display_name());
                let mut selected_device = None;
                field(ui, "Printer", |ui| {
                    egui::ComboBox::from_id_salt("hardcopy-printer")
                        .selected_text(selected_name)
                        .width(ui.available_width())
                        .show_ui(ui, |ui| {
                            if let Some(report) = draft.printer_report.as_ref() {
                                for entry in report.printers() {
                                    if ui
                                        .selectable_label(
                                            draft.printer_id == entry.capabilities().device_id(),
                                            entry.capabilities().display_name(),
                                        )
                                        .clicked()
                                    {
                                        selected_device =
                                            Some(entry.capabilities().device_id().to_owned());
                                    }
                                }
                            }
                        });
                });
                if let Some(device_id) = selected_device {
                    action = BodyAction::SelectPrinter(device_id);
                }
                if let (Some(capabilities), Some(current_job)) = (
                    draft.printer_capabilities.clone(),
                    draft.printer_job.clone(),
                ) {
                    let paper_id = current_job.selected_paper_id().to_owned();
                    let mut media = current_job.media_source().clone();
                    field(ui, "Media source", |ui| {
                        egui::ComboBox::from_id_salt("hardcopy-printer-media")
                            .selected_text(media_label(&media))
                            .width(ui.available_width())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut media,
                                    crate::hardcopy::PrinterMediaSource::AutomaticCompatibleTray,
                                    "Automatic compatible tray",
                                );
                                if capabilities
                                    .trays()
                                    .iter()
                                    .any(|tray| matches!(tray.platform_id(), 4 | 6))
                                {
                                    ui.selectable_value(
                                        &mut media,
                                        crate::hardcopy::PrinterMediaSource::ManualFeed,
                                        "Manual feed",
                                    );
                                }
                                for tray in capabilities.trays() {
                                    ui.selectable_value(
                                        &mut media,
                                        crate::hardcopy::PrinterMediaSource::NamedTray(
                                            tray.display_name().to_owned(),
                                        ),
                                        tray.display_name(),
                                    );
                                }
                            });
                    });
                    let mut dpi = current_job.resolution_dpi();
                    field(ui, "Resolution", |ui| {
                        egui::ComboBox::from_id_salt("hardcopy-printer-resolution")
                            .selected_text(format!("{dpi} dpi"))
                            .width(ui.available_width())
                            .show_ui(ui, |ui| {
                                for resolution in
                                    capabilities.resolutions().iter().filter(|resolution| {
                                        resolution.horizontal_dpi() == resolution.vertical_dpi()
                                    })
                                {
                                    let value = resolution.horizontal_dpi();
                                    ui.selectable_value(&mut dpi, value, format!("{value} dpi"));
                                }
                            });
                    });
                    let mut duplex = current_job.duplex();
                    field(ui, "Duplex", |ui| {
                        egui::ComboBox::from_id_salt("hardcopy-printer-duplex")
                            .selected_text(duplex_label(duplex))
                            .width(ui.available_width())
                            .show_ui(ui, |ui| {
                                for value in capabilities.duplex_modes() {
                                    ui.selectable_value(&mut duplex, *value, duplex_label(*value));
                                }
                            });
                    });
                    if paper_id != current_job.selected_paper_id()
                        || media != *current_job.media_source()
                        || dpi != current_job.resolution_dpi()
                        || duplex != current_job.duplex()
                    {
                        apply_printer_job(
                            draft,
                            &capabilities,
                            &paper_id,
                            media,
                            dpi,
                            duplex,
                            current_job.copies(),
                            current_job.collate(),
                        );
                    }
                }
            }
            #[cfg(not(target_os = "windows"))]
            ui.label("Printer selection and properties are owned by the browser print dialog.");
        }
        _ => {
            if let (Some(capabilities), Some(job)) = (
                draft.printer_capabilities.as_ref(),
                draft.printer_job.as_ref(),
            ) {
                let (_, _, printable_width, printable_height) =
                    job.raster_geometry().printable_rect_px();
                property_list(ui, |ui| {
                    property(
                        ui,
                        "Printable area",
                        &format!(
                            "{:.2} × {:.2} in",
                            printable_width as f64 / f64::from(job.resolution_dpi()),
                            printable_height as f64 / f64::from(job.resolution_dpi())
                        ),
                    );
                    property(
                        ui,
                        "Color",
                        if capabilities.supports_color() {
                            "Driver color output"
                        } else {
                            "Monochrome only"
                        },
                    );
                    property(
                        ui,
                        "Print pipeline",
                        &format!("RSpice raster · {} dpi", job.resolution_dpi()),
                    );
                    property(ui, "Driver", capabilities.driver_name());
                });
            } else {
                note_block(
                    ui,
                    "Capabilities unavailable",
                    "Select an authenticated system printer to inspect its exact driver boundary.",
                );
            }
            #[cfg(target_os = "windows")]
            if Button::new("Open operating-system driver properties…")
                .enabled(!draft.printer_id.is_empty())
                .show(ui)
                .clicked()
            {
                action = BodyAction::DriverProperties;
            }
        }
    });
    action
}

pub(super) fn custom_paper_body(ui: &mut Ui, draft: &mut HardcopyDialogState) {
    let before = content_inputs(draft);
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    if !matches!(draft.paper, PaperDraft::Custom { .. }) {
        draft.paper = custom_from_current(draft);
    }
    dialog_split(ui, |column, column_index| match column_index {
        0 => {
            if let PaperDraft::Custom {
                name,
                width,
                height,
            } = &mut draft.paper
            {
                field(column, "Paper name", |ui| {
                    ui.text_edit_singleline(name);
                });
                length_field(column, "Width", width, draft.display_unit);
                length_field(column, "Height", height, draft.display_unit);
            }
            let old_unit = draft.display_unit;
            enum_combo(
                column,
                "Units",
                "hardcopy-units",
                &mut draft.display_unit,
                &[
                    (LengthUnit::Inches, "inches"),
                    (LengthUnit::Millimetres, "millimeters"),
                ],
            );
            if draft.display_unit != old_unit {
                let target = draft.display_unit;
                draft.display_unit = old_unit;
                if let Err(error) = draft.set_display_unit(target) {
                    draft.error = Some(error.to_string());
                }
            }
            enum_combo(
                column,
                "Orientation",
                "hardcopy-custom-orientation",
                &mut draft.orientation,
                &[
                    (Orientation::Landscape, "Landscape"),
                    (Orientation::Portrait, "Portrait"),
                ],
            );
        }
        _ => {
            length_field(
                column,
                "Top margin",
                &mut draft.margin_top,
                draft.display_unit,
            );
            length_field(
                column,
                "Right margin",
                &mut draft.margin_right,
                draft.display_unit,
            );
            length_field(
                column,
                "Bottom margin",
                &mut draft.margin_bottom,
                draft.display_unit,
            );
            length_field(
                column,
                "Left margin",
                &mut draft.margin_left,
                draft.display_unit,
            );
            field(column, "Bleed / crop", |ui| {
                let bleed_value = if draft.bleed.trim() == "0" { 0 } else { 1 };
                let mut choice = bleed_value;
                egui::ComboBox::from_id_salt("hardcopy-custom-bleed")
                    .selected_text(if choice == 0 {
                        "None"
                    } else if draft.display_unit == LengthUnit::Inches {
                        "0.125 in bleed"
                    } else {
                        "3.175 mm bleed"
                    })
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut choice, 0, "None");
                        ui.selectable_value(
                            &mut choice,
                            1,
                            if draft.display_unit == LengthUnit::Inches {
                                "0.125 in bleed"
                            } else {
                                "3.175 mm bleed"
                            },
                        );
                    });
                draft.bleed = match (choice, draft.display_unit) {
                    (0, _) => "0".to_owned(),
                    (_, LengthUnit::Inches) => "0.125".to_owned(),
                    (_, LengthUnit::Millimetres) => "3.175".to_owned(),
                };
            });
            let native_job_ok = reconcile_native_printer_job(draft);
            let native_target = draft.format == OutputFormat::NativePrinter;
            // Page validity is a semantic setup/plan contract. It must not
            // flicker while the asynchronous preview raster is being replaced,
            // and an unrelated missing source must not make valid physical
            // dimensions appear invalid in this subflow.
            let setup = draft.build_setup().ok();
            let exact_plan_ok = match (
                setup.as_ref(),
                draft.source.as_ref(),
                draft.preview_plan.as_ref(),
            ) {
                (Some(setup), Some(_), Some(plan)) => plan.setup() == setup,
                (Some(_), None, _) => true,
                _ => false,
            };
            let capability_ok = native_job_ok && setup.is_some() && exact_plan_ok;
            workflow_status(
                column,
                capability_ok,
                if capability_ok {
                    if native_target {
                        "Within printer capabilities"
                    } else {
                        "Page setup valid"
                    }
                } else {
                    if native_target {
                        "Outside current capabilities"
                    } else {
                        "Page setup invalid"
                    }
                },
                if capability_ok {
                    if native_target {
                        "Header, provenance footer, legends, and registration marks fit the printer safe area."
                    } else {
                        "Header, provenance footer, legends, and registration marks fit the configured page."
                    }
                } else {
                    if native_target {
                        "Adjust the page or margins to a mode supported by the current printer."
                    } else {
                        "Adjust the page or margins until the exact setup validates."
                    }
                },
            );
        }
    });
    if content_inputs(draft) != before {
        draft.refresh_preview();
    }
}

pub(super) fn split_column<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::NONE
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 10.0;
            add_contents(ui)
        })
        .inner
}

pub(super) fn print_mapping_body(ui: &mut Ui, draft: &mut HardcopyDialogState) {
    let mut mapping_changed = false;
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    let t = Tokens::get(ui.ctx());
    let mut changed_entry = None;
    egui::ScrollArea::horizontal()
        .id_salt("hardcopy-print-mapping-horizontal")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(660.0);
            let header = Frame::NONE.fill(t.color.bg_panel_2).show(ui, |ui| {
                let width = ui.available_width();
                let column_width = width / 5.0;
                ui.set_width(width);
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    for label in [
                        "ENGINEERING OBJECT",
                        "SCREEN STYLE",
                        "PRINT COLOR",
                        "LINE / FILL",
                        "LEGEND",
                    ] {
                        ui.allocate_ui_with_layout(
                            vec2(column_width, 28.0),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                ui.set_width(column_width);
                                mapping_cell(ui, 28.0, |ui| {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(label)
                                                .strong()
                                                .size(tokens::FS_0)
                                                .color(ui.visuals().weak_text_color()),
                                        )
                                        .truncate(),
                                    );
                                });
                            },
                        );
                    }
                });
            });
            ui.painter().hline(
                header.response.rect.x_range(),
                header.response.rect.bottom(),
                Stroke::new(1.0, t.color.border),
            );
            let row_count = draft.print_mapping.entries().len();
            egui::ScrollArea::vertical()
                .id_salt("hardcopy-print-mapping-rows")
                .max_height(360.0)
                .auto_shrink([false, true])
                .show_rows(ui, 36.0, row_count, |ui, visible| {
                    for index in visible {
                        let entry = &draft.print_mapping.entries()[index];
                        let row = Frame::NONE.show(ui, |ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.columns(5, |columns| {
                                mapping_cell(&mut columns[0], 36.0, |ui| {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(entry.object().display_name())
                                                .size(11.0)
                                                .color(ui.visuals().weak_text_color()),
                                        )
                                        .truncate(),
                                    );
                                });
                                mapping_cell(&mut columns[1], 36.0, |ui| {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(entry.object().screen_style())
                                                .size(11.0)
                                                .color(ui.visuals().weak_text_color()),
                                        )
                                        .truncate(),
                                    );
                                });
                                let mut color = entry.print_color();
                                let color_response = mapping_cell(&mut columns[2], 36.0, |ui| {
                                    egui::ComboBox::from_id_salt(("mapping-color", index))
                                        .selected_text(print_color_label(color))
                                        .width(ui.available_width())
                                        .show_ui(ui, |ui| {
                                            for (value, label) in print_color_options(color) {
                                                ui.selectable_value(&mut color, value, label);
                                            }
                                        })
                                        .response
                                });
                                color_response.widget_info(|| {
                                    egui::WidgetInfo::labeled(
                                        egui::WidgetType::ComboBox,
                                        true,
                                        format!(
                                            "Print color for {}",
                                            entry.object().display_name()
                                        ),
                                    )
                                });
                                let mut redundancy = entry.redundancy();
                                let redundancy_response =
                                    mapping_cell(&mut columns[3], 36.0, |ui| {
                                        egui::ComboBox::from_id_salt(("mapping-redundancy", index))
                                            .selected_text(redundancy_label(redundancy))
                                            .width(ui.available_width())
                                            .show_ui(ui, |ui| {
                                                for value in redundancy_options(redundancy) {
                                                    ui.selectable_value(
                                                        &mut redundancy,
                                                        value,
                                                        redundancy_label(value),
                                                    );
                                                }
                                            })
                                            .response
                                    });
                                redundancy_response.widget_info(|| {
                                    egui::WidgetInfo::labeled(
                                        egui::WidgetType::ComboBox,
                                        true,
                                        format!(
                                            "Line or fill style for {}",
                                            entry.object().display_name()
                                        ),
                                    )
                                });
                                let mut legend = entry.include_in_legend();
                                let legend_response = mapping_cell(&mut columns[4], 36.0, |ui| {
                                    ui.with_layout(
                                        Layout::centered_and_justified(
                                            egui::Direction::LeftToRight,
                                        ),
                                        |ui| ui.checkbox(&mut legend, ""),
                                    )
                                    .inner
                                });
                                legend_response.widget_info(|| {
                                    egui::WidgetInfo::selected(
                                        egui::WidgetType::Checkbox,
                                        true,
                                        legend,
                                        format!(
                                            "Include {} in the printed legend",
                                            entry.object().display_name()
                                        ),
                                    )
                                });
                                if color != entry.print_color()
                                    || redundancy != entry.redundancy()
                                    || legend != entry.include_in_legend()
                                {
                                    changed_entry = Some((
                                        index,
                                        PrintMappingEntry::try_new(
                                            entry.object().clone(),
                                            color,
                                            redundancy,
                                            legend,
                                        ),
                                    ));
                                }
                            });
                        });
                        ui.painter().hline(
                            row.response.rect.x_range(),
                            row.response.rect.bottom(),
                            Stroke::new(1.0, t.color.border),
                        );
                    }
                });
        });
    if let Some((index, result)) = changed_entry {
        match result {
            Ok(entry) => {
                let mut entries = draft.print_mapping.entries().to_vec();
                entries[index] = entry;
                match PrintMappingTable::try_new(draft.print_mapping.save_scope().clone(), entries)
                {
                    Ok(table) => {
                        draft.print_mapping = table;
                        mapping_changed = true;
                    }
                    Err(error) => draft.error = Some(error.to_string()),
                }
            }
            Err(error) => draft.error = Some(error.to_string()),
        }
    }
    mapping_note_grid(ui);
    if mapping_changed {
        draft.refresh_preview();
    }
}
