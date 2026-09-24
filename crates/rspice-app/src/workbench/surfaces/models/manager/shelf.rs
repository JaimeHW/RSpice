//! The shelf: a dense part chooser, not a card wall.
//!
//! One row per addressable part, and the columns themselves change with what
//! the rows actually differ on — because the facts that pick a zener are not
//! the facts that pick an op-amp, and a column whose every cell says the same
//! word is a fact about the filter rather than about the rows.
//!
//! # A column earns its width
//!
//! [`shelf_columns`] is handed the page of rows and keeps only the columns
//! those rows disagree on. Narrowing to one device class collapses the class
//! column and hands its width to the ones that still distinguish parts; the
//! collapsed fact is stated once, in the footer, where it is true of the whole
//! page rather than repeated on every line.
//!
//! # The spec seam, now answered
//!
//! A part chooser worth the name answers spec questions — "zeners between 5
//! and 6 volts" — and until schema 2 this client could not, because the
//! catalog carried a part's identity, class and terminals and no parameters.
//! It carries up to six authored specifications now, so [`part_spec`] reads
//! them and the columns light for the classes whose parts answer.
//!
//! Nothing had to be invented to make that work, which was the point of the
//! seam: [`spec_columns`] keeps only the declared keys some row on the page
//! actually answered, so a class whose parts publish nothing still collapses
//! to no column at all rather than a column of em-dashes. A shipped-corpus row
//! is such a row — the index beside the binary predates the field and has no
//! specification to give — and the shelf says nothing about it rather than
//! guessing one from a model card it never parsed.
//!
//! # Two halves, one page
//!
//! The shelf's rows come from two places and say so on every line. The shipped
//! index is *streamed*, one bounded page at a time, because a development
//! corpus can hold two hundred thousand definitions; what this machine holds —
//! the project's own retained libraries, the foundation cards, an installed
//! release nobody has adopted yet — is projected by [`held_parts`] and cannot
//! be interleaved with a stream without materializing it. So the two are
//! concatenated: held first, because those are the parts a reader can place
//! this second, and they were the half the shelf could not see at all.
//!
//! # Why the columns do not sort
//!
//! Every other table in this workspace carries a sortable header, and this one
//! deliberately does not. Sorting a table means ordering the rows it is
//! *about*, and the rows this table is about are not in it: a page is at most
//! `CATALOG_LIMIT` of them, drawn from an index that can hold two hundred
//! thousand and that takes a query, a class filter, an offset and a limit —
//! and no ordering. A header cell here could only reorder the rows already
//! fetched, while looking exactly like a control that had ordered the catalog;
//! a reader clicking `VDS` would read the top row as the highest-rated part on
//! the shelf when it is the highest-rated part of one arbitrary page. That is
//! a worse answer than no control at all, and it is not a question about where
//! the sort state is kept — a durable field and a frame-scoped one would both
//! be wrong in the same way.
//!
//! The mockup's shelf does sort, and the difference is exactly this: it holds
//! every matching part in one array and orders all of them, having no index,
//! no stream and no pages. Its spec ordering is not a header click either — a
//! spec is a string carrying its own unit, so `"900 mV"` sorts above `"5.1 V"`
//! under any lexical comparison, and the mockup orders on a parsed magnitude
//! from a control in the bar over the table. Ordering this shelf honestly
//! needs an order the pack index itself applies, over every match rather than
//! over one page; that is a seam in the index, not a header cell, and it is
//! not something this alignment pass can invent.

use super::*;

use crate::state::model_hub::InstalledPack;
use held_parts::{HeldQuery, PartOrigin, ShelfRow};

/// Spec keys each class facet sorts and filters on.
///
/// Declared per class rather than globally because that is the whole point of
/// the shelf: the three numbers that pick a MOSFET are not the three that pick
/// an op-amp. A key here is a column this workspace is willing to show; a part
/// answering some other key publishes it for a reader who opens the part, not
/// for a column nobody asked for.
///
/// These are the datasheet abbreviations the published corpus signs its
/// specifications with, character for character. A key spelled any other way
/// here is a column that never lights, however many parts answer the quantity
/// it names, so the two canons move together or not at all.
const CLASS_SPEC_KEYS: [(RSpicePartFacet, &[&str]); 7] = [
    (RSpicePartFacet::All, &[]),
    (RSpicePartFacet::Mosfet, &["VDS", "ID", "RDS_on"]),
    (RSpicePartFacet::Bipolar, &["VCEO", "IC", "hFE"]),
    (RSpicePartFacet::Diode, &["VRRM", "IF_AV", "VF"]),
    (RSpicePartFacet::JfetAndHemt, &["VGS_off", "IDSS", "gfs"]),
    (RSpicePartFacet::Passive, &["value", "tolerance", "V"]),
    (RSpicePartFacet::IcAndMacro, &["Vos", "GBW", "Iq"]),
];

/// One authored spec of one part, exactly as its publisher signed it.
///
/// The value is a string carrying its own unit, because that is what the
/// format defines: the canonical encoding admits no floats, so a specification
/// is `"5.1 V"` and never a bare number this workspace would have to guess a
/// unit for. It is shown verbatim and never reformatted.
///
/// The key is matched without regard to ASCII case. A declared key and a
/// published one name the same specification whether the publisher wrote
/// `gbw` or `GBW`, and refusing to show a value over that would leave a column
/// empty for a reason no reader could see. Nothing else about the key is
/// normalized: two keys that differ by anything more are two specifications.
fn part_spec(row: &ShelfRow, key: &str) -> Option<String> {
    row.specs
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(key))
        .map(|(_, value)| value.clone())
}

/// The spec columns a class earns: the declared keys some row answered.
fn spec_columns(rows: &[ShelfRow], facet: RSpicePartFacet) -> Vec<&'static str> {
    CLASS_SPEC_KEYS
        .iter()
        .find(|(candidate, _)| *candidate == facet)
        .map(|(_, keys)| *keys)
        .unwrap_or_default()
        .iter()
        .copied()
        .filter(|key| rows.iter().any(|row| part_spec(row, key).is_some()))
        .collect()
}

/// What one shelf column is, and how to read it off a row.
///
/// `read` takes the whole row rather than its catalog hit: the description a
/// pack publishes per *part* lives on the row beside the hit, and a column that
/// could only see the hit had no way to prefer it over the pack's title.
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
/// Identity, pack and state are always present: they are the row's name, where
/// it came from, and whether anything is wrong with it. Class and kind are
/// there only while the rows disagree about them.
fn shelf_columns(rows: &[ShelfRow], facet: RSpicePartFacet) -> ShelfColumns {
    let varies = |read: fn(&ShelfRow) -> &str| {
        let mut values = rows.iter().map(read);
        let first = values.next();
        values.any(|value| Some(value) != first)
    };
    let class_varies = varies(|row| row.hit.device.as_str());
    let kind_varies = varies(|row| row.hit.kind.as_str());
    let mut shared = Vec::new();
    if !class_varies && let Some(first) = rows.first() {
        shared.push(format!("all {}", first.hit.device));
    }
    if !kind_varies && let Some(first) = rows.first() {
        shared.push(format!("all .{}", first.hit.kind));
    }
    let specs = spec_columns(rows, facet);
    // The mockup's proportions, with the width of every collapsed column given
    // back to Description, which is the column a reader is actually reading.
    let optional = f32::from(u8::from(class_varies)).mul_add(0.10, 0.0)
        + f32::from(u8::from(kind_varies)).mul_add(0.08, 0.0)
        + specs.len() as f32 * 0.10;
    let mut columns = vec![
        ShelfColumn {
            heading: "PART",
            width: 0.14,
            mono: true,
            read: |row| row.hit.name.clone(),
        },
        ShelfColumn {
            heading: "DESCRIPTION",
            width: 0.58 - optional,
            mono: false,
            // The part's own signed line where the publisher wrote one, and
            // the pack's title where they did not. Falling back rather than
            // leaving the cell empty keeps the column readable across a page
            // that mixes a pack's described parts with a corpus that has no
            // descriptions at all — and the pack title is a true statement
            // about the row either way, which is why it was the column before.
            read: |row| {
                row.description
                    .clone()
                    .unwrap_or_else(|| row.hit.pack_name.clone())
            },
        },
    ];
    if class_varies {
        columns.push(ShelfColumn {
            heading: "CLASS",
            width: 0.10,
            mono: true,
            read: |row| row.hit.device.clone(),
        });
    }
    if kind_varies {
        columns.push(ShelfColumn {
            heading: "KIND",
            width: 0.08,
            mono: true,
            read: |row| row.hit.kind.clone(),
        });
    }
    // Spec columns render through the same `read` shape as everything else, so
    // nothing about the table changes but which columns it is handed.
    columns.extend(specs.iter().map(|key| ShelfColumn {
        heading: key,
        width: 0.10,
        mono: true,
        // The heading is the key; the closure cannot capture it in a `fn`
        // pointer, so the value is read at paint time in `shelf_row`.
        read: |_| String::new(),
    }));
    columns.push(ShelfColumn {
        heading: "PACK",
        width: 0.16,
        mono: true,
        read: |row| row.hit.pack.clone(),
    });
    columns.push(ShelfColumn {
        heading: "STATE",
        width: 0.12,
        mono: true,
        read: |_| String::new(),
    });
    ShelfColumns { columns, shared }
}

/// How loudly a shelf row's one state word is said.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StateTone {
    /// A licence refusal: the part cannot be used at all.
    Severe,
    /// Something a reader has to resolve before the part is usable.
    Warn,
    /// A fact worth knowing that refuses nothing.
    Quiet,
}

/// The one exception a shelf row may carry, and how loudly.
///
/// A part whose source is on this machine and licensed for a project says
/// nothing: that is the state most rows are in, and a column of "ok" chips
/// would bury the handful that are not.
///
/// "in project" is the quietest of them and never outranks a refusal. It is
/// said at all because a part the project already holds looks exactly like one
/// it does not, and a reader who cannot tell them apart either adds the same
/// source twice or believes a part they hold is still out of reach. It changes
/// nothing about placing it: a second instance of a part already in the project
/// is the ordinary case, not an error.
///
/// The licence and sync words are facts about the *shipped corpus tree* — a
/// directory of source files with a per-file redistribution audit. What this
/// machine already holds went through retention or was compiled in, so those
/// questions were settled before the row existed and asking them again would
/// paint "sync required" across every part in the project.
fn shelf_state(row: &ShelfRow) -> Option<(&'static str, StateTone)> {
    if matches!(row.origin, PartOrigin::Corpus) {
        if row.hit.restricted {
            return Some(("restricted", StateTone::Severe));
        }
        if !row.hit.redistributable {
            return Some(("license review", StateTone::Warn));
        }
        if !row.hit.source.as_ref().is_some_and(|path| path.is_file()) {
            return Some(("sync required", StateTone::Warn));
        }
    }
    row.in_project.then_some(("in project", StateTone::Quiet))
}

pub(super) fn parts_catalog(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    installed: &[InstalledPack],
) {
    let facet = app.state.workbench.models_view.part_facet;
    let pack_filter = app.state.workbench.models_view.selected_pack.clone();
    let mut offset = app.state.workbench.models_view.part_catalog_offset;
    let (mut total, mut rows) = shelf_page(app, installed, pack_filter.as_deref(), facet, offset);
    if total > 0 && offset >= total {
        offset = ((total - 1) / CATALOG_LIMIT) * CATALOG_LIMIT;
        app.state.workbench.models_view.part_catalog_offset = offset;
        (total, rows) = shelf_page(app, installed, pack_filter.as_deref(), facet, offset);
    }
    // Nothing narrowed and nothing to show is a machine with no parts on it,
    // which is a different statement from a filter that matched nothing — and
    // it used to be decided by the shipped index alone, so a browser session
    // with a project full of retained cards was told it had none.
    if total == 0
        && pack_filter.is_none()
        && facet == RSpicePartFacet::All
        && app
            .state
            .workbench
            .models_view
            .catalog_query
            .trim()
            .is_empty()
    {
        page_empty_state(
            ui,
            "No addressable parts are installed",
            "Install a signed model pack from the hub, or add a model source to this project, to browse and place parts here.",
        );
        return;
    }
    let mut layout = shelf_columns(&rows, facet);
    // The pack filter arrives from another surface — "Browse parts" on a
    // corpus pack, "Show pack" on a shelf row, or a ledger selection carried
    // across the scope switch — so the shelf says which pack it is narrowed to
    // rather than leaving a reader to wonder where the other two thousand
    // parts went.
    if let Some(pack) = pack_filter.as_deref() {
        layout.shared.insert(0, format!("pack {pack}"));
    }
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
            .max_height(table_h)
            .show(ui, |ui| {
                if rows.is_empty() {
                    empty_state(
                        ui,
                        "No addressable part matches the current search and class.",
                        "Private helper models declared inside macromodel bodies are intentionally excluded.",
                    );
                    if Button::new("Clear search").show(ui).clicked() {
                        app.state.workbench.models_view.catalog_query.clear();
                        app.state.workbench.models_view.part_facet = RSpicePartFacet::All;
                        app.state.workbench.models_view.selected_pack = None;
                        app.state.workbench.models_view.part_catalog_offset = 0;
                        app.state.workbench.models_view.selected_part = None;
                    }
                }
                for row in &rows {
                    shelf_row(ui, app, row, &layout);
                }
            });
    });
    parts_catalog_footer(ui, app, rows.len(), total, &layout.shared);
    selected_part_detail(ui, app, installed, &rows);
}

/// One page of the shelf, across both halves, and the count behind it.
///
/// Held rows come first and the streamed index follows, so the page window is
/// split between them: an offset inside the held half takes what it can from
/// there and fills the rest from the start of the index, and an offset past it
/// asks the index for the remainder. The index reports its complete match count
/// whatever page size it is asked for, so a full page of held rows still knows
/// how many more there are behind it.
fn shelf_page(
    app: &mut ManagerRenderContext<'_>,
    installed: &[InstalledPack],
    pack_filter: Option<&str>,
    facet: RSpicePartFacet,
    offset: usize,
) -> (usize, Vec<ShelfRow>) {
    let needle = app
        .state
        .workbench
        .models_view
        .catalog_query
        .trim()
        .to_ascii_lowercase();
    let held = held_parts::held_parts(
        &app.state.model_library_manager,
        installed,
        &HeldQuery {
            needle: &needle,
            pack: pack_filter,
            devices: facet.device_filters(),
            offset,
            limit: CATALOG_LIMIT,
        },
    );
    let mut rows = held.page;
    let (corpus_total, hits) = browse(
        app,
        pack_filter,
        facet,
        offset.saturating_sub(held.total),
        CATALOG_LIMIT.saturating_sub(rows.len()),
    );
    // An indexed row whose definition this session has actually loaded is not
    // merely a file on disk: it is a part the project's catalog can execute,
    // and it says so and places like one.
    rows.extend(hits.into_iter().map(|hit| {
        let holder = held.holders.get(&hit.name);
        ShelfRow {
            origin: holder.map_or(PartOrigin::Corpus, held_parts::Holder::origin),
            in_project: holder.is_some_and(|holder| !holder.built_in),
            // The shipped index is generated from source files and carries no
            // authored presentation; see [`ShelfRow`].
            description: None,
            specs: std::collections::BTreeMap::new(),
            hit,
        }
    }));
    (held.total.saturating_add(corpus_total), rows)
}

/// One page of the corpus index, in the exact current query and class.
fn browse(
    app: &mut ManagerRenderContext<'_>,
    pack_filter: Option<&str>,
    facet: RSpicePartFacet,
    offset: usize,
    limit: usize,
) -> (usize, Vec<PackModelHit>) {
    app.state
        .model_library_manager
        .browse_pack_models(
            &app.state.workbench.models_view.catalog_query,
            pack_filter,
            facet.device_filters(),
            offset,
            limit,
        )
        .unwrap_or_else(|error| {
            receipt(app, Err(error));
            (0, Vec::new())
        })
}

/// One shelf line, with the state cell painted in the tone it earns.
fn shelf_row(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    row: &ShelfRow,
    layout: &ShelfColumns,
) {
    let t = Tokens::get(ui.ctx());
    let hit = &row.hit;
    let key = part_key(hit);
    let selected = app.state.workbench.models_view.selected_part.as_deref() == Some(key.as_str());
    let values = layout
        .columns
        .iter()
        .map(|column| {
            // A spec column's heading *is* its key, which is what lets one
            // painter serve declared columns and read ones alike.
            part_spec(row, column.heading).unwrap_or_else(|| (column.read)(row))
        })
        .collect::<Vec<_>>();
    let cells = layout
        .columns
        .iter()
        .zip(&values)
        .map(|(column, value)| (value.as_str(), column.width, column.mono))
        .collect::<Vec<_>>();
    let response = selectable_data_row(ui, selected, &cells);
    let state = shelf_state(row);
    if let Some((phrase, tone)) = state {
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
            match tone {
                StateTone::Severe => t.color.err,
                StateTone::Warn => t.color.warn,
                StateTone::Quiet => t.color.text_faint,
            },
        );
    }
    // Painter text publishes no node, so the row's own node carries the part
    // and whatever is wrong with it.
    let announcement = match state {
        Some((phrase, _)) => format!("{} · {phrase}", hit.name),
        None => hit.name.clone(),
    };
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(announcement.clone());
    });
    if response.clicked() {
        app.state.workbench.models_view.selected_part = Some(key);
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
    let offset = app.state.workbench.models_view.part_catalog_offset;
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
                            app.state.workbench.models_view.part_catalog_offset =
                                offset.saturating_add(CATALOG_LIMIT);
                            app.state.workbench.models_view.selected_part = None;
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
                            app.state.workbench.models_view.part_catalog_offset =
                                offset.saturating_sub(CATALOG_LIMIT);
                            app.state.workbench.models_view.selected_part = None;
                        }
                    });
                },
            );
        });
}

/// The selected part's identity, and the source it executes from.
fn selected_part_detail(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    installed: &[InstalledPack],
    rows: &[ShelfRow],
) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_part
        .as_deref()
        .and_then(|key| rows.iter().find(|row| part_key(&row.hit) == key))
        .or_else(|| rows.first())
        .cloned();
    let Some(row) = selected else {
        return;
    };
    let hit = row.hit.clone();
    app.state.workbench.models_view.selected_part = Some(part_key(&hit));
    let built_in =
        matches!(row.origin, PartOrigin::Foundation { .. }) || is_builtin_pack(app, &hit.pack);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        ui.add_space(12.0);
        ui.label(RichText::new(&hit.name).monospace().strong());
        ui.label(format!("{} · {} · {}", hit.device, hit.kind, hit.pack_name));
        place_action(ui, app, installed, &row);
        // The packs scope lists packs, so the button is offered only for a row
        // that belongs to one. A project library is not a pack, and sending a
        // reader to a table that cannot contain what they asked for is worse
        // than not offering the trip.
        if pack_scope_identity(app, &row).is_some() && Button::new("Show pack").show(ui).clicked() {
            app.state.workbench.models_view.catalog_scope = ModelsCatalogScope::InstalledPacks;
            app.state.workbench.models_view.selected_pack = pack_scope_identity(app, &row);
            app.state.workbench.models_view.catalog_query.clear();
        }
        if built_in {
            ui.label(RichText::new("Built in").small());
        } else if matches!(row.origin, PartOrigin::ProjectRetained { .. }) {
            // Already retained: the licence question was answered and the bytes
            // are in the project's closure. Saying so is what stops a reader
            // hunting for an "add" control that would do nothing.
            ui.label(RichText::new("In this project").small());
        } else if matches!(row.origin, PartOrigin::InstalledPack { .. }) {
            ui.label(RichText::new("Installed pack").small());
        } else if Button::new("Add to project…")
            .enabled(
                hit.source.as_ref().is_some_and(|path| path.is_file())
                    && hit.redistributable
                    && !hit.restricted
                    && !app.state.workbench.models_view.model_import_in_progress,
            )
            .show(ui)
            .on_disabled_hover_text(if hit.restricted || !hit.redistributable {
                "The source is not licensed for embedding in a project."
            } else {
                "The card is not present on disk; rescan or sync the corpus."
            })
            .clicked()
        {
            app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPart {
                pack_id: hit.pack.clone(),
                part_name: hit.name.clone(),
            });
        }
        if Button::new("Open qualification").show(ui).clicked() {
            app.state.workbench.models_page = ModelsPage::Qualification;
        }
        // Refused rather than merely greyed. On a browser build the source is
        // never readable, so this control was permanently disabled with no
        // tooltip and no node description — visible, dead, and silent about
        // why. Nothing here gains a capability; the reason is stated.
        let refusal = browser::current_host().card_refusal(hit.source.as_deref());
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
            && let Some(source) = hit.source.as_ref()
        {
            open_card(app, &hit, source);
        }
    });
    // The mockup's `.model-detail-body.hub-shelf-detail`: identity and specs
    // in one pane, what the part executes from in the other, both the same
    // height and both reaching the panel's bottom edge. A single shrink-to-fit
    // card left the page ending a quarter of a screen short of that edge, and
    // dead space under a table reads as a block that failed to render rather
    // than as room.
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

/// What the selected part is, and every specification its publisher signed.
fn identity_pane(ui: &mut Ui, row: &ShelfRow, height: f32) {
    let hit = &row.hit;
    filled_detail_pane(
        ui,
        "IDENTITY & SPECS",
        // The statement the part is declared with, spelled the way the page's
        // own footer spells it when a whole page shares one.
        Some(&format!(".{}", hit.kind)),
        height,
        "models-shelf-identity",
        |ui| {
            property(ui, "Name", &hit.name, "catalog identity");
            // Only where the publisher wrote one. A row with no description
            // shows no line rather than an em-dash: the pane is short enough
            // that an absent fact is legible by absence, and a placeholder
            // would say the publisher declined where in fact the format
            // predates the field.
            if let Some(description) = row.description.as_deref() {
                property(ui, "Summary", description, "as the publisher signed it");
            }
            property(ui, "Device class", &hit.device, "canonical");
            property(ui, "Pack", &hit.pack_name, &hit.pack);
            // Every specification the manifest publishes, not only the ones
            // this class declares a column for: the columns are what a table
            // has room to compare, and this pane is where a reader has come to
            // read one part.
            if row.specs.is_empty() {
                empty_state(
                    ui,
                    "No specification is published for this part.",
                    "The part is reachable by name, class and pack; no spec column or filter can \
                     see it.",
                );
            } else {
                for (key, value) in &row.specs {
                    property(ui, key, value, "published specification");
                }
            }
        },
    );
}

/// What holds the selected part's definition, and whether a file backs it.
///
/// A missing path is four different facts on this shelf and only one of them
/// is a problem. A foundation card is written into the build and has no file
/// by construction; an installed release keeps its parts inside the archive
/// the pack store proved, which is not a path this workspace reads; a retained
/// or corpus row names a file, and only there does the file being absent mean
/// anything is wrong. One `is_file()` test would have told all four the same
/// story, and the story would have been false for two of them.
fn source_standing(row: &ShelfRow) -> (&'static str, Option<&'static str>) {
    match row.origin {
        PartOrigin::Foundation { .. } => (
            "compiled in",
            Some(
                "This card is written into the build rather than into a file, so there is nothing \
                 here to open, sync or re-read. Every machine running this version has the same \
                 one.",
            ),
        ),
        PartOrigin::InstalledPack { .. } => (
            "in the release",
            Some(
                "The definition is inside the release archive this machine holds, in the pack \
                 store rather than in a file this workspace reads. Placing the part retains those \
                 bytes into the project first.",
            ),
        ),
        _ if row.hit.source.as_ref().is_some_and(|path| path.is_file()) => {
            ("on this machine", None)
        }
        _ => (
            "not on disk",
            Some(
                "There is no readable card behind this row on this machine, so it cannot be \
                 opened or added until the source is rescanned or the corpus synced.",
            ),
        ),
    }
}

/// Where the selected part's definition executes from, and on what terms.
fn source_pane(ui: &mut Ui, row: &ShelfRow, height: f32) {
    let hit = &row.hit;
    let (standing, note) = source_standing(row);
    filled_detail_pane(
        ui,
        "MODEL SOURCE",
        Some(standing),
        height,
        "models-shelf-source",
        |ui| {
            let path = hit.source.as_deref().map(path_label);
            property(
                ui,
                "Source",
                path.as_deref().unwrap_or(standing),
                match (path.is_some(), hit.line) {
                    // A parsed card does not always carry the line it was read
                    // at, and "line 0" is a number nobody can look up.
                    (true, 0) => "line not recorded".to_owned(),
                    (true, line) => format!("line {line}"),
                    (false, _) => "no file this workspace reads".to_owned(),
                }
                .as_str(),
            );
            property(
                ui,
                "Project eligibility",
                if hit.redistributable && !hit.restricted {
                    "eligible"
                } else {
                    "blocked"
                },
                "license policy",
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

/// The pack the packs scope could show this row under, when there is one.
///
/// A corpus row names its own pack. A pack row names the release it came from.
/// A library row names a pack only if it was retained from one, which is what
/// the pin records; a library imported from a file names nothing, and that is
/// the honest answer rather than its own name dressed as a pack id.
fn pack_scope_identity(app: &ManagerRenderContext<'_>, row: &ShelfRow) -> Option<String> {
    match &row.origin {
        PartOrigin::Corpus => Some(row.hit.pack.clone()),
        PartOrigin::InstalledPack { pack_id, .. } => Some(pack_id.clone()),
        PartOrigin::Foundation { library } | PartOrigin::ProjectRetained { library } => app
            .state
            .model_library_manager
            .get_library(library)
            .and_then(|library| library.pack_id.clone()),
    }
}

/// Arm the schematic cursor with this part, or say why it cannot be.
///
/// The control is always drawn. A reader looking at a part they cannot place is
/// owed the reason on the part itself — a control that disappears leaves them
/// to guess whether the part is wrong, the project is wrong, or the button was
/// never there. The reason reaches a screen reader through the button's own
/// node, because a hover tooltip reaches nobody who is not holding a pointer.
fn place_action(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    installed: &[InstalledPack],
    row: &ShelfRow,
) {
    let offer = place::PlaceOffer::for_row(app.state, installed, row);
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
    if !response.clicked() {
        return;
    }
    match offer.route {
        Some(place::PlaceRoute::Arm(placement)) => app.queue_placement(placement),
        Some(place::PlaceRoute::Retain {
            pack_id,
            version,
            part,
        }) => app.queue_model_hub(crate::workbench::app::ModelHubRequest::InstallPack {
            pack_id,
            version,
            part: Some(part),
        }),
        None => {}
    }
}

/// Open the exact installed bytes behind one part, read-only.
fn open_card(app: &mut ManagerRenderContext<'_>, hit: &PackModelHit, source: &Path) {
    match std::fs::read_to_string(source) {
        Ok(body) => {
            app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::SourcePreview {
                title: hit.name.clone(),
                subtitle: format!(
                    "{}:{} · read-only corpus source",
                    source.display(),
                    hit.line
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

    /// One row as the streamed index reports it.
    fn corpus_row(name: &str, device: &str, kind: &str) -> ShelfRow {
        ShelfRow {
            hit: PackModelHit {
                name: name.to_owned(),
                kind: kind.to_owned(),
                device: device.to_owned(),
                pack: "rspice-foundation".to_owned(),
                pack_name: "RSpice foundation models".to_owned(),
                source: None,
                line: 1,
                redistributable: true,
                restricted: false,
            },
            origin: PartOrigin::Corpus,
            in_project: false,
            description: None,
            specs: std::collections::BTreeMap::new(),
        }
    }

    /// One row as an installed signed pack reports it.
    fn pack_row(name: &str, device: &str, specs: &[(&str, &str)]) -> ShelfRow {
        ShelfRow {
            description: Some(format!("{name}, published by a signed pack")),
            specs: specs
                .iter()
                .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
                .collect(),
            origin: PartOrigin::InstalledPack {
                pack_id: "rspice-proving".to_owned(),
                version: "1.0.0".to_owned(),
            },
            ..corpus_row(name, device, "model")
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
    fn page(state: &mut AppState, installed: &[InstalledPack]) -> (usize, Vec<ShelfRow>) {
        let mut pending = Vec::new();
        let mut context = ManagerRenderContext {
            state,
            pending_actions: &mut pending,
        };
        shelf_page(&mut context, installed, None, RSpicePartFacet::All, 0)
    }

    fn row_named<'a>(rows: &'a [ShelfRow], name: &str, origin: &PartOrigin) -> &'a ShelfRow {
        rows.iter()
            .find(|row| row.hit.name == name && &row.origin == origin)
            .unwrap_or_else(|| {
                panic!(
                    "no {origin:?} row for '{name}' among {:?}",
                    rows.iter()
                        .map(|row| (row.hit.name.as_str(), &row.origin))
                        .collect::<Vec<_>>()
                )
            })
    }

    /// Every button the shelf publishes, with where it landed.
    fn shelf_buttons(height: f32) -> Vec<(String, egui::accesskit::Rect)> {
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = AppState::default();
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
                    let mut context = ManagerRenderContext {
                        state: &mut state,
                        pending_actions: &mut pending,
                    };
                    parts_catalog(ui, &mut context, &[]);
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
            let detail = bounds("Show pack");
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

    /// The part detail is one row of equal panes reaching the panel's edge.
    ///
    /// The surface's own guard against ending in unpainted canvas renders the
    /// Models page in its default scope, so it never saw this one: the shelf
    /// ended a quarter of a screen short of the bottom edge for as long as it
    /// has existed, with the shell's charcoal canvas showing under it, and
    /// every assertion about the shelf passed the whole time because a pane's
    /// *position* is the only thing that could tell anyone.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_part_detail_is_one_row_of_equal_panes_filling_the_page() {
        fn walk(shape: &egui::epaint::Shape, fill: egui::Color32, out: &mut Vec<egui::Rect>) {
            match shape {
                egui::epaint::Shape::Rect(rect) if rect.fill == fill => out.push(rect.rect),
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        walk(shape, fill, out);
                    }
                }
                _ => {}
            }
        }

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = AppState::default();
        let mut pending = Vec::new();
        let size = egui::vec2(1180.0, 900.0);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                        let mut context = ManagerRenderContext {
                            state: &mut state,
                            pending_actions: &mut pending,
                        };
                        parts_catalog(ui, &mut context, &[]);
                    });
            },
        );
        let panel = Tokens::get(&ctx).color.bg_panel;
        let mut painted = Vec::new();
        for clipped in &output.shapes {
            walk(&clipped.shape, panel, &mut painted);
        }
        // The panes are the only blocks that reach the bottom band, and the
        // fill is what a reader sees there: an unfilled band is the shell's
        // canvas, which reads as a page that failed to render.
        let mut panes = painted
            .into_iter()
            .filter(|rect| rect.bottom() >= size.y - 3.0 && rect.height() > 100.0)
            .collect::<Vec<_>>();
        panes.sort_by(|left, right| left.left().total_cmp(&right.left()));
        assert!(
            panes.len() >= 2,
            "the detail row reaches the bottom edge as two panes; found {:?}",
            panes
        );
        let (left, right) = (panes[0], panes[panes.len() - 1]);
        assert!(
            (left.height() - right.height()).abs() < 1.0,
            "the two panes are one row of equal heights: {left:?} vs {right:?}"
        );
        assert!(
            left.right() <= right.left() + 1.0,
            "and they sit beside each other rather than overlapping: {left:?} vs {right:?}"
        );
    }

    /// A part with no file behind it is four facts, and one of them is a fault.
    ///
    /// The detail pane used to decide this with a single `is_file()`, which
    /// told a foundation card compiled into the binary and an installed
    /// release proved in the pack store that their definitions had never been
    /// read — a sentence that is true of neither and actionable for neither.
    #[test]
    fn a_row_with_no_file_says_which_of_the_four_reasons_it_has_none() {
        let foundation = ShelfRow {
            origin: PartOrigin::Foundation {
                library: "rspice-foundation".to_owned(),
            },
            ..corpus_row("RSPICE_DIODE", "diode", "model")
        };
        let (standing, note) = source_standing(&foundation);
        assert_eq!(standing, "compiled in");
        assert!(
            note.expect("a card with no file says why")
                .contains("written into the build"),
            "the build is where it is, not a file nobody synced"
        );

        let (standing, note) = source_standing(&pack_row("1N4148", "diode", &[]));
        assert_eq!(standing, "in the release");
        assert!(note.is_some_and(|note| note.contains("pack store")));

        // The one fault of the four, and the only one that asks for anything.
        let mut absent = corpus_row("RSPICE_ZENER", "diode", "model");
        absent.hit.source = Some(std::path::PathBuf::from("/no/such/card.lib"));
        let (standing, note) = source_standing(&absent);
        assert_eq!(standing, "not on disk");
        assert!(note.is_some_and(|note| note.contains("rescanned")));

        // And a file that is actually there says nothing extra at all.
        let mut present = corpus_row("RSPICE_ZENER", "diode", "model");
        present.hit.source = Some(std::env::current_exe().expect("this test has a binary"));
        assert_eq!(source_standing(&present), ("on this machine", None));
    }

    /// A column every row agrees on is stated once, not repeated down the page.
    #[test]
    fn a_column_the_rows_agree_on_collapses_into_the_footer() {
        let hit = corpus_row;

        let mixed = [
            hit("RSPICE_DIODE", "diode", "model"),
            hit("RSPICE_OPAMP", "subckt", "subckt"),
        ];
        let layout = shelf_columns(&mixed, RSpicePartFacet::All);
        let headings = |layout: &ShelfColumns| {
            layout
                .columns
                .iter()
                .map(|column| column.heading)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            headings(&layout),
            ["PART", "DESCRIPTION", "CLASS", "KIND", "PACK", "STATE"]
        );
        assert!(layout.shared.is_empty(), "nothing here is shared");

        let one_class = [
            hit("RSPICE_DIODE", "diode", "model"),
            hit("RSPICE_ZENER", "diode", "model"),
        ];
        let layout = shelf_columns(&one_class, RSpicePartFacet::Diode);
        assert_eq!(
            headings(&layout),
            ["PART", "DESCRIPTION", "PACK", "STATE"],
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

    /// A declared column appears only once a part on the page answers it.
    ///
    /// Both directions, because either alone is half a rule. A shipped-corpus
    /// row has no signed presentation to give, so a page of them collapses
    /// every declared column rather than painting em-dashes; a page carrying a
    /// pack's parts earns exactly the columns those parts answered, and no
    /// column for the third key nobody published.
    #[test]
    fn a_declared_spec_column_appears_only_once_a_part_answers_it() {
        assert_eq!(
            CLASS_SPEC_KEYS
                .iter()
                .find(|(facet, _)| *facet == RSpicePartFacet::Diode)
                .map(|(_, keys)| *keys),
            Some(&["VRRM", "IF_AV", "VF"][..]),
            "the keys a diode is chosen by are declared"
        );

        let corpus = [corpus_row("RSPICE_ZENER", "diode", "model")];
        assert!(
            part_spec(&corpus[0], "VRRM").is_none(),
            "the shipped index carries no signed specification"
        );
        assert!(
            spec_columns(&corpus, RSpicePartFacet::Diode).is_empty(),
            "so that page ships no column it would have to fill with dashes"
        );

        let published = [
            pack_row("1N4148", "diode", &[("VRRM", "100 V"), ("IF_AV", "150 mA")]),
            corpus_row("RSPICE_ZENER", "diode", "model"),
        ];
        assert_eq!(
            spec_columns(&published, RSpicePartFacet::Diode),
            ["VRRM", "IF_AV"],
            "the two keys a part answered earn columns; VF, which none did, \
             does not"
        );
        assert_eq!(
            part_spec(&published[0], "VRRM").as_deref(),
            Some("100 V"),
            "and the value is shown exactly as the publisher signed it, unit \
             and all"
        );
        // A publisher's key spelling is not an identity the shelf enforces.
        assert_eq!(
            part_spec(&published[0], "vrrm").as_deref(),
            Some("100 V"),
            "a declared key matches the published one without regard to case"
        );
        assert!(
            part_spec(&published[1], "VRRM").is_none(),
            "and the corpus row beside it still answers nothing"
        );
    }

    /// The description column prefers the part's own signed line.
    ///
    /// A page mixing both halves is the case that matters: the pack's parts say
    /// what they are, and the corpus rows beside them keep saying which pack
    /// they came from rather than going blank.
    #[test]
    fn the_description_column_reads_the_part_then_falls_back_to_its_pack() {
        let rows = [
            pack_row("BZX84C5V1", "diode", &[("VZ", "5.1 V")]),
            corpus_row("RSPICE_ZENER", "diode", "model"),
        ];
        let layout = shelf_columns(&rows, RSpicePartFacet::Diode);
        let description = layout
            .columns
            .iter()
            .find(|column| column.heading == "DESCRIPTION")
            .expect("the description column is always present");
        assert_eq!(
            (description.read)(&rows[0]),
            "BZX84C5V1, published by a signed pack"
        );
        assert_eq!((description.read)(&rows[1]), "RSpice foundation models");
    }

    /// A foundation card arms the device its own family is drawn as.
    ///
    /// The card is compiled into the binary, so nothing is downloaded, retained
    /// or published: the cursor is armed directly, carrying the card's name,
    /// which is what every native emitter reads the model from.
    #[test]
    fn a_foundation_card_places_as_the_native_device_its_family_draws() {
        let mut state = AppState::default();
        let (_, rows) = page(&mut state, &[]);
        let row = rows
            .iter()
            .find(|row| row.hit.name == "RSPICE_DIODE")
            .expect("the foundation diode is on the shelf");

        let route = place::place_route(&state.model_library_manager, row)
            .expect("a foundation diode places");
        let place::PlaceRoute::Arm(placement) = route else {
            panic!("a card the build already holds is armed, not retained: {route:?}");
        };
        state.schematic.arm_pack_part(*placement);
        assert_eq!(
            state.schematic.tool,
            crate::state::Tool::Place(crate::state::ComponentType::Diode)
        );
        assert_eq!(
            state
                .schematic
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
        let mut state = AppState::default();
        state
            .model_library_manager
            .add_library(retained_library("proving_parts"));

        let (_, rows) = page(&mut state, &[]);
        let row = row_named(
            &rows,
            "PROVING_DIV",
            &PartOrigin::ProjectRetained {
                library: "proving_parts".to_owned(),
            },
        );
        assert_eq!(
            shelf_state(row),
            Some(("in project", StateTone::Quiet)),
            "a part the project holds says so, quietly"
        );

        let route = place::place_route(&state.model_library_manager, row)
            .expect("a retained subckt places");
        let place::PlaceRoute::Arm(placement) = route else {
            panic!("bytes already in the project are armed, not retained: {route:?}");
        };
        state.schematic.arm_pack_part(*placement);
        assert_eq!(
            state.schematic.tool,
            crate::state::Tool::Place(crate::state::ComponentType::CellInstance)
        );
        let binding = state
            .schematic
            .pending_library_cell
            .as_ref()
            .expect("the armed cursor carries the cell it was chosen from");
        assert_eq!(binding.library, "proving_parts");
        assert_eq!(binding.cell, "PROVING_DIV");
        assert_eq!(binding.terminal_order, ["IN", "OUT", "GND"]);
        assert_eq!(binding.reference_prefix.as_deref(), Some("X"));
    }

    /// An installed release the project has not adopted retains first.
    ///
    /// Placing one cannot arm anything yet: the schematic would hold an
    /// instance of a definition the project does not carry, and the project
    /// would stop opening anywhere the pack is not installed. So the shelf
    /// enqueues the same retention the pack confirmation raises, and the part
    /// becomes placeable the moment those bytes land — which is what the second
    /// half of this test walks through, using the very call that retention
    /// makes.
    #[test]
    fn an_installed_release_retains_its_part_before_the_cursor_can_be_armed() {
        use crate::state::model_hub::tests::{
            PACK_ID, PART_ID, StubTransport, VERSION, anchor_for, hub_signing_key, signed_archive,
            signed_snapshot,
        };
        use crate::state::model_hub::{MemoryModelHubStore, ModelHub};

        let key = hub_signing_key();
        let capabilities = ["subckt", "resistor"];
        let archive = signed_archive(&key, &capabilities);
        let snapshot = signed_snapshot(&key, &archive, &capabilities, VERSION);
        let transport = StubTransport::with_snapshot(snapshot).serving(VERSION, archive);
        let store = std::sync::Arc::new(MemoryModelHubStore::new());
        let mut hub =
            ModelHub::open(anchor_for(&key), Box::new(store), None).expect("a memory hub opens");
        hub.refresh_catalog(&transport).expect("catalog");
        hub.install(&transport, PACK_ID, VERSION).expect("install");

        let mut state = AppState::default();
        let (_, rows) = page(&mut state, hub.installed());
        let row = row_named(
            &rows,
            PART_ID,
            &PartOrigin::InstalledPack {
                pack_id: PACK_ID.to_owned(),
                version: VERSION.to_owned(),
            },
        );
        assert!(
            !row.in_project,
            "a release nobody adopted is not in the project"
        );
        assert_eq!(
            place::place_route(&state.model_library_manager, row)
                .expect("an installed part routes"),
            place::PlaceRoute::Retain {
                pack_id: PACK_ID.to_owned(),
                version: VERSION.to_owned(),
                part: PART_ID.to_owned(),
            },
            "the retention names the exact release the row came from"
        );

        // The retention itself — the one call `InstallPack { part }` makes.
        let library = hub
            .add_part_to_project(&mut state.model_library_manager, PACK_ID, VERSION, PART_ID)
            .expect("the proved bytes are retained");

        let (_, rows) = page(&mut state, hub.installed());
        let row = row_named(
            &rows,
            PART_ID,
            &PartOrigin::ProjectRetained {
                library: library.clone(),
            },
        );
        assert_eq!(shelf_state(row), Some(("in project", StateTone::Quiet)));
        assert_eq!(
            rows.iter()
                .filter(|candidate| candidate.hit.name == PART_ID)
                .count(),
            1,
            "the adopted part is one row, not one per place its bytes exist"
        );

        let place::PlaceRoute::Arm(placement) =
            place::place_route(&state.model_library_manager, row).expect("a retained part places")
        else {
            panic!("bytes already retained are armed, not retained again");
        };
        state.schematic.arm_pack_part(*placement);
        assert_eq!(
            state
                .schematic
                .pending_library_cell
                .as_ref()
                .map(|binding| binding.cell.as_str()),
            Some(PART_ID),
            "the armed cursor carries the part the release published"
        );
    }

    /// A part already in the project stays listed, and stays placeable.
    ///
    /// The shelf used to read one source of rows — the shipped index beside the
    /// binary — so a definition the project retained was nowhere on it, and a
    /// second instance of a part could not be placed from the workspace at all.
    /// Both halves are asserted here: the retained row is present, and the
    /// index's own row for the same name now says the project holds it and
    /// places through the library that holds the bytes.
    #[test]
    fn a_part_the_project_already_holds_is_listed_and_placeable_again() {
        let mut state = AppState::default();
        let before = page(&mut state, &[]).0;
        state
            .model_library_manager
            .add_library(retained_library("proving_parts"));
        let (after, rows) = page(&mut state, &[]);
        assert_eq!(
            after,
            before + 2,
            "both definitions the retained library holds reached the shelf"
        );

        let indexed = rows
            .iter()
            .find(|row| row.hit.name == "RSPICE_ZENER" && row.origin == PartOrigin::Corpus);
        // The shipped index is discovered beside the test binary, so its row for
        // this name is present here; a build without the tree has only the
        // retained row, which is the case this whole projection exists for.
        if let Some(indexed) = indexed {
            assert_eq!(
                shelf_state(indexed),
                Some(("in project", StateTone::Quiet)),
                "an indexed part the project adopted says so rather than looking unadded"
            );
            let route = place::place_route(&state.model_library_manager, indexed)
                .expect("an adopted corpus part places");
            assert!(
                matches!(route, place::PlaceRoute::Arm(_)),
                "it places through the library that holds its bytes: {route:?}"
            );
        }
    }

    /// Placement that cannot happen is refused in a sentence, on the control.
    #[test]
    fn an_unplaceable_part_disables_the_control_and_says_why() {
        let mut state = AppState::default();
        state.workspace.ensure_active_buffer();

        // A card whose declared family no schematic device is drawn for.
        let mut library = ModelLibrary::new("opaque");
        library.source_authority = ModelSourceAuthority::External;
        let mut card = DeviceModel::new("VENDOR_PRIVATE", ModelType::Other);
        card.spice_type = Some("VENDOR_PRIVATE".to_owned());
        library.add_model(card);
        state.model_library_manager.add_library(library);

        let (_, rows) = page(&mut state, &[]);
        let opaque = row_named(
            &rows,
            "VENDOR_PRIVATE",
            &PartOrigin::ProjectRetained {
                library: "opaque".to_owned(),
            },
        );
        let refusal = place::PlaceOffer::for_row(&state, &[], opaque)
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
            .find(|row| row.hit.name == "RSPICE_DIODE")
            .expect("the foundation diode is on the shelf");
        assert!(
            place::PlaceOffer::for_row(&state, &[], placeable)
                .blocked
                .is_none(),
            "a placeable part on an open sheet offers Place"
        );
        state.workbench.safe_mode.activate(
            crate::workbench::state::LocalSafeModeOptions {
                open_project_read_only: true,
                ..Default::default()
            },
            "session".to_owned(),
        );
        let blocked = place::PlaceOffer::for_row(&state, &[], placeable)
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
            let mut state = AppState::default();
            if read_only {
                state.workbench.safe_mode.activate(
                    crate::workbench::state::LocalSafeModeOptions {
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
                        let mut context = ManagerRenderContext {
                            state: &mut state,
                            pending_actions: &mut pending,
                        };
                        parts_catalog(ui, &mut context, &[]);
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
    /// holds them together. This is that something: the chip said "17" over a
    /// table of nineteen rows the moment the shelf gained its second half.
    #[test]
    fn the_scope_chip_and_the_shelf_agree_about_how_many_parts_there_are() {
        let mut state = AppState::default();
        state
            .model_library_manager
            .add_library(retained_library("proving_parts"));

        let chip = state.model_library_manager.pack_definition_count()
            + held_parts::held_count(&state.model_library_manager, &[]);
        assert_eq!(chip, page(&mut state, &[]).0);
        assert!(chip > 0, "the guard is measuring something");
    }

    /// A symbol view in front is a sheet a device cannot land on.
    #[test]
    fn a_view_that_is_not_a_schematic_refuses_placement_and_says_so() {
        let mut state = AppState::default();
        state.workspace.activate_view(
            crate::state::CellViewRef::new("design", "top", "symbol"),
            crate::state::ViewType::Symbol,
        );
        assert!(state.workspace.active_schematic().is_none());

        let (_, rows) = page(&mut state, &[]);
        let row = rows
            .iter()
            .find(|row| row.hit.name == "RSPICE_DIODE")
            .expect("the foundation diode is on the shelf");
        let refusal = place::PlaceOffer::for_row(&state, &[], row)
            .blocked
            .expect("a symbol view cannot receive a device");
        assert!(
            refusal.contains("not a schematic"),
            "the refusal names the view, not the part: {refusal}"
        );
    }
}
