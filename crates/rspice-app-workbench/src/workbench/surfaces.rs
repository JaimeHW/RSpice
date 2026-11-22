//! Canonical task-owner surfaces.

use egui::{Align, Layout, Rect, Sense, Ui, UiBuilder, WidgetInfo, WidgetType};

use crate::ui::tokens::Tokens;
use crate::workbench::framework::host::Point;
use crate::workbench::framework::requests::Drain;
use crate::workbench::framework::workbench_host::WorkbenchHost;
use crate::workbench::state::workbench_session::{ResultsSplitAxis, ResultsSplitLayout};

pub fn show(ui: &mut Ui, app: &mut impl WorkbenchHost) {
    // The page that draws a source editor publishes it again below; whatever
    // else is on screen has none for the Edit menu to act on.
    app.workbench_mut().active_source_editor = None;
    if split_results_is_visible(app) {
        show_split_with_results(ui, app);
    } else {
        show_primary(ui, app);
    }
    if app.workbench().outbox.waits_at(Drain::SourceEditor) {
        if app.workbench().active_source_editor.is_some() {
            // Asked after the editor was drawn: it takes it next frame.
            ui.ctx().request_repaint();
        } else {
            // No editor is on screen to take it.
            app.workbench_mut().outbox.decline_at(Drain::SourceEditor);
        }
    }
}

/// The stage of the workspace that is showing, drawn by whoever owns it.
fn show_primary(ui: &mut Ui, app: &mut impl WorkbenchHost) {
    let workspace = app.workbench().workspace;
    app.contribute(Point::Surface(workspace), ui);
}

fn split_results_is_visible(app: &impl WorkbenchHost) -> bool {
    app.workbench().results_split_visible(
        app.platform().project_lifecycle.is_open(),
        app.model()
            .simulation
            .retained
            .has_retained_result_dataset(),
    )
}

/// The thinnest either pane can be dragged to, in points: a plot keeps room
/// for its axis and a trace, a drawing for a part.
const SPLIT_PANE_MIN: f32 = 96.0;
/// How far either side of the drawn divider a drag takes hold of it.
const SPLIT_DIVIDER_GRAB: f32 = 3.0;

/// The document's pane, the divider and the plot's pane, as `layout` shares
/// `stage` between them: the plot below the document or beside it, the
/// document keeping its share along that axis, and neither pane thinner
/// than a plot or a drawing can use while the stage has room for both.
pub fn split_stage_rects(stage: Rect, layout: ResultsSplitLayout) -> (Rect, Rect, Rect) {
    let below = layout.axis == ResultsSplitAxis::Below;
    let extent = if below { stage.height() } else { stage.width() }.max(0.0);
    let divider = 1.0_f32.min(extent);
    let room = (extent - divider).max(0.0);
    let share = if layout.document_share.is_finite() {
        layout.document_share.clamp(0.0, 1.0)
    } else {
        ResultsSplitLayout::DEFAULT_DOCUMENT_SHARE
    };
    let mut first = room * share;
    if room >= 2.0 * SPLIT_PANE_MIN {
        first = first.clamp(SPLIT_PANE_MIN, room - SPLIT_PANE_MIN);
    }
    if below {
        let primary = Rect::from_min_max(stage.min, egui::pos2(stage.right(), stage.top() + first));
        let divider = Rect::from_min_max(
            primary.left_bottom(),
            egui::pos2(stage.right(), primary.bottom() + divider),
        );
        let secondary = Rect::from_min_max(divider.left_bottom(), stage.right_bottom());
        (primary, divider, secondary)
    } else {
        let primary =
            Rect::from_min_max(stage.min, egui::pos2(stage.left() + first, stage.bottom()));
        let divider = Rect::from_min_max(
            primary.right_top(),
            egui::pos2(primary.right() + divider, stage.bottom()),
        );
        let secondary = Rect::from_min_max(divider.right_top(), stage.right_bottom());
        (primary, divider, secondary)
    }
}

/// The document's share of `stage` that puts the divider under `pointer`.
fn share_at(stage: Rect, axis: ResultsSplitAxis, pointer: egui::Pos2) -> f32 {
    let (offset, extent) = match axis {
        ResultsSplitAxis::Below => (pointer.y - stage.top(), stage.height()),
        ResultsSplitAxis::Beside => (pointer.x - stage.left(), stage.width()),
    };
    let room = (extent - 1.0).max(1.0);
    (offset / room).clamp(0.0, 1.0)
}

fn show_split_with_results(ui: &mut Ui, app: &mut impl WorkbenchHost) {
    let stage = ui.available_rect_before_wrap();
    let (_, stage_response) = ui.allocate_exact_size(stage.size(), Sense::hover());
    let stage = stage_response.rect;
    let layout = app.workbench().results_split;
    let (primary_rect, divider_rect, results_rect) = split_stage_rects(stage, layout);

    let mut primary = ui.new_child(
        UiBuilder::new()
            .max_rect(primary_rect)
            .layout(Layout::top_down(Align::LEFT)),
    );
    primary.set_clip_rect(primary_rect);
    primary.set_min_size(primary_rect.size());
    primary.set_max_size(primary_rect.size());
    show_primary(&mut primary, app);

    register_results_split_region(ui, results_rect);

    let mut results = ui.new_child(
        UiBuilder::new()
            .max_rect(results_rect)
            .layout(Layout::top_down(Align::LEFT)),
    );
    results.set_clip_rect(results_rect);
    results.set_min_size(results_rect.size());
    results.set_max_size(results_rect.size());
    app.contribute(Point::SplitResults, &mut results);

    // The divider comes last, so it is drawn over the edges of both panes
    // and its grip, which reaches a few points into each, is above what
    // they draw there and takes the pointer first.
    if let Some(share) = split_divider(ui, stage, divider_rect, layout) {
        app.workbench_mut().results_split.document_share = share;
    }
}

/// Draw the divider between the document and its plot and take a drag of
/// it, as the docks' splitters take theirs: the pointer shows which way it
/// moves, the line lights while it is held, and a double click puts it back
/// where it started. Returns the document's new share when it moved.
fn split_divider(
    ui: &mut Ui,
    stage: Rect,
    divider: Rect,
    layout: ResultsSplitLayout,
) -> Option<f32> {
    let grab = match layout.axis {
        ResultsSplitAxis::Below => divider.expand2(egui::vec2(0.0, SPLIT_DIVIDER_GRAB)),
        ResultsSplitAxis::Beside => divider.expand2(egui::vec2(SPLIT_DIVIDER_GRAB, 0.0)),
    };
    let response = ui.interact(
        grab,
        ui.make_persistent_id("results-split-divider"),
        Sense::click_and_drag(),
    );
    let t = Tokens::get(ui.ctx());
    let held = response.hovered() || response.dragged();
    if held {
        ui.ctx().set_cursor_icon(match layout.axis {
            ResultsSplitAxis::Below => egui::CursorIcon::ResizeRow,
            ResultsSplitAxis::Beside => egui::CursorIcon::ResizeColumn,
        });
    }
    let color = if held {
        t.color.accent
    } else {
        t.color.border_strong
    };
    ui.painter().rect_filled(divider, 0.0, color);
    let share = layout.document_share;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Splitter);
        node.set_label("Plot divider");
        node.set_numeric_value(f64::from(share));
        node.set_min_numeric_value(0.0);
        node.set_max_numeric_value(1.0);
    });
    if response.double_clicked() {
        return Some(ResultsSplitLayout::DEFAULT_DOCUMENT_SHARE);
    }
    if response.dragged() {
        return response
            .interact_pointer_pos()
            .map(|pointer| share_at(stage, layout.axis, pointer));
    }
    None
}

pub fn register_results_split_region(ui: &mut Ui, results_rect: Rect) {
    let region = ui.interact(
        results_rect,
        ui.make_persistent_id("results-split-pane"),
        Sense::hover(),
    );
    region.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Other, ui.is_enabled(), "Results split pane")
    });
    ui.ctx().accesskit_node_builder(region.id, |node| {
        node.set_role(egui::accesskit::Role::Region);
        node.set_label("Results split pane");
    });
}
