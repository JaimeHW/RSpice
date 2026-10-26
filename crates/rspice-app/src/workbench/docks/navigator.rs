//! Workspace-aware navigation tree.

pub(in crate::workbench) mod design;
pub(in crate::workbench) mod symbol;

pub(in crate::workbench) mod models;
pub(in crate::workbench) mod netlist;
pub(in crate::workbench) mod project;
mod rail;
pub(in crate::workbench) mod results;
mod results_actions;
mod results_rows;
pub(in crate::workbench) mod simulate;
pub(in crate::workbench) mod source_bundle;
pub(in crate::workbench) mod stimulus;
pub(in crate::workbench) mod verify;
pub(in crate::workbench) mod veriloga;

use netlist::*;

use egui::{Key, Modifiers, Response, ScrollArea, Sense, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::framework::host::Point;
use crate::workbench::framework::requests::{FocusNavigatorSearch, Outbox, Request};
use crate::workbench::framework::workbench_host::WorkbenchHost;

use super::super::state::Workspace;
use crate::ui::panels::{PANEL_HEADER_H, WorkbenchIcon};

const PANEL_SEARCH_MARGIN_X: f32 = 8.0;
/// The right inset a filter field keeps while it holds a query, so its text
/// column ends before the control that empties it.
const PANEL_SEARCH_CLEAR_INSET: i8 = 26;
/// The clear control's hit target — the field's own height less its border, so
/// the whole right inset answers the pointer.
const PANEL_SEARCH_CLEAR_TARGET: f32 = 22.0;
/// The crossing strokes inside that target, at the size the search mark
/// opposite it is drawn.
const PANEL_SEARCH_CLEAR_MARK: f32 = 13.0;
const SCHEMATIC_NAV_ROW_HEIGHT: f32 = 24.0;
const SCHEMATIC_NAV_LABEL_SIZE: f32 = tokens::FS_1;
const SCHEMATIC_NAV_META_SIZE: f32 = 10.0;
const EMPTY_HINT_PADDING_X: i8 = 12;
const EMPTY_HINT_PADDING_Y: i8 = 20;
// The mock's 49 px is a minimum. Its three grid rows resolve to 63 px at the
// reviewed 11 px type scale (title/detail row, 4 px gap, status row).
const FLOW_ROW_HEIGHT: f32 = 63.0;
const FLOW_TEXT_LEFT: f32 = 35.0;
const FLOW_LABEL_TOP: f32 = 7.0;
const FLOW_DETAIL_TOP: f32 = 23.0;
const FLOW_STATUS_TOP: f32 = 43.0;
const FLOW_DETAIL_LINE_HEIGHT: f32 = 15.0;
const NETLIST_OUTLINE_ROW_HEIGHT: f32 = 27.0;
const NETLIST_OUTLINE_TOUCH_ROW_HEIGHT: f32 = 44.0;
const NETLIST_OUTLINE_ICON_SIZE: f32 = 14.0;
const NETLIST_OUTLINE_PADDING_X: f32 = 9.0;
const NETLIST_OUTLINE_ICON_GAP: f32 = 7.0;

fn panel_search_field_width(available_width: f32) -> f32 {
    (available_width - PANEL_SEARCH_MARGIN_X * 2.0).max(1.0)
}

pub fn show(ui: &mut Ui, app: &mut impl WorkbenchHost) {
    ui.spacing_mut().item_spacing.y = 0.0;
    let workspace = app.workbench().workspace;
    let section = app.navigator_section(workspace);
    // Canonical navigator order: header, search, then the workspace's own
    // navigator, which may lead with tabs of its own (the Code workspace's
    // page tabs, then the page's outline).
    header(ui, app);
    if section.container_draws_search {
        workspace_search(ui, app, workspace, section.placeholder);
    }
    if let Some(requires_scroll) = section.pins_scroll_bar {
        // The salt the verification flows, the one section that scrolls,
        // have always been scrolled under.
        let scroll_bar_visibility = if requires_scroll(ui.available_height()) {
            egui::scroll_area::ScrollBarVisibility::AlwaysVisible
        } else {
            egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded
        };
        ScrollArea::vertical()
            .id_salt("workbench.verify.navigator")
            .auto_shrink([false, false])
            .scroll_bar_visibility(scroll_bar_visibility)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                app.contribute(Point::Navigator(workspace), ui);
            });
    } else {
        app.contribute(Point::Navigator(workspace), ui);
    }
}

fn header(ui: &mut Ui, app: &mut impl WorkbenchHost) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PANEL_HEADER_H),
        egui::Sense::hover(),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let title = app.navigator_section(app.workbench().workspace).title;
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(title);
        node.set_level(2);
    });
    let size = if app.workbench().workspace == Workspace::Design {
        tokens::FS_2
    } else {
        tokens::FS_0
    };
    ui.painter().text(
        egui::pos2(
            rect.left() + rspice_ui_kit::panels::PANEL_INSET,
            rect.center().y,
        ),
        egui::Align2::LEFT_CENTER,
        title,
        theme::sans(size, FontWeight::SemiBold),
        t.color.text,
    );
}

fn workspace_search(
    ui: &mut Ui,
    app: &mut impl WorkbenchHost,
    workspace: Workspace,
    placeholder: &'static str,
) -> bool {
    let mut focus =
        search_focus_requested::<FocusNavigatorSearch>(ui, &mut app.workbench_mut().outbox);
    panel_search(
        ui,
        app.workbench_mut().navigator_trees.filter_mut(workspace),
        "workbench.navigator.filter",
        placeholder,
        &mut focus,
    )
}

/// Take `R`, the request to focus a search field this pass draws, when the
/// field can take the focus. A drawer first measures an invisible pass; the
/// request waits for the visible one.
pub(super) fn search_focus_requested<R: Request>(ui: &Ui, outbox: &mut Outbox) -> bool {
    ui.is_visible() && outbox.take::<R>().is_some()
}

/// The panel's filter field: the query, the control that empties it, and the
/// two keys a filter standing above a list owes its reader.
///
/// Returns whether Down was pressed in the field — the reader stepping out of
/// the query and into the rows it narrowed. The field consumes that press
/// itself, because only the field knows it had the focus; where it lands is
/// the rail's answer, not this function's.
///
/// Escape empties the field. egui surrenders the focus of a text field on
/// Escape on its own, so the pair is one gesture: the query goes and the
/// keyboard comes back out. An already-empty field keeps its hands off the
/// press, so Escape goes on meaning "put the tool away" everywhere else.
pub(super) fn panel_search(
    ui: &mut Ui,
    query: &mut String,
    id: &'static str,
    placeholder: &'static str,
    focus_pending: &mut bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let field_width = panel_search_field_width(ui.available_width());
    let mut enter_rows = false;
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(PANEL_SEARCH_MARGIN_X);
        // The clear control lives inside the field's own right inset, so the
        // text column gives way to it rather than running underneath it.
        let clearable = !query.is_empty();
        let field_id = ui.make_persistent_id(id);
        // A drawer first measures an invisible area. Retain its focus request
        // until the field can receive input, then focus before processing text.
        if ui.is_visible() && std::mem::take(focus_pending) {
            ui.memory_mut(|memory| memory.request_focus(field_id));
        }
        let response = ui.add_sized(
            [field_width, t.metrics.ctl_h],
            egui::TextEdit::singleline(query)
                .id(field_id)
                .hint_text(placeholder)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .margin(egui::Margin {
                    left: 29,
                    right: if clearable {
                        PANEL_SEARCH_CLEAR_INSET
                    } else {
                        8
                    },
                    top: 5,
                    bottom: 5,
                }),
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label(placeholder);
            node.set_description("Filter the current navigator contents");
        });
        WorkbenchIcon::Search.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(response.rect.left() + 15.0, response.rect.center().y),
                egui::vec2(16.0, 16.0),
            ),
            t.color.text_faint,
        );
        let cleared = clearable && panel_search_clear(ui, id, &response);
        // `lost_focus` as well as `has_focus`: egui takes the focus off a text
        // field the moment it reads the Escape, so by the time the field is
        // painted the press has already cost it the focus it is being asked
        // about.
        let escaped = clearable
            && (response.has_focus() || response.lost_focus())
            && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape));
        if cleared || escaped {
            query.clear();
        }
        if cleared {
            // The pointer emptied the field; the reader is still in it.
            response.request_focus();
        }
        enter_rows = response.has_focus()
            && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowDown));
    });
    ui.add_space(8.0);
    enter_rows
}

/// The control that empties the filter, drawn in the field's right inset.
///
/// The mark is [`WorkbenchIcon::Close`] — the same crossing strokes every
/// other dismissal in the workbench paints — rather than a `✕` set in text.
/// The bundled IBM Plex cuts are Latin subsets and the character is not in
/// them, so a text mark would ship as a replacement box while every test that
/// only reads the announced control still passed.
fn panel_search_clear(ui: &mut Ui, id: &'static str, field: &Response) -> bool {
    let t = Tokens::get(ui.ctx());
    let rect = egui::Rect::from_center_size(
        egui::pos2(
            field.rect.right() - PANEL_SEARCH_CLEAR_INSET as f32 * 0.5 - 2.0,
            field.rect.center().y,
        ),
        egui::Vec2::splat(PANEL_SEARCH_CLEAR_TARGET),
    );
    let response = ui.interact(rect, egui::Id::new((id, "clear")), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Clear the filter",
        )
    });
    WorkbenchIcon::Close.paint(
        ui.painter(),
        egui::Rect::from_center_size(rect.center(), egui::Vec2::splat(PANEL_SEARCH_CLEAR_MARK)),
        if response.hovered() || response.has_focus() {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.clicked()
}

/// Whether one line of the simulation rail survives the navigator's filter.
///
/// `query` arrives trimmed and lower-cased from the panel's own field. An
/// empty one keeps everything and allocates nothing, which is the path the
/// rail takes on nearly every frame; only a reader who is actually filtering
/// pays for the comparison.
///
/// The rail used to spend the query on the nine route rows and nothing else,
/// so a search for a corner name hid every page and left the Run set axes
/// sitting under it, unfiltered and unrelated to what had been typed.
fn nav_matches(query: &str, text: &str) -> bool {
    query.is_empty() || text.to_lowercase().contains(query)
}

/// Thousands-grouped sample count. A six-figure retained length is read as a
/// magnitude, not parsed digit by digit.
pub(super) fn grouped_count(value: usize) -> String {
    let digits = value.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

/// A row's context menu, answered by the pointer and the keyboard alike:
/// Shift+F10 on the focused row opens it. Returns the popup and whether the
/// keyboard opened it this frame.
///
/// A context menu sits at the pointer position it was opened at. A menu
/// raised from the keyboard has no such position, and a popup anchored to a
/// position it does not have draws nothing at all — so anchor that one to
/// the row instead, on every frame it stays open rather than only on the
/// frame that opened it.
fn row_context_menu(response: &egui::Response) -> (egui::Popup<'_>, bool) {
    let keyboard_open = response.has_focus()
        && response
            .ctx
            .input_mut(|input| input.consume_key(egui::Modifiers::SHIFT, egui::Key::F10));
    let popup_id = egui::Popup::default_response_id(response);
    let mut popup = egui::Popup::context_menu(response).id(popup_id);
    if keyboard_open {
        popup = popup.open_memory(Some(egui::SetOpenCommand::Bool(true)));
    }
    if egui::Popup::position_of_id(&response.ctx, popup_id).is_none() {
        popup = popup.anchor(response);
    }
    (popup, keyboard_open)
}

mod verification_footer;

use verification_footer::*;
