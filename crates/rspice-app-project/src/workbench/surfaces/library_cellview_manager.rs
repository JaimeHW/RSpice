//! Project-owned library, cellview, symbol, and component-form authoring.
//!
//! The canonical specialist route deliberately composes existing authoritative
//! owners. The Libraries page reuses the Project workspace's exact
//! three-column browser; the Symbol & form page projects the selected
//! `ModelBoundSymbolDefinition` and the same `SymbolDocument` used by the
//! Design editor. No parallel catalog, symbol, form, or mutation state lives
//! here.

use std::sync::Arc;

use egui::{
    Align, Frame, Grid, Key, Layout, Margin, Modifiers, Rect, RichText, ScrollArea, Sense, Stroke,
    Ui, Vec2, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::state::{
    CellViewRef, ModelBoundSymbolDefinition, ParameterInheritance, PortDirection, PropertyType,
    SymbolDocument, SymbolParameterVisibility, SymbolSourceContract, ViewType,
};
use crate::ui::accessibility::plural_suffix;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::app_state::Model;
use crate::workbench::design_system::{Problem, paint_problem_mark, problem_line};
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::requests::OpenSymbolAuthoring;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::project::workspace_state::LibraryCellviewPage;
use crate::workbench::surfaces::project::library;

const COMPACT_BREAKPOINT: f32 = 820.0;
const SPLIT_BREAKPOINT: f32 = 960.0;
/// Side of a header chip's problem mark: the kit's own, so it matches every
/// other marked line in the product.
use crate::ui::widgets::PROBLEM_MARK_SIDE as HEADER_CHIP_MARK_SIDE;
const TAB_HEIGHT: f32 = 34.0;
const PIN_TABLE_MIN_WIDTH: f32 = 690.0;
const FORM_TABLE_MIN_WIDTH: f32 = 900.0;

const SURFACE_TITLE: &str = "Library, cellview, symbol and form authoring";

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Times this thread projected the catalog for the symbol the manager's
    /// Symbol & form page shows.
    pub static CELLVIEW_PROJECTION_BUILDS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

fn count_projection_build() {
    #[cfg(any(test, feature = "test-support"))]
    CELLVIEW_PROJECTION_BUILDS.with(|builds| builds.set(builds.get() + 1));
}

/// The symbol the Symbol & form page shows, decoded, checked and drawn once
/// for the catalog content and the selection it was resolved from. Whether
/// the view has unsaved changes is read as the page draws: an editor marks
/// that without changing the catalog.
struct SymbolProjection {
    reference: CellViewRef,
    library_read_only: bool,
    /// The typed definition, if the view carries one, or why it cannot be
    /// read.
    definition: Result<Option<TypedSymbol>, String>,
    legacy_document: Result<SymbolDocument, String>,
}

/// A typed definition and what the page derives from it.
struct TypedSymbol {
    definition: ModelBoundSymbolDefinition,
    document: SymbolDocument,
    ports: Vec<crate::state::PortSpec>,
    validation: Result<(), String>,
    form_errors: usize,
    netlist_order: String,
}

/// What the projection was resolved from: the catalog content, the
/// browser's selection and the active view.
struct ProjectionKey {
    stamp: u64,
    library: Option<String>,
    cell: Option<String>,
    view: Option<String>,
    active: CellViewRef,
}

impl ProjectionKey {
    fn of(model: &Model) -> Self {
        let manager = &model.library_manager;
        Self {
            stamp: manager.catalog().stamp(),
            library: manager.selected_library.clone(),
            cell: manager.selected_cell.clone(),
            view: manager.selected_view.clone(),
            active: model.workspace.content.active_view.clone(),
        }
    }

    fn holds(&self, model: &Model) -> bool {
        let manager = &model.library_manager;
        self.stamp == manager.catalog().stamp()
            && self.library == manager.selected_library
            && self.cell == manager.selected_cell
            && self.view == manager.selected_view
            && self.active == model.workspace.content.active_view
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymbolAction {
    OpenEditor,
    EditForm,
    Create,
    Import,
}

/// The manager in the project feature's context: its identity, its page
/// tabs, then the page in front, which is the page it returns. The
/// Libraries page is the project's own Library browser; the shell draws
/// that page's dialogs once what the manager raised has been answered.
pub fn show(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) -> LibraryCellviewPage {
    let tokens = Tokens::get(ui.ctx());
    Frame::new()
        .fill(tokens.color.bg_app)
        .show(ui, |ui| {
            surface_header(ui, cx);
            page_tabs(ui, cx);

            let page = cx.state.library_cellview_page;
            let panel = ui.scope(|ui| match page {
                LibraryCellviewPage::Libraries => library::page(ui, cx),
                LibraryCellviewPage::SymbolForm => symbol_form_page(ui, cx),
            });
            ui.ctx().accesskit_node_builder(panel.response.id, |node| {
                node.set_role(egui::accesskit::Role::TabPanel);
                node.set_label(format!("Library Cellview Manager {}", page.label()));
            });
            page
        })
        .inner
}

fn surface_header(ui: &mut Ui, cx: &Cx<'_, ProjectFeature>) {
    let tokens = Tokens::get(ui.ctx());
    let compact = ui.available_width() <= COMPACT_BREAKPOINT;
    let shown = Frame::new()
        .fill(tokens.color.bg_panel)
        .inner_margin(Margin::symmetric(12, 9))
        .show(ui, |ui| {
            let access = project_access(cx);
            if compact {
                identity_block(ui, &tokens);
                if let Some((value, problem)) = access {
                    ui.add_space(6.0);
                    header_chip(ui, "Access", value, problem, &tokens);
                }
            } else {
                ui.horizontal(|ui| {
                    identity_block(ui, &tokens);
                    if let Some((value, problem)) = access {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            header_chip(ui, "Access", value, problem, &tokens);
                        });
                    }
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
    ui.label(
        RichText::new(SURFACE_TITLE)
            .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
            .color(tokens.color.text),
    );
}

/// What the reader cannot do here, if anything. An open, writable project is
/// the ordinary case and says nothing.
fn project_access(cx: &Cx<'_, ProjectFeature>) -> Option<(&'static str, Option<Problem>)> {
    if !cx.platform.project_lifecycle.is_open() {
        Some(("closed", None))
    } else if cx.model.safe_mode.project_read_only() {
        Some(("safe-mode read only", Some(Problem::Warning)))
    } else {
        None
    }
}

fn header_chip(ui: &mut Ui, label: &str, value: &str, problem: Option<Problem>, tokens: &Tokens) {
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
                if let Some(problem) = problem {
                    let (mark, _) = ui.allocate_exact_size(
                        Vec2::splat(HEADER_CHIP_MARK_SIDE),
                        egui::Sense::hover(),
                    );
                    paint_problem_mark(ui.painter(), mark, problem);
                }
                ui.label(
                    RichText::new(value)
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(tokens.color.text_dim),
                );
            });
        });
}

fn page_tabs(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
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
                    let selected = cx.state.library_cellview_page == page;
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
                        cx.state.library_cellview_page = page;
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
                            cx.state.library_cellview_page = LibraryCellviewPage::ALL[target];
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

fn symbol_form_page(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    let projection = symbol_projection(ui.ctx(), cx.model);
    ScrollArea::vertical()
        .id_salt("library-cellview-symbol-form")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            symbol_context(ui, cx, projection.as_deref());
            ui.add_space(10.0);
            match projection.as_deref() {
                None => no_symbol_state(ui, cx),
                Some(projection) => selected_symbol_surface(ui, cx, projection),
            }
            ui.add_space(16.0);
        });
}

/// The symbol the page shows, remembered in the context's memory while the
/// catalog content and the selection it was resolved from stand.
fn symbol_projection(ctx: &egui::Context, model: &Model) -> Option<Arc<SymbolProjection>> {
    type Remembered = Arc<(ProjectionKey, Option<Arc<SymbolProjection>>)>;
    let id = egui::Id::new("library-cellview-symbol-projection");
    if let Some(remembered) = ctx.data(|data| data.get_temp::<Remembered>(id))
        && remembered.0.holds(model)
    {
        return remembered.1.clone();
    }
    count_projection_build();
    let projection = project_symbol(model).map(Arc::new);
    let remembered: Remembered = Arc::new((ProjectionKey::of(model), projection.clone()));
    ctx.data_mut(|data| data.insert_temp(id, remembered));
    projection
}

fn project_symbol(model: &Model) -> Option<SymbolProjection> {
    let reference = resolve_symbol_reference(model)?;
    let library = model.library_manager.get_library(&reference.library)?;
    let view = library
        .get_cell(&reference.cell)?
        .get_view(&reference.view)?;
    let definition = crate::state::load_model_bound_symbol(view)
        .map(|definition| {
            definition.map(|definition| TypedSymbol {
                document: crate::state::materialize_symbol_document(&definition),
                ports: definition.pins.iter().map(|pin| pin.port_spec()).collect(),
                validation: definition.validate().map_err(|error| error.to_string()),
                form_errors: definition.parameter_form.validate_diagnostics().len(),
                netlist_order: emitted_parameter_order(&definition),
                definition,
            })
        })
        .map_err(|error| error.to_string());
    Some(SymbolProjection {
        reference,
        library_read_only: library.read_only,
        definition,
        legacy_document: SymbolDocument::load_from_view(view),
    })
}

pub fn resolve_symbol_reference(model: &Model) -> Option<CellViewRef> {
    let manager = &model.library_manager;
    let selected = manager.current_library().and_then(|library| {
        let cell = manager.current_cell()?;
        let view = manager.current_view()?;
        (view.view_type == ViewType::Symbol)
            .then(|| CellViewRef::new(&library.name, &cell.name, &view.name))
    });
    if selected.is_some() {
        return selected;
    }

    let active = &model.workspace.content.active_view;
    let active_is_symbol = manager
        .get_library(&active.library)
        .and_then(|library| library.get_cell(&active.cell))
        .and_then(|cell| cell.get_view(&active.view))
        .is_some_and(|view| view.view_type == ViewType::Symbol);
    if active_is_symbol {
        return Some(active.clone());
    }

    first_symbol_reference(model, true).or_else(|| first_symbol_reference(model, false))
}

fn first_symbol_reference(model: &Model, typed_only: bool) -> Option<CellViewRef> {
    model
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
                        && !matches!(crate::state::load_model_bound_symbol(view), Ok(Some(_)))
                    {
                        return None;
                    }
                    Some(CellViewRef::new(&library.name, &cell.name, &view.name))
                })
            })
        })
        .next()
}

fn symbol_context(ui: &mut Ui, cx: &Cx<'_, ProjectFeature>, projection: Option<&SymbolProjection>) {
    let tokens = Tokens::get(ui.ctx());
    let selection = projection
        .map(|projection| projection.reference.display_path())
        .unwrap_or_else(|| "No symbol view available".to_owned());
    // With no view selected the Selection value already says so, and a valid
    // contract is the case that needs no words. The rest are what is wrong
    // with the contract this view does carry.
    let contract = projection.and_then(|projection| match &projection.definition {
        Ok(Some(typed)) if typed.validation.is_ok() => None,
        Ok(Some(_)) => Some(("typed contract invalid", Problem::Failure)),
        Ok(None) => Some(("No typed contract", Problem::Warning)),
        Err(_) => Some(("typed metadata unreadable", Problem::Failure)),
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
                        context_value(ui, "Selection", &selection, None, &tokens);
                        if let Some((value, problem)) = contract {
                            ui.separator();
                            context_value(ui, "Contract", value, Some(problem), &tokens);
                        }
                        ui.separator();
                        context_value(
                            ui,
                            "Project",
                            &format!("r{}", cx.model.workspace.content.project.revision().get()),
                            None,
                            &tokens,
                        );
                        ui.separator();
                        context_value(
                            ui,
                            "Catalog",
                            &format!("r{}", cx.model.library_manager.revision()),
                            None,
                            &tokens,
                        );
                    });
                });
        });
}

fn context_value(ui: &mut Ui, label: &str, value: &str, problem: Option<Problem>, tokens: &Tokens) {
    ui.label(
        RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(tokens.color.text_faint),
    );
    if let Some(problem) = problem {
        let (mark, _) =
            ui.allocate_exact_size(Vec2::splat(HEADER_CHIP_MARK_SIDE), egui::Sense::hover());
        paint_problem_mark(ui.painter(), mark, problem);
    }
    ui.label(
        RichText::new(value)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(tokens.color.text_dim),
    );
}

fn no_symbol_state(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    let tokens = Tokens::get(ui.ctx());
    section_card(
        ui,
        "No symbol view is available",
        "Create or import a model-bound symbol to establish an explicit terminal, source, component-form, and netlist contract.",
        |ui| {
            let block = project_write_block_reason(cx);
            let create = Button::new("Create model-bound symbol\u{2026}")
                .accent()
                .enabled(block.is_none())
                .show(ui);
            if let Some(reason) = block {
                create.clone().on_disabled_hover_text(reason);
            }
            if create.clicked() {
                cx.wb.outbox.push(OpenSymbolAuthoring::ModelBound);
            }
            let import = Button::new("Import symbol\u{2026}")
                .enabled(block.is_none())
                .show(ui);
            if let Some(reason) = block {
                import.clone().on_disabled_hover_text(reason);
            }
            if import.clicked() {
                cx.wb.outbox.push(OpenSymbolAuthoring::Import(None));
            }
        },
    );
    ui.label(
        RichText::new("No placeholder symbol or inferred electrical interface is rendered.")
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(tokens.color.text_faint),
    );
}

fn selected_symbol_surface(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    projection: &SymbolProjection,
) {
    symbol_actions(ui, cx, projection);
    ui.add_space(10.0);

    match &projection.definition {
        Ok(Some(typed)) => typed_symbol_surface(ui, typed, projection),
        Ok(None) => legacy_symbol_surface(ui, projection),
        Err(error) => invalid_symbol_surface(ui, projection, error),
    }
}

fn symbol_actions(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>, projection: &SymbolProjection) {
    let tokens = Tokens::get(ui.ctx());
    let typed = projection.definition.as_ref().is_ok_and(Option::is_some);
    let modified = cx
        .model
        .library_manager
        .get_library(&projection.reference.library)
        .and_then(|library| library.get_cell(&projection.reference.cell))
        .and_then(|cell| cell.get_view(&projection.reference.view))
        .is_some_and(|view| view.modified);
    let write_block = symbol_write_block_reason(cx, projection);
    let project_block = project_write_block_reason(cx);
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
                    action =
                        action_buttons(ui, projection, modified, typed, write_block, project_block);
                });
            } else {
                ui.horizontal(|ui| {
                    action =
                        action_buttons(ui, projection, modified, typed, write_block, project_block);
                });
            }
        });
    ui.ctx().accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Toolbar);
        node.set_label("Symbol and component form actions");
    });

    // The symbol editor and the symbol dialogs are raised for the shell,
    // which opens them as the manager returns: the editor on the symbol
    // shown, in Design, and the form and the importer naming it.
    let raised = match action {
        Some(SymbolAction::OpenEditor) => {
            cx.wb.outbox.push(ProjectRequest::OpenView(
                projection.reference.clone(),
                ViewType::Symbol,
            ));
            None
        }
        Some(SymbolAction::EditForm) => Some(OpenSymbolAuthoring::ParameterForm(Some(
            projection.reference.clone(),
        ))),
        Some(SymbolAction::Create) => Some(OpenSymbolAuthoring::ModelBound),
        Some(SymbolAction::Import) => Some(OpenSymbolAuthoring::Import(Some(
            projection.reference.clone(),
        ))),
        None => None,
    };
    if let Some(request) = raised {
        cx.wb.outbox.push(request);
    }
}

fn action_buttons(
    ui: &mut Ui,
    projection: &SymbolProjection,
    modified: bool,
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
    } else if modified {
        "view has unsaved changes"
    } else {
        "view synchronized"
    };
    ui.label(state);
    action
}

fn project_write_block_reason(cx: &Cx<'_, ProjectFeature>) -> Option<&'static str> {
    if !cx.platform.project_lifecycle.is_open() {
        Some("Open a project before authoring symbols.")
    } else if cx.model.safe_mode.project_read_only() {
        Some("The project is read-only in local safe mode.")
    } else if !cx
        .model
        .library_manager
        .libraries_by_key()
        .any(|(_, library)| !library.read_only)
    {
        Some("No writable project design library is available.")
    } else {
        None
    }
}

fn symbol_write_block_reason(
    cx: &Cx<'_, ProjectFeature>,
    projection: &SymbolProjection,
) -> Option<&'static str> {
    project_write_block_reason(cx).or_else(|| {
        projection
            .library_read_only
            .then_some("The selected symbol belongs to a read-only library.")
    })
}

fn typed_symbol_surface(ui: &mut Ui, typed: &TypedSymbol, projection: &SymbolProjection) {
    let definition = &typed.definition;
    let document = &typed.document;
    let ports = &typed.ports;
    let split = ui.available_width() >= SPLIT_BREAKPOINT;

    if split {
        ui.columns(2, |columns| {
            symbol_preview_card(&mut columns[0], document, ports, &projection.reference.cell);
            contract_identity_card(&mut columns[1], definition, typed, projection);
        });
    } else {
        symbol_preview_card(ui, document, ports, &projection.reference.cell);
        ui.add_space(10.0);
        contract_identity_card(ui, definition, typed, projection);
    }

    ui.add_space(10.0);
    pin_contract_table(ui, definition);
    ui.add_space(10.0);
    parameter_form_table(ui, definition, typed);
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
        "The saved symbol document, as the Design editor draws it.",
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
    typed: &TypedSymbol,
    projection: &SymbolProjection,
) {
    let validation = &typed.validation;
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
                validation_message(ui, error);
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

/// One refusal under a card: its mark, then the words in the ordinary tone.
///
/// There is no success variant. A card whose contract agrees with its sources
/// used to carry a green frame saying so; the Validation row above already
/// reads "Valid", and nothing here is in doubt.
fn validation_message(ui: &mut Ui, message: &str) {
    Frame::new()
        .inner_margin(Margin::symmetric(8, 6))
        .show(ui, |ui| {
            problem_line(ui, Problem::Failure, message);
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

fn parameter_form_table(ui: &mut Ui, definition: &ModelBoundSymbolDefinition, typed: &TypedSymbol) {
    let field_count = definition.parameter_form.fields().count();
    section_card(
        ui,
        "Component form and inheritance",
        &format!(
            "Form revision r{} · {} section{} · {} field{} · netlist order: {}",
            definition.parameter_form.revision,
            definition.parameter_form.sections.len(),
            plural_suffix(definition.parameter_form.sections.len()),
            field_count,
            plural_suffix(field_count),
            typed.netlist_order,
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
            if typed.form_errors > 0 {
                validation_message(
                    ui,
                    &format!(
                        "{} form error{}",
                        typed.form_errors,
                        plural_suffix(typed.form_errors)
                    ),
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
            // Not a refusal of anything the reader did: it is the route out
            // of the state the card above already described.
            let tokens = Tokens::get(ui.ctx());
            ui.label(
                RichText::new("Create or import a model-bound symbol to edit its form.")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(tokens.color.text_dim),
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
            validation_message(ui, error);
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

#[cfg(any(test, feature = "test-support"))]
pub mod tests {
    use super::*;
    #[cfg(test)]
    use crate::state::{Cell, Library, PortSpec, View};
    use crate::state::{
        GeneratedSymbolViews, ParameterInheritance, SymbolElectricalType, SymbolGraphicTemplate,
        SymbolIdentity, SymbolNetlistBinding, SymbolParameterConstraints, SymbolParameterDefault,
        SymbolParameterField, SymbolParameterForm, SymbolParameterSection,
        SymbolParameterVisibility, SymbolPinDefinition, SymbolPinSide, SymbolSourceContract,
    };
    #[cfg(test)]
    use crate::workbench::framework::feature::harness::FeatureHarness;
    #[cfg(test)]
    use crate::workbench::project::tests::harness;

    pub fn typed_symbol() -> ModelBoundSymbolDefinition {
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

    /// The project feature over a session holding one library, `authoring`,
    /// whose `amp` cell's typed symbol view is selected, with the Symbol &
    /// form page in front.
    #[cfg(test)]
    fn harness_with_typed_symbol() -> FeatureHarness<ProjectFeature> {
        let mut harness = harness();
        let definition = typed_symbol();
        let mut view = View::new("symbol", ViewType::Symbol);
        crate::state::store_model_bound_symbol(&definition, &mut view)
            .expect("typed symbol stores");
        let mut cell = Cell::new("amp");
        cell.add_view(view);
        let mut library = Library::new("authoring");
        library.add_cell(cell);
        harness.model.library_manager.add_library(library);
        harness
            .model
            .library_manager
            .select_view("authoring", "amp", "symbol");
        harness.state.library_cellview_page = LibraryCellviewPage::SymbolForm;
        harness
    }

    /// One frame of the manager alone, in the feature's context, at `size`.
    #[cfg(test)]
    fn render(harness: &mut FeatureHarness<ProjectFeature>, size: Vec2) -> egui::FullOutput {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut harness.cx()));
            },
        )
    }

    #[cfg(test)]
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
                let mut harness = harness_with_typed_symbol();
                harness.state.library_cellview_page = page;
                let output = render(&mut harness, size);
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
        let mut harness = harness_with_typed_symbol();
        let output = render(&mut harness, vec2(1280.0, 900.0));
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
        let harness = harness_with_typed_symbol();
        assert_ne!(
            harness.model.workspace.content.active_view,
            CellViewRef::new("authoring", "amp", "symbol")
        );
        assert_eq!(
            resolve_symbol_reference(&harness.model),
            Some(CellViewRef::new("authoring", "amp", "symbol"))
        );
    }

    #[test]
    fn legacy_symbol_discloses_missing_contract_and_disables_form_editing() {
        let mut harness = harness();
        let mut library = Library::new("legacy");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        harness.model.library_manager.add_library(library);
        harness
            .model
            .library_manager
            .select_view("legacy", "amp", "symbol");
        harness.state.library_cellview_page = LibraryCellviewPage::SymbolForm;

        let output = render(&mut harness, vec2(1100.0, 760.0));
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
        let mut harness = harness_with_typed_symbol();
        harness.model.safe_mode.activate(
            crate::state::write_policy::LocalSafeModeOptions {
                open_project_read_only: true,
                ..crate::state::write_policy::LocalSafeModeOptions::default()
            },
            "{}".to_owned(),
        );

        let output = render(&mut harness, vec2(1100.0, 760.0));
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
}
