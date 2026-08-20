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
//!
//! The two actions that genuinely need the whole record — pinning a source and
//! opening one — take names and resolve it themselves, once per click, which is
//! not a cost a frame pays.

use super::*;

/// What the detail pane paints, and nothing else.
///
/// Every field is either a scalar the pane prints or a list it prints in full;
/// nothing here grows with the corpus, the library's retained bytes, or the
/// number of cards in the section the model came from.
struct SelectedModelDetail<'a> {
    library: String,
    model: String,
    family: &'static str,
    level: &'static str,
    /// Whether the library names an external root a pin can be taken from, and
    /// whether it already holds one.
    has_root: bool,
    pinned: bool,
    project_owned: bool,
    source_available: bool,
    /// The parameter rows the column paints, already ordered and truncated.
    parameters: Vec<(String, String, &'static str)>,
    parameter_total: usize,
    /// Typed fields the project definition declares, when the library keeps
    /// authoring metadata for this model.
    typed_schema_fields: Option<usize>,
    envelope: DeclaredEnvelope,
    qualification: Option<QualificationCounts>,
    /// The instances bound to this model, borrowed from the catalog page's one
    /// consumer index rather than copied out of it.
    usages: &'a [String],
    selected_component: Option<u64>,
    binding_block_reason: Option<String>,
}

/// What the card itself declares about the device's operating envelope.
struct DeclaredEnvelope {
    vth0: Option<f64>,
    vdd: Option<f64>,
    length: (Option<f64>, Option<f64>),
    width: (Option<f64>, Option<f64>),
    /// Level and version, which the card states together or not at all.
    level: Option<(u32, f64)>,
}

impl DeclaredEnvelope {
    /// Whether the card declared anything at all, which decides between the
    /// properties and the empty state.
    fn is_empty(&self) -> bool {
        self.vth0.is_none()
            && self.vdd.is_none()
            && self.length == (None, None)
            && self.width == (None, None)
            && self.level.is_none()
    }
}

/// Retained qualification evidence, counted.
struct QualificationCounts {
    suites: usize,
    vectors: usize,
    evidence: usize,
    releases: usize,
    open_dispositions: usize,
}

/// Project the selected model, or say why there is nothing to paint.
///
/// The borrow of the catalog ends with this function. Everything the pane needs
/// afterwards has been copied out of it by value, and the largest of those is a
/// screenful of parameter rows.
fn project_selection<'a>(
    app: &ManagerRenderContext<'_>,
    consumers: &'a ConsumerIndex,
) -> Result<SelectedModelDetail<'a>, (&'static str, &'static str)> {
    let Some(library_name) = app.state.model_library_manager.selected_library.as_deref() else {
        return Err((
            "Select a model to inspect its exact source and resolved contract.",
            "The detail area never invents a model when the catalog selection is empty.",
        ));
    };
    let Some(model_name) = app.state.workbench.selected_model.as_deref() else {
        return Err((
            "Select a model to inspect its exact source and resolved contract.",
            "Use the table above or choose a model source in the Navigator.",
        ));
    };
    let Some((library, model)) = app
        .state
        .model_library_manager
        .get_library(library_name)
        .and_then(|library| library.get_model(model_name).map(|model| (library, model)))
    else {
        return Err((
            "The selected model no longer resolves.",
            "Rescan or clear the selection; stale identities are never retargeted automatically.",
        ));
    };

    let mut parameters = model
        .parameters
        .iter()
        .map(|(name, value)| (name.clone(), value.to_string(), "source card"))
        .collect::<Vec<_>>();
    parameters.extend(
        model
            .string_parameters
            .iter()
            .map(|(name, value)| (name.clone(), value.clone(), "source string")),
    );
    parameters.sort_by(|left, right| left.0.cmp(&right.0));
    let parameter_total = parameters.len();
    // A BSIM4 card carries several hundred parameters and the column paints at
    // most `PARAMETER_ROWS` of them, so the rest never crosses the borrow.
    parameters.truncate(PARAMETER_ROWS);

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

    Ok(SelectedModelDetail {
        library: library.name.clone(),
        model: model.name.clone(),
        family: model.model_type.display_name(),
        level: model.level.display_name(),
        has_root: library.has_external_root(),
        pinned: library.has_retained_closure(),
        project_owned: library.source_authority.is_project_owned(),
        source_available: !library.source_contents.is_empty() || model.file_path.is_some(),
        parameters,
        parameter_total,
        typed_schema_fields: library
            .model_definition_metadata
            .get(&model.name)
            .map(|metadata| metadata.parameters.len()),
        envelope: DeclaredEnvelope {
            vth0: model.vth0.or_else(|| model.parameters.get("vth0").copied()),
            vdd: model.vdd,
            length: (model.l_min, model.l_max),
            width: (model.w_min, model.w_max),
            level: model.spice_level.zip(model.model_version),
        },
        qualification: library.model_qualification.get(&model.name).map(|state| {
            QualificationCounts {
                suites: state.suites.len(),
                vectors: state
                    .suites
                    .iter()
                    .map(|suite| suite.vectors.len())
                    .sum::<usize>(),
                evidence: state.evidence.len(),
                releases: state.releases.len(),
                open_dispositions: state
                    .vector_dispositions
                    .iter()
                    .filter(|disposition| disposition.is_open())
                    .count(),
            }
        }),
        usages: consumers.of(library, &model.name),
        selected_component,
        binding_block_reason,
    })
}

pub(super) fn selected_model_detail(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    consumers: &ConsumerIndex,
) {
    let detail = match project_selection(app, consumers) {
        Ok(detail) => detail,
        Err((title, reason)) => {
            empty_state(ui, title, reason);
            return;
        }
    };

    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(12, 7))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.label(
                    RichText::new(&detail.model)
                        .monospace()
                        .font(theme::mono(tokens::FS_2, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.label(
                    RichText::new(format!(
                        "{} · {} · {}",
                        detail.family, detail.level, detail.library
                    ))
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let blocked = pin_source_block_reason(
                    detail.has_root,
                    app.state.workbench.models_view.model_import_in_progress,
                );
                if ui
                    .add_enabled(
                        blocked.is_none(),
                        compact_button(if detail.pinned {
                            "Refresh pin"
                        } else {
                            "Pin source"
                        }),
                    )
                    .on_disabled_hover_text(blocked.unwrap_or_default())
                    .clicked()
                {
                    refresh_library(app, &detail.library);
                }
                if ui
                    .add_enabled(detail.source_available, compact_button("Open source"))
                    .on_disabled_hover_text("This built-in definition has no source document.")
                    .clicked()
                {
                    open_model_source(app, &detail.library, &detail.model);
                }
                if ui.add(compact_button("Compare…")).clicked() {
                    open_model_compare(app, &detail.library, &detail.model);
                }
                if ui
                    .add(compact_button(if detail.project_owned {
                        "Model editor…"
                    } else {
                        "Author project copy…"
                    }))
                    .clicked()
                {
                    if detail.project_owned {
                        app.queue_command(Command::ModelEditor);
                    } else {
                        app.queue_command(Command::ModelCreateProjectCopy);
                    }
                }
                if ui.add(compact_button("Qualification")).clicked() {
                    app.state.workbench.models_page = ModelsPage::Qualification;
                }
                let bind = ui.add_enabled(
                    detail.selected_component.is_some() && detail.binding_block_reason.is_none(),
                    compact_button("Bind to selection…"),
                );
                let bind = if let Some(reason) = detail.binding_block_reason.as_deref() {
                    bind.on_disabled_hover_text(reason)
                } else {
                    bind
                };
                if bind.clicked()
                    && let Some(component_id) = detail.selected_component
                {
                    app.queue_model_binding(component_id, &detail.library, &detail.model);
                }
            });
        });
    ui.painter().hline(
        ui.min_rect().x_range(),
        ui.min_rect().bottom(),
        Stroke::new(1.0, t.color.border),
    );
    // Exception-only, and above the cards rather than inside one: a source
    // that no longer hashes to its pin invalidates everything the cards below
    // say about this model, so it cannot be a property row among properties.
    drift::detail_banner(ui, app, &detail.library);

    ScrollArea::vertical()
        .id_salt("models-selected-detail")
        .show(ui, |ui| {
            let detail_width = ui.available_width();
            if detail_width > 1100.0 {
                ui.columns(4, |columns| {
                    parameter_card(&mut columns[0], &detail);
                    characteristic_card(&mut columns[1], &detail);
                    qualification_card(&mut columns[2], &detail);
                    usage_card(&mut columns[3], &detail, app);
                });
            } else if detail_width > 650.0 {
                ui.columns(2, |columns| {
                    parameter_card(&mut columns[0], &detail);
                    characteristic_card(&mut columns[1], &detail);
                });
                ui.columns(2, |columns| {
                    qualification_card(&mut columns[0], &detail);
                    usage_card(&mut columns[1], &detail, app);
                });
            } else {
                parameter_card(ui, &detail);
                characteristic_card(ui, &detail);
                qualification_card(ui, &detail);
                usage_card(ui, &detail, app);
            }
        });
}

/// Open the exact bytes behind a model.
///
/// Taking names rather than records is what keeps the detail pane from holding
/// a library: the retained source bytes are the largest thing in one, and this
/// click is the only place that reads them.
fn open_model_source(app: &mut ManagerRenderContext<'_>, library_name: &str, model_name: &str) {
    let Some(library) = app.state.model_library_manager.get_library(library_name) else {
        return;
    };
    let Some(model) = library.get_model(model_name) else {
        return;
    };
    let content = model
        .file_path
        .as_ref()
        .and_then(|path| {
            library
                .source_contents
                .iter()
                .find(|source| source.path == *path)
        })
        .or_else(|| library.source_contents.first());
    let opened = if let Some(content) = content {
        Ok(ModelsWorkbenchDialog::SourcePreview {
            title: model.name.clone(),
            subtitle: format!(
                "{} · {}",
                content.path.display(),
                match library.source_authority {
                    ModelSourceAuthority::ProjectOwned { .. } => "project revision",
                    ModelSourceAuthority::RetainedImport { .. } => "retained import bytes",
                    ModelSourceAuthority::External => "pinned external bytes",
                    ModelSourceAuthority::BuiltIn => "built-in",
                }
            ),
            source: String::from_utf8_lossy(&content.bytes).into_owned(),
            editable: library.source_authority.is_project_owned(),
        })
    } else if let Some(path) = model.file_path.as_ref().or(library.root_path.as_ref()) {
        match std::fs::read_to_string(path) {
            Ok(source) => Ok(ModelsWorkbenchDialog::SourcePreview {
                title: model.name.clone(),
                subtitle: format!("{} · live unpinned source", path.display()),
                source,
                editable: false,
            }),
            Err(error) => Err(format!(
                "Could not read model source '{}': {error}",
                path.display()
            )),
        }
    } else {
        return;
    };
    match opened {
        Ok(dialog) => app.state.workbench.models_view.dialog = Some(dialog),
        Err(error) => receipt(app, Err(error)),
    }
}

fn parameter_card(ui: &mut Ui, detail: &SelectedModelDetail<'_>) {
    detail_pane(
        ui,
        "RESOLVED PARAMETERS",
        Some(&format!("{} values", detail.parameter_total)),
        |ui| {
            if detail.parameters.is_empty() {
                empty_state(
                    ui,
                    "No parameter card is attached to this model.",
                    "Built-in equation defaults remain owned by the engine.",
                );
            } else {
                // A BSIM4 card carries several hundred parameters and this is
                // a column in a four-pane row. What is not listed is counted;
                // a list that stops silently reads as the whole card.
                for (name, value, origin) in &detail.parameters {
                    property(ui, name, value, origin);
                }
                let hidden = detail.parameter_total - detail.parameters.len();
                if hidden > 0 {
                    property(
                        ui,
                        "…",
                        &format!("{hidden} more"),
                        "open source for the full card",
                    );
                }
            }
            if let Some(fields) = detail.typed_schema_fields {
                ui.separator();
                property(
                    ui,
                    "Schema",
                    &format!("{fields} typed fields"),
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
fn characteristic_card(ui: &mut Ui, detail: &SelectedModelDetail<'_>) {
    detail_pane(ui, "DECLARED ENVELOPE", Some("from the card"), |ui| {
        let envelope = &detail.envelope;
        if let Some(vth) = envelope.vth0 {
            property(ui, "VTH0", &format!("{vth:.6} V"), "source card");
        }
        if let Some(vdd) = envelope.vdd {
            property(ui, "Supply", &format!("{vdd:.6} V"), "source card");
        }
        for (label, (low, high)) in [("Length", envelope.length), ("Width", envelope.width)] {
            match (low, high) {
                (Some(low), Some(high)) => {
                    property(ui, label, &format!("{low:.4e} … {high:.4e} m"), "bin range");
                }
                (Some(low), None) => {
                    property(ui, label, &format!("≥ {low:.4e} m"), "bin range");
                }
                (None, Some(high)) => {
                    property(ui, label, &format!("≤ {high:.4e} m"), "bin range");
                }
                (None, None) => {}
            }
        }
        if let Some((level, version)) = envelope.level {
            property(
                ui,
                "Level",
                &format!("{level} · version {version}"),
                "source card",
            );
        }
        if envelope.is_empty() {
            empty_state(
                ui,
                "This card declares no operating envelope.",
                "Bin ranges, threshold and supply are read from the card; nothing is inferred.",
            );
        }
    });
}

fn qualification_card(ui: &mut Ui, detail: &SelectedModelDetail<'_>) {
    detail_pane(ui, "QUALIFICATION", Some("source-owned evidence"), |ui| {
        if let Some(counts) = &detail.qualification {
            property(ui, "Suites", &counts.suites.to_string(), "retained");
            property(ui, "Vectors", &counts.vectors.to_string(), "declared");
            property(ui, "Evidence", &counts.evidence.to_string(), "immutable");
            property(ui, "Releases", &counts.releases.to_string(), "promoted");
            property(
                ui,
                "Open dispositions",
                &counts.open_dispositions.to_string(),
                if counts.open_dispositions == 0 {
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

fn usage_card(ui: &mut Ui, detail: &SelectedModelDetail<'_>, app: &mut ManagerRenderContext<'_>) {
    let usages = detail.usages;
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
                                model: detail.model.clone(),
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
                                model: detail.model.clone(),
                                consumers: usages.to_vec(),
                            });
                    }
                }
            }
        },
    );
}
