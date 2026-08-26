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
    /// Whether this card carries a finding an engineer has to look at, and
    /// whether its library's retained bytes still hash to their pin. Both are
    /// stated in the header band rather than only in the catalog row above it:
    /// a reader who arrived at this pane from anywhere else could not see the
    /// row's cell.
    review: bool,
    drifted: bool,
    /// Whether the definition is one of the simulator's own compiled-in cards.
    engine_owned: bool,
    /// Where the card is written — `OPA189.lib:41` — when the parse retained
    /// both the file and the line.
    source_reference: Option<String>,
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
    /// The reference designator the bind action would land on, resolved only
    /// when it would actually land: a primary that names an instance it is
    /// refusing to bind to is worse than one that says "selection".
    bind_target: Option<String>,
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
        .map(|(name, value)| (name.clone(), engineering_value(*value), "source card"))
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
    // One walk of the *sheet* for one designator, and only while the action is
    // live. The catalog is the corpus-sized side of this pane; the schematic
    // is the side the consumer index already walks once a frame.
    let bind_target = selected_component
        .filter(|_| binding_block_reason.is_none())
        .and_then(|component_id| {
            app.state
                .schematic
                .components
                .iter()
                .find(|component| component.id == component_id)
                .map(|component| component.name.clone())
        })
        .filter(|reference| !reference.is_empty());

    Ok(SelectedModelDetail {
        library: library.name.clone(),
        model: model.name.clone(),
        family: model.model_type.display_name(),
        level: model.level.display_name(),
        has_root: library.has_external_root(),
        pinned: library.has_retained_closure(),
        project_owned: library.source_authority.is_project_owned(),
        source_available: !library.source_contents.is_empty() || model.file_path.is_some(),
        review: model_needs_review(library, model),
        drifted: !drift::findings_for(app.state, &library.name).is_empty(),
        engine_owned: matches!(library.source_authority, ModelSourceAuthority::BuiltIn),
        source_reference: model.file_path.as_deref().map(|path| {
            // The line is the one the card *starts* on: the parse retains a
            // card's position, not a position per parameter, and claiming a
            // line per row would be inventing provenance the source never had.
            model.source_line.map_or_else(
                || path_label(path),
                |line| format!("{}:{line}", path_label(path)),
            )
        }),
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
        bind_target,
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
                // One line of identity. A long model name wrapping the band
                // open pushes the cards below it off the page, which is the
                // same failure the action row's allocation guards against.
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
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
                // The band's own state, beside the identity it qualifies. Each
                // word is a fact this pane already holds; nothing here is a
                // verdict the release gate has not published — that page owns
                // "qualified", and repeating it from weaker evidence is how two
                // surfaces come to disagree about one model.
                for (word, tone) in [
                    ("review", detail.review.then_some(t.color.warn)),
                    ("drift", detail.drifted.then_some(t.color.warn)),
                    ("pinned", detail.pinned.then_some(t.color.ok)),
                    (
                        "engine-owned",
                        detail.engine_owned.then_some(t.color.text_faint),
                    ),
                ] {
                    if let Some(tone) = tone {
                        ui.label(
                            RichText::new(word)
                                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                .color(tone),
                        );
                    }
                }
            });
            // Right to left, so the Models page's one accent primary — the
            // binding this pane exists to make — sits hard against the right
            // edge and the supporting actions read leftward from it.
            //
            // The row's height is allocated rather than inferred: a
            // `right_to_left` layout handed the parent's whole remaining rect
            // takes all of it, which turned this header band into four hundred
            // points of inset colour and pushed the detail cards off the page.
            let track = ui.available_width();
            ui.allocate_ui_with_layout(
                egui::vec2(track, t.metrics.ctl_h),
                Layout::right_to_left(Align::Center),
                |ui| {
                    ui.set_min_width(track);
                    ui.spacing_mut().item_spacing.x = 6.0;
                    // The primary names the instance it would bind, once there
                    // is exactly one and it would take the binding. "Bind to
                    // selection…" is the same control with nothing to name;
                    // the base spelling is what the control ratchet reads.
                    let bind_label = detail.bind_target.as_deref().map_or_else(
                        || "Bind to selection…".to_owned(),
                        |reference| format!("Bind to {reference}…"),
                    );
                    let bind = Button::new(&bind_label)
                        .accent()
                        .enabled(
                            detail.selected_component.is_some()
                                && detail.binding_block_reason.is_none(),
                        )
                        .show(ui);
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
                    if Button::new("Qualification").show(ui).clicked() {
                        app.state.workbench.models_page = ModelsPage::Qualification;
                    }
                    if Button::new(if detail.project_owned {
                        "Model editor…"
                    } else {
                        "Author project copy…"
                    })
                    .show(ui)
                    .clicked()
                    {
                        if detail.project_owned {
                            app.queue_command(Command::ModelEditor);
                        } else {
                            app.queue_command(Command::ModelCreateProjectCopy);
                        }
                    }
                    if Button::new("Compare…").show(ui).clicked() {
                        open_model_compare(app, &detail.library, &detail.model);
                    }
                    if Button::new("Open source")
                        .enabled(detail.source_available)
                        .show(ui)
                        .on_disabled_hover_text("This built-in definition has no source document.")
                        .clicked()
                    {
                        open_model_source(app, &detail.library, &detail.model);
                    }
                    let blocked = pin_source_block_reason(
                        detail.has_root,
                        app.state.workbench.models_view.model_import_in_progress,
                    );
                    if Button::new(if detail.pinned {
                        "Refresh pin"
                    } else {
                        "Pin source"
                    })
                    .enabled(blocked.is_none())
                    .show(ui)
                    .on_disabled_hover_text(blocked.unwrap_or_default())
                    .clicked()
                    {
                        refresh_library(app, &detail.library);
                    }
                },
            );
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

    // The mockup reflows on the width of the *document column*, not the
    // window, and both thresholds are the container queries it declares.
    let detail_width = ui.available_width();
    let region_h = ui.available_height().max(1.0);
    if detail_width > 1100.0 {
        // One row of equal columns filling the region: no outer scroll, so the
        // panes reach the panel's bottom edge and each one scrolls its own
        // rows. Anything less leaves the surface ending in dead space.
        ui.columns(4, |columns| {
            parameter_card(&mut columns[0], &detail, region_h);
            characteristic_card(&mut columns[1], &detail, region_h);
            qualification_card(&mut columns[2], &detail, app, region_h);
            usage_card(&mut columns[3], &detail, app, region_h);
        });
    } else if detail_width > 650.0 {
        let row_h = (region_h * 0.5).max(DETAIL_PANE_MIN_H);
        ScrollArea::vertical()
            .id_salt("models-selected-detail")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.columns(2, |columns| {
                    parameter_card(&mut columns[0], &detail, row_h);
                    characteristic_card(&mut columns[1], &detail, row_h);
                });
                ui.columns(2, |columns| {
                    qualification_card(&mut columns[0], &detail, app, row_h);
                    usage_card(&mut columns[1], &detail, app, row_h);
                });
            });
    } else {
        ScrollArea::vertical()
            .id_salt("models-selected-detail")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                parameter_card(ui, &detail, DETAIL_PANE_MIN_H);
                characteristic_card(ui, &detail, DETAIL_PANE_MIN_H);
                qualification_card(ui, &detail, app, DETAIL_PANE_MIN_H);
                usage_card(ui, &detail, app, DETAIL_PANE_MIN_H);
            });
    }
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

/// A parameter row two lines tall: what the value came from, over where that
/// is written.
///
/// [`paint::property`] states an origin as one trailing word, which is the
/// right shape for a card with no source document behind it. A resolved
/// parameter has one, and "source card" with nothing beside it is a claim a
/// reader has no way to check — the point of the column is that a value
/// inherited from a technology file carries different weight from one an
/// instance line overrode, and neither can be told apart without the file.
///
/// Private to this pane rather than promoted to the shared painters: it is the
/// only two-line row in the workspace, and the shared module should gain it
/// when a second page needs one, not in anticipation of that.
fn origin_property(ui: &mut Ui, name: &str, value: &str, origin: &str, reference: &str) {
    /// Two lines of caption text plus the badge's own box.
    const ORIGIN_ROW_H: f32 = 32.0;
    /// The mockup's `.model-param-table` column widths, less the origin
    /// column, which takes what is left.
    const NAME_FRACTION: f32 = 0.26;
    const VALUE_FRACTION: f32 = 0.30;
    const INSET: f32 = 3.0;

    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), ORIGIN_ROW_H),
        Sense::hover(),
    );
    let name_width = rect.width() * NAME_FRACTION;
    let value_width = rect.width() * VALUE_FRACTION;
    let origin_left = rect.left() + name_width + value_width + INSET;
    let origin_width = (rect.right() - origin_left - INSET).max(1.0);
    // Both cells of the origin column sit on their own line, so the name and
    // the value align to the first one rather than to the row's centre.
    let first_line = rect.top() + 9.0;
    let second_line = rect.top() + 23.0;

    ui.painter().text(
        egui::pos2(rect.left() + INSET, first_line),
        egui::Align2::LEFT_CENTER,
        elide(ui, name, (name_width - INSET * 2.0).max(1.0), false),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(rect.left() + name_width + INSET, first_line),
        egui::Align2::LEFT_CENTER,
        elide(ui, value, (value_width - INSET * 2.0).max(1.0), true),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text,
    );

    let badge_font = theme::sans(tokens::FS_MICRO, FontWeight::Regular);
    let badge_text = crate::workbench::design_system::elide_text(
        ui,
        origin,
        &badge_font,
        (origin_width - 10.0).max(1.0),
    );
    let badge_width = ui
        .painter()
        .layout_no_wrap(badge_text.clone(), badge_font.clone(), t.color.text_dim)
        .size()
        .x;
    let badge = egui::Rect::from_min_size(
        egui::pos2(origin_left, first_line - 7.0),
        egui::vec2((badge_width + 10.0).min(origin_width), 14.0),
    );
    ui.painter().rect(
        badge,
        3.0,
        Color32::TRANSPARENT,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        badge.left_center() + egui::vec2(5.0, 0.0),
        egui::Align2::LEFT_CENTER,
        badge_text,
        badge_font,
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(origin_left, second_line),
        egui::Align2::LEFT_CENTER,
        crate::workbench::design_system::elide_text(
            ui,
            reference,
            &theme::mono(tokens::FS_MICRO, FontWeight::Regular),
            origin_width,
        ),
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_faint,
    );
}

/// A counted row that opens the page the count is kept on.
///
/// The qualification column states five numbers a reader cannot act on where
/// they are shown; the page that owns them is one route away and the card had
/// no way in. Each row is that one route rather than five new controls — the
/// action bar's "Qualification" button is the same destination, and this is
/// the count itself being the way there.
///
/// Painted rather than built from links: this is a data row, and a column of
/// five link widgets in a four-pane detail region reads as a form. The node it
/// publishes carries the count, because nothing else painted here reaches a
/// reader who cannot see it.
fn routed_count(ui: &mut Ui, name: &str, value: &str, note: &str) -> egui::Response {
    const INSET: f32 = 3.0;
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 22.0), Sense::click());
    let announcement = format!("{name}: {value} · open Qualification");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &announcement)
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    theme::paint_focus_ring(ui, &response, rect);
    let name_width = rect.width() * 0.30;
    let value_width = rect.width() * 0.34;
    let note_width = (rect.width() - name_width - value_width).max(1.0);
    ui.painter().text(
        egui::pos2(rect.left() + INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        elide(ui, name, (name_width - INSET * 2.0).max(1.0), false),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    // The number is the control, so it is the number that carries the accent.
    ui.painter().text(
        egui::pos2(rect.left() + name_width + INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        elide(ui, value, (value_width - INSET * 2.0).max(1.0), true),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.accent,
    );
    ui.painter().text(
        egui::pos2(rect.right() - INSET, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        elide(ui, note, (note_width - INSET * 2.0).max(1.0), false),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    response
}

fn parameter_card(ui: &mut Ui, detail: &SelectedModelDetail<'_>, height: f32) {
    filled_detail_pane(
        ui,
        "RESOLVED PARAMETERS",
        Some(&format!("{} values", detail.parameter_total)),
        height,
        "models-detail-parameters",
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
                    match detail.source_reference.as_deref() {
                        Some(reference) => origin_property(ui, name, value, origin, reference),
                        // A built-in card is written in the simulator, not in
                        // a file, so the second line would have nothing true
                        // to put on it.
                        None => property(ui, name, value, origin),
                    }
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
fn characteristic_card(ui: &mut Ui, detail: &SelectedModelDetail<'_>, height: f32) {
    filled_detail_pane(
        ui,
        "DECLARED ENVELOPE",
        Some("from the card"),
        height,
        "models-detail-envelope",
        |ui| {
            let envelope = &detail.envelope;
            if let Some(vth) = envelope.vth0 {
                property(ui, "VTH0", &engineering_quantity(vth, "V"), "source card");
            }
            if let Some(vdd) = envelope.vdd {
                property(ui, "Supply", &engineering_quantity(vdd, "V"), "source card");
            }
            for (label, (low, high)) in [("Length", envelope.length), ("Width", envelope.width)] {
                match (low, high) {
                    (Some(low), Some(high)) => {
                        property(
                            ui,
                            label,
                            &format!(
                                "{} … {}",
                                engineering_value(low),
                                engineering_quantity(high, "m")
                            ),
                            "bin range",
                        );
                    }
                    (Some(low), None) => {
                        property(
                            ui,
                            label,
                            &format!("≥ {}", engineering_quantity(low, "m")),
                            "bin range",
                        );
                    }
                    (None, Some(high)) => {
                        property(
                            ui,
                            label,
                            &format!("≤ {}", engineering_quantity(high, "m")),
                            "bin range",
                        );
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
                // A built-in card declares nothing because the engine holds
                // its defaults, which is its design and not a gap in it. The
                // general copy read as the latter on every foundation family.
                if detail.engine_owned {
                    empty_state(
                        ui,
                        "The engine owns this family's defaults.",
                        "A built-in card declares no envelope of its own; bin ranges, threshold and supply arrive with a source card.",
                    );
                } else {
                    empty_state(
                        ui,
                        "This card declares no operating envelope.",
                        "Bin ranges, threshold and supply are read from the card; nothing is inferred.",
                    );
                }
            }
        },
    );
}

fn qualification_card(
    ui: &mut Ui,
    detail: &SelectedModelDetail<'_>,
    app: &mut ManagerRenderContext<'_>,
    height: f32,
) {
    filled_detail_pane(
        ui,
        "QUALIFICATION",
        Some("source-owned evidence"),
        height,
        "models-detail-qualification",
        |ui| {
            if let Some(counts) = &detail.qualification {
                // Every count is a way into the page that holds it — the same
                // route the action bar's "Qualification" button takes, reached
                // from the number a reader is already looking at.
                let mut routed = false;
                for (name, value, note) in [
                    ("Suites", counts.suites, "retained"),
                    ("Vectors", counts.vectors, "declared"),
                    ("Evidence", counts.evidence, "immutable"),
                    ("Releases", counts.releases, "promoted"),
                    (
                        "Open dispositions",
                        counts.open_dispositions,
                        if counts.open_dispositions == 0 {
                            "clean"
                        } else {
                            "review required"
                        },
                    ),
                ] {
                    routed |= routed_count(ui, name, &value.to_string(), note).clicked();
                }
                if routed {
                    app.state.workbench.models_page = ModelsPage::Qualification;
                }
            } else {
                empty_state(
                    ui,
                    "This model has no qualification suite.",
                    "Qualification claims remain empty until a retained suite and exact-source evidence exist.",
                );
            }
        },
    );
}

fn usage_card(
    ui: &mut Ui,
    detail: &SelectedModelDetail<'_>,
    app: &mut ManagerRenderContext<'_>,
    height: f32,
) {
    let usages = detail.usages;
    filled_detail_pane(
        ui,
        "WHERE USED",
        Some(&format!("{} consumers", usages.len())),
        height,
        "models-detail-usage",
        |ui| {
            if usages.is_empty() {
                empty_state(
                    ui,
                    "Not bound in the active project.",
                    "Place an instance or select one and use Bind to selection.",
                );
            } else {
                let t = Tokens::get(ui.ctx());
                for usage in usages.iter().take(USAGE_ROWS) {
                    // The designator is what a reader scans this column for,
                    // so it is the designator that carries the row and the
                    // rest of the label — what the instance is, where it sits
                    // — reads as the location it is.
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        let opened = ui
                            .link(
                                RichText::new(bindings::consumer_designator(usage))
                                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold)),
                            )
                            .clicked();
                        ui.label(
                            RichText::new(bindings::consumer_location(usage))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                        if opened {
                            app.state.workbench.models_view.dialog =
                                Some(ModelsWorkbenchDialog::BindingTrace {
                                    model: detail.model.clone(),
                                    consumers: usages.to_vec(),
                                });
                        }
                    });
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
