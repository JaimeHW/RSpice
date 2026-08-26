//! Modal dialogs the Models & PDKs workspace opens.
//!
//! Each arm renders one `ModelsWorkbenchDialog` variant and then does one of
//! three things: clears the field, writes the edited variant back so the next
//! frame keeps the typed text, or hands the decision to the transaction that
//! owns it. The transactions live beside this module; what is here is the
//! asking.

use super::corner_ops::{add_corner, bind_corner_section, delete_corner, edit_corner};
use super::*;

pub(super) fn render_dialog(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    hub: &hub::HubCatalog,
) {
    let Some(dialog) = app.state.workbench.models_view.dialog.clone() else {
        return;
    };
    match dialog {
        ModelsWorkbenchDialog::HeldCatalog => super::held_catalog::dialog(ui, app, hub),
        #[cfg(target_arch = "wasm32")]
        ModelsWorkbenchDialog::SelectBrowserImportRoot {
            candidates,
            selected,
        } => {
            let mut selection = selected.min(candidates.len().saturating_sub(1));
            const NOTICE: &str = "Choose the one SPICE or Spectre source that owns this import. \
                                  Only its reachable include closure will be authenticated and \
                                  retained.";
            let choice = Dialog::new("Model sources", "Choose model-library entry", "Import")
                .description(NOTICE)
                .size(DialogSize::Transaction)
                .primary_enabled(!candidates.is_empty())
                .ghost("Cancel")
                .show(ui.ctx(), |ui| {
                    ui.label(NOTICE);
                    ui.add_space(8.0);
                    let picked = candidates
                        .get(selection)
                        .map(String::as_str)
                        .unwrap_or("No supported entry");
                    if let Some(index) = widgets::select(
                        ui,
                        "browser-model-import-root",
                        "Model-library entry",
                        picked,
                        &candidates,
                        ui.available_width().max(1.0),
                    ) {
                        selection = index;
                    }
                });
            match choice {
                DialogChoice::Primary => {
                    if let Some(root) = candidates.get(selection).cloned() {
                        app.queue_browser_import_root(root);
                    }
                }
                DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
                    app.queue_cancel_browser_import_root();
                }
                DialogChoice::None => {
                    if selection != selected {
                        app.state.workbench.models_view.dialog =
                            Some(ModelsWorkbenchDialog::SelectBrowserImportRoot {
                                candidates,
                                selected: selection,
                            });
                    }
                }
            }
        }
        ModelsWorkbenchDialog::SourcePreview {
            title,
            subtitle,
            source,
            editable,
        } => {
            // A read-only source has no transaction to commit, so its footer
            // is the note strip rather than a primary that would only close
            // what Escape already closes.
            let mut dialog = Dialog::new("Model sources", title.clone(), "Edit in Model Editor…")
                .description(format!("The retained source behind {title}."))
                .size(DialogSize::WideWorkflow)
                .hint(subtitle.clone())
                .ghost("Close")
                // Enter belongs to nothing here: the body is a scrollable
                // read-only buffer and the primary opens another surface.
                .primary_on_enter(false);
            if !editable {
                dialog = dialog.note_only_footer();
            }
            let choice = dialog.show(ui.ctx(), |ui| {
                ui.label(
                    RichText::new(if editable {
                        "Editing publishes one validated project revision."
                    } else {
                        "Read-only: this source is not owned by the project."
                    })
                    .small()
                    .color(Tokens::get(ui.ctx()).color.text_faint),
                );
                ui.add_space(6.0);
                // A preview, always. The text stays selectable and copyable,
                // but it is never a writable buffer: this dialog holds a
                // per-frame clone of the retained bytes and has nowhere to
                // write an edit back to, so an interactive field here accepted
                // keystrokes and dropped every one of them. Authoring goes
                // through Model Editor, which owns validation and revision
                // history.
                let mut body = source.as_str();
                ScrollArea::both().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut body)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .desired_rows(26),
                    );
                });
            });
            match choice {
                DialogChoice::Primary => {
                    app.queue_command(Command::ModelEditor);
                    app.state.workbench.models_view.dialog = None;
                }
                DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
                    app.state.workbench.models_view.dialog = None;
                }
                DialogChoice::None => {}
            }
        }
        ModelsWorkbenchDialog::CompareModels {
            left_library,
            left_model,
            right,
        } => {
            let mut chosen = right.clone();
            // A comparison commits nothing: it is a reading surface, so its
            // footer states what it read rather than offering an action.
            let choice = Dialog::new("Model sources", "Compare model definitions", "Close")
                .description(format!(
                    "Parameter-by-parameter comparison of {left_library}/{left_model} against \
                     another loaded definition."
                ))
                .size(DialogSize::WideWorkflow)
                .primary_on_enter(false)
                .note_only_footer()
                .hint(format!("{left_library} / {left_model}"))
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
            if choice == DialogChoice::Cancelled {
                app.state.workbench.models_view.dialog = None;
            } else if chosen != right {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::CompareModels {
                        left_library,
                        left_model,
                        right: chosen,
                    });
            }
        }
        ModelsWorkbenchDialog::ConfirmPack {
            pack_id,
            attach,
            release,
        } => {
            let (title, primary) = match (release.as_deref(), attach) {
                (Some(_), _) => ("Install model pack", "Install pack"),
                (None, true) => ("Attach model pack", "Attach pack"),
                (None, false) => ("Detach model pack", "Detach pack"),
            };
            let blocked = match release.as_deref() {
                Some(release) => hub::release_install_block_reason(app, release),
                None => app
                    .state
                    .workbench
                    .models_view
                    .model_import_in_progress
                    .then(|| "Another model-source operation is still running.".to_owned()),
            };
            let mut dialog = Dialog::new("Model packs", title, primary)
                .description(format!("{title}: {pack_id}"))
                .size(DialogSize::Transaction)
                .primary_enabled(blocked.is_none())
                .ghost("Cancel");
            if let Some(reason) = blocked.as_deref() {
                dialog = dialog.hint(reason);
            }
            let choice = dialog.show(ui.ctx(), |ui| {
                if let Some(release) = release.as_deref() {
                    hub::release_confirmation(ui, &pack_id, release);
                    return;
                }
                ui.label(if attach {
                    "RSpice will authenticate the pack entry and publish its retained include closure as one undoable project revision."
                } else {
                    "RSpice will remove the attached source as one undoable project revision. Existing instance references may become unresolved."
                });
                ui.label(RichText::new(&pack_id).monospace().strong());
            });
            match choice {
                DialogChoice::Primary => {
                    match release.as_deref() {
                        Some(release) => {
                            let request = hub::release_request(&pack_id, release);
                            app.queue_model_hub(request);
                        }
                        None if attach => attach_pack(app, &pack_id),
                        None => detach_pack(app, &pack_id),
                    }
                    app.state.workbench.models_view.dialog = None;
                }
                DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
                    app.state.workbench.models_view.dialog = None;
                    app.state.workbench.models_view.operational_state =
                        ModelsOperationalState::Cancelled;
                }
                DialogChoice::None => {}
            }
        }
        ModelsWorkbenchDialog::ConfirmPart { pack_id, part_name } => {
            let busy = app.state.workbench.models_view.model_import_in_progress;
            let mut dialog = Dialog::new(
                "Model packs",
                "Add shipped part to project",
                "Add to project",
            )
            .description(format!(
                "Retain {part_name} from {pack_id} into this project as one revision."
            ))
            .size(DialogSize::Transaction)
            .primary_enabled(!busy)
            .ghost("Cancel");
            if busy {
                dialog = dialog.hint("Another model-source operation is still running.");
            }
            let choice = dialog.show(ui.ctx(), |ui| {
                ui.label(
                    "The exact installed source file will be parsed, license-checked, pinned by digest, and published as one undoable project revision.",
                );
                ui.label(
                    RichText::new(format!("{part_name} · {pack_id}"))
                        .monospace()
                        .strong(),
                );
            });
            match choice {
                DialogChoice::Primary => {
                    add_part(app, &pack_id, &part_name);
                    app.state.workbench.models_view.dialog = None;
                }
                DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
                    app.state.workbench.models_view.dialog = None;
                    app.state.workbench.models_view.operational_state =
                        ModelsOperationalState::Cancelled;
                }
                DialogChoice::None => {}
            }
        }
        ModelsWorkbenchDialog::AuthorTechnologySymbolVariant {
            package_id,
            source_cell,
            mut target_library,
            mut target_cell,
        } => {
            let writable = app
                .state
                .library_manager
                .libraries_sorted()
                .into_iter()
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
                .collect::<Vec<_>>();
            let choice = Dialog::new(
                "Symbols",
                "Author technology-symbol variant",
                "Author project variant",
            )
            .description(format!(
                "Copy the signed contract of {package_id}/{source_cell} into a writable project \
                 cell."
            ))
            .size(DialogSize::Transaction)
            .initial_focus(DialogInitialFocus::BodyControl)
            .primary_enabled(!target_cell.trim().is_empty() && !target_library.is_empty())
            .ghost("Cancel")
            .show_with_initial_body_focus(ui.ctx(), |ui| {
                ui.label(
                    "RSpice will copy the signed pin, netlist, and typed form contract into one writable project cell. The signed implementation binding remains exact and read-only.",
                );
                property(ui, "Source", &format!("{package_id}/{source_cell}"), "signed PDK");
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label("Target library");
                    if let Some(index) = widgets::select(
                        ui,
                        "technology-symbol-variant-library",
                        "Target library",
                        &target_library,
                        &writable,
                        (ui.available_width() - 4.0).max(1.0),
                    ) && let Some(name) = writable.get(index)
                    {
                        target_library = name.clone();
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Target cell");
                    let cell = ui.add(
                        egui::TextEdit::singleline(&mut target_cell)
                            .desired_width(f32::INFINITY),
                    );
                    Some(cell.id)
                })
                .inner
            });
            match choice {
                DialogChoice::Primary => {
                    let result = author_technology_symbol_variant(
                        app,
                        &package_id,
                        &source_cell,
                        &target_library,
                        &target_cell,
                    );
                    receipt(app, result);
                    app.state.workbench.models_view.dialog = None;
                }
                DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
                    app.state.workbench.models_view.dialog = None;
                    app.state.workbench.models_view.operational_state =
                        ModelsOperationalState::Cancelled;
                }
                DialogChoice::None => {
                    app.state.workbench.models_view.dialog =
                        Some(ModelsWorkbenchDialog::AuthorTechnologySymbolVariant {
                            package_id,
                            source_cell,
                            target_library,
                            target_cell,
                        });
                }
            }
        }
        ModelsWorkbenchDialog::DefinitionConflict {
            definition,
            scope,
            providers,
            mut selected_provider,
            mut reason,
        } => {
            let has_record = app
                .state
                .model_library_manager
                .model_resolution_record(scope, &definition)
                .is_some();
            let mut clear = false;
            let mut dialog = Dialog::new(
                "Model sources",
                "Contested model definition",
                "Publish provider decision",
            )
            .description(format!(
                "{} '{definition}' is provided by {} loaded libraries and cannot execute until \
                 one is published as the authenticated provider.",
                scope.label(),
                providers.len()
            ))
            .size(DialogSize::Transaction)
            // The audit reason is a multiline field, so Enter belongs to it.
            .primary_on_enter(false)
            .primary_enabled(!selected_provider.is_empty() && !reason.trim().is_empty())
            .secondary("Open Model Editor")
            .ghost("Cancel");
            if reason.trim().is_empty() {
                dialog = dialog.hint("A nonempty audit reason is required.");
            }
            let choice = dialog.show(ui.ctx(), |ui| {
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
                ui.add_space(6.0);
                ui.label("Engineering audit reason");
                ui.text_edit_multiline(&mut reason);
                // Withdrawing a published decision is not the transaction this
                // dialog commits, so it sits beside the record it withdraws
                // rather than in the footer where the commit lives.
                if has_record {
                    ui.add_space(6.0);
                    clear = Button::new("Clear provider decision")
                        .destructive(true)
                        .show(ui)
                        .clicked();
                }
            });
            if clear {
                clear_definition_provider(app, scope, &definition);
                app.state.workbench.models_view.dialog = None;
                return;
            }
            match choice {
                DialogChoice::Primary => {
                    publish_definition_provider(
                        app,
                        scope,
                        &definition,
                        &selected_provider,
                        &reason,
                    );
                    app.state.workbench.models_view.dialog = None;
                }
                DialogChoice::Secondary => {
                    app.queue_command(Command::ModelEditor);
                    app.state.workbench.models_view.dialog = None;
                }
                DialogChoice::Ghost | DialogChoice::Cancelled => {
                    app.state.workbench.models_view.dialog = None;
                    app.state.workbench.models_view.operational_state =
                        ModelsOperationalState::Cancelled;
                }
                DialogChoice::None => {
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
        }
        ModelsWorkbenchDialog::BindingTrace { model, consumers } => {
            // A trace commits nothing; its footer states what it traced.
            let choice = Dialog::new("Model sources", "Model binding trace", "Close")
                .description(format!(
                    "Every consumer the active schematic resolves to {model}."
                ))
                .size(DialogSize::Transaction)
                .primary_on_enter(false)
                .note_only_footer()
                .hint(format!(
                    "{model} · {} consumer{}",
                    consumers.len(),
                    if consumers.len() == 1 { "" } else { "s" }
                ))
                .show(ui.ctx(), |ui| {
                    ui.label(RichText::new(&model).monospace().strong());
                    if consumers.is_empty() {
                        empty_state(
                            ui,
                            "No consumer resolves to this model.",
                            "The trace was derived from the active schematic.",
                        );
                    } else {
                        for consumer in &consumers {
                            ui.label(consumer);
                        }
                    }
                });
            if choice == DialogChoice::Cancelled {
                app.state.workbench.models_view.dialog = None;
            }
        }
        ModelsWorkbenchDialog::AddCorner {
            library,
            mut name,
            mut temperature_c,
            mut supply_factor,
        } => {
            let choice = Dialog::new("Corners", "Add process corner", "Add corner")
                .description(format!("Retain a new authoring draft corner in {library}."))
                .size(DialogSize::Transaction)
                .initial_focus(DialogInitialFocus::BodyControl)
                .primary_enabled(!name.trim().is_empty())
                .ghost("Cancel")
                .show_with_initial_body_focus(ui.ctx(), |ui| {
                    ui.label(
                        "The corner is retained as an unbound authoring draft in one guarded project revision. Bind an explicit authenticated section before execution.",
                    );
                    ui.label(RichText::new(&library).monospace().strong());
                    ui.add_space(6.0);
                    let named = ui.horizontal(|ui| {
                        ui.label("Corner name");
                        ui.text_edit_singleline(&mut name).id
                    });
                    ui.horizontal(|ui| {
                        ui.label("Temperature °C");
                        ui.text_edit_singleline(&mut temperature_c);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Supply factor");
                        ui.text_edit_singleline(&mut supply_factor);
                    });
                    Some(named.inner)
                });
            let decision = match choice {
                DialogChoice::Primary => Some(true),
                DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
                    Some(false)
                }
                DialogChoice::None => None,
            };
            if decision == Some(true) {
                add_corner(app, &library, &name, &temperature_c, &supply_factor);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) {
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
            let mut name_field = None;
            let choice = Dialog::new(
                "Corners",
                if duplicate {
                    "Duplicate process corner"
                } else {
                    "Edit process corner"
                },
                if duplicate {
                    "Duplicate corner"
                } else {
                    "Save corner"
                },
            )
            .description(format!(
                "Corner metadata and required domains for {library} / {original_name}."
            ))
            .size(DialogSize::Transaction)
            .initial_focus(DialogInitialFocus::BodyControl)
            .primary_enabled(!name.trim().is_empty())
            .ghost("Cancel")
            .show_with_initial_body_focus(ui.ctx(), |ui| {
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
                        name_field = Some(ui.text_edit_singleline(&mut name).id);
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
                name_field
            });
            let decision = match choice {
                DialogChoice::Primary => Some(true),
                DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
                    Some(false)
                }
                DialogChoice::None => None,
            };
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
            } else if decision == Some(false) {
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
            // The one destructive confirm in this workspace, and the one
            // primary that carries the error fill.
            let choice = Dialog::new("Corners", "Delete process corner", "Delete corner")
                .description(format!("Delete the corner {library} / {corner}."))
                .size(DialogSize::Transaction)
                .destructive()
                .ghost("Cancel")
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "Delete this corner as one undoable project revision? If it is the default, a deterministic replacement will be selected.",
                    );
                    ui.label(
                        RichText::new(format!("{library} / {corner}"))
                            .monospace()
                            .strong(),
                    );
                });
            match choice {
                DialogChoice::Primary => {
                    delete_corner(app, &library, &corner);
                    app.state.workbench.models_view.dialog = None;
                }
                DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
                    app.state.workbench.models_view.dialog = None;
                    app.state.workbench.models_view.operational_state =
                        ModelsOperationalState::Cancelled;
                }
                DialogChoice::None => {}
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
            let domain_labels = CornerSectionDomain::ALL
                .iter()
                .map(|candidate| candidate.label().to_owned())
                .collect::<Vec<_>>();
            let section_labels = sections
                .iter()
                .map(|candidate| candidate.to_uppercase())
                .collect::<Vec<_>>();
            let mut dialog = Dialog::new("Corners", "Bind corner section", "Bind section")
                .description(format!(
                    "Bind one authenticated source section to {library} / {corner}."
                ))
                .size(DialogSize::Transaction)
                .primary_enabled(!section.is_empty() && !sections.is_empty())
                .ghost("Cancel");
            if sections.is_empty() {
                dialog =
                    dialog.hint("The authenticated source catalog defines no non-empty sections.");
            }
            let choice = dialog.show(ui.ctx(), |ui| {
                ui.label(
                    "Choose an explicit functional domain and one section from the authenticated source catalog. No name inference is used.",
                );
                ui.label(
                    RichText::new(format!("{library} / {corner}"))
                        .monospace()
                        .strong(),
                );
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label("Domain");
                    if let Some(index) = widgets::select(
                        ui,
                        "models-corner-section-domain",
                        "Domain",
                        domain.label(),
                        &domain_labels,
                        220.0,
                    ) && let Some(candidate) = CornerSectionDomain::ALL.get(index)
                    {
                        domain = *candidate;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Authenticated section");
                    let selected = if section.is_empty() {
                        "Select section".to_owned()
                    } else {
                        section.to_uppercase()
                    };
                    if let Some(index) = widgets::select(
                        ui,
                        "models-corner-section-name",
                        "Authenticated section",
                        &selected,
                        &section_labels,
                        260.0,
                    ) && let Some(candidate) = sections.get(index)
                    {
                        section = candidate.clone();
                    }
                });
                if sections.is_empty() {
                    ui.colored_label(
                        Tokens::get(ui.ctx()).color.err,
                        "The authenticated source catalog defines no non-empty sections.",
                    );
                }
            });
            let decision = match choice {
                DialogChoice::Primary => Some(true),
                DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
                    Some(false)
                }
                DialogChoice::None => None,
            };
            if decision == Some(true) {
                bind_corner_section(app, &library, &corner, domain, &section);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) {
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
            let blocked = drift::repin_block_reason(app, &library);
            let mut dialog = Dialog::new(
                "Model sources",
                format!("Resolve source drift · {library}"),
                "Re-pin this library",
            )
            .description(format!(
                "Sources of {library} whose bytes no longer hash to the digests this project \
                 accepted."
            ))
            .size(DialogSize::Transaction)
            .primary_enabled(blocked.is_none())
            .ghost("Cancel");
            if let Some(reason) = blocked {
                dialog = dialog.hint(reason);
            }
            let choice = dialog.show(ui.ctx(), |ui| {
                drift::resolve_dialog(ui, app, &library);
            });
            match choice {
                DialogChoice::Primary => {
                    app.state.workbench.models_view.dialog = None;
                    refresh_library(app, &library);
                }
                DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
                    app.state.workbench.models_view.dialog = None;
                    app.state.workbench.models_view.operational_state =
                        ModelsOperationalState::Cancelled;
                }
                DialogChoice::None => {}
            }
        }
    }
}
