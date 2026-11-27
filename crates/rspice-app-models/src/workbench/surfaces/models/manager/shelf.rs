//! The shelf: a dense part chooser, not a card wall.
//!
//! One row per part a loaded library holds, and the columns themselves change
//! with what the rows actually differ on — because a column whose every cell
//! says the same word is a fact about the filter rather than about the rows.
//!
//! # A column earns its width
//!
//! [`shelf_columns`] is handed the page of rows and keeps only the columns
//! those rows disagree on. Narrowing to one device class collapses the class
//! column and hands its width to the ones that still distinguish parts; the
//! collapsed fact is stated once, in the footer, where it is true of the whole
//! page rather than repeated on every line.
//!
//! # Where the rows come from
//!
//! The rows are the loaded libraries' own parts, read through
//! [`library_part_index`]: the cards compiled into this build and every
//! definition the project holds, in name order. Each row names the library
//! that holds it, which is also the library that places it.

use super::*;

use crate::state::model_library::{LibraryPart, LibraryPartKind, library_part_index};

/// One shelf line: a part a loaded library holds, and the card behind it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ShelfRow {
    pub(super) part: LibraryPart,
    /// The holding library's title, or its name where it has none.
    pub(super) title: String,
    /// The file the holding library was read from, when a file backs it.
    pub(super) source: Option<PathBuf>,
    /// The line the card starts at; 0 where the parse recorded none.
    pub(super) line: usize,
}

/// The word a part's definition is declared with, as the Kind column and the
/// footer spell it.
const fn kind_word(kind: LibraryPartKind) -> &'static str {
    match kind {
        LibraryPartKind::Model => "model",
        LibraryPartKind::Subcircuit => "subckt",
    }
}

/// A library's title, or its name where it has none.
fn library_title(library: &ModelLibrary) -> &str {
    if library.pdk_name.trim().is_empty() {
        &library.name
    } else {
        &library.pdk_name
    }
}

/// The selection key of one row: the library and the part it holds.
fn part_key(row: &ShelfRow) -> String {
    format!("{}\u{1f}{}", row.part.library, row.part.part_id)
}

/// What one shelf column is, and how to read it off a row.
struct ShelfColumn {
    heading: &'static str,
    width: f32,
    mono: bool,
    read: fn(&ShelfRow) -> String,
}

/// The columns this page of rows earns, and the facts they all share.
struct ShelfColumns {
    columns: Vec<ShelfColumn>,
    /// Facts every row on the page agrees on, stated once for the footer
    /// instead of repeated down a column.
    shared: Vec<String>,
}

/// Build the column set from the rows themselves.
///
/// Identity, library and state are always present: they are the row's name,
/// where it came from, and whether the project holds it. Class and kind are
/// there only while the rows disagree about them.
fn shelf_columns(rows: &[ShelfRow]) -> ShelfColumns {
    let varies = |read: fn(&ShelfRow) -> &str| {
        let mut values = rows.iter().map(read);
        let first = values.next();
        values.any(|value| Some(value) != first)
    };
    let class_varies = varies(|row| row.part.device.as_str());
    let kind_varies = varies(|row| kind_word(row.part.kind));
    let mut shared = Vec::new();
    if !class_varies && let Some(first) = rows.first() {
        shared.push(format!("all {}", first.part.device));
    }
    if !kind_varies && let Some(first) = rows.first() {
        shared.push(format!("all .{}", kind_word(first.part.kind)));
    }
    // The mockup's proportions, with the width of every collapsed column given
    // back to Description, which is the column a reader is actually reading.
    let optional = f32::from(u8::from(class_varies)).mul_add(0.10, 0.0)
        + f32::from(u8::from(kind_varies)).mul_add(0.08, 0.0);
    let mut columns = vec![
        ShelfColumn {
            heading: "Part",
            width: 0.14,
            mono: true,
            read: |row| row.part.part_id.clone(),
        },
        ShelfColumn {
            heading: "Description",
            width: 0.58 - optional,
            mono: false,
            read: |row| row.title.clone(),
        },
    ];
    if class_varies {
        columns.push(ShelfColumn {
            heading: "Class",
            width: 0.10,
            mono: true,
            read: |row| row.part.device.clone(),
        });
    }
    if kind_varies {
        columns.push(ShelfColumn {
            heading: "Kind",
            width: 0.08,
            mono: true,
            read: |row| kind_word(row.part.kind).to_owned(),
        });
    }
    columns.push(ShelfColumn {
        heading: "Library",
        width: 0.16,
        mono: true,
        read: |row| row.part.library.clone(),
    });
    columns.push(ShelfColumn {
        heading: "State",
        width: 0.12,
        mono: true,
        read: |_| String::new(),
    });
    ShelfColumns { columns, shared }
}

/// The one word a shelf row's state cell says, when it says anything.
///
/// "in project" is said at all because a part the project already holds looks
/// exactly like a compiled-in one, and a reader who cannot tell them apart
/// either adds the same source twice or believes a part they hold is still out
/// of reach. It changes nothing about placing it: a second instance of a part
/// already in the project is the ordinary case, not an error.
fn shelf_state(row: &ShelfRow) -> Option<&'static str> {
    (!row.part.built_in).then_some("in project")
}

pub(super) fn parts_catalog(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let facet = app.state.models_view.part_facet;
    let mut offset = app.state.models_view.part_catalog_offset;
    let (mut total, mut rows) = shelf_page(app, facet, offset);
    if total > 0 && offset >= total {
        offset = ((total - 1) / CATALOG_LIMIT) * CATALOG_LIMIT;
        app.state.models_view.part_catalog_offset = offset;
        (total, rows) = shelf_page(app, facet, offset);
    }
    // Nothing narrowed and nothing to show is a session with no parts loaded,
    // which is a different statement from a filter that matched nothing.
    if total == 0
        && facet == RSpicePartFacet::All
        && app.state.models_view.catalog_query.trim().is_empty()
    {
        page_empty_state(
            ui,
            "No addressable parts are loaded",
            "Add a model source to this project to browse and place parts here.",
        );
        return;
    }
    let layout = shelf_columns(&rows);
    let headings = layout
        .columns
        .iter()
        .map(|column| (column.heading, column.width))
        .collect::<Vec<_>>();
    // The mockup's `.model-browser` grid, which the shelf and the Models page
    // are both laid out on: the table keeps a little under half of what is
    // left once its own header and footer are paid for, and the part detail
    // takes the rest. Written as the same expression the Models page uses,
    // because two spellings of one grid is how the two pages came to split
    // their panes differently in the first place.
    let table_h =
        ((ui.available_height() - HEADER_H - CATALOG_FOOT_H) * CATALOG_TRACK_FRACTION).max(120.0);
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        table_header(ui, &headings);
        ScrollArea::vertical()
            .id_salt("models-parts-table")
            .auto_shrink([false, true])
            .max_height(table_h)
            .show(ui, |ui| {
                if rows.is_empty() {
                    empty_state(
                        ui,
                        "No addressable part matches the current search and class.",
                        "Private helper models declared inside macromodel bodies are intentionally excluded.",
                    );
                    if Button::new("Clear search").show(ui).clicked() {
                        app.state.models_view.catalog_query.clear();
                        app.state.models_view.part_facet = RSpicePartFacet::All;
                        app.state.models_view.part_catalog_offset = 0;
                        app.state.models_view.selected_part = None;
                    }
                }
                for row in &rows {
                    shelf_row(ui, app, row, &layout);
                }
            });
    });
    parts_catalog_footer(ui, app, rows.len(), total, &layout.shared);
    selected_part_detail(ui, app, &rows);
}

/// How many parts the shelf holds before a search or a class narrows it.
///
/// The scope chip states it, and it must agree with [`shelf_page`]'s
/// unfiltered total, which is what the test beside them asserts: a chip that
/// disagrees with the table under it is worse than no chip.
pub(super) fn part_count(manager: &ModelLibraryManager) -> usize {
    library_part_index(&manager.libraries_sorted()).len()
}

/// One page of the shelf in the current search and class, and the count of
/// every match behind it. Only the page is materialized as rows.
fn shelf_page(
    app: &ManagerRenderContext<'_>,
    facet: RSpicePartFacet,
    offset: usize,
) -> (usize, Vec<ShelfRow>) {
    let needle = app
        .state
        .models_view
        .catalog_query
        .trim()
        .to_ascii_lowercase();
    let devices = facet.device_filters();
    let libraries = app.model.model_library_manager.libraries_sorted();
    let mut total = 0_usize;
    let mut rows = Vec::new();
    for part in library_part_index(&libraries) {
        let Some(library) = libraries
            .iter()
            .find(|library| library.name == part.library)
        else {
            continue;
        };
        let title = library_title(library);
        if !admits(&part, title, &needle, devices) {
            continue;
        }
        if total >= offset && rows.len() < CATALOG_LIMIT {
            let card = library
                .models
                .get(&part.part_id)
                .or_else(|| library.top_level_models.get(&part.part_id));
            rows.push(ShelfRow {
                title: title.to_owned(),
                source: library.root_path.clone(),
                line: card.and_then(|card| card.source_line).unwrap_or(0),
                part,
            });
        }
        total = total.saturating_add(1);
    }
    (total, rows)
}

/// Whether a part answers the search and the class chips: what the part is
/// called, what class it is, what kind of definition it is, and which library
/// holds it.
fn admits(part: &LibraryPart, title: &str, needle: &str, devices: &[&str]) -> bool {
    if !devices.is_empty()
        && !devices
            .iter()
            .any(|device| part.device.eq_ignore_ascii_case(device))
    {
        return false;
    }
    needle.is_empty()
        || [
            part.part_id.as_str(),
            part.device.as_str(),
            kind_word(part.kind),
            part.library.as_str(),
            title,
        ]
        .iter()
        .any(|field| field.to_ascii_lowercase().contains(needle))
}

/// One shelf line, with the state cell painted when it has a word to say.
fn shelf_row(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    row: &ShelfRow,
    layout: &ShelfColumns,
) {
    let t = Tokens::get(ui.ctx());
    let key = part_key(row);
    let selected = app.state.models_view.selected_part.as_deref() == Some(key.as_str());
    let values = layout
        .columns
        .iter()
        .map(|column| (column.read)(row))
        .collect::<Vec<_>>();
    let cells = layout
        .columns
        .iter()
        .zip(&values)
        .map(|(column, value)| (value.as_str(), column.width, column.mono))
        .collect::<Vec<_>>();
    let response = selectable_data_row(ui, selected, &cells);
    let state = shelf_state(row);
    if let Some(phrase) = state {
        let start: f32 = layout
            .columns
            .iter()
            .take(layout.columns.len() - 1)
            .map(|column| column.width)
            .sum();
        ui.painter().text(
            egui::pos2(
                response.rect.left() + response.rect.width() * start + 5.0,
                response.rect.center().y,
            ),
            egui::Align2::LEFT_CENTER,
            phrase,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
    }
    // Painter text publishes no node, so the row's own node carries the part
    // and its state.
    let announcement = match state {
        Some(phrase) => format!("{} · {phrase}", row.part.part_id),
        None => row.part.part_id.clone(),
    };
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(announcement.clone());
    });
    if response.clicked() {
        app.state.models_view.selected_part = Some(key);
    }
    // A subcircuit the project holds can be given a symbol of its own.
    if !row.part.built_in && row.part.kind == LibraryPartKind::Subcircuit {
        response.context_menu(|ui| {
            if ui.button("Create symbol\u{2026}").clicked() {
                app.queue_subcircuit_symbol(&row.part.library, &row.part.part_id);
                ui.close();
            }
        });
    }
}

fn parts_catalog_footer(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    shown: usize,
    total: usize,
    shared: &[String],
) {
    let t = Tokens::get(ui.ctx());
    let offset = app.state.models_view.part_catalog_offset;
    let start = if shown == 0 { 0 } else { offset + 1 };
    let end = offset.saturating_add(shown).min(total);
    let page_count = total.div_ceil(CATALOG_LIMIT).max(1);
    let page = (offset / CATALOG_LIMIT).saturating_add(1).min(page_count);
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(12, 3))
        .show(ui, |ui| {
            // A fixed track, not `horizontal_centered`: that layout centres
            // against the whole *remaining* height of the page, so inside an
            // unconstrained frame the footer silently grew to fill everything
            // below the table and pushed the part detail six hundred pixels
            // off the bottom of the surface. The detail was in the access tree
            // the whole time, which is why nothing failed.
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), CATALOG_FOOT_H - 6.0),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.label(
                        RichText::new(format!(
                            "Showing {start}–{end} of {}",
                            crate::ui::accessibility::counted(
                                total,
                                "addressable part",
                                "addressable parts"
                            )
                        ))
                        .small()
                        .color(t.color.text_faint),
                    );
                    // The columns this page collapsed, said once. A reader who
                    // sees no CLASS column is owed the reason it is missing.
                    if !shared.is_empty() {
                        ui.label(
                            RichText::new(format!("· {}", shared.join(" · ")))
                                .small()
                                .monospace()
                                .color(t.color.text_faint),
                        );
                    }
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if Button::new("Next").enabled(end < total).show(ui).clicked() {
                            app.state.models_view.part_catalog_offset =
                                offset.saturating_add(CATALOG_LIMIT);
                            app.state.models_view.selected_part = None;
                        }
                        ui.label(
                            RichText::new(format!("Page {page} of {page_count}"))
                                .monospace()
                                .small()
                                .color(t.color.text_dim),
                        );
                        if Button::new("Previous")
                            .enabled(offset > 0)
                            .show(ui)
                            .clicked()
                        {
                            app.state.models_view.part_catalog_offset =
                                offset.saturating_sub(CATALOG_LIMIT);
                            app.state.models_view.selected_part = None;
                        }
                    });
                },
            );
        });
}

/// The selected part's identity, and the source it executes from.
fn selected_part_detail(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, rows: &[ShelfRow]) {
    let selected = app
        .state
        .models_view
        .selected_part
        .as_deref()
        .and_then(|key| rows.iter().find(|row| part_key(row) == key))
        .or_else(|| rows.first())
        .cloned();
    let Some(row) = selected else {
        return;
    };
    app.state.models_view.selected_part = Some(part_key(&row));
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        ui.add_space(12.0);
        ui.label(RichText::new(&row.part.part_id).monospace().strong());
        ui.label(format!(
            "{} · {} · {}",
            row.part.device,
            kind_word(row.part.kind),
            row.title
        ));
        place_action(ui, app, &row);
        // A part the project holds is already retained: the bytes are in the
        // project's closure. Saying so is what stops a reader hunting for an
        // "add" control that would do nothing.
        ui.label(
            RichText::new(if row.part.built_in {
                "Built in"
            } else {
                "In this project"
            })
            .small(),
        );
        // Refused rather than merely greyed. On a browser build the source is
        // never readable, so this control was permanently disabled with no
        // tooltip and no node description — visible, dead, and silent about
        // why. Nothing here gains a capability; the reason is stated.
        let refusal = card_refusal(cfg!(target_arch = "wasm32"), row.source.as_deref());
        let card_response = Button::new("Open card").enabled(refusal.is_none()).show(ui);
        if let Some(reason) = refusal {
            // The same route the disabled Place control takes: the reason
            // reaches a screen reader on the button's own node, because a
            // hover tooltip reaches nobody who is not holding a pointer.
            ui.ctx().accesskit_node_builder(card_response.id, |node| {
                node.set_description(reason);
            });
            card_response.on_disabled_hover_text(reason);
        } else if card_response.clicked()
            && let Some(source) = row.source.as_ref()
        {
            open_card(app, &row, source);
        }
    });
    // The mockup's `.model-detail-body.hub-shelf-detail`: identity in one
    // pane, what the part executes from in the other, both the same height and
    // both reaching the panel's bottom edge. A single shrink-to-fit card left
    // the page ending a quarter of a screen short of that edge, and dead space
    // under a table reads as a block that failed to render rather than as room.
    let region_h = ui.available_height().max(1.0);
    if ui.available_width() > 650.0 {
        ui.columns(2, |columns| {
            identity_pane(&mut columns[0], &row, region_h);
            source_pane(&mut columns[1], &row, region_h);
        });
    } else {
        // Below the mockup's container query the two panes stack rather than
        // squeezing two half-width columns into a width neither can be read
        // at. They take half the region each, so the stack still reaches the
        // bottom edge; the region scrolls only once half of it is shorter than
        // a pane's floor.
        let row_h = (region_h * 0.5).max(DETAIL_PANE_MIN_H);
        ScrollArea::vertical()
            .id_salt("models-shelf-detail")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                identity_pane(ui, &row, row_h);
                source_pane(ui, &row, row_h);
            });
    }
}

/// What the selected part is, and which library holds it.
fn identity_pane(ui: &mut Ui, row: &ShelfRow, height: f32) {
    filled_detail_pane(
        ui,
        "Identity",
        // The statement the part is declared with, spelled the way the page's
        // own footer spells it when a whole page shares one.
        Some(&format!(".{}", kind_word(row.part.kind))),
        height,
        "models-shelf-identity",
        |ui| {
            property(ui, "Name", &row.part.part_id, "catalog identity");
            property(ui, "Device class", &row.part.device, "standard class");
            property(ui, "Library", &row.title, &row.part.library);
        },
    );
}

/// What holds the selected part's definition, and whether a file backs it.
///
/// A missing path is two different facts on this shelf and only one of them
/// is a problem. A foundation card is written into the build and has no file
/// by construction; a library the project holds names a file, and only there
/// does the file being absent mean anything is wrong.
fn source_standing(row: &ShelfRow) -> (&'static str, Option<&'static str>) {
    if row.part.built_in {
        return (
            "compiled in",
            Some(
                "This card is written into the build rather than into a file, so there is nothing \
                 here to open, sync or re-read. Every machine running this version has the same \
                 one.",
            ),
        );
    }
    if row.source.as_ref().is_some_and(|path| path.is_file()) {
        ("on this machine", None)
    } else {
        (
            "not on disk",
            Some(
                "There is no readable card behind this row on this machine, so it cannot be \
                 opened until the source is rescanned.",
            ),
        )
    }
}

/// Where the selected part's definition executes from.
fn source_pane(ui: &mut Ui, row: &ShelfRow, height: f32) {
    let (standing, note) = source_standing(row);
    filled_detail_pane(
        ui,
        "Model source",
        Some(standing),
        height,
        "models-shelf-source",
        |ui| {
            let path = row.source.as_deref().map(path_label);
            property(
                ui,
                "Source",
                path.as_deref().unwrap_or(standing),
                match (path.is_some(), row.line) {
                    // A parsed card does not always carry the line it was read
                    // at, and "line 0" is a number nobody can look up.
                    (true, 0) => "line not recorded".to_owned(),
                    (true, line) => format!("line {line}"),
                    (false, _) => "no file this workspace reads".to_owned(),
                }
                .as_str(),
            );
            // Prose rather than a property row: it is the reason the pane says
            // so little, and a reason is a sentence.
            if let Some(note) = note {
                let t = Tokens::get(ui.ctx());
                ui.label(RichText::new(note).small().color(t.color.text_dim));
            }
        },
    );
}

/// Arm the schematic cursor with this part, or say why it cannot be.
///
/// The control is always drawn. A reader looking at a part they cannot place is
/// owed the reason on the part itself — a control that disappears leaves them
/// to guess whether the part is wrong, the project is wrong, or the button was
/// never there. The reason reaches a screen reader through the button's own
/// node, because a hover tooltip reaches nobody who is not holding a pointer.
fn place_action(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, row: &ShelfRow) {
    let offer = place::PlaceOffer::for_row(app.model, row);
    // The shelf's own primary: placing the selected part is what this table is
    // for, so it carries the accent the way each page's one primary does.
    let response = Button::new("Place")
        .accent()
        .enabled(offer.blocked.is_none())
        .show(ui);
    if let Some(reason) = offer.blocked {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_description(reason.clone());
        });
        response.on_disabled_hover_text(reason);
        return;
    }
    if response.clicked()
        && let Some(placement) = offer.placement
    {
        app.queue_placement(placement);
    }
}

/// Why the card behind a row cannot be opened here, if it cannot.
///
/// Opening a card reads a file, and a browser session has no filesystem to
/// read one out of. That used to render as a permanently greyed control with
/// no tooltip and no node description, so a reader could see the button and
/// never learn why it never worked.
fn card_refusal(browser: bool, source: Option<&Path>) -> Option<&'static str> {
    if browser {
        return Some(
            "Cards are opened from the source file on disk, and a browser session has no \
             filesystem to read one out of.",
        );
    }
    match source {
        Some(path) if path.is_file() => None,
        Some(_) => Some("The source this card names is not on this machine."),
        None => Some("This definition records no source file to open."),
    }
}

/// Open the bytes behind one part, read-only.
fn open_card(app: &mut ManagerRenderContext<'_>, row: &ShelfRow, source: &Path) {
    match std::fs::read_to_string(source) {
        Ok(body) => {
            app.state.models_view.dialog = Some(ModelsWorkbenchDialog::SourcePreview {
                title: row.part.part_id.clone(),
                subtitle: format!(
                    "{}:{} · read-only library source",
                    source.display(),
                    row.line
                ),
                source: body,
                editable: false,
            });
        }
        Err(error) => receipt(
            app,
            Err(format!("Could not open '{}': {error}", source.display())),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::state::model_library::{DeviceModel, ModelSubcircuitInterface, ModelType};

    /// One row of a library that holds `name`.
    fn row(name: &str, device: &str, kind: LibraryPartKind) -> ShelfRow {
        ShelfRow {
            part: LibraryPart {
                part_id: name.to_owned(),
                kind,
                device: device.to_owned(),
                library: "proving_parts".to_owned(),
                built_in: false,
            },
            title: "Proving parts".to_owned(),
            source: None,
            line: 1,
        }
    }

    /// A library of pinned bytes the project retained, holding one macromodel
    /// and one card.
    fn retained_library(name: &str) -> ModelLibrary {
        use sha2::{Digest as _, Sha256};

        let mut library = ModelLibrary::new(name);
        let root = std::path::PathBuf::from(format!("/retained/{name}.lib"));
        library.root_path = Some(root.clone());
        let digest =
            crate::product::ContentDigest::from_bytes(Sha256::digest(name.as_bytes()).into());
        library.source_authority = ModelSourceAuthority::RetainedImport {
            source_id: crate::product::ModelSourceId::new(),
            digest,
        };
        library.subcircuits.insert(
            "PROVING_DIV".to_owned(),
            ModelSubcircuitInterface {
                name: "PROVING_DIV".to_owned(),
                ports: vec!["IN".to_owned(), "OUT".to_owned(), "GND".to_owned()],
                parameter_defaults: BTreeMap::new(),
                description: None,
                file_path: Some(root),
                source_line: Some(2),
                section: None,
            },
        );
        let mut zener = DeviceModel::new("RSPICE_ZENER", ModelType::Diode);
        zener.spice_type = Some("D".to_owned());
        library.add_model(zener);
        library
    }

    /// The shelf page a session would paint, and the count behind it.
    fn page(state: &mut FeatureHarness<ModelsFeature>) -> (usize, Vec<ShelfRow>) {
        let mut pending = Vec::new();
        let context = ManagerRenderContext::over(state, &mut pending);
        shelf_page(&context, RSpicePartFacet::All, 0)
    }

    fn row_named<'a>(rows: &'a [ShelfRow], name: &str, library: &str) -> &'a ShelfRow {
        rows.iter()
            .find(|row| row.part.part_id == name && row.part.library == library)
            .unwrap_or_else(|| {
                panic!(
                    "no row for '{name}' in '{library}' among {:?}",
                    rows.iter()
                        .map(|row| (row.part.part_id.as_str(), row.part.library.as_str()))
                        .collect::<Vec<_>>()
                )
            })
    }

    /// Every button the shelf publishes, with where it landed.
    fn shelf_buttons(height: f32) -> Vec<(String, egui::accesskit::Rect)> {
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = harness();
        let mut pending = Vec::new();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1180.0, height),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut context = ManagerRenderContext::over(&mut state, &mut pending);
                    parts_catalog(ui, &mut context);
                });
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("the shelf publishes an access tree")
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Button)
            .filter_map(|(_, node)| Some((node.label()?.to_owned(), node.bounds()?)))
            .collect()
    }

    /// The part detail sits under the table, not off the bottom of the page.
    ///
    /// It landed six hundred pixels below the surface for as long as the shelf
    /// has existed, and every test passed the whole time: the detail is in the
    /// accessibility tree wherever it is put, so only its *position* could tell
    /// anyone. The cause was the footer's `horizontal_centered`, which centres
    /// against the remaining height of the page rather than against a track, so
    /// inside an unconstrained frame the footer grew to fill everything below
    /// the table and pushed the detail past the edge.
    #[test]
    fn the_part_detail_lands_under_the_table_rather_than_off_the_page() {
        for height in [700.0, 1000.0, 1600.0] {
            let buttons = shelf_buttons(height);
            let bounds = |label: &str| {
                buttons
                    .iter()
                    .find(|(found, _)| found == label)
                    .map(|(_, bounds)| *bounds)
                    .unwrap_or_else(|| panic!("{label} is reachable at {height}px"))
            };
            let footer = bounds("Next");
            let detail = bounds("Open card");
            assert!(
                detail.y0 >= footer.y1,
                "the detail starts above the footer at {height}px: {detail:?} vs {footer:?}"
            );
            assert!(
                detail.y0 - footer.y1 <= 24.0,
                "{:.0} px of nothing between the footer and the detail at {height}px",
                detail.y0 - footer.y1
            );
        }
    }

    /// A part with no file behind it is two facts, and one of them is a fault.
    ///
    /// The detail pane used to decide this with a single `is_file()`, which
    /// told a foundation card compiled into the binary that its definition had
    /// never been read — a sentence that is neither true nor actionable.
    #[test]
    fn a_row_with_no_file_says_which_reason_it_has_none() {
        let mut foundation = row("RSPICE_DIODE", "diode", LibraryPartKind::Model);
        foundation.part.built_in = true;
        let (standing, note) = source_standing(&foundation);
        assert_eq!(standing, "compiled in");
        assert!(
            note.expect("a card with no file says why")
                .contains("written into the build"),
            "the build is where it is, not a file nobody synced"
        );

        // The fault, and the only one that asks for anything.
        let mut absent = row("RSPICE_ZENER", "diode", LibraryPartKind::Model);
        absent.source = Some(std::path::PathBuf::from("/no/such/card.lib"));
        let (standing, note) = source_standing(&absent);
        assert_eq!(standing, "not on disk");
        assert!(note.is_some_and(|note| note.contains("rescanned")));

        // And a file that is actually there says nothing extra at all.
        let mut present = row("RSPICE_ZENER", "diode", LibraryPartKind::Model);
        present.source = Some(std::env::current_exe().expect("this test has a binary"));
        assert_eq!(source_standing(&present), ("on this machine", None));
    }

    /// A column every row agrees on is stated once, not repeated down the page.
    #[test]
    fn a_column_the_rows_agree_on_collapses_into_the_footer() {
        let mixed = [
            row("RSPICE_DIODE", "diode", LibraryPartKind::Model),
            row("RSPICE_OPAMP", "subckt", LibraryPartKind::Subcircuit),
        ];
        let layout = shelf_columns(&mixed);
        let headings = |layout: &ShelfColumns| {
            layout
                .columns
                .iter()
                .map(|column| column.heading)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            headings(&layout),
            ["Part", "Description", "Class", "Kind", "Library", "State"]
        );
        assert!(layout.shared.is_empty(), "nothing here is shared");

        let one_class = [
            row("RSPICE_DIODE", "diode", LibraryPartKind::Model),
            row("RSPICE_ZENER", "diode", LibraryPartKind::Model),
        ];
        let layout = shelf_columns(&one_class);
        assert_eq!(
            headings(&layout),
            ["Part", "Description", "Library", "State"],
            "a class every row shares is not a column"
        );
        assert_eq!(layout.shared, ["all diode", "all .model"]);
        // And the width the collapsed columns were using goes to the one a
        // reader is actually reading rather than leaving a gap.
        let widths: f32 = layout.columns.iter().map(|column| column.width).sum();
        assert!(
            (widths - 1.0).abs() < 1.0e-4,
            "the columns cover the row exactly: {widths}"
        );
    }

    /// A foundation card arms the device its own family is drawn as.
    ///
    /// The card is compiled into the binary, so nothing is retained or
    /// published: the cursor is armed directly, carrying the card's name,
    /// which is what every native emitter reads the model from.
    #[test]
    fn a_foundation_card_places_as_the_native_device_its_family_draws() {
        let mut state = harness();
        let (_, rows) = page(&mut state);
        let row = rows
            .iter()
            .find(|row| row.part.part_id == "RSPICE_DIODE")
            .expect("the foundation diode is on the shelf");
        assert_eq!(shelf_state(row), None, "a compiled-in card says nothing");

        let placement = place::plan(&state.model.model_library_manager, row)
            .expect("a foundation diode places");
        crate::workbench::app::arm_part_placement(&mut state.model, placement);
        assert_eq!(
            state.model.schematic.session.editor.tool,
            crate::state::Tool::Place(crate::state::ComponentType::Diode)
        );
        assert_eq!(
            state
                .model
                .schematic
                .session
                .editor
                .pending_part_model
                .as_ref()
                .map(|armed| armed.model.as_str()),
            Some("RSPICE_DIODE"),
            "the armed cursor carries the card it was chosen from"
        );
    }

    /// A macromodel the project retained arms a cell instance over its ports.
    ///
    /// Terminal order is netlist order, so the assertion is the order and not
    /// merely the set: a placement that reordered ports would connect the same
    /// symbol to different nodes.
    #[test]
    fn a_retained_macromodel_places_as_a_cell_instance_in_its_own_port_order() {
        let mut state = harness();
        let before = page(&mut state).0;
        state
            .model
            .model_library_manager
            .add_library(retained_library("proving_parts"));

        let (after, rows) = page(&mut state);
        assert_eq!(
            after,
            before + 2,
            "both definitions the retained library holds reached the shelf"
        );
        let row = row_named(&rows, "PROVING_DIV", "proving_parts");
        assert_eq!(
            shelf_state(row),
            Some("in project"),
            "a part the project holds says so, quietly"
        );

        let placement =
            place::plan(&state.model.model_library_manager, row).expect("a retained subckt places");
        crate::workbench::app::arm_part_placement(&mut state.model, placement);
        assert_eq!(
            state.model.schematic.session.editor.tool,
            crate::state::Tool::Place(crate::state::ComponentType::CellInstance)
        );
        let binding = state
            .model
            .schematic
            .session
            .editor
            .pending_library_cell
            .as_ref()
            .map(|pending| &pending.binding)
            .expect("the armed cursor carries the cell it was chosen from");
        assert_eq!(binding.library, "proving_parts");
        assert_eq!(binding.cell, "PROVING_DIV");
        assert_eq!(binding.terminal_order, ["IN", "OUT", "GND"]);
        assert_eq!(binding.reference_prefix.as_deref(), Some("X"));
    }

    /// Placement that cannot happen is refused in a sentence, on the control.
    #[test]
    fn an_unplaceable_part_disables_the_control_and_says_why() {
        let mut state = harness();
        state.model.workspace.ensure_active_buffer();

        // A card whose declared family no schematic device is drawn for.
        let mut library = ModelLibrary::new("opaque");
        library.source_authority = ModelSourceAuthority::External;
        let mut card = DeviceModel::new("VENDOR_PRIVATE", ModelType::Other);
        card.spice_type = Some("VENDOR_PRIVATE".to_owned());
        library.add_model(card);
        state.model.model_library_manager.add_library(library);

        let (_, rows) = page(&mut state);
        let opaque = row_named(&rows, "VENDOR_PRIVATE", "opaque");
        let refusal = place::PlaceOffer::for_row(&state.model, opaque)
            .blocked
            .expect("a part with no symbol is refused");
        assert!(
            refusal.contains("VENDOR_PRIVATE") && refusal.ends_with('.'),
            "the refusal names the part and reads as a sentence: {refusal}"
        );

        // And a part that *is* placeable is refused for the session instead,
        // naming the session's reason rather than the part's.
        let placeable = rows
            .iter()
            .find(|row| row.part.part_id == "RSPICE_DIODE")
            .expect("the foundation diode is on the shelf");
        assert!(
            place::PlaceOffer::for_row(&state.model, placeable)
                .blocked
                .is_none(),
            "a placeable part on an open sheet offers Place"
        );
        state.model.safe_mode.activate(
            crate::state::write_policy::LocalSafeModeOptions {
                open_project_read_only: true,
                ..Default::default()
            },
            "session".to_owned(),
        );
        let blocked = place::PlaceOffer::for_row(&state.model, placeable)
            .blocked
            .expect("a read-only project cannot receive an instance");
        assert!(
            blocked.contains("read-only"),
            "the session's own refusal is the one shown: {blocked}"
        );
    }

    /// The Place control is published, and carries its refusal when it has one.
    ///
    /// A tooltip reaches nobody who is not holding a pointer, and `ui.label`
    /// publishes no node at all, so the reason a control is dead has to travel
    /// on the control's own node or it does not travel.
    #[test]
    fn the_place_control_publishes_its_refusal_as_a_node_description() {
        fn place_description(read_only: bool) -> Option<String> {
            let ctx = egui::Context::default();
            ctx.enable_accesskit();
            crate::ui::Theme::default().apply(&ctx);
            let mut state = harness();
            if read_only {
                state.model.safe_mode.activate(
                    crate::state::write_policy::LocalSafeModeOptions {
                        open_project_read_only: true,
                        ..Default::default()
                    },
                    "session".to_owned(),
                );
            }
            let mut pending = Vec::new();
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1180.0, 1000.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let mut context = ManagerRenderContext::over(&mut state, &mut pending);
                        parts_catalog(ui, &mut context);
                    });
                },
            );
            let nodes = output
                .platform_output
                .accesskit_update
                .expect("the shelf publishes an access tree")
                .nodes;
            let place = nodes
                .iter()
                .find(|(_, node)| node.label() == Some("Place"))
                .map(|(_, node)| node)
                .expect("the shelf publishes a Place control");
            place.description().map(str::to_owned)
        }

        assert_eq!(
            place_description(false),
            None,
            "a live control explains nothing; it acts"
        );
        let refusal = place_description(true).expect("a refused control carries its reason");
        assert!(
            refusal.contains("read-only"),
            "the reason is the one that applies: {refusal}"
        );
    }

    /// The scope chip counts what the table under it holds.
    ///
    /// The chip is painted before the page is projected, so it counts by a
    /// second route — and a second route is a second answer unless something
    /// holds them together.
    #[test]
    fn the_scope_chip_and_the_shelf_agree_about_how_many_parts_there_are() {
        let mut state = harness();
        state
            .model
            .model_library_manager
            .add_library(retained_library("proving_parts"));

        let chip = part_count(&state.model.model_library_manager);
        assert_eq!(chip, page(&mut state).0);
        assert!(chip > 0, "the guard is measuring something");
    }

    /// A symbol view in front is a sheet a device cannot land on.
    #[test]
    fn a_view_that_is_not_a_schematic_refuses_placement_and_says_so() {
        let mut state = harness();
        state.model.workspace.activate_view(
            crate::state::CellViewRef::new("design", "top", "symbol"),
            crate::state::ViewType::Symbol,
        );
        assert!(state.model.workspace.active_schematic().is_none());

        let (_, rows) = page(&mut state);
        let row = rows
            .iter()
            .find(|row| row.part.part_id == "RSPICE_DIODE")
            .expect("the foundation diode is on the shelf");
        let refusal = place::PlaceOffer::for_row(&state.model, row)
            .blocked
            .expect("a symbol view cannot receive a device");
        assert!(
            refusal.contains("not a schematic"),
            "the refusal names the view, not the part: {refusal}"
        );
    }

    /// Opening a card is refused in a browser whatever the row records, and
    /// refused on the desktop for a reason that names which of the two it is.
    #[test]
    fn every_refusal_to_open_a_card_states_which_one_it_is() {
        let present = Path::new(file!());
        assert!(
            card_refusal(true, Some(present)).is_some(),
            "there is no filesystem to open it out of, whatever the row says"
        );
        assert_eq!(
            card_refusal(false, None),
            Some("This definition records no source file to open.")
        );
        assert_eq!(
            card_refusal(false, Some(Path::new("/no/such/model.lib"))),
            Some("The source this card names is not on this machine.")
        );
    }
}
