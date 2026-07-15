//! Session notification and activity center specified by the workbench
//! mockup. The source of truth is the real toast stream; this surface never
//! manufactures fixture activity.

use egui::{Frame, Margin, Sense, Stroke, Ui, UiKind, Vec2};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogSize, NotificationCategory, NotificationRecord, ToastKind,
};

use super::commands::Command;
use super::state::NotificationFilter;

pub(super) fn show(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.workbench.notification_center_open {
        return;
    }

    let unread = app.state.ui.toasts.unread_count();
    let hint = match unread {
        0 => "Retention · this session · no unread activity".to_owned(),
        1 => "Retention · this session · 1 unread item".to_owned(),
        count => format!("Retention · this session · {count} unread items"),
    };
    let has_read = app
        .state
        .ui
        .toasts
        .activity()
        .iter()
        .any(NotificationRecord::is_read);
    let records = app.state.ui.toasts.activity().to_vec();
    let now = ctx.input(|input| input.time);

    let choice = Dialog::new(
        "BACKGROUND WORK · APPROVALS · SYSTEM EVENTS",
        "Notifications and activity",
        "Close",
    )
    .size(DialogSize::Lg)
    .ghost("Clear read")
    .ghost_enabled(has_read)
    .hint(&hint)
    .show(ctx, |ui| {
        toolbar(ui, app);
        ui.add_space(12.0);
        activity_list(ui, app, &records, now);
    });

    match choice {
        DialogChoice::Primary | DialogChoice::Cancelled => {
            app.state.workbench.notification_center_open = false;
        }
        DialogChoice::Ghost => {
            app.state.ui.toasts.clear_read();
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

fn toolbar(ui: &mut Ui, app: &mut RSpiceApp) {
    let available_width = ui.available_width();
    if available_width < 560.0 {
        ui.horizontal_wrapped(|ui| filter_buttons(ui, app));
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| activity_commands(ui, app));
    } else {
        ui.horizontal(|ui| {
            filter_buttons(ui, app);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Right-to-left placement keeps the command group flush with
                // the dialog edge. Emit settings first so the visible order
                // remains the mockup's “Mark all read · Settings”.
                notification_settings_button(ui, app);
                mark_all_read_button(ui, app);
            });
        });
    }
}

fn filter_buttons(ui: &mut Ui, app: &mut RSpiceApp) {
    for filter in NotificationFilter::ALL {
        let selected = app.state.workbench.notification_filter == filter;
        if ui.selectable_label(selected, filter.label()).clicked() {
            app.state.workbench.notification_filter = filter;
        }
    }
}

fn activity_commands(ui: &mut Ui, app: &mut RSpiceApp) {
    mark_all_read_button(ui, app);
    notification_settings_button(ui, app);
}

fn mark_all_read_button(ui: &mut Ui, app: &mut RSpiceApp) {
    let has_unread = app.state.ui.toasts.unread_count() > 0;
    if ui
        .add_enabled(has_unread, egui::Button::new("Mark all read"))
        .clicked()
    {
        app.state.ui.toasts.mark_all_read();
    }
}

fn notification_settings_button(ui: &mut Ui, app: &mut RSpiceApp) {
    if ui.button("Notification settings…").clicked() {
        // This is an atomic modal transition: retained open state changes in
        // the same interaction that opens the real Preferences surface.
        let command = begin_notification_settings_transition(
            &mut app.state.workbench.notification_center_open,
        );
        command.execute(app);
        ui.close_kind(UiKind::Modal);
    }
}

fn begin_notification_settings_transition(notification_center_open: &mut bool) -> Command {
    *notification_center_open = false;
    Command::Preferences
}

fn activity_list(ui: &mut Ui, app: &mut RSpiceApp, records: &[NotificationRecord], now: f64) {
    let filter = app.state.workbench.notification_filter;
    let visible: Vec<_> = records
        .iter()
        .filter(|record| filter_includes(filter, record.category()))
        .collect();

    if visible.is_empty() {
        empty_state(ui, records.is_empty());
        return;
    }

    for record in visible {
        if activity_row(ui, record, now).clicked() {
            app.state.ui.toasts.mark_read(record.id());
        }
        ui.add_space(6.0);
    }
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

fn activity_row(ui: &mut Ui, record: &NotificationRecord, now: f64) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let color = kind_color(&t, record.kind());
    let frame = Frame::new()
        .fill(if record.is_read() {
            t.color.bg_inset
        } else {
            t.color.bg_elevated
        })
        .stroke(Stroke::new(
            1.0,
            if record.is_read() {
                t.color.border
            } else {
                t.color.border_strong
            },
        ))
        .corner_radius(t.radius)
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(12.0), Sense::hover());
                ui.painter().circle_filled(
                    dot_rect.center(),
                    if record.is_read() { 3.0 } else { 4.0 },
                    color,
                );
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "{} · {}",
                                record.category().label(),
                                record.kind().label()
                            ))
                            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                            .color(color),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(age_label(now, record.created()))
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_faint),
                            );
                        });
                    });
                    ui.label(
                        egui::RichText::new(record.message())
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(if record.is_read() {
                                t.color.text_dim
                            } else {
                                t.color.text
                            }),
                    );
                });
            });
        });
    let response = ui.interact(
        frame.response.rect,
        ui.id().with(("notification", record.id())),
        Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!(
                "{} {}: {}",
                record.category().label(),
                record.kind().label(),
                record.message()
            ),
        )
    });
    response.on_hover_text(if record.is_read() {
        "Activity item already read"
    } else {
        "Mark activity item as read"
    })
}

fn empty_state(ui: &mut Ui, no_activity: bool) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::same(18))
        .show(ui, |ui| {
            ui.set_min_height(90.0);
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
        });
}

fn kind_color(tokens: &Tokens, kind: ToastKind) -> egui::Color32 {
    match kind {
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
    fn settings_transition_closes_activity_before_routing_to_preferences() {
        let mut notification_center_open = true;

        let command = begin_notification_settings_transition(&mut notification_center_open);

        assert!(!notification_center_open);
        assert_eq!(command, Command::Preferences);
    }
}
