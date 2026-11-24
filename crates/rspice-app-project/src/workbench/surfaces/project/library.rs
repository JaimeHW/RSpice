//! Dense project library browser.
//!
//! The surface projects the persisted design-library manager directly: each
//! visible row names a real library, cell, or view, selection writes back to
//! that manager, and opening a view uses the normal workspace transaction.

pub use super::*;
use std::sync::Arc;

use crate::state::shortcuts::ShortcutStroke;
use crate::state::{CellViewRef, LibraryManager, ViewType};
use crate::ui::accessibility::{counted, grouped};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::LibraryMode;
use crate::workbench::design_system::elide_text;
use crate::workbench::docks::inspector::project::LibraryShown;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::{LibraryCommand, ProjectCommand, ProjectRequest};
use crate::workbench::project::library_dialogs::library_files::{
    LibraryFileIntent, library_file_blocker, library_file_intents, library_file_label,
    library_file_refusal,
};
use crate::workbench::project::workspace_state::LibraryDeletionTarget;
use crate::workbench::slots::ProjectState;
use rspice_app_workbench::workbench::app_state::session::shortcuts::runtime_command_platform;
use rspice_ui_kit::widgets::{PageBar, PageBarFilter, page_bar_kind};

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
    /// Where used of a view, of every view of a cell, or of a library's
    /// cells as placed from outside it.
    ReportUsage(UsageSubject),
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
    /// Move the cell to another library, with its open tabs, every
    /// placement and the project top following it.
    MoveCell {
        library: String,
        cell: String,
    },
    /// Copy the library into a new one of the project.
    CopyLibrary(String),
    /// Copy the view into a cell of a writable library.
    CopyView(CellViewRef),
    /// Open the view in Design as a reference that refuses edits.
    OpenReadOnly(CellViewRef),
    /// Make a library the project file holds read-only, or editable again,
    /// as its properties do.
    SetReadOnly {
        library: String,
        read_only: bool,
    },
    /// Library properties, or with `cell` Cell properties.
    EditProperties {
        library: String,
        cell: Option<String>,
    },
    RenameView(CellViewRef),
    DeleteView(CellViewRef),
    /// One review of a multi-selection's cells or views.
    DeleteSeveral(Vec<LibraryDeletionTarget>),
    ShowAuditHistory,
    PublishProject,
    RestoreFromPublication,
    /// A command over a library kept in a folder of its own, or over an
    /// embedded one on its way to or from a folder.
    Files(crate::workbench::project::library_dialogs::library_files::LibraryFileIntent),
}

/// What Where used is asked of: one view, every view of a cell, or the
/// cells of a library as placed from outside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsageSubject {
    View(CellViewRef),
    Cell { library: String, cell: String },
    Library(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LibraryUsageReport {
    subject: UsageSubject,
    /// Each schematic placement: the schematic it is on and the instance's
    /// name, with the cell it places in a library's report.
    placements: Vec<(String, String)>,
    /// Each layout that places it, with how many times.
    layouts: Vec<(String, usize)>,
}

impl LibraryUsageReport {
    fn of(model: &Model, subject: UsageSubject) -> Self {
        let schematic = |library: &str, cell: &str, view: Option<&str>| {
            model
                .cell_instances(library, cell, view)
                .into_iter()
                .map(|usage| (usage.schematic.display_path(), usage.instance))
                .collect::<Vec<_>>()
        };
        let (placements, layouts) = match &subject {
            UsageSubject::View(reference) => {
                let view = Some(reference.view.as_str());
                (
                    schematic(&reference.library, &reference.cell, view),
                    model.cell_layout_placements(&reference.library, &reference.cell, view),
                )
            }
            UsageSubject::Cell { library, cell } => (
                schematic(library, cell, None),
                model.cell_layout_placements(library, cell, None),
            ),
            UsageSubject::Library(library) => {
                let (instances, layouts) = model.library_external_placements(library);
                let placements = instances
                    .into_iter()
                    .map(|(cell, usage)| {
                        (
                            usage.schematic.display_path(),
                            format!("{} ({cell})", usage.instance),
                        )
                    })
                    .collect();
                (placements, layouts)
            }
        };
        Self {
            subject,
            placements,
            layouts: layouts
                .into_iter()
                .map(|usage| (usage.layout.display_path(), usage.placements))
                .collect(),
        }
    }

    /// "amp/symbol is placed 3 times in 2 schematics and once in 1 layout."
    fn summary(&self) -> String {
        let count = |count: usize, one: &str, many: &str| {
            if count == 1 {
                format!("{count} {one}")
            } else {
                format!("{count} {many}")
            }
        };
        let times = |placements: usize| match placements {
            1 => "once".to_owned(),
            placements => format!("{placements} times"),
        };
        let mut schematics = self
            .placements
            .iter()
            .map(|(schematic, _)| schematic.as_str())
            .collect::<Vec<_>>();
        schematics.dedup();
        let mut places = Vec::new();
        if !self.placements.is_empty() {
            places.push(format!(
                "{} in {}",
                times(self.placements.len()),
                count(schematics.len(), "schematic", "schematics")
            ));
        }
        let layout_placements = self.layouts.iter().map(|(_, placements)| placements).sum();
        if layout_placements > 0 {
            places.push(format!(
                "{} in {}",
                times(layout_placements),
                count(self.layouts.len(), "layout", "layouts")
            ));
        }
        let places = places.join(" and ");
        match &self.subject {
            UsageSubject::Library(library) if places.is_empty() => {
                format!("No cell of {library} is placed outside it in this project.")
            }
            UsageSubject::Library(library) => {
                format!("Cells of {library} are placed {places} outside it.")
            }
            _ if places.is_empty() => format!(
                "{} is not placed in any schematic or layout of this project.",
                self.path()
            ),
            _ => format!("{} is placed {places}.", self.path()),
        }
    }

    /// The path the report names its subject by.
    fn path(&self) -> String {
        match &self.subject {
            UsageSubject::View(reference) => reference.display_path(),
            UsageSubject::Cell { library, cell } => format!("{library}/{cell}"),
            UsageSubject::Library(library) => library.clone(),
        }
    }
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
    // A folder the browser finished reading for Embed library from folder…
    // or Read from folder… goes in before the page draws.
    crate::workbench::project::library_dialogs::library_files::take_browser_folder(cx);
    // What a command asked of the page since it last drew.
    if let Some(intent) = cx.state.library_browser.queued.take() {
        apply_library_intent(&ctx, cx, intent);
    }
    ensure_valid_selection(
        &mut cx.model.library_manager,
        &cx.model.workspace.content.active_view,
    );
    ui.spacing_mut().item_spacing.y = 0.0;
    if let Some(intent) = library_bar(ui, cx) {
        apply_library_intent(&ctx, cx, intent);
    }
    let (intent, source_height) = library_browser(ui, cx);
    if let Some(intent) = intent {
        apply_library_intent(&ctx, cx, intent);
    }
    if cx.state.builtin_models.open
        && let Some(intent) = builtin_source(ui, cx, source_height)
    {
        apply_library_intent(&ctx, cx, intent);
    }
    show_library_usage_report(&ctx);
    show_library_audit_history(&ctx, cx.model, &mut cx.state.library_browser.audit);
}

/// A bar's frame: the bar's padding above and below its one row of controls.
fn bar_frame(ui: &Ui, fill: Color32) -> egui::Frame {
    let pad = Tokens::get(ui.ctx()).metrics.bar_pad.round() as i8;
    egui::Frame::new()
        .fill(fill)
        .inner_margin(Margin::symmetric(10, pad))
}

/// The browser: the library, cell and view columns and the menu a row asked
/// for. It returns what the reader asked of a column or a row's menu, and the
/// height the columns leave an open built-in source under them.
fn library_browser(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) -> (Option<LibraryIntent>, f32) {
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
    // pixel that must not raise a scroll bar; a narrower one scrolls them.
    let scrolls_sideways = visible_width < MIN_BROWSER_WIDTH;
    let scroll_bar = if scrolls_sideways {
        ui.spacing().scroll.allocated_width()
    } else {
        0.0
    };
    let page_height = (available.y - scroll_bar).max(120.0);
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
            let [library_width, cell_width, view_width] = column_widths(browser.split, width);
            let top = ui.cursor().min;
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                let (find, height) = (&find, browser_height);
                let library =
                    columns::library_column(ui, read, &mut browser, find, library_width, height);
                let cell = columns::cell_column(ui, read, &mut browser, find, cell_width, height);
                let view = columns::view_column(ui, read, &mut browser, find, view_width, height);
                intent = library.or(cell).or(view);
            });
            let rules = [library_width, library_width + cell_width];
            resize_columns(ui, &mut browser.split, top, rules, width, browser_height);
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

/// The least a column is made, by the page or by a drag.
const MIN_COLUMN_WIDTH: f32 = 160.0;

/// Where the rules between the columns stand while the reader has not moved
/// them: three equal columns.
const EQUAL_COLUMNS: [f32; 2] = [1.0 / 3.0, 2.0 / 3.0];

/// The three columns' widths across `width`: where the two rules between
/// them stand, as fractions of the width, each column at least its least.
fn column_widths(split: Option<[f32; 2]>, width: f32) -> [f32; 3] {
    let [first, second] = split.unwrap_or(EQUAL_COLUMNS);
    let least = MIN_COLUMN_WIDTH.min(width / 3.0);
    let left = (width * first).round().clamp(least, width - 2.0 * least);
    let middle = (width * second).round().clamp(left + least, width - least);
    [left, middle - left, width - middle]
}

/// The two rules between the columns, each dragged sideways to share the
/// width of the columns on either side of it. The pointer says a rule moves,
/// and the rule lights while it is pointed at or held.
fn resize_columns(
    ui: &mut Ui,
    split: &mut Option<[f32; 2]>,
    top: egui::Pos2,
    rules: [f32; 2],
    width: f32,
    height: f32,
) {
    const NAMES: [&str; 2] = [
        "Resize the Library and Cell columns",
        "Resize the Cell and View columns",
    ];
    let t = Tokens::get(ui.ctx());
    let mut moved = rules;
    for (index, x) in rules.into_iter().enumerate() {
        let at = top.x + x;
        let rule = Rect::from_x_y_ranges(at - 3.0..=at + 3.0, top.y..=top.y + height);
        let id = egui::Id::new(("workbench.project.library.column-rule", index));
        let response = ui.interact(rule, id, Sense::drag());
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Other, true, NAMES[index]));
        if response.hovered() || response.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
            ui.painter()
                .vline(at, rule.y_range(), Stroke::new(2.0, t.color.accent));
        }
        if response.dragged() {
            moved[index] += response.drag_delta().x;
        }
    }
    if moved != rules {
        *split = Some([moved[0] / width, moved[1] / width]);
    }
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
    let builtin = cx.state.builtin_models.open;
    // The built-in library itself takes no command: its models are copied
    // from their own rows.
    if builtin && column == BrowserColumn::Library {
        return None;
    }
    let target = Target::of(cx.model, cx.state);
    let mut intent = None;
    egui::Popup::new(
        id,
        ctx.clone(),
        egui::PopupAnchor::Position(position),
        egui::LayerId::new(egui::Order::Foreground, id),
    )
    .kind(egui::PopupKind::Menu)
    // A menu, not a stack of buttons: one column of full-width rows in the
    // menu style, as every desktop tool's context menu reads.
    .layout(egui::Layout::top_down_justified(egui::Align::Min))
    .style(egui::containers::menu::menu_style)
    .open_memory(asked.is_some().then_some(egui::SetOpenCommand::Bool(true)))
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
    .show(|ui| {
        intent = match column {
            BrowserColumn::Library => target
                .library
                .clone()
                .and_then(|library| library_items(ui, cx.model, library)),
            BrowserColumn::Cell | BrowserColumn::View if builtin => builtin_models::row_items(
                ui,
                &cx.state.builtin_models,
                target.writable_library_available,
            ),
            BrowserColumn::Cell => cell_items(ui, &target),
            BrowserColumn::View => view_items(ui, cx.model, &target),
        };
    });
    intent
}

/// The open built-in source, `height` tall, under the columns.
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

/// The page's one bar: the find and the view type, then New library…,
/// Attach library…, Refresh… and the page's other actions under More; on
/// the right, how many libraries and cells there are, or how many cells the
/// find turns up. What is selected is the inspector's to state; the columns
/// mark it, and a read-only library carries its lock.
fn library_bar(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) -> Option<LibraryIntent> {
    let find = columns::Find::of(cx.state);
    let report = bar_report(ui.ctx(), cx.model, &find);
    let project_open = cx.platform.project_lifecycle.is_open();
    let mut intent = None;
    let asked = PageBar::new().report(report, None).show(ui, |ui| {
        let search = PageBarFilter::new("workbench.project.library", "Find cells in every library")
            .hint("Find in every library\u{2026}")
            .show(ui, &mut cx.state.project_library_filter);
        let mut asked = search.changed();
        // Enter takes the keyboard to the cells the find turned up, and is
        // spent there: the list it reaches opens nothing with it.
        if search.lost_focus()
            && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Enter))
        {
            cx.state.library_browser.focus = Some(BrowserColumn::Cell);
        }
        let current = cx
            .state
            .library_browser
            .view_type
            .and_then(|wanted| {
                ViewType::ALL
                    .iter()
                    .position(|view_type| *view_type == wanted)
            })
            .map_or(0, |index| index + 1);
        if let Some(index) = page_bar_kind(
            ui,
            "workbench.project.library.view-type",
            "View type",
            &VIEW_TYPE_CHOICES,
            current,
        ) {
            cx.state.library_browser.view_type =
                index.checked_sub(1).map(|index| ViewType::ALL[index]);
            asked = true;
        }
        if bar_button(ui, "New library\u{2026}", None) {
            intent = Some(LibraryIntent::NewLibrary);
        }
        let attach = LibraryFileIntent::Attach;
        let blocker = library_file_blocker(cx.model, &attach);
        if bar_button(ui, library_file_label(&attach), blocker.as_deref()) {
            intent = Some(LibraryIntent::Files(attach));
        }
        let refresh = refresh_refusal(cx.model);
        if bar_button(ui, "Refresh\u{2026}", refresh.as_deref()) {
            intent = Some(LibraryIntent::Files(LibraryFileIntent::Refresh));
        }
        let side = Tokens::get(ui.ctx()).metrics.ctl_h;
        let more = crate::ui::panels::icon_button(
            ui,
            WorkbenchIcon::More,
            "More library actions",
            false,
            vec2(side, side),
        );
        egui::Popup::menu(&more).show(|ui| {
            if let Some(asked) = more_items(ui, cx.model, project_open) {
                intent = Some(asked);
            }
        });
        asked
    });
    if asked {
        select_found(ui.ctx(), cx);
    }
    intent
}

/// A button of the bar, disabled with its reason under the pointer.
fn bar_button(ui: &mut Ui, label: &str, refusal: Option<&str>) -> bool {
    Button::new(label)
        .enabled(refusal.is_none())
        .show(ui)
        .on_disabled_hover_text(refusal.unwrap_or_default())
        .clicked()
}

/// Why Refresh… cannot run, if it cannot: it reads what changed in the
/// folders libraries are kept in, and the project may keep none.
fn refresh_refusal(model: &Model) -> Option<String> {
    if model.library_manager.references().next().is_none() {
        return Some("No library is kept in a folder of its own.".to_owned());
    }
    library_file_blocker(model, &LibraryFileIntent::Refresh)
}

/// More: the page's commands taken less often, each disabled with its
/// reason. Embed library from folder… brings a library in as Attach does;
/// Publish project…, Restore project from publication… and Audit history…
/// act on every library the project holds.
fn more_items(ui: &mut Ui, model: &Model, project_open: bool) -> Option<LibraryIntent> {
    let readiness = super::library_publication::PublicationReadiness::of(model, project_open);
    let mut intent = file_item(ui, model, LibraryFileIntent::EmbedFromFolder);
    ui.separator();
    if menu_item(ui, "Publish project\u{2026}", readiness.publish_blocker()) {
        intent = Some(LibraryIntent::PublishProject);
    }
    if menu_item(
        ui,
        "Restore project from publication\u{2026}",
        readiness.restore_blocker(),
    ) {
        intent = Some(LibraryIntent::RestoreFromPublication);
    }
    if menu_item(ui, "Audit history\u{2026}", None) {
        intent = Some(LibraryIntent::ShowAuditHistory);
    }
    intent
}

/// What the bar says on its right: how many libraries and cells the page
/// lists, the built-in library and its models counted, or while a find asks
/// for something, how many of the cells it turns up. Counted again only when
/// the catalog or the find moves.
fn bar_report(ctx: &Context, model: &Model, find: &columns::Find) -> String {
    let manager = &model.library_manager;
    let (libraries, cells, found) = super::derived::derived(
        ctx,
        "library.report",
        (manager.catalog().stamp(), find.clone()),
        || {
            let models = rspice_app_state::veriloga_library::BUILTIN_VERILOGA_MODELS.len();
            let libraries = manager.library_count() + 1;
            let cells = manager
                .libraries_by_key()
                .map(|(_, library)| library.cell_count())
                .sum::<usize>()
                + models;
            let found = if find.is_everything() {
                cells
            } else {
                manager
                    .libraries_by_key()
                    .map(|(_, library)| {
                        library
                            .cells
                            .values()
                            .filter(|cell| find.cell(cell))
                            .count()
                    })
                    .sum::<usize>()
                    + find.builtin_models().len()
            };
            (libraries, cells, found)
        },
    );
    let cells = counted(cells, "cell", "cells");
    if find.is_everything() {
        format!(
            "{} \u{b7} {cells}",
            counted(libraries, "library", "libraries")
        )
    } else {
        format!("{} of {cells}", grouped(found))
    }
}

/// The view-type select's choice that asks for no type.
const ANY_VIEW_TYPE: &str = "Any type";

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
        let shown = shown_selection(model, state);
        let cell = selected_cell(manager).filter(|_| shown.cell);
        let view = selected_view(manager)
            .filter(|_| shown.view)
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

/// The menu a cell row opens: open its default view or its symbol's form,
/// make, rename, copy and move it, ask where it is used, make it the project
/// top, its properties, and delete it, or every cell chosen with it. Each
/// item is disabled with its reason.
fn cell_items(ui: &mut Ui, target: &Target) -> Option<LibraryIntent> {
    let (library, cell, _) = target.cell.clone()?;
    let read_only = target.read_only();
    let mut intent = None;
    if menu_item(ui, "Open", None) {
        intent = Some(LibraryIntent::OpenCell {
            library: library.clone(),
            cell: cell.clone(),
        });
    }
    if let Some(symbol) = &target.symbol
        && menu_item(ui, "Open in Symbol & form", None)
    {
        intent = Some(LibraryIntent::OpenSymbolForm(symbol.clone()));
    }
    ui.separator();
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
        "Rename cell\u{2026}",
        read_only.then_some("The library is read-only; its cells cannot be renamed."),
    ) {
        intent = Some(LibraryIntent::RenameCell {
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
        "Move cell\u{2026}",
        read_only.then_some("The library is read-only; its cells cannot be moved."),
    ) {
        intent = Some(LibraryIntent::MoveCell {
            library: library.clone(),
            cell: cell.clone(),
        });
    }
    ui.separator();
    if menu_item(ui, "Where cell is used\u{2026}", None) {
        intent = Some(LibraryIntent::ReportUsage(UsageSubject::Cell {
            library: library.clone(),
            cell: cell.clone(),
        }));
    }
    if menu_item(ui, "Set as project top", target.top_refusal.as_deref()) {
        intent = Some(LibraryIntent::SetProjectTop {
            library: library.clone(),
            cell: cell.clone(),
        });
    }
    if menu_item(ui, "Cell properties\u{2026}", None) {
        intent = Some(LibraryIntent::EditProperties {
            library: library.clone(),
            cell: Some(cell.clone()),
        });
    }
    ui.separator();
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

/// The menu a view row opens: open it, or open it read-only, copy and
/// rename it, ask where it is used, break another holder's lock on its file,
/// and delete it, or every view chosen with it. Each item is disabled with
/// its reason.
fn view_items(ui: &mut Ui, model: &Model, target: &Target) -> Option<LibraryIntent> {
    let (reference, view_type, _) = target.view.clone()?;
    let read_only = target.read_only();
    let mut intent = None;
    if menu_item(ui, open_view_label(view_type), None) {
        intent = Some(LibraryIntent::OpenView(reference.clone(), view_type));
    }
    if menu_item(ui, "Open read-only", read_only_open_refusal(view_type)) {
        intent = Some(LibraryIntent::OpenReadOnly(reference.clone()));
    }
    ui.separator();
    if menu_item(
        ui,
        "Rename view\u{2026}",
        read_only.then_some("The library is read-only; its views cannot be renamed."),
    ) {
        intent = Some(LibraryIntent::RenameView(reference.clone()));
    }
    if menu_item(
        ui,
        "Copy view\u{2026}",
        (!target.writable_library_available)
            .then_some("Every library is read-only; none can take a copy."),
    ) {
        intent = Some(LibraryIntent::CopyView(reference.clone()));
    }
    ui.separator();
    if menu_item(ui, "Where view is used\u{2026}", None) {
        intent = Some(LibraryIntent::ReportUsage(UsageSubject::View(
            reference.clone(),
        )));
    }
    if let Some(lock) = lock_elsewhere(model, &reference)
        && let Some(asked) = file_item(ui, model, lock)
    {
        intent = Some(asked);
    }
    ui.separator();
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

/// Break lock… on the view's file, while someone else holds it: only a
/// library kept in a folder of its own has files others can lock.
fn lock_elsewhere(model: &Model, reference: &CellViewRef) -> Option<LibraryFileIntent> {
    if !model.library_manager.is_referenced(&reference.library) {
        return None;
    }
    let view = model
        .library_manager
        .get_library(&reference.library)?
        .get_cell(&reference.cell)?
        .get_view(&reference.view)?;
    let file = crate::workbench::app_state::model::LibraryFile::cell_view(&reference.cell, view);
    model.library_lock_elsewhere(&reference.library, &file)?;
    Some(LibraryFileIntent::BreakLock {
        library: reference.library.clone(),
        file,
    })
}

/// Why a view of `view_type` does not open read-only, or `None` when it
/// does: only the drawings Design edits have a read-only reference.
fn read_only_open_refusal(view_type: ViewType) -> Option<&'static str> {
    match view_type {
        ViewType::Schematic | ViewType::Testbench | ViewType::Symbol => None,
        _ => Some("Only a schematic, a testbench or a symbol opens read-only."),
    }
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

/// A command over library folders, or the clause its refusal is stated in.
fn file_command(model: &Model, intent: LibraryFileIntent) -> Result<LibraryIntent, &'static str> {
    library_file_refusal(model, &intent).map_or(Ok(LibraryIntent::Files(intent)), Err)
}

/// Why a library kept in a folder of its own cannot be renamed.
const KEEPS_ITS_NAME: &str = "A library in its own folder keeps its name; copy it to rename it.";

/// The menu a library row opens: rename, copy, where used and properties;
/// then its access and where it is kept: for one in a folder of its own its
/// mode, Relink…, Embed in project and Detach library…, for one the project
/// file holds Make read-only or Make editable, Move to its own folder… and
/// Delete library…. Each item is disabled with its reason.
fn library_items(
    ui: &mut Ui,
    model: &Model,
    (library, read_only): (String, bool),
) -> Option<LibraryIntent> {
    let referenced = model.library_manager.is_referenced(&library);
    let mut intent = None;
    let rename_refusal = if referenced {
        Some(KEEPS_ITS_NAME)
    } else {
        read_only.then_some("The library is read-only; it cannot be renamed.")
    };
    if menu_item(ui, "Rename library\u{2026}", rename_refusal) {
        intent = Some(LibraryIntent::RenameLibrary(library.clone()));
    }
    if menu_item(ui, "Copy library\u{2026}", None) {
        intent = Some(LibraryIntent::CopyLibrary(library.clone()));
    }
    if menu_item(ui, "Where used\u{2026}", None) {
        intent = Some(LibraryIntent::ReportUsage(UsageSubject::Library(
            library.clone(),
        )));
    }
    if menu_item(ui, "Library properties\u{2026}", None) {
        intent = Some(LibraryIntent::EditProperties {
            library: library.clone(),
            cell: None,
        });
    }
    ui.separator();
    if !referenced {
        // A library the project file holds takes its access as its
        // properties do.
        let label = if read_only {
            "Make editable"
        } else {
            "Make read-only"
        };
        let refusal = model
            .safe_mode
            .project_read_only()
            .then_some("The project is open read-only.");
        if menu_item(ui, label, refusal) {
            intent = Some(LibraryIntent::SetReadOnly {
                library: library.clone(),
                read_only: !read_only,
            });
        }
    }
    // Refresh… reads every library folder, so it is the bar's.
    for file_intent in library_file_intents(model, &library)
        .into_iter()
        .filter(|file_intent| *file_intent != LibraryFileIntent::Refresh)
    {
        if let Some(asked) = file_item(ui, model, file_intent) {
            intent = Some(asked);
        }
    }
    // A library kept in a folder of its own is detached, never deleted:
    // its files are its folder's.
    if !referenced {
        ui.separator();
        if menu_item(
            ui,
            "Delete library\u{2026}",
            read_only.then_some("The library is read-only; it cannot be deleted."),
        ) {
            intent = Some(LibraryIntent::DeleteLibrary(library));
        }
    }
    intent
}

/// One command over library files as a menu item, disabled with its reason.
fn file_item(ui: &mut Ui, model: &Model, file_intent: LibraryFileIntent) -> Option<LibraryIntent> {
    let blocker = library_file_blocker(model, &file_intent);
    menu_item(ui, library_file_label(&file_intent), blocker.as_deref())
        .then_some(LibraryIntent::Files(file_intent))
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

/// Which of the selected cell and view the Cell and View columns list: a
/// filter can hide either, and then neither the page nor the inspector
/// beside it acts on or describes it.
pub(in crate::workbench) fn shown_selection(model: &Model, state: &ProjectState) -> LibraryShown {
    let find = columns::Find::of(state);
    let cell = selected_cell_shown(model, &find);
    let view = cell
        && model
            .library_manager
            .current_view()
            .is_some_and(|view| find.view(view.view_type));
    LibraryShown { cell, view }
}

/// Whether the selected cell is one the Cell column lists.
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
        LibraryIntent::ReportUsage(subject) => {
            let report = LibraryUsageReport::of(cx.model, subject);
            let summary = report.summary();
            ctx.data_mut(|data| data.insert_temp(egui::Id::new(USAGE_REPORT_ID), report));
            cx.wb.prompt(summary);
        }
        LibraryIntent::NewLibrary => cx.state.open_new_library_dialog(cx.model),
        LibraryIntent::RenameLibrary(library)
            if cx.model.library_manager.is_referenced(&library) =>
        {
            cx.wb.prompt(KEEPS_ITS_NAME);
        }
        LibraryIntent::RenameLibrary(library) => {
            if let Err(error) = cx.state.open_rename_library_dialog(cx.model, &library) {
                cx.wb.prompt(error);
            }
        }
        // A library kept in a folder of its own is not deleted from here:
        // its files are its folder's. It is detached.
        LibraryIntent::DeleteLibrary(library)
            if cx.model.library_manager.is_referenced(&library) =>
        {
            crate::workbench::project::library_dialogs::library_files::run_library_file_intent(
                cx,
                LibraryFileIntent::Detach(library),
            );
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
        LibraryIntent::MoveCell { library, cell } => {
            if let Err(error) = cx.state.open_move_cell_dialog(cx.model, &library, &cell) {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::CopyLibrary(library) => {
            if let Err(error) = cx.state.open_copy_library_dialog(cx.model, &library) {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::CopyView(reference) => {
            if let Err(error) = cx.state.open_copy_view_dialog(cx.model, &reference) {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::OpenReadOnly(reference) => {
            cx.wb
                .outbox
                .push(ProjectRequest::OpenViewReadOnly(reference));
        }
        LibraryIntent::SetReadOnly { library, read_only } => {
            let description = cx
                .model
                .library_manager
                .get_library(&library)
                .map(|found| found.description().to_owned())
                .unwrap_or_default();
            if let Err(error) = cx
                .model
                .edit_library_properties(&library, &description, read_only)
            {
                cx.wb.prompt(error);
            }
        }
        LibraryIntent::EditProperties { library, cell } => {
            if let Err(error) =
                cx.state
                    .open_library_properties_dialog(cx.model, &library, cell.as_deref())
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
        LibraryIntent::Files(intent) => {
            crate::workbench::project::library_dialogs::library_files::run_library_file_intent(
                cx, intent,
            );
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
        // Attach, embed a folder and refresh act on the project's library
        // folders from anywhere; the rest act on the library the page shows.
        LibraryCommand::Attach | LibraryCommand::EmbedFromFolder | LibraryCommand::Refresh
            if !project_open =>
        {
            Err("no project is open")
        }
        LibraryCommand::Attach => file_command(model, LibraryFileIntent::Attach),
        LibraryCommand::EmbedFromFolder => file_command(model, LibraryFileIntent::EmbedFromFolder),
        LibraryCommand::Refresh if model.library_manager.references().next().is_none() => {
            Err("no library is kept in a folder of its own")
        }
        LibraryCommand::Refresh => file_command(model, LibraryFileIntent::Refresh),
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
        LibraryCommand::MoveToFolder
        | LibraryCommand::Embed
        | LibraryCommand::MakeEditable
        | LibraryCommand::MakeReadOnly
        | LibraryCommand::Relink
        | LibraryCommand::Detach => {
            let (library, _) = target.library.ok_or("select a library")?;
            let referenced = model.library_manager.is_referenced(&library);
            let intent = match command {
                LibraryCommand::MoveToFolder if referenced => {
                    return Err("the library is already in a folder of its own");
                }
                LibraryCommand::MoveToFolder => LibraryFileIntent::MoveToFolder(library),
                _ if !referenced => return Err("the library is held in the project"),
                LibraryCommand::Embed => LibraryFileIntent::Embed(library),
                LibraryCommand::MakeEditable => {
                    LibraryFileIntent::SetMode(library, LibraryMode::Editable)
                }
                LibraryCommand::MakeReadOnly => {
                    LibraryFileIntent::SetMode(library, LibraryMode::ReadOnly)
                }
                LibraryCommand::Relink if cfg!(target_arch = "wasm32") => {
                    LibraryFileIntent::ReadFromFolder(library)
                }
                LibraryCommand::Relink => LibraryFileIntent::Relink(library),
                _ => LibraryFileIntent::Detach(library),
            };
            file_command(model, intent)
        }
        LibraryCommand::CopyCell => {
            let (library, cell, _) = target.cell.ok_or("select a cell")?;
            if target.writable_library_available {
                Ok(LibraryIntent::CopyCell { library, cell })
            } else {
                Err("every library is read-only")
            }
        }
        LibraryCommand::CopyLibrary => target
            .library
            .map(|(library, _)| LibraryIntent::CopyLibrary(library))
            .ok_or("select a library"),
        LibraryCommand::CopyView => {
            let (reference, _, _) = target.view.ok_or("select a view")?;
            if target.writable_library_available {
                Ok(LibraryIntent::CopyView(reference))
            } else {
                Err("every library is read-only")
            }
        }
        LibraryCommand::MoveCell => {
            let (library, cell, read_only) = target.cell.ok_or("select a cell")?;
            writable(read_only, LibraryIntent::MoveCell { library, cell })
        }
        LibraryCommand::OpenReadOnly => {
            let (reference, view_type, _) = target.view.ok_or("select a view")?;
            match read_only_open_refusal(view_type) {
                None => Ok(LibraryIntent::OpenReadOnly(reference)),
                Some(_) => Err("only a schematic, a testbench or a symbol opens read-only"),
            }
        }
        LibraryCommand::Properties => match column {
            BrowserColumn::Library => target
                .library
                .map(|(library, _)| LibraryIntent::EditProperties {
                    library,
                    cell: None,
                })
                .ok_or("select a library"),
            BrowserColumn::Cell | BrowserColumn::View => target
                .cell
                .map(|(library, cell, _)| LibraryIntent::EditProperties {
                    library,
                    cell: Some(cell),
                })
                .ok_or("select a cell"),
        },
        LibraryCommand::WhereUsed => match column {
            BrowserColumn::Library => target
                .library
                .map(|(library, _)| LibraryIntent::ReportUsage(UsageSubject::Library(library)))
                .ok_or("select a library"),
            BrowserColumn::Cell => target
                .cell
                .map(|(library, cell, _)| {
                    LibraryIntent::ReportUsage(UsageSubject::Cell { library, cell })
                })
                .ok_or("select a cell"),
            BrowserColumn::View => target
                .view
                .map(|(reference, _, _)| LibraryIntent::ReportUsage(UsageSubject::View(reference)))
                .ok_or("select a view"),
        },
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

fn show_library_usage_report(ctx: &Context) {
    let Some(report) =
        ctx.data(|data| data.get_temp::<LibraryUsageReport>(egui::Id::new(USAGE_REPORT_ID)))
    else {
        return;
    };
    let description = report.summary();
    let choice = Dialog::new("Where used", "Close")
        .description(&description)
        .size(DialogSize::Transaction)
        .show(ctx, |ui| {
            // With nothing to list, the answer is the summary itself, in
            // the body: the description is not shown on every screen.
            if report.placements.is_empty() && report.layouts.is_empty() {
                ui.add(egui::Label::new(&description).wrap());
            }
            if !report.placements.is_empty() {
                property_card(ui, "Schematic placements", |ui| {
                    for (schematic, instance) in &report.placements {
                        property_row(ui, schematic, instance);
                    }
                });
            }
            if !report.layouts.is_empty() {
                property_card(ui, "Layout placements", |ui| {
                    for (layout, placements) in &report.layouts {
                        let count = match placements {
                            1 => "1 placement".to_owned(),
                            count => format!("{count} placements"),
                        };
                        property_row(ui, layout, &count);
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

/// One record the Audit history lists: a change made to a library, or a
/// publication of one.
struct AuditRecord {
    /// The project revision the record left behind, which orders them.
    revision: u64,
    sequence: u64,
    change: String,
    what: String,
    /// When it was made, where the record says: a publication does.
    made_unix_ms: Option<u64>,
    by: String,
    approved_by: String,
    reason: String,
    project_revision: String,
    library_revision: String,
}

impl AuditRecord {
    /// The project's library changes and publications, newest first.
    fn of(project: &crate::state::ProjectDescriptor) -> Vec<Self> {
        let changes = project.library_mutation_audit().iter().map(|receipt| Self {
            revision: receipt.target_project_revision().get(),
            sequence: receipt.sequence(),
            change: receipt.mutation().operation_label().to_owned(),
            what: library_mutation_summary(receipt.mutation()),
            made_unix_ms: None,
            by: String::new(),
            approved_by: String::new(),
            reason: String::new(),
            project_revision: format!(
                "{} \u{2192} {}",
                receipt.source_project_revision().get(),
                receipt.target_project_revision().get()
            ),
            library_revision: format!(
                "{} \u{2192} {}",
                receipt.source_library_revision(),
                receipt.target_library_revision()
            ),
        });
        let publications = project
            .library_publications()
            .iter()
            .map(|publication| Self {
                revision: publication.receipt_project_revision().get(),
                sequence: publication.sequence(),
                change: "Publication".to_owned(),
                what: publication.label().to_owned(),
                made_unix_ms: Some(publication.created_unix_ms()).filter(|made| *made != 0),
                by: publication.actor_id().to_owned(),
                approved_by: publication.authority_id().to_owned(),
                reason: publication.reason().to_owned(),
                project_revision: format!(
                    "{} \u{2192} {}",
                    publication.source_project_revision().get(),
                    publication.receipt_project_revision().get()
                ),
                library_revision: publication.library_revision().to_string(),
            });
        let mut records = changes.chain(publications).collect::<Vec<_>>();
        records.sort_by_key(|record| {
            std::cmp::Reverse((
                record.revision,
                record.made_unix_ms.is_some(),
                record.sequence,
            ))
        });
        records
    }
}

/// The project's library changes and publications in one table sized to the
/// dialog, newest first; where the project and its libraries stand now, and
/// how many of each it records, said above it.
fn show_library_audit_history(
    ctx: &Context,
    model: &Model,
    table: &mut crate::ui::widgets::table::TableState,
) {
    use crate::ui::widgets::table::{Cell, Column, DataTable};
    if !ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new(AUDIT_REPORT_ID))
            .unwrap_or(false)
    }) {
        return;
    }

    let project = &model.workspace.content.project;
    let records = AuditRecord::of(project);
    let description = format!(
        "Project revision {} \u{b7} library revision {}. {} and {}.",
        project.revision().get(),
        model.library_manager.revision(),
        counted(
            project.library_mutation_audit().len(),
            "recorded library change",
            "recorded library changes"
        ),
        counted(
            project.library_publications().len(),
            "publication",
            "publications"
        ),
    );
    let columns = [
        Column::number("Revision", 76.0),
        Column::name("Change", 120.0),
        Column::name("What", 160.0),
        Column::number("When", 128.0),
        Column::name("By", 90.0),
        Column::name("Approved by", 90.0),
        Column::name("Reason", 160.0),
        Column::number("Library revision", 96.0),
    ];
    let choice = Dialog::new("Audit history", "Close")
        .description(&description)
        .size(DialogSize::Manager)
        .flush_body()
        .overlay_scrollbar()
        .show(ctx, |ui| {
            if records.is_empty() {
                crate::ui::panels::empty_state(
                    ui,
                    crate::ui::panels::WorkbenchIcon::History,
                    "Nothing recorded",
                    "No library in this project has been changed or published yet.",
                );
                return;
            }
            let absent = || Cell::absent();
            let words = |text: &str| {
                if text.is_empty() {
                    absent()
                } else {
                    Cell::text(text)
                }
            };
            let _ = DataTable::new(
                "workbench.project.library.audit-history",
                &columns,
                records.len(),
            )
            .state(table)
            .max_height(ui.available_height().max(1.0))
            .show(ui, |index| {
                let record = &records[index];
                vec![
                    Cell::number(&record.project_revision),
                    Cell::text(&record.change),
                    words(&record.what),
                    record.made_unix_ms.map_or_else(absent, |made| {
                        Cell::number(crate::time_compat::local_stamp(made))
                    }),
                    words(&record.by),
                    words(&record.approved_by),
                    words(&record.reason),
                    Cell::number(&record.library_revision),
                ]
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
        | crate::state::ProjectLibraryMutation::DeleteLibrary { library }
        | crate::state::ProjectLibraryMutation::EditProperties {
            library,
            cell: None,
        } => library.clone(),
        crate::state::ProjectLibraryMutation::EditProperties {
            library,
            cell: Some(cell),
        } => format!("{library}/{cell}"),
        crate::state::ProjectLibraryMutation::CopyLibrary {
            source_library,
            target_library,
        } => format!("{source_library} \u{2192} {target_library}"),
        crate::state::ProjectLibraryMutation::MoveCell {
            source_library,
            source_cell,
            target_library,
            target_cell,
        } => format!("{source_library}/{source_cell} \u{2192} {target_library}/{target_cell}"),
        crate::state::ProjectLibraryMutation::CopyView {
            source_library,
            source_cell,
            source_view,
            target_library,
            target_cell,
            target_view,
        } => format!(
            "{source_library}/{source_cell}/{source_view} \u{2192} {target_library}/{target_cell}/{target_view}"
        ),
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

        let report = LibraryUsageReport::of(&harness.model, UsageSubject::View(reference.clone()));
        assert_eq!(report.placements.len(), 2);
        assert!(
            report
                .placements
                .iter()
                .any(|(schematic, _)| schematic.contains("work/consumer/schematic"))
        );
        assert_eq!(
            report.summary(),
            format!(
                "{} is placed 2 times in 2 schematics.",
                reference.display_path()
            )
        );
        let nowhere = LibraryUsageReport {
            placements: Vec::new(),
            ..report
        };
        assert_eq!(
            nowhere.summary(),
            format!(
                "{} is not placed in any schematic or layout of this project.",
                reference.display_path()
            )
        );
    }

    #[test]
    fn where_used_counts_layouts_and_a_librarys_placements_from_outside_it() {
        let cell = LibraryUsageReport {
            subject: UsageSubject::Cell {
                library: "user".to_owned(),
                cell: "amp".to_owned(),
            },
            placements: vec![("user/top/schematic".to_owned(), "I1".to_owned())],
            layouts: vec![("soc/chip/layout".to_owned(), 2)],
        };
        assert_eq!(
            cell.summary(),
            "user/amp is placed once in 1 schematic and 2 times in 1 layout."
        );
        let library = LibraryUsageReport {
            subject: UsageSubject::Library("user".to_owned()),
            placements: Vec::new(),
            ..cell
        };
        assert_eq!(
            library.summary(),
            "Cells of user are placed 2 times in 1 layout outside it."
        );
        let unplaced = LibraryUsageReport {
            layouts: Vec::new(),
            ..library
        };
        assert_eq!(
            unplaced.summary(),
            "No cell of user is placed outside it in this project."
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
