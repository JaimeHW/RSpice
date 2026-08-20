//! Modal dialogs the Models & PDKs workspace opens.
//!
//! Each arm renders one `ModelsWorkbenchDialog` variant and then does one of
//! three things: clears the field, writes the edited variant back so the next
//! frame keeps the typed text, or hands the decision to the transaction that
//! owns it. The transactions live beside this module; what is here is the
//! asking.

use super::corner_ops::{add_corner, bind_corner_section, delete_corner, edit_corner};
use super::*;

pub(super) fn render_dialog(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let Some(dialog) = app.state.workbench.models_view.dialog.clone() else {
        return;
    };
    match dialog {
        #[cfg(target_arch = "wasm32")]
        ModelsWorkbenchDialog::SelectBrowserImportRoot {
            candidates,
            selected,
        } => {
            let mut open = true;
            let mut selection = selected.min(candidates.len().saturating_sub(1));
            let mut import = false;
            let mut cancel = false;
            egui::Window::new("Choose model-library entry")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "Choose the one SPICE or Spectre source that owns this import. Only its reachable include closure will be authenticated and retained.",
                    );
                    egui::ComboBox::from_id_salt("browser-model-import-root")
                        .selected_text(
                            candidates
                                .get(selection)
                                .map(String::as_str)
                                .unwrap_or("No supported entry"),
                        )
                        .show_ui(ui, |ui| {
                            for (index, candidate) in candidates.iter().enumerate() {
                                ui.selectable_value(&mut selection, index, candidate);
                            }
                        });
                    ui.horizontal(|ui| {
                        cancel = ui.button("Cancel").clicked();
                        import = ui
                            .add_enabled(!candidates.is_empty(), egui::Button::new("Import"))
                            .clicked();
                    });
                });
            if import {
                if let Some(root) = candidates.get(selection).cloned() {
                    app.queue_browser_import_root(root);
                }
            } else if cancel || !open {
                app.queue_cancel_browser_import_root();
            } else if selection != selected {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::SelectBrowserImportRoot {
                        candidates,
                        selected: selection,
                    });
            }
        }
        ModelsWorkbenchDialog::SourcePreview {
            title,
            subtitle,
            source,
            editable,
        } => {
            let mut open = true;
            let mut edit = false;
            egui::Window::new(title)
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .default_size(egui::vec2(760.0, 520.0))
                .show(ui.ctx(), |ui| {
                    ui.label(RichText::new(subtitle).monospace().small());
                    ui.separator();
                    // A preview, always. The text stays selectable and
                    // copyable, but it is never a writable buffer: this
                    // dialog holds a per-frame clone of the retained bytes and
                    // has nowhere to write an edit back to, so an interactive
                    // field here accepted keystrokes and dropped every one of
                    // them. Authoring goes through Model Editor, which owns
                    // validation and revision history.
                    let mut body = source.as_str();
                    ScrollArea::both().show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut body)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                                .desired_rows(26),
                        );
                    });
                    ui.horizontal(|ui| {
                        if editable {
                            edit = ui.button("Edit in Model Editor…").clicked();
                            ui.label(
                                RichText::new("Editing publishes one validated project revision.")
                                    .small()
                                    .color(Tokens::get(ui.ctx()).color.text_faint),
                            );
                        } else {
                            ui.label(
                                RichText::new(
                                    "Read-only: this source is not owned by the project.",
                                )
                                .small()
                                .color(Tokens::get(ui.ctx()).color.text_faint),
                            );
                        }
                    });
                });
            if edit {
                app.queue_command(Command::ModelEditor);
                app.state.workbench.models_view.dialog = None;
            } else if !open {
                app.state.workbench.models_view.dialog = None;
            }
        }
        ModelsWorkbenchDialog::CompareModels {
            left_library,
            left_model,
            right,
        } => {
            let mut open = true;
            let mut chosen = right.clone();
            egui::Window::new("Compare model definitions")
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .default_size(egui::vec2(760.0, 500.0))
                .show(ui.ctx(), |ui| {
                    comparison_counterpart_picker(
                        ui,
                        app,
                        &left_library,
                        &left_model,
                        &mut chosen,
                    );
                    ui.separator();
                    match chosen.as_ref() {
                        Some((right_library, right_model)) => compare_models(
                            ui,
                            app,
                            &left_library,
                            &left_model,
                            right_library,
                            right_model,
                        ),
                        None => empty_state(
                            ui,
                            "Choose a definition to compare against.",
                            "There is no meaningful default beyond the same card in another library.",
                        ),
                    }
                });
            if chosen != right {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::CompareModels {
                        left_library,
                        left_model,
                        right: chosen,
                    });
            } else if !open {
                app.state.workbench.models_view.dialog = None;
            }
        }
        ModelsWorkbenchDialog::ConfirmPack {
            pack_id,
            attach,
            release,
        } => {
            let mut open = true;
            let mut decision = None;
            egui::Window::new(match (release.as_deref(), attach) {
                (Some(_), _) => "Install model pack",
                (None, true) => "Attach model pack",
                (None, false) => "Detach model pack",
            })
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                if let Some(release) = release.as_deref() {
                    decision = hub::release_confirmation(ui, app, &pack_id, release);
                    return;
                }
                ui.label(if attach {
                    "RSpice will authenticate the pack entry and publish its retained include closure as one undoable project revision."
                } else {
                    "RSpice will remove the attached source as one undoable project revision. Existing instance references may become unresolved."
                });
                ui.label(RichText::new(&pack_id).monospace().strong());
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        decision = Some(false);
                    }
                    if ui
                        .add_enabled(
                            !app.state.workbench.models_view.model_import_in_progress,
                            egui::Button::new(if attach { "Attach pack" } else { "Detach pack" }),
                        )
                        .clicked()
                    {
                        decision = Some(true);
                    }
                });
            });
            if decision == Some(true) {
                match release.as_deref() {
                    Some(release) => {
                        let request = hub::release_request(&pack_id, release);
                        app.queue_model_hub(request);
                    }
                    None if attach => attach_pack(app, &pack_id),
                    None => detach_pack(app, &pack_id),
                }
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            }
        }
        ModelsWorkbenchDialog::ConfirmPart { pack_id, part_name } => {
            let mut open = true;
            let mut decision = None;
            egui::Window::new("Add shipped part to project")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "The exact installed source file will be parsed, license-checked, pinned by digest, and published as one undoable project revision.",
                    );
                    ui.label(
                        RichText::new(format!("{part_name} · {pack_id}"))
                            .monospace()
                            .strong(),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            decision = Some(false);
                        }
                        if ui
                            .add_enabled(
                                !app.state.workbench.models_view.model_import_in_progress,
                                egui::Button::new("Add to project"),
                            )
                            .clicked()
                        {
                            decision = Some(true);
                        }
                    });
                });
            if decision == Some(true) {
                add_part(app, &pack_id, &part_name);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            }
        }
        ModelsWorkbenchDialog::AuthorTechnologySymbolVariant {
            package_id,
            source_cell,
            mut target_library,
            mut target_cell,
        } => {
            let mut open = true;
            let mut decision = None;
            egui::Window::new("Author technology-symbol variant")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "RSpice will copy the signed pin, netlist, and typed form contract into one writable project cell. The signed implementation binding remains exact and read-only.",
                    );
                    property(ui, "Source", &format!("{package_id}/{source_cell}"), "signed PDK");
                    ui.horizontal(|ui| {
                        ui.label("Target library");
                        egui::ComboBox::from_id_salt("technology-symbol-variant-library")
                            .selected_text(&target_library)
                            .show_ui(ui, |ui| {
                                for library in app
                                    .state
                                    .library_manager
                                    .libraries_sorted()
                                    .into_iter()
                                    .filter(|library| !library.read_only)
                                {
                                    ui.selectable_value(
                                        &mut target_library,
                                        library.name.clone(),
                                        &library.name,
                                    );
                                }
                            });
                    });
                    ui.horizontal(|ui| {
                        ui.label("Target cell");
                        ui.text_edit_singleline(&mut target_cell);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            decision = Some(false);
                        }
                        if ui.button("Author project variant").clicked() {
                            decision = Some(true);
                        }
                    });
                });
            if decision == Some(true) {
                let result = author_technology_symbol_variant(
                    app,
                    &package_id,
                    &source_cell,
                    &target_library,
                    &target_cell,
                );
                receipt(app, result);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            } else {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::AuthorTechnologySymbolVariant {
                        package_id,
                        source_cell,
                        target_library,
                        target_cell,
                    });
            }
        }
        ModelsWorkbenchDialog::DefinitionConflict {
            definition,
            scope,
            providers,
            mut selected_provider,
            mut reason,
        } => {
            let mut open = true;
            let mut decision = None;
            egui::Window::new("Contested model definition")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label(format!(
                        "{} '{definition}' is provided by {} loaded libraries. Execution remains blocked until an exact authenticated provider is published.",
                        scope.label(),
                        providers.len()
                    ));
                    for provider in &providers {
                        ui.radio_value(
                            &mut selected_provider,
                            provider.clone(),
                            RichText::new(provider).monospace(),
                        );
                    }
                    ui.label("Engineering audit reason");
                    ui.text_edit_multiline(&mut reason);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            decision = Some("cancel");
                        }
                        if ui.button("Open Model Editor").clicked() {
                            decision = Some("editor");
                        }
                        if app
                            .state
                            .model_library_manager
                            .model_resolution_record(scope, &definition)
                            .is_some()
                            && ui.button("Clear provider decision").clicked()
                        {
                            decision = Some("clear");
                        }
                        if ui
                            .add_enabled(
                                !selected_provider.is_empty() && !reason.trim().is_empty(),
                                egui::Button::new("Publish provider decision"),
                            )
                            .clicked()
                        {
                            decision = Some("publish");
                        }
                    });
                    if reason.trim().is_empty() {
                        ui.label(
                            RichText::new("A nonempty audit reason is required.")
                                .small()
                                .color(Tokens::get(ui.ctx()).color.warn),
                        );
                    }
                });
            if decision == Some("publish") {
                publish_definition_provider(app, scope, &definition, &selected_provider, &reason);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some("clear") {
                clear_definition_provider(app, scope, &definition);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some("editor") {
                app.queue_command(Command::ModelEditor);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some("cancel") || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            } else {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::DefinitionConflict {
                        definition,
                        scope,
                        providers,
                        selected_provider,
                        reason,
                    });
            }
        }
        ModelsWorkbenchDialog::BindingTrace { model, consumers } => {
            let mut open = true;
            egui::Window::new("Model binding trace")
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .show(ui.ctx(), |ui| {
                    ui.label(RichText::new(model).monospace().strong());
                    if consumers.is_empty() {
                        empty_state(
                            ui,
                            "No consumer resolves to this model.",
                            "The trace was derived from the active schematic.",
                        );
                    } else {
                        for consumer in consumers {
                            ui.label(consumer);
                        }
                    }
                });
            if !open {
                app.state.workbench.models_view.dialog = None;
            }
        }
        ModelsWorkbenchDialog::AddCorner {
            library,
            mut name,
            mut temperature_c,
            mut supply_factor,
        } => {
            let mut open = true;
            let mut decision = None;
            egui::Window::new("Add process corner")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "The corner is retained as an unbound authoring draft in one guarded project revision. Bind an explicit authenticated section before execution.",
                    );
                    ui.label(RichText::new(&library).monospace().strong());
                    ui.horizontal(|ui| {
                        ui.label("Corner name");
                        ui.text_edit_singleline(&mut name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Temperature °C");
                        ui.text_edit_singleline(&mut temperature_c);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Supply factor");
                        ui.text_edit_singleline(&mut supply_factor);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            decision = Some(false);
                        }
                        if ui.button("Add corner").clicked() {
                            decision = Some(true);
                        }
                    });
                });
            if decision == Some(true) {
                add_corner(app, &library, &name, &temperature_c, &supply_factor);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            } else {
                app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::AddCorner {
                    library,
                    name,
                    temperature_c,
                    supply_factor,
                });
            }
        }
        ModelsWorkbenchDialog::EditCorner {
            library,
            original_name,
            duplicate,
            mut name,
            mut description,
            mut nmos_corner,
            mut pmos_corner,
            mut temperature_c,
            mut supply_factor,
            mut minimum_temperature_c,
            mut maximum_temperature_c,
            mut required_domains,
            mut make_default,
        } => {
            let mut open = true;
            let mut decision = None;
            egui::Window::new(if duplicate {
                "Duplicate process corner"
            } else {
                "Edit process corner"
            })
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_width(520.0)
            .show(ui.ctx(), |ui| {
                ui.label(
                    "Corner metadata and required domains are saved as one guarded revision. Incomplete bindings remain recoverable drafts and fail closed at execution.",
                );
                ui.label(
                    RichText::new(format!("{library} / {original_name}"))
                        .monospace()
                        .strong(),
                );
                egui::Grid::new("models-corner-editor-fields")
                    .num_columns(2)
                    .spacing(egui::vec2(12.0, 8.0))
                    .show(ui, |ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut name);
                        ui.end_row();
                        ui.label("Description");
                        ui.text_edit_singleline(&mut description);
                        ui.end_row();
                        ui.label("NMOS axis");
                        ui.text_edit_singleline(&mut nmos_corner);
                        ui.end_row();
                        ui.label("PMOS axis");
                        ui.text_edit_singleline(&mut pmos_corner);
                        ui.end_row();
                        ui.label("Nominal temperature °C");
                        ui.text_edit_singleline(&mut temperature_c);
                        ui.end_row();
                        ui.label("Supply factor");
                        ui.text_edit_singleline(&mut supply_factor);
                        ui.end_row();
                        ui.label("Minimum qualified °C");
                        ui.text_edit_singleline(&mut minimum_temperature_c);
                        ui.end_row();
                        ui.label("Maximum qualified °C");
                        ui.text_edit_singleline(&mut maximum_temperature_c);
                        ui.end_row();
                    });
                ui.separator();
                ui.label(RichText::new("REQUIRED FUNCTIONAL DOMAINS").small().strong());
                ui.horizontal_wrapped(|ui| {
                    for domain in CornerSectionDomain::ALL {
                        let mut required = required_domains.contains(&domain);
                        if ui.checkbox(&mut required, domain.label()).changed() {
                            if required {
                                required_domains.push(domain);
                                required_domains.sort();
                                required_domains.dedup();
                            } else {
                                required_domains.retain(|candidate| *candidate != domain);
                            }
                        }
                    }
                });
                ui.checkbox(&mut make_default, "Use as the library default corner");
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        decision = Some(false);
                    }
                    if ui
                        .button(if duplicate {
                            "Duplicate corner"
                        } else {
                            "Save corner"
                        })
                        .clicked()
                    {
                        decision = Some(true);
                    }
                });
            });
            if decision == Some(true) {
                edit_corner(
                    app,
                    &library,
                    &original_name,
                    duplicate,
                    &name,
                    &description,
                    &nmos_corner,
                    &pmos_corner,
                    &temperature_c,
                    &supply_factor,
                    &minimum_temperature_c,
                    &maximum_temperature_c,
                    &required_domains,
                    make_default,
                );
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            } else {
                app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::EditCorner {
                    library,
                    original_name,
                    duplicate,
                    name,
                    description,
                    nmos_corner,
                    pmos_corner,
                    temperature_c,
                    supply_factor,
                    minimum_temperature_c,
                    maximum_temperature_c,
                    required_domains,
                    make_default,
                });
            }
        }
        ModelsWorkbenchDialog::ConfirmDeleteCorner { library, corner } => {
            let mut open = true;
            let mut decision = None;
            egui::Window::new("Delete process corner")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "Delete this corner as one undoable project revision? If it is the default, a deterministic replacement will be selected.",
                    );
                    ui.label(
                        RichText::new(format!("{library} / {corner}"))
                            .monospace()
                            .strong(),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            decision = Some(false);
                        }
                        if ui.button("Delete corner").clicked() {
                            decision = Some(true);
                        }
                    });
                });
            if decision == Some(true) {
                delete_corner(app, &library, &corner);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            }
        }
        ModelsWorkbenchDialog::BindCornerSection {
            library,
            corner,
            mut domain,
            mut section,
        } => {
            let sections = app
                .state
                .model_library_manager
                .get_library(&library)
                .map(ModelLibrary::section_index)
                .unwrap_or_default()
                .into_iter()
                .collect::<Vec<_>>();
            let mut open = true;
            let mut decision = None;
            egui::Window::new("Bind corner section")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "Choose an explicit functional domain and one section from the authenticated source catalog. No name inference is used.",
                    );
                    ui.label(
                        RichText::new(format!("{library} / {corner}"))
                            .monospace()
                            .strong(),
                    );
                    egui::ComboBox::from_label("Domain")
                        .selected_text(domain.label())
                        .show_ui(ui, |ui| {
                            for candidate in CornerSectionDomain::ALL {
                                ui.selectable_value(&mut domain, candidate, candidate.label());
                            }
                        });
                    egui::ComboBox::from_label("Authenticated section")
                        .selected_text(if section.is_empty() {
                            "Select section"
                        } else {
                            section.as_str()
                        })
                        .show_ui(ui, |ui| {
                            for candidate in &sections {
                                ui.selectable_value(
                                    &mut section,
                                    candidate.clone(),
                                    candidate.to_uppercase(),
                                );
                            }
                        });
                    if sections.is_empty() {
                        ui.colored_label(
                            Tokens::get(ui.ctx()).color.err,
                            "The authenticated source catalog defines no non-empty sections.",
                        );
                    }
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            decision = Some(false);
                        }
                        if ui
                            .add_enabled(
                                !section.is_empty() && !sections.is_empty(),
                                egui::Button::new("Bind section"),
                            )
                            .clicked()
                        {
                            decision = Some(true);
                        }
                    });
                });
            if decision == Some(true) {
                bind_corner_section(app, &library, &corner, domain, &section);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            } else {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::BindCornerSection {
                        library,
                        corner,
                        domain,
                        section,
                    });
            }
        }
        ModelsWorkbenchDialog::ResolveDrift { library } => {
            let mut open = true;
            let mut decision = None;
            egui::Window::new(format!("Resolve source drift · {library}"))
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .default_size(egui::vec2(640.0, 460.0))
                .show(ui.ctx(), |ui| {
                    decision = drift::resolve_dialog(ui, app, &library);
                });
            if decision == Some(true) {
                app.state.workbench.models_view.dialog = None;
                refresh_library(app, &library);
            } else if decision == Some(false) || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            }
        }
    }
}
