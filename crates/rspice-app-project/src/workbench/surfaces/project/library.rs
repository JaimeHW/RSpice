//! Dense project library browser.
//!
//! The surface projects the persisted design-library manager directly: each
//! visible row names a real library, cell, or view, selection writes back to
//! that manager, and opening a view uses the normal workspace transaction.

pub use super::*;
use std::sync::Arc;

use crate::state::shortcuts::ShortcutStroke;
use crate::state::{CellViewRef, LibraryManager, ViewType};
use crate::workbench::app_state::Model;
use crate::workbench::design_system::elide_text;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::{LibraryCommand, ProjectCommand, ProjectRequest};
use crate::workbench::project::workspace_state::LibraryDeletionTarget;
use crate::workbench::slots::ProjectState;
use rspice_app_workbench::workbench::app_state::session::shortcuts::runtime_command_platform;

pub mod builtin_models;
mod columns;

use columns::column_head_height;
pub use columns::{BrowserColumn, LibraryBrowserState};

const MIN_BROWSER_WIDTH: f32 = 680.0;
const USAGE_REPORT_ID: &str = "workbench.project.library.where-used";
pub const AUDIT_REPORT_ID: &str = "workbench.project.library.audit-history";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryIntent {
    BuiltinModels(builtin_models::BuiltinModelsIntent),
    SelectLibrary(String),
    SelectCell {
        library: String,
        cell: String,
    },
    SelectView(CellViewRef),
    OpenView(CellViewRef, ViewType),
    /// The cell's default view: the one a root's schematic is found by.
    OpenCell {
        library: String,
        cell: String,
    },
    /// The manager's Symbol & form page, on this symbol view.
    OpenSymbolForm(CellViewRef),
    ReportUsage(CellViewRef),
    NewLibrary,
    RenameLibrary(String),
    DeleteLibrary(String),
    NewCell,
    NewView {
        library: String,
        cell: String,
    },
    CopyCell {
        library: String,
        cell: String,
    },
    /// Copy Cell into `target`, the library the cell was dragged onto.
    CopyCellInto {
        library: String,
        cell: String,
        target: String,
    },
    /// Copy Cell of a built-in model into `target`, the library it was
    /// dragged onto.
    CopyModelInto {
        registry_name: &'static str,
        target: String,
    },
    RenameCell {
        library: String,
        cell: String,
    },
    /// Make the cell the project's top cell.
    SetProjectTop {
        library: String,
        cell: String,
    },
    DeleteCell {
        library: String,
        cell: String,
    },
    RenameView(CellViewRef),
    DeleteView(CellViewRef),
    /// One review of a multi-selection's cells or views.
    DeleteSeveral(Vec<LibraryDeletionTarget>),
    ShowAuditHistory,
    PublishProject,
    RestoreFromPublication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LibraryUsageReport {
    reference: CellViewRef,
    /// Each placement: the schematic it is on and the instance's name.
    placements: Vec<(String, String)>,
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Times this thread listed and sorted a column of the library browser.
    pub static LIBRARY_ROW_BUILDS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The Library page: the browser, then the open built-in source and the
/// detail, and what the reader asked of each carried out as its call
/// returns, or raised for the shell when it opens a view or runs a command.
/// The detail reads the selection the browser's intent writes, so that
/// intent is carried out before the detail draws. Then the reports.
pub fn page(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    let ctx = ui.ctx().clone();
    // What a command asked of the page since it last drew.
    if let Some(intent) = cx.state.library_browser.queued.take() {
        apply_library_intent(&ctx, cx, intent);
    }
    let (intent, source_height) = library_browser(ui, cx);
    if let Some(intent) = intent {
        apply_library_intent(&ctx, cx, intent);
    }
    if cx.state.builtin_models.open {
        if let Some(intent) = builtin_source(ui, cx, source_height) {
            apply_library_intent(&ctx, cx, intent);
        }
        if let Some(intent) = builtin_detail(ui, cx) {
            apply_library_intent(&ctx, cx, intent);
        }
    } else if let Some(intent) = library_detail(ui, cx) {
        apply_library_intent(&ctx, cx, intent);
    }
    show_library_usage_report(&ctx);
    show_library_audit_history(&ctx, cx.model);
}

/// The height of the strip over the columns and of the band under them: one
/// control and the bar's padding above and below it, under the live metrics.
fn bar_height(ui: &Ui) -> f32 {
    let metrics = Tokens::get(ui.ctx()).metrics;
    metrics.ctl_h + 2.0 * metrics.bar_pad
}

/// A bar's frame: the bar's padding above and below its one row of controls.
fn bar_frame(ui: &Ui, fill: Color32) -> egui::Frame {
    let pad = Tokens::get(ui.ctx()).metrics.bar_pad.round() as i8;
    egui::Frame::new()
        .fill(fill)
        .inner_margin(Margin::symmetric(10, pad))
}

/// The browser: the selection repaired to a real library, cell and view, the
/// context strip, the library, cell and view columns and the menu a row asked
/// for. It returns what the reader asked of a column or a row's menu, and the
/// height the columns leave an open built-in source above the band.
fn library_browser(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) -> (Option<LibraryIntent>, f32) {
    ensure_valid_selection(
        &mut cx.model.library_manager,
        &cx.model.workspace.content.active_view,
    );
    library_context(ui, cx);

    let find = columns::Find::of(cx.state);
    // The lists' memory is the feature's; it is taken for the frame, so the
    // columns write it while they read the rest of the feature.
    let mut browser = std::mem::take(&mut cx.state.library_browser);
    // The page arrives with the keyboard in the column last worked in, as a
    // library manager's window opens on its lists, unless something holds it.
    let frame = ui.ctx().cumulative_frame_nr();
    let arrived = browser.drawn.is_none_or(|drawn| drawn + 1 < frame);
    browser.drawn = Some(frame);
    if arrived && ui.ctx().memory(|memory| memory.focused().is_none()) {
        browser.focus.get_or_insert(browser.active);
    }
    browser.focusing = browser.focus.take();
    browser.holds_keyboard = false;
    let read = &*cx;
    let available = ui.available_size();
    let visible_width = available.x.min(visible_workspace_width(ui)).max(1.0);
    // The columns fill a page at least their minimum wide, to a fraction of a
    // pixel that must not raise a scroll bar. On a narrower one the bar takes
    // its height from the columns, never from the band under them.
    let scrolls_sideways = visible_width < MIN_BROWSER_WIDTH;
    let scroll_bar = if scrolls_sideways {
        ui.spacing().scroll.allocated_width()
    } else {
        0.0
    };
    let page_height = (available.y - bar_height(ui) - scroll_bar).max(120.0);
    let browser_height =
        builtin_models::columns_height(ui, &read.state.builtin_models, page_height);
    let mut intent = None;
    ScrollArea::horizontal()
        .hscroll(scrolls_sideways)
        .id_salt("workbench.project.library.columns")
        .auto_shrink([false, true])
        .max_height(browser_height)
        .show(ui, |ui| {
            let width = visible_width.max(MIN_BROWSER_WIDTH);
            ui.set_min_width(width);
            let library_width = (width * 0.25).max(205.0);
            let cell_width = (width * 0.37).max(210.0);
            let view_width = (width - library_width - cell_width).max(220.0);
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                let (find, height) = (&find, browser_height);
                let library =
                    columns::library_column(ui, read, &mut browser, find, library_width, height);
                let cell = columns::cell_column(ui, read, &mut browser, find, cell_width, height);
                let view = columns::view_column(ui, read, &mut browser, find, view_width, height);
                intent = library.or(cell).or(view);
            });
        });
    let asked = browser.asked_menu.take();
    let holds_keyboard = browser.holds_keyboard;
    cx.state.library_browser = browser;
    if holds_keyboard {
        hand_on_undo(ui.ctx(), cx);
    }
    let menu = row_menu(ui.ctx(), cx, asked);
    (intent.or(menu), (page_height - browser_height).max(0.0))
}

/// Undo or Redo, pressed while a list holds the keyboard.
///
/// The shell keeps Edit's keys from any control that holds the keyboard, as
/// it must from a text field; a list edits no text, and a deletion made from
/// the keyboard is one the reader undoes from it. Only the profile's own
/// single-stroke bindings are taken, matched exactly, so Redo's Shift is
/// never read as Undo.
fn hand_on_undo(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    let platform = runtime_command_platform(ctx);
    let shortcuts = cx.model.preferences.shortcuts();
    let pressed = [ids::REDO, ids::UNDO].into_iter().find(|command| {
        shortcuts
            .effective_bindings(cx.commands, *command)
            .iter()
            .filter(|binding| binding.supports(platform))
            .filter_map(|binding| match binding.sequence().strokes() {
                [stroke] => Some(*stroke),
                _ => None,
            })
            .any(|stroke| consume_stroke(ctx, stroke))
    });
    if let Some(command) = pressed {
        cx.execute(command);
    }
}

/// Take a press of `stroke` from this frame's input, its modifiers exact.
fn consume_stroke(ctx: &Context, stroke: ShortcutStroke) -> bool {
    let mut modifiers = egui::Modifiers::NONE;
    if stroke.primary() {
        modifiers |= egui::Modifiers::COMMAND;
    }
    if stroke.alt() {
        modifiers |= egui::Modifiers::ALT;
    }
    if stroke.shift() {
        modifiers |= egui::Modifiers::SHIFT;
    }
    ctx.input_mut(|input| {
        let before = input.events.len();
        input.events.retain(|event| {
            !matches!(
                event,
                egui::Event::Key { key, pressed: true, modifiers: pressed, .. }
                    if *key == stroke.key() && pressed.matches_exact(modifiers)
            )
        });
        input.events.len() != before
    })
}

/// The menu a row of `column` asked for at `position`, or the one already
/// open: the band's own menu for what the row names, which the press
/// selected. Kept open until an item is taken or the reader clicks elsewhere.
fn row_menu(
    ctx: &Context,
    cx: &Cx<'_, ProjectFeature>,
    asked: Option<(BrowserColumn, egui::Pos2)>,
) -> Option<LibraryIntent> {
    let id = egui::Id::new("workbench.project.library.row-menu");
    let at = id.with("at");
    if let Some(asked) = asked {
        ctx.data_mut(|data| data.insert_temp(at, asked));
    }
    let (column, position) = ctx.data(|data| data.get_temp::<(BrowserColumn, egui::Pos2)>(at))?;
    if asked.is_none() && !egui::Popup::is_id_open(ctx, id) {
        return None;
    }
    let target = Target::of(cx.model, cx.state);
    let builtin = cx.state.builtin_models.open;
    let mut intent = None;
    egui::Popup::new(
        id,
        ctx.clone(),
        egui::PopupAnchor::Position(position),
        egui::LayerId::new(egui::Order::Foreground, id),
    )
    .kind(egui::PopupKind::Menu)
    .open_memory(asked.is_some().then_some(egui::SetOpenCommand::Bool(true)))
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
    .show(|ui| {
        intent = match column {
            BrowserColumn::Library if builtin => library_items(ui, cx, None),
            BrowserColumn::Library => library_items(ui, cx, target.library.clone()),
            BrowserColumn::Cell | BrowserColumn::View if builtin => builtin_models::row_items(
                ui,
                &cx.state.builtin_models,
                target.writable_library_available,
            ),
            BrowserColumn::Cell => cell_items(ui, &target),
            BrowserColumn::View => {
                let open = target.view.as_ref().and_then(|(reference, view_type, _)| {
                    ui.button(open_view_label(*view_type))
                        .clicked()
                        .then(|| LibraryIntent::OpenView(reference.clone(), *view_type))
                });
                if open.is_some() {
                    ui.close();
                }
                open.or_else(|| view_items(ui, &target))
            }
        };
    });
    intent
}

/// The open built-in source, `height` tall, between the columns and the band.
fn builtin_source(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    height: f32,
) -> Option<LibraryIntent> {
    let messages = cx.wb.messages();
    builtin_models::source_pane(
        ui,
        &mut cx.state.builtin_models,
        cx.peers.code(),
        messages,
        height,
    )
}

/// The built-in library's band under the columns. It keeps Library actions:
/// a reader looking at the built-in library can still make a cell or a
/// library of their own.
fn builtin_detail(ui: &mut Ui, cx: &Cx<'_, ProjectFeature>) -> Option<LibraryIntent> {
    builtin_models::detail(
        ui,
        &cx.state.builtin_models,
        &cx.model.library_manager,
        |ui| library_actions_menu(ui, cx, None),
    )
}

fn ensure_valid_selection(manager: &mut crate::state::LibraryManager, active_view: &CellViewRef) {
    let selected_library_valid = manager
        .selected_library
        .as_deref()
        .is_some_and(|name| manager.get_library(name).is_some());
    if !selected_library_valid {
        let preferred = manager
            .get_library(&active_view.library)
            .map(|library| library.name.clone())
            .or_else(|| {
                manager
                    .libraries_by_key()
                    .map(|(_, library)| library)
                    .min_by(|left, right| left.name.cmp(&right.name))
                    .map(|library| library.name.clone())
            });
        if let Some(library) = preferred {
            manager.select_library(&library);
        }
    }

    let Some(library_name) = manager.selected_library.clone() else {
        return;
    };
    let selected_cell_valid = manager
        .get_library(&library_name)
        .and_then(|library| {
            manager
                .selected_cell
                .as_deref()
                .and_then(|name| library.get_cell(name))
        })
        .is_some();
    if !selected_cell_valid {
        let preferred = manager.get_library(&library_name).and_then(|library| {
            library
                .get_cell(&active_view.cell)
                .map(|cell| cell.name.clone())
                .or_else(|| {
                    library
                        .cells
                        .values()
                        .min_by(|left, right| left.name.cmp(&right.name))
                        .map(|cell| cell.name.clone())
                })
        });
        if let Some(cell) = preferred {
            manager.select_cell(&library_name, &cell);
        }
    }

    let Some(cell_name) = manager.selected_cell.clone() else {
        return;
    };
    let selected_view_valid = manager
        .get_library(&library_name)
        .and_then(|library| library.get_cell(&cell_name))
        .and_then(|cell| {
            manager
                .selected_view
                .as_deref()
                .and_then(|name| cell.get_view(name))
        })
        .is_some();
    if !selected_view_valid {
        let preferred = manager
            .get_library(&library_name)
            .and_then(|library| library.get_cell(&cell_name))
            .and_then(|cell| {
                cell.get_view(&active_view.view)
                    .map(|view| view.name.clone())
                    .or_else(|| {
                        cell.views
                            .values()
                            .min_by(|left, right| left.name.cmp(&right.name))
                            .map(|view| view.name.clone())
                    })
            });
        if let Some(view) = preferred {
            manager.select_view(&library_name, &cell_name, &view);
        }
    }
}

pub fn library_context(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    let t = Tokens::get(ui.ctx());
    let builtin = &cx.state.builtin_models;
    let selection = if builtin.open {
        builtin_models::selection_path(builtin)
    } else {
        cx.model
            .library_manager
            .selected_lcv_path()
            .unwrap_or_else(|| "No library selection".to_owned())
    };
    // Writable is the ordinary library and says nothing; read-only is the
    // exception the strip states.
    let read_only = builtin.open
        || cx
            .model
            .library_manager
            .current_library()
            .is_some_and(|library| library.read_only);
    // No rule under this strip: the column header below opens with one of its
    // own, and two hairlines a pixel apart read as one thick, uneven line.
    bar_frame(ui, t.color.bg_inset).show(ui, |ui| {
        ui.set_min_height(t.metrics.ctl_h);
        // A `Frame` shrinks to its content, so a band that is not told
        // to take the visible width stops partway across the workspace.
        // Measured from inside the frame, the visible width is already the
        // band's inner width: subtracting the margins again leaves the fill
        // 20 px short of the edge.
        ui.set_width(visible_workspace_width(ui));
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            let search_width = (ui.available_width() * 0.32).clamp(150.0, 380.0);
            let search = ui.add_sized(
                [search_width, t.metrics.ctl_h],
                egui::TextEdit::singleline(&mut cx.state.project_library_filter)
                    .hint_text("Find in every library\u{2026}")
                    .vertical_align(Align::Center),
            );
            ui.ctx().accesskit_node_builder(search.id, |node| {
                node.set_label("Find cells in every library");
            });
            let mut asked = search.changed();
            // Enter takes the keyboard to the cells the find turned up, and
            // is spent there: the list it reaches opens nothing with it.
            if search.lost_focus()
                && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Enter))
            {
                cx.state.library_browser.focus = Some(BrowserColumn::Cell);
            }
            let view_type = cx.state.library_browser.view_type;
            if let Some(index) = crate::ui::widgets::select(
                ui,
                "workbench.project.library.view-type",
                "View type",
                view_type.map_or(ANY_VIEW_TYPE, |view_type| view_type.caption()),
                &VIEW_TYPE_CHOICES,
                VIEW_TYPE_WIDTH,
            ) {
                cx.state.library_browser.view_type =
                    index.checked_sub(1).map(|index| ViewType::ALL[index]);
                asked = true;
            }
            if asked {
                select_found(ui.ctx(), cx);
            }
            // A strip too narrow for any of the path leaves the selection
            // to the band and the inspector, rather than a label over none;
            // the separator before it is counted in.
            let separator = ui.spacing().item_spacing.x * 2.0;
            if ui.available_width() - CONTEXT_FIXED_WIDTH - separator < MIN_SELECTION_ROOM {
                return;
            }
            ui.separator();
            let room = (ui.available_width() - CONTEXT_FIXED_WIDTH).max(0.0);
            let font = theme::mono(tokens::FS_0, FontWeight::Regular);
            let selection = elide_text(ui, &selection, &font, room);
            context_value(ui, "Selection", &selection, t.color.text);
            if read_only {
                ui.separator();
                context_value(ui, "Access", "read-only", t.color.text_dim);
            }
        });
    });
}

/// What the context strip spends beside the selection's own text: its label,
/// the separator and the access pair.
const CONTEXT_FIXED_WIDTH: f32 = 190.0;

/// The least of a path worth showing: a library's name and an ellipsis.
const MIN_SELECTION_ROOM: f32 = 48.0;

/// The view-type select's choice that asks for no type.
const ANY_VIEW_TYPE: &str = "Any type";

/// The view-type select's width: its longest caption and the chevron.
const VIEW_TYPE_WIDTH: f32 = 116.0;

/// The view-type select's choices: any type, then each type by its caption.
static VIEW_TYPE_CHOICES: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
    std::iter::once(ANY_VIEW_TYPE)
        .chain(ViewType::ALL.iter().map(|view_type| view_type.caption()))
        .map(str::to_owned)
        .collect()
});

/// Bring what the find turns up into the columns. The selected cell stays
/// while the find turns it up; otherwise the first cell it does is selected,
/// the shown library's before any other's, and the built-in library's first
/// model when only that library answers. A view the find does not list
/// gives way to the cell's first one it does.
fn select_found(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    let find = columns::Find::of(cx.state);
    if find.is_everything() {
        return;
    }
    let models = find.builtin_models();
    let builtin = &cx.state.builtin_models;
    if builtin.open {
        let kept = models
            .iter()
            .any(|model| builtin.model.as_deref() == Some(model.registry_name));
        if kept {
            return;
        }
    }
    let manager = &cx.model.library_manager;
    let kept = !builtin.open && manager.current_cell().is_some_and(|cell| find.cell(cell));
    if !kept {
        if let Some((library, cell)) = first_found(manager, &find) {
            cx.state.builtin_models.open = false;
            cx.model.library_manager.select_cell(&library, &cell);
        } else if let Some(model) = models.first() {
            for intent in [
                builtin_models::BuiltinModelsIntent::ShowLibrary,
                builtin_models::BuiltinModelsIntent::SelectModel(model.registry_name),
            ] {
                apply_library_intent(ctx, cx, LibraryIntent::BuiltinModels(intent));
            }
            return;
        } else {
            return;
        }
    }
    let manager = &mut cx.model.library_manager;
    let listed = manager
        .current_view()
        .is_some_and(|view| find.view(view.view_type));
    let first = manager.current_cell().and_then(|cell| {
        let view = cell
            .views_sorted()
            .into_iter()
            .find(|view| find.view(view.view_type))?;
        Some((cell.name.clone(), view.name.clone()))
    });
    if !listed
        && let (Some(library), Some((cell, view))) = (manager.selected_library.clone(), first)
    {
        manager.select_view(&library, &cell, &view);
    }
}

/// The first cell `find` turns up: in the selected library, then in the
/// others by name.
fn first_found(manager: &LibraryManager, find: &columns::Find) -> Option<(String, String)> {
    let selected = manager.selected_library.as_deref();
    let (here, elsewhere): (Vec<_>, Vec<_>) = manager
        .libraries_sorted()
        .into_iter()
        .partition(|library| Some(library.name.as_str()) == selected);
    here.into_iter().chain(elsewhere).find_map(|library| {
        let cell = library
            .cells_sorted()
            .into_iter()
            .find(|cell| find.cell(cell))?;
        Some((library.name.clone(), cell.name.clone()))
    })
}

fn context_value(ui: &mut Ui, label: &str, value: &str, color: Color32) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
    ui.label(
        egui::RichText::new(value)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color),
    );
}

/// A column: its head with the title and the row count, then whatever
/// `body` draws under it, and the rule on its right.
fn column_frame<R>(
    ui: &mut Ui,
    title: &str,
    count: &str,
    width: f32,
    height: f32,
    body: impl FnOnce(&mut Ui) -> R,
) -> R {
    let t = Tokens::get(ui.ctx());
    let head_height = column_head_height(ui);
    let shown =
        ui.allocate_ui_with_layout(vec2(width, height), Layout::top_down(Align::Min), |ui| {
            // The column keeps its whole height however few rows it lists, so
            // the band stays at the foot of the page.
            ui.set_min_size(vec2(width, height));
            ui.set_width(width);
            ui.spacing_mut().item_spacing.y = 0.0;
            let (header, head) = ui.allocate_exact_size(vec2(width, head_height), Sense::hover());
            // The head reads as the column's name, its count as the value.
            head.widget_info(|| {
                WidgetInfo::labeled(WidgetType::Label, true, format!("{title} column"))
            });
            ui.ctx()
                .accesskit_node_builder(head.id, |node| node.set_value(count));
            ui.painter().rect_filled(header, 0.0, t.color.bg_panel);
            ui.painter().text(
                pos2(header.left() + 10.0, header.center().y),
                Align2::LEFT_CENTER,
                title,
                theme::sans(tokens::FS_0, FontWeight::SemiBold),
                t.color.text,
            );
            ui.painter().text(
                pos2(header.right() - 10.0, header.center().y),
                Align2::RIGHT_CENTER,
                count,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            );
            ui.painter().hline(
                header.x_range(),
                header.bottom(),
                Stroke::new(1.0, t.color.border),
            );
            body(ui)
        });
    ui.painter().vline(
        shown.response.rect.right(),
        shown.response.rect.y_range(),
        Stroke::new(1.0, t.color.border_strong),
    );
    shown.inner
}

/// What a column's header and its empty body say: the rows it lists, as
/// "3 of 120" while a filter hides some, and what an empty column holds.
struct ColumnCount<'a> {
    shown: usize,
    total: usize,
    empty: &'a str,
}

impl ColumnCount<'_> {
    fn header(&self) -> String {
        if self.shown == self.total {
            self.shown.to_string()
        } else {
            format!("{} of {}", self.shown, self.total)
        }
    }
}

/// The text an empty column states in its body, in place of rows.
fn empty_column_text(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
    });
}

/// What the page's menus and band act on: the library the page shows, the
/// cell and view selected in its columns, which a filter can hide, and what
/// each allows.
struct Target {
    /// The library's name and whether it is read-only.
    library: Option<(String, bool)>,
    /// The cell's library and name, and whether its library is read-only.
    cell: Option<(String, String, bool)>,
    /// The view, its type and whether an editor holds unsaved changes to it.
    view: Option<(CellViewRef, ViewType, bool)>,
    /// The cell's symbol view, the one Symbol & form edits.
    symbol: Option<CellViewRef>,
    /// Some library can take a copy.
    writable_library_available: bool,
    /// Why the cell cannot become the project top, if it cannot.
    top_refusal: Option<String>,
    /// The cells the Cell column has chosen, `cell` first: more than one
    /// while the reader has selected several.
    cells_chosen: Vec<String>,
    /// The views the View column has chosen, `view` first.
    views_chosen: Vec<String>,
}

impl Target {
    fn of(model: &Model, state: &ProjectState) -> Self {
        let manager = &model.library_manager;
        let find = columns::Find::of(state);
        let cell_shown = selected_cell_shown(model, &find);
        let cell = selected_cell(manager).filter(|_| cell_shown);
        let view = selected_view(manager)
            .filter(|(_, view_type, _, _)| cell_shown && find.view(*view_type))
            .map(|(reference, view_type, modified, _)| (reference, view_type, modified));
        // A list's choice counts only while it leads with the selection the
        // page acts on; a selection made elsewhere stands alone.
        let led_by = |chosen: Vec<String>, lead: Option<&str>| match lead {
            Some(lead) if chosen.first().is_some_and(|first| first == lead) => chosen,
            Some(lead) => vec![lead.to_owned()],
            None => Vec::new(),
        };
        let browser = &state.library_browser;
        Self {
            top_refusal: cell
                .as_ref()
                .and_then(|(library, cell, _)| project_top_refusal(model, library, cell)),
            library: manager
                .current_library()
                .map(|library| (library.name.clone(), library.read_only)),
            symbol: cell
                .as_ref()
                .and_then(|(library, cell, _)| symbol_view(model, library, cell)),
            cells_chosen: led_by(
                columns::chosen(&browser.cells),
                cell.as_ref().map(|(_, cell, _)| cell.as_str()),
            ),
            views_chosen: led_by(
                columns::chosen(&browser.views),
                view.as_ref()
                    .map(|(reference, _, _)| reference.view.as_str()),
            ),
            cell,
            view,
            writable_library_available: manager
                .libraries_by_key()
                .any(|(_, library)| !library.read_only),
        }
    }

    /// Whether the selected cell's library refuses changes.
    fn read_only(&self) -> bool {
        self.cell
            .as_ref()
            .is_some_and(|(_, _, read_only)| *read_only)
    }
}

/// One item of a menu: enabled while `blocker` is `None`, which otherwise
/// says why it is not. Returns whether it was taken, and closes the menu.
fn menu_item(ui: &mut Ui, label: &str, blocker: Option<&str>) -> bool {
    let taken = ui
        .add_enabled(blocker.is_none(), egui::Button::new(label))
        .on_disabled_hover_text(blocker.unwrap_or_default())
        .clicked();
    if taken {
        ui.close();
    }
    taken
}

/// The cell's half of Cell actions, the menu a cell row opens too.
fn cell_items(ui: &mut Ui, target: &Target) -> Option<LibraryIntent> {
    let (library, cell, _) = target.cell.clone()?;
    let read_only = target.read_only();
    let mut intent = None;
    if let Some(symbol) = &target.symbol
        && menu_item(ui, "Open in Symbol & form", None)
    {
        intent = Some(LibraryIntent::OpenSymbolForm(symbol.clone()));
    }
    if menu_item(
        ui,
        "New view\u{2026}",
        read_only.then_some("The library is read-only; a view cannot be added to it."),
    ) {
        intent = Some(LibraryIntent::NewView {
            library: library.clone(),
            cell: cell.clone(),
        });
    }
    if menu_item(
        ui,
        "Copy cell\u{2026}",
        (!target.writable_library_available)
            .then_some("Every library is read-only; none can take a copy."),
    ) {
        intent = Some(LibraryIntent::CopyCell {
            library: library.clone(),
            cell: cell.clone(),
        });
    }
    if menu_item(
        ui,
        "Rename cell\u{2026}",
        read_only.then_some("The library is read-only; its cells cannot be renamed."),
    ) {
        intent = Some(LibraryIntent::RenameCell {
            library: library.clone(),
            cell: cell.clone(),
        });
    }
    if menu_item(ui, "Set as project top", target.top_refusal.as_deref()) {
        intent = Some(LibraryIntent::SetProjectTop {
            library: library.clone(),
            cell: cell.clone(),
        });
    }
    let label = match target.cells_chosen.len() {
        0 | 1 => "Delete cell\u{2026}".to_owned(),
        count => format!("Delete {count} cells\u{2026}"),
    };
    if menu_item(
        ui,
        &label,
        read_only.then_some("The library is read-only; its cells cannot be deleted."),
    ) {
        intent = deletion_intent(&library, None, &target.cells_chosen);
    }
    intent
}

/// The view's half of Cell actions, the menu a view row opens too.
fn view_items(ui: &mut Ui, target: &Target) -> Option<LibraryIntent> {
    let (reference, _, _) = target.view.clone()?;
    let read_only = target.read_only();
    let mut intent = None;
    if menu_item(ui, "Where used\u{2026}", None) {
        intent = Some(LibraryIntent::ReportUsage(reference.clone()));
    }
    if menu_item(
        ui,
        "Rename view\u{2026}",
        read_only.then_some("The library is read-only; its views cannot be renamed."),
    ) {
        intent = Some(LibraryIntent::RenameView(reference.clone()));
    }
    let label = match target.views_chosen.len() {
        0 | 1 => "Delete view\u{2026}".to_owned(),
        count => format!("Delete {count} views\u{2026}"),
    };
    if menu_item(
        ui,
        &label,
        read_only.then_some("The library is read-only; its views cannot be deleted."),
    ) {
        intent = deletion_intent(
            &reference.library,
            Some(&reference.cell),
            &target.views_chosen,
        );
    }
    intent
}

/// What Delete asks for in a column: the review of the one row chosen, or
/// one review of every chosen row; cells when `cell` is `None`, else that
/// cell's views. `chosen` holds the lead first.
fn deletion_intent(library: &str, cell: Option<&str>, chosen: &[String]) -> Option<LibraryIntent> {
    let target = |name: &String| match cell {
        Some(cell) => LibraryDeletionTarget::View {
            library: library.to_owned(),
            cell: cell.to_owned(),
            view: name.clone(),
        },
        None => LibraryDeletionTarget::Cell {
            library: library.to_owned(),
            cell: name.clone(),
        },
    };
    match chosen {
        [] => None,
        [lead] => Some(match target(lead) {
            LibraryDeletionTarget::Cell { library, cell } => {
                LibraryIntent::DeleteCell { library, cell }
            }
            LibraryDeletionTarget::View {
                library,
                cell,
                view,
            } => LibraryIntent::DeleteView(CellViewRef::new(&library, &cell, &view)),
        }),
        several => Some(LibraryIntent::DeleteSeveral(
            several.iter().map(target).collect(),
        )),
    }
}

/// The band under the columns: what the selection is, its path cut to the
/// width the controls leave, then Library actions, Cell actions and Open.
fn library_detail(ui: &mut Ui, cx: &Cx<'_, ProjectFeature>) -> Option<LibraryIntent> {
    let t = Tokens::get(ui.ctx());
    let target = Target::of(cx.model, cx.state);
    let mut intent = None;
    let shown = bar_frame(ui, t.color.bg_panel).show(ui, |ui| {
        ui.set_min_height(t.metrics.ctl_h);
        // The buttons end at the visible edge, even where the page is
        // wider than the screen and scrolls.
        ui.set_max_width(visible_workspace_width(ui));
        ui.horizontal(|ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if let Some((reference, view_type, _)) = &target.view
                    && Button::new(open_view_label(*view_type))
                        .accent()
                        .show(ui)
                        .clicked()
                {
                    intent = Some(LibraryIntent::OpenView(reference.clone(), *view_type));
                }
                if target.cell.is_some() {
                    ui.menu_button("Cell actions", |ui| {
                        let cell = cell_items(ui, &target);
                        if target.view.is_some() {
                            ui.separator();
                        }
                        let view = view_items(ui, &target);
                        if let Some(asked) = cell.or(view) {
                            intent = Some(asked);
                        }
                    });
                }
                if let Some(asked) = library_actions_menu(ui, cx, target.library.clone()) {
                    intent = Some(asked);
                }
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    selection_summary(ui, &target);
                });
            });
        });
    });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    intent
}

/// The selected view's path, cut to the width its type and mark leave, its
/// type and, only when it has them, its unsaved changes; or what to do when
/// nothing is selected.
fn selection_summary(ui: &mut Ui, target: &Target) {
    let t = Tokens::get(ui.ctx());
    let body = egui::TextStyle::Body.resolve(ui.style());
    let width = |ui: &Ui, text: &str, font: &egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), t.color.text)
            .size()
            .x
    };
    let Some((reference, view_type, modified)) = &target.view else {
        let prompt = "Select a view to inspect or open";
        let shown = elide_text(ui, prompt, &body, ui.available_width());
        ui.label(egui::RichText::new(shown).color(t.color.text_dim));
        return;
    };
    let gap = ui.spacing().item_spacing.x * 2.0 + 1.0;
    let mut tail = gap + width(ui, view_type.caption(), &body);
    if *modified {
        tail += gap + width(ui, "modified", &body);
    }
    // A band too narrow for any of the path keeps its buttons whole and
    // leaves the selection to the inspector.
    let room = ui.available_width() - tail;
    if room < MIN_SELECTION_ROOM {
        return;
    }
    let path = reference.display_path();
    let font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let shown = elide_text(ui, &path, &font, room);
    let label = ui.label(egui::RichText::new(&shown).font(font).color(t.color.text));
    if shown != path {
        label.on_hover_text(path);
    }
    ui.separator();
    ui.label(view_type.caption());
    // Only the exception is stated: the row and the strip already say a
    // library is read-only, and an unsaved edit is a marker, not an alarm.
    if *modified {
        ui.separator();
        ui.label(egui::RichText::new("modified").color(t.color.text_dim));
    }
}

/// Library actions: create a library or a cell, rename and delete the
/// library, then the commands over the whole project that live with them,
/// Publish project…, Restore project from publication… and Audit
/// history…, each disabled with its reason. `selected_library` is the
/// library the page shows and whether it is read-only; the built-in
/// library's band passes none.
pub(super) fn library_actions_menu(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    selected_library: Option<(String, bool)>,
) -> Option<LibraryIntent> {
    let mut intent = None;
    ui.menu_button("Library actions", |ui| {
        intent = library_items(ui, cx, selected_library);
    });
    intent
}

/// The items of Library actions, the menu a library row opens too.
fn library_items(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    selected_library: Option<(String, bool)>,
) -> Option<LibraryIntent> {
    let readiness = super::library_publication::PublicationReadiness::of(
        cx.model,
        cx.platform.project_lifecycle.is_open(),
    );
    let mut intent = None;
    if menu_item(ui, "New library\u{2026}", None) {
        intent = Some(LibraryIntent::NewLibrary);
    }
    if menu_item(ui, "New cell\u{2026}", None) {
        intent = Some(LibraryIntent::NewCell);
    }
    if let Some((library, read_only)) = selected_library {
        if menu_item(
            ui,
            "Rename library\u{2026}",
            read_only.then_some("The library is read-only; it cannot be renamed."),
        ) {
            intent = Some(LibraryIntent::RenameLibrary(library.clone()));
        }
        if menu_item(
            ui,
            "Delete library\u{2026}",
            read_only.then_some("The library is read-only; it cannot be deleted."),
        ) {
            intent = Some(LibraryIntent::DeleteLibrary(library));
        }
    }
    ui.separator();
    let publish = readiness.publish_blocker();
    if menu_item(ui, "Publish project\u{2026}", publish) {
        intent = Some(LibraryIntent::PublishProject);
    }
    let restore = readiness.restore_blocker();
    if menu_item(ui, "Restore project from publication\u{2026}", restore) {
        intent = Some(LibraryIntent::RestoreFromPublication);
    }
    if menu_item(ui, "Audit history\u{2026}", None) {
        intent = Some(LibraryIntent::ShowAuditHistory);
    }
    intent
}

/// The view a double-click on a cell opens: the first the hierarchy's own
/// search order names for a root, else its first view by name.
fn default_view(model: &Model, library: &str, cell: &str) -> Option<(CellViewRef, ViewType)> {
    let record = model.library_manager.get_library(library)?.get_cell(cell)?;
    let view = rspice_design::hierarchy::default_view_search_order(
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        true,
    )
    .iter()
    .find_map(|name| record.get_view(name))
    .or_else(|| record.views_sorted().into_iter().next())?;
    Some((
        CellViewRef::new(library, &record.name, &view.name),
        view.view_type,
    ))
}

/// The cell's symbol view, the one Symbol & form edits, when it has one.
fn symbol_view(model: &Model, library: &str, cell: &str) -> Option<CellViewRef> {
    model
        .library_manager
        .get_library(library)?
        .get_cell(cell)?
        .views_sorted()
        .into_iter()
        .find(|view| view.view_type == ViewType::Symbol)
        .map(|view| CellViewRef::new(library, cell, &view.name))
}

/// Whether the selected cell is one the Cell column lists: a filter can
/// hide it, and then the page does not act on it.
fn selected_cell_shown(model: &Model, find: &columns::Find) -> bool {
    model
        .library_manager
        .current_cell()
        .is_some_and(|cell| find.cell(cell))
}

fn selected_cell(manager: &crate::state::LibraryManager) -> Option<(String, String, bool)> {
    let library = manager.current_library()?;
    let cell = manager.current_cell()?;
    Some((library.name.clone(), cell.name.clone(), library.read_only))
}

fn selected_view(
    manager: &crate::state::LibraryManager,
) -> Option<(CellViewRef, ViewType, bool, bool)> {
    let library = manager.current_library()?;
    let cell = manager.current_cell()?;
    let view = manager.current_view()?;
    Some((
        CellViewRef::new(&library.name, &cell.name, &view.name),
        view.view_type,
        view.modified,
        library.read_only,
    ))
}

/// Carry out what the reader asked of the page: the feature's own intents
/// in place, and opening a view or creating a cell raised for the shell,
/// which answers them once the page returns.
pub fn apply_library_intent(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>, intent: LibraryIntent) {
    match intent {
        LibraryIntent::BuiltinModels(intent) => builtin_models::apply(cx, intent),
        LibraryIntent::SelectLibrary(library) => {
            cx.state.builtin_models.open = false;
            cx.model.library_manager.select_library(&library);
        }
        LibraryIntent::SelectCell { library, cell } => {
            cx.model.library_manager.select_cell(&library, &cell);
        }
        LibraryIntent::SelectView(reference) => {
            cx.model.library_manager.select_view(
                &reference.library,
                &reference.cell,
                &reference.view,
            );
        }
        LibraryIntent::OpenView(reference, view_type) => {
            cx.wb
                .outbox
                .push(ProjectRequest::OpenView(reference, view_type));
        }
        LibraryIntent::OpenCell { library, cell } => {
            match default_view(cx.model, &library, &cell) {
                Some((reference, view_type)) => cx
                    .wb
                    .outbox
                    .push(ProjectRequest::OpenView(reference, view_type)),
                None => cx
                    .wb
                    .prompt(format!("Cell '{library}/{cell}' has no view to open.")),
            }
        }
        LibraryIntent::OpenSymbolForm(reference) => {
            cx.state.builtin_models.open = false;
            cx.model.library_manager.select_view(
                &reference.library,
                &reference.cell,
                &reference.view,
            );
            cx.state.library_cellview_page =
                crate::workbench::project::workspace_state::LibraryCellviewPage::SymbolForm;
            let manager = crate::workbench::SurfaceId::LibraryCellviewManager;
            if let Err(error) = cx.wb.navigate(
                crate::workbench::SurfaceRoute::surface(manager),
                crate::workbench::RouteTransitionSource::User,
            ) {
                cx.wb
                    .prompt(format!("Cannot open {}: {error}", manager.label()));
            }
        }
        LibraryIntent::ReportUsage(reference) => {
            let placements = library_view_usage(cx.model, &reference);
            let summary = usage_summary(&reference, &placements);
            ctx.data_mut(|data| {
                data.insert_temp(
                    egui::Id::new(USAGE_REPORT_ID),
                    LibraryUsageReport {
                        reference,
                        placements,
                    },
                );
            });
            cx.wb.prompt(summary);
        }
        LibraryIntent::NewLibrary => cx.state.open_new_library_dialog(cx.model),
        LibraryIntent::RenameLibrary(library) => {
            if let Err(error) = cx.state.open_rename_library_dialog(cx.model, &library) {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::DeleteLibrary(library) => {
            if let Err(error) = cx.state.open_delete_library_review(cx.model, &library) {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::NewCell => cx.execute(ProjectCommand::NewCell.key()),
        LibraryIntent::NewView { library, cell } => {
            if let Err(error) = cx
                .state
                .open_new_view_dialog(cx.model, &library, &cell, None)
            {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::CopyCell { library, cell } => {
            if let Err(error) = cx.state.open_copy_cell_dialog(cx.model, &library, &cell) {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::CopyCellInto {
            library,
            cell,
            target,
        } => {
            let opened = match copy_refusal(cx.model, &target) {
                Some(refusal) => Err(refusal),
                None => cx.state.open_copy_cell_dialog(cx.model, &library, &cell),
            };
            match opened {
                Ok(()) => {
                    // Into another library the copy keeps its name, if free.
                    let free = cx
                        .model
                        .library_manager
                        .get_library(&target)
                        .is_some_and(|library| library.get_cell(&cell).is_none());
                    if free {
                        cx.state.copy_cell_name = cell;
                    }
                    cx.state.copy_cell_target_library = target;
                }
                Err(error) => cx.wb.prompt(error),
            }
        }
        LibraryIntent::CopyModelInto {
            registry_name,
            target,
        } => {
            let opened = match copy_refusal(cx.model, &target) {
                Some(refusal) => Err(refusal),
                None => cx
                    .state
                    .open_copy_builtin_model_dialog(cx.model, registry_name),
            };
            match opened {
                Ok(()) => cx.state.copy_cell_target_library = target,
                Err(error) => cx.wb.prompt(error),
            }
        }
        LibraryIntent::RenameCell { library, cell } => {
            if let Err(error) = cx.state.open_rename_cell_dialog(cx.model, &library, &cell) {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::SetProjectTop { library, cell } => {
            match cx.model.set_project_top(&library, &cell) {
                Ok(()) => cx.wb.prompt(format!(
                    "{library}/{cell} is the project top; runs without a configuration start there."
                )),
                Err(error) => cx.wb.prompt(error),
            }
        }
        LibraryIntent::DeleteCell { library, cell } => {
            if let Err(error) = cx
                .state
                .open_library_cell_deletion_review(cx.model, &library, &cell)
            {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::RenameView(reference) => {
            if let Err(error) = cx.state.open_rename_view_dialog(
                cx.model,
                &reference.library,
                &reference.cell,
                &reference.view,
            ) {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::DeleteView(reference) => {
            if let Err(error) = cx.state.open_library_view_deletion_review(
                cx.model,
                &reference.library,
                &reference.cell,
                &reference.view,
            ) {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::DeleteSeveral(targets) => {
            if let Err(error) = cx.state.open_library_deletions_review(cx.model, targets) {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::ShowAuditHistory => {
            ctx.data_mut(|data| {
                data.insert_temp(egui::Id::new(AUDIT_REPORT_ID), true);
            });
        }
        LibraryIntent::PublishProject => {
            super::library_publication::open_publication(ctx, cx.model);
        }
        LibraryIntent::RestoreFromPublication => {
            super::library_publication::open_restore(ctx, cx.model);
        }
    }
}

/// Why library `target` cannot take a copy dragged onto it, or `None` when
/// it can.
fn copy_refusal(model: &Model, target: &str) -> Option<String> {
    match model.library_manager.get_library(target) {
        None => Some(format!("Library '{target}' no longer exists.")),
        Some(library) if library.read_only => Some(format!(
            "Library '{target}' is read-only; it cannot take a copy."
        )),
        Some(_) => None,
    }
}

/// Why Set as project top is not offered for `library`/`cell`, or `None`
/// when it is.
fn project_top_refusal(model: &Model, library: &str, cell: &str) -> Option<String> {
    let project = &model.workspace.content.project;
    if (project.root_library.as_str(), project.top_cell.as_str()) == (library, cell) {
        return Some("This cell is the project top.".to_owned());
    }
    if model.safe_mode.project_read_only() {
        return Some("The project is open read-only.".to_owned());
    }
    model.project_top_refusal(library, cell)
}

/// What a library command asks of the page, or why it cannot run now.
/// Rename, delete, copy and where used act on the row the page's keyboard
/// last worked in, so they need the Library page in front; publish, restore
/// and the audit history act on the project's libraries from anywhere.
pub fn command_intent(
    command: LibraryCommand,
    model: &Model,
    state: &ProjectState,
    page_in_front: bool,
    project_open: bool,
) -> Result<LibraryIntent, &'static str> {
    let readiness = super::library_publication::PublicationReadiness::of(model, project_open);
    let target = Target::of(model, state);
    let builtin = &state.builtin_models;
    let column = state.library_browser.active;
    let writable = |read_only: bool, intent: LibraryIntent| {
        if read_only {
            Err("the library is read-only")
        } else {
            Ok(intent)
        }
    };
    match command {
        LibraryCommand::Publish => readiness
            .publish_blocker()
            .map_or(Ok(LibraryIntent::PublishProject), Err),
        LibraryCommand::Restore => readiness
            .restore_blocker()
            .map_or(Ok(LibraryIntent::RestoreFromPublication), Err),
        LibraryCommand::AuditHistory if project_open => Ok(LibraryIntent::ShowAuditHistory),
        LibraryCommand::AuditHistory => Err("no project is open"),
        _ if !page_in_front => Err("open the Library page"),
        LibraryCommand::CopyCell if builtin.open => {
            let chosen = builtin_models::selected_model(builtin).ok_or("select a model")?;
            if target.writable_library_available {
                Ok(LibraryIntent::BuiltinModels(
                    builtin_models::BuiltinModelsIntent::CopyModel(chosen.registry_name),
                ))
            } else {
                Err("every library is read-only")
            }
        }
        _ if builtin.open => Err("the built-in library is read-only"),
        LibraryCommand::CopyCell => {
            let (library, cell, _) = target.cell.ok_or("select a cell")?;
            if target.writable_library_available {
                Ok(LibraryIntent::CopyCell { library, cell })
            } else {
                Err("every library is read-only")
            }
        }
        LibraryCommand::WhereUsed => target
            .view
            .map(|(reference, _, _)| LibraryIntent::ReportUsage(reference))
            .ok_or("select a view"),
        LibraryCommand::Rename | LibraryCommand::Delete => {
            let rename = command == LibraryCommand::Rename;
            match column {
                BrowserColumn::Library => {
                    let (library, read_only) = target.library.ok_or("select a library")?;
                    writable(
                        read_only,
                        if rename {
                            LibraryIntent::RenameLibrary(library)
                        } else {
                            LibraryIntent::DeleteLibrary(library)
                        },
                    )
                }
                BrowserColumn::Cell => {
                    let (library, cell, read_only) = target.cell.clone().ok_or("select a cell")?;
                    let intent = if rename {
                        Some(LibraryIntent::RenameCell { library, cell })
                    } else {
                        deletion_intent(&library, None, &target.cells_chosen)
                    };
                    writable(read_only, intent.ok_or("select a cell")?)
                }
                BrowserColumn::View => {
                    let read_only = target.read_only();
                    let (reference, _, _) = target.view.clone().ok_or("select a view")?;
                    let intent = if rename {
                        Some(LibraryIntent::RenameView(reference))
                    } else {
                        deletion_intent(
                            &reference.library,
                            Some(&reference.cell),
                            &target.views_chosen,
                        )
                    };
                    writable(read_only, intent.ok_or("select a view")?)
                }
            }
        }
    }
}

/// Run a library command: the Library page comes to the front and carries
/// out what the command asks as it next draws, or the status bar says why
/// the command cannot run.
pub fn run_command(cx: &mut Cx<'_, ProjectFeature>, command: LibraryCommand) {
    let page_in_front = library_page_in_front(cx.wb.workspace, cx.state);
    let project_open = cx.platform.project_lifecycle.is_open();
    match command_intent(command, cx.model, cx.state, page_in_front, project_open) {
        Ok(intent) => {
            cx.wb.activate(Workspace::Project);
            cx.state.project_page = ProjectPage::Library;
            cx.state.library_browser.queued = Some(intent);
        }
        Err(reason) => cx.wb.prompt(reason),
    }
}

/// Whether the Library page is the one in front.
pub fn library_page_in_front(workspace: Workspace, state: &ProjectState) -> bool {
    workspace == Workspace::Project && state.project_page == ProjectPage::Library
}

fn library_view_usage(model: &Model, reference: &CellViewRef) -> Vec<(String, String)> {
    model
        .cell_instances(&reference.library, &reference.cell, Some(&reference.view))
        .into_iter()
        .map(|usage| (usage.schematic.display_path(), usage.instance))
        .collect()
}

/// "amp/symbol is placed 3 times in 2 schematics."
fn usage_summary(reference: &CellViewRef, placements: &[(String, String)]) -> String {
    let path = reference.display_path();
    if placements.is_empty() {
        return format!("{path} is not placed in any schematic of this project.");
    }
    let mut schematics = placements
        .iter()
        .map(|(schematic, _)| schematic.as_str())
        .collect::<Vec<_>>();
    schematics.dedup();
    let plural = |count: usize, one: &str, many: &str| {
        if count == 1 {
            format!("{count} {one}")
        } else {
            format!("{count} {many}")
        }
    };
    format!(
        "{path} is placed {} in {}.",
        plural(placements.len(), "time", "times"),
        plural(schematics.len(), "schematic", "schematics")
    )
}

fn show_library_usage_report(ctx: &Context) {
    let Some(report) =
        ctx.data(|data| data.get_temp::<LibraryUsageReport>(egui::Id::new(USAGE_REPORT_ID)))
    else {
        return;
    };
    let description = usage_summary(&report.reference, &report.placements);
    let choice = Dialog::new("Where used", "Close")
        .description(&description)
        .size(DialogSize::Transaction)
        .show(ctx, |ui| {
            if !report.placements.is_empty() {
                property_card(ui, "Placements", |ui| {
                    for (schematic, instance) in &report.placements {
                        property_row(ui, schematic, instance);
                    }
                });
            }
        });
    if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        ctx.data_mut(|data| {
            data.remove::<LibraryUsageReport>(egui::Id::new(USAGE_REPORT_ID));
        });
    }
}

fn show_library_audit_history(ctx: &Context, model: &Model) {
    if !ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new(AUDIT_REPORT_ID))
            .unwrap_or(false)
    }) {
        return;
    }

    let receipts = model.workspace.content.project.library_mutation_audit();
    let publications = model.workspace.content.project.library_publications();
    let description = format!(
        "{} recorded library change{} and {} publication{}.",
        receipts.len(),
        if receipts.len() == 1 { "" } else { "s" },
        publications.len(),
        if publications.len() == 1 { "" } else { "s" }
    );
    let choice = Dialog::new("Audit history", "Close")
        .description(&description)
        .size(DialogSize::Manager)
        .show(ctx, |ui| {
            property_card(ui, "Audit chain", |ui| {
                property_row(
                    ui,
                    "Project revision",
                    &model.workspace.content.project.revision().get().to_string(),
                );
                property_row(
                    ui,
                    "Library revision",
                    &model.library_manager.revision().to_string(),
                );
                property_row(ui, "Retained receipts", &receipts.len().to_string());
                property_row(ui, "Retained publications", &publications.len().to_string());
            });
            ui.add_space(8.0);
            ScrollArea::vertical()
                .id_salt("workbench.project.library.audit-history.receipts")
                .auto_shrink([false, true])
                .max_height(360.0)
                .show(ui, |ui| {
                    if receipts.is_empty() && publications.is_empty() {
                        property_card(ui, "No retained mutations", |ui| {
                            property_row(
                                ui,
                                "Status",
                                "The project has no audited library changes or publications.",
                            );
                        });
                    }
                    for publication in publications.iter().rev() {
                        let title = format!(
                            "Publication #{:04} \u{00b7} {}",
                            publication.sequence(),
                            publication.label()
                        );
                        property_card(ui, &title, |ui| {
                            property_row(
                                ui,
                                "Publication identity",
                                &publication.publication_id().to_string(),
                            );
                            property_row(ui, "Actor", publication.actor_id());
                            property_row(ui, "Authority", publication.authority_id());
                            property_row(ui, "Reason", publication.reason());
                            property_row(
                                ui,
                                "Project revision",
                                &format!(
                                    "{} \u{2192} {}",
                                    publication.source_project_revision().get(),
                                    publication.receipt_project_revision().get()
                                ),
                            );
                            property_row(
                                ui,
                                "Library revision",
                                &publication.library_revision().to_string(),
                            );
                            property_row(
                                ui,
                                "Artifact",
                                &format!(
                                    "{} bytes \u{00b7} {}",
                                    publication.snapshot_byte_len(),
                                    publication.snapshot_digest()
                                ),
                            );
                            property_row(
                                ui,
                                "Receipt digest",
                                &publication.receipt_digest().to_string(),
                            );
                        });
                        ui.add_space(6.0);
                    }
                    for receipt in receipts.iter().rev() {
                        let title = format!(
                            "#{:04} \u{00b7} {}",
                            receipt.sequence(),
                            receipt.mutation().operation_label()
                        );
                        property_card(ui, &title, |ui| {
                            property_row(
                                ui,
                                "Object",
                                &library_mutation_summary(receipt.mutation()),
                            );
                            property_row(
                                ui,
                                "Project revision",
                                &format!(
                                    "{} \u{2192} {}",
                                    receipt.source_project_revision().get(),
                                    receipt.target_project_revision().get()
                                ),
                            );
                            property_row(
                                ui,
                                "Library revision",
                                &format!(
                                    "{} \u{2192} {}",
                                    receipt.source_library_revision(),
                                    receipt.target_library_revision()
                                ),
                            );
                            property_row(
                                ui,
                                "Receipt digest",
                                &receipt.receipt_digest().to_string(),
                            );
                        });
                        ui.add_space(6.0);
                    }
                });
        });
    if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        ctx.data_mut(|data| {
            data.remove::<bool>(egui::Id::new(AUDIT_REPORT_ID));
        });
    }
}

fn library_mutation_summary(mutation: &crate::state::ProjectLibraryMutation) -> String {
    match mutation {
        crate::state::ProjectLibraryMutation::CreateLibrary { library }
        | crate::state::ProjectLibraryMutation::DeleteLibrary { library } => library.clone(),
        crate::state::ProjectLibraryMutation::RenameLibrary {
            from_library,
            to_library,
        } => format!("{from_library} \u{2192} {to_library}"),
        crate::state::ProjectLibraryMutation::CreateCell { library, cell }
        | crate::state::ProjectLibraryMutation::DeleteCell { library, cell } => {
            format!("{library}/{cell}")
        }
        crate::state::ProjectLibraryMutation::CreateView {
            library,
            cell,
            view,
        }
        | crate::state::ProjectLibraryMutation::DeleteView {
            library,
            cell,
            view,
        } => format!("{library}/{cell}/{view}"),
        crate::state::ProjectLibraryMutation::CopyCell {
            source_library,
            source_cell,
            target_library,
            target_cell,
        } => format!("{source_library}/{source_cell} \u{2192} {target_library}/{target_cell}"),
        crate::state::ProjectLibraryMutation::RenameCell {
            library,
            from_cell,
            to_cell,
        } => format!("{library}/{from_cell} \u{2192} {library}/{to_cell}"),
        crate::state::ProjectLibraryMutation::RenameView {
            library,
            cell,
            from_view,
            to_view,
        } => format!("{library}/{cell}/{from_view} \u{2192} {library}/{cell}/{to_view}"),
        crate::state::ProjectLibraryMutation::RollbackPublication {
            publication_label, ..
        } => format!("Project restored from {publication_label}"),
        crate::state::ProjectLibraryMutation::Batch { operations } => operations
            .iter()
            .map(library_mutation_summary)
            .collect::<Vec<_>>()
            .join(", "),
    }
}

fn open_view_label(view_type: ViewType) -> &'static str {
    match view_type {
        ViewType::Schematic => "Open schematic",
        ViewType::Symbol => "Open symbol",
        ViewType::Layout => "Open layout",
        ViewType::Testbench => "Open testbench",
        ViewType::Verilog => "Open Verilog",
        ViewType::VerilogA => "Open Verilog-A",
        ViewType::VerilogAms => "Open Verilog-AMS",
        ViewType::Spice => "Open SPICE source",
        ViewType::Document => "Open document",
        ViewType::Extracted => "Open extracted view",
        ViewType::Abstract => "Open abstract view",
        ViewType::Config => "Open configuration",
        ViewType::Custom => "Open view",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::project::tests::harness;

    #[test]
    fn selection_repairs_to_real_library_cell_and_view() {
        let harness = harness();
        let mut manager = harness.model.library_manager.clone();
        manager.selected_library = Some("missing".to_owned());
        manager.selected_cell = Some("missing".to_owned());
        manager.selected_view = Some("missing".to_owned());

        ensure_valid_selection(&mut manager, &harness.model.workspace.content.active_view);

        assert!(manager.current_library().is_some());
        assert!(manager.current_cell().is_some());
        assert!(manager.current_view().is_some());
    }

    #[test]
    fn where_used_report_reads_live_and_inactive_schematic_buffers() {
        let mut harness = harness();
        let reference = harness.model.workspace.content.active_view.clone();
        let binding = crate::state::LibraryCellInstance::new(
            &reference.library,
            &reference.cell,
            &reference.view,
        );
        harness
            .model
            .schematic
            .add_library_cell_component(crate::state::Point::origin(), binding.clone());

        let mut inactive = crate::state::SchematicState::default();
        inactive.add_library_cell_component(crate::state::Point::origin(), binding);
        harness
            .model
            .workspace
            .insert_schematic_editor("work/consumer/schematic".to_owned(), inactive);

        let placements = library_view_usage(&harness.model, &reference);
        assert_eq!(placements.len(), 2);
        assert!(
            placements
                .iter()
                .any(|(schematic, _)| schematic.contains("work/consumer/schematic"))
        );
        assert_eq!(
            usage_summary(&reference, &placements),
            format!(
                "{} is placed 2 times in 2 schematics.",
                reference.display_path()
            )
        );
        assert_eq!(
            usage_summary(&reference, &[]),
            format!(
                "{} is not placed in any schematic of this project.",
                reference.display_path()
            )
        );
    }

    #[test]
    fn audit_history_formats_exact_source_and_target_identities() {
        assert_eq!(
            library_mutation_summary(&crate::state::ProjectLibraryMutation::CopyCell {
                source_library: "work".to_owned(),
                source_cell: "amp".to_owned(),
                target_library: "release".to_owned(),
                target_cell: "amp_v2".to_owned(),
            }),
            "work/amp \u{2192} release/amp_v2"
        );
        assert_eq!(
            library_mutation_summary(&crate::state::ProjectLibraryMutation::DeleteView {
                library: "work".to_owned(),
                cell: "amp".to_owned(),
                view: "symbol".to_owned(),
            }),
            "work/amp/symbol"
        );
    }
}
