//! The catalog's selected-model detail pane.
//!
//! Everything this pane paints about one model is a handful of scalars and at
//! most a screenful of parameter rows. It used to obtain them by cloning the
//! selected model *and its whole library* out of the catalog, to get out from
//! under the borrow the surrounding page holds — and a model library carries
//! the retained bytes of its entire include closure, both namespace maps, every
//! section's cards and the qualification state, so a megabyte-scale foundry
//! import was copied on every frame. Nothing made that conditional: the catalog
//! force-selects a row whenever it is not empty.
//!
//! [`SelectedModelDetail`] crosses that line instead — the projection this file
//! paints, built under an immutable borrow and holding nothing the catalog also
//! holds. The include page took the same shape for the same reason.

use super::*;

pub(super) fn selected_model_detail(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let Some(library_name) = app.state.model_library_manager.selected_library.clone() else {
        empty_state(
            ui,
            "Select a model to inspect its exact source and resolved contract.",
            "The detail area never invents a model when the catalog selection is empty.",
        );
        return;
    };
    let Some(model_name) = app.state.workbench.selected_model.clone() else {
        empty_state(
            ui,
            "Select a model to inspect its exact source and resolved contract.",
            "Use the table above or choose a model source in the Navigator.",
        );
        return;
    };
    let Some((library, model)) = app
        .state
        .model_library_manager
        .get_library(&library_name)
        .and_then(|library| library.get_model(&model_name).map(|model| (library, model)))
        .map(|(library, model)| (library.clone(), model.clone()))
    else {
        empty_state(
            ui,
            "The selected model no longer resolves.",
            "Rescan or clear the selection; stale identities are never retargeted automatically.",
        );
        return;
    };
    let usages = model_consumers_for_provider(app, &library, &model.name);
    let selected_component = exactly_one_selected_component(app);
    let binding_block_reason = selected_component.map_or_else(
        || Some("Select exactly one compatible schematic instance first.".to_owned()),
        |component_id| {
            crate::workbench::docks::validate_component_model_catalog_binding(
                app.state,
                component_id,
                &library.name,
                &model.name,
            )
            .err()
        },
    );
    let source_available = !library.source_contents.is_empty() || model.file_path.is_some();

    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(12, 7))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.label(
                    RichText::new(&model.name)
                        .monospace()
                        .font(theme::mono(tokens::FS_2, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.label(
                    RichText::new(format!(
                        "{} · {} · {}",
                        model.model_type.display_name(),
                        model.level.display_name(),
                        library.name
                    ))
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let blocked = pin_source_block_reason(
                    library.root_path.is_some(),
                    app.state.workbench.models_view.model_import_in_progress,
                );
                if ui
                    .add_enabled(
                        blocked.is_none(),
                        compact_button(if library.source_closure.is_empty() {
                            "Pin source"
                        } else {
                            "Refresh pin"
                        }),
                    )
                    .on_disabled_hover_text(blocked.unwrap_or_default())
                    .clicked()
                {
                    refresh_library(app, &library);
                }
                if ui
                    .add_enabled(source_available, compact_button("Open source"))
                    .on_disabled_hover_text("This built-in definition has no source document.")
                    .clicked()
                {
                    open_model_source(app, &library, &model);
                }
                if ui.add(compact_button("Compare…")).clicked() {
                    open_model_compare(app, &library.name, &model.name);
                }
                let project_owned = library.source_authority.is_project_owned();
                if ui
                    .add(compact_button(if project_owned {
                        "Model editor…"
                    } else {
                        "Author project copy…"
                    }))
                    .clicked()
                {
                    if project_owned {
                        app.queue_command(Command::ModelEditor);
                    } else {
                        app.queue_command(Command::ModelCreateProjectCopy);
                    }
                }
                if ui.add(compact_button("Qualification")).clicked() {
                    app.state.workbench.models_page = ModelsPage::Qualification;
                }
                let bind = ui.add_enabled(
                    selected_component.is_some() && binding_block_reason.is_none(),
                    compact_button("Bind to selection…"),
                );
                let bind = if let Some(reason) = binding_block_reason.as_deref() {
                    bind.on_disabled_hover_text(reason)
                } else {
                    bind
                };
                if bind.clicked()
                    && let Some(component_id) = selected_component
                {
                    app.queue_model_binding(component_id, &library.name, &model.name);
                }
            });
        });
    ui.painter().hline(
        ui.min_rect().x_range(),
        ui.min_rect().bottom(),
        Stroke::new(1.0, t.color.border),
    );

    ScrollArea::vertical()
        .id_salt("models-selected-detail")
        .show(ui, |ui| {
            let detail_width = ui.available_width();
            if detail_width > 1100.0 {
                ui.columns(4, |columns| {
                    parameter_card(&mut columns[0], &library, &model);
                    characteristic_card(&mut columns[1], &model);
                    qualification_card(&mut columns[2], &library, &model);
                    usage_card(&mut columns[3], &model, &usages, app);
                });
            } else if detail_width > 650.0 {
                ui.columns(2, |columns| {
                    parameter_card(&mut columns[0], &library, &model);
                    characteristic_card(&mut columns[1], &model);
                });
                ui.columns(2, |columns| {
                    qualification_card(&mut columns[0], &library, &model);
                    usage_card(&mut columns[1], &model, &usages, app);
                });
            } else {
                parameter_card(ui, &library, &model);
                characteristic_card(ui, &model);
                qualification_card(ui, &library, &model);
                usage_card(ui, &model, &usages, app);
            }
        });
}

fn parameter_card(ui: &mut Ui, library: &ModelLibrary, model: &DeviceModel) {
    detail_pane(
        ui,
        "RESOLVED PARAMETERS",
        Some(&format!(
            "{} values",
            model.parameters.len() + model.string_parameters.len()
        )),
        |ui| {
            let mut values = model
                .parameters
                .iter()
                .map(|(name, value)| (name.clone(), value.to_string(), "source card"))
                .collect::<Vec<_>>();
            values.extend(
                model
                    .string_parameters
                    .iter()
                    .map(|(name, value)| (name.clone(), value.clone(), "source string")),
            );
            values.sort_by(|left, right| left.0.cmp(&right.0));
            if values.is_empty() {
                empty_state(
                    ui,
                    "No parameter card is attached to this model.",
                    "Built-in equation defaults remain owned by the engine.",
                );
            } else {
                // A BSIM4 card carries several hundred parameters and this is
                // a column in a four-pane row. What is not listed is counted;
                // a list that stops silently reads as the whole card.
                let hidden = values.len().saturating_sub(PARAMETER_ROWS);
                for (name, value, origin) in values.into_iter().take(PARAMETER_ROWS) {
                    property(ui, &name, &value, origin);
                }
                if hidden > 0 {
                    property(
                        ui,
                        "…",
                        &format!("{hidden} more"),
                        "open source for the full card",
                    );
                }
            }
            if let Some(metadata) = library.model_definition_metadata.get(&model.name) {
                ui.separator();
                property(
                    ui,
                    "Schema",
                    &format!("{} typed fields", metadata.parameters.len()),
                    "project definition",
                );
            }
        },
    );
}

/// What the card declares about the device's operating envelope.
///
/// This pane used to draw a curve: a square-law `(V − VTH0)²` sketch, plotted
/// for any card carrying a `vth0` — which every BSIM4, BSIM-CMG and PSP card
/// does, and for none of which is the square law the model. Normalised, with no
/// axis units and a hard-coded 1.8 V supply, it looked like an I-V
/// characteristic and was an unrelated equation. A plot that a reader can
/// mistake for the device's behaviour has to come from the engine evaluating
/// the actual model; until it does, this states only what the card itself
/// declares.
fn characteristic_card(ui: &mut Ui, model: &DeviceModel) {
    detail_pane(ui, "DECLARED ENVELOPE", Some("from the card"), |ui| {
        let mut declared = false;
        if let Some(vth) = model.vth0.or_else(|| model.parameters.get("vth0").copied()) {
            property(ui, "VTH0", &format!("{vth:.6} V"), "source card");
            declared = true;
        }
        if let Some(vdd) = model.vdd {
            property(ui, "Supply", &format!("{vdd:.6} V"), "source card");
            declared = true;
        }
        for (label, low, high) in [
            ("Length", model.l_min, model.l_max),
            ("Width", model.w_min, model.w_max),
        ] {
            match (low, high) {
                (Some(low), Some(high)) => {
                    property(ui, label, &format!("{low:.4e} … {high:.4e} m"), "bin range");
                    declared = true;
                }
                (Some(low), None) => {
                    property(ui, label, &format!("≥ {low:.4e} m"), "bin range");
                    declared = true;
                }
                (None, Some(high)) => {
                    property(ui, label, &format!("≤ {high:.4e} m"), "bin range");
                    declared = true;
                }
                (None, None) => {}
            }
        }
        if let (Some(low), Some(high)) = (model.spice_level, model.model_version) {
            property(
                ui,
                "Level",
                &format!("{low} · version {high}"),
                "source card",
            );
            declared = true;
        }
        if !declared {
            empty_state(
                ui,
                "This card declares no operating envelope.",
                "Bin ranges, threshold and supply are read from the card; nothing is inferred.",
            );
        }
    });
}

fn qualification_card(ui: &mut Ui, library: &ModelLibrary, model: &DeviceModel) {
    detail_pane(ui, "QUALIFICATION", Some("source-owned evidence"), |ui| {
        if let Some(state) = library.model_qualification.get(&model.name) {
            property(ui, "Suites", &state.suites.len().to_string(), "retained");
            property(
                ui,
                "Vectors",
                &state
                    .suites
                    .iter()
                    .map(|suite| suite.vectors.len())
                    .sum::<usize>()
                    .to_string(),
                "declared",
            );
            property(
                ui,
                "Evidence",
                &state.evidence.len().to_string(),
                "immutable",
            );
            property(
                ui,
                "Releases",
                &state.releases.len().to_string(),
                "promoted",
            );
            let open = state
                .vector_dispositions
                .iter()
                .filter(|disposition| disposition.is_open())
                .count();
            property(
                ui,
                "Open dispositions",
                &open.to_string(),
                if open == 0 {
                    "clean"
                } else {
                    "review required"
                },
            );
        } else {
            empty_state(
                ui,
                "This model has no qualification suite.",
                "Qualification claims remain empty until a retained suite and exact-source evidence exist.",
            );
        }
    });
}

fn usage_card(
    ui: &mut Ui,
    model: &DeviceModel,
    usages: &[String],
    app: &mut ManagerRenderContext<'_>,
) {
    detail_pane(
        ui,
        "WHERE USED",
        Some(&format!("{} consumers", usages.len())),
        |ui| {
            if usages.is_empty() {
                empty_state(
                    ui,
                    "Not bound in the active project.",
                    "Place an instance or select one and use Bind to selection.",
                );
            } else {
                for usage in usages.iter().take(USAGE_ROWS) {
                    if ui.link(usage).clicked() {
                        app.state.workbench.models_view.dialog =
                            Some(ModelsWorkbenchDialog::BindingTrace {
                                model: model.name.clone(),
                                consumers: usages.to_vec(),
                            });
                    }
                }
                if usages.len() > USAGE_ROWS {
                    // The trace dialog lists all of them; only this column stops.
                    if ui
                        .link(format!("{} more…", usages.len() - USAGE_ROWS))
                        .clicked()
                    {
                        app.state.workbench.models_view.dialog =
                            Some(ModelsWorkbenchDialog::BindingTrace {
                                model: model.name.clone(),
                                consumers: usages.to_vec(),
                            });
                    }
                }
            }
        },
    );
}
