//! Viewer columns over the application's live toolbar, library, stage and inspector adapters.

pub mod library;
pub mod toolbar;

use super::widgets::section_heading;
use egui::{Align, Layout, Rect, Sense, Stroke, Ui, vec2};
use rspice_results::studio_presentation::VisualizationSection;
use rspice_ui_kit::tokens::Tokens;

pub trait ViewerHost {
    fn toolbar(&mut self, ui: &mut Ui, compact: bool);
    fn library(&mut self, ui: &mut Ui);
    fn stage(&mut self, ui: &mut Ui);
    fn inspector(&mut self, ui: &mut Ui, compact: bool);
}

const NARROW_VIEWER_BREAKPOINT: f32 = 1_100.0;

fn viewer_column_rects(rect: Rect, library_width: f32, inspector_width: f32) -> [Rect; 3] {
    let library = Rect::from_min_size(rect.min, vec2(library_width, rect.height()));
    let inspector = Rect::from_min_max(
        egui::pos2(rect.right() - inspector_width, rect.top()),
        rect.max,
    );
    let stage = Rect::from_min_max(
        egui::pos2(library.right() + 1.0, rect.top()),
        egui::pos2(inspector.left() - 1.0, rect.bottom()),
    );
    [library, stage, inspector]
}

fn visible_available_width(available: f32, cursor_left: f32, clip_right: f32) -> f32 {
    available.min((clip_right - cursor_left).max(1.0)).max(1.0)
}

pub fn show(ui: &mut Ui, host: &mut impl ViewerHost, compact: bool) {
    section_heading(ui, VisualizationSection::Viewers);
    host.toolbar(ui, compact);
    let height = ui.available_height().max(1.0);
    if compact {
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), height),
            Layout::top_down(Align::Min),
            |ui| host.stage(ui),
        );
        return;
    }

    // A horizontally scrollable ancestor may expose a logical available
    // width wider than the visible canvas. The mockup columns are viewport
    // columns, so clamp their allocation to the active clip rectangle.
    let available = visible_available_width(
        ui.available_width(),
        ui.cursor().left(),
        ui.clip_rect().right(),
    );
    let (library_width, inspector_width) = if available <= NARROW_VIEWER_BREAKPOINT {
        (158.0, 196.0)
    } else {
        (190.0, 224.0)
    };
    // `allocate_ui_with_layout` is allowed to grow beyond its requested size
    // when a descendant reports a larger minimum. The exact-data table and
    // long status strings therefore used to steal width from the inspector at
    // 1280 px even though the mockup declares fixed 190/224 px side columns.
    // Reserve and clip all three column rectangles up front so content can
    // scroll or elide within its owner, never resize a sibling pane.
    let (rect, _) = ui.allocate_exact_size(vec2(available, height), Sense::hover());
    let [library_rect, stage_rect, inspector_rect] =
        viewer_column_rects(rect, library_width, inspector_width);
    let t = Tokens::get(ui.ctx());
    for x in [library_rect.right() + 0.5, inspector_rect.left() - 0.5] {
        ui.painter()
            .vline(x, rect.y_range(), Stroke::new(1.0, t.color.border_strong));
    }

    let mut library_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(library_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    library_ui.set_clip_rect(library_ui.clip_rect().intersect(library_rect));
    host.library(&mut library_ui);

    let mut stage_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(stage_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    stage_ui.set_clip_rect(stage_ui.clip_rect().intersect(stage_rect));
    host.stage(&mut stage_ui);

    let mut inspector_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(inspector_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    inspector_ui.set_clip_rect(inspector_ui.clip_rect().intersect(inspector_rect));
    host.inspector(&mut inspector_ui, false);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn viewer_columns_preserve_the_mockup_side_widths_exactly() {
        assert_eq!(visible_available_width(1_312.0, 50.0, 1_280.0), 1_230.0);
        let desktop = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_230.0, 540.0));
        let [library, stage, inspector] = viewer_column_rects(desktop, 190.0, 224.0);
        assert_eq!(library.width(), 190.0);
        assert_eq!(inspector.width(), 224.0);
        assert_eq!(stage.width(), 814.0);
        assert_eq!(stage.left() - library.right(), 1.0);
        assert_eq!(inspector.left() - stage.right(), 1.0);

        let tablet = Rect::from_min_size(egui::Pos2::ZERO, vec2(900.0, 430.0));
        let [library, stage, inspector] = viewer_column_rects(tablet, 158.0, 196.0);
        assert_eq!(library.width(), 158.0);
        assert_eq!(inspector.width(), 196.0);
        assert_eq!(stage.width(), 544.0);
    }
}
