//! Project Launcher specified by the workbench mockup.
//!
//! This surface is deliberately data-backed. It shows only project entries
//! that exist in the application's persisted recent-file store. Recovery is a
//! separate audited page; account-backed and governed-template controls remain
//! withheld until those product services can supply real records.

use std::path::Path;

use egui::{
    Align, Align2, Color32, Context, Frame, Id, Margin, Modifiers, Order, Popup, Rect, Response,
    Sense, Stroke, Ui, UiKind, Vec2, WidgetInfo, WidgetType, vec2,
};

use crate::common::RSpiceApp;
use crate::common::app::{RecentFile, RecentKind};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, IconButton, select};

use super::commands::Command;
use super::design_system::WorkbenchIcon;
use super::recovery::{
    RecoveryCandidate, RecoveryIntegrity, RecoveryNoticeTone, discard_checkpoint, open_comparison,
    recovery_replacement_block_reason, refresh_catalog_if_requested, start_local_safe_mode,
};
use super::state::{
    LocalSafeModeOptions, ProjectLauncherFilter, ProjectLauncherPage, ProjectLauncherSort,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectGroup {
    Pinned,
    Recent,
    Shared,
}

impl ProjectGroup {
    const ALL: [Self; 3] = [Self::Pinned, Self::Recent, Self::Shared];

    const fn label(self) -> &'static str {
        match self {
            Self::Pinned => "Pinned",
            Self::Recent => "Recent",
            Self::Shared => "Shared",
        }
    }
}

#[derive(Debug, Clone)]
struct ProjectEntry {
    recent: RecentFile,
    name: String,
    path_text: String,
    owner: Option<String>,
    opened_at_unix_ms: u64,
    group: ProjectGroup,
    shared: bool,
    available: bool,
    current: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct LauncherFocusState {
    prior_focus: Option<Id>,
    last_seen_pass: u64,
}

struct LauncherControlHeightOverride {
    ctx: Context,
    original: Option<Tokens>,
}

impl LauncherControlHeightOverride {
    fn new(ctx: &Context, enabled: bool) -> Self {
        let original = (*Tokens::get(ctx)).clone();
        if !enabled || original.metrics.ctl_h >= LAUNCHER_TOUCH_TARGET {
            return Self {
                ctx: ctx.clone(),
                original: None,
            };
        }
        let mut adjusted = original.clone();
        adjusted.metrics.ctl_h = LAUNCHER_TOUCH_TARGET;
        adjusted.install(ctx);
        Self {
            ctx: ctx.clone(),
            original: Some(original),
        }
    }
}

impl Drop for LauncherControlHeightOverride {
    fn drop(&mut self) {
        if let Some(original) = self.original.take() {
            original.install(&self.ctx);
        }
    }
}

enum LauncherAction {
    Close,
    Browse,
    NewProject,
    Open(RecentFile),
    Page(ProjectLauncherPage),
    Recover(RecoveryCandidate),
    RequestDiscard(RecoveryCandidate),
    StartSafeMode(LocalSafeModeOptions),
}

const LAUNCHER_DESKTOP_WIDTH: f32 = 1180.0;
const LAUNCHER_DESKTOP_HEIGHT: f32 = 650.0;
const LAUNCHER_VIEWPORT_INSET: f32 = 28.0;
const LAUNCHER_EDGE_TO_EDGE_MAX_WIDTH: f32 = 760.0;
const LAUNCHER_HEADER_HEIGHT: f32 = 58.0;
const LAUNCHER_STATUS_HEIGHT: f32 = 30.0;
const LAUNCHER_COMPACT_HEADER_HEIGHT: f32 = 52.0;
const LAUNCHER_COMPACT_STATUS_HEIGHT: f32 = 28.0;
const LAUNCHER_NAV_WIDTH: f32 = 184.0;
const LAUNCHER_COMPACT_NAV_HEIGHT: f32 = 42.0;
const LAUNCHER_PAGE_HEADING_MIN_HEIGHT: f32 = 76.0;
const LAUNCHER_PHONE_HEADING_MIN_HEIGHT: f32 = 68.0;
const LAUNCHER_PAGE_FOOTER_MIN_HEIGHT: f32 = 51.0;
const LAUNCHER_PHONE_MAX_WIDTH: f32 = 460.0;
const LAUNCHER_TOUCH_MAX_WIDTH: f32 = 820.0;
const LAUNCHER_TOUCH_TARGET: f32 = 44.0;
const LAUNCHER_SEARCH_MAX_WIDTH: f32 = 420.0;
const LAUNCHER_SORT_WIDTH: f32 = 145.0;
const LAUNCHER_ROW_MIN_HEIGHT: f32 = 47.0;
const LAUNCHER_GROUP_HEIGHT: f32 = 27.0;
const LAUNCHER_SEGMENT_MIN_WIDTH: f32 = 54.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct LauncherLayout {
    surface: Rect,
    edge_to_edge: bool,
    compact: bool,
    phone: bool,
    header_height: f32,
    status_height: f32,
}

impl LauncherLayout {
    fn resolve(viewport: Rect) -> Self {
        let edge_to_edge = viewport.width() <= LAUNCHER_EDGE_TO_EDGE_MAX_WIDTH;
        let surface = if edge_to_edge {
            viewport
        } else {
            Rect::from_center_size(
                viewport.center(),
                Vec2::new(
                    LAUNCHER_DESKTOP_WIDTH
                        .min((viewport.width() - LAUNCHER_VIEWPORT_INSET).max(1.0)),
                    LAUNCHER_DESKTOP_HEIGHT
                        .min((viewport.height() - LAUNCHER_VIEWPORT_INSET).max(1.0)),
                ),
            )
        };
        let compact = viewport.width() <= LAUNCHER_EDGE_TO_EDGE_MAX_WIDTH;
        Self {
            surface,
            edge_to_edge,
            compact,
            phone: viewport.width() <= LAUNCHER_PHONE_MAX_WIDTH,
            header_height: if compact {
                LAUNCHER_COMPACT_HEADER_HEIGHT
            } else {
                LAUNCHER_HEADER_HEIGHT
            },
            status_height: if compact {
                LAUNCHER_COMPACT_STATUS_HEIGHT
            } else {
                LAUNCHER_STATUS_HEIGHT
            },
        }
    }
}

pub(super) fn show(ctx: &Context, app: &mut RSpiceApp) {
    if !app.state.workbench.project_launcher_open {
        show_discard_confirmation(ctx, app);
        return;
    }

    refresh_catalog_if_requested(app);

    let screen = ctx.screen_rect();
    let layout = LauncherLayout::resolve(screen);
    let large_targets = app.state.workbench.coarse_pointer
        || screen.width() <= LAUNCHER_TOUCH_MAX_WIDTH
        || ctx.input(|input| input.has_touch_screen());
    let _control_height = LauncherControlHeightOverride::new(ctx, large_targets);
    let t = Tokens::get(ctx);
    let focus_state_id = Id::new("workbench.project_launcher.focus-session");
    let opened_this_pass = begin_launcher_focus(ctx, focus_state_id);
    let confirmation_open = app
        .state
        .workbench
        .project_launcher_recovery
        .pending_discard
        .is_some();
    let popup_was_open = Popup::is_any_open(ctx);
    let mut action = (!confirmation_open
        && !popup_was_open
        && ctx.input_mut(|input| input.consume_key(Modifiers::NONE, egui::Key::Escape)))
    .then_some(LauncherAction::Close);
    let surface_rect = layout.surface;
    let edge_to_edge = layout.edge_to_edge;
    let size = surface_rect.size();

    let area = egui::Area::new(Id::new("workbench.project_launcher"))
        .kind(UiKind::Modal)
        .sense(Sense::focusable_noninteractive())
        .order(Order::Foreground)
        .fixed_pos(screen.min);
    let modal_layer = area.layer();
    ctx.memory_mut(|memory| memory.set_modal_layer(modal_layer));
    let mut close_control_id = None;
    let area_response = area.show(ctx, |ui| {
        // The non-dismissable scrim prevents a project transition from
        // racing an interaction with the workbench below it.
        // accessibility-pointer-shim: it consumes pointer gestures while
        // remaining deliberately absent from keyboard/AT order.
        ui.allocate_rect(
            screen,
            Sense::click_and_drag().difference(Sense::focusable_noninteractive()),
        );
        let backdrop = if t.mode == crate::ui::Mode::Light {
            Color32::from_rgba_unmultiplied(41, 46, 50, 148)
        } else {
            Color32::from_rgba_unmultiplied(3, 7, 9, 199)
        };
        ui.painter().rect_filled(screen, 0.0, backdrop);

        let mut surface = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(surface_rect)
                .layout(egui::Layout::top_down(Align::Min)),
        );
        Frame::new()
            .fill(t.color.bg_app)
            .stroke(if edge_to_edge {
                Stroke::NONE
            } else {
                Stroke::new(1.0, t.color.border_strong)
            })
            .corner_radius(if edge_to_edge { 0.0 } else { t.radius_lg })
            .shadow(if edge_to_edge {
                egui::epaint::Shadow::NONE
            } else {
                t.shadow()
            })
            .show(&mut surface, |ui| {
                ui.set_min_size(size);
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                let header =
                    launcher_header(ui, layout.header_height, large_targets, layout.edge_to_edge);
                let header_bottom = ui.cursor().top();
                close_control_id = Some(header.id);
                if header.clicked() {
                    action = Some(LauncherAction::Close);
                }
                launcher_status(ui, app, layout.status_height, layout.compact);
                let status_bottom = ui.cursor().top();
                launcher_layout(ui, app, &mut action, layout);
                ui.painter().hline(
                    ui.max_rect().x_range(),
                    header_bottom,
                    Stroke::new(1.0, t.color.border),
                );
                ui.painter().hline(
                    ui.max_rect().x_range(),
                    status_bottom,
                    Stroke::new(1.0, t.color.border),
                );
            });
        if !edge_to_edge {
            // `Frame` backgrounds are inserted behind their children. Repaint
            // the inside border above full-bleed header/nav/footer fills so the
            // mockup's strong 1 px perimeter remains continuously visible.
            surface.painter().rect_stroke(
                surface_rect,
                t.radius_lg,
                Stroke::new(1.0, t.color.border_strong),
                egui::StrokeKind::Inside,
            );
        }
    });
    area_response
        .response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Window, true, "Start RSpice"));
    ctx.accesskit_node_builder(area_response.response.id, |node| {
        node.set_role(egui::accesskit::Role::Dialog);
        node.set_label("Start RSpice");
        node.set_description(
            "Open, create, recover, or configure an RSpice project and local session.",
        );
        node.set_modal();
    });

    if let Some(close_control_id) = close_control_id
        && !confirmation_open
        && (opened_this_pass || !focus_is_within_launcher(ctx, modal_layer))
    {
        let target = if app.state.workbench.project_launcher_page == ProjectLauncherPage::Projects {
            project_search_id()
        } else {
            close_control_id
        };
        ctx.memory_mut(|memory| memory.request_focus(target));
    }

    if let Some(action) = action {
        match action {
            LauncherAction::Close => app.state.workbench.project_launcher_open = false,
            LauncherAction::Browse => {
                app.state.workbench.project_launcher_open = false;
                Command::OpenProject.execute(app);
            }
            LauncherAction::NewProject => {
                app.state.workbench.project_launcher_open = false;
                Command::NewProject.execute(app);
            }
            LauncherAction::Open(recent) => {
                app.state.workbench.project_launcher_open = false;
                app.open_recent_file(recent);
            }
            LauncherAction::Page(page) => {
                app.state.workbench.project_launcher_page = page;
                if page == ProjectLauncherPage::Recovery {
                    app.state
                        .workbench
                        .project_launcher_recovery
                        .request_refresh();
                }
            }
            LauncherAction::Recover(candidate) => match open_comparison(app, &candidate) {
                Ok(()) => app.state.workbench.project_launcher_open = false,
                Err(error) => app.state.workbench.project_launcher_recovery.warning(error),
            },
            LauncherAction::RequestDiscard(candidate) => {
                app.state
                    .workbench
                    .project_launcher_recovery
                    .pending_discard = Some(candidate);
            }
            LauncherAction::StartSafeMode(options) => match start_local_safe_mode(app, options) {
                Ok(()) => app.state.workbench.project_launcher_open = false,
                Err(error) => app.state.workbench.project_launcher_recovery.warning(error),
            },
        }
    }
    if !app.state.workbench.project_launcher_open {
        Popup::close_all(ctx);
        restore_launcher_focus(ctx, focus_state_id, area_response.response.id, modal_layer);
    }
    show_discard_confirmation(ctx, app);
}

fn launcher_layout(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    action: &mut Option<LauncherAction>,
    layout: LauncherLayout,
) {
    if layout.compact {
        Frame::new()
            .fill(Tokens::get(ui.ctx()).color.bg_panel)
            .inner_margin(Margin::ZERO)
            .show(ui, |ui| {
                ui.set_min_height(
                    LAUNCHER_COMPACT_NAV_HEIGHT.max(Tokens::get(ui.ctx()).metrics.ctl_h),
                );
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                let navigation = egui::ScrollArea::horizontal()
                    .id_salt("project-launcher-mobile-navigation")
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| ui.scope(|ui| launcher_nav(ui, app, action, true)));
                ui.ctx()
                    .accesskit_node_builder(navigation.inner.response.id, |node| {
                        node.set_role(egui::accesskit::Role::Navigation);
                        node.set_label("Startup pages");
                    });
            });
        let nav_bottom = ui.cursor().top();
        launcher_page(ui, app, action, layout);
        ui.painter().hline(
            ui.max_rect().x_range(),
            nav_bottom,
            Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
        );
        return;
    }

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        Frame::new()
            .fill(Tokens::get(ui.ctx()).color.bg_panel)
            .inner_margin(Margin::ZERO)
            .corner_radius(egui::CornerRadius {
                nw: 0,
                ne: 0,
                sw: if layout.edge_to_edge { 0 } else { 6 },
                se: 0,
            })
            .show(ui, |ui| {
                ui.set_width(LAUNCHER_NAV_WIDTH);
                ui.set_min_height(ui.available_height());
                let navigation = ui.scope(|ui| launcher_nav(ui, app, action, false));
                ui.ctx()
                    .accesskit_node_builder(navigation.response.id, |node| {
                        node.set_role(egui::accesskit::Role::Navigation);
                        node.set_label("Startup pages");
                    });
            });
        let nav_right = ui.cursor().left();
        launcher_page(ui, app, action, layout);
        ui.painter().vline(
            nav_right,
            ui.max_rect().y_range(),
            Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
        );
    });
}

fn launcher_nav(ui: &mut Ui, app: &RSpiceApp, action: &mut Option<LauncherAction>, compact: bool) {
    let t = Tokens::get(ui.ctx());
    let render = |ui: &mut Ui, page: ProjectLauncherPage, action: &mut Option<LauncherAction>| {
        let icon = match page {
            ProjectLauncherPage::Projects => WorkbenchIcon::Folder,
            ProjectLauncherPage::Recovery => WorkbenchIcon::History,
            ProjectLauncherPage::SafeMode => WorkbenchIcon::Warning,
        };
        let label = page.label();
        let selected = app.state.workbench.project_launcher_page == page;
        let label_font = theme::sans(tokens::FS_2, FontWeight::Regular);
        let label_width = ui
            .painter()
            .layout_no_wrap(label.to_owned(), label_font.clone(), t.color.text)
            .size()
            .x;
        let row_height = (if compact {
            LAUNCHER_COMPACT_NAV_HEIGHT
        } else {
            35.0
        })
        .max(t.metrics.ctl_h);
        let size = if compact {
            Vec2::new(
                (label_width + 2.0 * 11.0 + 15.0 + 8.0).max(92.0),
                row_height,
            )
        } else {
            Vec2::new(ui.available_width(), row_height)
        };
        let (rect, mut response) = ui.allocate_exact_size(size, Sense::click());
        let fill = if selected {
            t.color.bg_active
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect_filled(rect, 0.0, fill);
        if selected {
            if compact {
                ui.painter().rect_filled(
                    Rect::from_min_max(
                        egui::pos2(rect.left(), rect.bottom() - 2.0),
                        rect.right_bottom(),
                    ),
                    0.0,
                    t.color.accent,
                );
            } else {
                ui.painter().rect_filled(
                    Rect::from_min_size(rect.min, Vec2::new(2.0, rect.height())),
                    0.0,
                    t.color.accent,
                );
            }
        }
        let horizontal_padding = if compact { 11.0 } else { 13.0 };
        let icon_column = if compact { 15.0 } else { 20.0 };
        let icon_left = rect.left() + horizontal_padding;
        let icon_rect = Rect::from_center_size(
            egui::pos2(icon_left + 7.5, rect.center().y),
            Vec2::splat(15.0),
        );
        icon.paint(
            ui.painter(),
            icon_rect,
            if selected || response.hovered() {
                Tokens::get(ui.ctx()).color.text
            } else {
                Tokens::get(ui.ctx()).color.text_dim
            },
        );
        ui.painter().text(
            egui::pos2(icon_left + icon_column + 8.0, rect.center().y),
            Align2::LEFT_CENTER,
            label,
            label_font,
            if selected || response.hovered() {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        response.widget_info(|| {
            WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), selected, label)
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_selected(selected);
            node.set_position_in_set(page_index(page) + 1);
            node.set_size_of_set(ProjectLauncherPage::ALL.len());
            if selected {
                node.set_aria_current(egui::accesskit::AriaCurrent::Page);
            }
        });
        theme::paint_focus_ring(ui, &response, rect);
        if response_activated(ui, &response) {
            response.mark_changed();
            *action = Some(LauncherAction::Page(page));
        }
    };

    if compact {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for page in ProjectLauncherPage::ALL {
                render(ui, page, action);
            }
        });
    } else {
        ui.vertical(|ui| {
            ui.add_space(8.0);
            for page in ProjectLauncherPage::ALL {
                render(ui, page, action);
            }
        });
    }
}

fn launcher_page(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    action: &mut Option<LauncherAction>,
    layout: LauncherLayout,
) {
    ui.spacing_mut().item_spacing = Vec2::ZERO;
    ui.set_min_size(ui.available_size());
    match app.state.workbench.project_launcher_page {
        ProjectLauncherPage::Projects => launcher_body(ui, app, action, layout),
        ProjectLauncherPage::Recovery => recovery_page(ui, app, action, layout),
        ProjectLauncherPage::SafeMode => safe_mode_page(ui, app, action, layout),
    }
}

fn launcher_header(ui: &mut Ui, height: f32, large_targets: bool, edge_to_edge: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let mut close = None;
    let header = Frame::new()
        .fill(t.color.bg_panel)
        .corner_radius(egui::CornerRadius {
            nw: if edge_to_edge { 0 } else { 6 },
            ne: if edge_to_edge { 0 } else { 6 },
            sw: 0,
            se: 0,
        })
        .inner_margin(Margin::symmetric(15, 0))
        .show(ui, |ui| {
            ui.set_min_height(height);
            ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(27.0), Sense::hover());
                paint_brand_logo(ui.painter(), icon_rect);
                ui.add_space(11.0);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    ui.label(
                        egui::RichText::new("RSPICE COMMERCIAL WORKBENCH")
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint)
                            .extra_letter_spacing(0.09 * tokens::FS_0),
                    );
                    let heading = ui.label(
                        egui::RichText::new("Start RSpice")
                            .font(theme::sans(20.0, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.ctx().accesskit_node_builder(heading.id, |node| {
                        node.set_role(egui::accesskit::Role::Heading);
                        node.set_label("Start RSpice");
                        node.set_level(2);
                    });
                });
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    close = Some(
                        IconButton::new(Icon::Close)
                            .side(if large_targets {
                                LAUNCHER_TOUCH_TARGET
                            } else {
                                28.0
                            })
                            .tooltip("Close project launcher (Esc)")
                            .show(ui),
                    );
                });
            });
        });
    close.unwrap_or(header.response)
}

/// Paint the checked-in `assets/brand/logo.svg` geometry directly so the
/// native, browser, and mobile renderers share the same mark without an SVG
/// decoder or texture-lifetime dependency.
fn paint_brand_logo(painter: &egui::Painter, rect: Rect) {
    let square =
        Rect::from_center_size(rect.center(), Vec2::splat(rect.width().min(rect.height())));
    let scale = square.width() / 96.0;
    let ink = Color32::from_rgb(23, 24, 26);
    let gold = Color32::from_rgb(242, 184, 36);
    painter.rect_filled(square, 21.0 * scale, gold);

    let point = |x: f32, y: f32| {
        egui::pos2(
            square.left() + (14.4 + 0.7 * x) * scale,
            square.top() + (14.4 + 0.7 * y) * scale,
        )
    };
    let heavy = Stroke::new((8.0 * 0.7 * scale).max(1.0), ink);
    for (from, to) in [
        (point(14.0, 34.0), point(27.5, 34.0)),
        (point(14.0, 62.0), point(27.5, 62.0)),
        (point(69.5, 48.0), point(82.0, 48.0)),
    ] {
        painter.line_segment([from, to], heavy);
    }
    let terminal = Stroke::new((4.0 * 0.7 * scale).max(1.0), ink);
    for center in [point(8.0, 34.0), point(8.0, 62.0), point(88.0, 48.0)] {
        painter.circle_stroke(center, 6.0 * 0.7 * scale, terminal);
    }
    painter.add(egui::Shape::convex_polygon(
        vec![point(25.5, 20.61), point(25.5, 75.39), point(70.5, 48.0)],
        ink,
        heavy,
    ));
}

fn launcher_status(ui: &mut Ui, app: &RSpiceApp, height: f32, compact: bool) {
    let t = Tokens::get(ui.ctx());
    let status = Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(Margin::symmetric(15, 0))
        .show(ui, |ui| {
            ui.set_min_height(height);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 9.0;
                let (dot, _) = ui.allocate_exact_size(Vec2::splat(9.0), Sense::hover());
                let (license_text, color) = app.state.license.as_ref().map_or_else(
                    || ("Activation required".to_owned(), t.color.warn),
                    |_| ("Commercial entitlement active".to_owned(), t.color.ok),
                );
                ui.painter().circle_filled(dot.center(), 3.0, color);
                ui.label(
                    egui::RichText::new(license_text)
                        .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                if !compact {
                    ui.label(
                        egui::RichText::new(concat!(
                            "RSpice Desktop ",
                            env!("CARGO_PKG_VERSION"),
                            " · engine ",
                            env!("RSPICE_BUILD_HASH")
                        ))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                    );
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(if app.state.workbench.safe_mode.active {
                                "Safe mode · local session isolation"
                            } else {
                                "Local runtime"
                            })
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                        );
                    });
                }
            });
        });
    ui.ctx().accesskit_node_builder(status.response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label("RSpice startup status");
    });
}

fn launcher_body(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    action: &mut Option<LauncherAction>,
    layout: LauncherLayout,
) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .inner_margin(if layout.phone {
            Margin::symmetric(11, 9)
        } else {
            Margin::symmetric(16, 10)
        })
        .show(ui, |ui| {
            ui.set_min_height(
                (if layout.phone {
                    LAUNCHER_PHONE_HEADING_MIN_HEIGHT
                } else {
                    LAUNCHER_PAGE_HEADING_MIN_HEIGHT
                }) - if layout.phone { 18.0 } else { 20.0 },
            );
            if layout.compact {
                project_heading_copy(ui);
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| project_heading_actions(ui, action, false));
            } else {
                ui.horizontal(|ui| {
                    project_heading_copy(ui);
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        // Right-to-left allocation right-aligns the button group, so
                        // allocate in reverse to preserve the mockup's visible order.
                        project_heading_actions(ui, action, true);
                    });
                });
            }
        });
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border),
    );

    launcher_toolbar(ui, app, layout);

    let list_height = (ui.available_height()
        - launcher_footer_reserve(ui, layout, &[("Continue without a project", false)]))
    .max(0.0);
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), list_height),
        egui::Layout::top_down(Align::Min),
        |ui| {
            egui::ScrollArea::vertical()
                .id_salt("workbench.project_launcher.projects")
                .auto_shrink([false, false])
                .show(ui, |ui| project_list(ui, app, action, layout));
        },
    );
    launcher_page_footer(ui, layout, |ui| {
        if Button::new("Continue without a project").show(ui).clicked() {
            *action = Some(LauncherAction::Close);
        }
    });
}

fn project_heading_copy(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 3.0;
        ui.label(
            egui::RichText::new("PROJECT LAUNCHER · LOCAL AND SHARED")
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(t.color.text_faint)
                .extra_letter_spacing(0.09 * tokens::FS_0),
        );
        let heading = ui.label(
            egui::RichText::new("Open engineering work")
                .font(theme::sans(15.0, FontWeight::SemiBold))
                .color(t.color.text),
        );
        ui.ctx().accesskit_node_builder(heading.id, |node| {
            node.set_role(egui::accesskit::Role::Heading);
            node.set_label("Open engineering work");
            node.set_level(3);
        });
        ui.label(
            egui::RichText::new("Recent, pinned, and shared projects use one searchable launcher.")
                .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                .color(t.color.text_dim),
        );
    });
}

fn project_heading_actions(
    ui: &mut Ui,
    action: &mut Option<LauncherAction>,
    reverse_allocation: bool,
) {
    ui.spacing_mut().item_spacing.x = 6.0;
    if reverse_allocation {
        if Button::new("New project")
            .icon(Icon::Add)
            .accent()
            .show(ui)
            .clicked()
        {
            *action = Some(LauncherAction::NewProject);
        }
        if Button::new("Browse…").icon(Icon::Folder).show(ui).clicked() {
            *action = Some(LauncherAction::Browse);
        }
    } else {
        if Button::new("Browse…").icon(Icon::Folder).show(ui).clicked() {
            *action = Some(LauncherAction::Browse);
        }
        if Button::new("New project")
            .icon(Icon::Add)
            .accent()
            .show(ui)
            .clicked()
        {
            *action = Some(LauncherAction::NewProject);
        }
    }
}

fn project_list(
    ui: &mut Ui,
    app: &RSpiceApp,
    action: &mut Option<LauncherAction>,
    layout: LauncherLayout,
) {
    ui.spacing_mut().item_spacing = Vec2::ZERO;
    ui.set_min_width(ui.available_width());
    let entries = project_entries(app);
    if entries.is_empty() {
        empty_project_list(
            ui,
            app.state.workbench.project_launcher_query.is_empty()
                && app.state.workbench.project_launcher_filter == ProjectLauncherFilter::All,
        );
        return;
    }

    for group in ProjectGroup::ALL {
        let group_entries = entries
            .iter()
            .filter(|entry| entry.group == group)
            .collect::<Vec<_>>();
        if group_entries.is_empty() {
            continue;
        }
        project_group_header(ui, group.label(), group_entries.len());
        for entry in group_entries {
            let row = project_row(ui, entry, layout);
            if entry.available && response_activated(ui, &row) {
                *action = Some(LauncherAction::Open(entry.recent.clone()));
            }
        }
    }
}

fn project_group_header(ui: &mut Ui, label: &str, count: usize) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_panel_2)
        .inner_margin(Margin::symmetric(13, 0))
        .show(ui, |ui| {
            ui.set_min_height(LAUNCHER_GROUP_HEIGHT);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(label)
                        .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(count.to_string())
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                });
            });
        });
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border),
    );
}

fn launcher_page_footer(ui: &mut Ui, layout: LauncherLayout, add: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let top = ui.cursor().top();
    Frame::new()
        .fill(t.color.bg_panel)
        .corner_radius(egui::CornerRadius {
            nw: 0,
            ne: 0,
            sw: 0,
            se: if layout.edge_to_edge { 0 } else { 6 },
        })
        .inner_margin(Margin::symmetric(if layout.phone { 9 } else { 16 }, 8))
        .show(ui, |ui| {
            ui.set_min_height(LAUNCHER_PAGE_FOOTER_MIN_HEIGHT - 16.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
                add(ui);
            });
        });
    ui.painter().hline(
        ui.max_rect().x_range(),
        top,
        Stroke::new(1.0, t.color.border),
    );
}

fn launcher_footer_reserve(ui: &Ui, layout: LauncherLayout, buttons: &[(&str, bool)]) -> f32 {
    let available = (ui.available_width() - if layout.phone { 18.0 } else { 32.0 }).max(1.0);
    let mut rows = 1usize;
    let mut used = 0.0;
    for &(label, accent) in buttons {
        let font = theme::sans(
            tokens::FS_0,
            if accent {
                FontWeight::SemiBold
            } else {
                FontWeight::Regular
            },
        );
        let width = ui
            .painter()
            .layout_no_wrap(label.to_owned(), font, Color32::WHITE)
            .size()
            .x
            + 20.0;
        let next = if used == 0.0 {
            width
        } else {
            used + 6.0 + width
        };
        if used > 0.0 && next > available {
            rows += 1;
            used = width;
        } else {
            used = next;
        }
    }
    (Tokens::get(ui.ctx()).metrics.ctl_h * rows as f32 + 6.0 * rows.saturating_sub(1) as f32 + 16.0)
        .max(LAUNCHER_PAGE_FOOTER_MIN_HEIGHT)
}

fn recovery_page(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    action: &mut Option<LauncherAction>,
    layout: LauncherLayout,
) {
    let t = Tokens::get(ui.ctx());
    launcher_page_heading(
        ui,
        "STARTUP RECOVERY · NON-DESTRUCTIVE",
        "Recover interrupted work",
        "Recovery opens comparison copies. It never overwrites the saved project, immutable results, or approved evidence until you explicitly accept changes.",
        layout,
    );
    let body_height = (ui.available_height()
        - launcher_footer_reserve(
            ui,
            layout,
            &[
                ("Discard selected checkpoint…", false),
                ("Recovery options", false),
                ("Open recovery comparison", true),
            ],
        ))
    .max(0.0);
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), body_height),
        egui::Layout::top_down(Align::Min),
        |ui| {
            egui::ScrollArea::vertical()
                .id_salt("workbench.project_launcher.recovery")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    if let Some(notice) =
                        app.state.workbench.project_launcher_recovery.notice.clone()
                    {
                        let color = match notice.tone {
                            RecoveryNoticeTone::Info => t.color.ok,
                            RecoveryNoticeTone::Warning => t.color.warn,
                        };
                        Frame::new()
                            .fill(color.gamma_multiply(0.10))
                            .stroke(Stroke::new(1.0, color.gamma_multiply(0.65)))
                            .corner_radius(t.radius)
                            .inner_margin(Margin::symmetric(12, 9))
                            .outer_margin(Margin::symmetric(16, 10))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(notice.message)
                                        .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                                        .color(t.color.text),
                                );
                            });
                    }

                    let candidates = app
                        .state
                        .workbench
                        .project_launcher_recovery
                        .candidates
                        .clone();
                    let selected = app
                        .state
                        .workbench
                        .project_launcher_recovery
                        .selected_checkpoint
                        .clone();
                    Frame::new()
                        .inner_margin(Margin::symmetric(16, 0))
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            if candidates.is_empty() {
                                Frame::new()
                                    .inner_margin(Margin::symmetric(18, 28))
                                    .show(ui, |ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.label(
                                                egui::RichText::new("No interrupted work found")
                                                    .font(theme::sans(
                                                        tokens::FS_2,
                                                        FontWeight::SemiBold,
                                                    ))
                                                    .color(t.color.text),
                                            );
                                            ui.label(
                                                egui::RichText::new(
                                                    "No eligible interrupted-session checkpoint is associated with a recent local schematic.",
                                                )
                                                .font(theme::sans(
                                                    tokens::FS_1,
                                                    FontWeight::Regular,
                                                ))
                                                .color(t.color.text_dim),
                                            );
                                        });
                                    });
                            } else {
                                for candidate in candidates {
                                    let is_selected =
                                        selected.as_ref() == Some(&candidate.checkpoint);
                                    let row = recovery_row(ui, &candidate, is_selected, layout);
                                    if response_activated(ui, &row) {
                                        app.state
                                            .workbench
                                            .project_launcher_recovery
                                            .select(candidate.checkpoint.clone());
                                    }
                                    ui.painter().hline(
                                        ui.max_rect().x_range(),
                                        ui.cursor().top(),
                                        Stroke::new(1.0, t.color.border),
                                    );
                                }
                            }
                        });

                    if let Some(candidate) = app
                        .state
                        .workbench
                        .project_launcher_recovery
                        .selected()
                        .cloned()
                    {
                        recovery_contract(ui, &candidate, layout);
                    }
                });
        },
    );

    let selected = app
        .state
        .workbench
        .project_launcher_recovery
        .selected()
        .cloned();
    launcher_page_footer(ui, layout, |ui| {
        let can_discard = selected
            .as_ref()
            .is_some_and(RecoveryCandidate::can_discard);
        let discard = Button::new("Discard selected checkpoint…")
            .enabled(can_discard)
            .show(ui);
        let discard = if selected
            .as_ref()
            .is_some_and(RecoveryCandidate::is_legacy_checkpoint)
        {
            discard.on_hover_text(
                "Legacy checkpoint ownership cannot be proven; open it non-destructively or use explicit recovery maintenance or migration",
            )
        } else {
            discard
        };
        if discard.clicked()
            && let Some(candidate) = selected.clone()
        {
            *action = Some(LauncherAction::RequestDiscard(candidate));
        }
        if Button::new("Recovery options").show(ui).clicked() {
            *action = Some(LauncherAction::Page(ProjectLauncherPage::SafeMode));
        }
        let recoverable = selected
            .as_ref()
            .is_some_and(|candidate| candidate.integrity.is_recoverable());
        let replacement_block_reason = recoverable
            .then(|| recovery_replacement_block_reason(&app.state))
            .flatten();
        let response = Button::new("Open recovery comparison")
            .accent()
            .enabled(recoverable && replacement_block_reason.is_none())
            .show(ui);
        let response = if let Some(reason) = replacement_block_reason {
            response.on_hover_text(reason)
        } else if !recoverable && selected.is_some() {
            response.on_hover_text("The selected checkpoint failed integrity validation")
        } else {
            response
        };
        if response.clicked()
            && let Some(candidate) = selected
        {
            *action = Some(LauncherAction::Recover(candidate));
        }
    });
}

fn recovery_row(
    ui: &mut Ui,
    candidate: &RecoveryCandidate,
    selected: bool,
    layout: LauncherLayout,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let response = Frame::new()
        .fill(if selected {
            t.color.bg_hover
        } else {
            egui::Color32::TRANSPARENT
        })
        .inner_margin(Margin::ZERO)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.set_min_height(54.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 11.0;
                let (radio_rect, _) = ui.allocate_exact_size(Vec2::splat(20.0), Sense::hover());
                ui.painter().circle_stroke(
                    radio_rect.center(),
                    6.0,
                    Stroke::new(
                        1.0,
                        if selected {
                            t.color.accent
                        } else {
                            t.color.border_strong
                        },
                    ),
                );
                if selected {
                    ui.painter()
                        .circle_filled(radio_rect.center(), 3.0, t.color.accent);
                }

                let trailing_width = if layout.compact { 64.0 } else { 211.0 };
                let content_width = (ui.available_width() - trailing_width - 11.0).max(1.0);
                ui.allocate_ui_with_layout(
                    vec2(content_width, 54.0),
                    egui::Layout::top_down(Align::Min).with_cross_align(Align::Min),
                    |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.add_space(7.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "{} · interrupted session",
                                candidate.display_name
                            ))
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(t.color.text),
                        );
                        ui.label(
                            egui::RichText::new(recovery_summary(candidate))
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                    },
                );
                if !layout.compact {
                    ui.add_sized(
                        [130.0, 54.0],
                        egui::Label::new(
                            egui::RichText::new(&candidate.age)
                                .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                                .color(t.color.text_faint),
                        ),
                    );
                }
                ui.add_sized(
                    [if layout.compact { 64.0 } else { 70.0 }, 54.0],
                    egui::Label::new(
                        egui::RichText::new(if candidate.integrity.is_recoverable() {
                            "review"
                        } else {
                            "blocked"
                        })
                        .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                        .color(if candidate.integrity.is_recoverable() {
                            t.color.warn
                        } else {
                            t.color.err
                        }),
                    ),
                );
            });
        })
        .response
        .interact(Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::RadioButton,
            ui.is_enabled(),
            selected,
            format!("Select recovery checkpoint for {}", candidate.display_name),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(recovery_summary(candidate));
    });
    theme::paint_focus_ring(ui, &response, response.rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn recovery_summary(candidate: &RecoveryCandidate) -> String {
    match &candidate.integrity {
        RecoveryIntegrity::Verified {
            baseline_available,
            baseline_note,
            components,
            wires,
            changed_objects,
        } => {
            let delta = changed_objects.map_or_else(
                || "change count unavailable".to_owned(),
                |count| format!("{count} structural changes"),
            );
            format!(
                "{components} components · {wires} wires · {delta} · {}",
                if *baseline_available && baseline_note.is_none() {
                    "checkpoint and baseline verified"
                } else if *baseline_available {
                    "checkpoint verified · saved baseline changed or unrecorded"
                } else {
                    "checkpoint verified · baseline unavailable"
                }
            )
        }
        RecoveryIntegrity::Invalid(error) => error.clone(),
    }
}

fn recovery_contract(ui: &mut Ui, candidate: &RecoveryCandidate, layout: LauncherLayout) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .outer_margin(Margin::symmetric(16, 15))
        .show(ui, |ui| {
            ui.painter().hline(
                ui.max_rect().x_range(),
                ui.cursor().top(),
                Stroke::new(1.0, t.color.border),
            );
            recovery_contract_row(
                ui,
                "Saved source",
                &candidate.original.display().to_string(),
                layout,
            );
            recovery_contract_row(
                ui,
                "Recovery point",
                &candidate.checkpoint.display().to_string(),
                layout,
            );
            match &candidate.integrity {
                RecoveryIntegrity::Verified {
                    baseline_available,
                    baseline_note,
                    ..
                } => {
                    recovery_contract_row(
                        ui,
                        "Protected data",
                        "Saved source and checkpoint remain unchanged; recovery opens as an unsaved project.",
                        layout,
                    );
                    recovery_contract_row(
                        ui,
                        "Recommended action",
                        if *baseline_available {
                            "Compare the editable recovery candidate with the read-only saved baseline."
                        } else {
                            baseline_note.as_deref().unwrap_or(
                                "Review the verified checkpoint without a saved baseline.",
                            )
                        },
                        layout,
                    );
                    if let Some(note) = baseline_note {
                        recovery_contract_row(ui, "Baseline status", note, layout);
                    }
                }
                RecoveryIntegrity::Invalid(error) => {
                    recovery_contract_row(ui, "Integrity", error, layout);
                    recovery_contract_row(
                        ui,
                        "Required action",
                        "Retain or discard the checkpoint; invalid content is never opened.",
                        layout,
                    );
                }
            }
        });
}

fn recovery_contract_row(ui: &mut Ui, label: &str, value: &str, layout: LauncherLayout) {
    let t = Tokens::get(ui.ctx());
    if layout.phone {
        // The mockup collapses the two-column grid into two ruled cells, not
        // one card-like stacked row.
        for (text, strong) in [(label, true), (value, false)] {
            Frame::new()
                .inner_margin(Margin::symmetric(9, 8))
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.set_min_height(18.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(text)
                                .font(theme::sans(
                                    tokens::FS_2,
                                    if strong {
                                        FontWeight::SemiBold
                                    } else {
                                        FontWeight::Regular
                                    },
                                ))
                                .color(if strong {
                                    t.color.text_dim
                                } else {
                                    t.color.text
                                }),
                        )
                        .wrap(),
                    );
                });
            ui.painter().hline(
                ui.max_rect().x_range(),
                ui.cursor().top(),
                Stroke::new(1.0, t.color.border),
            );
        }
        return;
    }
    Frame::new()
        .inner_margin(Margin::symmetric(9, 8))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            let add_label = |ui: &mut Ui| {
                ui.label(
                    egui::RichText::new(label)
                        .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
            };
            let add_value = |ui: &mut Ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(value)
                            .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                            .color(t.color.text),
                    )
                    .wrap(),
                );
            };
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.allocate_ui_with_layout(
                    vec2(170.0, 18.0),
                    egui::Layout::left_to_right(Align::Center),
                    add_label,
                );
                add_value(ui);
            });
        });
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border),
    );
}

fn safe_mode_page(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    action: &mut Option<LauncherAction>,
    layout: LauncherLayout,
) {
    let t = Tokens::get(ui.ctx());
    launcher_page_heading(
        ui,
        "SAFE MODE · STARTUP ISOLATION",
        "Start with recoverable session state isolated",
        "Safe mode changes only the current launch. The prior session remains the source of truth for the next normal launch.",
        layout,
    );
    let active = app.state.workbench.safe_mode.active;
    let body_height = (ui.available_height()
        - launcher_footer_reserve(ui, layout, &[("Start RSpice in safe mode", true)]))
    .max(0.0);
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), body_height),
        egui::Layout::top_down(Align::Min),
        |ui| {
            egui::ScrollArea::vertical()
                .id_salt("workbench.project_launcher.safe-mode")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    if active {
                        Frame::new()
                            .fill(t.color.ok.gamma_multiply(0.10))
                            .stroke(Stroke::new(1.0, t.color.ok.gamma_multiply(0.65)))
                            .corner_radius(t.radius)
                            .inner_margin(Margin::symmetric(12, 9))
                            .outer_margin(Margin::symmetric(16, 10))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("Safe mode is active for this launch")
                                        .font(theme::sans(
                                            tokens::FS_1,
                                            FontWeight::SemiBold,
                                        ))
                                        .color(t.color.text),
                                );
                            });
                    }

                    Frame::new()
                        .inner_margin(Margin::symmetric(16, 0))
                        .show(ui, |ui| {
                            ui.add_enabled_ui(!active, |ui| {
                                safe_mode_option(
                                    ui,
                                    &mut app
                                        .state
                                        .workbench
                                        .safe_mode
                                        .draft
                                        .isolate_prior_documents,
                                    "Do not reopen prior documents",
                                    "Start with a new unsaved project. The complete prior session is restored on the next normal launch.",
                                );
                                safe_mode_option(
                                    ui,
                                    &mut app.state.workbench.safe_mode.draft.reset_layout,
                                    "Reset dock and monitor geometry",
                                    "Restore navigator, inspector, console, and dock dimensions to the primary workbench layout.",
                                );
                            });
                        });

                });
        },
    );

    let options = app.state.workbench.safe_mode.draft;
    launcher_page_footer(ui, layout, |ui| {
        let response = Button::new("Start RSpice in safe mode")
            .accent()
            .enabled(!active && options.has_effect())
            .show(ui);
        let response = if active {
            response.on_hover_text("Safe mode is already active for this launch")
        } else if !options.has_effect() {
            response.on_hover_text("Select at least one isolation option")
        } else {
            response
        };
        if response.clicked() {
            *action = Some(LauncherAction::StartSafeMode(options));
        }
    });
}

fn safe_mode_option(ui: &mut Ui, checked: &mut bool, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    let enabled = ui.is_enabled();
    let response = Frame::new()
        .inner_margin(Margin::ZERO)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.set_min_height(54.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 11.0;
                let (check_rect, _) = ui.allocate_exact_size(Vec2::splat(20.0), Sense::hover());
                let box_rect = Rect::from_center_size(check_rect.center(), Vec2::splat(13.0));
                ui.painter().rect(
                    box_rect,
                    t.radius.min(2.0),
                    if *checked {
                        t.color.accent
                    } else {
                        t.color.bg_inset
                    },
                    Stroke::new(
                        1.0,
                        if *checked {
                            t.color.accent
                        } else {
                            t.color.border_strong
                        },
                    ),
                    egui::StrokeKind::Inside,
                );
                if *checked {
                    Icon::Check.paint(ui.painter(), box_rect.shrink(2.0), t.color.accent_ink);
                }
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    ui.label(
                        egui::RichText::new(title)
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.label(
                        egui::RichText::new(detail)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                });
            });
        })
        .response
        .interact(if enabled {
            Sense::click()
        } else {
            Sense::hover()
        });
    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, enabled, *checked, title));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(detail);
    });
    theme::paint_focus_ring(ui, &response, response.rect);
    if enabled && response_activated(ui, &response) {
        *checked = !*checked;
    }
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border),
    );
}

fn launcher_page_heading(
    ui: &mut Ui,
    eyebrow: &str,
    title: &str,
    detail: &str,
    layout: LauncherLayout,
) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .inner_margin(if layout.phone {
            Margin::symmetric(11, 9)
        } else {
            Margin::symmetric(16, 10)
        })
        .show(ui, |ui| {
            ui.set_min_height(
                (if layout.phone {
                    LAUNCHER_PHONE_HEADING_MIN_HEIGHT
                } else {
                    LAUNCHER_PAGE_HEADING_MIN_HEIGHT
                }) - if layout.phone { 18.0 } else { 20.0 },
            );
            ui.with_layout(
                egui::Layout::left_to_right(if layout.compact {
                    Align::Min
                } else {
                    Align::Center
                })
                .with_main_wrap(true),
                |ui| {
                    ui.spacing_mut().item_spacing = vec2(12.0, 3.0);
                    ui.label(
                        egui::RichText::new(eyebrow)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint)
                            .extra_letter_spacing(0.09 * tokens::FS_0),
                    );
                    let heading = ui.label(
                        egui::RichText::new(title)
                            .font(theme::sans(15.0, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.ctx().accesskit_node_builder(heading.id, |node| {
                        node.set_role(egui::accesskit::Role::Heading);
                        node.set_label(title);
                        node.set_level(3);
                    });
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(detail)
                                .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                                .color(t.color.text_dim),
                        )
                        .wrap(),
                    );
                },
            );
        });
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border),
    );
}

fn show_discard_confirmation(ctx: &Context, app: &mut RSpiceApp) {
    let Some(candidate) = app
        .state
        .workbench
        .project_launcher_recovery
        .pending_discard
        .clone()
    else {
        return;
    };
    let choice = Dialog::new(
        "Recovery",
        "Discard recovery checkpoint?",
        "Discard checkpoint",
    )
    .description(
        "Confirm permanent deletion of the selected autosave checkpoint without modifying its saved source.",
    )
    .size(DialogSize::Transaction)
    .destructive()
    .ghost("Keep checkpoint")
    .hint("The saved source is not modified")
    .show(ctx, |ui| {
        ui.label(format!(
            "Discard the autosave checkpoint for '{}'? This removes only '{}'.",
            candidate.display_name,
            candidate.checkpoint.display()
        ));
    });

    match choice {
        DialogChoice::None => {}
        DialogChoice::Primary => {
            let outcome = discard_checkpoint(&candidate);
            let catalog = &mut app.state.workbench.project_launcher_recovery;
            catalog.pending_discard = None;
            match outcome {
                Ok(message) => catalog.info(message),
                Err(error) => catalog.warning(error),
            }
            catalog.request_refresh();
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state
                .workbench
                .project_launcher_recovery
                .pending_discard = None;
        }
        DialogChoice::Secondary => {}
    }
}

fn launcher_toolbar(ui: &mut Ui, app: &mut RSpiceApp, layout: LauncherLayout) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .inner_margin(Margin::symmetric(if layout.phone { 9 } else { 12 }, 7))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let filter_width = if layout.compact {
                    0.0
                } else {
                    ProjectLauncherFilter::ALL.len() as f32 * LAUNCHER_SEGMENT_MIN_WIDTH
                };
                let sort_width = if layout.phone {
                    0.0
                } else {
                    LAUNCHER_SORT_WIDTH
                };
                let control_count = 1 + usize::from(!layout.compact) + usize::from(!layout.phone);
                let gaps = (control_count.saturating_sub(1) as f32) * 8.0;
                let search_width = (ui.available_width() - filter_width - sort_width - gaps)
                    .clamp(1.0, LAUNCHER_SEARCH_MAX_WIDTH);
                let search = ui.add_sized(
                    [search_width, t.metrics.ctl_h],
                    egui::TextEdit::singleline(&mut app.state.workbench.project_launcher_query)
                        .id(project_search_id())
                        .hint_text("Project, path, owner, tag…")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .margin(Margin {
                            left: 29,
                            right: 8,
                            top: 0,
                            bottom: 0,
                        })
                        .vertical_align(Align::Center),
                );
                let icon_rect = Rect::from_center_size(
                    egui::pos2(search.rect.left() + 13.5, search.rect.center().y),
                    Vec2::splat(13.0),
                );
                WorkbenchIcon::Search.paint(ui.painter(), icon_rect, t.color.text_faint);
                search.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::TextEdit,
                        ui.is_enabled(),
                        "Project, path, owner, tag",
                    )
                });
                if std::mem::take(&mut app.state.workbench.focus_project_launcher_search) {
                    search.request_focus();
                }

                if !layout.compact {
                    launcher_filter_segments(ui, &mut app.state.workbench.project_launcher_filter);
                }

                if !layout.phone {
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        let selected = app.state.workbench.project_launcher_sort.label();
                        let options = vec![
                            "Last opened".to_owned(),
                            "Name".to_owned(),
                            "Owner".to_owned(),
                        ];
                        if let Some(index) = select(
                            ui,
                            "project_launcher_sort",
                            "Project sort",
                            selected,
                            &options,
                            LAUNCHER_SORT_WIDTH,
                        ) {
                            app.state.workbench.project_launcher_sort = match index {
                                0 => ProjectLauncherSort::LastOpened,
                                1 => ProjectLauncherSort::Name,
                                _ => ProjectLauncherSort::Owner,
                            };
                        }
                    });
                }
            });
        });
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border),
    );
}

fn launcher_filter_segments(ui: &mut Ui, selected: &mut ProjectLauncherFilter) {
    let t = Tokens::get(ui.ctx());
    let options = ProjectLauncherFilter::ALL;
    let width = options.len() as f32 * LAUNCHER_SEGMENT_MIN_WIDTH;
    let (rect, group_response) =
        ui.allocate_exact_size(vec2(width, t.metrics.ctl_h), Sense::hover());
    group_response
        .widget_info(|| WidgetInfo::labeled(WidgetType::RadioGroup, true, "Project filter"));
    ui.ctx().accesskit_node_builder(group_response.id, |node| {
        node.set_role(egui::accesskit::Role::RadioGroup);
        node.set_label("Project filter");
    });
    ui.painter().rect(
        rect,
        t.radius,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let cell_width = rect.width() / options.len() as f32;
    for (index, option) in options.iter().copied().enumerate() {
        let cell = Rect::from_min_max(
            egui::pos2(rect.left() + index as f32 * cell_width, rect.top()),
            egui::pos2(rect.left() + (index + 1) as f32 * cell_width, rect.bottom()),
        );
        let mut response = ui.interact(
            cell,
            ui.make_persistent_id(("project-launcher-filter", option.label())),
            Sense::click(),
        );
        let active = *selected == option;
        response.widget_info(|| {
            WidgetInfo::selected(WidgetType::RadioButton, true, active, option.label())
        });
        if index > 0 {
            ui.painter().vline(
                cell.left(),
                cell.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        let fill = if active {
            t.color.bg_active
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            Color32::TRANSPARENT
        };
        if fill != Color32::TRANSPARENT {
            ui.painter().rect_filled(cell.shrink(1.0), 0.0, fill);
        }
        if active {
            ui.painter().hline(
                (cell.left() + 1.0)..=(cell.right() - 1.0),
                cell.bottom() - 1.0,
                Stroke::new(2.0, t.color.accent),
            );
        }
        ui.painter().text(
            cell.center(),
            Align2::CENTER_CENTER,
            option.label(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if active || response.hovered() {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        theme::paint_focus_ring(ui, &response, cell);
        let previous = response.has_focus()
            && ui.input_mut(|input| input.consume_key(Modifiers::NONE, egui::Key::ArrowLeft));
        let next = response.has_focus()
            && ui.input_mut(|input| input.consume_key(Modifiers::NONE, egui::Key::ArrowRight));
        let destination = if previous {
            index.checked_sub(1)
        } else if next {
            (index + 1 < options.len()).then_some(index + 1)
        } else {
            None
        };
        if let Some(destination) = destination {
            *selected = options[destination];
            let destination_id =
                ui.make_persistent_id(("project-launcher-filter", options[destination].label()));
            ui.memory_mut(|memory| {
                memory.request_focus(destination_id);
            });
        } else if response_activated(ui, &response) && !active {
            *selected = option;
            response.mark_changed();
        }
    }
}

fn project_entries(app: &RSpiceApp) -> Vec<ProjectEntry> {
    let query = app
        .state
        .workbench
        .project_launcher_query
        .trim()
        .to_lowercase();
    let current_path = app.state.workspace.project.path.as_deref();
    let current_name = app.state.workspace.project.display_name();
    let filter = app.state.workbench.project_launcher_filter;
    let mut entries = app
        .state
        .recent_files
        .iter()
        .filter(|recent| recent.kind == RecentKind::Project)
        .filter_map(|recent| {
            let current = current_path.is_some_and(|path| path == recent.path);
            let name = if current {
                current_name.to_owned()
            } else {
                project_name_from_path(&recent.path)
            };
            let path_text = recent.path.display().to_string();
            let shared = path_is_shared(&recent.path);
            let matches_filter = project_matches_filter(filter, recent.pinned, shared);
            let searchable_metadata = format!(
                "{} {} {}",
                recent.owner.as_deref().unwrap_or_default(),
                recent.tags.join(" "),
                if shared { "shared" } else { "local" }
            )
            .to_lowercase();
            let matches = query.is_empty()
                || name.to_lowercase().contains(&query)
                || path_text.to_lowercase().contains(&query)
                || searchable_metadata.contains(&query);
            (matches_filter && matches).then(|| ProjectEntry {
                recent: recent.clone(),
                name,
                path_text,
                owner: recent.owner.clone(),
                opened_at_unix_ms: recent.opened_at_unix_ms,
                group: project_group(recent.pinned, shared),
                shared,
                available: project_path_available(&recent.path),
                current,
            })
        })
        .collect::<Vec<_>>();

    match app.state.workbench.project_launcher_sort {
        ProjectLauncherSort::LastOpened => {}
        ProjectLauncherSort::Name => entries.sort_by(|left, right| {
            left.name
                .to_lowercase()
                .cmp(&right.name.to_lowercase())
                .then_with(|| left.path_text.cmp(&right.path_text))
        }),
        ProjectLauncherSort::Owner => entries.sort_by(|left, right| {
            match (&left.owner, &right.owner) {
                (Some(left), Some(right)) => left.to_lowercase().cmp(&right.to_lowercase()),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
            .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
            .then_with(|| left.path_text.cmp(&right.path_text))
        }),
    }
    entries
}

fn project_row(ui: &mut Ui, entry: &ProjectEntry, layout: LauncherLayout) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let enabled = entry.available && ui.is_enabled();
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), LAUNCHER_ROW_MIN_HEIGHT),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    let fill = if entry.current || response.hovered() {
        t.color.bg_hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, 0.0, fill);
    if entry.current {
        ui.painter().rect_filled(
            Rect::from_min_size(rect.min, vec2(2.0, rect.height())),
            0.0,
            t.color.accent,
        );
    }
    let horizontal_padding = if layout.phone { 9.0 } else { 13.0 };
    let inner = rect.shrink2(vec2(horizontal_padding, 5.0));
    let mut row = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(inner)
            .layout(egui::Layout::left_to_right(Align::Center)),
    );
    row.spacing_mut().item_spacing.x = 9.0;
    let (icon_column, _) = row.allocate_exact_size(vec2(31.0, 37.0), Sense::hover());
    let icon_rect = Rect::from_center_size(icon_column.center(), Vec2::splat(27.0));
    let unavailable_color = if cfg!(target_arch = "wasm32") {
        t.color.warn
    } else {
        t.color.err
    };
    row.painter().rect_filled(
        icon_rect,
        0.0,
        if entry.available {
            t.color.accent_dim
        } else {
            unavailable_color.gamma_multiply(0.12)
        },
    );
    WorkbenchIcon::Project.paint(
        row.painter(),
        icon_rect.shrink(5.0),
        if entry.available {
            t.color.accent
        } else {
            unavailable_color
        },
    );
    let trailing_width = if layout.compact { 62.0 } else { 213.0 };
    let content_width = (row.available_width() - trailing_width - 9.0).max(1.0);
    row.allocate_ui_with_layout(
        vec2(content_width, 37.0),
        egui::Layout::top_down(Align::Min),
        |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.add(
                egui::Label::new(
                    egui::RichText::new(&entry.name)
                        .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                        .color(t.color.text),
                )
                .truncate(),
            );
            let detail = entry.owner.as_ref().map_or_else(
                || entry.path_text.clone(),
                |owner| format!("{} · {owner}", entry.path_text),
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new(detail)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .truncate(),
            );
        },
    );
    if !layout.compact {
        row.add_sized(
            [125.0, 37.0],
            egui::Label::new(
                egui::RichText::new(recent_age(entry.opened_at_unix_ms))
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            ),
        );
    }
    let (state, color) = if !entry.available && cfg!(target_arch = "wasm32") {
        ("browse", t.color.warn)
    } else if !entry.available {
        ("missing", t.color.err)
    } else if entry.current {
        ("ready", t.color.ok)
    } else if entry.shared {
        ("shared", t.color.ok)
    } else {
        ("local", t.color.text_dim)
    };
    row.add_sized(
        [if layout.compact { 62.0 } else { 70.0 }, 37.0],
        egui::Label::new(
            egui::RichText::new(state)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(color),
        ),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            enabled,
            format!("Open project {}", entry.name),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(format!("{}; {state}", entry.path_text));
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    theme::paint_focus_ring(ui, &response, response.rect);
    if enabled {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response.on_hover_text(if cfg!(target_arch = "wasm32") {
            "Reopen this project through Browse so browser file permission can be verified"
        } else {
            "This project file is no longer available at its recorded path"
        })
    }
}

fn empty_project_list(ui: &mut Ui, query_empty: bool) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .inner_margin(Margin::symmetric(18, 28))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(if query_empty {
                        "No recent projects"
                    } else {
                        "No projects match this view"
                    })
                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                    .color(t.color.text),
                );
                ui.label(
                    egui::RichText::new(if query_empty {
                        "Browse to an existing project or create a new one."
                    } else {
                        "Change the search term. Project data has not been modified."
                    })
                    .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
        });
}

fn project_name_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("Unnamed project")
        .to_owned()
}

const fn project_matches_filter(filter: ProjectLauncherFilter, pinned: bool, shared: bool) -> bool {
    match filter {
        ProjectLauncherFilter::All => true,
        ProjectLauncherFilter::Recent => !pinned && !shared,
        ProjectLauncherFilter::Pinned => pinned,
        ProjectLauncherFilter::Shared => shared,
    }
}

const fn project_group(pinned: bool, shared: bool) -> ProjectGroup {
    if pinned {
        ProjectGroup::Pinned
    } else if shared {
        ProjectGroup::Shared
    } else {
        ProjectGroup::Recent
    }
}

fn path_is_shared(path: &Path) -> bool {
    let text = path.as_os_str().to_string_lossy();
    text.starts_with(r"\\") || text.starts_with("//")
}

fn recent_age(opened_at_unix_ms: u64) -> String {
    if opened_at_unix_ms == 0 {
        return "unknown".to_owned();
    }
    let now = crate::common::time_compat::unix_epoch()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX);
    recent_age_at(opened_at_unix_ms, now)
}

fn recent_age_at(opened_at_unix_ms: u64, now_unix_ms: u64) -> String {
    let elapsed_seconds = now_unix_ms.saturating_sub(opened_at_unix_ms) / 1_000;
    match elapsed_seconds {
        0..=59 => "just now".to_owned(),
        60..=3_599 => format!("{} min ago", elapsed_seconds / 60),
        3_600..=86_399 => format!("{} h ago", elapsed_seconds / 3_600),
        86_400..=172_799 => "Yesterday".to_owned(),
        _ => format!("{} d ago", elapsed_seconds / 86_400),
    }
}

fn project_search_id() -> Id {
    Id::new("workbench.project_launcher.search")
}

const fn page_index(page: ProjectLauncherPage) -> usize {
    match page {
        ProjectLauncherPage::Projects => 0,
        ProjectLauncherPage::Recovery => 1,
        ProjectLauncherPage::SafeMode => 2,
    }
}

fn response_activated(ui: &mut Ui, response: &Response) -> bool {
    response.clicked()
        || (response.has_focus()
            && ui.input_mut(|input| {
                input.consume_key(Modifiers::NONE, egui::Key::Enter)
                    || input.consume_key(Modifiers::NONE, egui::Key::Space)
            }))
}

fn begin_launcher_focus(ctx: &Context, state_id: Id) -> bool {
    let pass = ctx.cumulative_pass_nr();
    let current_focus = ctx.memory(|memory| memory.focused());
    ctx.data_mut(|data| {
        let previous = data.get_temp::<LauncherFocusState>(state_id);
        let continuing =
            previous.is_some_and(|state| pass <= state.last_seen_pass.saturating_add(1));
        let state = match previous {
            Some(mut state) if continuing => {
                state.last_seen_pass = pass;
                state
            }
            _ => LauncherFocusState {
                prior_focus: current_focus,
                last_seen_pass: pass,
            },
        };
        data.insert_temp(state_id, state);
        !continuing
    })
}

fn focus_is_within_launcher(ctx: &Context, modal_layer: egui::LayerId) -> bool {
    let Some(focused) = ctx.memory(|memory| memory.focused()) else {
        return false;
    };
    let Some(response) = ctx.read_response(focused) else {
        return false;
    };
    response.layer_id == modal_layer
        || ctx.memory(|memory| memory.is_above_modal_layer(response.layer_id))
}

fn restore_launcher_focus(
    ctx: &Context,
    state_id: Id,
    modal_focus_id: Id,
    modal_layer: egui::LayerId,
) {
    let state = ctx.data_mut(|data| data.remove_temp::<LauncherFocusState>(state_id));
    let restorable = state.and_then(|state| state.prior_focus).filter(|prior| {
        *prior != modal_focus_id
            && ctx.read_response(*prior).is_some_and(|response| {
                response.layer_id != modal_layer
                    && response.enabled()
                    && response.sense.is_focusable()
            })
    });
    ctx.memory_mut(|memory| {
        if let Some(current) = memory.focused() {
            memory.surrender_focus(current);
        }
        if let Some(prior) = restorable {
            memory.request_focus(prior);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn project_path_available(_path: &Path) -> bool {
    // Filesystem availability is deliberately verified only by the explicit
    // open transaction. Probing local, mapped, removable, or UNC paths from
    // every paint pass can stall the UI; the open path reports failure and
    // removes a stale recent entry atomically.
    true
}

#[cfg(target_arch = "wasm32")]
fn project_path_available(_path: &Path) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_launcher_matches_the_mockup_surface_contract() {
        let viewport = Rect::from_min_size(egui::Pos2::ZERO, Vec2::new(1440.0, 900.0));
        let layout = LauncherLayout::resolve(viewport);

        assert!(!layout.edge_to_edge);
        assert_eq!(layout.surface.size(), Vec2::new(1180.0, 650.0));
        assert_eq!(layout.surface.center(), viewport.center());
        assert_eq!(LAUNCHER_HEADER_HEIGHT, 58.0);
        assert_eq!(LAUNCHER_STATUS_HEIGHT, 30.0);
        assert_eq!(LAUNCHER_NAV_WIDTH, 184.0);
    }

    #[test]
    fn constrained_desktop_launcher_preserves_the_exact_viewport_gutter() {
        let viewport = Rect::from_min_size(egui::Pos2::ZERO, Vec2::new(900.0, 700.0));
        let layout = LauncherLayout::resolve(viewport);

        assert!(!layout.edge_to_edge);
        assert_eq!(layout.surface.size(), Vec2::new(872.0, 650.0));
        assert_eq!(layout.surface.left(), 14.0);
        assert_eq!(layout.surface.right(), 886.0);
    }

    #[test]
    fn phone_and_narrow_tablet_launcher_are_edge_to_edge() {
        for size in [Vec2::new(390.0, 844.0), Vec2::new(760.0, 900.0)] {
            let viewport = Rect::from_min_size(egui::Pos2::ZERO, size);
            let layout = LauncherLayout::resolve(viewport);
            assert!(layout.edge_to_edge);
            assert_eq!(layout.surface, viewport);
            assert!(layout.compact);
            assert_eq!(layout.header_height, LAUNCHER_COMPACT_HEADER_HEIGHT);
            assert_eq!(layout.status_height, LAUNCHER_COMPACT_STATUS_HEIGHT);
        }
    }

    #[test]
    fn responsive_breakpoints_match_the_mockup_contract() {
        let compact = LauncherLayout::resolve(Rect::from_min_size(
            egui::Pos2::ZERO,
            Vec2::new(760.0, 900.0),
        ));
        let desktop = LauncherLayout::resolve(Rect::from_min_size(
            egui::Pos2::ZERO,
            Vec2::new(761.0, 900.0),
        ));
        let phone = LauncherLayout::resolve(Rect::from_min_size(
            egui::Pos2::ZERO,
            Vec2::new(460.0, 800.0),
        ));
        let narrow_tablet = LauncherLayout::resolve(Rect::from_min_size(
            egui::Pos2::ZERO,
            Vec2::new(461.0, 800.0),
        ));

        assert!(compact.compact);
        assert!(!desktop.compact);
        assert!(phone.phone);
        assert!(!narrow_tablet.phone);
        assert_eq!(desktop.header_height, LAUNCHER_HEADER_HEIGHT);
        assert_eq!(desktop.status_height, LAUNCHER_STATUS_HEIGHT);
    }

    #[test]
    fn project_names_come_from_the_file_name_without_extension() {
        assert_eq!(
            project_name_from_path(Path::new("C:/Engineering/Precision AFE.rspiceproj")),
            "Precision AFE"
        );
    }

    #[test]
    fn project_names_have_a_nonempty_fallback() {
        assert_eq!(project_name_from_path(Path::new("/")), "Unnamed project");
    }

    #[test]
    fn shared_project_paths_cover_windows_and_portable_unc_forms() {
        assert!(path_is_shared(Path::new(
            r"\\lab-server\projects\afe.rspiceproj"
        )));
        assert!(path_is_shared(Path::new(
            "//lab-server/projects/afe.rspiceproj"
        )));
        assert!(!path_is_shared(Path::new("C:/Engineering/afe.rspiceproj")));
    }

    #[test]
    fn recent_age_uses_stable_human_scale_boundaries() {
        const NOW: u64 = 200_000_000;
        assert_eq!(recent_age_at(NOW, NOW), "just now");
        assert_eq!(recent_age_at(NOW - 60_000, NOW), "1 min ago");
        assert_eq!(recent_age_at(NOW - 3_600_000, NOW), "1 h ago");
        assert_eq!(recent_age_at(NOW - 86_400_000, NOW), "Yesterday");
        assert_eq!(recent_age_at(NOW - 172_800_000, NOW), "2 d ago");
        assert_eq!(recent_age_at(NOW + 60_000, NOW), "just now");
    }

    #[test]
    fn launcher_filter_and_sort_labels_are_complete_and_stable() {
        assert_eq!(
            ProjectLauncherFilter::ALL.map(ProjectLauncherFilter::label),
            ["All", "Recent", "Pinned", "Shared"]
        );
        assert_eq!(ProjectLauncherSort::Owner.label(), "Owner");
    }

    #[test]
    fn launcher_filters_and_groups_preserve_overlapping_durable_metadata() {
        assert!(project_matches_filter(
            ProjectLauncherFilter::All,
            false,
            false
        ));
        assert!(project_matches_filter(
            ProjectLauncherFilter::Recent,
            false,
            false
        ));
        assert!(!project_matches_filter(
            ProjectLauncherFilter::Recent,
            true,
            false
        ));
        assert!(project_matches_filter(
            ProjectLauncherFilter::Pinned,
            true,
            true
        ));
        assert!(project_matches_filter(
            ProjectLauncherFilter::Shared,
            true,
            true
        ));
        assert_eq!(project_group(true, true), ProjectGroup::Pinned);
        assert_eq!(project_group(false, true), ProjectGroup::Shared);
        assert_eq!(project_group(false, false), ProjectGroup::Recent);
    }
}
