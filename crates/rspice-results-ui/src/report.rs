//! Report presentation over borrowed canonical documents and application-owned edits.
pub mod composer;
pub mod dialogs;
pub mod inspector;
pub mod preview;
pub mod session;
use egui::{Color32, Rect, Sense, Stroke, Ui, Vec2};
use rspice_results::report_document::{ReportPageUpdatePolicy, ReportTemplate};
use rspice_ui_kit::theme;
const PAPER: Color32 = Color32::from_rgb(255, 255, 255);
const PAPER_PANEL: Color32 = Color32::from_rgb(246, 247, 247);
const PAPER_TEXT: Color32 = Color32::from_rgb(32, 36, 40);
const PAPER_MUTED: Color32 = Color32::from_rgb(82, 89, 94);
const PAPER_FAINT: Color32 = Color32::from_rgb(98, 105, 110);
const PAPER_BORDER: Color32 = Color32::from_rgb(205, 210, 213);
const PAPER_ACCENT: Color32 = Color32::from_rgb(122, 93, 0);

fn paper_switch(ui: &mut Ui, value: &mut bool) -> egui::Response {
    const TRACK_SIZE: Vec2 = Vec2::new(32.0, 18.0);
    const HIT_SIZE: Vec2 = Vec2::new(40.0, 28.0);
    let (rect, mut response) = ui.allocate_exact_size(HIT_SIZE, Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            *value,
            "Include report element",
        )
    });
    if response.clicked() {
        *value = !*value;
        response.mark_changed();
    }

    let track = Rect::from_center_size(rect.center(), TRACK_SIZE);
    let fill = if *value {
        PAPER_ACCENT
    } else if response.hovered() {
        Color32::from_rgb(225, 228, 229)
    } else {
        PAPER_PANEL
    };
    ui.painter().rect(
        track,
        TRACK_SIZE.y * 0.5,
        fill,
        Stroke::new(1.0, if *value { PAPER_ACCENT } else { PAPER_BORDER }),
        egui::StrokeKind::Inside,
    );
    let knob_x = if *value {
        track.right() - 7.0
    } else {
        track.left() + 7.0
    };
    ui.painter().circle_filled(
        egui::pos2(knob_x, track.center().y),
        5.5,
        if *value { PAPER } else { PAPER_MUTED },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response
}

fn paint_dashed_rect(ui: &Ui, rect: Rect, color: Color32) {
    const DASH: f32 = 4.0;
    const GAP: f32 = 3.0;
    let stroke = Stroke::new(1.0, color);
    let painter = ui.painter();

    let mut x = rect.left();
    while x < rect.right() {
        let end = (x + DASH).min(rect.right());
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(end, rect.top())],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(x, rect.bottom()), egui::pos2(end, rect.bottom())],
            stroke,
        );
        x += DASH + GAP;
    }

    let mut y = rect.top();
    while y < rect.bottom() {
        let end = (y + DASH).min(rect.bottom());
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.left(), end)],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(rect.right(), y), egui::pos2(rect.right(), end)],
            stroke,
        );
        y += DASH + GAP;
    }
}

pub const INITIAL_PAGES: [(&str, &str); 7] = [
    ("1", "Executive summary"),
    ("2", "Design and configuration"),
    ("3", "Nominal results"),
    ("4", "PVT and yield"),
    ("5", "SOA and regression"),
    ("6", "Physical DRC and waivers"),
    ("A", "Run manifests"),
];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PaneSeparators {
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
}

pub fn paint_pane_separators(ui: &Ui, rect: Rect, separators: PaneSeparators, color: Color32) {
    let stroke = Stroke::new(1.0, color);
    if separators.top {
        let y = rect.top() + 0.5;
        ui.painter().line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            stroke,
        );
    }
    if separators.right {
        let x = rect.right() - 0.5;
        ui.painter().line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            stroke,
        );
    }
    if separators.bottom {
        let y = rect.bottom() - 0.5;
        ui.painter().line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            stroke,
        );
    }
}

pub fn report_template_label(template: ReportTemplate) -> &'static str {
    match template {
        ReportTemplate::ReleaseVerification42 => "Release verification 4.2",
        ReportTemplate::DesignReview => "Design review",
        ReportTemplate::ModelQualification => "Model qualification",
    }
}

pub fn page_update_policy_label(policy: ReportPageUpdatePolicy) -> &'static str {
    match policy {
        ReportPageUpdatePolicy::RefreshLinkedAutomatically => "Refresh linked automatically",
        ReportPageUpdatePolicy::FreezeSelectedRevision => "Freeze selected revision",
    }
}

pub fn page_marker(_index: usize, title: &str) -> &str {
    INITIAL_PAGES
        .iter()
        .find(|(_, expected)| *expected == title)
        .map_or("+", |(marker, _)| *marker)
}
