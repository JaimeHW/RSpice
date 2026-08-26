//! Specialist Models & PDKs page: corners.

use std::sync::Arc;

use super::*;

use crate::state::model_library::RetainedClosure;

pub(super) fn corners_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let rows = corner_rows(app);
    let unresolved = rows.iter().filter(|row| !row.resolved()).count();
    section_title(
        ui,
        "Corners & sections",
        &matrix_subtitle(&rows, unresolved),
        // Outermost-right first: the band lays its actions out right to left.
        |ui| {
            if Button::new("Validate bindings").accent().show(ui).clicked() {
                let result = validate_current_model_execution_plan(app, unresolved);
                receipt(app, result);
            }
            if Button::new("Add corner…").show(ui).clicked() {
                if let Some(library) = app
                    .state
                    .model_library_manager
                    .selected_library
                    .clone()
                    .or_else(|| {
                        app.state
                            .model_library_manager
                            .libraries_sorted()
                            .first()
                            .map(|library| library.name.clone())
                    })
                {
                    app.state.workbench.models_view.dialog =
                        Some(ModelsWorkbenchDialog::AddCorner {
                            library,
                            name: String::new(),
                            temperature_c: "27".to_owned(),
                            supply_factor: "1.0".to_owned(),
                        });
                } else {
                    receipt(
                        app,
                        Err("Attach a model library before adding a corner.".to_owned()),
                    );
                }
            }
            if Button::new("Import section map")
                .enabled(!app.state.workbench.models_view.model_import_in_progress)
                .show(ui)
                .on_hover_text(
                    "Import an authenticated SPICE model source whose .lib sections define the map.",
                )
                .clicked()
            {
                app.queue_model_source_import();
            }
        },
    );
    if rows.is_empty() {
        page_empty_state(
            ui,
            "No corner bindings are loaded",
            "Import a PDK section map or attach a sectioned model library to publish executable corner bindings.",
        );
        return;
    }
    temperature_validity_findings(ui, &rows);
    // Four bands share the body, so each is budgeted against the room actually
    // left rather than against a constant: the page carries no scroll of its
    // own, and a band that took a fixed height pushed the pane below it off the
    // bottom edge at short viewports.
    let body = ui.available_height().max(1.0);
    corner_matrix(ui, app, &rows, (body * 0.20).clamp(72.0, 170.0));
    fail_closed_note(ui, unresolved);
    // Until a reader picks a row, the page opens on the corner the library
    // actually executes — not on whichever name sorts first. Everything below
    // this line describes one corner, and describing an arbitrary one while
    // the matrix marks a different one "active reference" is two answers to
    // the same question.
    let selected = app
        .state
        .workbench
        .models_view
        .selected_corner
        .as_deref()
        .and_then(|key| rows.iter().find(|row| row.key == key))
        .or_else(|| rows.iter().find(|row| row.active))
        .or_else(|| rows.iter().find(|row| row.corner.is_default))
        .or_else(|| rows.first())
        .cloned();
    let Some(row) = selected else {
        return;
    };
    app.state.workbench.models_view.selected_corner = Some(row.key.clone());
    section_source_pane(ui, app, &row, (body * 0.24).clamp(45.0, 200.0));
    statistical_and_aging_card(ui, app, &row);
    corner_detail(ui, app, &row);
}

/// What the page bar says about the binding set, in the mockup's own order:
/// how many corners resolve, the one file their sections live in, its
/// authenticated identity, and what is still held out of every run.
///
/// The "fail closed" clause this replaces now sits under the matrix, where it
/// is next to the rows it is a contract about; stating it in both places said
/// it twice and explained it in neither.
fn matrix_subtitle(rows: &[CornerRow], unresolved: usize) -> String {
    let mut subtitle = format!(
        "{} of {} corners bound",
        rows.len() - unresolved,
        rows.len()
    );
    if let Some(source) = rows.iter().find_map(|row| row.source.as_deref()) {
        subtitle.push_str(" · ");
        subtitle.push_str(source);
    }
    if let Some(digest) = rows.iter().find_map(|row| row.source_digest.as_deref()) {
        subtitle.push_str(" · digest ");
        subtitle.push_str(digest);
    }
    if unresolved > 0 {
        subtitle.push_str(&format!(" · {unresolved} unresolved"));
    }
    subtitle
}

/// The matrix's columns, shared by its header and its rows so a cell can never
/// land under the wrong heading.
///
/// `MACRO` is the mockup's own column and a per-corner fact like the rest: a
/// PDK that publishes macromodels as their own section lets a corner bind it
/// independently of the device sections beside it.
const MATRIX_COLUMNS: [(&str, f32); 9] = [
    ("CORNER", 0.11),
    ("MOS", 0.11),
    ("BJT", 0.10),
    ("PASSIVES", 0.11),
    ("MACRO", 0.10),
    ("STATISTICAL", 0.12),
    ("AGING", 0.09),
    ("TEMP °C", 0.10),
    ("STATUS", 0.16),
];

fn corner_matrix(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    rows: &[CornerRow],
    table_h: f32,
) {
    let selected_key = app
        .state
        .workbench
        .models_view
        .selected_corner
        .clone()
        .unwrap_or_default();
    // Both actions are settled after the table so the row loop borrows nothing
    // it would have to give back mid-frame.
    let mut select = None;
    let mut bind = None;
    card(ui, |ui| {
        table_header(ui, &MATRIX_COLUMNS);
        ScrollArea::vertical()
            .id_salt("models-corner-matrix")
            .max_height(table_h)
            .show(ui, |ui| {
                for row in rows {
                    let response = selectable_data_row(
                        ui,
                        selected_key == row.key,
                        &[
                            (&row.corner.name.to_uppercase(), MATRIX_COLUMNS[0].1, true),
                            // The MOS *section*, like every other domain
                            // column. This cell used to paint the corner's
                            // nmos/pmos axis labels — "typical/typical" on
                            // every corner of a conventional PDK — so the one
                            // column a reader looks at first was the one that
                            // did not answer the matrix's question, and the
                            // axes it did answer are stated in the details
                            // pane where the rest of the corner's metadata is.
                            (
                                &domain_cell(&row.corner, CornerSectionDomain::Mos),
                                MATRIX_COLUMNS[1].1,
                                true,
                            ),
                            (
                                &domain_cell(&row.corner, CornerSectionDomain::Bjt),
                                MATRIX_COLUMNS[2].1,
                                true,
                            ),
                            (
                                &domain_cell(&row.corner, CornerSectionDomain::Passives),
                                MATRIX_COLUMNS[3].1,
                                true,
                            ),
                            (
                                &domain_cell(&row.corner, CornerSectionDomain::MacroModels),
                                MATRIX_COLUMNS[4].1,
                                true,
                            ),
                            (&statistical_cell(&row.corner), MATRIX_COLUMNS[5].1, true),
                            (
                                &domain_cell(&row.corner, CornerSectionDomain::Aging),
                                MATRIX_COLUMNS[6].1,
                                true,
                            ),
                            (
                                &engineering_value(row.corner.temperature),
                                MATRIX_COLUMNS[7].1,
                                true,
                            ),
                            (row.status_label(), MATRIX_COLUMNS[8].1, true),
                        ],
                    );
                    if row.binding_blocked && row_bind_action(ui, response.rect, row) {
                        bind = Some(row.clone());
                    } else if response.clicked() {
                        select = Some(row.clone());
                    }
                }
            });
    });
    if let Some(row) = bind {
        open_corner_binding_dialog(app, &row);
    } else if let Some(row) = select {
        inspect_corner(app, &row);
    }
}

/// The row's own "Bind section…", drawn inside the matrix the mockup puts it
/// in rather than only in the action row below.
///
/// The rows are painted, not built out of widgets — that is what keeps a
/// matrix of a hundred corners affordable — so the control is a hit-tested
/// sub-rect of the row that publishes its own button node, the same technique
/// `paint.rs` uses for a sortable header cell. Painted glyphs alone would be a
/// control only a sighted reader could find.
fn row_bind_action(ui: &mut Ui, row: egui::Rect, corner: &CornerRow) -> bool {
    /// The literal the action row uses, so a reader meets one spelling of this
    /// action wherever they reach it.
    const LABEL: &str = "Bind section…";
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_MICRO, FontWeight::Regular);
    let width = ui
        .painter()
        .layout_no_wrap(LABEL.to_owned(), font.clone(), t.color.text)
        .size()
        .x
        + 16.0;
    let rect = egui::Rect::from_min_max(
        egui::pos2(row.right() - width - 5.0, row.center().y - 9.0),
        egui::pos2(row.right() - 5.0, row.center().y + 9.0),
    );
    // A pane too narrow to hold the control leaves the row alone rather than
    // painting the action over the status it would cover; the action row below
    // still carries it.
    if rect.left() < row.left() + row.width() * 0.5 {
        return false;
    }
    let response = ui.interact(
        rect,
        ui.make_persistent_id(("models-corner-row-bind", &corner.key)),
        Sense::click(),
    );
    let announced = format!("Bind section for corner {}", corner.corner.name);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), announced.clone())
    });
    let hovered = response.hovered();
    ui.painter().rect(
        rect,
        t.radius,
        if hovered {
            t.color.bg_hover
        } else {
            t.color.bg_panel_2
        },
        Stroke::new(
            1.0,
            if hovered {
                t.color.border_strong
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        LABEL,
        font,
        t.color.text,
    );
    if hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.clicked()
}

/// The contract the matrix enforces, stated once underneath it.
///
/// It is a property of every row rather than of the selected one, which is why
/// it sits under the table and not in the detail pane: a reader deciding
/// whether an unresolved corner still runs needs the answer where the
/// unresolved corner is.
fn fail_closed_note(ui: &mut Ui, unresolved: usize) {
    let t = Tokens::get(ui.ctx());
    let mut prose = "A corner with any unbound section can never expand into tasks — there is \
                     no implicit typical fallback and no silent alias resolution."
        .to_owned();
    if unresolved > 0 {
        prose.push_str(
            " The unresolved corners above are held out of every run until they bind one.",
        );
    }
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(12, 6))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.label(
                    RichText::new("fail closed")
                        .small()
                        .strong()
                        .color(t.color.err),
                );
                super::hub::announced(
                    ui,
                    RichText::new(&prose).small().color(t.color.text_dim),
                    &prose,
                );
            });
        });
}

/// Lines of the bound section this pane will ever hold.
///
/// A cap rather than a scroll: a PDK's corner file is megabytes, and a pane
/// that loaded it to show a reader the top of one section would pay for the
/// whole file on every frame it was open. The header's Open action is the
/// route to the rest, and the footer says how much of it there is.
const SECTION_EXCERPT_LINES: usize = 16;

/// What a cached excerpt was sliced for.
///
/// Cheap to build on every frame, which is what a cache key has to be: four
/// short strings the row already carries, and no walk of the corpus to decide
/// whether the slice beside it is still good. The digest is in it because a
/// refreshed import replaces the bytes under an unchanged path and name.
#[derive(Clone, PartialEq, Eq)]
struct SectionExcerptKey {
    library: String,
    corner: String,
    section: String,
    digest: String,
}

/// The bound section's own lines, sliced out of the retained bytes once.
#[derive(Clone)]
struct SectionExcerpt {
    file: String,
    section: String,
    /// 1-based line the section opens on, so the gutter says where in the file
    /// this is instead of counting from one.
    first_line: usize,
    /// At most [`SECTION_EXCERPT_LINES`] of them.
    lines: Vec<String>,
    /// Lines the whole section holds, painted or not.
    total_lines: usize,
}

#[derive(Clone)]
struct SectionExcerptCache {
    key: SectionExcerptKey,
    /// Behind an `Arc` because a hit clones this on every frame, and a cache
    /// whose hit costs sixteen string allocations is not one.
    excerpt: Arc<Result<SectionExcerpt, String>>,
}

/// The section this pane shows when a corner binds several.
///
/// A composite `.lib TT` *is* the corner, so it wins outright; otherwise the
/// first declared binding is shown and every other one is named in the details
/// pane below. Picking one is honest as long as the header says which.
fn displayed_section(corner: &ProcessCorner) -> Option<CornerSectionBinding> {
    let bindings = corner.effective_section_bindings();
    bindings
        .iter()
        .find(|binding| binding.domain == CornerSectionDomain::Composite)
        .or_else(|| bindings.first())
        .cloned()
}

/// The `.lib <name>` … `.endl` pair a corner binds, read out of retained bytes.
///
/// Tolerant on purpose, because the file is whatever a foundry shipped: SPICE
/// is case-insensitive, the directive may be indented or tab-separated, and
/// `.endl` may or may not repeat the section's name. A three-token
/// `.lib <file> <name>` is an *include* of a section defined elsewhere rather
/// than a definition, so it never opens one here.
///
/// Returns `None` when no such pair is in the file, which the caller states
/// rather than showing an empty pane.
fn slice_section(bytes: &[u8], section: &str) -> Option<SectionExcerpt> {
    let mut lines = Vec::new();
    let mut first_line = 0;
    let mut total_lines = 0;
    for (index, raw) in bytes.split(|byte| *byte == b'\n').enumerate() {
        let text = String::from_utf8_lossy(raw);
        if first_line == 0 {
            if !section_definition(&text).is_some_and(|name| name.eq_ignore_ascii_case(section)) {
                continue;
            }
            first_line = index + 1;
        }
        total_lines += 1;
        if lines.len() < SECTION_EXCERPT_LINES {
            // Trailing whitespace and the CR of a CRLF file go; leading
            // indentation stays, because a nested `.param` block is unreadable
            // flattened against the margin.
            lines.push(text.trim_end().to_owned());
        }
        if ends_section(&text) {
            break;
        }
    }
    (first_line > 0).then_some(SectionExcerpt {
        file: String::new(),
        section: section.to_owned(),
        first_line,
        lines,
        total_lines,
    })
}

/// The name a `.lib` line *defines*, if it defines one.
fn section_definition(line: &str) -> Option<&str> {
    let mut tokens = line.split_whitespace();
    if !tokens.next()?.eq_ignore_ascii_case(".lib") {
        return None;
    }
    let name = tokens.next()?;
    tokens.next().is_none().then_some(name)
}

/// Whether this line closes the section it is inside.
fn ends_section(line: &str) -> bool {
    line.split_whitespace()
        .next()
        .is_some_and(|directive| directive.eq_ignore_ascii_case(".endl"))
}

/// The excerpt for the selected corner, sliced once per selection.
///
/// Keyed like the bin page's engine inspection next door: an id-keyed
/// `ctx.data` entry whose key is cheaper than the miss it prevents. The miss
/// walks the retained bytes once; the hit clones one `Arc`.
fn bound_section_excerpt(
    ui: &Ui,
    app: &ManagerRenderContext<'_>,
    row: &CornerRow,
    section: &str,
) -> Arc<Result<SectionExcerpt, String>> {
    let key = SectionExcerptKey {
        library: row.library.clone(),
        corner: row.corner.name.clone(),
        section: section.to_owned(),
        digest: row.source_digest.clone().unwrap_or_default(),
    };
    let cache_id = egui::Id::new("models-corner-section-excerpt");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<SectionExcerptCache>(cache_id))
        && cached.key == key
    {
        return cached.excerpt;
    }
    let excerpt = Arc::new(retained_section(app, row, section));
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            cache_id,
            SectionExcerptCache {
                key,
                excerpt: Arc::clone(&excerpt),
            },
        );
    });
    excerpt
}

fn retained_section(
    app: &ManagerRenderContext<'_>,
    row: &CornerRow,
    section: &str,
) -> Result<SectionExcerpt, String> {
    let library = app
        .state
        .model_library_manager
        .get_library(&row.library)
        .ok_or_else(|| format!("Library '{}' no longer exists.", row.library))?;
    let path = row
        .corner
        .file_path
        .as_deref()
        .or(library.root_path.as_deref())
        .ok_or_else(|| {
            format!(
                "Corner '{}' is bound to no retained source.",
                row.corner.name
            )
        })?;
    // The retained bytes only. Reading the live file here would show a reader
    // something the run does not execute, which is the whole point of the
    // closure being retained.
    let content = library
        .source_contents
        .iter()
        .find(|content| content.path == path)
        .ok_or_else(|| {
            format!(
                "The authenticated closure retains no bytes for {}.",
                path.display()
            )
        })?;
    let mut excerpt = slice_section(&content.bytes, section).ok_or_else(|| {
        format!(
            "No `.lib {section}` … `.endl` pair appears in the retained {}.",
            path.display()
        )
    })?;
    excerpt.file = path.display().to_string();
    Ok(excerpt)
}

/// The bound section's source, read-only, between the matrix and the details.
///
/// The matrix says which section a corner binds; this says what that section
/// is. Without it the page named a file and a digest and never showed a line
/// of either, so the one question a binding raises — what does this actually
/// select — was answerable only by leaving the page.
fn section_source_pane(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    row: &CornerRow,
    height: f32,
) {
    let Some(binding) = displayed_section(&row.corner) else {
        return;
    };
    let excerpt = bound_section_excerpt(ui, app, row, &binding.section);
    let t = Tokens::get(ui.ctx());
    let mut open = false;
    card(ui, |ui| {
        let title = match excerpt.as_ref() {
            Ok(excerpt) => format!("{}({})", excerpt.file, excerpt.section),
            Err(_) => format!("({})", binding.section),
        };
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            super::hub::announced(
                ui,
                RichText::new(&title)
                    .monospace()
                    .strong()
                    .color(t.color.text),
                &title,
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if Button::new("Open the file")
                    .ghost()
                    .enabled(row.source.is_some())
                    .show(ui)
                    .on_hover_text("Open the whole retained file in a read-only source preview.")
                    .clicked()
                {
                    open = true;
                }
                let identity = format!(
                    "read-only · {}",
                    row.source_digest.as_deref().unwrap_or("not pinned")
                );
                super::hub::announced(
                    ui,
                    RichText::new(&identity)
                        .small()
                        .monospace()
                        .color(t.color.text_faint),
                    &identity,
                );
                let bound = format!("bound by {}", row.corner.name.to_uppercase());
                super::hub::announced(
                    ui,
                    RichText::new(&bound).small().color(t.color.text_dim),
                    &bound,
                );
            });
        });
        ui.separator();
        match excerpt.as_ref() {
            Ok(excerpt) => paint_section_lines(ui, excerpt, height),
            Err(error) => {
                super::hub::announced(ui, RichText::new(error).small().color(t.color.warn), error);
            }
        }
    });
    if open {
        open_corner_source(app, row);
    }
}

/// One line of the excerpt, at the mockup's `.code-line` rhythm.
const SECTION_LINE_H: f32 = 15.0;
/// Room the line-number gutter takes.
const SECTION_GUTTER_W: f32 = 36.0;

fn paint_section_lines(ui: &mut Ui, excerpt: &SectionExcerpt, height: f32) {
    let t = Tokens::get(ui.ctx());
    // However many lines the pane was given room for, never more than were
    // sliced. The footer counts whatever that leaves.
    let painted = ((height / SECTION_LINE_H).floor().max(1.0) as usize).min(excerpt.lines.len());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), painted as f32 * SECTION_LINE_H + 6.0),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.painter().vline(
        rect.left() + SECTION_GUTTER_W,
        rect.y_range(),
        Stroke::new(1.0, t.color.border),
    );
    for (offset, line) in excerpt.lines.iter().take(painted).enumerate() {
        let y = rect.top() + 3.0 + offset as f32 * SECTION_LINE_H + SECTION_LINE_H * 0.5;
        let text = elide(
            ui,
            line,
            (rect.width() - SECTION_GUTTER_W - 14.0).max(1.0),
            true,
        );
        ui.painter().text(
            egui::pos2(rect.left() + SECTION_GUTTER_W - 6.0, y),
            egui::Align2::RIGHT_CENTER,
            (excerpt.first_line + offset).to_string(),
            theme::mono(tokens::FS_MICRO, FontWeight::Regular),
            t.color.text_faint,
        );
        ui.painter().text(
            egui::pos2(rect.left() + SECTION_GUTTER_W + 8.0, y),
            egui::Align2::LEFT_CENTER,
            text,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text,
        );
    }
    // Painted glyphs publish nothing, so the excerpt is declared once as a
    // whole — sixteen nodes for sixteen lines would be a table of contents for
    // a paragraph.
    let spoken = excerpt
        .lines
        .iter()
        .take(painted)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), spoken.clone())
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_label(spoken.clone());
    });

    let remaining = excerpt.total_lines.saturating_sub(painted);
    let mut foot = if remaining > 0 {
        format!(
            "{remaining} more line{} · open the file",
            if remaining == 1 { "" } else { "s" }
        )
    } else {
        format!("{} lines · the whole section", excerpt.total_lines)
    };
    foot.push_str(
        " · sections are addressed by name inside one file, never a file per corner; the run \
         set selects the section per PVT point and nothing here is copied into the deck.",
    );
    super::hub::announced_widget(
        ui,
        egui::Label::new(RichText::new(&foot).small().color(t.color.text_faint)),
        &foot,
    );
}

/// What this corner binds for statistics and aging, and what those sections
/// declare.
///
/// Every line is a fact the project already holds: which section each domain
/// binds, and how many statistical variables the models inside that section
/// declare. Nothing about sampling, seeds or run counts appears — those belong
/// to the run set, and inventing them here would put a number in front of a
/// reader that no run would honour.
fn statistical_and_aging_card(ui: &mut Ui, app: &ManagerRenderContext<'_>, row: &CornerRow) {
    let global = row
        .corner
        .section_for_domain(CornerSectionDomain::StatisticalGlobal);
    let local = row
        .corner
        .section_for_domain(CornerSectionDomain::StatisticalLocal);
    let aging = row.corner.section_for_domain(CornerSectionDomain::Aging);
    let bound = global.is_some() || local.is_some() || aging.is_some();
    let library = app.state.model_library_manager.get_library(&row.library);
    detail_pane(
        ui,
        "STATISTICAL & AGING SECTIONS",
        Some(if bound { "bound" } else { "none bound" }),
        |ui| {
            if !bound {
                // One honest line rather than three rows of "not bound", and
                // the second half is the fact that makes the first actionable:
                // a library can declare statistics that no corner reaches.
                let line = if row.has_statistics {
                    format!(
                        "{} binds no statistical or aging section, so no variable this \
                         library's models declare is perturbed by it.",
                        row.corner.name.to_uppercase()
                    )
                } else {
                    format!(
                        "{} binds no statistical or aging section, and no model in this \
                         library declares a statistical variable.",
                        row.corner.name.to_uppercase()
                    )
                };
                super::hub::announced(
                    ui,
                    RichText::new(&line)
                        .small()
                        .color(Tokens::get(ui.ctx()).color.text_faint),
                    &line,
                );
                return;
            }
            for (label, section) in [
                ("Process (global)", global.as_deref()),
                ("Mismatch (local)", local.as_deref()),
            ] {
                let declared = section
                    .zip(library)
                    .map_or_else(String::new, |(section, library)| {
                        declared_statistics(library, section).0
                    });
                property(
                    ui,
                    label,
                    section.unwrap_or("not bound"),
                    if section.is_none() {
                        "this corner binds none"
                    } else {
                        &declared
                    },
                );
            }
            property(
                ui,
                "Aging",
                aging.as_deref().unwrap_or("not bound"),
                if aging.is_some() {
                    "authenticated section"
                } else {
                    "this corner binds none"
                },
            );
            let groups = [global.as_deref(), local.as_deref()]
                .into_iter()
                .flatten()
                .zip(std::iter::repeat(library))
                .filter_map(|(section, library)| Some(declared_statistics(library?, section).1))
                .max()
                .unwrap_or_default();
            property(
                ui,
                "Correlation groups",
                &groups.to_string(),
                "model schema",
            );
            let note = "Distributions, σ and correlation groups for these sections are owned \
                        by the model editor's statistics view.";
            super::hub::announced(
                ui,
                RichText::new(note)
                    .small()
                    .color(Tokens::get(ui.ctx()).color.text_faint),
                note,
            );
        },
    );
}

/// What the models inside one section declare: variables, and the correlation
/// groups they are grouped into.
///
/// Attributed by section rather than across the library, because a PDK that
/// publishes global and local statistics as separate sections declares
/// different variables in each — a library-wide total would print the same
/// number under both and say nothing about either.
fn declared_statistics(library: &ModelLibrary, section: &str) -> (String, usize) {
    let Some(models) = library
        .section_models
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(section))
        .map(|(_, models)| models)
    else {
        return ("no retained model carries this section".to_owned(), 0);
    };
    let mut variables = 0_usize;
    let mut groups = BTreeSet::new();
    for name in models.keys() {
        let Some(metadata) = library.model_definition_metadata.get(name) else {
            continue;
        };
        variables += metadata.statistics.variables.len();
        groups.extend(
            metadata
                .statistics
                .correlation_matrices
                .iter()
                .map(|matrix| matrix.group.clone()),
        );
    }
    let declared = if variables == 0 {
        "no declared statistical variable".to_owned()
    } else {
        format!(
            "{variables} declared variable{}",
            if variables == 1 { "" } else { "s" }
        )
    };
    (declared, groups.len())
}

/// What one corner binds one domain to, as the matrix cell states it.
///
/// The cells this replaces were not per-corner facts at all: BJT and passives
/// painted the literal word "section" for every corner whose *composite*
/// binding happened to resolve, so a PDK with independently selectable device
/// sections showed identical cells for corners that bound different sections —
/// and for corners that bound none. An unbound domain is now blank, which is
/// the answer most PDKs give and the one the run expansion acts on.
fn domain_cell(corner: &ProcessCorner, domain: CornerSectionDomain) -> String {
    corner.section_for_domain(domain).unwrap_or_default()
}

/// The statistical cell, which two domains can answer.
///
/// A PDK may publish global and local statistics as separate sections, and a
/// corner may bind either or both. Naming both is the only reading that does
/// not hide one of them behind the other.
fn statistical_cell(corner: &ProcessCorner) -> String {
    [
        CornerSectionDomain::StatisticalGlobal,
        CornerSectionDomain::StatisticalLocal,
    ]
    .into_iter()
    .filter_map(|domain| corner.section_for_domain(domain))
    .collect::<Vec<_>>()
    .join(" · ")
}

/// Findings the page lists before it stops and counts the rest.
const TEMPERATURE_FINDING_ROWS: usize = 3;

/// Corners the run set asks to run outside the range the PDK qualified them for.
///
/// A finding rather than a refusal: a run at 150 °C against a corner qualified
/// to 125 °C is a run the foundry does not vouch for, which is an engineer's
/// judgement rather than a verdict the tool can reach. The page's job is that
/// the judgement is made knowingly. It renders only when there is something to
/// say — a project whose corners cover its run set sees nothing here.
fn temperature_validity_findings(ui: &mut Ui, rows: &[CornerRow]) {
    let findings = rows
        .iter()
        .filter(|row| !row.unqualified_temperatures.is_empty())
        .collect::<Vec<_>>();
    if findings.is_empty() {
        return;
    }
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.warn))
        .inner_margin(egui::Margin::symmetric(12, 6))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            ui.spacing_mut().item_spacing.y = 2.0;
            for row in findings.iter().take(TEMPERATURE_FINDING_ROWS) {
                let line = format!(
                    "{} is qualified {}; this run set requests {}",
                    row.corner.name.to_uppercase(),
                    row.corner.qualified_range_label(),
                    crate::state::model_library::stated_temperatures(&row.unqualified_temperatures)
                );
                super::hub::announced(ui, RichText::new(&line).small().color(t.color.warn), &line);
            }
            if findings.len() > TEMPERATURE_FINDING_ROWS {
                let more = format!(
                    "{} more corner{} are qualified outside this run set",
                    findings.len() - TEMPERATURE_FINDING_ROWS,
                    if findings.len() - TEMPERATURE_FINDING_ROWS == 1 {
                        ""
                    } else {
                        "s"
                    }
                );
                super::hub::announced(
                    ui,
                    RichText::new(&more).small().color(t.color.text_faint),
                    &more,
                );
            }
        });
}

fn validate_current_model_execution_plan(
    app: &mut ManagerRenderContext<'_>,
    unresolved: usize,
) -> Result<String, String> {
    if unresolved > 0 {
        return Err(format!(
            "Corner validation found {unresolved} bindings without an exact source section."
        ));
    }
    if app.state.workbench.safe_mode.project_read_only() {
        return Err(
            "A durable model-validation receipt cannot be published while the project is read-only."
                .to_owned(),
        );
    }
    let has_project_technology = app.state.project_technology_in_effect();
    if has_project_technology {
        app.state.technology_gate_block_reason()?;
    }
    let sealed = if has_project_technology {
        app.state.seal_project_execution_model_sources()?
    } else {
        app.state.model_library_manager.seal_execution_sources()?
    };
    let plan = sealed.reference_model_execution_plan(app.state.sim_setup.reference_pvt.process)?;
    let mut findings = vec![
        crate::state::model_library::ModelValidationFinding {
            code: "SOURCE_CLOSURE_AUTHENTICATED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: "Every executable SPICE source and transitive dependency matched its accepted content digest.".to_owned(),
        },
        crate::state::model_library::ModelValidationFinding {
            code: "SPICE_NAMESPACE_COMPILED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: format!(
                "The frozen SPICE namespace compiled with {} bindings and {} explicit provider decisions.",
                plan.bindings().len(),
                plan.applied_resolutions().len()
            ),
        },
    ];
    let mut veriloga_count = 0_usize;
    if let Some((package, archive_digest, artifacts, bindings)) = sealed.pdk_veriloga_authority() {
        for binding in bindings {
            crate::simulation::veriloga::compile_signed_pdk_source_runtime(
                package,
                archive_digest,
                artifacts,
                binding,
            )?;
            veriloga_count += 1;
        }
        findings.push(crate::state::model_library::ModelValidationFinding {
            code: "VERILOGA_RUNTIME_COMPILED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: format!(
                "Compiled and validated {veriloga_count} authenticated signed-PDK Verilog-A runtime bindings."
            ),
        });
    }
    let pdk_archive_digest = sealed
        .pdk_model_identity()
        .map(|(_, archive_digest)| archive_digest);
    if pdk_archive_digest.is_some() {
        findings.push(crate::state::model_library::ModelValidationFinding {
            code: "SIGNED_PDK_TRUST_VERIFIED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: "The exact project-pinned signed PDK archive, platform contract, and trust chain were verified.".to_owned(),
        });
    }
    let receipt = app
        .state
        .model_library_manager
        .issue_model_validation_receipt(
            app.state.workspace.project.revision(),
            plan.digest(),
            pdk_archive_digest,
            crate::io::PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
            findings,
        )?;
    app.state
        .model_library_manager
        .validate_model_validation_receipt(
            app.state.workspace.project.revision(),
            plan.digest(),
            pdk_archive_digest,
            crate::io::PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
        )?;
    app.state.workspace.project_metadata_dirty = true;
    Ok(format!(
        "Published durable model-validation receipt {} for exact plan {} with {} authenticated bindings, {} source-qualified provider decisions, and {veriloga_count} Verilog-A runtimes.",
        receipt.receipt_digest,
        plan.digest(),
        plan.bindings().len(),
        plan.applied_resolutions().len()
    ))
}

#[derive(Clone)]
struct CornerRow {
    key: String,
    library: String,
    corner: ProcessCorner,
    /// Why a run cannot expand this corner, in the words the run itself uses.
    /// `None` means the corner resolves.
    blocker: Option<String>,
    /// Whether naming a section is what would unblock this row — the one
    /// blocker the bind dialog can act on.
    ///
    /// Three unrelated things stop a corner and only one of them is a binding.
    /// A malformed contract needs the corner editor, a corner bound to no
    /// retained source at all needs a library, and what is left is the closure
    /// verdict: a required domain with no binding, or a binding naming a
    /// section the authenticated closure does not carry. Offering "Bind
    /// section…" on the first two would be a control that cannot help.
    binding_blocked: bool,
    has_statistics: bool,
    source: Option<String>,
    source_digest: Option<String>,
    active: bool,
    /// Temperatures this run set asks for that the corner's qualified range
    /// excludes. Empty is the healthy state and the usual one.
    unqualified_temperatures: Vec<f64>,
}

impl CornerRow {
    const fn resolved(&self) -> bool {
        self.blocker.is_none()
    }

    /// The status cell, in the mockup's own words.
    const fn status_label(&self) -> &'static str {
        match (self.active, self.resolved()) {
            (true, true) => "active reference",
            (true, false) => "active · blocked",
            (false, true) => "resolved",
            (false, false) => "unresolved",
        }
    }
}

/// Why a corner cannot be expanded into a run, or `None` if it can.
///
/// The verdict is the run's own, asked rather than restated: `RetainedClosure`
/// holds the one acceptance rule, and `io::project_execution`'s
/// `persisted_active_model_section_names` calls the same function. Restating it
/// here is what let the page and the run disagree twice — the page had no
/// counterpart for the run's project-owned escape, and it asked whether a
/// section *defined* anything where the run asks only whether the closure
/// carries it.
///
/// The corner contract is checked first because a malformed corner is a
/// finding about the corner, not about run expansion.
fn corner_blocker(library: &ModelLibrary, corner: &ProcessCorner) -> Option<String> {
    if let Err(errors) = corner.validate_contract() {
        return Some(errors.join("; "));
    }
    RetainedClosure::from(library).expansion_blocker(corner)
}

fn corner_rows(app: &ManagerRenderContext<'_>) -> Vec<CornerRow> {
    let mut rows = Vec::new();
    // Every corner on this page is one a reader is authoring, so every one is
    // compared against the run set — not only the corner that happens to be
    // active, which is the narrower question the preflight report asks.
    let requested = app.state.sim_setup.requested_temperatures_celsius();
    let libraries = app.state.model_library_manager.libraries_sorted();
    let active_library = app
        .state
        .model_library_manager
        .selected_library
        .as_deref()
        .and_then(|selected| {
            libraries
                .iter()
                .find(|library| library.name.eq_ignore_ascii_case(selected))
        })
        .or_else(|| libraries.iter().find(|library| !library.corners.is_empty()))
        .map(|library| library.name.clone());
    for library in libraries {
        if active_library.as_deref() != Some(library.name.as_str()) {
            continue;
        }
        let has_statistics = library
            .model_definition_metadata
            .values()
            .any(|metadata| !metadata.statistics.variables.is_empty());
        for corner in library.corners.values() {
            let source_path = corner.file_path.as_deref().or(library.root_path.as_deref());
            let source = source_path.map(|path| path.display().to_string());
            let source_digest = source_path.and_then(|path| {
                library
                    .source_closure
                    .iter()
                    .find(|pin| pin.path == path)
                    .map(|pin| short_digest(&pin.digest.to_string()))
            });
            let blocker = if source.is_none() {
                Some(format!(
                    "corner '{}' is not bound to a retained source",
                    corner.name
                ))
            } else {
                corner_blocker(library, corner)
            };
            rows.push(CornerRow {
                key: format!("{}\u{1f}{}", library.name, corner.name),
                library: library.name.clone(),
                unqualified_temperatures: corner.temperatures_outside_qualified_range(&requested),
                // `validate_draft_contract` is the right question and
                // `validate_contract` is not: the executable form fails on an
                // unbound required domain, which is precisely the blocker this
                // control exists to fix. The draft form tolerates that and
                // still rejects a malformed corner, so what it accepts is
                // exactly "well-formed, and missing a section".
                binding_blocked: blocker.is_some()
                    && source.is_some()
                    && corner.validate_draft_contract().is_ok(),
                corner: corner.clone(),
                blocker,
                has_statistics,
                source,
                source_digest,
                active: library
                    .selected_corner
                    .as_deref()
                    .is_some_and(|active| active.eq_ignore_ascii_case(&corner.name)),
            });
        }
    }
    rows.sort_by(|left, right| {
        left.corner
            .name
            .to_ascii_lowercase()
            .cmp(&right.corner.name.to_ascii_lowercase())
            .then_with(|| left.library.cmp(&right.library))
    });
    rows
}

fn corner_detail(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, row: &CornerRow) {
    let mut open_editor = false;
    let mut duplicate = false;
    let mut make_default = false;
    let mut activate = false;
    let mut delete = false;
    let mut unbind = None;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        ui.label(
            RichText::new(format!(
                "{} / {}",
                row.library,
                row.corner.name.to_uppercase()
            ))
            .monospace()
            .strong(),
        );
        if let Some(blocker) = row.blocker.as_deref() {
            ui.label(
                RichText::new(format!("run expansion blocked · {blocker}"))
                    .small()
                    .color(Tokens::get(ui.ctx()).color.err),
            );
        }
        if row.active {
            ui.label(
                RichText::new("ACTIVE FOR EXECUTION")
                    .small()
                    .strong()
                    .color(Tokens::get(ui.ctx()).color.accent),
            );
        }
        // Accent only while activating is both possible and the thing left to
        // do: a corner already active, or one that cannot run at all, has no
        // primary action for the row to advertise, and an accent fill on a
        // disabled control reads as the page's recommendation.
        let activatable = !row.active && row.resolved();
        let mut activate_action = Button::new("Use for execution").enabled(activatable);
        if activatable {
            activate_action = activate_action.accent();
        }
        if activate_action
            .show(ui)
            .on_disabled_hover_text(if row.active {
                "This corner is already active for this library's executable model projection."
            } else {
                "Resolve every required source-section binding before activating this corner."
            })
            .clicked()
        {
            activate = true;
        }
        if Button::new("Edit corner…").show(ui).clicked() {
            open_editor = true;
        }
        if Button::new("Duplicate…").show(ui).clicked() {
            duplicate = true;
        }
        if Button::new("Set default")
            .enabled(!row.corner.is_default)
            .show(ui)
            .clicked()
        {
            make_default = true;
        }
        if Button::new("Delete corner…")
            .destructive(true)
            .show(ui)
            .clicked()
        {
            delete = true;
        }
        if Button::new("Bind section…").show(ui).clicked() {
            open_corner_binding_dialog(app, row);
        }
        // No "Unbind <domain>" per binding here: a PDK that binds six domains
        // grew six destructive controls in the row that carries the corner's
        // own lifecycle, ahead of Delete and after nothing in particular. Each
        // unbind now sits on the section row it removes, in the details pane
        // below, where the section it acts on is named beside it.
        //
        // The corner's own retained file, not whichever model the library
        // happens to iterate first.
        if Button::new("Open source")
            .enabled(row.source.is_some())
            .show(ui)
            .clicked()
        {
            open_corner_source(app, row);
        }
        if Button::new("View include graph").show(ui).clicked() {
            app.state.workbench.models_page = ModelsPage::Include;
        }
        if Button::new("Model editor…").show(ui).clicked() {
            app.queue_command(Command::ModelEditor);
        }
    });
    if activate {
        activate_corner(app, &row.library, &row.corner.name);
    } else if open_editor || duplicate {
        open_corner_editor(app, row, duplicate);
    } else if make_default {
        corner_ops::set_default_corner(app, &row.library, &row.corner.name);
    } else if delete {
        app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmDeleteCorner {
            library: row.library.clone(),
            corner: row.corner.name.clone(),
        });
    }
    // Filled rather than plain: this is the page's last band, and with three
    // more panes above it a corner declaring a dozen properties used to run off
    // the bottom edge. The pane now takes the remainder exactly and scrolls its
    // own rows, which is also what leaves the document surface painted to the
    // bottom instead of ending in bare canvas.
    filled_detail_pane(
        ui,
        "CORNER BINDING DETAILS",
        Some("section, environment, statistics, and aging"),
        ui.available_height(),
        "models-corner-detail",
        |ui| {
            property(
                ui,
                "Description",
                &row.corner.description,
                "project metadata",
            );
            property(
                ui,
                "Default",
                if row.corner.is_default { "yes" } else { "no" },
                "new-plan fallback",
            );
            property(
                ui,
                "Execution active",
                if row.active { "yes" } else { "no" },
                "executable model projection",
            );
            property(ui, "NMOS", &row.corner.nmos_corner, "exact section axis");
            property(ui, "PMOS", &row.corner.pmos_corner, "exact section axis");
            property(
                ui,
                "Source",
                row.source.as_deref().unwrap_or("not bound"),
                if row.resolved() {
                    "retained"
                } else {
                    "unresolved"
                },
            );
            property(
                ui,
                "Source digest",
                row.source_digest.as_deref().unwrap_or("not pinned"),
                "authenticated content identity",
            );
            property(
                ui,
                "Supply factor",
                &engineering_value(row.corner.vdd_factor),
                "environment axis",
            );
            property(
                ui,
                "Temperature",
                &engineering_quantity(row.corner.temperature, " °C"),
                "environment axis",
            );
            property(
                ui,
                "Qualified range",
                &row.corner.qualified_range_label(),
                &if row.unqualified_temperatures.is_empty() {
                    "temperature validity".to_owned()
                } else {
                    format!(
                        "excludes {}, which this run set requests",
                        crate::state::model_library::stated_temperatures(
                            &row.unqualified_temperatures
                        )
                    )
                },
            );
            property(
                ui,
                "Required domains",
                &row.corner
                    .effective_required_domains()
                    .into_iter()
                    .map(CornerSectionDomain::label)
                    .collect::<Vec<_>>()
                    .join(", "),
                "execution contract",
            );
            for binding in row.corner.effective_section_bindings() {
                if bound_section_row(ui, binding.domain, &binding.section) {
                    unbind = Some(binding.domain);
                }
            }
            ui.separator();
            property(
                ui,
                "Statistical variables",
                if row.has_statistics {
                    "declared"
                } else {
                    "none"
                },
                "model schema",
            );
            // No "Aging evidence" row: the one this replaces read
            // `model_qualification.evidence` — reviewer evidence for whatever a
            // suite qualified — and printed it under an aging label. Nothing in
            // this project owns aging data yet, so the aging column above
            // states the binding fact and this pane states nothing at all.
            property(
                ui,
                "Binding policy",
                if row.resolved() {
                    "executable"
                } else {
                    "fail closed"
                },
                "run expansion",
            );
            if let Some(receipt) = app.state.model_library_manager.model_validation_receipt() {
                let current_revision = app.state.workspace.project.revision();
                let receipt_state = if receipt.project_revision == current_revision {
                    "current revision"
                } else {
                    "stale revision"
                };
                property(
                    ui,
                    "Validation receipt",
                    &format!(
                        "{} ({receipt_state})",
                        short_digest(&receipt.receipt_digest.to_string())
                    ),
                    &format!(
                        "project revision {} · plan {} · {} authenticated sources · {}",
                        receipt.project_revision.get(),
                        short_digest(&receipt.model_execution_plan_digest.to_string()),
                        receipt.source_count,
                        receipt.platform
                    ),
                );
            }
        },
    );
    // Settled after the pane rather than with the action row above, because
    // this is the pane the control now lives in.
    if let Some(domain) = unbind {
        corner_ops::unbind_corner_section(app, &row.library, &row.corner.name, domain);
    }
}

/// One bound section, with the action that removes it on the row that names it.
///
/// `property` owns the three-column rhythm every other row in this pane uses,
/// so the row hands it the track less the action's own width rather than
/// painting a second spelling of the same thing.
fn bound_section_row(ui: &mut Ui, domain: CornerSectionDomain, section: &str) -> bool {
    /// Room the inline action takes out of the property row.
    const ACTION_W: f32 = 54.0;
    let mut unbind = false;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let track = (ui.available_width() - ACTION_W).max(1.0);
        ui.allocate_ui(egui::vec2(track, PROPERTY_ROW_H), |ui| {
            ui.set_min_width(track);
            property(ui, domain.label(), section, "authenticated section");
        });
        unbind = inline_unbind(ui, domain, section, ACTION_W);
    });
    unbind
}

/// The height `property` draws one row at, which the inline action matches so
/// a pane of bound sections keeps one rhythm.
const PROPERTY_ROW_H: f32 = 22.0;

/// The destructive half of a bound-section row.
///
/// Painted rather than built from the design system's button because the
/// button is a control-height widget and would set the pane's row rhythm by
/// itself; what it publishes — a button node carrying the section it removes —
/// is declared here instead.
fn inline_unbind(ui: &mut Ui, domain: CornerSectionDomain, section: &str, width: f32) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, PROPERTY_ROW_H), Sense::click());
    let announced = format!("Unbind {} section {section}", domain.label());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), announced.clone())
    });
    let hovered = response.hovered();
    let inner = rect.shrink2(egui::vec2(2.0, 3.0));
    if hovered {
        ui.painter().rect(
            inner,
            t.radius,
            t.color.bg_hover,
            Stroke::new(1.0, t.color.err),
            egui::StrokeKind::Inside,
        );
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    ui.painter().text(
        inner.center(),
        egui::Align2::CENTER_CENTER,
        "Unbind",
        theme::sans(tokens::FS_MICRO, FontWeight::Regular),
        if hovered {
            t.color.err
        } else {
            t.color.err.gamma_multiply(0.7)
        },
    );
    theme::paint_focus_ring(ui, &response, inner);
    response.clicked()
}

fn open_corner_binding_dialog(app: &mut ManagerRenderContext<'_>, row: &CornerRow) {
    let section = app
        .state
        .model_library_manager
        .get_library(&row.library)
        .and_then(|library| library.section_index().into_iter().next())
        .unwrap_or_default();
    let bindings = row.corner.effective_section_bindings();
    let domain = row
        .corner
        .effective_required_domains()
        .into_iter()
        .find(|required| !bindings.iter().any(|binding| binding.domain == *required))
        .or_else(|| bindings.first().map(|binding| binding.domain))
        .unwrap_or(CornerSectionDomain::Composite);
    app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::BindCornerSection {
        library: row.library.clone(),
        corner: row.corner.name.clone(),
        domain,
        section,
    });
}

fn open_corner_editor(app: &mut ManagerRenderContext<'_>, row: &CornerRow, duplicate: bool) {
    let name = if duplicate {
        let base = format!("{}_copy", row.corner.name);
        let mut candidate = base.clone();
        let mut suffix = 2_u32;
        if let Some(library) = app.state.model_library_manager.get_library(&row.library) {
            while library
                .corners
                .keys()
                .any(|existing| existing.eq_ignore_ascii_case(&candidate))
            {
                candidate = format!("{base}_{suffix}");
                suffix += 1;
            }
        }
        candidate
    } else {
        row.corner.name.clone()
    };
    app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::EditCorner {
        library: row.library.clone(),
        original_name: row.corner.name.clone(),
        duplicate,
        name,
        description: row.corner.description.clone(),
        nmos_corner: row.corner.nmos_corner.clone(),
        pmos_corner: row.corner.pmos_corner.clone(),
        temperature_c: row.corner.temperature.to_string(),
        supply_factor: row.corner.vdd_factor.to_string(),
        minimum_temperature_c: row
            .corner
            .minimum_temperature_c
            .map_or_else(String::new, |value| value.to_string()),
        maximum_temperature_c: row
            .corner
            .maximum_temperature_c
            .map_or_else(String::new, |value| value.to_string()),
        required_domains: row.corner.effective_required_domains(),
        make_default: !duplicate && row.corner.is_default,
    });
}

/// Show the retained bytes of the file this corner is bound to.
fn open_corner_source(app: &mut ManagerRenderContext<'_>, row: &CornerRow) {
    let Some(library) = app
        .state
        .model_library_manager
        .get_library(&row.library)
        .cloned()
    else {
        receipt(
            app,
            Err(format!("Library '{}' no longer exists.", row.library)),
        );
        return;
    };
    let path = row
        .corner
        .file_path
        .as_deref()
        .or(library.root_path.as_deref());
    let Some(path) = path else {
        receipt(
            app,
            Err(format!(
                "Corner '{}' is not bound to a retained source.",
                row.corner.name
            )),
        );
        return;
    };
    let retained = library
        .source_contents
        .iter()
        .find(|content| content.path == path);
    match retained {
        Some(content) => {
            app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::SourcePreview {
                title: format!("{} / {}", library.name, row.corner.name.to_uppercase()),
                subtitle: format!("{} · retained closure member", content.path.display()),
                source: String::from_utf8_lossy(&content.bytes).into_owned(),
                editable: false,
            });
        }
        None => match std::fs::read_to_string(path) {
            Ok(source) => {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::SourcePreview {
                        title: format!("{} / {}", library.name, row.corner.name.to_uppercase()),
                        subtitle: format!("{} · live unpinned source", path.display()),
                        source,
                        editable: false,
                    });
            }
            Err(error) => receipt(
                app,
                Err(format!(
                    "Could not read corner source '{}': {error}",
                    path.display()
                )),
            ),
        },
    }
}

fn inspect_corner(app: &mut ManagerRenderContext<'_>, row: &CornerRow) {
    app.state.workbench.models_view.selected_corner = Some(row.key.clone());
}

fn activate_corner(app: &mut ManagerRenderContext<'_>, library_name: &str, corner_name: &str) {
    app.state.select_model_library(library_name);
    let mut candidate = app.state.model_library_manager.clone();
    let result = candidate
        .get_library_mut(library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))
        .and_then(|library| {
            library
                .activate_corner(corner_name)
                .then_some(())
                .ok_or_else(|| {
                    format!(
                        "Corner '{corner_name}' no longer exists in library '{library_name}'."
                    )
                })
        })
        .and_then(|()| {
            publish_model_library_candidate(
                app.state,
                candidate,
                library_name,
                format!("activate model corner {corner_name}"),
            )
        })
        .map(|revision| {
            format!(
                "Activated exact corner '{corner_name}' for '{library_name}' at project revision {}.",
                revision.get()
            )
        });
    receipt(app, result);
}

#[cfg(test)]
mod tests;
