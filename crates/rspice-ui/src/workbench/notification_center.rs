//! Session notification and activity center specified by the workbench
//! mockup. The source of truth is the real toast stream; this surface never
//! manufactures fixture activity.

use egui::{Align, Frame, Margin, Sense, Stroke, Ui, Vec2, pos2, vec2};

use crate::common::{AppState, RSpiceApp, app::ConsoleMessage};
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogSize, NotificationCategory, NotificationRecord, ToastKind,
};

use super::design_system::{WorkbenchIcon, elide_text};
use super::state::NotificationFilter;
use super::{RouteTransitionSource, SurfaceId, SurfaceRoute};

const TOOLBAR_MIN_HEIGHT: f32 = 41.0;
const TOOLBAR_PADDING_X: i8 = 10;
const TOOLBAR_PADDING_Y: i8 = 6;
const TOOLBAR_GAP: f32 = 6.0;
const FILTER_CELL_WIDTH: f32 = 54.0;
const LIST_MIN_HEIGHT: f32 = 240.0;
const LIST_MAX_HEIGHT: f32 = 560.0;
const ROW_COLUMN_WIDTH: f32 = 34.0;
const ROW_ICON_SIDE: f32 = 30.0;
const ROW_GAP: f32 = 10.0;
const ROW_HEADING_GAP: f32 = 8.0;
const ROW_PADDING_X: i8 = 12;
const ROW_PADDING_Y: i8 = 10;
const FOOTER_MIN_HEIGHT: f32 = 38.0;
const LARGE_TARGET_MAX_WIDTH: f32 = 820.0;

pub(super) fn show(ctx: &egui::Context, app: &mut RSpiceApp) {
    let route_owned =
        app.state.workbench.current_route().surface_id() == SurfaceId::NotificationCenter;
    if !app.state.workbench.notification_center_open && !route_owned {
        return;
    }

    let records = app.state.ui.toasts.activity().to_vec();
    let now = ctx.input(|input| input.time);

    let choice = Dialog::new(
        "BACKGROUND WORK · APPROVALS · SYSTEM EVENTS",
        "Notifications and activity",
        "Close",
    )
    .description("Review and filter retained background work, approval, and system activity.")
    .size(DialogSize::Manager)
    .flush_body()
    .show(ctx, |ui| {
        toolbar(ui, app);
        activity_list(ui, &records, app.state.workbench.notification_filter, now);
        activity_footer(ui, app);
    });

    match choice {
        DialogChoice::Primary | DialogChoice::Cancelled => {
            app.state.workbench.notification_center_open = false;
            if route_owned {
                close_to_source(&mut app.state);
            }
        }
        DialogChoice::None | DialogChoice::Secondary | DialogChoice::Ghost => {}
    }
}

fn close_to_source(state: &mut AppState) {
    if state
        .workbench
        .navigate_back(RouteTransitionSource::User)
        .is_some()
    {
        return;
    }
    let fallback = SurfaceRoute::surface(SurfaceId::from_workspace(state.workbench.workspace));
    if let Err(error) = state
        .workbench
        .replace_route(fallback, RouteTransitionSource::User)
    {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Could not close Notifications and activity: {error}"
        )));
    }
}

fn toolbar(ui: &mut Ui, app: &mut RSpiceApp) {
    let tokens = Tokens::get(ui.ctx());
    let large_targets = use_large_targets(
        ui.available_width(),
        ui.ctx().input(|input| input.has_touch_screen()),
    );
    let control_height = if large_targets {
        44.0
    } else {
        tokens.metrics.ctl_h
    };
    let toolbar = Frame::NONE
        .fill(tokens.color.bg_panel)
        .inner_margin(Margin::symmetric(TOOLBAR_PADDING_X, TOOLBAR_PADDING_Y))
        .show(ui, |ui| {
            ui.set_min_height(
                (TOOLBAR_MIN_HEIGHT - f32::from(TOOLBAR_PADDING_Y) * 2.0).max(control_height),
            );
            ui.spacing_mut().item_spacing.x = TOOLBAR_GAP;
            if ui.available_width() < 380.0 {
                ui.horizontal_wrapped(|ui| {
                    filter_buttons(ui, app, control_height);
                    mark_all_read_button(ui, app, control_height);
                });
            } else {
                ui.horizontal(|ui| {
                    filter_buttons(ui, app, control_height);
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        mark_all_read_button(ui, app, control_height);
                    });
                });
            }
        });
    ui.painter().line_segment(
        [
            toolbar.response.rect.left_bottom(),
            toolbar.response.rect.right_bottom(),
        ],
        Stroke::new(1.0, tokens.color.border_strong),
    );
}

fn filter_buttons(ui: &mut Ui, app: &mut RSpiceApp, control_height: f32) {
    let tokens = Tokens::get(ui.ctx());
    let width = FILTER_CELL_WIDTH * NotificationFilter::ALL.len() as f32;
    let (rect, group_response) =
        ui.allocate_exact_size(vec2(width, control_height), Sense::hover());
    ui.painter().rect(
        rect,
        tokens.radius,
        tokens.color.bg_inset,
        Stroke::new(1.0, tokens.color.border),
        egui::StrokeKind::Inside,
    );
    group_response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::RadioGroup, true, "Activity filter")
    });
    ui.ctx().accesskit_node_builder(group_response.id, |node| {
        node.set_role(egui::accesskit::Role::RadioGroup);
        node.set_label("Activity filter");
    });

    let mut focused = None;
    for (index, filter) in NotificationFilter::ALL.into_iter().enumerate() {
        let cell = egui::Rect::from_min_max(
            pos2(rect.left() + FILTER_CELL_WIDTH * index as f32, rect.top()),
            pos2(
                rect.left() + FILTER_CELL_WIDTH * (index + 1) as f32,
                rect.bottom(),
            ),
        );
        let id = ui.id().with(("notification-filter", filter.label()));
        let response = ui.interact(cell, id, Sense::click());
        let selected = app.state.workbench.notification_filter == filter;
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::RadioButton,
                ui.is_enabled(),
                selected,
                filter.label(),
            )
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::RadioButton);
            node.set_selected(selected);
        });
        if response.clicked() {
            app.state.workbench.notification_filter = filter;
            response.request_focus();
        }
        if response.has_focus() {
            focused = Some(index);
        }

        let fill = if selected {
            tokens.color.bg_active
        } else if response.hovered() {
            tokens.color.bg_hover
        } else {
            egui::Color32::TRANSPARENT
        };
        if fill != egui::Color32::TRANSPARENT {
            ui.painter().rect_filled(cell.shrink(1.0), 0.0, fill);
        }
        if selected {
            ui.painter().rect_filled(
                egui::Rect::from_min_max(
                    pos2(cell.left() + 1.0, cell.bottom() - 2.0),
                    pos2(cell.right() - 1.0, cell.bottom()),
                ),
                0.0,
                tokens.color.accent,
            );
        }
        if index + 1 < NotificationFilter::ALL.len() {
            ui.painter().line_segment(
                [
                    pos2(cell.right(), cell.top() + 1.0),
                    pos2(cell.right(), cell.bottom() - 1.0),
                ],
                Stroke::new(1.0, tokens.color.border),
            );
        }
        ui.painter().text(
            cell.center(),
            egui::Align2::CENTER_CENTER,
            filter.label(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if selected || response.hovered() || response.has_focus() {
                tokens.color.text
            } else {
                tokens.color.text_dim
            },
        );
        theme::paint_focus_ring(ui, &response, cell.shrink(1.0));
    }

    if let Some(index) = focused {
        let direction = ui.input(|input| {
            if input.key_pressed(egui::Key::ArrowRight) || input.key_pressed(egui::Key::ArrowDown) {
                1_i32
            } else if input.key_pressed(egui::Key::ArrowLeft)
                || input.key_pressed(egui::Key::ArrowUp)
            {
                -1_i32
            } else {
                0_i32
            }
        });
        if direction != 0 {
            let count = NotificationFilter::ALL.len() as i32;
            let next = (index as i32 + direction).rem_euclid(count) as usize;
            let filter = NotificationFilter::ALL[next];
            app.state.workbench.notification_filter = filter;
            ui.memory_mut(|memory| {
                memory.request_focus(ui.id().with(("notification-filter", filter.label())));
            });
        }
    }
}

fn mark_all_read_button(ui: &mut Ui, app: &mut RSpiceApp, control_height: f32) {
    let has_unread = app.state.ui.toasts.unread_count() > 0;
    if Button::new("Mark all read")
        .enabled(has_unread)
        .min_height(control_height)
        .show(ui)
        .clicked()
    {
        app.state.ui.toasts.mark_all_read();
    }
}

fn activity_list(
    ui: &mut Ui,
    records: &[NotificationRecord],
    filter: NotificationFilter,
    now: f64,
) {
    let visible: Vec<_> = records
        .iter()
        .filter(|record| filter_includes(filter, record.category()))
        .collect();

    if visible.is_empty() {
        empty_state(ui, records.is_empty());
        return;
    }

    let list_height = (ui.available_height() - FOOTER_MIN_HEIGHT).clamp(
        LIST_MIN_HEIGHT.min(ui.available_height().max(1.0)),
        LIST_MAX_HEIGHT,
    );
    egui::ScrollArea::vertical()
        .id_salt("notification-center-list")
        .max_height(list_height)
        .min_scrolled_height(list_height)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let list = ui.scope(|ui| {
                for record in visible {
                    activity_row(ui, record, now);
                }
            });
            ui.ctx().accesskit_node_builder(list.response.id, |node| {
                node.set_role(egui::accesskit::Role::List);
                node.set_label("Retained notification activity");
            });
        });
}

const fn filter_includes(filter: NotificationFilter, category: NotificationCategory) -> bool {
    matches!(filter, NotificationFilter::All)
        || matches!(
            (filter, category),
            (NotificationFilter::Jobs, NotificationCategory::Job)
                | (
                    NotificationFilter::Approvals,
                    NotificationCategory::Approval
                )
                | (NotificationFilter::System, NotificationCategory::System)
        )
}

fn activity_row(ui: &mut Ui, record: &NotificationRecord, now: f64) {
    let t = Tokens::get(ui.ctx());
    let color = kind_color(&t, record.kind());
    let frame = Frame::NONE
        .fill(if record.is_read() {
            t.color.bg_elevated
        } else {
            mix(t.color.bg_app, t.color.accent, 0.05)
        })
        .inner_margin(Margin::symmetric(ROW_PADDING_X, ROW_PADDING_Y))
        .show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = ROW_GAP;
                let (tone_column, _) =
                    ui.allocate_exact_size(vec2(ROW_COLUMN_WIDTH, ROW_ICON_SIDE), Sense::hover());
                let tone_rect =
                    egui::Rect::from_min_size(tone_column.left_top(), Vec2::splat(ROW_ICON_SIDE));
                ui.painter().rect(
                    tone_rect,
                    0.0,
                    t.color.bg_panel_2,
                    Stroke::new(1.0, t.color.border_strong),
                    egui::StrokeKind::Inside,
                );
                notification_icon(record.category()).paint(
                    ui.painter(),
                    egui::Rect::from_center_size(tone_rect.center(), Vec2::splat(16.0)),
                    color,
                );

                ui.vertical(|ui| {
                    ui.set_min_width(ui.available_width());
                    ui.spacing_mut().item_spacing.y = 0.0;
                    activity_heading(ui, record, now, &t);
                    ui.add_space(3.0);
                    ui.label(
                        egui::RichText::new(record.message())
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                    ui.add_space(7.0);
                });
            });
        });
    ui.painter().line_segment(
        [
            frame.response.rect.left_bottom(),
            frame.response.rect.right_bottom(),
        ],
        Stroke::new(1.0, t.color.border),
    );
    let response = ui.interact(
        frame.response.rect,
        ui.id().with(("notification", record.id())),
        Sense::hover(),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::ListItem);
        node.set_label(format!(
            "{} {}: {}",
            record.category().label(),
            record.kind().label(),
            record.title(),
        ));
        node.set_description(record.message());
    });
}

fn activity_heading(ui: &mut Ui, record: &NotificationRecord, now: f64, t: &Tokens) {
    let title_font = theme::sans(tokens::FS_1, FontWeight::SemiBold);
    let age_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let age = age_label(now, record.created());
    let title_width = ui
        .painter()
        .layout_no_wrap(record.title().to_owned(), title_font.clone(), t.color.text)
        .size()
        .x;
    let age_color = if record.is_read() {
        t.color.text_dim
    } else {
        t.color.text_faint
    };
    let age_width = ui
        .painter()
        .layout_no_wrap(age.clone(), age_font.clone(), age_color)
        .size()
        .x;
    let heading_height = ui
        .painter()
        .layout_no_wrap("Ag".to_owned(), title_font.clone(), t.color.text)
        .size()
        .y
        .max(
            ui.painter()
                .layout_no_wrap("Ag".to_owned(), age_font.clone(), age_color)
                .size()
                .y,
        );
    let available_width = ui.available_width().max(0.0);
    let (rect, response) =
        ui.allocate_exact_size(vec2(available_width, heading_height), Sense::hover());
    let (title_region_width, age_region_width) =
        heading_column_widths(available_width, title_width, age_width);

    if title_region_width > 0.0 {
        let title_rect = egui::Rect::from_min_max(
            rect.left_top(),
            pos2(rect.left() + title_region_width, rect.bottom()),
        );
        let title = elide_text(ui, record.title(), &title_font, title_rect.width());
        ui.painter().with_clip_rect(title_rect).text(
            title_rect.left_center(),
            egui::Align2::LEFT_CENTER,
            title,
            title_font,
            t.color.text,
        );
    }
    if age_region_width > 0.0 {
        let age_rect = egui::Rect::from_min_max(
            pos2(rect.right() - age_region_width, rect.top()),
            rect.right_bottom(),
        );
        let age = elide_text(ui, &age, &age_font, age_rect.width());
        ui.painter().with_clip_rect(age_rect).text(
            age_rect.right_center(),
            egui::Align2::RIGHT_CENTER,
            age,
            age_font,
            age_color,
        );
    }

    let accessible_heading = format!("{} \u{00b7} {age}", record.title());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            &accessible_heading,
        )
    });
    response.on_hover_text(accessible_heading);
}

/// Resolve the title and timestamp into disjoint measured regions. The title
/// remains the primary content under pressure; the timestamp uses at most 40%
/// of the available columns and is elided when the row is exceptionally narrow.
fn heading_column_widths(
    available_width: f32,
    desired_title_width: f32,
    desired_age_width: f32,
) -> (f32, f32) {
    let available_width = available_width.max(0.0);
    if desired_age_width <= 0.0 || available_width <= ROW_HEADING_GAP {
        return (available_width, 0.0);
    }

    let columns_width = (available_width - ROW_HEADING_GAP).max(0.0);
    if desired_title_width.max(0.0) + desired_age_width.max(0.0) <= columns_width {
        return (
            (columns_width - desired_age_width.max(0.0)).max(0.0),
            desired_age_width.max(0.0),
        );
    }

    let age_region_width = desired_age_width.max(0.0).min(columns_width * 0.4);
    (
        (columns_width - age_region_width).max(0.0),
        age_region_width,
    )
}

fn notification_icon(category: NotificationCategory) -> WorkbenchIcon {
    match category {
        NotificationCategory::Job => WorkbenchIcon::Run,
        NotificationCategory::Approval => WorkbenchIcon::Check,
        NotificationCategory::System => WorkbenchIcon::Info,
    }
}

fn activity_footer(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let has_read = app
        .state
        .ui
        .toasts
        .activity()
        .iter()
        .any(NotificationRecord::is_read);
    let large_targets = use_large_targets(
        ui.available_width(),
        ui.ctx().input(|input| input.has_touch_screen()),
    );
    let control_height = if large_targets { 44.0 } else { t.metrics.ctl_h };
    let footer = Frame::NONE
        .fill(t.color.bg_elevated)
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.set_min_height((FOOTER_MIN_HEIGHT - 12.0).max(control_height));
            if ui.available_width() < 520.0 {
                ui.horizontal_wrapped(|ui| {
                    footer_copy(ui, &t);
                    clear_read_button(ui, app, has_read, control_height);
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Retention · this session")
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        clear_read_button(ui, app, has_read, control_height);
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(
                                    "Simulation, file, validation, and export activity is retained for this application session.",
                                )
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                            )
                            .wrap(),
                        );
                    });
                });
            }
        });
    ui.painter().line_segment(
        [
            footer.response.rect.left_top(),
            footer.response.rect.right_top(),
        ],
        Stroke::new(1.0, t.color.border_strong),
    );
}

fn footer_copy(ui: &mut Ui, t: &Tokens) {
    ui.label(
        egui::RichText::new("Retention · this session")
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.label(
        egui::RichText::new(
            "Simulation, file, validation, and export activity is retained for this application session.",
        )
        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
        .color(t.color.text_dim),
    );
}

fn clear_read_button(ui: &mut Ui, app: &mut RSpiceApp, enabled: bool, control_height: f32) {
    if Button::new("Clear read")
        .ghost()
        .enabled(enabled)
        .min_height(control_height)
        .show(ui)
        .clicked()
    {
        app.state.ui.toasts.clear_read();
    }
}

fn empty_state(ui: &mut Ui, no_activity: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::same(18))
        .show(ui, |ui| {
            // Frames shrink to their contents unless the child explicitly owns
            // the available track. Keep empty/filter-empty states aligned with
            // the full-width activity rows at every dialog width.
            ui.set_min_width(ui.available_width());
            ui.set_min_height(LIST_MIN_HEIGHT - 36.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(if no_activity {
                        "No activity in this session"
                    } else {
                        "No activity matches this filter"
                    })
                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold)),
                );
                ui.label(
                    egui::RichText::new(if no_activity {
                        "Simulation, file, validation, and export notices will appear here."
                    } else {
                        "Choose another category to review retained session activity."
                    })
                    .color(t.color.text_dim),
                );
            });
        })
        .response
}

fn use_large_targets(local_width: f32, has_touch_screen: bool) -> bool {
    local_width <= LARGE_TARGET_MAX_WIDTH || has_touch_screen
}

fn kind_color(tokens: &Tokens, kind: ToastKind) -> egui::Color32 {
    match kind {
        ToastKind::Success => tokens.color.ok,
        ToastKind::Info => tokens.color.accent,
        ToastKind::Warn => tokens.color.warn,
        ToastKind::Error => tokens.color.err,
    }
}

fn age_label(now: f64, created: f64) -> String {
    let age = (now - created).max(0.0);
    if age < 60.0 {
        "now".to_owned()
    } else if age < 3_600.0 {
        format!("{} min ago", (age / 60.0).floor() as u64)
    } else {
        format!("{} hr ago", (age / 3_600.0).floor() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_notification_deep_link_is_modal_and_closes_to_its_source() {
        let mut state = AppState::default();
        state
            .workbench
            .navigate(
                SurfaceRoute::surface(SurfaceId::NotificationCenter),
                RouteTransitionSource::BrowserPop,
            )
            .expect("notification route is executable");

        assert!(state.workbench.application_modal_open());
        assert_eq!(
            state.workbench.current_route().surface_id(),
            SurfaceId::NotificationCenter
        );

        close_to_source(&mut state);
        assert_ne!(
            state.workbench.current_route().surface_id(),
            SurfaceId::NotificationCenter
        );
    }

    #[test]
    fn filters_project_mockup_domains_without_mutating_activity() {
        assert!(filter_includes(
            NotificationFilter::All,
            NotificationCategory::Approval
        ));
        assert!(filter_includes(
            NotificationFilter::Jobs,
            NotificationCategory::Job
        ));
        assert!(!filter_includes(
            NotificationFilter::Jobs,
            NotificationCategory::System
        ));
    }

    #[test]
    fn age_labels_are_session_relative_and_bounded() {
        assert_eq!(age_label(2.0, 4.0), "now");
        assert_eq!(age_label(180.0, 0.0), "3 min ago");
        assert_eq!(age_label(7_200.0, 0.0), "2 hr ago");
    }

    #[test]
    fn notification_center_geometry_matches_the_mockup_contract() {
        assert_eq!(TOOLBAR_MIN_HEIGHT, 41.0);
        assert_eq!(FILTER_CELL_WIDTH, 54.0);
        assert_eq!(LIST_MIN_HEIGHT, 240.0);
        assert_eq!(LIST_MAX_HEIGHT, 560.0);
        assert_eq!(ROW_COLUMN_WIDTH, 34.0);
        assert_eq!(ROW_ICON_SIDE, 30.0);
        assert_eq!(ROW_GAP, 10.0);
        assert_eq!(ROW_HEADING_GAP, 8.0);
        assert_eq!(FOOTER_MIN_HEIGHT, 38.0);
    }

    #[test]
    fn long_notification_titles_never_overlap_the_timestamp_region() {
        let desired_title_width = 8_000.0;
        let desired_age_width = 86.0;

        for available_width in [0.0, 4.0, 8.0, 24.0, 96.0, 240.0, 560.0] {
            let (title_width, age_width) =
                heading_column_widths(available_width, desired_title_width, desired_age_width);
            assert!(title_width >= 0.0);
            assert!(age_width >= 0.0);
            assert!(age_width <= desired_age_width);
            let gap = if age_width > 0.0 {
                ROW_HEADING_GAP
            } else {
                0.0
            };
            assert!(
                title_width + gap + age_width <= available_width.max(0.0) + f32::EPSILON,
                "columns exceed {available_width}px: title={title_width}, age={age_width}"
            );
        }
    }

    #[test]
    fn measured_heading_preserves_a_short_timestamp_when_space_allows() {
        let (title_width, age_width) = heading_column_widths(320.0, 900.0, 62.0);
        assert_eq!(age_width, 62.0);
        assert_eq!(title_width + ROW_HEADING_GAP + age_width, 320.0);

        let (title_width, age_width) = heading_column_widths(6.0, 900.0, 62.0);
        assert_eq!((title_width, age_width), (6.0, 0.0));
    }

    #[test]
    fn responsive_target_size_uses_the_local_container_width() {
        assert!(use_large_targets(480.0, false));
        assert!(!use_large_targets(1_120.0, false));
        assert!(use_large_targets(1_120.0, true));
    }

    #[test]
    fn empty_state_claims_the_full_available_track() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut geometry = None;
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(390.0, 640.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(Frame::NONE)
                    .show(ctx, |ui| {
                        let available_width = ui.available_width();
                        let response = empty_state(ui, true);
                        geometry = Some((available_width, response.rect.width()));
                    });
            },
        );

        let (available_width, rendered_width) = geometry.expect("empty-state geometry");
        assert!(
            (rendered_width - available_width).abs() <= 0.5,
            "empty state rendered at {rendered_width}px in a {available_width}px track"
        );
    }
}
