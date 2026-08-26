//! Specialist Models & PDKs page: symbols.

use super::symbol_contracts::{SymbolRow, SymbolRowAuthority, symbol_rows};
use super::*;

pub(super) fn symbols_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    section_title(
        ui,
        "Symbols, pins & device forms",
        "Project and technology symbol contracts · pin order is netlist order",
        |ui| {
            if ui.button("Library manager").clicked() {
                navigate_specialist(app, crate::workbench::SurfaceId::LibraryCellviewManager);
            }
            if ui.button("Import symbol").clicked() {
                super::super::open_symbol_import_dialog(app.state);
            }
            if ui.button("Form designer").clicked() {
                super::super::open_symbol_parameter_form_dialog(app.state);
            }
            if ui.button("Create symbol").clicked() {
                super::super::open_create_model_bound_symbol_dialog(app.state);
            }
        },
    );
    let rows = symbol_rows(ui, app);
    if rows.is_empty() {
        page_empty_state(
            ui,
            "No symbol contracts are loaded",
            "Import a symbol or create a model-bound symbol to establish an executable pin and parameter contract.",
        );
        return;
    }
    let project_count = rows
        .iter()
        .filter(|row| matches!(&row.authority, SymbolRowAuthority::DesignLibrary { .. }))
        .count();
    let technology_count = rows
        .iter()
        .filter(|row| matches!(&row.authority, SymbolRowAuthority::SignedTechnology { .. }))
        .count();
    let available = ui.available_size();
    let right_width = (available.x * 0.425).max(330.0).min(available.x - 260.0);
    let left_width = (available.x - right_width - 1.0).max(260.0);
    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(left_width, available.y),
            Layout::top_down(Align::Min),
            |ui| {
                table_header(
                    ui,
                    &[
                        ("SYMBOL", 0.30),
                        ("BOUND FAMILY", 0.25),
                        ("PINS", 0.20),
                        ("FORM", 0.13),
                        ("STATUS", 0.12),
                    ],
                );
                ScrollArea::vertical()
                    .id_salt("models-symbol-registry")
                    .max_height((available.y - HEADER_H - CATALOG_FOOT_H).max(120.0))
                    .show_rows(ui, SYMBOL_ROW_H, rows.len(), |ui, range| {
                        for row in &rows[range] {
                            let key = symbol_key(&row.reference);
                            let selected =
                                app.state.workbench.models_view.selected_symbol.as_deref()
                                    == Some(key.as_str());
                            if symbol_registry_row(ui, selected, row).clicked() {
                                app.state.workbench.models_view.selected_symbol = Some(key);
                                if !row.read_only() {
                                    app.state.library_manager.select_view(
                                        &row.reference.library,
                                        &row.reference.cell,
                                        &row.reference.view,
                                    );
                                }
                            }
                        }
                    });
                catalog_footer(
                    ui,
                    rows.len(),
                    rows.len(),
                    rows.iter()
                        .filter(|row| row.status == "review" || row.status == "pin mismatch")
                        .count(),
                    &format!("symbols · {project_count} project · {technology_count} technology"),
                );
            },
        );
        ui.separator();
        ui.allocate_ui_with_layout(
            egui::vec2(right_width, available.y),
            Layout::top_down(Align::Min),
            |ui| symbol_detail(ui, app, &rows),
        );
    });
}

/// Height of a symbol registry row, which the scroll area needs up front to
/// place the scrollbar while building only the rows on screen.
const SYMBOL_ROW_H: f32 = 36.0;

fn symbol_registry_row(ui: &mut Ui, selected: bool, row: &SymbolRow) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), SYMBOL_ROW_H),
        Sense::click(),
    );
    if selected {
        ui.painter()
            .rect_filled(rect, 0.0, t.color.accent.linear_multiply(0.14));
        ui.painter().vline(
            rect.left(),
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }

    let glyph = egui::Rect::from_min_size(
        egui::pos2(rect.left() + 5.0, rect.center().y - 13.0),
        egui::vec2(34.0, 26.0),
    );
    ui.painter().rect(
        glyph,
        3.0,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    paint_symbol_glyph(ui, glyph.shrink(5.0), &row.family, row.pins.len());

    let fractions = [0.30, 0.25, 0.20, 0.13, 0.12];
    let values = [
        format!("{}/{}", row.reference.cell, row.reference.view),
        row.family.clone(),
        row.pins.join(" "),
        row.form.clone(),
        row.status.clone(),
    ];
    let mut x = rect.left();
    for (index, (value, fraction)) in values.iter().zip(fractions).enumerate() {
        let width = rect.width() * fraction;
        let text_x = if index == 0 { x + 44.0 } else { x + 5.0 };
        let available = (width - if index == 0 { 48.0 } else { 9.0 }).max(8.0);
        ui.painter().text(
            egui::pos2(text_x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            elide(ui, value, available, index == 0 || index == 2),
            if index == 0 || index == 2 {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            },
            if index == 4 && row.status == "pin mismatch" {
                t.color.err
            } else {
                t.color.text_dim
            },
        );
        x += width;
    }
    let row_label = row.family.clone();
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            row_label.clone(),
        )
    });
    crate::ui::theme::paint_focus_ring(ui, &response, rect);
    response
}

fn paint_symbol_glyph(ui: &Ui, rect: egui::Rect, family: &str, pins: usize) {
    let t = Tokens::get(ui.ctx());
    let stroke = Stroke::new(1.1, t.color.text_dim);
    let center = rect.center();
    if family.to_ascii_lowercase().contains("mos") {
        ui.painter().vline(
            center.x + 2.0,
            (rect.top() + 2.0)..=(rect.bottom() - 2.0),
            stroke,
        );
        ui.painter().vline(
            center.x - 3.0,
            (rect.top() + 5.0)..=(rect.bottom() - 5.0),
            stroke,
        );
        ui.painter().line_segment(
            [
                egui::pos2(rect.left(), center.y),
                egui::pos2(center.x - 4.0, center.y),
            ],
            stroke,
        );
        ui.painter().line_segment(
            [
                egui::pos2(center.x + 2.0, rect.top()),
                egui::pos2(rect.right(), rect.top()),
            ],
            stroke,
        );
        ui.painter().line_segment(
            [
                egui::pos2(center.x + 2.0, rect.bottom()),
                egui::pos2(rect.right(), rect.bottom()),
            ],
            stroke,
        );
    } else {
        let body = egui::Rect::from_center_size(center, egui::vec2(10.0, 14.0));
        ui.painter()
            .rect_stroke(body, 1.0, stroke, egui::StrokeKind::Inside);
        ui.painter().line_segment(
            [egui::pos2(rect.left(), center.y), body.left_center()],
            stroke,
        );
        ui.painter().line_segment(
            [body.right_center(), egui::pos2(rect.right(), center.y)],
            stroke,
        );
    }
    if pins > 3 {
        ui.painter().circle_filled(center, 1.4, t.color.accent);
    }
}

fn symbol_detail(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, rows: &[SymbolRow]) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_symbol
        .as_deref()
        .and_then(|key| rows.iter().find(|row| symbol_key(&row.reference) == key))
        .cloned()
        .or_else(|| rows.first().cloned());
    let Some(row) = selected else {
        return;
    };
    app.state.workbench.models_view.selected_symbol = Some(symbol_key(&row.reference));
    ScrollArea::vertical()
        .id_salt("models-symbol-detail")
        .show(ui, |ui| {
            let t = Tokens::get(ui.ctx());
            egui::Frame::NONE
                .fill(t.color.bg_inset)
                .inner_margin(egui::Margin::symmetric(12, 10))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new(format!(
                                "{}/{}/{}",
                                row.reference.library, row.reference.cell, row.reference.view
                            ))
                            .monospace()
                            .font(theme::mono(tokens::FS_1, FontWeight::SemiBold)),
                        );
                        if let SymbolRowAuthority::SignedTechnology {
                            technology_name,
                            revision,
                            ..
                        } = &row.authority
                        {
                            ui.label(
                                RichText::new(format!(
                                    "{technology_name} {revision} · signed · read-only"
                                ))
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.info),
                            );
                        } else if row.read_only() {
                            ui.label(
                                RichText::new("design library · read-only")
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_faint),
                            );
                        }
                        if ui
                            .add(compact_button(if row.read_only() {
                                "Author a variant…"
                            } else {
                                "Open symbol editor"
                            }))
                            .clicked()
                        {
                            if matches!(
                                &row.authority,
                                SymbolRowAuthority::SignedTechnology { .. }
                            ) {
                                open_author_symbol_variant_dialog(app, &row);
                            } else if row.read_only() {
                                receipt(
                                    app,
                                    Err("This read-only design library is not a signed technology overlay and cannot be varied here.".to_owned()),
                                );
                            } else {
                                app.state.open_workspace_view(row.reference.clone());
                                app.state
                                    .workbench
                                    .activate(crate::workbench::state::Workspace::Design);
                            }
                        }
                        if ui
                            .add_enabled(!row.read_only(), compact_button("Edit form…"))
                            .on_disabled_hover_text(
                                "Technology symbols must be copied into the project before editing.",
                            )
                            .clicked()
                        {
                            app.state.library_manager.select_view(
                                &row.reference.library,
                                &row.reference.cell,
                                &row.reference.view,
                            );
                            super::super::open_symbol_parameter_form_dialog(app.state);
                        }
                    });
                });
            if !row.diagnostics.is_empty() {
                card(ui, |ui| {
                    card_title(ui, "BLOCKING CONTRACT FINDINGS", Some(&row.status));
                    for diagnostic in &row.diagnostics {
                        ui.label(
                            RichText::new(format!("Warning: {diagnostic}"))
                                .small()
                                .color(Tokens::get(ui.ctx()).color.err),
                        );
                    }
                });
            }
            if let SymbolRowAuthority::SignedTechnology {
                technology_name,
                revision,
                manifest_digest,
                archive_digest,
            } = &row.authority
            {
                card(ui, |ui| {
                    card_title(ui, "SIGNED TECHNOLOGY AUTHORITY", Some(technology_name));
                    property(ui, "Revision", revision, "exact project pin");
                    property(
                        ui,
                        "Manifest",
                        &short_digest(&manifest_digest.to_string()),
                        "publisher-signed",
                    );
                    property(
                        ui,
                        "Archive",
                        &short_digest(&archive_digest.to_string()),
                        "artifact closure",
                    );
                });
            }
            card(ui, |ui| {
                card_title(
                    ui,
                    "PIN PROVIDER MAP",
                    Some(&format!("{} pins", row.pins.len())),
                );
                if let Some(definition) = &row.definition {
                    for (index, pin) in definition.pins.iter().enumerate() {
                        property(
                            ui,
                            &format!("{:02} {}", index + 1, pin.name),
                            &format!("{:?}", pin.direction),
                            "netlist order",
                        );
                    }
                } else {
                    for (index, pin) in row.pins.iter().enumerate() {
                        property(ui, &format!("{:02}", index + 1), pin, "legacy symbol");
                    }
                }
            });
            card(ui, |ui| {
                card_title(ui, "NETLIST CONTRACT", Some(&row.family));
                property(ui, "Template", &row.template, "validated token grammar");
                if let Some(definition) = &row.definition {
                    property(
                        ui,
                        "Prefix",
                        &definition.netlist.device_prefix,
                        "instance reference",
                    );
                    property(
                        ui,
                        "Parameters",
                        &definition.netlist.parameter_order.join(" "),
                        "emission order",
                    );
                    property(
                        ui,
                        "Revision",
                        &definition.identity.revision.to_string(),
                        &definition.identity.binding_id,
                    );
                }
            });
            card(ui, |ui| {
                let field_count = row
                    .definition
                    .as_ref()
                    .map_or(0, |definition| definition.parameter_form.fields().count());
                card_title(
                    ui,
                    "CDF / COMPONENT FORM",
                    Some(&format!("{field_count} typed fields")),
                );
                if let Some(definition) = &row.definition {
                    if field_count == 0 {
                        empty_state(
                            ui,
                            "No component-form fields are declared.",
                            "The executable template emits no editable instance parameters.",
                        );
                    }
                    for section in &definition.parameter_form.sections {
                        ui.label(RichText::new(&section.label).strong());
                        for field in &section.fields {
                            property(
                                ui,
                                &field.key,
                                &format!(
                                    "{:?} · {:?} · {}",
                                    field.property_type,
                                    field.visibility,
                                    field.unit.as_deref().unwrap_or("unitless")
                                ),
                                if field.required {
                                    "required"
                                } else {
                                    "optional"
                                },
                            );
                        }
                    }
                } else {
                    empty_state(
                        ui,
                        "This legacy symbol has no typed component form.",
                        "Open Form designer to publish an explicit parameter contract.",
                    );
                }
            });
        });
}

fn open_author_symbol_variant_dialog(app: &mut ManagerRenderContext<'_>, row: &SymbolRow) {
    let Some(target_library) = app
        .state
        .library_manager
        .selected_library
        .as_deref()
        .and_then(|name| {
            app.state
                .library_manager
                .get_library(name)
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
        })
        .or_else(|| {
            app.state
                .library_manager
                .libraries_sorted()
                .into_iter()
                .find(|library| !library.read_only)
                .map(|library| library.name.clone())
        })
    else {
        receipt(
            app,
            Err(
                "Authoring a technology-symbol variant requires a writable design library."
                    .to_owned(),
            ),
        );
        return;
    };
    app.state.workbench.models_view.dialog =
        Some(ModelsWorkbenchDialog::AuthorTechnologySymbolVariant {
            package_id: row.reference.library.clone(),
            source_cell: row.reference.cell.clone(),
            target_library,
            target_cell: row.reference.cell.clone(),
        });
}

fn symbol_key(reference: &CellViewRef) -> String {
    format!(
        "{}\u{1f}{}\u{1f}{}",
        reference.library, reference.cell, reference.view
    )
}
