//! Specialist Models & PDKs page: symbols.
//!
//! Two corpora in one registry — the project's own symbol libraries and the
//! signed technology package's — with the selected symbol's executable
//! contract beside them: its pins against the provider's ports, the instance
//! line it emits token by token, where it is placed, and what revision of it
//! is saved.
//!
//! Everything the detail states is derived in [`super::symbol_contracts`] from
//! the stored definition. Nothing here invents a fact: a legacy symbol with no
//! typed contract gets a card that says so, not a table of plausible-looking
//! blanks.

use super::symbol_contracts::{
    InstanceBinding, NetlistTemplate, PinContract, PinContractRow, PlacedInstances, SymbolRow,
    SymbolRowAuthority, SymbolStatus, netlist_template, pin_contract, placed_instances,
    project_row_count, symbol_rows, technology_group_band,
};
use super::*;

pub(super) fn symbols_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    // The rows come first so the page bar can state the registry's real
    // populations instead of a fixed sentence about them. Deriving them is
    // cached; asking early costs nothing.
    let rows = symbol_rows(ui, app);
    let project_count = project_row_count(&rows);
    let technology_count = rows.len() - project_count;
    let blocking = rows
        .iter()
        .filter(|row| row.status == SymbolStatus::PinMismatch)
        .count();
    section_title(
        ui,
        "Symbols, pins & device forms",
        // Three zeroes is not a population, it is the empty state saying so
        // twice; an empty registry keeps the sentence that says what the page
        // is for.
        &if rows.is_empty() {
            "Project and technology symbol contracts · pin order is netlist order".to_owned()
        } else {
            format!(
                "{project_count} project forms · {technology_count} technology symbols · \
                 {blocking} blocking · pin order is netlist order"
            )
        },
        // Outermost-right first: the band lays its actions out right to left.
        |ui| {
            if Button::new("Create symbol").accent().show(ui).clicked() {
                super::super::open_create_model_bound_symbol_dialog(app.state);
            }
            if Button::new("Form designer").show(ui).clicked() {
                super::super::open_symbol_parameter_form_dialog(app.state);
            }
            if Button::new("Import symbol").show(ui).clicked() {
                super::super::open_symbol_import_dialog(app.state);
            }
            if Button::new("Library manager").show(ui).clicked() {
                navigate_specialist(app, crate::workbench::SurfaceId::LibraryCellviewManager);
            }
        },
    );
    if rows.is_empty() {
        page_empty_state(
            ui,
            "No symbol contracts are loaded",
            "Import a symbol or create a model-bound symbol to establish an executable pin and parameter contract.",
        );
        return;
    }
    let available = ui.available_size();
    // The rule between the two columns is a real widget with item spacing on
    // both sides of it. Splitting the panel as if it cost one pixel pushed the
    // detail column past the panel's right edge by the rest, and every card
    // title's right-aligned meta — the pin tally, the token count, the
    // instance count — was clipped off the end of it.
    let gap = SEPARATOR_W + ui.spacing().item_spacing.x * 2.0;
    let splittable = (available.x - gap).max(1.0);
    let right_width = (splittable * 0.425)
        .max(330.0)
        .min((splittable - 260.0).max(1.0));
    let left_width = (splittable - right_width).max(260.0);
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
                let layout = RegistryLayout {
                    rows: rows.len(),
                    band: technology_group_band(&rows).map(|label| (project_count, label)),
                };
                let selected = app
                    .state
                    .workbench
                    .models_view
                    .selected_symbol
                    .as_deref()
                    .map(str::to_owned);
                let height = (available.y - HEADER_H - CATALOG_FOOT_H).max(120.0);
                if let Some(index) =
                    symbol_registry(ui, height, &rows, &layout, selected.as_deref())
                {
                    let row = &rows[index];
                    app.state.workbench.models_view.selected_symbol =
                        Some(symbol_key(&row.reference));
                    if !row.read_only() {
                        app.state.library_manager.select_view(
                            &row.reference.library,
                            &row.reference.cell,
                            &row.reference.view,
                        );
                    }
                }
                catalog_footer(
                    ui,
                    rows.len(),
                    rows.len(),
                    rows.iter()
                        .filter(|row| row.status.needs_attention())
                        .count(),
                    &format!("symbols · {project_count} project · {technology_count} technology"),
                );
            },
        );
        ui.separator();
        ui.allocate_ui_with_layout(
            egui::vec2(right_width, available.y),
            Layout::top_down(Align::Min),
            |ui| symbol_detail(ui, app, &rows, project_count, technology_count),
        );
    });
}

/// The width `ui.separator()` allocates for the rule itself, from egui's
/// `Separator::spacing` default. The item spacing on either side of it is the
/// theme's and is read from the `Ui`.
const SEPARATOR_W: f32 = 6.0;

/// Height of a symbol registry row, which the scroll area needs up front to
/// place the scrollbar while building only the rows on screen.
const SYMBOL_ROW_H: f32 = 36.0;
/// Height of the band that separates the two corpora. Deliberately shorter
/// than a row — it is a label over the group, not an entry in it.
const SYMBOL_BAND_H: f32 = 22.0;

/// Where every entry of the registry sits, so the scroll area can build only
/// the entries on screen.
///
/// The rows are uniform and the group band is not, so this cannot be a
/// `show_rows` over one pitch — and it must stay virtualized, because a
/// technology library contributes a symbol per device it offers and the
/// production-scale fixture holds six hundred. With exactly one band the
/// offsets are still closed-form: an entry above it starts at `index *
/// SYMBOL_ROW_H` and an entry below it starts at that plus the band. Nothing
/// here is per-row state, so the layout costs the same at six rows and six
/// hundred.
struct RegistryLayout {
    rows: usize,
    /// Where the band sits in the entry list, and what it says. `None` when
    /// only one of the two corpora has any rows and there is nothing to
    /// separate.
    band: Option<(usize, String)>,
}

impl RegistryLayout {
    /// Rows plus the band, which occupies one entry of its own.
    fn entries(&self) -> usize {
        self.rows + usize::from(self.band.is_some())
    }

    fn band_at(&self) -> Option<usize> {
        self.band.as_ref().map(|(at, _)| *at)
    }

    fn band_height(&self) -> f32 {
        if self.band.is_some() {
            SYMBOL_BAND_H
        } else {
            0.0
        }
    }

    fn total_height(&self) -> f32 {
        self.rows as f32 * SYMBOL_ROW_H + self.band_height()
    }

    /// The top edge of an entry, in the scroll area's content coordinates.
    fn top(&self, entry: usize) -> f32 {
        match self.band_at() {
            Some(at) if entry > at => (entry - 1) as f32 * SYMBOL_ROW_H + SYMBOL_BAND_H,
            _ => entry as f32 * SYMBOL_ROW_H,
        }
    }

    /// Which row an entry is, or `None` where it is the band.
    fn row_of(&self, entry: usize) -> Option<usize> {
        match self.band_at() {
            Some(at) if entry == at => None,
            Some(at) if entry > at => Some(entry - 1),
            _ => Some(entry),
        }
    }

    /// The entry containing a content-space `y` — [`Self::top`] inverted, and
    /// clamped to the entry list so a viewport past either end is empty rather
    /// than out of bounds.
    fn entry_at(&self, y: f32) -> usize {
        let entries = self.entries();
        let index_from = |origin: f32, offset: usize| {
            offset + (((y - origin) / SYMBOL_ROW_H).floor().max(0.0) as usize)
        };
        match self.band_at() {
            Some(at) => {
                let band_top = at as f32 * SYMBOL_ROW_H;
                if y < band_top {
                    index_from(0.0, 0).min(at)
                } else if y < band_top + SYMBOL_BAND_H {
                    at
                } else {
                    index_from(band_top + SYMBOL_BAND_H, at + 1).min(entries)
                }
            }
            None => index_from(0.0, 0).min(entries),
        }
    }
}

/// The registry table, virtualized over its heterogeneous entries.
///
/// Returns the row a click landed on, if any.
fn symbol_registry(
    ui: &mut Ui,
    height: f32,
    rows: &[SymbolRow],
    layout: &RegistryLayout,
    selected_key: Option<&str>,
) -> Option<usize> {
    let mut clicked = None;
    ScrollArea::vertical()
        .id_salt("models-symbol-registry")
        .max_height(height)
        .show_viewport(ui, |ui, viewport| {
            // The offsets below are exact heights, so the pitch must be the
            // row height and nothing else.
            ui.spacing_mut().item_spacing.y = 0.0;
            let width = ui.available_width();
            let first = layout.entry_at(viewport.min.y);
            let last = (layout.entry_at(viewport.max.y) + 1).min(layout.entries());
            ui.allocate_space(egui::vec2(width, layout.top(first)));
            for entry in first..last {
                match layout.row_of(entry) {
                    None => {
                        let (_, label) = layout.band.as_ref().expect("row_of names the band");
                        symbol_group_band(ui, label);
                    }
                    Some(index) => {
                        let row = &rows[index];
                        let selected = selected_key == Some(symbol_key(&row.reference).as_str());
                        if symbol_registry_row(ui, selected, row).clicked() {
                            clicked = Some(index);
                        }
                    }
                }
            }
            ui.allocate_space(egui::vec2(
                width,
                (layout.total_height() - layout.top(last)).max(0.0),
            ));
        });
    clicked
}

/// The band between the two corpora.
///
/// Painted like a table header rather than drawn as a chip, because that is
/// what it is: a second header, for the rows under it. Its text is painted and
/// therefore invisible to a screen reader on its own, so the response carries
/// the whole sentence — the band is the only thing that says a reader has
/// crossed from symbols this project may edit into symbols it may not.
fn symbol_group_band(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), SYMBOL_BAND_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 5.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        elide(ui, label, (rect.width() - 10.0).max(1.0), false),
        theme::sans(tokens::FS_MICRO, FontWeight::SemiBold),
        t.color.text_faint,
    );
    let announced = label.to_owned();
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), announced.clone())
    });
}

/// The colour a status reads in.
fn status_color(t: &Tokens, status: SymbolStatus) -> Color32 {
    match status {
        SymbolStatus::PinMismatch => t.color.err,
        SymbolStatus::Review => t.color.warn,
        SymbolStatus::Bound | SymbolStatus::ReadOnly => t.color.text_dim,
    }
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
    } else if row.status == SymbolStatus::PinMismatch {
        // A blocking row is tinted whole, not only in its last column: the
        // mismatch is a property of the symbol, and the status cell is the
        // narrowest column on the table.
        ui.painter()
            .rect_filled(rect, 0.0, t.color.err.linear_multiply(0.08));
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
        row.status.label().to_owned(),
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
            if index == 4 {
                status_color(&t, row.status)
            } else {
                t.color.text_dim
            },
        );
        x += width;
    }
    // The family alone was the announced name, which is the one column two
    // rows of the same device family share. The cellview and the verdict are
    // what tell them apart.
    let row_label = format!(
        "{}/{} · {} · {}",
        row.reference.cell,
        row.reference.view,
        row.family,
        row.status.label()
    );
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

fn symbol_detail(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    rows: &[SymbolRow],
    project_count: usize,
    technology_count: usize,
) {
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
                        if Button::new(if row.read_only() {
                                "Author a variant…"
                            } else {
                                "Open symbol editor"
                            }).show(ui)
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
                        if Button::new("Edit form…").enabled(!row.read_only()).show(ui)
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
                    card_title(ui, "BLOCKING CONTRACT FINDINGS", Some(row.status.label()));
                    for diagnostic in &row.diagnostics {
                        ui.label(
                            RichText::new(format!("Warning: {diagnostic}"))
                                .small()
                                .color(status_color(&t, row.status)),
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
            pin_contract_card(ui, &row);
            netlist_template_card(ui, &row);
            placed_instances_card(ui, app, &row);
            provenance_card(ui, &row, project_count, technology_count);
            parameter_form_card(ui, &row);
        });
}

/// The pin contract, position by position, against the provider's ports.
fn pin_contract_card(ui: &mut Ui, row: &SymbolRow) {
    let contract = pin_contract(row);
    card(ui, |ui| match &contract {
        PinContract::Checked(pins) => {
            let aligned = pins.iter().filter(|pin| pin.check.is_aligned()).count();
            card_title(
                ui,
                "PIN CONTRACT VS PROVIDER PORTS",
                Some(&format!("{aligned}/{} aligned", pins.len())),
            );
            detail_table_header(
                ui,
                &[
                    ("PIN", 0.20),
                    ("ELECTRICAL", 0.26),
                    ("ORDER", 0.10),
                    ("PROVIDER PORT", 0.24),
                    ("CHECK", 0.20),
                ],
            );
            for pin in pins {
                pin_contract_row(ui, pin);
            }
            ui.label(
                RichText::new(
                    "Pin order is netlist order, and it is validated against the bound \
                         provider's terminals on every save.",
                )
                .small()
                .color(Tokens::get(ui.ctx()).color.text_faint),
            );
        }
        PinContract::NoProviderPorts(reason) => {
            card_title(ui, "PIN CONTRACT VS PROVIDER PORTS", Some("no provider"));
            empty_state(ui, "There are no provider ports to check against.", reason);
        }
        PinContract::Legacy(pins) => {
            card_title(
                ui,
                "PIN CONTRACT VS PROVIDER PORTS",
                Some(&format!("{} drawn pins", pins.len())),
            );
            for (index, pin) in pins.iter().enumerate() {
                property(ui, &format!("{:02}", index + 1), pin, "drawn artwork");
            }
            empty_state(
                ui,
                "This legacy symbol declares no typed contract.",
                "Its drawn pins are listed above; nothing states which provider terminal \
                     each one is, so there is nothing to check them against.",
            );
        }
    });
}

fn pin_contract_row(ui: &mut Ui, pin: &PinContractRow) {
    let t = Tokens::get(ui.ctx());
    let check_color = if pin.check.is_aligned() {
        t.color.ok
    } else {
        t.color.err
    };
    let order = pin.order.to_string();
    let columns = [
        (pin.pin.as_deref().unwrap_or("—"), 0.20, true),
        (pin.electrical.as_str(), 0.26, false),
        (order.as_str(), 0.10, true),
        (pin.provider_port.as_deref().unwrap_or("—"), 0.24, true),
    ];
    let announced = format!(
        "pin {} · {} · order {} · provider port {} · {}",
        pin.pin.as_deref().unwrap_or("none"),
        pin.electrical,
        pin.order,
        pin.provider_port.as_deref().unwrap_or("none"),
        pin.check.label()
    );
    let rect = detail_table_row(ui, &columns, announced);
    ui.painter().text(
        egui::pos2(rect.left() + rect.width() * 0.80 + 5.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        elide(
            ui,
            if pin.check.is_aligned() {
                "aligned"
            } else {
                pin.check.label()
            },
            (rect.width() * 0.20 - 9.0).max(8.0),
            false,
        ),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        check_color,
    );
}

/// The instance line this symbol emits, token by token.
fn netlist_template_card(ui: &mut Ui, row: &SymbolRow) {
    let Some(definition) = &row.definition else {
        // A legacy symbol has no validated template to break into tokens, so
        // it keeps the one line it does carry.
        card(ui, |ui| {
            card_title(ui, "NETLIST TEMPLATE", Some(&row.family));
            property(ui, "Template", &row.template, "not a typed contract");
            empty_state(
                ui,
                "This legacy symbol emits no declared instance line.",
                "Create a model-bound symbol to publish an executable template.",
            );
        });
        return;
    };
    let template = netlist_template(definition);
    card(ui, |ui| {
        let t = Tokens::get(ui.ctx());
        match &template {
            NetlistTemplate::Tokens(tokens) => {
                let unresolved = tokens.iter().filter(|token| !token.resolves).count();
                card_title(
                    ui,
                    "NETLIST TEMPLATE",
                    Some(&if unresolved == 0 {
                        format!("{} tokens · all resolve", tokens.len())
                    } else {
                        format!("{} tokens · {unresolved} unresolved", tokens.len())
                    }),
                );
                template_line(ui, &definition.netlist.template, t.color.text);
                detail_table_header(
                    ui,
                    &[
                        ("TOKEN", 0.22),
                        ("SOURCE", 0.22),
                        ("VALUE", 0.32),
                        ("OWNER", 0.24),
                    ],
                );
                for token in tokens {
                    let columns = [
                        (token.token.as_str(), 0.22, true),
                        (token.source, 0.22, false),
                        (token.value.as_str(), 0.32, true),
                        (token.owner, 0.24, false),
                    ];
                    let announced = format!(
                        "token {} · from {} · {} · owned by {}{}",
                        token.token,
                        token.source,
                        token.value,
                        token.owner,
                        if token.resolves {
                            ""
                        } else {
                            " · resolves from nothing"
                        }
                    );
                    let rect = detail_table_row(ui, &columns, announced);
                    if !token.resolves {
                        ui.painter().vline(
                            rect.left(),
                            rect.y_range(),
                            Stroke::new(2.0, t.color.err),
                        );
                    }
                }
                ui.label(
                    RichText::new(
                        "A token that resolves from nothing blocks netlisting rather than \
                         emitting an empty field.",
                    )
                    .small()
                    .color(t.color.text_faint),
                );
            }
            NetlistTemplate::Invalid(reason) => {
                card_title(ui, "NETLIST TEMPLATE", Some("refused"));
                template_line(ui, &definition.netlist.template, t.color.err);
                ui.label(
                    RichText::new(format!(
                        "The netlist writer refuses this template: {reason}. No instance line \
                         is emitted for it, so there are no tokens to account for.",
                    ))
                    .small()
                    .color(t.color.err),
                );
            }
        }
    });
}

/// The template itself, in the well the mockup gives it.
fn template_line(ui: &mut Ui, template: &str, color: Color32) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(8, 5))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            ui.label(
                RichText::new(if template.trim().is_empty() {
                    "not defined"
                } else {
                    template
                })
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(color),
            );
        });
}

/// Where this symbol is placed on the active sheet.
fn placed_instances_card(ui: &mut Ui, app: &ManagerRenderContext<'_>, row: &SymbolRow) {
    let PlacedInstances { total, shown } = placed_instances(ui, app, row);
    card(ui, |ui| {
        card_title(
            ui,
            "PLACED INSTANCES",
            Some(&format!("{total} on this sheet")),
        );
        if total == 0 {
            empty_state(
                ui,
                "Nothing on the active sheet is placed from this symbol.",
                "Instances are counted on the sheet being edited, not across the hierarchy.",
            );
            return;
        }
        for instance in &shown {
            let columns = [
                (instance.designator.as_str(), 0.22, true),
                (instance.location.as_str(), 0.52, true),
                (instance.binding.label(), 0.26, false),
            ];
            let announced = format!(
                "instance {} at {} · bound by {}",
                instance.designator,
                instance.location,
                instance.binding.label()
            );
            detail_table_row(ui, &columns, announced);
        }
        if total > shown.len() {
            ui.label(
                RichText::new(format!(
                    "{} more on this sheet · open the design to see them all",
                    total - shown.len()
                ))
                .small()
                .color(Tokens::get(ui.ctx()).color.text_faint),
            );
        }
        if shown
            .iter()
            .any(|instance| instance.binding == InstanceBinding::Model)
        {
            ui.label(
                RichText::new(
                    "An instance bound by model name reaches the same provider without naming \
                     this symbol, so it does not follow the symbol's pin contract.",
                )
                .small()
                .color(Tokens::get(ui.ctx()).color.text_faint),
            );
        }
    });
}

/// What revision of this symbol is saved, who owns it, and whether it can be
/// edited from here.
fn provenance_card(ui: &mut Ui, row: &SymbolRow, project_count: usize, technology_count: usize) {
    card(ui, |ui| {
        card_title(ui, "PROVENANCE", Some(row.status.label()));
        match (&row.authority, &row.definition) {
            (SymbolRowAuthority::SignedTechnology { revision, .. }, _) => {
                property(ui, "Saved revision", revision, "signed manifest");
            }
            (_, Some(definition)) => {
                property(
                    ui,
                    "Saved revision",
                    &format!("r{}", definition.identity.revision),
                    &definition.identity.binding_id,
                );
                property(
                    ui,
                    "Form revision",
                    &format!("r{}", definition.parameter_form.revision),
                    "parameter contract",
                );
            }
            (_, None) => {
                property(
                    ui,
                    "Saved revision",
                    "not versioned",
                    "legacy view metadata",
                );
            }
        }
        match &row.authority {
            SymbolRowAuthority::SignedTechnology {
                technology_name, ..
            } => property(ui, "Owner", technology_name, "signed technology package"),
            SymbolRowAuthority::DesignLibrary { read_only } => property(
                ui,
                "Owner",
                &row.reference.library,
                if *read_only {
                    "read-only design library"
                } else {
                    "project symbol library"
                },
            ),
        }
        let (editable, why) = match &row.authority {
            SymbolRowAuthority::SignedTechnology { .. } => {
                ("no", "publisher-signed · author a project variant to edit")
            }
            SymbolRowAuthority::DesignLibrary { read_only: true } => {
                ("no", "the owning design library is read-only")
            }
            SymbolRowAuthority::DesignLibrary { read_only: false } => {
                ("yes", "a save publishes the next revision")
            }
        };
        property(ui, "Editable here", editable, why);
        property(
            ui,
            "Registry",
            &format!("{project_count} project · {technology_count} technology"),
            "symbols this project browses",
        );
    });
}

/// The typed component form, which is what an instance dialog offers.
fn parameter_form_card(ui: &mut Ui, row: &SymbolRow) {
    card(ui, |ui| {
        let field_count = row
            .definition
            .as_ref()
            .map_or(0, |definition| definition.parameter_form.fields().count());
        card_title(
            ui,
            "PARAMETER FORM · CDF",
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
}

/// Height of a row in one of the detail's tables. Tighter than the catalog's
/// `ROW_H`, because these tables sit inside a card rather than under a header.
const DETAIL_ROW_H: f32 = 20.0;

/// A column header for a table drawn inside a detail card.
///
/// The workspace's shared `table_header` fills a whole inset band the width of
/// a catalogue and stands 27px tall, which inside a 7px card margin reads as a
/// second page header. This is the same column arithmetic at the card's scale.
fn detail_table_header(ui: &mut Ui, columns: &[(&str, f32)]) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), DETAIL_ROW_H),
        Sense::hover(),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    let mut x = rect.left() + 5.0;
    for (label, fraction) in columns {
        let width = rect.width() * fraction;
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            elide(ui, label, (width - 9.0).max(8.0), false),
            theme::sans(tokens::FS_MICRO, FontWeight::SemiBold),
            t.color.text_faint,
        );
        x += width;
    }
}

/// One row of a table drawn inside a detail card.
///
/// Every cell is painted, which publishes nothing at all to a screen reader —
/// so the whole row is announced as one sentence on the response, the way the
/// catalogue's `selectable_data_row` announces its identifier. A five-column
/// pin verdict that only sighted readers could read would be the one table on
/// this page whose entire content is the finding.
fn detail_table_row(ui: &mut Ui, columns: &[(&str, f32, bool)], announced: String) -> egui::Rect {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), DETAIL_ROW_H),
        Sense::hover(),
    );
    paint_columns(ui, rect, columns);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), announced.clone())
    });
    rect
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

// At the bottom, and spelled exactly this way: `source_guard::production_half`
// cuts a self-scanning source here, and the control ratchet reads this file
// through it. A declaration higher up would leave the ratchet scanning the
// module doc comment, and a guard that scans nothing passes forever.
#[cfg(test)]
mod tests;
