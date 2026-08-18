//! The sheet strip: the sheets of the active drawing, as navigation.
//!
//! A sheet is where the engineer is, not a row in a manager, so the strip is
//! chrome directly beneath the document row and every chip is a live tab. It
//! projects the governed catalog and owns no copy of it: activation, ordering,
//! naming and deletion all publish through `app::sheets`, which is what keeps
//! a chip, a chord and the design manager from disagreeing about which sheet
//! is current.
//!
//! Rename and drag state are the only things retained here, and they live in
//! egui memory for exactly as long as the gesture does. A drag interrupted by
//! a sheet switch is abandoned rather than committed: the order it was aiming
//! at describes a catalog the session has already left.

use egui::{Frame, Id, Layout, Panel, Sense, Ui, Vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app::sheets::{self as sheet_actions, SheetDeletePlan, SheetEntry};
use crate::workbench::{AppState, RSpiceApp};

use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;

const CHIP_MAX_WIDTH: f32 = 190.0;
const CHIP_PADDING_X: f32 = 10.0;
const CHIP_CONTENT_GAP: f32 = 7.0;
const CHIP_ACTIVE_EDGE: f32 = 2.0;
const ADD_CHIP_WIDTH: f32 = 30.0;
const ADD_ICON_SIZE: f32 = 14.0;
const CARET_WIDTH: f32 = 2.0;
const RENAME_WIDTH: f32 = 132.0;

/// Whether the active cell view has enough sheets for the strip to mean
/// anything. One sheet is a drawing, not a set of them.
pub(in crate::workbench) fn is_visible(state: &AppState) -> bool {
    sheet_actions::sheet_count(state) > 1
}

pub fn show(root: &mut Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    if !is_visible(&app.state) {
        clear_gestures(root.ctx());
        return;
    }
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    let t = Tokens::get(ctx);
    let entries = sheet_actions::sheet_entries(&app.state);
    let active = sheet_actions::active_sheet_id(&app.state);
    let mut action = StripAction::None;
    // A context menu opens inside the chip's own pass, which no longer has the
    // application to ask what deleting that sheet would do. Resolve every
    // outcome first so each menu row can state its own consequence.
    let plans = DeletePlans(
        entries
            .iter()
            .map(|entry| (entry.id, sheet_actions::plan_delete(&app.state, entry.id)))
            .collect(),
    );
    ctx.data_mut(|data| data.insert_temp(delete_plans_id(), plans));

    Panel::top("workbench.sheet_strip")
        .exact_size(layout.sheet_strip_height)
        .frame(Frame::new().fill(t.color.bg_app))
        .show_separator_line(false)
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                (rect.bottom() - 0.5).max(rect.top()),
                egui::Stroke::new(1.0, t.color.border),
            );
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width().max(1.0), layout.sheet_strip_height),
                    Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        egui::ScrollArea::horizontal()
                            .id_salt("workbench.sheet_strip.scroll")
                            .scroll_bar_visibility(
                                egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                            )
                            .show(ui, |ui| {
                                let chips = ui.horizontal(|ui| {
                                    action = sheet_chips(
                                        ui,
                                        &entries,
                                        active,
                                        layout.sheet_strip_height,
                                    );
                                });
                                ui.ctx().accesskit_node_builder(chips.response.id, |node| {
                                    node.set_role(egui::accesskit::Role::TabList);
                                    node.set_label("Sheets in this drawing");
                                });
                            });
                    },
                );
            });
        });

    apply(app, action, &entries);
}

/// What one frame of the strip asked for. The chips paint while the catalog is
/// borrowed for reading, so the mutation is named here and run afterwards.
#[derive(Debug, Clone, PartialEq, Eq)]
enum StripAction {
    None,
    Activate(crate::state::SheetId),
    NewSheetAfter(Option<crate::state::SheetId>),
    Delete(crate::state::SheetId),
    MoveSelection(crate::state::SheetId),
    PageSetup(crate::state::SheetId),
    Reorder {
        sheet: crate::state::SheetId,
        to_index: usize,
    },
    Rename {
        sheet: crate::state::SheetId,
        name: String,
    },
}

fn apply(app: &mut RSpiceApp, action: StripAction, entries: &[SheetEntry]) {
    match action {
        StripAction::None => {}
        StripAction::Activate(id) => {
            let _ = sheet_actions::activate_sheet(&mut app.state, id);
        }
        StripAction::NewSheetAfter(after) => {
            let _ = sheet_actions::new_sheet_after(&mut app.state, after);
        }
        StripAction::Delete(id) => {
            let _ = sheet_actions::delete_sheet(&mut app.state, id);
        }
        StripAction::MoveSelection(id) => {
            let _ = sheet_actions::move_selection_to_sheet(&mut app.state, id);
        }
        StripAction::PageSetup(id) => {
            let _ = sheet_actions::activate_sheet(&mut app.state, id);
            crate::workbench::app::open_drawing_sheet_setup_for_state(&mut app.state);
        }
        StripAction::Reorder { sheet, to_index } => {
            let to_index = to_index.min(entries.len().saturating_sub(1));
            let _ = sheet_actions::move_sheet(&mut app.state, sheet, to_index);
        }
        StripAction::Rename { sheet, name } => {
            let _ = sheet_actions::rename_sheet(&mut app.state, sheet, name);
        }
    }
}

/// The sheet being renamed and the text typed for it so far.
#[derive(Clone, Default, PartialEq, Eq)]
struct RenameGesture {
    sheet: Option<crate::state::SheetId>,
    draft: String,
}

/// The sheet being dragged, the sheet that was active when the drag started,
/// and the slot the caret is currently showing.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct DragGesture {
    sheet: Option<crate::state::SheetId>,
    started_on: Option<crate::state::SheetId>,
    target: Option<usize>,
}

fn rename_id() -> Id {
    Id::new("workbench.sheet_strip.rename")
}

fn drag_id() -> Id {
    Id::new("workbench.sheet_strip.drag")
}

fn rename_gesture(ctx: &egui::Context) -> RenameGesture {
    ctx.data(|data| data.get_temp::<RenameGesture>(rename_id()))
        .unwrap_or_default()
}

fn set_rename_gesture(ctx: &egui::Context, gesture: RenameGesture) {
    ctx.data_mut(|data| data.insert_temp(rename_id(), gesture));
}

fn drag_gesture(ctx: &egui::Context) -> DragGesture {
    ctx.data(|data| data.get_temp::<DragGesture>(drag_id()))
        .unwrap_or_default()
}

fn set_drag_gesture(ctx: &egui::Context, gesture: DragGesture) {
    ctx.data_mut(|data| data.insert_temp(drag_id(), gesture));
}

fn clear_gestures(ctx: &egui::Context) {
    set_rename_gesture(ctx, RenameGesture::default());
    set_drag_gesture(ctx, DragGesture::default());
}

fn sheet_chips(
    ui: &mut Ui,
    entries: &[SheetEntry],
    active: Option<crate::state::SheetId>,
    height: f32,
) -> StripAction {
    let ctx = ui.ctx().clone();
    let mut rename = rename_gesture(&ctx);
    let mut drag = drag_gesture(&ctx);
    // A sheet switch retires an in-flight drag: the order it was aiming at
    // describes a catalog the session has already left.
    if drag.sheet.is_some() && drag.started_on != active {
        drag = DragGesture::default();
    }
    if rename
        .sheet
        .is_some_and(|sheet| !entries.iter().any(|entry| entry.id == sheet))
    {
        rename = RenameGesture::default();
    }

    let mut action = StripAction::None;
    let mut caret = None;
    for (index, entry) in entries.iter().enumerate() {
        let selected = active == Some(entry.id);
        let editing = rename.sheet == Some(entry.id);
        let response = ui
            .push_id(("sheet", entry.id), |ui| {
                if editing {
                    rename_chip(ui, &mut rename, height)
                } else {
                    sheet_chip(ui, entry, selected, height)
                }
            })
            .inner;

        if editing {
            match response.outcome {
                RenameOutcome::Pending => {}
                RenameOutcome::Commit => {
                    let name = rename.draft.trim().to_owned();
                    rename = RenameGesture::default();
                    if !name.is_empty() && name != entry.name {
                        action = StripAction::Rename {
                            sheet: entry.id,
                            name,
                        };
                    }
                }
                RenameOutcome::Cancel => rename = RenameGesture::default(),
            }
            continue;
        }

        if response.response.drag_started() {
            drag = DragGesture {
                sheet: Some(entry.id),
                started_on: active,
                target: Some(index),
            };
        }
        if drag.sheet.is_some()
            && response.response.contains_pointer()
            && ui.input(|input| input.pointer.is_decidedly_dragging())
        {
            drag.target = Some(index);
        }
        if drag.target == Some(index) && drag.sheet.is_some() && drag.sheet != Some(entry.id) {
            caret = Some(response.response.rect.left());
        }
        if response.response.drag_stopped() {
            if let (Some(sheet), Some(target)) = (drag.sheet, drag.target)
                && drag.started_on == active
                && Some(target) != entries.iter().position(|candidate| candidate.id == sheet)
            {
                action = StripAction::Reorder {
                    sheet,
                    to_index: target,
                };
            }
            drag = DragGesture::default();
        } else if response.response.clicked() && !selected {
            action = StripAction::Activate(entry.id);
        }

        if let Some(menu) = response.menu {
            match menu {
                ChipMenu::Rename => {
                    rename = RenameGesture {
                        sheet: Some(entry.id),
                        draft: entry.name.clone(),
                    };
                }
                ChipMenu::Delete => action = StripAction::Delete(entry.id),
                ChipMenu::MoveSelection => action = StripAction::MoveSelection(entry.id),
                ChipMenu::NewAfter => action = StripAction::NewSheetAfter(Some(entry.id)),
                ChipMenu::PageSetup => action = StripAction::PageSetup(entry.id),
            }
        }
    }

    if add_chip(ui, height).clicked() {
        action = StripAction::NewSheetAfter(active);
    }
    if let Some(x) = caret {
        let t = Tokens::get(ui.ctx());
        let rect = ui.max_rect();
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(x - CARET_WIDTH * 0.5, rect.top()),
                egui::pos2(x + CARET_WIDTH * 0.5, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }

    set_rename_gesture(&ctx, rename);
    set_drag_gesture(&ctx, drag);
    action
}

enum ChipMenu {
    Rename,
    Delete,
    MoveSelection,
    NewAfter,
    PageSetup,
}

enum RenameOutcome {
    Pending,
    Commit,
    Cancel,
}

struct ChipResponse {
    response: egui::Response,
    menu: Option<ChipMenu>,
    outcome: RenameOutcome,
}

fn sheet_chip(ui: &mut Ui, entry: &SheetEntry, selected: bool, height: f32) -> ChipResponse {
    let t = Tokens::get(ui.ctx());
    let index_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let name_font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let index = format!("{}", entry.page);
    let index_width = ui
        .painter()
        .layout_no_wrap(index.clone(), index_font.clone(), t.color.text)
        .size()
        .x;
    let name_width = ui
        .painter()
        .layout_no_wrap(entry.name.clone(), name_font.clone(), t.color.text)
        .size()
        .x;
    let width = (CHIP_PADDING_X + index_width + CHIP_CONTENT_GAP + name_width + CHIP_PADDING_X)
        .min(CHIP_MAX_WIDTH);
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(width, height), Sense::click_and_drag());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            &entry.name,
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        node.set_selected(selected);
        node.set_description(format!("Sheet {} of this drawing", entry.page));
    });

    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.right(), rect.top() + CHIP_ACTIVE_EDGE),
            ),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().vline(
        rect.right(),
        rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );

    let text_colour = if selected {
        t.color.text
    } else {
        t.color.text_dim
    };
    ui.painter().text(
        egui::pos2(rect.left() + CHIP_PADDING_X, rect.center().y),
        egui::Align2::LEFT_CENTER,
        &index,
        index_font,
        t.color.text_faint,
    );
    let name_left = rect.left() + CHIP_PADDING_X + index_width + CHIP_CONTENT_GAP;
    let clip = egui::Rect::from_min_max(
        egui::pos2(name_left, rect.top()),
        egui::pos2(
            (rect.right() - CHIP_PADDING_X).max(name_left),
            rect.bottom(),
        ),
    );
    ui.painter().with_clip_rect(clip).text(
        egui::pos2(name_left, rect.center().y),
        egui::Align2::LEFT_CENTER,
        &entry.name,
        name_font,
        text_colour,
    );
    theme::paint_focus_ring(ui, &response, rect);

    let mut menu = None;
    egui::Popup::context_menu(&response).show(|ui| {
        ui.set_min_width(200.0);
        if ui.button("Rename\u{2026}").clicked() {
            menu = Some(ChipMenu::Rename);
            ui.close();
        }
        if ui.button("New sheet after this").clicked() {
            menu = Some(ChipMenu::NewAfter);
            ui.close();
        }
        if ui.button("Move selection here").clicked() {
            menu = Some(ChipMenu::MoveSelection);
            ui.close();
        }
        if ui.button("Page setup\u{2026}").clicked() {
            menu = Some(ChipMenu::PageSetup);
            ui.close();
        }
        ui.separator();
        let delete = delete_label(ui, entry.id);
        if ui.button(delete).clicked() {
            menu = Some(ChipMenu::Delete);
            ui.close();
        }
    });

    ChipResponse {
        response,
        menu,
        outcome: RenameOutcome::Pending,
    }
}

/// What deleting this sheet does, said in the control rather than discovered
/// afterwards. The catalog's own policy decides, so the label changes with it.
fn delete_label(ui: &Ui, id: crate::state::SheetId) -> String {
    let plan = ui
        .ctx()
        .data(|data| data.get_temp::<DeletePlans>(delete_plans_id()));
    match plan.and_then(|plans| plans.plan(id)) {
        Some(SheetDeletePlan::MovesObjects {
            objects,
            destination_name,
            ..
        }) => format!("Delete \u{00b7} move {objects} object(s) to `{destination_name}`"),
        Some(SheetDeletePlan::Blocked { objects }) => {
            format!("Delete \u{00b7} blocked: sheet holds {objects} object(s)")
        }
        Some(SheetDeletePlan::LastSheet) => "Delete \u{00b7} the last sheet is retained".to_owned(),
        Some(SheetDeletePlan::Empty) | None => "Delete".to_owned(),
    }
}

/// The delete outcome for every chip, resolved once per frame before the chips
/// paint. A context menu opens inside the chip's own pass, which no longer has
/// the application to ask.
#[derive(Clone, Default)]
struct DeletePlans(Vec<(crate::state::SheetId, SheetDeletePlan)>);

impl DeletePlans {
    fn plan(&self, id: crate::state::SheetId) -> Option<SheetDeletePlan> {
        self.0
            .iter()
            .find(|(candidate, _)| *candidate == id)
            .map(|(_, plan)| plan.clone())
    }
}

fn delete_plans_id() -> Id {
    Id::new("workbench.sheet_strip.delete_plans")
}

fn rename_chip(ui: &mut Ui, rename: &mut RenameGesture, height: f32) -> ChipResponse {
    let response = ui
        .allocate_ui_with_layout(
            Vec2::new(RENAME_WIDTH, height),
            Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.add_sized(
                    Vec2::new(RENAME_WIDTH, (height - 6.0).max(16.0)),
                    egui::TextEdit::singleline(&mut rename.draft).id(rename_id().with("edit")),
                )
            },
        )
        .inner;
    // Focus is claimed once, on the pass the editor appears. Re-claiming it
    // every pass would trap the pointer inside a chip until Enter or Escape.
    if !response.has_focus() && !response.lost_focus() {
        response.request_focus();
    }
    let outcome = if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
        RenameOutcome::Cancel
    } else if ui.input(|input| input.key_pressed(egui::Key::Enter)) || response.lost_focus() {
        RenameOutcome::Commit
    } else {
        RenameOutcome::Pending
    };
    ChipResponse {
        response,
        menu: None,
        outcome,
    }
}

fn add_chip(ui: &mut Ui, height: f32) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ADD_CHIP_WIDTH, height), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), "New sheet")
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    WorkbenchIcon::Add.paint(
        ui.painter(),
        egui::Rect::from_center_size(rect.center(), Vec2::splat(ADD_ICON_SIZE)),
        if response.hovered() {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text("New sheet")
}

#[cfg(test)]
mod tests;
