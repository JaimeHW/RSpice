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
//! # The spec seam
//!
//! A part chooser worth the name answers spec questions — "zeners between 5
//! and 6 volts" — and this client cannot yet, because the catalog schema the
//! shelf reads carries a part's identity, class and terminals and no
//! parameters. [`part_spec`] is the one place that will change when
//! `SnapshotPart.specs` is vendored: it is asked for a key and answers `None`
//! for every key today, and [`spec_columns`] keeps only keys that some row on
//! the page actually answered. So no column of em-dashes ships in the
//! meantime, and none has to be invented — the columns appear the day the
//! parts carry the values.

use super::*;

/// Spec keys each class facet would sort and filter on, once parts carry them.
///
/// Declared per class rather than globally because that is the whole point of
/// the shelf: the three numbers that pick a MOSFET are not the three that pick
/// an op-amp. Nothing here reaches a reader until [`part_spec`] can answer,
/// which is what keeps this a plan rather than a promise.
const CLASS_SPEC_KEYS: [(RSpicePartFacet, &[&str]); 7] = [
    (RSpicePartFacet::All, &[]),
    (RSpicePartFacet::Mosfet, &["VDS", "ID", "RDS(on)"]),
    (RSpicePartFacet::Bipolar, &["VCEO", "IC", "hFE"]),
    (RSpicePartFacet::Diode, &["VR", "IF", "VF"]),
    (RSpicePartFacet::JfetAndHemt, &["VGS", "IDSS", "gm"]),
    (RSpicePartFacet::Passive, &["value", "tolerance", "V"]),
    (RSpicePartFacet::IcAndMacro, &["VS", "GBW", "Iq"]),
];

/// One authored spec of one part.
///
/// # This is the seam
///
/// Schema-2 `SnapshotPart` carries `id`, `kind`, `device`, `aliases`,
/// `terminals` and `symbol`, and no parameters at all — so there is nothing on
/// this machine to answer with, and answering anyway would mean the shelf
/// asserting numbers no publisher ever signed. When the catalog carries specs,
/// this function reads them and everything above it — the columns, the sort,
/// the range filter — starts working without moving.
fn part_spec(hit: &PackModelHit, key: &str) -> Option<String> {
    let (_, _) = (hit, key);
    None
}

/// The spec columns a class earns: the declared keys some row answered.
fn spec_columns(hits: &[PackModelHit], facet: RSpicePartFacet) -> Vec<&'static str> {
    CLASS_SPEC_KEYS
        .iter()
        .find(|(candidate, _)| *candidate == facet)
        .map(|(_, keys)| *keys)
        .unwrap_or_default()
        .iter()
        .copied()
        .filter(|key| hits.iter().any(|hit| part_spec(hit, key).is_some()))
        .collect()
}

/// What one shelf column is, and how to read it off a row.
struct ShelfColumn {
    heading: &'static str,
    width: f32,
    mono: bool,
    read: fn(&PackModelHit) -> String,
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
fn shelf_columns(hits: &[PackModelHit], facet: RSpicePartFacet) -> ShelfColumns {
    let varies = |read: fn(&PackModelHit) -> &str| {
        let mut values = hits.iter().map(read);
        let first = values.next();
        values.any(|value| Some(value) != first)
    };
    let class_varies = varies(|hit| hit.device.as_str());
    let kind_varies = varies(|hit| hit.kind.as_str());
    let mut shared = Vec::new();
    if !class_varies && let Some(first) = hits.first() {
        shared.push(format!("all {}", first.device));
    }
    if !kind_varies && let Some(first) = hits.first() {
        shared.push(format!("all .{}", first.kind));
    }
    let specs = spec_columns(hits, facet);
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
            read: |hit| hit.name.clone(),
        },
        ShelfColumn {
            heading: "DESCRIPTION",
            width: 0.58 - optional,
            mono: false,
            read: |hit| hit.pack_name.clone(),
        },
    ];
    if class_varies {
        columns.push(ShelfColumn {
            heading: "CLASS",
            width: 0.10,
            mono: true,
            read: |hit| hit.device.clone(),
        });
    }
    if kind_varies {
        columns.push(ShelfColumn {
            heading: "KIND",
            width: 0.08,
            mono: true,
            read: |hit| hit.kind.clone(),
        });
    }
    // Spec columns render through the same `read` shape as everything else, so
    // the day `part_spec` answers, nothing about the table changes but which
    // columns it is handed.
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
        read: |hit| hit.pack.clone(),
    });
    columns.push(ShelfColumn {
        heading: "STATE",
        width: 0.12,
        mono: true,
        read: |_| String::new(),
    });
    ShelfColumns { columns, shared }
}

/// The one exception a shelf row may carry, and how loudly.
///
/// A part whose source is on this machine and licensed for a project says
/// nothing: that is the state most rows are in, and a column of "ok" chips
/// would bury the handful that are not.
fn shelf_state(hit: &PackModelHit) -> Option<(&'static str, bool)> {
    if hit.restricted {
        return Some(("restricted", true));
    }
    if !hit.redistributable {
        return Some(("license review", false));
    }
    if !hit.source.as_ref().is_some_and(|path| path.is_file()) {
        return Some(("sync required", false));
    }
    None
}

pub(super) fn parts_catalog(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    if app.state.model_library_manager.pack_definition_count() == 0 {
        page_empty_state(
            ui,
            "No addressable parts are installed",
            "Install the versioned model-pack corpus to browse licensed models and macromodel definitions.",
        );
        return;
    }
    let facet = app.state.workbench.models_view.part_facet;
    let pack_filter = app.state.workbench.models_view.selected_pack.clone();
    let mut offset = app.state.workbench.models_view.part_catalog_offset;
    let (mut total, mut hits) = browse(app, pack_filter.as_deref(), facet, offset);
    if total > 0 && offset >= total {
        offset = ((total - 1) / CATALOG_LIMIT) * CATALOG_LIMIT;
        app.state.workbench.models_view.part_catalog_offset = offset;
        (total, hits) = browse(app, pack_filter.as_deref(), facet, offset);
    }
    let mut layout = shelf_columns(&hits, facet);
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
    let table_h = (ui.available_height() * 0.40).max(120.0);
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        table_header(ui, &headings);
        ScrollArea::vertical()
            .id_salt("models-parts-table")
            .max_height(table_h)
            .show(ui, |ui| {
                if hits.is_empty() {
                    empty_state(
                        ui,
                        "No addressable part matches the current search and class.",
                        "Private helper models declared inside macromodel bodies are intentionally excluded.",
                    );
                    if ui.button("Clear search").clicked() {
                        app.state.workbench.models_view.catalog_query.clear();
                        app.state.workbench.models_view.part_facet = RSpicePartFacet::All;
                        app.state.workbench.models_view.selected_pack = None;
                        app.state.workbench.models_view.part_catalog_offset = 0;
                        app.state.workbench.models_view.selected_part = None;
                    }
                }
                for hit in &hits {
                    shelf_row(ui, app, hit, &layout);
                }
            });
    });
    parts_catalog_footer(ui, app, hits.len(), total, &layout.shared);
    selected_part_detail(ui, app, &hits);
}

/// One page of the corpus index, in the exact current query and class.
fn browse(
    app: &mut ManagerRenderContext<'_>,
    pack_filter: Option<&str>,
    facet: RSpicePartFacet,
    offset: usize,
) -> (usize, Vec<PackModelHit>) {
    app.state
        .model_library_manager
        .browse_pack_models(
            &app.state.workbench.models_view.catalog_query,
            pack_filter,
            facet.device_filters(),
            offset,
            CATALOG_LIMIT,
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
    hit: &PackModelHit,
    layout: &ShelfColumns,
) {
    let t = Tokens::get(ui.ctx());
    let key = part_key(hit);
    let selected = app.state.workbench.models_view.selected_part.as_deref() == Some(key.as_str());
    let values = layout
        .columns
        .iter()
        .map(|column| {
            // A spec column's heading *is* its key, which is what lets one
            // painter serve declared columns and read ones alike.
            part_spec(hit, column.heading).unwrap_or_else(|| (column.read)(hit))
        })
        .collect::<Vec<_>>();
    let cells = layout
        .columns
        .iter()
        .zip(&values)
        .map(|(column, value)| (value.as_str(), column.width, column.mono))
        .collect::<Vec<_>>();
    let response = selectable_data_row(ui, selected, &cells);
    let state = shelf_state(hit);
    if let Some((phrase, severe)) = state {
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
            if severe { t.color.err } else { t.color.warn },
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
                            "Showing {start}–{end} of {total} addressable parts"
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
                        if ui
                            .add_enabled(end < total, egui::Button::new("Next"))
                            .clicked()
                        {
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
                        if ui
                            .add_enabled(offset > 0, egui::Button::new("Previous"))
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
fn selected_part_detail(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hits: &[PackModelHit]) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_part
        .as_deref()
        .and_then(|key| hits.iter().find(|hit| part_key(hit) == key))
        .cloned()
        .or_else(|| hits.first().cloned());
    let Some(hit) = selected else {
        return;
    };
    app.state.workbench.models_view.selected_part = Some(part_key(&hit));
    let built_in = is_builtin_pack(app, &hit.pack);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        ui.add_space(12.0);
        ui.label(RichText::new(&hit.name).monospace().strong());
        ui.label(format!("{} · {} · {}", hit.device, hit.kind, hit.pack_name));
        if ui.button("Show pack").clicked() {
            app.state.workbench.models_view.catalog_scope = ModelsCatalogScope::InstalledPacks;
            app.state.workbench.models_view.selected_pack = Some(hit.pack.clone());
            app.state.workbench.models_view.catalog_query.clear();
        }
        if built_in {
            ui.label(RichText::new("Built in").small());
        } else if ui
            .add_enabled(
                hit.source.as_ref().is_some_and(|path| path.is_file())
                    && hit.redistributable
                    && !hit.restricted
                    && !app.state.workbench.models_view.model_import_in_progress,
                egui::Button::new("Add to project…"),
            )
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
        if ui.button("Open qualification").clicked() {
            app.state.workbench.models_page = ModelsPage::Qualification;
        }
        if ui
            .add_enabled(
                hit.source.as_ref().is_some_and(|path| path.is_file()),
                egui::Button::new("Open card"),
            )
            .clicked()
            && let Some(source) = hit.source.as_ref()
        {
            open_card(app, &hit, source);
        }
    });
    card(ui, |ui| {
        card_title(ui, "DEFINITION", Some(&hit.kind));
        property(ui, "Name", &hit.name, "catalog identity");
        property(ui, "Device class", &hit.device, "canonical");
        property(ui, "Pack", &hit.pack_name, &hit.pack);
        property(
            ui,
            "Source",
            hit.source
                .as_deref()
                .map(path_label)
                .as_deref()
                .unwrap_or("not on disk"),
            &format!("line {}", hit.line),
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
    });
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

    /// A column every row agrees on is stated once, not repeated down the page.
    #[test]
    fn a_column_the_rows_agree_on_collapses_into_the_footer() {
        let hit = |name: &str, device: &str, kind: &str| PackModelHit {
            name: name.to_owned(),
            kind: kind.to_owned(),
            device: device.to_owned(),
            pack: "rspice-foundation".to_owned(),
            pack_name: "RSpice foundation models".to_owned(),
            source: None,
            line: 1,
            redistributable: true,
            restricted: false,
        };

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

    /// The spec seam answers nothing, so no spec column ships.
    ///
    /// The catalog schema this client reads carries a part's identity, class
    /// and terminals and no parameters at all. Declaring the keys a class would
    /// sort on is a plan; painting a column of em-dashes for them would be a
    /// promise the data cannot keep, and inventing values would be worse.
    #[test]
    fn a_declared_spec_column_appears_only_once_a_part_answers_it() {
        let hits = [PackModelHit {
            name: "RSPICE_ZENER".to_owned(),
            kind: "model".to_owned(),
            device: "diode".to_owned(),
            pack: "rspice-foundation".to_owned(),
            pack_name: "RSpice foundation models".to_owned(),
            source: None,
            line: 1,
            redistributable: true,
            restricted: false,
        }];
        assert_eq!(
            CLASS_SPEC_KEYS
                .iter()
                .find(|(facet, _)| *facet == RSpicePartFacet::Diode)
                .map(|(_, keys)| *keys),
            Some(&["VR", "IF", "VF"][..]),
            "the keys a diode would be chosen by are declared"
        );
        assert!(
            part_spec(&hits[0], "VR").is_none(),
            "and nothing on this machine can answer them yet"
        );
        assert!(
            spec_columns(&hits, RSpicePartFacet::Diode).is_empty(),
            "so the shelf ships no column it would have to fill with dashes"
        );
    }
}
