//! Project-owned library, cellview, symbol, and component-form authoring.
//!
//! The canonical specialist route deliberately composes existing authoritative
//! owners. The Libraries page reuses the Project workspace's exact
//! three-column browser; the Symbol & form page projects the selected
//! `ModelBoundSymbolDefinition` and the same `SymbolDocument` used by the
//! Design editor. No parallel catalog, symbol, form, or mutation state lives
//! here.

use egui::{
    Align, Color32, Frame, Grid, Key, Layout, Margin, Modifiers, Rect, RichText, ScrollArea, Sense,
    Stroke, Ui, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::state::{
    CellViewRef, ModelBoundSymbolDefinition, ParameterInheritance, PortDirection, PropertyType,
    SymbolDocument, SymbolParameterVisibility, SymbolSourceContract, ViewType,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;
use crate::workbench::app::{
    open_create_model_bound_symbol_dialog, open_symbol_import_dialog_for,
    open_symbol_parameter_form_dialog_for,
};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::{LibraryCellviewPage, Workspace};

use super::project;

const COMPACT_BREAKPOINT: f32 = 820.0;
const SPLIT_BREAKPOINT: f32 = 960.0;
const HEADER_MIN_HEIGHT: f32 = 68.0;
const TAB_HEIGHT: f32 = 34.0;
const PIN_TABLE_MIN_WIDTH: f32 = 690.0;
const FORM_TABLE_MIN_WIDTH: f32 = 900.0;

const SURFACE_TITLE: &str = "Library, cellview, symbol and form authoring";
const SURFACE_SUMMARY: &str = "Author project libraries, cells, views, terminal contracts, component forms, locks, revisions, and publication evidence from one project-owned identity.";

#[derive(Clone)]
struct SymbolProjection {
    reference: CellViewRef,
    library_read_only: bool,
    modified: bool,
    definition: Result<Option<ModelBoundSymbolDefinition>, String>,
    legacy_document: Result<SymbolDocument, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymbolAction {
    OpenEditor,
    EditForm,
    Create,
    Import,
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let tokens = Tokens::get(ui.ctx());
    Frame::new().fill(tokens.color.bg_app).show(ui, |ui| {
        surface_header(ui, app);
        page_tabs(ui, app);

        let page = app.state.workbench.library_cellview_page;
        let panel = ui.scope(|ui| match page {
            LibraryCellviewPage::Libraries => project::show_library_browser(ui, app),
            LibraryCellviewPage::SymbolForm => symbol_form_page(ui, app),
        });
        ui.ctx().accesskit_node_builder(panel.response.id, |node| {
            node.set_role(egui::accesskit::Role::TabPanel);
            node.set_label(format!("Library Cellview Manager {}", page.label()));
        });
    });
}

fn surface_header(ui: &mut Ui, app: &RSpiceApp) {
    let tokens = Tokens::get(ui.ctx());
    let snapshot = catalog_snapshot(app);
    let compact = ui.available_width() <= COMPACT_BREAKPOINT;
    let shown = Frame::new()
        .fill(tokens.color.bg_panel)
        .inner_margin(Margin::symmetric(12, 9))
        .show(ui, |ui| {
            ui.set_min_height(HEADER_MIN_HEIGHT - 18.0);
            if compact {
                identity_block(ui, &tokens);
                ui.add_space(6.0);
                ScrollArea::horizontal()
                    .id_salt("library-cellview-header-context")
                    .auto_shrink([false, true])
                    .show(ui, |ui| header_context(ui, app, snapshot, &tokens));
            } else {
                ui.horizontal(|ui| {
                    identity_block(ui, &tokens);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        header_context(ui, app, snapshot, &tokens);
                    });
                });
            }
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, tokens.color.border_strong),
    );
    ui.ctx().accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Region);
        node.set_label("Library Cellview Manager identity");
    });
}

fn identity_block(ui: &mut Ui, tokens: &Tokens) {
    ui.vertical(|ui| {
        ui.label(
            RichText::new(SURFACE_TITLE)
                .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                .color(tokens.color.text),
        );
        ui.add(
            egui::Label::new(
                RichText::new(SURFACE_SUMMARY)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(tokens.color.text_dim),
            )
            .wrap(),
        );
    });
}

#[derive(Debug, Clone, Copy)]
struct CatalogSnapshot {
    libraries: usize,
    cells: usize,
    views: usize,
}

fn catalog_snapshot(app: &RSpiceApp) -> CatalogSnapshot {
    let libraries = app.state.library_manager.libraries_sorted();
    CatalogSnapshot {
        libraries: libraries.len(),
        cells: libraries.iter().map(|library| library.cell_count()).sum(),
        views: libraries
            .iter()
            .map(|library| library.total_view_count())
            .sum(),
    }
}

fn header_context(ui: &mut Ui, app: &RSpiceApp, snapshot: CatalogSnapshot, tokens: &Tokens) {
    ui.spacing_mut().item_spacing.x = 8.0;
    header_chip(
        ui,
        "Project",
        &format!("r{}", app.state.workspace.project.revision().get()),
        tokens.color.text,
        tokens,
    );
    header_chip(
        ui,
        "Catalog",
        &format!("r{}", app.state.library_manager.revision()),
        tokens.color.text,
        tokens,
    );
    header_chip(
        ui,
        "Contents",
        &format!(
            "{} libraries · {} cells · {} views",
            snapshot.libraries, snapshot.cells, snapshot.views
        ),
        tokens.color.text_dim,
        tokens,
    );
    let access = if !app.state.project_lifecycle.project_open {
        ("closed", tokens.color.err)
    } else if app.state.workbench.safe_mode.project_read_only() {
        ("safe-mode read only", tokens.color.warn)
    } else {
        ("project governed", tokens.color.ok)
    };
    header_chip(ui, "Access", access.0, access.1, tokens);
}

fn header_chip(ui: &mut Ui, label: &str, value: &str, color: Color32, tokens: &Tokens) {
    Frame::new()
        .fill(tokens.color.bg_inset)
        .stroke(Stroke::new(1.0, tokens.color.border))
        .corner_radius(tokens.radius)
        .inner_margin(Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(label)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(tokens.color.text_faint),
                );
                ui.label(
                    RichText::new(value)
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(color),
                );
            });
        });
}

fn page_tabs(ui: &mut Ui, app: &mut RSpiceApp) {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let tab_ids =
        LibraryCellviewPage::ALL.map(|page| ui.id().with(("library-cellview-page", page)));
    let shown = Frame::new()
        .fill(tokens.color.bg_panel)
        .inner_margin(Margin {
            left: 10,
            right: 10,
            top: 0,
            bottom: 0,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                for (index, page) in LibraryCellviewPage::ALL.into_iter().enumerate() {
                    let width = ui
                        .painter()
                        .layout_no_wrap(page.label().to_owned(), font.clone(), tokens.color.text)
                        .size()
                        .x
                        + 24.0;
                    let (_, rect) = ui.allocate_space(vec2(width, TAB_HEIGHT));
                    let response = ui.interact(rect, tab_ids[index], Sense::click());
                    let selected = app.state.workbench.library_cellview_page == page;
                    response.widget_info(|| {
                        WidgetInfo::selected(
                            WidgetType::SelectableLabel,
                            ui.is_enabled(),
                            selected,
                            page.label(),
                        )
                    });
                    ui.ctx().accesskit_node_builder(response.id, |node| {
                        node.set_role(egui::accesskit::Role::Tab);
                        node.set_label(page.label());
                        node.set_selected(selected);
                    });
                    if response.hovered() {
                        ui.painter().rect_filled(rect, 0.0, tokens.color.bg_hover);
                    }
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        page.label(),
                        font.clone(),
                        if selected {
                            tokens.color.text
                        } else {
                            tokens.color.text_dim
                        },
                    );
                    if selected {
                        ui.painter().rect_filled(
                            Rect::from_min_max(
                                pos2(rect.left() + 7.0, rect.bottom() - 2.0),
                                pos2(rect.right() - 7.0, rect.bottom()),
                            ),
                            0.0,
                            tokens.color.accent,
                        );
                    }
                    theme::paint_focus_ring(ui, &response, rect);
                    if response.clicked() {
                        app.state.workbench.library_cellview_page = page;
                    }
                    if response.has_focus() {
                        let target = ui.input_mut(|input| {
                            if input.consume_key(Modifiers::NONE, Key::ArrowRight) {
                                Some((index + 1) % LibraryCellviewPage::ALL.len())
                            } else if input.consume_key(Modifiers::NONE, Key::ArrowLeft) {
                                Some(
                                    (index + LibraryCellviewPage::ALL.len() - 1)
                                        % LibraryCellviewPage::ALL.len(),
                                )
                            } else if input.consume_key(Modifiers::NONE, Key::Home) {
                                Some(0)
                            } else if input.consume_key(Modifiers::NONE, Key::End) {
                                Some(LibraryCellviewPage::ALL.len() - 1)
                            } else {
                                None
                            }
                        });
                        if let Some(target) = target {
                            app.state.workbench.library_cellview_page =
                                LibraryCellviewPage::ALL[target];
                            ui.memory_mut(|memory| memory.request_focus(tab_ids[target]));
                        }
                    }
                }
            });
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, tokens.color.border_strong),
    );
    ui.ctx().accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::TabList);
        node.set_label("Library Cellview Manager pages");
    });
}

fn symbol_form_page(ui: &mut Ui, app: &mut RSpiceApp) {
    let projection = symbol_projection(app);
    ScrollArea::vertical()
        .id_salt("library-cellview-symbol-form")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            symbol_context(ui, app, projection.as_ref());
            ui.add_space(10.0);
            match projection {
                None => no_symbol_state(ui, app),
                Some(projection) => selected_symbol_surface(ui, app, &projection),
            }
            ui.add_space(16.0);
        });
}

fn symbol_projection(app: &RSpiceApp) -> Option<SymbolProjection> {
    let reference = resolve_symbol_reference(app)?;
    let library = app.state.library_manager.get_library(&reference.library)?;
    let view = library
        .get_cell(&reference.cell)?
        .get_view(&reference.view)?;
    Some(SymbolProjection {
        reference,
        library_read_only: library.read_only,
        modified: view.modified,
        definition: ModelBoundSymbolDefinition::load_from_view(view)
            .map_err(|error| error.to_string()),
        legacy_document: SymbolDocument::load_from_view(view),
    })
}

fn resolve_symbol_reference(app: &RSpiceApp) -> Option<CellViewRef> {
    let manager = &app.state.library_manager;
    let selected = manager.current_library().and_then(|library| {
        let cell = manager.current_cell()?;
        let view = manager.current_view()?;
        (view.view_type == ViewType::Symbol)
            .then(|| CellViewRef::new(&library.name, &cell.name, &view.name))
    });
    if selected.is_some() {
        return selected;
    }

    let active = &app.state.workspace.active_view;
    let active_is_symbol = manager
        .get_library(&active.library)
        .and_then(|library| library.get_cell(&active.cell))
        .and_then(|cell| cell.get_view(&active.view))
        .is_some_and(|view| view.view_type == ViewType::Symbol);
    if active_is_symbol {
        return Some(active.clone());
    }

    first_symbol_reference(app, true).or_else(|| first_symbol_reference(app, false))
}

fn first_symbol_reference(app: &RSpiceApp, typed_only: bool) -> Option<CellViewRef> {
    app.state
        .library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| {
            library.cells_sorted().into_iter().flat_map(move |cell| {
                cell.views_sorted().into_iter().filter_map(move |view| {
                    if view.view_type != ViewType::Symbol {
                        return None;
                    }
                    if typed_only
                        && !matches!(
                            ModelBoundSymbolDefinition::load_from_view(view),
                            Ok(Some(_))
                        )
                    {
                        return None;
                    }
                    Some(CellViewRef::new(&library.name, &cell.name, &view.name))
                })
            })
        })
        .next()
}

fn symbol_context(ui: &mut Ui, app: &RSpiceApp, projection: Option<&SymbolProjection>) {
    let tokens = Tokens::get(ui.ctx());
    let selection = projection
        .map(|projection| projection.reference.display_path())
        .unwrap_or_else(|| "No symbol view available".to_owned());
    let state = projection.map_or(("empty", tokens.color.warn), |projection| match &projection
        .definition
    {
        Ok(Some(definition)) if definition.validate().is_ok() => {
            ("typed contract valid", tokens.color.ok)
        }
        Ok(Some(_)) => ("typed contract invalid", tokens.color.err),
        Ok(None) => ("legacy symbol · no typed contract", tokens.color.warn),
        Err(_) => ("typed metadata unreadable", tokens.color.err),
    });
    Frame::new()
        .fill(tokens.color.bg_inset)
        .inner_margin(Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ScrollArea::horizontal()
                .id_salt("library-cellview-symbol-context")
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        context_value(ui, "Selection", &selection, tokens.color.text, &tokens);
                        ui.separator();
                        context_value(ui, "Contract", state.0, state.1, &tokens);
                        ui.separator();
                        context_value(
                            ui,
                            "Project",
                            &format!("r{}", app.state.workspace.project.revision().get()),
                            tokens.color.text_dim,
                            &tokens,
                        );
                        ui.separator();
                        context_value(
                            ui,
                            "Catalog",
                            &format!("r{}", app.state.library_manager.revision()),
                            tokens.color.text_dim,
                            &tokens,
                        );
                    });
                });
        });
}

fn context_value(ui: &mut Ui, label: &str, value: &str, color: Color32, tokens: &Tokens) {
    ui.label(
        RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(tokens.color.text_faint),
    );
    ui.label(
        RichText::new(value)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color),
    );
}

fn no_symbol_state(ui: &mut Ui, app: &mut RSpiceApp) {
    let tokens = Tokens::get(ui.ctx());
    section_card(
        ui,
        "No symbol view is available",
        "Create or import a model-bound symbol to establish an explicit terminal, source, component-form, and netlist contract.",
        |ui| {
            let block = project_write_block_reason(app);
            let create = Button::new("Create model-bound symbol\u{2026}")
                .accent()
                .enabled(block.is_none())
                .show(ui);
            if let Some(reason) = block {
                create.clone().on_disabled_hover_text(reason);
            }
            if create.clicked() {
                open_create_model_bound_symbol_dialog(&mut app.state);
            }
            let import = Button::new("Import symbol\u{2026}")
                .enabled(block.is_none())
                .show(ui);
            if let Some(reason) = block {
                import.clone().on_disabled_hover_text(reason);
            }
            if import.clicked() {
                open_symbol_import_dialog_for(&mut app.state, None);
            }
        },
    );
    ui.label(
        RichText::new("No placeholder symbol or inferred electrical interface is rendered.")
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(tokens.color.text_faint),
    );
}

fn selected_symbol_surface(ui: &mut Ui, app: &mut RSpiceApp, projection: &SymbolProjection) {
    symbol_actions(ui, app, projection);
    ui.add_space(10.0);

    match &projection.definition {
        Ok(Some(definition)) => typed_symbol_surface(ui, definition, projection),
        Ok(None) => legacy_symbol_surface(ui, projection),
        Err(error) => invalid_symbol_surface(ui, projection, error),
    }
}

fn symbol_actions(ui: &mut Ui, app: &mut RSpiceApp, projection: &SymbolProjection) {
    let tokens = Tokens::get(ui.ctx());
    let typed = projection.definition.as_ref().is_ok_and(Option::is_some);
    let write_block = symbol_write_block_reason(app, projection);
    let project_block = project_write_block_reason(app);
    let compact = ui.available_width() <= COMPACT_BREAKPOINT;
    let mut action = None;

    let shown = Frame::new()
        .fill(tokens.color.bg_panel)
        .stroke(Stroke::new(1.0, tokens.color.border))
        .corner_radius(tokens.radius)
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            if compact {
                ui.vertical(|ui| {
                    action = action_buttons(ui, projection, typed, write_block, project_block);
                });
            } else {
                ui.horizontal(|ui| {
                    action = action_buttons(ui, projection, typed, write_block, project_block);
                });
            }
        });
    ui.ctx().accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Toolbar);
        node.set_label("Symbol and component form actions");
    });

    match action {
        Some(SymbolAction::OpenEditor) => {
            app.state.open_workspace_view(projection.reference.clone());
            Command::OpenWorkspace(Workspace::Design).execute(app);
        }
        Some(SymbolAction::EditForm) => {
            open_symbol_parameter_form_dialog_for(&mut app.state, projection.reference.clone())
        }
        Some(SymbolAction::Create) => open_create_model_bound_symbol_dialog(&mut app.state),
        Some(SymbolAction::Import) => {
            open_symbol_import_dialog_for(&mut app.state, Some(&projection.reference));
        }
        None => {}
    }
}

fn action_buttons(
    ui: &mut Ui,
    projection: &SymbolProjection,
    typed: bool,
    write_block: Option<&'static str>,
    project_block: Option<&'static str>,
) -> Option<SymbolAction> {
    let mut action = None;
    if Button::new("Open symbol editor")
        .accent()
        .show(ui)
        .clicked()
    {
        action = Some(SymbolAction::OpenEditor);
    }

    let edit_form = Button::new("Edit component form\u{2026}")
        .enabled(typed && write_block.is_none())
        .show(ui);
    if !typed {
        edit_form
            .clone()
            .on_disabled_hover_text("A valid typed symbol contract is required.");
    } else if let Some(reason) = write_block {
        edit_form.clone().on_disabled_hover_text(reason);
    }
    if edit_form.clicked() {
        action = Some(SymbolAction::EditForm);
    }

    let create = Button::new("Create symbol\u{2026}")
        .enabled(project_block.is_none())
        .show(ui);
    if let Some(reason) = project_block {
        create.clone().on_disabled_hover_text(reason);
    }
    if create.clicked() {
        action = Some(SymbolAction::Create);
    }

    let import = Button::new("Import symbol\u{2026}")
        .enabled(project_block.is_none())
        .show(ui);
    if let Some(reason) = project_block {
        import.clone().on_disabled_hover_text(reason);
    }
    if import.clicked() {
        action = Some(SymbolAction::Import);
    }

    let state = if projection.library_read_only {
        "read-only library"
    } else if projection.modified {
        "view has unsaved changes"
    } else {
        "view synchronized"
    };
    ui.label(state);
    action
}

fn project_write_block_reason(app: &RSpiceApp) -> Option<&'static str> {
    if !app.state.project_lifecycle.project_open {
        Some("Open a project before authoring symbols.")
    } else if app.state.workbench.safe_mode.project_read_only() {
        Some("The project is read-only in local safe mode.")
    } else if !app
        .state
        .library_manager
        .libraries_sorted()
        .into_iter()
        .any(|library| !library.read_only)
    {
        Some("No writable project design library is available.")
    } else {
        None
    }
}

fn symbol_write_block_reason(
    app: &RSpiceApp,
    projection: &SymbolProjection,
) -> Option<&'static str> {
    project_write_block_reason(app).or_else(|| {
        projection
            .library_read_only
            .then_some("The selected symbol belongs to a read-only library.")
    })
}

fn typed_symbol_surface(
    ui: &mut Ui,
    definition: &ModelBoundSymbolDefinition,
    projection: &SymbolProjection,
) {
    let document = definition.symbol_document();
    let ports = definition
        .pins
        .iter()
        .map(|pin| pin.port_spec())
        .collect::<Vec<_>>();
    let split = ui.available_width() >= SPLIT_BREAKPOINT;

    if split {
        ui.columns(2, |columns| {
            symbol_preview_card(
                &mut columns[0],
                &document,
                &ports,
                &projection.reference.cell,
            );
            contract_identity_card(&mut columns[1], definition, projection);
        });
    } else {
        symbol_preview_card(ui, &document, &ports, &projection.reference.cell);
        ui.add_space(10.0);
        contract_identity_card(ui, definition, projection);
    }

    ui.add_space(10.0);
    pin_contract_table(ui, definition);
    ui.add_space(10.0);
    parameter_form_table(ui, definition);
}

fn symbol_preview_card(
    ui: &mut Ui,
    document: &SymbolDocument,
    ports: &[crate::state::PortSpec],
    cell: &str,
) {
    let tokens = Tokens::get(ui.ctx());
    section_card(
        ui,
        "Authored symbol preview",
        "The exact authored symbol document rendered by the Design editor.",
        |ui| {
            let height = if ui.available_width() <= COMPACT_BREAKPOINT {
                210.0
            } else {
                270.0
            };
            let (rect, response) =
                ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
            ui.painter()
                .rect_filled(rect, tokens.radius, tokens.color.canvas_bg);
            ui.painter().rect_stroke(
                rect,
                tokens.radius,
                Stroke::new(1.0, tokens.color.border_strong),
                egui::StrokeKind::Inside,
            );
            crate::schematic::symbol_editor::draw_document_preview(
                ui.painter(),
                rect.shrink(12.0),
                document,
                ports,
                cell,
                tokens.color.symbol,
            );
            response.widget_info(|| {
                WidgetInfo::labeled(
                    WidgetType::Other,
                    ui.is_enabled(),
                    "Authored symbol preview",
                )
            });
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::Image);
                node.set_label(format!(
                    "Authored symbol preview for {cell}; {} shapes and {} terminals",
                    document.body.len(),
                    document.pins.len()
                ));
            });
        },
    );
}

fn contract_identity_card(
    ui: &mut Ui,
    definition: &ModelBoundSymbolDefinition,
    projection: &SymbolProjection,
) {
    let validation = definition.validate();
    section_card(
        ui,
        "Symbol contract",
        "Stable identity, electrical source, generated views, and executable netlist binding.",
        |ui| {
            Grid::new("library-cellview-symbol-identity")
                .num_columns(2)
                .spacing(vec2(14.0, 7.0))
                .show(ui, |ui| {
                    property(ui, "Cellview", &projection.reference.display_path());
                    property(
                        ui,
                        "Identity revision",
                        &format!("r{}", definition.identity.revision),
                    );
                    property(ui, "Binding ID", &definition.identity.binding_id);
                    property(ui, "Source", source_contract_label(&definition.source));
                    property(
                        ui,
                        "Netlist",
                        if definition.netlist.is_executable() {
                            "Executable typed template"
                        } else {
                            "Review-only · no executable template"
                        },
                    );
                    property(ui, "Generated views", &generated_views_label(definition));
                    property(
                        ui,
                        "Validation",
                        validation.as_ref().map(|()| "Valid").unwrap_or("Invalid"),
                    );
                });
            if let Err(error) = validation {
                validation_message(ui, &error.to_string(), true);
            } else {
                validation_message(
                    ui,
                    "Typed source, terminal order, component form, and netlist contract agree.",
                    false,
                );
            }
        },
    );
}

fn source_contract_label(source: &SymbolSourceContract) -> &str {
    match source {
        SymbolSourceContract::Model { .. } => "Model-bound",
        SymbolSourceContract::ExistingSchematicPins { .. } => "Existing schematic pins",
        SymbolSourceContract::BlankExplicitContract => "Blank explicit review contract",
    }
}

fn generated_views_label(definition: &ModelBoundSymbolDefinition) -> String {
    let mut views = Vec::new();
    if definition.generated_views.symbol {
        views.push("symbol");
    }
    if definition.generated_views.parameter_form {
        views.push("component form");
    }
    if definition.generated_views.simulation_test_fixture {
        views.push("simulation fixture");
    }
    views.join(", ")
}

fn property(ui: &mut Ui, label: &str, value: &str) {
    let tokens = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(tokens.color.text_faint),
    );
    ui.label(
        RichText::new(value)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(tokens.color.text),
    );
    ui.end_row();
}

fn validation_message(ui: &mut Ui, message: &str, error: bool) {
    let tokens = Tokens::get(ui.ctx());
    Frame::new()
        .fill(if error {
            tokens.color.err.gamma_multiply(0.12)
        } else {
            tokens.color.ok.gamma_multiply(0.10)
        })
        .stroke(Stroke::new(
            1.0,
            if error {
                tokens.color.err
            } else {
                tokens.color.ok
            },
        ))
        .corner_radius(tokens.radius)
        .inner_margin(Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    RichText::new(message)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(if error {
                            tokens.color.err
                        } else {
                            tokens.color.text
                        }),
                )
                .wrap(),
            );
        });
}

fn pin_contract_table(ui: &mut Ui, definition: &ModelBoundSymbolDefinition) {
    section_card(
        ui,
        "Terminal contract",
        "One-based positional order is the authoritative SPICE terminal order; direction and electrical type drive editor checks.",
        |ui| {
            if definition.pins.is_empty() {
                ui.label("No electrical terminals are declared; this symbol is review-only.");
                return;
            }
            let mut pins = definition.pins.iter().collect::<Vec<_>>();
            pins.sort_by_key(|pin| pin.order);
            ScrollArea::horizontal()
                .id_salt("library-cellview-terminal-table")
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.set_min_width(PIN_TABLE_MIN_WIDTH);
                    Grid::new("library-cellview-terminal-grid")
                        .striped(true)
                        .num_columns(5)
                        .min_col_width(100.0)
                        .spacing(vec2(18.0, 7.0))
                        .show(ui, |ui| {
                            table_header(ui, "Pin");
                            table_header(ui, "Direction");
                            table_header(ui, "Electrical type");
                            table_header(ui, "Side");
                            table_header(ui, "Order");
                            ui.end_row();
                            for pin in pins {
                                table_cell(ui, &pin.name, true);
                                table_cell(ui, direction_label(pin.direction), false);
                                table_cell(ui, pin.electrical_type.label(), false);
                                table_cell(ui, pin.side.label(), false);
                                table_cell(ui, &pin.order.to_string(), true);
                                ui.end_row();
                            }
                        });
                });
        },
    );
}

fn parameter_form_table(ui: &mut Ui, definition: &ModelBoundSymbolDefinition) {
    let field_count = definition.parameter_form.fields().count();
    section_card(
        ui,
        "Component form and inheritance",
        &format!(
            "Form revision r{} · {} section{} · {} field{} · exact emitted order: {}",
            definition.parameter_form.revision,
            definition.parameter_form.sections.len(),
            plural(definition.parameter_form.sections.len()),
            field_count,
            plural(field_count),
            emitted_parameter_order(definition),
        ),
        |ui| {
            if field_count == 0 {
                ui.label("This typed symbol declares no instance-editable component-form fields.");
                return;
            }
            ScrollArea::horizontal()
                .id_salt("library-cellview-component-form-table")
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.set_min_width(FORM_TABLE_MIN_WIDTH);
                    Grid::new("library-cellview-component-form-grid")
                        .striped(true)
                        .num_columns(8)
                        .min_col_width(92.0)
                        .spacing(vec2(16.0, 7.0))
                        .show(ui, |ui| {
                            for heading in [
                                "Section",
                                "Parameter",
                                "Type",
                                "Default",
                                "Unit",
                                "Inheritance",
                                "Visibility",
                                "Required",
                            ] {
                                table_header(ui, heading);
                            }
                            ui.end_row();
                            for section in &definition.parameter_form.sections {
                                for field in &section.fields {
                                    table_cell(ui, &section.label, false);
                                    table_cell(ui, &field.key, true);
                                    table_cell(ui, property_type_label(field.property_type), false);
                                    table_cell(ui, &field.default.display_string(), true);
                                    table_cell(ui, field.unit.as_deref().unwrap_or("—"), true);
                                    table_cell(ui, inheritance_label(field.inheritance), false);
                                    table_cell(ui, visibility_label(field.visibility), false);
                                    table_cell(
                                        ui,
                                        if field.required {
                                            "required"
                                        } else {
                                            "optional"
                                        },
                                        false,
                                    );
                                    ui.end_row();
                                }
                            }
                        });
                });
            let diagnostics = definition.parameter_form.validate_diagnostics();
            if diagnostics.is_empty() {
                validation_message(
                    ui,
                    "Form keys, typed defaults, constraints, inheritance, and emitted parameter order are valid.",
                    false,
                );
            } else {
                validation_message(
                    ui,
                    &format!(
                        "{} form diagnostic{} must be resolved before publication.",
                        diagnostics.len(),
                        plural(diagnostics.len())
                    ),
                    true,
                );
            }
        },
    );
}

fn emitted_parameter_order(definition: &ModelBoundSymbolDefinition) -> String {
    let order = definition.parameter_form.netlist_parameter_order();
    if order.is_empty() {
        "none".to_owned()
    } else {
        order.join(" → ")
    }
}

fn table_header(ui: &mut Ui, value: &str) {
    let tokens = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(value)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(tokens.color.text_faint),
    );
}

fn table_cell(ui: &mut Ui, value: &str, mono: bool) {
    let tokens = Tokens::get(ui.ctx());
    let font = if mono {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    };
    ui.label(RichText::new(value).font(font).color(tokens.color.text));
}

fn direction_label(direction: PortDirection) -> &'static str {
    match direction {
        PortDirection::In => "input",
        PortDirection::Out => "output",
        PortDirection::InOut => "bidirectional",
        PortDirection::Supply => "supply",
    }
}

fn property_type_label(value: PropertyType) -> &'static str {
    match value {
        PropertyType::Number => "Number",
        PropertyType::String => "String",
        PropertyType::Expression => "Expression",
        PropertyType::Enum => "Enumeration",
        PropertyType::Boolean => "Boolean",
    }
}

fn inheritance_label(value: ParameterInheritance) -> &'static str {
    match value {
        ParameterInheritance::InstanceOverride => "Instance override",
        ParameterInheritance::CellDefault => "Cell default",
        ParameterInheritance::ModelDefault => "Model default",
    }
}

fn visibility_label(value: SymbolParameterVisibility) -> &'static str {
    match value {
        SymbolParameterVisibility::Visible => "Visible",
        SymbolParameterVisibility::Advanced => "Advanced",
        SymbolParameterVisibility::Hidden => "Hidden",
    }
}

fn legacy_symbol_surface(ui: &mut Ui, projection: &SymbolProjection) {
    match &projection.legacy_document {
        Ok(document) => {
            symbol_preview_card(ui, document, &[], &projection.reference.cell);
        }
        Err(error) => {
            invalid_symbol_surface(ui, projection, error);
            return;
        }
    }
    ui.add_space(10.0);
    section_card(
        ui,
        "Typed contract required",
        "The selected legacy symbol may be opened for graphical editing, but it has no versioned source, terminal-order, component-form, or netlist contract.",
        |ui| {
            validation_message(
                ui,
                "Create or import a model-bound symbol before form editing or governed publication. RSpice will not infer electrical semantics from drawing geometry.",
                true,
            );
        },
    );
}

fn invalid_symbol_surface(ui: &mut Ui, projection: &SymbolProjection, error: &str) {
    section_card(
        ui,
        "Invalid symbol definition",
        &format!(
            "{} contains unreadable or contract-invalid symbol metadata.",
            projection.reference.display_path()
        ),
        |ui| {
            validation_message(
                ui,
                &format!(
                    "{error} The definition is not used as an executable placement or component-form contract."
                ),
                true,
            );
        },
    );
}

fn section_card(ui: &mut Ui, title: &str, description: &str, add_body: impl FnOnce(&mut Ui)) {
    let tokens = Tokens::get(ui.ctx());
    let shown = Frame::new()
        .fill(tokens.color.bg_panel)
        .stroke(Stroke::new(1.0, tokens.color.border))
        .corner_radius(tokens.radius)
        .inner_margin(Margin::same(12))
        .show(ui, |ui| {
            ui.label(
                RichText::new(title)
                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                    .color(tokens.color.text),
            );
            ui.add(
                egui::Label::new(
                    RichText::new(description)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(tokens.color.text_dim),
                )
                .wrap(),
            );
            ui.add_space(8.0);
            add_body(ui);
        });
    ui.ctx().accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Region);
        node.set_label(title);
    });
}

const fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, GeneratedSymbolViews, Library, ParameterInheritance, PortSpec, SymbolElectricalType,
        SymbolGraphicTemplate, SymbolIdentity, SymbolNetlistBinding, SymbolParameterConstraints,
        SymbolParameterDefault, SymbolParameterField, SymbolParameterForm, SymbolParameterSection,
        SymbolParameterVisibility, SymbolPinDefinition, SymbolPinSide, SymbolSourceContract, View,
    };

    fn typed_symbol() -> ModelBoundSymbolDefinition {
        let pins = vec![
            SymbolPinDefinition::new(
                "IN",
                SymbolElectricalType::Analog,
                PortDirection::In,
                SymbolPinSide::Left,
                1,
            ),
            SymbolPinDefinition::new(
                "OUT",
                SymbolElectricalType::Analog,
                PortDirection::Out,
                SymbolPinSide::Right,
                2,
            ),
        ];
        let ports = pins.iter().map(SymbolPinDefinition::port_spec).collect();
        let form = SymbolParameterForm {
            revision: 3,
            sections: vec![SymbolParameterSection {
                key: "electrical".to_owned(),
                label: "Electrical".to_owned(),
                help: "Electrical instance parameters.".to_owned(),
                fields: vec![SymbolParameterField {
                    key: "gain".to_owned(),
                    label: "Gain".to_owned(),
                    help: "Closed-loop gain.".to_owned(),
                    property_type: PropertyType::Number,
                    default: SymbolParameterDefault::Number {
                        engineering: "10".to_owned(),
                        unit: None,
                    },
                    unit: None,
                    constraints: SymbolParameterConstraints {
                        minimum: Some("1".to_owned()),
                        maximum: Some("1k".to_owned()),
                        ..SymbolParameterConstraints::default()
                    },
                    inheritance: ParameterInheritance::InstanceOverride,
                    visibility: SymbolParameterVisibility::Visible,
                    required: true,
                    aliases: Vec::new(),
                }],
            }],
        };
        ModelBoundSymbolDefinition::new(
            SymbolIdentity::new("authoring", "amp", 7, "symbol:authoring/amp"),
            SymbolSourceContract::existing_schematic_pins("schematic", ports),
            pins,
            SymbolGraphicTemplate::RectangularIc,
            form,
            SymbolNetlistBinding {
                device_prefix: "X".to_owned(),
                model: None,
                template: "X{name} {nodes} {model} {params}".to_owned(),
                parameter_order: vec!["gain".to_owned()],
            },
            GeneratedSymbolViews::default(),
        )
    }

    fn app_with_typed_symbol() -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        let definition = typed_symbol();
        let mut view = View::new("symbol", ViewType::Symbol);
        definition
            .store_in_view(&mut view)
            .expect("typed symbol stores");
        let mut cell = Cell::new("amp");
        cell.add_view(view);
        let mut library = Library::new("authoring");
        library.add_cell(cell);
        app.state.library_manager.add_library(library);
        app.state
            .library_manager
            .select_view("authoring", "amp", "symbol");
        app.state.workbench.library_cellview_page = LibraryCellviewPage::SymbolForm;
        app
    }

    fn render(app: &mut RSpiceApp, size: egui::Vec2) -> egui::FullOutput {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, app));
            },
        )
    }

    fn access_nodes(output: &egui::FullOutput) -> Vec<&egui::accesskit::Node> {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("accessibility tree")
            .nodes
            .iter()
            .map(|(_, node)| node)
            .collect()
    }

    #[test]
    fn specialist_renders_both_pages_at_desktop_tablet_and_phone_widths() {
        for page in LibraryCellviewPage::ALL {
            for size in [vec2(1440.0, 900.0), vec2(820.0, 1180.0), vec2(390.0, 844.0)] {
                let mut app = app_with_typed_symbol();
                app.state.workbench.library_cellview_page = page;
                let output = render(&mut app, size);
                assert!(
                    !output.shapes.is_empty(),
                    "{} did not render at {}x{}",
                    page.label(),
                    size.x,
                    size.y
                );
            }
        }
    }

    #[test]
    fn symbol_page_exposes_live_preview_terminal_form_and_actions() {
        let mut app = app_with_typed_symbol();
        let output = render(&mut app, vec2(1280.0, 900.0));
        let nodes = access_nodes(&output);
        for label in [
            "Authored symbol preview",
            "Terminal contract",
            "Component form and inheritance",
            "Open symbol editor",
            "Edit component form\u{2026}",
            "Create symbol\u{2026}",
            "Import symbol\u{2026}",
        ] {
            assert!(
                nodes.iter().any(|node| node.label() == Some(label)),
                "missing accessible node {label}"
            );
        }
        assert!(
            output.shapes.len() > 20,
            "the live symbol projection is empty"
        );
    }

    #[test]
    fn exact_selected_symbol_wins_over_an_unrelated_active_document() {
        let app = app_with_typed_symbol();
        assert_ne!(
            app.state.workspace.active_view,
            CellViewRef::new("authoring", "amp", "symbol")
        );
        assert_eq!(
            resolve_symbol_reference(&app),
            Some(CellViewRef::new("authoring", "amp", "symbol"))
        );
    }

    #[test]
    fn explicit_form_action_targets_the_selected_symbol() {
        let mut app = app_with_typed_symbol();
        open_symbol_parameter_form_dialog_for(
            &mut app.state,
            CellViewRef::new("authoring", "amp", "symbol"),
        );
        assert_eq!(
            app.state
                .dialogs
                .symbol_parameter_form
                .target
                .as_ref()
                .map(CellViewRef::display_path),
            Some("authoring/amp/symbol".to_owned())
        );
        assert_eq!(
            app.state
                .dialogs
                .symbol_parameter_form
                .draft_form
                .as_ref()
                .map(|form| form.revision),
            Some(4)
        );
    }

    #[test]
    fn legacy_symbol_discloses_missing_contract_and_disables_form_editing() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("legacy");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        app.state.library_manager.add_library(library);
        app.state
            .library_manager
            .select_view("legacy", "amp", "symbol");
        app.state.workbench.library_cellview_page = LibraryCellviewPage::SymbolForm;

        let output = render(&mut app, vec2(1100.0, 760.0));
        let nodes = access_nodes(&output);
        assert!(
            nodes
                .iter()
                .any(|node| node.label() == Some("Typed contract required"))
        );
        assert!(nodes.iter().any(|node| {
            node.label() == Some("Edit component form\u{2026}") && node.is_disabled()
        }));
    }

    #[test]
    fn safe_mode_disables_every_symbol_mutation_but_keeps_editor_handoff() {
        let mut app = app_with_typed_symbol();
        app.state.workbench.safe_mode.activate(
            crate::workbench::state::LocalSafeModeOptions {
                open_project_read_only: true,
                ..crate::workbench::state::LocalSafeModeOptions::default()
            },
            "{}".to_owned(),
        );

        let output = render(&mut app, vec2(1100.0, 760.0));
        let nodes = access_nodes(&output);
        for label in [
            "Edit component form\u{2026}",
            "Create symbol\u{2026}",
            "Import symbol\u{2026}",
        ] {
            assert!(
                nodes
                    .iter()
                    .any(|node| node.label() == Some(label) && node.is_disabled()),
                "{label} remained enabled in safe mode"
            );
        }
        assert!(
            nodes
                .iter()
                .any(|node| { node.label() == Some("Open symbol editor") && !node.is_disabled() })
        );
    }

    #[test]
    fn typed_fixture_is_a_valid_real_contract() {
        let definition = typed_symbol();
        definition.validate().expect("fixture validates");
        assert_eq!(
            definition
                .pins
                .iter()
                .map(|pin| (pin.name.as_str(), pin.order))
                .collect::<Vec<_>>(),
            [("IN", 1), ("OUT", 2)]
        );
        assert_eq!(
            definition.parameter_form.netlist_parameter_order(),
            ["gain"]
        );
        assert_eq!(
            definition.source,
            SymbolSourceContract::ExistingSchematicPins {
                schematic_view: "schematic".to_owned(),
                ports: vec![
                    PortSpec {
                        name: "IN".to_owned(),
                        direction: PortDirection::In,
                    },
                    PortSpec {
                        name: "OUT".to_owned(),
                        direction: PortDirection::Out,
                    },
                ],
            }
        );
    }
}
