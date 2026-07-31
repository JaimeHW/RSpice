//! Specialist Models & PDKs pages: symbols, corners, bins, and includes.

use super::*;

#[derive(Clone)]
struct SymbolRow {
    reference: CellViewRef,
    read_only: bool,
    family: String,
    pins: Vec<String>,
    form: String,
    template: String,
    status: String,
    definition: Option<ModelBoundSymbolDefinition>,
    diagnostics: Vec<String>,
}

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
                super::super::open_symbol_import_dialog(&mut app.state);
            }
            if ui.button("Form designer").clicked() {
                super::super::open_symbol_parameter_form_dialog(&mut app.state);
            }
            if ui.button("Create symbol").clicked() {
                super::super::open_create_model_bound_symbol_dialog(&mut app.state);
            }
        },
    );
    let rows = symbol_rows(app);
    let project_count = rows.iter().filter(|row| !row.read_only).count();
    let technology_count = rows.iter().filter(|row| row.read_only).count();
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
                    .show(ui, |ui| {
                        if rows.is_empty() {
                            empty_state(
                                ui,
                                "No symbol views are present in the loaded design libraries.",
                                "Import a symbol or create a model-bound symbol to establish an executable pin contract.",
                            );
                        }
                        for row in &rows {
                            let key = symbol_key(&row.reference);
                            let selected =
                                app.state.workbench.models_view.selected_symbol.as_deref()
                                    == Some(key.as_str());
                            if symbol_registry_row(ui, selected, row).clicked() {
                                app.state.workbench.models_view.selected_symbol = Some(key);
                                app.state.library_manager.select_view(
                                    &row.reference.library,
                                    &row.reference.cell,
                                    &row.reference.view,
                                );
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

fn symbol_registry_row(ui: &mut Ui, selected: bool, row: &SymbolRow) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 36.0), Sense::click());
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

fn symbol_rows(app: &ManagerRenderContext<'_>) -> Vec<SymbolRow> {
    let mut rows = Vec::new();
    for library in app.state.library_manager.libraries_sorted() {
        for cell in library.cells_sorted() {
            for view in cell
                .views_sorted()
                .into_iter()
                .filter(|view| view.view_type == ViewType::Symbol)
            {
                let definition_result = ModelBoundSymbolDefinition::load_from_view(view);
                let document_result = SymbolDocument::load_from_view(view);
                let definition = definition_result.as_ref().ok().and_then(Clone::clone);
                let pins = document_result
                    .as_ref()
                    .map(|document| {
                        document
                            .pins
                            .iter()
                            .map(|pin| pin.name.clone())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                let family = definition
                    .as_ref()
                    .and_then(|definition| definition.netlist.model.as_ref())
                    .map(|model| format!("{}/{}", model.library, model.model))
                    .unwrap_or_else(|| symbol_model_family(app, cell));
                let form = definition
                    .as_ref()
                    .map(super::super::symbol_parameter_form_label)
                    .unwrap_or_else(|| "legacy / none".to_owned());
                let template = definition
                    .as_ref()
                    .map(|definition| definition.netlist.template.clone())
                    .filter(|template| !template.trim().is_empty())
                    .unwrap_or_else(|| "not defined".to_owned());
                let mut diagnostics = Vec::new();
                if let Err(error) = definition_result {
                    diagnostics.push(format!("Definition metadata: {error}"));
                }
                if let Err(error) = &document_result {
                    diagnostics.push(format!("Symbol document: {error}"));
                }
                if let (Some(definition), Ok(document)) = (&definition, &document_result) {
                    let expected = definition
                        .pins
                        .iter()
                        .map(|pin| pin.name.to_ascii_lowercase())
                        .collect::<Vec<_>>();
                    let observed = document
                        .pins
                        .iter()
                        .map(|pin| pin.name.to_ascii_lowercase())
                        .collect::<Vec<_>>();
                    if expected != observed {
                        diagnostics.push(format!(
                            "Blocking pin mismatch: provider {:?}, symbol {:?}",
                            expected, observed
                        ));
                    }
                    if let Err(error) = definition.validate() {
                        diagnostics.push(format!("Executable contract: {error}"));
                    }
                } else if definition.is_none() {
                    diagnostics
                        .push("Legacy symbol has no typed model/netlist/form contract.".to_owned());
                }
                let status = if diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.contains("Blocking"))
                {
                    "pin mismatch"
                } else if !diagnostics.is_empty() {
                    "review"
                } else if library.read_only {
                    "read-only"
                } else {
                    "bound"
                }
                .to_owned();
                rows.push(SymbolRow {
                    reference: CellViewRef::new(&library.name, &cell.name, &view.name),
                    read_only: library.read_only,
                    family,
                    pins,
                    form,
                    template,
                    status,
                    definition,
                    diagnostics,
                });
            }
        }
    }
    rows.sort_by(|left, right| {
        left.reference
            .library
            .to_ascii_lowercase()
            .cmp(&right.reference.library.to_ascii_lowercase())
            .then_with(|| {
                left.reference
                    .cell
                    .to_ascii_lowercase()
                    .cmp(&right.reference.cell.to_ascii_lowercase())
            })
    });
    rows
}

fn symbol_model_family(app: &ManagerRenderContext<'_>, cell: &crate::state::Cell) -> String {
    if let Some(value) = super::super::metadata_value(
        [&cell.metadata],
        &["model.family", "model_family", "model", "model.name"],
    ) {
        return value;
    }
    app.state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| library.models.values())
        .find(|model| model.name.eq_ignore_ascii_case(&cell.name))
        .map(|model| model.name.clone())
        .unwrap_or_else(|| "unbound".to_owned())
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
                        if row.read_only {
                            ui.label(
                                RichText::new("technology-owned · read-only")
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.info),
                            );
                        }
                        if ui
                            .add(compact_button(if row.read_only {
                                "Author a variant…"
                            } else {
                                "Open symbol editor"
                            }))
                            .clicked()
                        {
                            if row.read_only {
                                super::super::open_create_model_bound_symbol_dialog(&mut app.state);
                            } else {
                                app.state.open_workspace_view(row.reference.clone());
                                app.state
                                    .workbench
                                    .activate(crate::workbench::state::Workspace::Design);
                            }
                        }
                        if ui
                            .add_enabled(!row.read_only, compact_button("Edit form…"))
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
                            super::super::open_symbol_parameter_form_dialog(&mut app.state);
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

pub(super) fn corners_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let rows = corner_rows(app);
    let unresolved = rows.iter().filter(|row| !row.resolved).count();
    section_title(
        ui,
        "Corners & sections",
        &format!(
            "{} bindings · {} unresolved · fail closed before run expansion",
            rows.len(),
            unresolved
        ),
        |ui| {
            if ui.button("Import section map").clicked() {
                app.queue_command(Command::PdkSettings);
            }
            if ui.button("Add corner…").clicked() {
                if let Some(library) = app
                    .state
                    .model_library_manager
                    .selected_library
                    .clone()
                    .or_else(|| {
                        app.state
                            .model_library_manager
                            .libraries_sorted()
                            .first()
                            .map(|library| library.name.clone())
                    })
                {
                    app.state.workbench.models_view.dialog =
                        Some(ModelsWorkbenchDialog::AddCorner {
                            library,
                            name: String::new(),
                            temperature_c: "27".to_owned(),
                            supply_factor: "1.0".to_owned(),
                        });
                } else {
                    receipt(
                        app,
                        Err("Attach a model library before adding a corner.".to_owned()),
                    );
                }
            }
            if ui.button("Validate bindings").clicked() {
                receipt(
                    app,
                    if unresolved == 0 {
                        Ok(format!(
                            "Validated all {} process-corner bindings.",
                            rows.len()
                        ))
                    } else {
                        Err(format!(
                            "Corner validation found {unresolved} bindings without an exact source section."
                        ))
                    },
                );
            }
        },
    );
    card(ui, |ui| {
        card_title(ui, "PACKAGE CANDIDATE", Some("migration review"));
        ui.label(
            RichText::new(
                "No unreviewed technology-package candidate is retained. Importing a new section map creates a transactional review before bindings change.",
            )
            .small()
            .color(Tokens::get(ui.ctx()).color.text_dim),
        );
    });
    let table_h = (ui.available_height() * 0.45).clamp(180.0, 340.0);
    card(ui, |ui| {
        table_header(
            ui,
            &[
                ("CORNER", 0.12),
                ("MOS", 0.13),
                ("BJT", 0.11),
                ("PASSIVES", 0.13),
                ("STAT", 0.10),
                ("AGING", 0.10),
                ("TEMP", 0.13),
                ("STATUS", 0.18),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-corner-matrix")
            .max_height(table_h)
            .show(ui, |ui| {
                if rows.is_empty() {
                    empty_state(
                        ui,
                        "No process-corner bindings are present.",
                        "Import a PDK section map or attach a sectioned model library.",
                    );
                }
                for row in &rows {
                    let selected = app.state.workbench.models_view.selected_corner.as_deref()
                        == Some(row.key.as_str());
                    if selectable_data_row(
                        ui,
                        selected,
                        &[
                            (&row.corner.name.to_uppercase(), 0.12, true),
                            (
                                &format!("{}/{}", row.corner.nmos_corner, row.corner.pmos_corner),
                                0.13,
                                true,
                            ),
                            (if row.resolved { "section" } else { "" }, 0.11, true),
                            (if row.resolved { "section" } else { "" }, 0.13, true),
                            (if row.has_statistics { "bound" } else { "" }, 0.10, true),
                            (if row.has_aging { "evidence" } else { "" }, 0.10, true),
                            (&format!("{:.1} °C", row.corner.temperature), 0.13, true),
                            (
                                if row.resolved {
                                    "resolved"
                                } else {
                                    "unresolved"
                                },
                                0.18,
                                true,
                            ),
                        ],
                    )
                    .clicked()
                    {
                        app.state.workbench.models_view.selected_corner = Some(row.key.clone());
                        select_corner(app, &row.library, &row.corner.name);
                    }
                }
            });
    });
    corner_detail(ui, app, &rows);
}

#[derive(Clone)]
struct CornerRow {
    key: String,
    library: String,
    corner: ProcessCorner,
    resolved: bool,
    has_statistics: bool,
    has_aging: bool,
    source: Option<String>,
}

fn corner_rows(app: &ManagerRenderContext<'_>) -> Vec<CornerRow> {
    let mut rows = Vec::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        let has_statistics = library
            .model_definition_metadata
            .values()
            .any(|metadata| !metadata.statistics.variables.is_empty());
        let has_aging = library
            .model_qualification
            .values()
            .any(|qualification| !qualification.evidence.is_empty());
        for corner in library.corners.values() {
            let source = corner
                .file_path
                .as_deref()
                .or(library.root_path.as_deref())
                .map(|path| path.display().to_string());
            let retained_section = library.source_contents.iter().any(|content| {
                String::from_utf8_lossy(&content.bytes)
                    .to_ascii_lowercase()
                    .contains(&format!(".lib {}", corner.name.to_ascii_lowercase()))
            });
            let resolved = source.is_some()
                && (retained_section
                    || (corner.name.eq_ignore_ascii_case("tt") && library.corners.len() == 1));
            rows.push(CornerRow {
                key: format!("{}\u{1f}{}", library.name, corner.name),
                library: library.name.clone(),
                corner: corner.clone(),
                resolved,
                has_statistics,
                has_aging,
                source,
            });
        }
    }
    rows.sort_by(|left, right| {
        left.corner
            .name
            .to_ascii_lowercase()
            .cmp(&right.corner.name.to_ascii_lowercase())
            .then_with(|| left.library.cmp(&right.library))
    });
    rows
}

fn corner_detail(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, rows: &[CornerRow]) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_corner
        .as_deref()
        .and_then(|key| rows.iter().find(|row| row.key == key))
        .cloned()
        .or_else(|| rows.first().cloned());
    let Some(row) = selected else {
        return;
    };
    app.state.workbench.models_view.selected_corner = Some(row.key.clone());
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new(format!(
                "{} / {}",
                row.library,
                row.corner.name.to_uppercase()
            ))
            .monospace()
            .strong(),
        );
        if !row.resolved {
            ui.label(
                RichText::new("unresolved · run expansion blocked")
                    .small()
                    .color(Tokens::get(ui.ctx()).color.err),
            );
        }
        if ui
            .add_enabled(row.source.is_some(), egui::Button::new("Open source"))
            .clicked()
            && let Some(library) = app
                .state
                .model_library_manager
                .get_library(&row.library)
                .cloned()
        {
            if let Some(model) = library.models.values().next().cloned() {
                open_model_source(app, &library, &model);
            }
        }
        if ui.button("View include graph").clicked() {
            app.state.workbench.models_page = ModelsPage::Include;
        }
        if ui.button("Model editor…").clicked() {
            app.queue_command(Command::ModelEditor);
        }
    });
    ui.columns(2, |columns| {
        card(&mut columns[0], |ui| {
            card_title(ui, "SECTION BINDING", Some(&row.library));
            property(ui, "NMOS", &row.corner.nmos_corner, "exact section axis");
            property(ui, "PMOS", &row.corner.pmos_corner, "exact section axis");
            property(
                ui,
                "Source",
                row.source.as_deref().unwrap_or("not bound"),
                if row.resolved {
                    "retained"
                } else {
                    "unresolved"
                },
            );
            property(
                ui,
                "Supply factor",
                &format!("{:.6}", row.corner.vdd_factor),
                "environment axis",
            );
            property(
                ui,
                "Temperature",
                &format!("{:.3} °C", row.corner.temperature),
                "environment axis",
            );
        });
        card(&mut columns[1], |ui| {
            card_title(ui, "STATISTICAL & AGING", Some("evidence projection"));
            property(
                ui,
                "Statistical variables",
                if row.has_statistics {
                    "declared"
                } else {
                    "none"
                },
                "model schema",
            );
            property(
                ui,
                "Aging evidence",
                if row.has_aging { "retained" } else { "none" },
                "qualification",
            );
            property(
                ui,
                "Binding policy",
                if row.resolved {
                    "executable"
                } else {
                    "fail closed"
                },
                "run expansion",
            );
        });
    });
}

fn select_corner(app: &mut ManagerRenderContext<'_>, library_name: &str, corner_name: &str) {
    let Some(library) = app
        .state
        .model_library_manager
        .get_library(library_name)
        .cloned()
    else {
        receipt(
            app,
            Err(format!("Library '{library_name}' no longer exists.")),
        );
        return;
    };
    app.state.model_library_manager.select_library(library_name);
    if let Some(root) = library.root_path {
        let mut candidate = app.state.model_library_manager.clone();
        let result = candidate
            .load_library_file(&root, Some(corner_name))
            .and_then(|loaded| {
                if loaded != library_name {
                    return Err(format!(
                        "Corner selection resolved library '{loaded}' instead of '{library_name}'."
                    ));
                }
                publish_model_library_candidate(
                    &mut app.state,
                    candidate,
                    library_name,
                    format!("select model corner {corner_name}"),
                )
                .map(|revision| {
                    format!(
                        "Selected exact section '{corner_name}' for '{library_name}' at revision {}.",
                        revision.get()
                    )
                })
            });
        receipt(app, result);
    } else if let Some(library) = app
        .state
        .model_library_manager
        .get_library_mut(library_name)
    {
        library.select_corner(corner_name);
    }
}

fn symbol_key(reference: &CellViewRef) -> String {
    format!(
        "{}\u{1f}{}\u{1f}{}",
        reference.library, reference.cell, reference.view
    )
}

pub(super) fn bins_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let mut families = BTreeMap::<String, Vec<(String, DeviceModel)>>::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        for model in library.models.values() {
            if model.l_min.is_some()
                || model.l_max.is_some()
                || model.w_min.is_some()
                || model.w_max.is_some()
            {
                families
                    .entry(model.model_type.display_name().to_owned())
                    .or_default()
                    .push((library.name.clone(), model.clone()));
            }
        }
    }
    let findings = geometry_findings(&families);
    section_title(
        ui,
        "Bins & geometry",
        &format!(
            "{} binned families · {} cards · {} findings",
            families.len(),
            families.values().map(Vec::len).sum::<usize>(),
            findings.len()
        ),
        |ui| {
            if ui.button("Import bin map").clicked() {
                app.queue_command(Command::PdkSettings);
            }
            if ui.button("Edit cards…").clicked() {
                app.queue_command(Command::ModelEditor);
            }
            if ui.button("Audit all families").clicked() {
                receipt(
                    app,
                    if findings.is_empty() {
                        Ok(
                            "Geometry audit completed with no overlapping card envelopes."
                                .to_owned(),
                        )
                    } else {
                        Err(format!(
                            "Geometry audit found {} overlapping or incomplete envelopes.",
                            findings.len()
                        ))
                    },
                );
            }
            if ui
                .add_enabled(
                    exactly_one_selected_component(app).is_some(),
                    egui::Button::new("Trace schematic"),
                )
                .clicked()
            {
                let model = app
                    .state
                    .workbench
                    .selected_model
                    .clone()
                    .unwrap_or_else(|| "selected geometry family".to_owned());
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::BindingTrace {
                        consumers: model_consumers(app, &model),
                        model,
                    });
            }
        },
    );
    if families.is_empty() {
        empty_state(
            ui,
            "No loaded model publishes a geometry envelope.",
            "Attach a binned PDK library or author L/W bounds in Model Editor.",
        );
        return;
    }
    ui.columns(2, |columns| {
        card(&mut columns[0], |ui| {
            card_title(ui, "BIN FAMILIES", Some("live loaded models"));
            for (family, cards) in &families {
                if ui
                    .selectable_label(
                        app.state
                            .workbench
                            .models_view
                            .selected_bin_family
                            .as_deref()
                            == Some(family.as_str()),
                        format!("{family}  ·  {} cards", cards.len()),
                    )
                    .clicked()
                {
                    app.state.workbench.models_view.selected_bin_family = Some(family.clone());
                }
            }
        });
        card(&mut columns[1], |ui| {
            card_title(ui, "AUDIT FINDINGS", Some("fail closed on overlap"));
            if findings.is_empty() {
                ui.label(
                    RichText::new("Every declared envelope is non-overlapping.")
                        .color(Tokens::get(ui.ctx()).color.ok),
                );
            } else {
                for finding in findings.iter().take(10) {
                    ui.label(
                        RichText::new(format!("Review: {finding}"))
                            .small()
                            .color(Tokens::get(ui.ctx()).color.err),
                    );
                }
            }
        });
    });
    let selected = app
        .state
        .workbench
        .models_view
        .selected_bin_family
        .clone()
        .or_else(|| families.keys().next().cloned());
    if let Some(selected) = selected {
        app.state.workbench.models_view.selected_bin_family = Some(selected.clone());
        if let Some(cards) = families.get(&selected) {
            geometry_map(ui, &selected, cards);
            geometry_instance_table(ui, app, cards);
        }
    }
}

fn geometry_findings(families: &BTreeMap<String, Vec<(String, DeviceModel)>>) -> Vec<String> {
    let mut findings = Vec::new();
    for (family, cards) in families {
        for (index, (_, left)) in cards.iter().enumerate() {
            if model_geometry_invalid(left) {
                findings.push(format!(
                    "{family}/{} has an inverted L/W envelope",
                    left.name
                ));
            }
            for (_, right) in cards.iter().skip(index + 1) {
                if envelopes_overlap(left, right) {
                    findings.push(format!("{family}/{} overlaps {}", left.name, right.name));
                }
            }
        }
    }
    findings
}

fn envelopes_overlap(left: &DeviceModel, right: &DeviceModel) -> bool {
    let (Some(ll), Some(lh), Some(lw), Some(wh)) = (left.l_min, left.l_max, left.w_min, left.w_max)
    else {
        return false;
    };
    let (Some(rl), Some(rh), Some(rw), Some(rwh)) =
        (right.l_min, right.l_max, right.w_min, right.w_max)
    else {
        return false;
    };
    ll < rh && rl < lh && lw < rwh && rw < wh
}

fn geometry_map(ui: &mut Ui, family: &str, cards: &[(String, DeviceModel)]) {
    card(ui, |ui| {
        card_title(ui, "LOG L/W MAP", Some(family));
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 210.0), Sense::hover());
        let t = Tokens::get(ui.ctx());
        ui.painter().rect(
            rect,
            2.0,
            t.color.bg_inset,
            Stroke::new(1.0, t.color.border),
            egui::StrokeKind::Inside,
        );
        let plot = rect.shrink(20.0);
        ui.painter().line_segment(
            [plot.left_bottom(), plot.right_bottom()],
            Stroke::new(1.0, t.color.border_strong),
        );
        ui.painter().line_segment(
            [plot.left_bottom(), plot.left_top()],
            Stroke::new(1.0, t.color.border_strong),
        );
        let finite = cards
            .iter()
            .filter_map(|(_, model)| {
                Some((
                    model.l_min?.max(f64::MIN_POSITIVE).log10(),
                    model.l_max?.max(f64::MIN_POSITIVE).log10(),
                    model.w_min?.max(f64::MIN_POSITIVE).log10(),
                    model.w_max?.max(f64::MIN_POSITIVE).log10(),
                    model.name.as_str(),
                ))
            })
            .collect::<Vec<_>>();
        let min_l = finite
            .iter()
            .map(|value| value.0)
            .reduce(f64::min)
            .unwrap_or(-9.0);
        let max_l = finite
            .iter()
            .map(|value| value.1)
            .reduce(f64::max)
            .unwrap_or(-3.0);
        let min_w = finite
            .iter()
            .map(|value| value.2)
            .reduce(f64::min)
            .unwrap_or(-9.0);
        let max_w = finite
            .iter()
            .map(|value| value.3)
            .reduce(f64::max)
            .unwrap_or(-3.0);
        let map_x = |value: f64| {
            plot.left() + plot.width() * ((value - min_l) / (max_l - min_l).max(1e-12)) as f32
        };
        let map_y = |value: f64| {
            plot.bottom() - plot.height() * ((value - min_w) / (max_w - min_w).max(1e-12)) as f32
        };
        for (index, (l0, l1, w0, w1, name)) in finite.iter().enumerate() {
            let bin = egui::Rect::from_min_max(
                egui::pos2(map_x(*l0), map_y(*w1)),
                egui::pos2(map_x(*l1), map_y(*w0)),
            );
            let color = [
                t.color.accent,
                t.color.info,
                t.color.ok,
                t.color.warn,
                t.color.text_dim,
            ][index % 5];
            ui.painter().rect(
                bin,
                1.0,
                color.linear_multiply(0.12),
                Stroke::new(1.2, color),
                egui::StrokeKind::Inside,
            );
            ui.painter().text(
                bin.center(),
                egui::Align2::CENTER_CENTER,
                *name,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                color,
            );
        }
    });
}

fn geometry_instance_table(
    ui: &mut Ui,
    app: &ManagerRenderContext<'_>,
    cards: &[(String, DeviceModel)],
) {
    card(ui, |ui| {
        card_title(ui, "INSTANCE RESOLUTION", Some("active schematic"));
        let Some(schematic) = app.state.workspace.active_schematic() else {
            empty_state(
                ui,
                "No active schematic is available.",
                "Open a project schematic to trace placed geometry.",
            );
            return;
        };
        let names = cards
            .iter()
            .map(|(_, model)| model.name.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let matches = schematic
            .components
            .iter()
            .filter(|component| {
                explicit_component_model(component)
                    .is_some_and(|model| names.contains(&model.to_ascii_lowercase()))
            })
            .collect::<Vec<_>>();
        if matches.is_empty() {
            empty_state(
                ui,
                "No placed instance resolves to this family.",
                "Binding resolution is derived from the active schematic, not fixture counts.",
            );
        }
        for component in matches {
            property(
                ui,
                &component.name,
                explicit_component_model(component)
                    .as_deref()
                    .unwrap_or("unbound"),
                &component.params,
            );
        }
    });
}

pub(super) fn include_page(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    diagnostics: &super::super::IncludeDiagnostics,
) {
    section_title(
        ui,
        "Model include graph",
        &format!(
            "{} files · {} edges · {} diagnostics",
            diagnostics.files,
            diagnostics.edges,
            diagnostics.unpinned_roots + diagnostics.cyclic_nodes
        ),
        |ui| {
            if ui.button("Resolve drift…").clicked() {
                if let Some(library) = app.state.model_library_manager.current_library().cloned() {
                    refresh_library(app, &library);
                } else {
                    receipt(
                        app,
                        Err("Select a model source to resolve first.".to_owned()),
                    );
                }
            }
            if ui.button("Export manifest").clicked() {
                export_include_manifest(app);
            }
        },
    );
    ui.horizontal(|ui| {
        ui.checkbox(
            &mut app.state.workbench.models_view.include_direct_only,
            "Direct dependencies only",
        );
        ui.add(
            egui::TextEdit::singleline(
                &mut app.state.workbench.models_view.include_definition_query,
            )
            .hint_text("Filter definitions or providers…")
            .desired_width(260.0),
        );
    });
    let libraries = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    if libraries.is_empty() {
        empty_state(
            ui,
            "No model-source closure is loaded.",
            "Add a library or attach a pack to build an authenticated include graph.",
        );
        return;
    }
    ui.columns(2, |columns| {
        card(&mut columns[0], |ui| {
            card_title(
                ui,
                "ORDERED CLOSURE",
                Some("root plus retained dependencies"),
            );
            ScrollArea::vertical()
                .id_salt("models-include-nodes")
                .max_height(270.0)
                .show(ui, |ui| {
                    for library in &libraries {
                        ui.label(
                            RichText::new(format!(
                                "{} · {} sources",
                                library.name,
                                library.source_closure.len()
                            ))
                            .strong(),
                        );
                        for (index, source) in library.source_closure.iter().enumerate() {
                            let label = format!(
                                "{:02}  {}  {}",
                                index + 1,
                                path_label(&source.path),
                                short_digest(&source.digest.to_string())
                            );
                            if ui
                                .selectable_label(
                                    app.state
                                        .workbench
                                        .models_view
                                        .include_selected_source
                                        .as_deref()
                                        == Some(source.path.to_string_lossy().as_ref()),
                                    label,
                                )
                                .clicked()
                            {
                                app.state.workbench.models_view.include_selected_source =
                                    Some(source.path.to_string_lossy().into_owned());
                            }
                        }
                    }
                });
        });
        card(&mut columns[1], |ui| {
            card_title(ui, "RESOLUTION HEALTH", Some("authenticated graph"));
            property(
                ui,
                "Pinned files",
                &diagnostics.files.to_string(),
                "digest retained",
            );
            property(
                ui,
                "Dependency edges",
                &diagnostics.edges.to_string(),
                "owner → target",
            );
            property(
                ui,
                "Unpinned roots",
                &diagnostics.unpinned_roots.to_string(),
                if diagnostics.unpinned_roots == 0 {
                    "clean"
                } else {
                    "blocks execution"
                },
            );
            property(
                ui,
                "Cycle nodes",
                &diagnostics.cyclic_nodes.to_string(),
                if diagnostics.cyclic_nodes == 0 {
                    "clean"
                } else {
                    "blocks execution"
                },
            );
        });
    });
    include_definition_table(ui, app, &libraries);
}

fn include_definition_table(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    libraries: &[ModelLibrary],
) {
    let query = app
        .state
        .workbench
        .models_view
        .include_definition_query
        .trim()
        .to_ascii_lowercase();
    let mut providers = BTreeMap::<String, Vec<String>>::new();
    for library in libraries {
        for model in library.models.values() {
            providers
                .entry(model.name.to_ascii_lowercase())
                .or_default()
                .push(library.name.clone());
        }
    }
    card(ui, |ui| {
        card_title(
            ui,
            "DEFINITION RESOLUTION",
            Some(&format!("{} unique names", providers.len())),
        );
        table_header(
            ui,
            &[
                ("DEFINITION", 0.25),
                ("KIND", 0.13),
                ("WINNING PROVIDER", 0.25),
                ("OTHER CANDIDATES", 0.20),
                ("RESOLUTION", 0.17),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-include-definitions")
            .max_height(260.0)
            .show(ui, |ui| {
                let mut shown = 0;
                for (definition, candidates) in &providers {
                    if !query.is_empty()
                        && !definition.contains(&query)
                        && !candidates
                            .iter()
                            .any(|provider| provider.to_ascii_lowercase().contains(&query))
                    {
                        continue;
                    }
                    shown += 1;
                    let contested = candidates.len() > 1;
                    let other_candidates = if contested {
                        (candidates.len() - 1).to_string()
                    } else {
                        "not bound".to_owned()
                    };
                    let response = selectable_data_row(
                        ui,
                        false,
                        &[
                            (definition, 0.25, true),
                            ("model", 0.13, true),
                            (&candidates[0], 0.25, false),
                            (&other_candidates, 0.20, true),
                            (if contested { "contested" } else { "unique" }, 0.17, true),
                        ],
                    );
                    if contested && response.clicked() {
                        app.state.workbench.models_view.dialog =
                            Some(ModelsWorkbenchDialog::DefinitionConflict {
                                definition: definition.clone(),
                                providers: candidates.clone(),
                            });
                    }
                }
                if shown == 0 {
                    empty_state(
                        ui,
                        "No definitions match.",
                        "The filter searches definition names and every retained provider.",
                    );
                }
            });
    });
}
