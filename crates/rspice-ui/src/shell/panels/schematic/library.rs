//! Library browser, preview, and placement strip for the schematic rail.

use super::*;

// ---------------------------------------------------------------------------
// Library
// ---------------------------------------------------------------------------

/// Library filter options for the component browser.
fn cell_sources(state: &AppState) -> Vec<String> {
    let mut libs: Vec<String> = vec!["All libs".to_owned(), "primitives".to_owned()];
    libs.extend(
        state
            .library_manager
            .libraries_sorted()
            .iter()
            .map(|lib| lib.name.clone()),
    );
    libs.dedup();
    libs
}

/// One placeable cell in the browser list.
pub(super) enum CellEntry {
    /// A built-in primitive (palette entry).
    Primitive(ComponentType, &'static str),
    /// A library cell (library, cell name).
    LibraryCell(String, String),
}

impl CellEntry {
    /// Stable ref string ("prim:<label>" / "cell:<lib>/<cell>") — selection,
    /// pins and recents survive list reordering and filtering.
    fn entry_ref(&self) -> String {
        match self {
            CellEntry::Primitive(_, label) => format!("prim:{label}"),
            CellEntry::LibraryCell(lib, cell) => format!("cell:{lib}/{cell}"),
        }
    }

    fn label(&self) -> &str {
        match self {
            CellEntry::Primitive(_, label) => label,
            CellEntry::LibraryCell(_, cell) => cell,
        }
    }
}

/// Resolve a ref string back to a placeable entry, validating it still
/// exists (libraries change; pins and recents must not dangle).
fn entry_from_ref(state: &AppState, entry_ref: &str) -> Option<CellEntry> {
    if let Some(label) = entry_ref.strip_prefix("prim:") {
        for section in crate::schematic::component_palette() {
            for entry in section.entries {
                if entry.label == label {
                    return Some(CellEntry::Primitive(entry.kind, entry.label));
                }
            }
        }
        return None;
    }
    if let Some(path) = entry_ref.strip_prefix("cell:") {
        let (lib, cell) = path.split_once('/')?;
        let library = state.library_manager.get_library(lib)?;
        library.get_cell(cell)?;
        return Some(CellEntry::LibraryCell(lib.to_owned(), cell.to_owned()));
    }
    None
}

/// The palette ref for a primitive kind (recents bookkeeping at drop time).
pub(crate) fn palette_ref(kind: ComponentType) -> Option<String> {
    for section in crate::schematic::component_palette() {
        for entry in section.entries {
            if entry.kind == kind {
                return Some(format!("prim:{}", entry.label));
            }
        }
    }
    None
}

/// One collapsible browser group.
pub(super) struct BrowserGroup {
    pub(super) title: String,
    read_only: bool,
    pub(super) entries: Vec<CellEntry>,
}

/// Browser groups for the current query + filter: pinned favorites first,
/// then palette categories, then libraries.
pub(super) fn browser_groups(state: &AppState) -> Vec<BrowserGroup> {
    let query = state.shell.cell_search.trim().to_ascii_lowercase();
    let filter = &state.shell.cell_lib_filter;
    let mut groups = Vec::new();

    if query.is_empty() && !state.shell.lib_pins.is_empty() {
        let entries: Vec<CellEntry> = state
            .shell
            .lib_pins
            .iter()
            .filter_map(|pin| entry_from_ref(state, pin))
            .collect();
        if !entries.is_empty() {
            groups.push(BrowserGroup {
                title: "★ Pinned".to_owned(),
                read_only: false,
                entries,
            });
        }
    }

    if filter == "All libs" || filter == "primitives" {
        for section in crate::schematic::component_palette() {
            let entries: Vec<CellEntry> = section
                .entries
                .iter()
                .filter(|entry| {
                    query.is_empty() || entry.label.to_ascii_lowercase().contains(&query)
                })
                .map(|entry| CellEntry::Primitive(entry.kind, entry.label))
                .collect();
            if !entries.is_empty() {
                groups.push(BrowserGroup {
                    title: section.title.to_owned(),
                    read_only: false,
                    entries,
                });
            }
        }
    }
    for library in state.library_manager.libraries_sorted() {
        if filter != "All libs" && *filter != library.name {
            continue;
        }
        let entries: Vec<CellEntry> = library
            .cells_sorted()
            .iter()
            .filter(|cell| {
                query.is_empty()
                    || cell.name.to_ascii_lowercase().contains(&query)
                    || library.name.to_ascii_lowercase().contains(&query)
            })
            .map(|cell| CellEntry::LibraryCell(library.name.clone(), cell.name.clone()))
            .collect();
        if !entries.is_empty() || (query.is_empty() && !library.read_only) {
            groups.push(BrowserGroup {
                title: library.name.clone(),
                read_only: library.read_only,
                entries,
            });
        }
    }
    groups
}

pub(super) fn library(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    // Stable preview card pinned above the place strip — selection changes
    // never shift the list layout.
    egui::TopBottomPanel::bottom("volta.rail.preview")
        .frame(
            egui::Frame::NONE
                .fill(c.bg_panel)
                .inner_margin(egui::Margin {
                    left: 12,
                    right: 12,
                    top: 8,
                    bottom: 4,
                }),
        )
        .show_separator_line(false)
        .show_inside(ui, |ui| {
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                rect.top() - 8.0,
                egui::Stroke::new(1.0, c.border),
            );
            preview_card(ui, state, symbol_library);
        });

    // Search + library filter.
    ui.add_space(8.0);
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), t.metrics.ctl_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.add_space(12.0);
            ui.spacing_mut().item_spacing.x = 6.0;
            let search_width = ui.available_width() - 96.0 - 12.0 - 6.0;
            mono_input(ui, &mut state.shell.cell_search, search_width.max(60.0));
            let libs = cell_sources(state);
            let current = state.shell.cell_lib_filter.clone();
            if let Some(index) = select(ui, "volta.cell.lib", &current, &libs, 90.0) {
                state.shell.cell_lib_filter = libs[index].clone();
            }
        },
    );
    ui.add_space(4.0);

    let groups = browser_groups(state);
    let searching = !state.shell.cell_search.trim().is_empty();
    let highlight_query = if searching {
        Some(state.shell.cell_search.trim().to_owned())
    } else {
        None
    };

    egui::ScrollArea::vertical()
        .id_salt("volta.rail.lib")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            if groups.is_empty() {
                empty_note(ui, "No cells match — clear the search or the filter.");
                return;
            }

            let mut select_ref: Option<String> = None;
            let mut place: Option<String> = None;
            let mut toggle_group: Option<String> = None;
            let mut toggle_pin: Option<String> = None;

            for group in &groups {
                let open = searching || !state.shell.lib_groups_closed.contains(&group.title);
                let meta = if group.read_only {
                    format!("ro · {}", group.entries.len())
                } else {
                    group.entries.len().to_string()
                };
                let header = TreeRow::new(&group.title)
                    .twist(open)
                    .meta(&meta)
                    .height(24.0)
                    .show(ui);
                if header.response.clicked() {
                    toggle_group = Some(group.title.clone());
                }
                if group.read_only {
                    header
                        .response
                        .on_hover_text("Read-only library — placeable, never editable");
                }
                if !open {
                    continue;
                }
                if group.entries.is_empty() {
                    TreeRow::new("No cells yet")
                        .meta("Library Manager")
                        .indent(1)
                        .dim()
                        .height(30.0)
                        .show(ui)
                        .response
                        .on_hover_text("Create a cell or import a design from the Library view");
                    continue;
                }

                for entry in &group.entries {
                    let entry_ref = entry.entry_ref();
                    let selected = state.shell.cell_selected.as_deref() == Some(&entry_ref);
                    let pinned = state.shell.lib_pins.contains(&entry_ref);

                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        ui.add_space(12.0);
                        paint_entry_thumb(ui, entry, symbol_library);
                        let meta = match entry {
                            CellEntry::Primitive(..) => if pinned { "★" } else { "" }.to_owned(),
                            CellEntry::LibraryCell(lib, _) => {
                                if pinned {
                                    format!("★ {lib}")
                                } else {
                                    lib.clone()
                                }
                            }
                        };
                        let row_width = (ui.available_width() - 26.0).max(80.0);
                        let row = ui
                            .allocate_ui_with_layout(
                                egui::vec2(row_width, 30.0),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    TreeRow::new(entry.label())
                                        .meta(&meta)
                                        .mono()
                                        .selected(selected)
                                        .highlight_query(highlight_query.as_deref())
                                        .height(30.0)
                                        .show(ui)
                                },
                            )
                            .inner;
                        if pin_star(ui, pinned).clicked() {
                            toggle_pin = Some(entry_ref.clone());
                        }
                        if row.response.double_clicked() {
                            place = Some(entry_ref.clone());
                        } else if row.response.clicked() {
                            select_ref = Some(entry_ref.clone());
                        }
                        row.response.context_menu(|ui| {
                            if ui
                                .button(if pinned {
                                    "Unpin from favorites"
                                } else {
                                    "Pin to favorites"
                                })
                                .clicked()
                            {
                                toggle_pin = Some(entry_ref.clone());
                                ui.close();
                            }
                            if ui.button("Place").clicked() {
                                place = Some(entry_ref.clone());
                                ui.close();
                            }
                        });
                    });
                }
            }

            if let Some(title) = toggle_group
                && !state.shell.lib_groups_closed.remove(&title)
            {
                state.shell.lib_groups_closed.insert(title);
            }
            if let Some(entry_ref) = toggle_pin {
                if let Some(index) = state.shell.lib_pins.iter().position(|p| *p == entry_ref) {
                    state.shell.lib_pins.remove(index);
                } else {
                    state.shell.lib_pins.push(entry_ref);
                }
            }
            if let Some(entry_ref) = select_ref {
                state.shell.cell_selected = Some(entry_ref);
            }
            if let Some(entry_ref) = place {
                state.shell.cell_selected = Some(entry_ref.clone());
                arm_ref(state, &entry_ref);
            }
            ui.add_space(8.0);
        });
}

fn pin_star(ui: &mut Ui, pinned: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let label = if pinned { "★" } else { "☆" };
    ui.add_sized(
        egui::vec2(18.0, 24.0),
        egui::Button::new(
            egui::RichText::new(label)
                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                .color(if pinned { c.accent } else { c.text_faint }),
        )
        .frame(false),
    )
    .on_hover_text(if pinned {
        "Unpin from favorites"
    } else {
        "Pin to favorites"
    })
}

/// 34x22 leading thumbnail: the real symbol for primitives, a block glyph
/// for cells.
fn paint_entry_thumb(
    ui: &mut Ui,
    entry: &CellEntry,
    symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(34.0, 22.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect(
        rect,
        t.radius,
        c.bg_inset,
        egui::Stroke::new(1.0, c.border),
        egui::StrokeKind::Inside,
    );
    let inner = rect.shrink(2.0);
    match entry {
        CellEntry::Primitive(kind, _) => {
            crate::schematic::view::draw_symbol_preview(
                painter,
                inner,
                *kind,
                c.symbol,
                symbol_library,
            );
        }
        CellEntry::LibraryCell(..) => {
            let block = egui::Rect::from_center_size(inner.center(), egui::vec2(14.0, 12.0));
            painter.rect_stroke(
                block,
                0.0,
                egui::Stroke::new(1.0, c.symbol),
                egui::StrokeKind::Inside,
            );
            for dy in [-3.0, 3.0] {
                painter.hline(
                    egui::Rangef::new(block.left() - 5.0, block.left()),
                    block.center().y + dy,
                    egui::Stroke::new(1.0, c.symbol),
                );
                painter.hline(
                    egui::Rangef::new(block.right(), block.right() + 5.0),
                    block.center().y + dy,
                    egui::Stroke::new(1.0, c.symbol),
                );
            }
        }
    }
}

/// Stable-height preview: symbol, identity, pins, one accent action.
pub(super) fn preview_place_label() -> &'static str {
    "Place ⏎"
}

fn preview_card(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let entry = state
        .shell
        .cell_selected
        .clone()
        .and_then(|entry_ref| entry_from_ref(state, &entry_ref));
    let Some(entry) = entry else {
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 64.0), egui::Sense::hover());
        ui.painter().rect(
            rect,
            t.radius,
            c.bg_inset,
            egui::Stroke::new(1.0, c.border),
            egui::StrokeKind::Inside,
        );
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "select a component to preview",
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
        ui.add_space(44.0);
        return;
    };

    // Symbol stage.
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 64.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect(
        rect,
        t.radius,
        c.bg_inset,
        egui::Stroke::new(1.0, c.border),
        egui::StrokeKind::Inside,
    );
    let stage = rect.shrink(6.0);
    let (name, meta) = match &entry {
        CellEntry::Primitive(kind, label) => {
            crate::schematic::view::draw_symbol_preview(
                painter,
                stage,
                *kind,
                c.symbol,
                symbol_library,
            );
            (
                format!("primitives / {label}"),
                format!("symbol · {}", kind.display_name()),
            )
        }
        CellEntry::LibraryCell(lib, cell) => {
            let key = format!("{lib}/{cell}/schematic");
            let ports = state
                .workspace
                .schematic_buffers
                .get(&key)
                .map(|master| master.interface_ports())
                .unwrap_or_default();
            paint_generated_preview(painter, stage, &ports, c.symbol);
            let views = state
                .library_manager
                .get_library(lib)
                .and_then(|library| library.get_cell(cell))
                .map(|cell| {
                    cell.views_sorted()
                        .iter()
                        .map(|view| view.name.clone())
                        .collect::<Vec<_>>()
                        .join(" · ")
                })
                .unwrap_or_default();
            let meta = if ports.is_empty() {
                "no ports yet — open the cell and place Port components".to_owned()
            } else {
                let pins: Vec<&str> = ports.iter().map(|p| p.name.as_str()).collect();
                format!("pins {} · {views}", pins.join(" "))
            };
            (format!("{lib} / {cell}"), meta)
        }
    };

    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(name)
            .font(theme::mono(tokens::FS_1, FontWeight::Medium))
            .color(c.text),
    );
    ui.label(
        egui::RichText::new(meta)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(c.text_faint),
    );
    ui.add_space(6.0);

    let entry_ref = entry.entry_ref();
    let pinned = state.shell.lib_pins.contains(&entry_ref);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        // Measured, not guessed: Place + the pin chip must fill the row
        // exactly — one extra pixel here ratchets the panel wider.
        let pin_label = if pinned { "★ pinned" } else { "☆ pin" };
        let chip_width = text_width(
            ui,
            pin_label,
            &theme::mono(tokens::FS_0, FontWeight::Regular),
        ) + 18.0;
        let place_width = ui.available_width() - chip_width - 6.0;
        if Button::new(preview_place_label())
            .accent()
            .min_width(place_width.max(60.0))
            .show(ui)
            .clicked()
        {
            arm_ref(state, &entry_ref);
        }
        if chip(ui, pin_label, pinned).clicked() {
            if let Some(index) = state.shell.lib_pins.iter().position(|p| *p == entry_ref) {
                state.shell.lib_pins.remove(index);
            } else {
                state.shell.lib_pins.push(entry_ref);
            }
        }
    });
    ui.add_space(2.0);
}

/// Miniature of the generated block symbol: body, side stubs, rails.
fn paint_generated_preview(
    painter: &egui::Painter,
    rect: egui::Rect,
    ports: &[PortSpec],
    color: egui::Color32,
) {
    let stroke = egui::Stroke::new(1.2, color);
    let symbol = crate::state::generate_symbol(ports);
    let (w, h) = (symbol.width as f32, symbol.height as f32);
    let scale = (rect.width() / (w + 10.0))
        .min(rect.height() / (h + 10.0))
        .min(1.2);
    let center = rect.center();
    let to_screen = |x: f32, y: f32| egui::pos2(center.x + x * scale, center.y + y * scale);

    let hh_body = (h / 2.0 - 5.0).max(15.0);
    painter.rect_stroke(
        egui::Rect::from_min_max(to_screen(-20.0, -hh_body), to_screen(20.0, hh_body)),
        0.0,
        stroke,
        egui::StrokeKind::Inside,
    );
    for pin in &symbol.pins {
        let (px, py) = (pin.offset.x as f32, pin.offset.y as f32);
        let inner = if py.abs() > hh_body {
            (px, py.signum() * hh_body)
        } else {
            (px.signum() * 20.0, py)
        };
        painter.line_segment([to_screen(px, py), to_screen(inner.0, inner.1)], stroke);
        painter.circle_filled(to_screen(px, py), 1.4, color);
    }
}

// ---------------------------------------------------------------------------
// Placement
// ---------------------------------------------------------------------------

/// Begin placement for a browser entry.
fn place_entry(state: &mut AppState, entry: &CellEntry) {
    match entry {
        CellEntry::Primitive(kind, _) => {
            state.schematic.tool = Tool::Place(*kind);
        }
        CellEntry::LibraryCell(lib, cell) => {
            let mut binding =
                crate::state::LibraryCellInstance::new(lib.clone(), cell.clone(), "schematic");
            // Bind the master's interface at placement time: the instance
            // gets its real pin count, names and directions, and the
            // generated symbol replaces the anonymous two-pin block.
            let key = format!("{lib}/{cell}/schematic");
            if let Some(master) = state.workspace.schematic_buffers.get(&key) {
                binding.bind_interface(&master.interface_ports());
            }
            state.schematic.pending_library_cell = Some(binding);
            state.schematic.tool = Tool::Place(ComponentType::CellInstance);
        }
    }
    state.shell.view = crate::shell::WorkspaceView::Schematic;
}

/// Arm placement from a ref string (chips, typeahead, preview).
fn arm_ref(state: &mut AppState, entry_ref: &str) {
    if let Some(entry) = entry_from_ref(state, entry_ref) {
        place_entry(state, &entry);
    }
}

/// Pixel width of `text` at `font`.
pub(super) fn text_width(ui: &Ui, text: &str, font: &egui::FontId) -> f32 {
    ui.fonts_mut(|f| {
        f.layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::WHITE)
            .size()
            .x
    })
}

/// Elide `text` with '…' to fit `budget` px at `font`.
///
/// For one-line labels inside horizontal layouts, where egui labels extend
/// instead of wrapping — an overflowing row is not just clipped: egui
/// persists the content rect as the panel's next-frame width, so a single
/// too-wide row ratchets the rail toward its maximum and fights the user's
/// resize. Every horizontal row in this panel must fit its budget.
pub(super) fn fit_text(ui: &Ui, text: &str, font: &egui::FontId, budget: f32) -> String {
    if text_width(ui, text, font) <= budget {
        return text.to_owned();
    }
    let mut out = text.to_owned();
    while !out.is_empty() {
        out.pop();
        let candidate = format!("{out}…");
        if text_width(ui, &candidate, font) <= budget {
            return candidate;
        }
    }
    "…".to_owned()
}

/// Short chip label for a ref ("Resistor" → "Resistor", cells → cell name).
fn ref_chip_label(entry_ref: &str) -> &str {
    entry_ref
        .strip_prefix("prim:")
        .or_else(|| {
            entry_ref
                .strip_prefix("cell:")
                .and_then(|p| p.split('/').nth(1))
        })
        .unwrap_or(entry_ref)
}

// ---------------------------------------------------------------------------
// Place strip
// ---------------------------------------------------------------------------

/// The persistent placement surface: a command slot with typeahead, and
/// the recently placed entries as one-click chips.
pub(super) fn place_strip(
    ui: &mut Ui,
    state: &mut AppState,
    _symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("PLACE")
                .font(theme::mono(10.0, FontWeight::SemiBold))
                .color(c.text_faint),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new("Shift+I")
                    .font(theme::mono(10.0, FontWeight::Regular))
                    .color(c.text_faint),
            );
        });
    });
    ui.add_space(5.0);

    let input = mono_input(ui, &mut state.shell.place_cmd, ui.available_width());
    if state.shell.focus_cell_search {
        input.request_focus();
        state.shell.focus_cell_search = false;
    }

    // Typeahead: the rail's one elevated surface. Enter arms the active
    // match; arrows move; Escape returns to the canvas.
    let query = state.shell.place_cmd.trim().to_ascii_lowercase();
    if !query.is_empty() && input.has_focus() {
        let matches = command_matches(state, &query);
        if !matches.is_empty() {
            let count = matches.len();
            ui.input(|i| {
                if i.key_pressed(egui::Key::ArrowDown) {
                    state.shell.place_pop_index = (state.shell.place_pop_index + 1) % count;
                }
                if i.key_pressed(egui::Key::ArrowUp) {
                    state.shell.place_pop_index = (state.shell.place_pop_index + count - 1) % count;
                }
            });
            state.shell.place_pop_index = state.shell.place_pop_index.min(count - 1);

            let active = state.shell.place_pop_index;
            let mut armed: Option<String> = None;
            egui::Area::new(ui.id().with("volta.place.pop"))
                .order(egui::Order::Foreground)
                .pivot(egui::Align2::LEFT_BOTTOM)
                .fixed_pos(input.rect.left_top() - egui::vec2(0.0, 4.0))
                .show(ui.ctx(), |ui| {
                    egui::Frame::NONE
                        .fill(c.bg_elevated)
                        .stroke(egui::Stroke::new(1.0, c.border_strong))
                        .rounding(t.radius_lg)
                        .shadow(egui::epaint::Shadow {
                            offset: [0, 4],
                            blur: 16,
                            spread: 0,
                            color: egui::Color32::from_black_alpha(96),
                        })
                        .inner_margin(egui::Margin::same(3))
                        .show(ui, |ui| {
                            ui.set_width(input.rect.width() - 6.0);
                            ui.spacing_mut().item_spacing.y = 0.0;
                            for (index, (entry_ref, label, group)) in matches.iter().enumerate() {
                                let row = TreeRow::new(label)
                                    .meta(group)
                                    .mono()
                                    .selected(index == active)
                                    .highlight_query(Some(query.as_str()))
                                    .height(24.0)
                                    .show(ui);
                                if row.response.clicked() {
                                    armed = Some(entry_ref.clone());
                                }
                            }
                        });
                });

            let commit = input.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if commit {
                armed = matches
                    .get(state.shell.place_pop_index)
                    .map(|(entry_ref, ..)| entry_ref.clone());
            }
            if let Some(entry_ref) = armed {
                arm_ref(state, &entry_ref);
                state.shell.place_cmd.clear();
                state.shell.place_pop_index = 0;
            }
        }
    } else if query.is_empty() {
        state.shell.place_pop_index = 0;
    }

    // Recents — the five-things-you-actually-place row.
    if !state.shell.lib_recents.is_empty() {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            // Chips render until the row is full, never past it — a chip
            // that doesn't fit is dropped, not clipped (an overflowing row
            // ratchets the panel wider).
            let chip_font = theme::mono(tokens::FS_0, FontWeight::Regular);
            let mut budget = ui.available_width();
            let recents = state.shell.lib_recents.clone();
            for entry_ref in recents.iter().take(6) {
                let chip_width = text_width(ui, ref_chip_label(entry_ref), &chip_font) + 18.0;
                if chip_width > budget {
                    break;
                }
                budget -= chip_width + 4.0;
                let armed = match (&state.schematic.tool, entry_from_ref(state, entry_ref)) {
                    (Tool::Place(active), Some(CellEntry::Primitive(kind, _))) => {
                        *active == kind && kind != ComponentType::CellInstance
                    }
                    (
                        Tool::Place(ComponentType::CellInstance),
                        Some(CellEntry::LibraryCell(lib, cell)),
                    ) => state
                        .schematic
                        .pending_library_cell
                        .as_ref()
                        .is_some_and(|binding| binding.library == lib && binding.cell == cell),
                    _ => false,
                };
                if chip(ui, ref_chip_label(entry_ref), armed)
                    .on_hover_text(format!("Place {}", ref_chip_label(entry_ref)))
                    .clicked()
                {
                    if armed {
                        state.schematic.tool = Tool::Select;
                    } else {
                        arm_ref(state, entry_ref);
                    }
                }
            }
        });
    }
}

/// Typeahead matches: palette + project/vendor cells, prefix matches
/// first, capped at six. The open cell is excluded — a cell cannot be
/// placed inside itself.
fn command_matches(state: &AppState, query: &str) -> Vec<(String, String, String)> {
    let active_cell = state.workspace.active_view.cell.to_ascii_lowercase();
    let mut matches: Vec<(String, String, String, bool)> = Vec::new();

    for section in crate::schematic::component_palette() {
        for entry in section.entries {
            let label_lower = entry.label.to_ascii_lowercase();
            if label_lower.contains(query) {
                matches.push((
                    format!("prim:{}", entry.label),
                    entry.label.to_owned(),
                    section.title.to_owned(),
                    label_lower.starts_with(query),
                ));
            }
        }
    }
    for library in state.library_manager.libraries_sorted() {
        for cell in library.cells_sorted() {
            let cell_lower = cell.name.to_ascii_lowercase();
            if cell_lower == active_cell {
                continue;
            }
            if cell_lower.contains(query) || library.name.to_ascii_lowercase().contains(query) {
                matches.push((
                    format!("cell:{}/{}", library.name, cell.name),
                    cell.name.clone(),
                    library.name.clone(),
                    cell_lower.starts_with(query),
                ));
            }
        }
    }

    matches.sort_by(|a, b| (!a.3, a.1.to_ascii_lowercase()).cmp(&(!b.3, b.1.to_ascii_lowercase())));
    matches
        .into_iter()
        .take(6)
        .map(|(entry_ref, label, group, _)| (entry_ref, label, group))
        .collect()
}
