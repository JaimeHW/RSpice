//! Workspace tab strip — Library / Schematic / Netlist / Simulate / Results.
//!
//! The active tab treatment follows the design direction: Instrument marks
//! the top edge with the accent, Meridian underlines, Graphite inverts.

use egui::{Context, Frame, Rect, Sense, TopBottomPanel, Ui, vec2};

use crate::common::AppState;
use crate::shell::WorkspaceView;
use crate::ui::Direction;
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};

/// Workspace tab strip height.
pub const WTABS_HEIGHT: f32 = 35.0;
const ICON_ONLY_TAB_WIDTH: f32 = 48.0;
const WTABS_LEADING_SPACE: f32 = 8.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkspaceTabsPresentation {
    FullLabels,
    IconOnly,
}

fn workspace_tabs_presentation_for_width(
    width: f32,
    schematic_label: &str,
) -> WorkspaceTabsPresentation {
    if width < workspace_tabs_required_width(WorkspaceTabsPresentation::FullLabels, schematic_label)
    {
        WorkspaceTabsPresentation::IconOnly
    } else {
        WorkspaceTabsPresentation::FullLabels
    }
}

fn workspace_tabs_required_width(
    presentation: WorkspaceTabsPresentation,
    schematic_label: &str,
) -> f32 {
    match presentation {
        WorkspaceTabsPresentation::IconOnly => {
            WTABS_LEADING_SPACE + WorkspaceView::ALL.len() as f32 * ICON_ONLY_TAB_WIDTH
        }
        WorkspaceTabsPresentation::FullLabels => {
            WTABS_LEADING_SPACE
                + tab_estimated_width("Library", false, None)
                + tab_estimated_width(schematic_label, true, None)
                + tab_estimated_width("Netlist", false, None)
                + tab_estimated_width("Simulate", false, None)
                + tab_estimated_width("Results", false, Some(1))
        }
    }
}

fn tab_estimated_width(label: &str, dirty: bool, badge: Option<u32>) -> f32 {
    let mut width = 14.0 + 13.0 + 7.0 + label.chars().count() as f32 * 7.5 + 14.0;
    if dirty {
        width += 6.0 + 7.0;
    }
    if badge.is_some() {
        width += 16.0 + 7.0;
    }
    width
}

fn view_icon(view: WorkspaceView) -> Icon {
    match view {
        WorkspaceView::Library => Icon::Library,
        WorkspaceView::Schematic => Icon::Schematic,
        WorkspaceView::Netlist => Icon::File,
        WorkspaceView::Simulate => Icon::Simulate,
        WorkspaceView::Results => Icon::Results,
    }
}

/// Render the workspace tab strip.
pub fn show(ctx: &Context, state: &mut AppState) {
    let t = Tokens::get(ctx);
    let c = t.color;

    TopBottomPanel::top("volta.wtabs")
        .exact_height(WTABS_HEIGHT)
        .frame(Frame::NONE.fill(c.bg_app))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let panel_rect = ui.max_rect();
            ui.painter().hline(
                panel_rect.x_range(),
                panel_rect.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );

            ui.horizontal_centered(|ui| {
                ui.add_space(8.0);
                ui.spacing_mut().item_spacing.x = 0.0;

                let active_document_type = state.workspace.active_view_type();
                let schematic_label = if active_document_type == crate::state::ViewType::Symbol {
                    format!("Symbol · {}", state.workspace.active_view.cell)
                } else {
                    format!("Schematic · {}", state.workspace.active_view.cell)
                };
                let presentation =
                    workspace_tabs_presentation_for_width(panel_rect.width(), &schematic_label);
                let show_labels = presentation == WorkspaceTabsPresentation::FullLabels;
                let new_results = state.simulation.data_version > state.shell.results_seen_version
                    && state.shell.view != WorkspaceView::Results;

                for view in WorkspaceView::ALL {
                    let label = match view {
                        WorkspaceView::Schematic => schematic_label.as_str(),
                        other => other.label(),
                    };
                    let dirty = view == WorkspaceView::Schematic
                        && (state.schematic.is_dirty
                            || state.workspace.open_views.iter().any(|open| open.dirty));
                    let badge = (view == WorkspaceView::Results && new_results).then_some(1u32);
                    let icon = if view == WorkspaceView::Schematic
                        && active_document_type == crate::state::ViewType::Symbol
                    {
                        Icon::Pin
                    } else {
                        view_icon(view)
                    };

                    let display_label = if show_labels { label } else { "" };
                    let response = tab(
                        ui,
                        display_label,
                        icon,
                        state.shell.view == view,
                        dirty,
                        badge,
                    );
                    let response = if show_labels {
                        response
                    } else {
                        response.on_hover_text(label)
                    };
                    if response.clicked() {
                        state.shell.view = view;
                        if view == WorkspaceView::Results {
                            state.shell.results_seen_version = state.simulation.data_version;
                        }
                    }
                }
            });
        });
}

/// One workspace tab. Returns the click response.
fn tab(
    ui: &mut Ui,
    label: &str,
    icon: Icon,
    active: bool,
    dirty: bool,
    badge: Option<u32>,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let galley = ui.fonts_mut(|f| {
        f.layout_no_wrap(
            label.to_owned(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            c.text,
        )
    });
    let badge_galley = badge.map(|count| {
        ui.fonts_mut(|f| {
            f.layout_no_wrap(
                count.to_string(),
                theme::mono(10.0, FontWeight::Medium),
                c.accent_ink,
            )
        })
    });

    let mut width = 14.0 + 13.0 + 7.0 + galley.size().x + 14.0;
    if dirty {
        width += 6.0 + 7.0;
    }
    if let Some(badge_galley) = &badge_galley {
        width += badge_galley.size().x.max(8.0) + 8.0 + 7.0;
    }

    let (rect, response) = ui.allocate_exact_size(vec2(width, WTABS_HEIGHT), Sense::click());
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered() && !active, 0.16);

    let graphite = t.direction == Direction::Graphite;
    let painter = ui.painter();

    // Tab background.
    let (fill, text_color, icon_color) = if active {
        if graphite {
            (c.text, c.bg_panel, c.bg_panel)
        } else {
            (c.bg_panel, c.text, c.text)
        }
    } else {
        (
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
            mix(c.text_dim, c.text, hover),
            mix(c.text_dim, c.text, hover),
        )
    };
    if fill != egui::Color32::TRANSPARENT {
        painter.rect_filled(rect, 0.0, fill);
    }

    // Direction-specific active edge marker.
    if active && !graphite {
        let edge = match t.direction {
            Direction::Instrument => {
                Rect::from_min_max(rect.left_top(), egui::pos2(rect.right(), rect.top() + 2.0))
            }
            _ => Rect::from_min_max(
                egui::pos2(rect.left(), rect.bottom() - 2.0),
                rect.right_bottom(),
            ),
        };
        painter.rect_filled(edge, 0.0, c.accent);
    }

    // Content: icon, label, dirty dot, badge.
    let mut x = rect.left() + 14.0;
    let cy = rect.center().y;
    let icon_rect = Rect::from_center_size(egui::pos2(x + 6.5, cy), vec2(13.0, 13.0));
    icon.paint(painter, icon_rect, icon_color);
    x += 13.0 + 7.0;
    painter.galley(
        egui::pos2(x, cy - galley.size().y * 0.5),
        galley.clone(),
        text_color,
    );
    x += galley.size().x;

    if dirty {
        painter.circle_filled(egui::pos2(x + 7.0 + 3.0, cy), 3.0, c.warn);
        x += 7.0 + 6.0;
    }
    if let Some(badge_galley) = badge_galley {
        let badge_w = (badge_galley.size().x + 8.0).max(16.0);
        let badge_rect =
            Rect::from_center_size(egui::pos2(x + 7.0 + badge_w * 0.5, cy), vec2(badge_w, 16.0));
        painter.rect_filled(badge_rect, 8.0, c.accent);
        painter.galley(
            egui::pos2(
                badge_rect.center().x - badge_galley.size().x * 0.5,
                badge_rect.center().y - badge_galley.size().y * 0.5,
            ),
            badge_galley,
            c.accent_ink,
        );
    }

    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_width_workspace_tabs_use_icon_presentation_that_fits() {
        let presentation =
            workspace_tabs_presentation_for_width(390.0, "Symbol - folded-current-mirror");

        assert_eq!(presentation, WorkspaceTabsPresentation::IconOnly);
        assert!(
            workspace_tabs_required_width(presentation, "Symbol - folded-current-mirror") <= 390.0,
            "icon-only workspace tabs should fit a phone viewport"
        );
    }

    #[test]
    fn desktop_workspace_tabs_keep_document_labels() {
        let presentation = workspace_tabs_presentation_for_width(900.0, "Schematic - top");

        assert_eq!(presentation, WorkspaceTabsPresentation::FullLabels);
    }

    #[test]
    fn workspace_tabs_stay_icon_only_until_long_label_estimate_fits() {
        let label = "Symbol - precision_frontend_with_long_corner_sweep";
        let full_required =
            workspace_tabs_required_width(WorkspaceTabsPresentation::FullLabels, label);

        assert_eq!(
            workspace_tabs_presentation_for_width(full_required - 1.0, label),
            WorkspaceTabsPresentation::IconOnly
        );
        assert_eq!(
            workspace_tabs_presentation_for_width(full_required, label),
            WorkspaceTabsPresentation::FullLabels
        );
    }
}
