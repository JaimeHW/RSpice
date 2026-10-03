//! Results tab strips and instrument controls over caller-owned selection.

pub mod bars;
pub mod instrument;
pub mod persistent;

use egui::{Ui, WidgetInfo, WidgetType};
use rspice_results::result_presentation::ResultViewer;
use rspice_ui_kit::{
    panels::WorkbenchIcon,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};

const fn viewer_tab_icon(viewer: ResultViewer) -> WorkbenchIcon {
    match viewer {
        ResultViewer::Waves | ResultViewer::DcSweep | ResultViewer::NoiseContrib => {
            WorkbenchIcon::Results
        }
        ResultViewer::Bode
        | ResultViewer::Fft
        | ResultViewer::HarmonicBalance
        | ResultViewer::PhaseNoise
        | ResultViewer::Hist
        | ResultViewer::Scatter
        | ResultViewer::BoxViolin
        | ResultViewer::Contribution => WorkbenchIcon::Results,
        ResultViewer::Eye
        | ResultViewer::Nyquist
        | ResultViewer::Smith
        | ResultViewer::Polar
        | ResultViewer::PoleZero => WorkbenchIcon::Target,
        ResultViewer::Op
        | ResultViewer::NetworkMatrix
        | ResultViewer::TransferFunction
        | ResultViewer::Specs
        | ResultViewer::Soa
        | ResultViewer::Events
        | ResultViewer::Table => WorkbenchIcon::Grid,
        ResultViewer::Optimization => WorkbenchIcon::Results,
        ResultViewer::Manifest => WorkbenchIcon::Layers,
    }
}

/// The Results shell own bar geometry, resolved against the pointer.
///
/// These were fixed numbers, so the workspace kept its workstation rows on a
/// tablet while `chip` and `IconButton` grew to the 44 px target the rest of
/// the shell already honours — a control taller than the band holding it.
/// Each row keeps the mockup fine-pointer height, or the touch target plus
/// that row own chrome, whichever the pointer calls for.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResultBarMetrics {
    pub viewer_tabs: f32,
    pub viewer_tab: f32,
    pub sheet_bar: f32,
    pub structured_strip: f32,
    pub instrument_control: f32,
}

/// Space a row keeps around its control at the mockup fine-pointer sizes:
/// 41 − 30 for the tab strip, 31 − 23 for the instrument bar.
const RESULT_TAB_STRIP_CHROME: f32 = 11.0;
const RESULT_CONTROL_ROW_CHROME: f32 = 8.0;

impl ResultBarMetrics {
    pub fn resolve(tokens: &Tokens) -> Self {
        let fine = Self {
            viewer_tabs: 41.0,
            viewer_tab: 30.0,
            sheet_bar: 31.0,
            structured_strip: 40.0,
            instrument_control: 23.0,
        };
        if !tokens.metrics.is_touch() {
            return fine;
        }
        let target = tokens.metrics.ctl_h;
        Self {
            viewer_tabs: fine.viewer_tabs.max(target + RESULT_TAB_STRIP_CHROME),
            viewer_tab: fine.viewer_tab.max(target),
            sheet_bar: fine.sheet_bar.max(target + RESULT_CONTROL_ROW_CHROME),
            structured_strip: fine
                .structured_strip
                .max(target + RESULT_CONTROL_ROW_CHROME),
            instrument_control: fine.instrument_control.max(target),
        }
    }

    pub fn of(ui: &Ui) -> Self {
        Self::resolve(&Tokens::get(ui.ctx()))
    }
}

/// Horizontal viewer-tab list with the mockup's overflow chevrons: 20×30
/// paddles flanking the list, present only while it genuinely overflows,
/// stepping the scroll position by the mockup's 220 px increment and
/// disabling at either extreme.
pub fn viewer_tab_scroller(ui: &mut Ui, salt: &'static str, add_tabs: impl FnOnce(&mut Ui)) {
    const SCROLL_STEP: f32 = 220.0;
    const CHEVRON_WIDTH: f32 = 20.0;
    let overflow_id = egui::Id::new(("results.viewer-tabs.overflow", salt));
    // Measured on the previous frame — the leading chevron must reserve its
    // width before the list lays out.
    let (overflowing, at_start, at_end) = ui
        .ctx()
        .data(|data| data.get_temp::<(bool, bool, bool)>(overflow_id))
        .unwrap_or((false, true, true));
    let mut step = 0.0_f32;
    if overflowing && viewer_tab_overflow_chevron(ui, -1.0, !at_start) {
        step -= SCROLL_STEP;
    }
    let trailing = if overflowing { CHEVRON_WIDTH } else { 0.0 };
    let width = (ui.available_width() - trailing).max(1.0);
    let output = egui::ScrollArea::horizontal()
        .id_salt(salt)
        .max_width(width)
        .auto_shrink([false, true])
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
        .show(ui, |ui| {
            let row = ui.horizontal(|ui| add_tabs(ui));
            ui.ctx().accesskit_node_builder(row.response.id, |node| {
                node.set_role(egui::accesskit::Role::TabList);
                node.set_label("Compatible result viewers");
            });
        });
    if overflowing && viewer_tab_overflow_chevron(ui, 1.0, !at_end) {
        step += SCROLL_STEP;
    }
    let visible = output.inner_rect.width();
    let content = output.content_size.x;
    let max_offset = (content - visible).max(0.0);
    let mut state = output.state;
    if step != 0.0 {
        state.offset.x = (state.offset.x + step).clamp(0.0, max_offset);
        state.store(ui.ctx(), output.id);
        ui.ctx().request_repaint();
    }
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            overflow_id,
            (
                content > visible + 0.5,
                state.offset.x <= 0.5,
                state.offset.x >= max_offset - 0.5,
            ),
        );
    });
}

/// One 20×30 overflow paddle. `direction` is −1 for the leading (scroll
/// left) chevron and +1 for the trailing one; the hairline sits on the side
/// facing the tab list. Returns true when an enabled paddle was clicked.
fn viewer_tab_overflow_chevron(ui: &mut Ui, direction: f32, enabled: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(20.0, ResultBarMetrics::of(ui).viewer_tab),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Button,
            enabled,
            if direction < 0.0 {
                "Scroll viewer tabs backward"
            } else {
                "Scroll viewer tabs forward"
            },
        )
    });
    if ui.is_rect_visible(rect) {
        let hovered = enabled && response.hovered();
        if hovered {
            ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
        }
        let mut color = if hovered {
            t.color.text
        } else {
            t.color.text_dim
        };
        if !enabled {
            color = color.gamma_multiply(0.3);
        }
        let center = rect.center();
        let arm_x = center.x - 2.0 * direction;
        let apex_x = center.x + 2.0 * direction;
        ui.painter().add(egui::Shape::line(
            vec![
                egui::pos2(arm_x, center.y - 3.5),
                egui::pos2(apex_x, center.y),
                egui::pos2(arm_x, center.y + 3.5),
            ],
            egui::Stroke::new(1.2, color),
        ));
        let border_x = if direction < 0.0 {
            rect.right() - 0.5
        } else {
            rect.left() + 0.5
        };
        ui.painter().vline(
            border_x,
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
        theme::paint_focus_ring(ui, &response, rect);
    }
    enabled && response.clicked()
}

pub fn viewer_picker(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    accessible_label: &str,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.text_dim,
    );
    let width = galley.size().x + 36.0;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width, ResultBarMetrics::of(ui).viewer_tab),
        egui::Sense::click(),
    );
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), accessible_label));
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        if hovered {
            ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
        }
        let color = if hovered {
            t.color.text
        } else {
            t.color.text_dim
        };
        icon.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(rect.left() + 14.0, rect.center().y),
                egui::vec2(13.0, 13.0),
            ),
            color,
        );
        ui.painter().galley(
            egui::pos2(rect.left() + 26.0, rect.center().y - galley.size().y * 0.5),
            galley,
            color,
        );
        theme::paint_focus_ring(ui, &response, rect);
    }
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

pub fn viewer_picker_separator(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(1.0, ResultBarMetrics::of(ui).viewer_tab),
        egui::Sense::hover(),
    );
    ui.painter().vline(
        rect.center().x,
        rect.y_range(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
}

pub fn instrument_control<'a>(
    ui: &mut Ui,
    label: &'a str,
    active: bool,
    tooltip: &'a str,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        if active {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    let width = (galley.size().x + 12.0).max(26.0);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width, ResultBarMetrics::of(ui).instrument_control),
        egui::Sense::click(),
    );
    response
        .widget_info(|| WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), active, tooltip));
    if ui.is_rect_visible(rect) {
        let hover = response.hovered();
        if active || hover {
            ui.painter().rect_filled(
                rect,
                2.0,
                if active {
                    t.color.accent_dim
                } else {
                    t.color.bg_hover
                },
            );
        }
        if active {
            ui.painter().hline(
                egui::Rangef::new(rect.left() + 2.0, rect.right() - 2.0),
                rect.bottom() - 0.5,
                egui::Stroke::new(1.0, t.color.accent),
            );
        }
        ui.painter().galley(
            egui::pos2(
                rect.center().x - galley.size().x * 0.5,
                rect.center().y - galley.size().y * 0.5,
            ),
            galley,
            if active {
                t.color.accent
            } else if hover {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        theme::paint_focus_ring(ui, &response, rect);
    }
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(tooltip)
}

pub fn instrument_separator(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(7.0, 17.0), egui::Sense::hover());
    ui.painter().vline(
        rect.center().x,
        rect.y_range(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
}

/// One viewer tab, per the mockup: full-strip hit target, compact horizontal
/// padding, hover fill, and a 2 px bottom rule when active.
///
/// Every tab drawn is a tab that can be opened. The strip lists only the
/// sheets the active dataset can feed (see [`bars::viewer_tabs`]), so there
/// is no disabled state to paint here — the Visualization Studio catalog is
/// where the full set of viewers and their requirements are published.
pub fn viewer_tab(ui: &mut Ui, viewer: ResultViewer, active: bool) -> bool {
    use rspice_ui_kit::theme::mix;

    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let mut job = egui::text::LayoutJob::default();
    job.append(
        viewer.tab_label(),
        0.0,
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_1, FontWeight::Regular),
            color: egui::Color32::PLACEHOLDER,
            ..Default::default()
        },
    );
    let galley = ui.fonts_mut(|f| f.layout_job(job));

    let height = ResultBarMetrics::of(ui)
        .viewer_tab
        .min(ui.available_height());
    let horizontal_padding = 20.0;
    let icon_width = 13.0;
    let icon_gap = 6.0;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            galley.size().x + horizontal_padding + icon_width + icon_gap,
            height,
        ),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::SelectableLabel,
            ui.is_enabled(),
            viewer.tab_label(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        if active {
            node.set_selected(true);
        } else {
            node.set_selected(false);
        }
    });
    if !ui.is_rect_visible(rect) {
        return false;
    }

    let hover = ui.ctx().animate_bool_with_time(
        response.id,
        !active && response.hovered(),
        ui.style().animation_time,
    );
    let (fill, text_color) = if active {
        (egui::Color32::TRANSPARENT, c.text)
    } else {
        (
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
            mix(c.text_dim, c.text, hover),
        )
    };

    let painter = ui.painter();
    if fill != egui::Color32::TRANSPARENT {
        painter.rect_filled(rect, 0.0, fill);
    }
    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(
            rect.left() + horizontal_padding * 0.5 + icon_width * 0.5,
            rect.center().y,
        ),
        egui::vec2(icon_width, icon_width),
    );
    viewer_tab_icon(viewer).paint(painter, icon_rect, text_color);
    painter.galley(
        egui::pos2(
            icon_rect.right() + icon_gap,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        text_color,
    );
    if active {
        painter.rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(rect.left() + 8.0, rect.bottom() - 2.0),
                egui::pos2(rect.right() - 8.0, rect.bottom()),
            ),
            0.0,
            c.accent,
        );
    }

    theme::paint_focus_ring(ui, &response, rect);

    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
        && !active
}

#[cfg(test)]
mod tests;

/// Export targets requested by the Results menu.
#[derive(Default)]
pub struct ExportRequests {
    pub csv: bool,
    pub figure: bool,
}

pub fn export_menu(ui: &mut Ui) -> ExportRequests {
    let mut requests = ExportRequests::default();
    ui.menu_button("Export…", |ui| {
        // Not "waveform data": the export routes on the active sheet and then
        // on the retained payload, so it writes a spectrum here and SOA rules,
        // SOA observations, optimizer candidates or an event history there —
        // none of them samples.
        if ui.button("Result data (CSV)…").clicked() {
            requests.csv = true;
            ui.close();
        }
        // The figure goes through the publication pipeline, which renders this
        // sheet as PDF/A, PDF, SVG or PNG at a chosen resolution. A window
        // screenshot used to live here instead: the same pixels the reader
        // already had, at whatever the display happened to be, with the chrome
        // cropped off by rectangle. One label for both targets, because the
        // browser reaches the same pipeline through its own worker.
        if ui.button("Viewer figure (PDF, SVG, PNG)…").clicked() {
            requests.figure = true;
            ui.close();
        }
    });
    requests
}
