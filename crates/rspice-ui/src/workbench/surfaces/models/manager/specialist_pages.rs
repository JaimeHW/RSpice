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
    let rows = symbol_rows(app);
    if rows.is_empty() {
        page_empty_state(
            ui,
            "No symbol contracts are loaded",
            "Import a symbol or create a model-bound symbol to establish an executable pin and parameter contract.",
        );
        return;
    }
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
                    .show_rows(ui, SYMBOL_ROW_H, rows.len(), |ui, range| {
                        for row in &rows[range] {
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

/// Height of a symbol registry row, which the scroll area needs up front to
/// place the scrollbar while building only the rows on screen.
const SYMBOL_ROW_H: f32 = 36.0;

/// The height one [`egui::Ui::selectable_label`] takes at the current style.
///
/// A virtualized list is told its row height up front, and every table on this
/// workspace allocates its rows at a height it names. The bin family list uses
/// the plain selectable label instead, whose height belongs to egui, so it is
/// derived the same way egui derives it rather than guessed. A guess that runs
/// short leaves the last families unreachable below the fold.
fn selectable_label_height(ui: &Ui) -> f32 {
    (ui.text_style_height(&egui::TextStyle::Button) + ui.spacing().button_padding.y * 2.0)
        .max(ui.spacing().interact_size.y)
}

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

fn symbol_rows(app: &ManagerRenderContext<'_>) -> Vec<SymbolRow> {
    let models_by_cell = model_names_by_cell(app);
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
                    .unwrap_or_else(|| symbol_model_family(&models_by_cell, cell));
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

/// Every model name in the corpus, folded for lookup by a symbol's cell name.
///
/// A symbol with no declared binding falls back to a model that shares its
/// cell name, which used to mean walking the whole corpus per symbol — the
/// product of the symbol registry and the model corpus, on every frame.
/// Insertion follows library order, so the model that wins a duplicated name
/// is the same one the linear search found.
fn model_names_by_cell(app: &ManagerRenderContext<'_>) -> BTreeMap<String, String> {
    let mut names = BTreeMap::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        for model in library.models.values() {
            names
                .entry(model.name.to_ascii_lowercase())
                .or_insert_with(|| model.name.clone());
        }
    }
    names
}

fn symbol_model_family(
    models_by_cell: &BTreeMap<String, String>,
    cell: &crate::state::Cell,
) -> String {
    if let Some(value) = super::super::metadata_value(
        [&cell.metadata],
        &["model.family", "model_family", "model", "model.name"],
    ) {
        return value;
    }
    models_by_cell
        .get(&cell.name.to_ascii_lowercase())
        .cloned()
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
                                super::super::open_create_model_bound_symbol_dialog(app.state);
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
    let unresolved = rows.iter().filter(|row| !row.resolved()).count();
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
                let result = validate_current_model_execution_plan(app, unresolved);
                receipt(app, result);
            }
        },
    );
    if rows.is_empty() {
        page_empty_state(
            ui,
            "No corner bindings are loaded",
            "Import a PDK section map or attach a sectioned model library to publish executable corner bindings.",
        );
        return;
    }
    let table_h = (ui.available_height() * 0.34).clamp(150.0, 240.0);
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
                            (if row.resolved() { "section" } else { "" }, 0.11, true),
                            (if row.resolved() { "section" } else { "" }, 0.13, true),
                            (if row.has_statistics { "bound" } else { "" }, 0.10, true),
                            (if row.has_aging { "evidence" } else { "" }, 0.10, true),
                            (&format!("{:.1} °C", row.corner.temperature), 0.13, true),
                            (
                                if row.resolved() {
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

fn validate_current_model_execution_plan(
    app: &mut ManagerRenderContext<'_>,
    unresolved: usize,
) -> Result<String, String> {
    if unresolved > 0 {
        return Err(format!(
            "Corner validation found {unresolved} bindings without an exact source section."
        ));
    }
    if app.state.workbench.safe_mode.project_read_only() {
        return Err(
            "A durable model-validation receipt cannot be published while the project is read-only."
                .to_owned(),
        );
    }
    let has_project_technology = app.state.project_technology_in_effect();
    if has_project_technology {
        app.state.technology_gate_block_reason()?;
    }
    let sealed = if has_project_technology {
        app.state.seal_project_execution_model_sources()?
    } else {
        app.state.model_library_manager.seal_execution_sources()?
    };
    let plan = sealed.reference_model_execution_plan(app.state.sim_setup.reference_pvt.process)?;
    let mut findings = vec![
        crate::state::model_library::ModelValidationFinding {
            code: "SOURCE_CLOSURE_AUTHENTICATED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: "Every executable SPICE source and transitive dependency matched its accepted content digest.".to_owned(),
        },
        crate::state::model_library::ModelValidationFinding {
            code: "SPICE_NAMESPACE_COMPILED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: format!(
                "The frozen SPICE namespace compiled with {} bindings and {} explicit provider decisions.",
                plan.bindings().len(),
                plan.applied_resolutions().len()
            ),
        },
    ];
    let mut veriloga_count = 0_usize;
    if let Some((package, archive_digest, artifacts, bindings)) = sealed.pdk_veriloga_authority() {
        for binding in bindings {
            crate::simulation::veriloga::compile_signed_pdk_source_runtime(
                package,
                archive_digest,
                artifacts,
                binding,
            )?;
            veriloga_count += 1;
        }
        findings.push(crate::state::model_library::ModelValidationFinding {
            code: "VERILOGA_RUNTIME_COMPILED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: format!(
                "Compiled and validated {veriloga_count} authenticated signed-PDK Verilog-A runtime bindings."
            ),
        });
    }
    let pdk_archive_digest = sealed
        .pdk_model_identity()
        .map(|(_, archive_digest)| archive_digest);
    if pdk_archive_digest.is_some() {
        findings.push(crate::state::model_library::ModelValidationFinding {
            code: "SIGNED_PDK_TRUST_VERIFIED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: "The exact project-pinned signed PDK archive, platform contract, and trust chain were verified.".to_owned(),
        });
    }
    let receipt = app
        .state
        .model_library_manager
        .issue_model_validation_receipt(
            app.state.workspace.project.revision(),
            plan.digest(),
            pdk_archive_digest,
            crate::io::PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
            findings,
        )?;
    app.state
        .model_library_manager
        .validate_model_validation_receipt(
            app.state.workspace.project.revision(),
            plan.digest(),
            pdk_archive_digest,
            crate::io::PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
        )?;
    app.state.workspace.project_metadata_dirty = true;
    Ok(format!(
        "Published durable model-validation receipt {} for exact plan {} with {} authenticated bindings, {} source-qualified provider decisions, and {veriloga_count} Verilog-A runtimes.",
        receipt.receipt_digest,
        plan.digest(),
        plan.bindings().len(),
        plan.applied_resolutions().len()
    ))
}

#[derive(Clone)]
struct CornerRow {
    key: String,
    library: String,
    corner: ProcessCorner,
    /// Why a run cannot expand this corner, in the words the run itself uses.
    /// `None` means the corner resolves.
    blocker: Option<String>,
    has_statistics: bool,
    has_aging: bool,
    source: Option<String>,
    source_digest: Option<String>,
}

impl CornerRow {
    const fn resolved(&self) -> bool {
        self.blocker.is_none()
    }
}

/// Why a corner cannot be expanded into a run, or `None` if it can.
///
/// This restates the rule the run itself applies — every declared section
/// binding must name a section the retained closure defines — instead of
/// searching the retained bytes for a `.lib` line. The text search this
/// replaced reported a section missing whenever the file spelled the directive
/// with a tab, and reported one present when the name appeared in a comment,
/// so the page's "run expansion blocked" and the run's own verdict were
/// independent guesses. See `io::project_execution`'s
/// `persisted_active_model_section_names`.
fn corner_blocker(library: &ModelLibrary, corner: &ProcessCorner) -> Option<String> {
    if let Err(errors) = corner.validate_contract() {
        return Some(errors.join("; "));
    }
    let bindings = corner.effective_section_bindings();
    if bindings.is_empty() {
        // A corner that names no section and has no retained source is not
        // bound to a file at all; there is nothing for a run to resolve and
        // nothing to report.
        return (corner.file_path.is_some() || !library.source_closure.is_empty()).then(|| {
            format!(
                "corner '{}' has no executable section bindings",
                corner.name
            )
        });
    }
    let mut missing = bindings
        .iter()
        .filter(|binding| !library.defines_section(&binding.section))
        .map(|binding| binding.section.clone())
        .collect::<Vec<_>>();
    if missing.is_empty() {
        return None;
    }
    missing.sort();
    missing.dedup();
    Some(format!(
        "the retained closure defines no section named {}",
        missing
            .iter()
            .map(|section| format!("'{section}'"))
            .collect::<Vec<_>>()
            .join(", ")
    ))
}

fn corner_rows(app: &ManagerRenderContext<'_>) -> Vec<CornerRow> {
    let mut rows = Vec::new();
    let libraries = app.state.model_library_manager.libraries_sorted();
    let active_library = app
        .state
        .model_library_manager
        .selected_library
        .as_deref()
        .and_then(|selected| {
            libraries
                .iter()
                .find(|library| library.name.eq_ignore_ascii_case(selected))
        })
        .or_else(|| libraries.iter().find(|library| !library.corners.is_empty()))
        .map(|library| library.name.clone());
    for library in libraries {
        if active_library.as_deref() != Some(library.name.as_str()) {
            continue;
        }
        let has_statistics = library
            .model_definition_metadata
            .values()
            .any(|metadata| !metadata.statistics.variables.is_empty());
        let has_aging = library
            .model_qualification
            .values()
            .any(|qualification| !qualification.evidence.is_empty());
        for corner in library.corners.values() {
            let source_path = corner.file_path.as_deref().or(library.root_path.as_deref());
            let source = source_path.map(|path| path.display().to_string());
            let source_digest = source_path.and_then(|path| {
                library
                    .source_closure
                    .iter()
                    .find(|pin| pin.path == path)
                    .map(|pin| short_digest(&pin.digest.to_string()))
            });
            let blocker = if source.is_none() {
                Some(format!(
                    "corner '{}' is not bound to a retained source",
                    corner.name
                ))
            } else {
                corner_blocker(library, corner)
            };
            rows.push(CornerRow {
                key: format!("{}\u{1f}{}", library.name, corner.name),
                library: library.name.clone(),
                corner: corner.clone(),
                blocker,
                has_statistics,
                has_aging,
                source,
                source_digest,
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
    let mut open_editor = false;
    let mut duplicate = false;
    let mut make_default = false;
    let mut delete = false;
    let mut unbind = None;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        ui.label(
            RichText::new(format!(
                "{} / {}",
                row.library,
                row.corner.name.to_uppercase()
            ))
            .monospace()
            .strong(),
        );
        if let Some(blocker) = row.blocker.as_deref() {
            ui.label(
                RichText::new(format!("run expansion blocked · {blocker}"))
                    .small()
                    .color(Tokens::get(ui.ctx()).color.err),
            );
        }
        if ui.button("Edit corner…").clicked() {
            open_editor = true;
        }
        if ui.button("Duplicate…").clicked() {
            duplicate = true;
        }
        if ui
            .add_enabled(!row.corner.is_default, egui::Button::new("Set default"))
            .clicked()
        {
            make_default = true;
        }
        if ui.button("Delete corner…").clicked() {
            delete = true;
        }
        if ui.button("Bind section…").clicked() {
            open_corner_binding_dialog(app, &row);
        }
        for binding in row.corner.effective_section_bindings() {
            if ui
                .button(format!("Unbind {}", binding.domain.label()))
                .clicked()
            {
                unbind = Some(binding.domain);
            }
        }
        // The corner's own retained file, not whichever model the library
        // happens to iterate first.
        if ui
            .add_enabled(row.source.is_some(), egui::Button::new("Open source"))
            .clicked()
        {
            open_corner_source(app, &row);
        }
        if ui.button("View include graph").clicked() {
            app.state.workbench.models_page = ModelsPage::Include;
        }
        if ui.button("Model editor…").clicked() {
            app.queue_command(Command::ModelEditor);
        }
    });
    if open_editor || duplicate {
        open_corner_editor(app, &row, duplicate);
    } else if make_default {
        set_default_corner(app, &row.library, &row.corner.name);
    } else if delete {
        app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmDeleteCorner {
            library: row.library.clone(),
            corner: row.corner.name.clone(),
        });
    } else if let Some(domain) = unbind {
        unbind_corner_section(app, &row.library, &row.corner.name, domain);
    }
    detail_pane(
        ui,
        "CORNER BINDING DETAILS",
        Some("section, environment, statistics, and aging"),
        |ui| {
            property(
                ui,
                "Description",
                &row.corner.description,
                "project metadata",
            );
            property(
                ui,
                "Default",
                if row.corner.is_default { "yes" } else { "no" },
                "library selection fallback",
            );
            property(ui, "NMOS", &row.corner.nmos_corner, "exact section axis");
            property(ui, "PMOS", &row.corner.pmos_corner, "exact section axis");
            property(
                ui,
                "Source",
                row.source.as_deref().unwrap_or("not bound"),
                if row.resolved() {
                    "retained"
                } else {
                    "unresolved"
                },
            );
            property(
                ui,
                "Source digest",
                row.source_digest.as_deref().unwrap_or("not pinned"),
                "authenticated content identity",
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
            property(
                ui,
                "Qualified range",
                &match (
                    row.corner.minimum_temperature_c,
                    row.corner.maximum_temperature_c,
                ) {
                    (Some(minimum), Some(maximum)) => {
                        format!("{minimum:.3} to {maximum:.3} °C")
                    }
                    _ => "not declared".to_owned(),
                },
                "temperature validity",
            );
            property(
                ui,
                "Required domains",
                &row.corner
                    .effective_required_domains()
                    .into_iter()
                    .map(CornerSectionDomain::label)
                    .collect::<Vec<_>>()
                    .join(", "),
                "execution contract",
            );
            for binding in row.corner.effective_section_bindings() {
                property(
                    ui,
                    binding.domain.label(),
                    &binding.section,
                    "authenticated section",
                );
            }
            ui.separator();
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
                if row.resolved() {
                    "executable"
                } else {
                    "fail closed"
                },
                "run expansion",
            );
            if let Some(receipt) = app.state.model_library_manager.model_validation_receipt() {
                let current_revision = app.state.workspace.project.revision();
                let receipt_state = if receipt.project_revision == current_revision {
                    "current revision"
                } else {
                    "stale revision"
                };
                property(
                    ui,
                    "Validation receipt",
                    &format!(
                        "{} ({receipt_state})",
                        short_digest(&receipt.receipt_digest.to_string())
                    ),
                    &format!(
                        "project revision {} · plan {} · {} authenticated sources · {}",
                        receipt.project_revision.get(),
                        short_digest(&receipt.model_execution_plan_digest.to_string()),
                        receipt.source_count,
                        receipt.platform
                    ),
                );
            }
        },
    );
}

fn open_corner_binding_dialog(app: &mut ManagerRenderContext<'_>, row: &CornerRow) {
    let section = app
        .state
        .model_library_manager
        .get_library(&row.library)
        .and_then(|library| library.section_index().into_iter().next())
        .unwrap_or_default();
    let bindings = row.corner.effective_section_bindings();
    let domain = row
        .corner
        .effective_required_domains()
        .into_iter()
        .find(|required| !bindings.iter().any(|binding| binding.domain == *required))
        .or_else(|| bindings.first().map(|binding| binding.domain))
        .unwrap_or(CornerSectionDomain::Composite);
    app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::BindCornerSection {
        library: row.library.clone(),
        corner: row.corner.name.clone(),
        domain,
        section,
    });
}

fn open_corner_editor(app: &mut ManagerRenderContext<'_>, row: &CornerRow, duplicate: bool) {
    let name = if duplicate {
        let base = format!("{}_copy", row.corner.name);
        let mut candidate = base.clone();
        let mut suffix = 2_u32;
        if let Some(library) = app.state.model_library_manager.get_library(&row.library) {
            while library
                .corners
                .keys()
                .any(|existing| existing.eq_ignore_ascii_case(&candidate))
            {
                candidate = format!("{base}_{suffix}");
                suffix += 1;
            }
        }
        candidate
    } else {
        row.corner.name.clone()
    };
    app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::EditCorner {
        library: row.library.clone(),
        original_name: row.corner.name.clone(),
        duplicate,
        name,
        description: row.corner.description.clone(),
        nmos_corner: row.corner.nmos_corner.clone(),
        pmos_corner: row.corner.pmos_corner.clone(),
        temperature_c: row.corner.temperature.to_string(),
        supply_factor: row.corner.vdd_factor.to_string(),
        minimum_temperature_c: row
            .corner
            .minimum_temperature_c
            .map_or_else(String::new, |value| value.to_string()),
        maximum_temperature_c: row
            .corner
            .maximum_temperature_c
            .map_or_else(String::new, |value| value.to_string()),
        required_domains: row.corner.effective_required_domains(),
        make_default: !duplicate && row.corner.is_default,
    });
}

/// Show the retained bytes of the file this corner is bound to.
fn open_corner_source(app: &mut ManagerRenderContext<'_>, row: &CornerRow) {
    let Some(library) = app
        .state
        .model_library_manager
        .get_library(&row.library)
        .cloned()
    else {
        receipt(
            app,
            Err(format!("Library '{}' no longer exists.", row.library)),
        );
        return;
    };
    let path = row
        .corner
        .file_path
        .as_deref()
        .or(library.root_path.as_deref());
    let Some(path) = path else {
        receipt(
            app,
            Err(format!(
                "Corner '{}' is not bound to a retained source.",
                row.corner.name
            )),
        );
        return;
    };
    let retained = library
        .source_contents
        .iter()
        .find(|content| content.path == path);
    match retained {
        Some(content) => {
            app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::SourcePreview {
                title: format!("{} / {}", library.name, row.corner.name.to_uppercase()),
                subtitle: format!("{} · retained closure member", content.path.display()),
                source: String::from_utf8_lossy(&content.bytes).into_owned(),
                editable: false,
            });
        }
        None => match std::fs::read_to_string(path) {
            Ok(source) => {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::SourcePreview {
                        title: format!("{} / {}", library.name, row.corner.name.to_uppercase()),
                        subtitle: format!("{} · live unpinned source", path.display()),
                        source,
                        editable: false,
                    });
            }
            Err(error) => receipt(
                app,
                Err(format!(
                    "Could not read corner source '{}': {error}",
                    path.display()
                )),
            ),
        },
    }
}

fn select_corner(app: &mut ManagerRenderContext<'_>, library_name: &str, corner_name: &str) {
    app.state.model_library_manager.select_library(library_name);
    let mut candidate = app.state.model_library_manager.clone();
    let result = candidate
        .get_library_mut(library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))
        .and_then(|library| {
            library
                .select_corner(corner_name)
                .then_some(())
                .ok_or_else(|| {
                    format!(
                        "Corner '{corner_name}' no longer exists in library '{library_name}'."
                    )
                })
        })
        .and_then(|()| {
            publish_model_library_candidate(
                app.state,
                candidate,
                library_name,
                format!("select model corner {corner_name}"),
            )
        })
        .map(|revision| {
            format!(
                "Selected exact corner '{corner_name}' for '{library_name}' at project revision {}.",
                revision.get()
            )
        });
    receipt(app, result);
}

fn symbol_key(reference: &CellViewRef) -> String {
    format!(
        "{}\u{1f}{}\u{1f}{}",
        reference.library, reference.cell, reference.view
    )
}

/// One card's place in a bin family, without the card.
struct BinCard {
    model: String,
    envelope: GeometryEnvelope,
}

/// The set of cards the engine would consider for one instance reference.
///
/// Keyed by library and by the family base name — `nch` for `nch.1`, `nch.2` —
/// because that is the set core's `resolve_binned_model_def` collects
/// candidates from. Grouping by device type instead, which this page used to
/// do, put every NMOS card in every attached foundry library into one family
/// and reported them as overlapping each other.
struct BinFamily {
    library: String,
    family: String,
    cards: Vec<BinCard>,
}

impl BinFamily {
    fn key(&self) -> String {
        format!("{} · {}", self.library, self.family)
    }
}

fn bin_families(app: &ManagerRenderContext<'_>) -> Vec<BinFamily> {
    let mut families = BTreeMap::<(&str, &str), Vec<BinCard>>::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        for model in library.models.values() {
            let envelope = GeometryEnvelope::of(model);
            if !envelope.is_declared() {
                continue;
            }
            families
                .entry((library.name.as_str(), bin_family_name(&model.name)))
                .or_default()
                .push(BinCard {
                    model: model.name.clone(),
                    envelope,
                });
        }
    }
    families
        .into_iter()
        .map(|((library, family), mut cards)| {
            cards.sort_by(|left, right| left.model.cmp(&right.model));
            BinFamily {
                library: library.to_owned(),
                family: family.to_owned(),
                cards,
            }
        })
        .collect()
}

pub(super) fn bins_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let families = bin_families(app);
    let findings = geometry_findings(&families);
    section_title(
        ui,
        "Bins & geometry",
        &format!(
            "{} binned families · {} cards · {} findings",
            families.len(),
            families
                .iter()
                .map(|family| family.cards.len())
                .sum::<usize>(),
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
            // Gated on the fact it traces — the selected model — rather than
            // on a schematic selection it never reads, and it says which.
            let traced = app.state.workbench.selected_model.clone();
            if ui
                .add_enabled(traced.is_some(), egui::Button::new("Trace schematic"))
                .on_disabled_hover_text("Select a model in the catalog first.")
                .clicked()
                && let Some(model) = traced
            {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::BindingTrace {
                        consumers: model_consumers(app, &model),
                        model,
                    });
            }
        },
    );
    if families.is_empty() {
        page_empty_state(
            ui,
            "No geometry envelopes are loaded",
            "Attach a binned PDK library or author L/W bounds in Model Editor.",
        );
        return;
    }
    ui.columns(2, |columns| {
        card(&mut columns[0], |ui| {
            card_title(
                ui,
                "BIN FAMILIES",
                Some(&format!("{} · library · family", families.len())),
            );
            let selected = app
                .state
                .workbench
                .models_view
                .selected_bin_family
                .clone()
                .unwrap_or_default();
            let mut picked = None;
            let row_height = selectable_label_height(ui);
            ScrollArea::vertical()
                .id_salt("models-bin-families")
                .max_height(ui.available_height().max(120.0))
                .show_rows(ui, row_height, families.len(), |ui, range| {
                    for family in &families[range] {
                        let key = family.key();
                        if ui
                            .selectable_label(
                                selected == key,
                                format!("{key}  ·  {} cards", family.cards.len()),
                            )
                            .clicked()
                        {
                            picked = Some(key);
                        }
                    }
                });
            if let Some(picked) = picked {
                app.state.workbench.models_view.selected_bin_family = Some(picked);
            }
        });
        card(&mut columns[1], |ui| {
            card_title(
                ui,
                "AUDIT FINDINGS",
                Some(&format!("{} · fail closed on overlap", findings.len())),
            );
            if findings.is_empty() {
                ui.label(
                    RichText::new("Every declared envelope is non-overlapping.")
                        .color(Tokens::get(ui.ctx()).color.ok),
                );
            } else {
                for finding in findings.iter().take(FINDING_ROWS) {
                    ui.label(
                        RichText::new(format!("Review: {finding}"))
                            .small()
                            .color(Tokens::get(ui.ctx()).color.err),
                    );
                }
                if findings.len() > FINDING_ROWS {
                    ui.label(
                        RichText::new(format!(
                            "…and {} more, all counted above",
                            findings.len() - FINDING_ROWS
                        ))
                        .small()
                        .color(Tokens::get(ui.ctx()).color.text_faint),
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
        .filter(|selected| families.iter().any(|family| family.key() == *selected))
        .or_else(|| families.first().map(BinFamily::key));
    if let Some(selected) = selected {
        app.state.workbench.models_view.selected_bin_family = Some(selected.clone());
        if let Some(family) = families.iter().find(|family| family.key() == selected) {
            geometry_map(ui, &selected, &family.cards);
            geometry_instance_table(ui, app, &family.cards);
        }
    }
}

/// Findings the audit reports, comparing only cards the engine would compare.
///
/// The pairwise sweep is quadratic in a family, which is why the family has to
/// be the engine's — a few dozen bins of one device — and not every card of one
/// device type in the whole corpus.
fn geometry_findings(families: &[BinFamily]) -> Vec<String> {
    let mut findings = Vec::new();
    for family in families {
        for (index, left) in family.cards.iter().enumerate() {
            if left.envelope.is_invalid() {
                findings.push(format!(
                    "{}/{} has an inverted L/W envelope",
                    family.family, left.model
                ));
            }
            for right in family.cards.iter().skip(index + 1) {
                if left.envelope.overlaps(right.envelope) {
                    findings.push(format!(
                        "{}/{} overlaps {}",
                        family.family, left.model, right.model
                    ));
                }
            }
        }
    }
    findings
}

fn geometry_map(ui: &mut Ui, family: &str, cards: &[BinCard]) {
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
            .filter_map(|card| {
                Some((
                    card.envelope.l_min?.max(f64::MIN_POSITIVE).log10(),
                    card.envelope.l_max?.max(f64::MIN_POSITIVE).log10(),
                    card.envelope.w_min?.max(f64::MIN_POSITIVE).log10(),
                    card.envelope.w_max?.max(f64::MIN_POSITIVE).log10(),
                    card.model.as_str(),
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

fn geometry_instance_table(ui: &mut Ui, app: &ManagerRenderContext<'_>, cards: &[BinCard]) {
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
            .map(|card| card.model.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let matches = schematic
            .components
            .iter()
            .filter_map(|component| {
                let model = explicit_component_model(component)?;
                names
                    .contains(&model.to_ascii_lowercase())
                    .then_some((component, model))
            })
            .collect::<Vec<_>>();
        if matches.is_empty() {
            empty_state(
                ui,
                "No placed instance resolves to this family.",
                "Binding resolution is derived from the active schematic, not fixture counts.",
            );
            return;
        }
        let row_height = ui.spacing().interact_size.y;
        ScrollArea::vertical()
            .id_salt("models-bin-instances")
            .max_height(ui.available_height().max(120.0))
            .show_rows(ui, row_height, matches.len(), |ui, range| {
                for (component, model) in &matches[range] {
                    property(ui, &component.name, model, &component.params);
                }
            });
    });
}

pub(super) fn include_page(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    diagnostics: &ClosureFacts,
) {
    section_title(
        ui,
        "Model include graph",
        &format!(
            "{} files · {} edges · {} diagnostics",
            diagnostics.files,
            diagnostics.edges,
            diagnostics.diagnostics()
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
        ui.spacing_mut().item_spacing.x = 8.0;
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
    // Both panes below read the libraries while the page also writes selection
    // state, so what crosses that line is the projection each pane paints — a
    // handful of nodes and a name index. Cloning the libraries themselves to
    // dodge the borrow, which is what this did, copied the whole model corpus
    // and its retained source bytes on every frame.
    if app.state.model_library_manager.library_count() == 0 {
        page_empty_state(
            ui,
            "No model-source closure is loaded",
            "Add a library or attach a pack to build an authenticated include graph.",
        );
        return;
    }
    let nodes = closure_nodes(app);
    let definitions = definition_index(app);
    include_closure_graph(ui, app, &nodes, diagnostics);
    include_definition_table(ui, app, &definitions);
}

/// Nodes the closure graph draws before it stops.
///
/// A closure can hold hundreds of sources and this pane is a few hundred
/// pixels tall. What is dropped is always reported as a count — a graph that
/// silently stops at twelve reads as a complete graph of twelve.
const GRAPH_NODE_LIMIT: usize = 12;

/// Geometry findings the audit column lists before reporting the remainder.
const FINDING_ROWS: usize = 10;

/// Whether a source is the library's root or something the root includes
/// itself, as opposed to a transitive member reached through another file.
fn is_direct_closure_member(library: &ModelLibrary, path: &Path) -> bool {
    let Some(root) = library.root_path.as_deref() else {
        return false;
    };
    root == path
        || library
            .source_edges
            .iter()
            .any(|edge| edge.owner == root && edge.target == path)
}

/// One retained source the closure graph draws.
struct ClosureNode {
    path: PathBuf,
    library: String,
    digest: String,
}

/// The nodes and edges the graph pane draws, and what it left out.
struct ClosureGraph {
    nodes: Vec<ClosureNode>,
    /// Sources that passed the filter, drawn or not.
    matching: usize,
    /// Edges between two drawn nodes.
    edges: Vec<(PathBuf, PathBuf)>,
}

fn closure_nodes(app: &ManagerRenderContext<'_>) -> ClosureGraph {
    // "Direct dependencies only" means the root of each library plus whatever
    // the root itself includes; anything reached through another file is a
    // transitive member and folds away.
    let direct_only = app.state.workbench.models_view.include_direct_only;
    let libraries = app.state.model_library_manager.libraries_sorted();
    let mut nodes = Vec::new();
    let mut matching = 0usize;
    for library in &libraries {
        for source in &library.source_closure {
            if direct_only && !is_direct_closure_member(library, &source.path) {
                continue;
            }
            matching += 1;
            if nodes.len() < GRAPH_NODE_LIMIT {
                nodes.push(ClosureNode {
                    path: source.path.clone(),
                    library: library.name.clone(),
                    digest: short_digest(&source.digest.to_string()),
                });
            }
        }
    }
    let drawn = nodes
        .iter()
        .map(|node| node.path.clone())
        .collect::<BTreeSet<_>>();
    let edges = libraries
        .iter()
        .flat_map(|library| &library.source_edges)
        .filter(|edge| drawn.contains(&edge.owner) && drawn.contains(&edge.target))
        .map(|edge| (edge.owner.clone(), edge.target.clone()))
        .collect();
    ClosureGraph {
        nodes,
        matching,
        edges,
    }
}

fn include_closure_graph(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    graph: &ClosureGraph,
    diagnostics: &ClosureFacts,
) {
    let direct_only = app.state.workbench.models_view.include_direct_only;
    detail_pane(
        ui,
        "RESOLVED CLOSURE",
        Some(if direct_only {
            "root plus direct dependencies"
        } else {
            "root plus authenticated dependencies"
        }),
        |ui| {
            let graph_height = (ui.available_height() * 0.42).clamp(150.0, 230.0);
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), graph_height),
                Sense::hover(),
            );
            let t = Tokens::get(ui.ctx());
            ui.painter().rect(
                rect,
                2.0,
                t.color.bg_inset,
                Stroke::new(1.0, t.color.border),
                egui::StrokeKind::Inside,
            );

            let sources = &graph.nodes;
            let matching = graph.matching;
            let hidden = matching.saturating_sub(GRAPH_NODE_LIMIT);
            if sources.is_empty() {
                ui.painter().text(
                    egui::pos2(rect.center().x, rect.center().y - 10.0),
                    egui::Align2::CENTER_CENTER,
                    "No retained include closure",
                    theme::sans(tokens::FS_1, FontWeight::SemiBold),
                    t.color.text_dim,
                );
                ui.painter().text(
                    egui::pos2(rect.center().x, rect.center().y + 14.0),
                    egui::Align2::CENTER_CENTER,
                    "Loaded definitions are built-in or have no authenticated source graph.",
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_faint,
                );
            } else {
                let node_width = ((rect.width() - 54.0) / 3.0).clamp(120.0, 210.0);
                let node_height = 38.0;
                let columns = 3usize;
                let row_count = sources.len().div_ceil(columns);
                let row_gap = if row_count > 1 {
                    ((rect.height() - 36.0 - node_height * row_count as f32)
                        / (row_count - 1) as f32)
                        .clamp(8.0, 24.0)
                } else {
                    0.0
                };
                let x_gap = ((rect.width() - node_width * columns as f32) / 4.0).max(8.0);
                let mut node_rects = BTreeMap::new();
                for (index, source) in sources.iter().enumerate() {
                    let column = index % columns;
                    let row = index / columns;
                    let x = rect.left() + x_gap + column as f32 * (node_width + x_gap);
                    let y = rect.top() + 18.0 + row as f32 * (node_height + row_gap);
                    let node = egui::Rect::from_min_size(
                        egui::pos2(x, y),
                        egui::vec2(node_width, node_height),
                    );
                    node_rects.insert(source.path.clone(), node);
                    let selected = app
                        .state
                        .workbench
                        .models_view
                        .include_selected_source
                        .as_deref()
                        == Some(source.path.to_string_lossy().as_ref());
                    ui.painter().rect(
                        node,
                        3.0,
                        if selected {
                            t.color.accent.linear_multiply(0.16)
                        } else {
                            t.color.bg_panel
                        },
                        Stroke::new(
                            if selected { 1.5 } else { 1.0 },
                            if selected {
                                t.color.accent
                            } else {
                                t.color.border
                            },
                        ),
                        egui::StrokeKind::Inside,
                    );
                    ui.painter().text(
                        egui::pos2(node.left() + 8.0, node.top() + 12.0),
                        egui::Align2::LEFT_CENTER,
                        elide(ui, &path_label(&source.path), node.width() - 16.0, true),
                        theme::mono(tokens::FS_0, FontWeight::SemiBold),
                        t.color.text,
                    );
                    ui.painter().text(
                        egui::pos2(node.left() + 8.0, node.bottom() - 10.0),
                        egui::Align2::LEFT_CENTER,
                        elide(
                            ui,
                            &format!("{} · {}", source.library, source.digest),
                            node.width() - 16.0,
                            false,
                        ),
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text_faint,
                    );
                    let response = ui.interact(
                        node,
                        ui.id()
                            .with(("models-include-node", source.path.as_os_str())),
                        Sense::click(),
                    );
                    let node_label = format!(
                        "{} · {} · {}",
                        path_label(&source.path),
                        source.library,
                        source.digest
                    );
                    response.widget_info(|| {
                        egui::WidgetInfo::selected(
                            egui::WidgetType::SelectableLabel,
                            ui.is_enabled(),
                            selected,
                            node_label.clone(),
                        )
                    });
                    crate::ui::theme::paint_focus_ring(ui, &response, node);
                    if response.clicked() {
                        app.state.workbench.models_view.include_selected_source =
                            Some(source.path.to_string_lossy().into_owned());
                    }
                }

                for (edge_owner, edge_target) in &graph.edges {
                    if let (Some(owner), Some(target)) =
                        (node_rects.get(edge_owner), node_rects.get(edge_target))
                    {
                        ui.painter().arrow(
                            owner.center_bottom(),
                            target.center_top() - owner.center_bottom(),
                            Stroke::new(1.0, t.color.text_faint),
                        );
                    }
                }
            }

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 14.0;
                ui.label(
                    RichText::new(format!("{} pinned", diagnostics.files))
                        .small()
                        .color(t.color.text_dim),
                );
                if hidden > 0 {
                    ui.label(
                        RichText::new(format!(
                            "showing {} of {matching} · {hidden} not drawn",
                            GRAPH_NODE_LIMIT
                        ))
                        .small()
                        .color(t.color.warn),
                    );
                }
                ui.label(
                    RichText::new(format!("{} dependency edges", diagnostics.edges))
                        .small()
                        .color(t.color.text_dim),
                );
                ui.label(
                    RichText::new(format!(
                        "{} unpinned · {} cyclic",
                        diagnostics.unpinned_roots, diagnostics.cyclic_nodes
                    ))
                    .small()
                    .color(if diagnostics.diagnostics() == 0 {
                        t.color.ok
                    } else {
                        t.color.err
                    }),
                );
            });
        },
    );
}

/// One name an instance could reference, and everything that defines it.
struct DefinitionRow {
    definition: String,
    scope: crate::state::model_library::ModelConsumerScope,
    providers: Vec<String>,
    provider_list: String,
    resolution: String,
}

impl DefinitionRow {
    /// A contested name has no winner: the duplicate has to be removed or
    /// renamed before an instance can bind at all.
    fn contested(&self) -> bool {
        self.providers.len() > 1
    }
}

/// Every definition name across the loaded libraries, with its providers.
///
/// Model names and subcircuit names share one namespace as far as an instance
/// reference is concerned, so both are here for "contested" to mean anything.
fn definition_index(app: &ManagerRenderContext<'_>) -> Vec<DefinitionRow> {
    use crate::state::model_library::ModelConsumerScope;
    let mut providers = BTreeMap::<(ModelConsumerScope, String), BTreeSet<String>>::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        for model in library.models.values() {
            providers
                .entry((
                    ModelConsumerScope::PrimitiveModel,
                    model.name.to_ascii_lowercase(),
                ))
                .or_default()
                .insert(library.name.clone());
        }
        for subcircuit in library.subcircuits.values() {
            if subcircuit.section.is_none() {
                providers
                    .entry((
                        ModelConsumerScope::Subcircuit,
                        subcircuit.name.to_ascii_lowercase(),
                    ))
                    .or_default()
                    .insert(library.name.clone());
            }
        }
    }
    providers
        .into_iter()
        .map(|((scope, definition), candidates)| {
            let resolution = app
                .state
                .model_library_manager
                .model_resolution_record(scope, &definition)
                .map_or_else(
                    || {
                        if candidates.len() > 1 {
                            "contested · fails closed".to_owned()
                        } else {
                            "unique".to_owned()
                        }
                    },
                    |record| format!("resolved · {}", record.provider_library),
                );
            DefinitionRow {
                definition,
                scope,
                provider_list: candidates.iter().cloned().collect::<Vec<_>>().join(", "),
                providers: candidates.into_iter().collect(),
                resolution,
            }
        })
        .collect()
}

fn include_definition_table(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    definitions: &[DefinitionRow],
) {
    let query = app
        .state
        .workbench
        .models_view
        .include_definition_query
        .trim()
        .to_ascii_lowercase();
    let matching = definitions
        .iter()
        .filter(|row| {
            query.is_empty()
                || row.definition.contains(&query)
                || row
                    .providers
                    .iter()
                    .any(|provider| provider.to_ascii_lowercase().contains(&query))
        })
        .collect::<Vec<_>>();
    let contested_count = definitions.iter().filter(|row| row.contested()).count();
    let mut conflict = None;
    card(ui, |ui| {
        card_title(
            ui,
            "DEFINITION RESOLUTION",
            Some(&format!(
                "{} unique names · {contested_count} contested",
                definitions.len()
            )),
        );
        table_header(
            ui,
            &[
                ("DEFINITION", 0.25),
                ("KIND", 0.13),
                ("PROVIDERS", 0.37),
                ("RESOLUTION", 0.25),
            ],
        );
        if matching.is_empty() {
            empty_state(
                ui,
                "No definitions match.",
                "The filter searches definition names and every retained provider.",
            );
            return;
        }
        ScrollArea::vertical()
            .id_salt("models-include-definitions")
            .max_height(ui.available_height().max(140.0))
            .show_rows(ui, ROW_H, matching.len(), |ui, range| {
                for row in &matching[range] {
                    // This column used to print the first provider under the
                    // heading "WINNING PROVIDER", which asserted a resolution
                    // policy that does not exist.
                    let response = selectable_data_row(
                        ui,
                        false,
                        &[
                            (&row.definition, 0.25, true),
                            (row.scope.label(), 0.13, false),
                            (&row.provider_list, 0.37, false),
                            (&row.resolution, 0.25, true),
                        ],
                    );
                    if row.contested() && response.clicked() {
                        conflict = Some((row.definition.clone(), row.scope, row.providers.clone()));
                    }
                }
            });
    });
    if let Some((definition, scope, providers)) = conflict {
        let selected_provider = app
            .state
            .model_library_manager
            .model_resolution_record(scope, &definition)
            .map(|record| record.provider_library.clone())
            .or_else(|| providers.first().cloned())
            .unwrap_or_default();
        app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::DefinitionConflict {
            definition,
            scope,
            providers,
            selected_provider,
            reason: String::new(),
        });
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::state::model_library::{
        CornerSectionBinding, CornerSectionDomain, DeviceModel, ModelSourceContent, ModelSourcePin,
        ModelType,
    };

    /// A library whose parsed models declare `sections`, and whose retained
    /// bytes deliberately disagree with them.
    fn library_with_sections(sections: &[&str], retained_bytes: &str) -> ModelLibrary {
        let mut library = ModelLibrary::new("pdk");
        library.root_path = Some(PathBuf::from("pdk.lib"));
        for section in sections {
            let mut model = DeviceModel::new(format!("nch_{section}"), ModelType::Nmos);
            model.section = Some((*section).to_owned());
            library.add_model(model);
        }
        library.source_closure = vec![ModelSourcePin {
            path: PathBuf::from("pdk.lib"),
            digest: crate::product::ContentDigest::from_bytes([0x11; 32]),
        }];
        library.source_contents = vec![ModelSourceContent {
            path: PathBuf::from("pdk.lib"),
            bytes: retained_bytes.as_bytes().to_vec(),
        }];
        library
    }

    fn corner_bound_to(name: &str, section: &str) -> ProcessCorner {
        let mut corner = ProcessCorner::new(name);
        corner.file_path = Some(PathBuf::from("pdk.lib"));
        corner.section_bindings = vec![CornerSectionBinding::new(
            CornerSectionDomain::Composite,
            section,
        )];
        corner
    }

    #[test]
    fn a_section_spelled_with_a_tab_still_resolves() {
        // The byte search this replaced looked for the literal `.lib tt`, so a
        // file writing `.LIB\ttt` reported the corner unresolved and told the
        // engineer a run was blocked when it was not.
        let library = library_with_sections(&["tt"], ".LIB\ttt\n.model nch_tt nmos\n.ENDL\n");
        assert_eq!(corner_blocker(&library, &corner_bound_to("tt", "tt")), None);
    }

    #[test]
    fn a_section_name_appearing_only_in_a_comment_does_not_resolve() {
        // And the same search reported a section present whenever its name
        // appeared anywhere in the retained bytes, including a comment.
        let library = library_with_sections(&["tt"], "* see .lib ff for the fast corner\n");
        let blocker = corner_blocker(&library, &corner_bound_to("ff", "ff"))
            .expect("a section nothing defines must block run expansion");
        assert!(
            blocker.contains("'ff'"),
            "the blocker must name the missing section: {blocker}"
        );
    }

    #[test]
    fn a_corner_binding_several_domains_needs_every_section() {
        let library = library_with_sections(&["tt", "res_tt"], "");
        let mut corner = ProcessCorner::new("tt");
        corner.file_path = Some(PathBuf::from("pdk.lib"));
        corner.section_bindings = vec![
            CornerSectionBinding::new(CornerSectionDomain::Mos, "tt"),
            CornerSectionBinding::new(CornerSectionDomain::Passives, "res_tt"),
        ];
        assert_eq!(corner_blocker(&library, &corner), None);

        corner.section_bindings = vec![
            CornerSectionBinding::new(CornerSectionDomain::Mos, "tt"),
            CornerSectionBinding::new(CornerSectionDomain::Passives, "res_ss"),
        ];
        let blocker = corner_blocker(&library, &corner).expect("one missing section blocks");
        assert!(blocker.contains("'res_ss'"), "{blocker}");
        assert!(
            !blocker.contains("'tt'"),
            "a resolved axis must not be reported as missing: {blocker}"
        );
    }

    #[test]
    fn a_legacy_corner_with_no_declared_binding_resolves_through_its_own_name() {
        // `effective_section_bindings` synthesises a composite binding named
        // for the corner when a source-backed corner declares none. The page
        // must follow that, not special-case `tt`.
        let library = library_with_sections(&["ss"], "");
        let mut corner = ProcessCorner::new("ss");
        corner.file_path = Some(PathBuf::from("pdk.lib"));
        assert_eq!(corner_blocker(&library, &corner), None);

        let mut absent = ProcessCorner::new("ff");
        absent.file_path = Some(PathBuf::from("pdk.lib"));
        assert!(corner_blocker(&library, &absent).is_some());
    }

    #[test]
    fn the_lone_typical_corner_is_no_longer_privileged() {
        // The rule this replaced resolved any single corner named `tt`
        // regardless of whether the closure defined it.
        let library = library_with_sections(&["ss"], "");
        let mut typical = ProcessCorner::new("tt");
        typical.file_path = Some(PathBuf::from("pdk.lib"));
        assert!(
            corner_blocker(&library, &typical).is_some(),
            "a lone 'tt' with no matching section must not resolve"
        );
    }

    #[test]
    fn a_corner_bound_to_nothing_at_all_reports_no_executable_bindings() {
        let library = library_with_sections(&["tt"], "");
        let unbound = ProcessCorner::new("floating");
        let blocker = corner_blocker(&library, &unbound)
            .expect("a retained closure with no binding is not executable");
        assert!(
            blocker.contains("no executable section bindings"),
            "the page must use the run's own wording: {blocker}"
        );

        // With no retained closure and no file there is nothing to resolve, so
        // there is nothing to report either.
        let bare = ModelLibrary::new("in-memory");
        assert_eq!(corner_blocker(&bare, &ProcessCorner::new("floating")), None);
    }

    #[test]
    fn corner_lifecycle_publishes_drafts_bindings_defaults_and_deletion() {
        let mut state = AppState::default();
        state.project_lifecycle.project_open = true;
        let library = state
            .model_library_manager
            .load_library_bytes(
                "corner-lifecycle.lib",
                b".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n".to_vec(),
                None,
            )
            .expect("sectioned source imports");
        let initial_revision = state.workspace.project.revision();
        let mut pending = Vec::new();
        let mut app = ManagerRenderContext {
            state: &mut state,
            pending_actions: &mut pending,
        };

        add_corner(&mut app, &library, "hot", "125", "0.9");
        let draft = app
            .state
            .model_library_manager
            .get_library(&library)
            .and_then(|library| library.corners.get("hot"))
            .expect("draft corner publishes");
        draft
            .validate_draft_contract()
            .expect("unbound draft remains persistable");
        assert!(draft.validate_contract().is_err());

        bind_corner_section(
            &mut app,
            &library,
            "hot",
            CornerSectionDomain::Composite,
            "TT",
        );
        app.state
            .model_library_manager
            .get_library(&library)
            .and_then(|library| library.corners.get("hot"))
            .expect("bound corner remains present")
            .validate_contract()
            .expect("exact authenticated section makes the corner executable");

        unbind_corner_section(&mut app, &library, "hot", CornerSectionDomain::Composite);
        let unbound = app
            .state
            .model_library_manager
            .get_library(&library)
            .and_then(|library| library.corners.get("hot"))
            .expect("unbound draft remains present");
        assert!(unbound.validate_draft_contract().is_ok());
        assert!(unbound.validate_contract().is_err());

        set_default_corner(&mut app, &library, "hot");
        delete_corner(&mut app, &library, "hot");
        let retained = app
            .state
            .model_library_manager
            .get_library(&library)
            .expect("library remains attached");
        assert!(!retained.corners.contains_key("hot"));
        assert!(retained.corners.values().any(|corner| corner.is_default));
        assert!(app.state.workspace.project.revision() > initial_revision);
        assert!(app.state.workspace.project_metadata_dirty);
    }

    #[test]
    fn the_family_list_declares_the_height_a_row_really_takes() {
        // `show_rows` places rows from the height it is given. If that height
        // is short, every row after the first drifts up under the one above
        // and the last families fall off the end of the list.
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut declared = 0.0;
        let mut measured = 0.0;
        for _ in 0..2 {
            let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    declared = selectable_label_height(ui);
                    measured = ui
                        .selectable_label(false, "pdk7 · nch7  ·  25 cards")
                        .rect
                        .height();
                });
            });
        }
        assert!(
            (declared - measured).abs() < 0.01,
            "the family list declares {declared} per row but a row takes {measured}"
        );
    }

    #[test]
    fn a_family_is_one_library_s_cards_sharing_a_base_name() {
        use crate::workbench::surfaces::models::scale;

        let mut app = scale::large_corpus_app();
        let mut pending = Vec::new();
        let render = ManagerRenderContext {
            state: &mut app.state,
            pending_actions: &mut pending,
        };
        let families = bin_families(&render);
        assert_eq!(
            families.len(),
            (scale::LIBRARIES - scale::PROJECT_LIBRARIES) * scale::FAMILIES.len(),
            "cards group by library and by the name before their last dot"
        );
        assert!(
            families
                .iter()
                .all(|family| family.cards.len()
                    == scale::MODELS_PER_LIBRARY / scale::FAMILIES.len()),
            "every family in the fixture holds the same number of bins"
        );
    }

    #[test]
    fn a_correctly_binned_pdk_produces_no_geometry_findings() {
        // Every PDK library in the fixture tiles its L axis, sharing a
        // boundary between adjacent bins, which is what a real binned library
        // does. Grouping cards by device type instead of by bin family
        // reported all 4,500 of them as overlapping one another.
        use crate::workbench::surfaces::models::scale;

        let mut app = scale::large_corpus_app();
        let mut pending = Vec::new();
        let render = ManagerRenderContext {
            state: &mut app.state,
            pending_actions: &mut pending,
        };
        let findings = geometry_findings(&bin_families(&render));
        assert!(
            findings.is_empty(),
            "a correctly binned corpus has no findings, got {}: {:?}",
            findings.len(),
            &findings[..findings.len().min(5)]
        );
    }
}
