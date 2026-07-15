//! Overlay drawers for tablet and phone layouts.

use egui::{Area, Context, FocusDirection, Frame, Id, Order, Sense, UiKind, Vec2};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Mode;
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::{Drawer, Workspace};

const MOBILE_NAV_HEADER_HEIGHT: f32 = 57.0;
const MOBILE_NAV_MAX_WIDTH: f32 = 700.0;
const MOBILE_NAV_GUTTER: f32 = 9.0;
const MOBILE_NAV_BREAKPOINT: f32 = 560.0;
const MOBILE_NAV_GRID_PADDING: f32 = 12.0;
const MOBILE_NAV_GRID_GAP: f32 = 8.0;
const MOBILE_NAV_GROUP_HEIGHT: f32 = 23.0;
const MOBILE_NAV_CARD_HEIGHT: f32 = 72.0;

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let Some(drawer) = app.state.workbench.drawer else {
        clear_side_drawer_focus_session(ctx);
        return;
    };
    if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
        app.state.workbench.close_drawer();
        return_drawer_focus(ctx, drawer);
        return;
    }
    let screen = ctx.content_rect();
    let top = layout.title_bar_height + layout.toolbar_height;
    let bottom = layout.status_bar_height
        + if layout.show_phone_navigation {
            layout.phone_navigation_height
        } else {
            0.0
        };
    let available_height = (screen.height() - top - bottom).max(180.0);
    let workbench_rect = egui::Rect::from_min_max(
        egui::pos2(screen.left(), screen.top() + top),
        egui::pos2(screen.right(), screen.bottom() - bottom),
    );
    let t = Tokens::get(ctx);

    let scrim_area = Area::new(Id::new("workbench.drawer.scrim"))
        .kind(UiKind::Modal)
        .sense(Sense::hover())
        .order(Order::Foreground)
        .fixed_pos(workbench_rect.min);
    // The scrim is deliberately the modal floor: its pointer target remains
    // interactive, while the navigator/inspector Tooltip layer and any popup
    // spawned from it remain above the floor. Background widgets can neither
    // receive pointer interaction nor enter egui's Tab focus chain.
    ctx.memory_mut(|memory| memory.set_modal_layer(scrim_area.layer()));
    let scrim = scrim_area
        .show(ctx, |ui| {
            // accessibility-pointer-shim: the drawer scrim dismisses on a
            // pointer press but is deliberately absent from keyboard/AT order.
            let (rect, response) = ui.allocate_exact_size(
                workbench_rect.size(),
                Sense::click().difference(Sense::focusable_noninteractive()),
            );
            ui.painter().rect_filled(rect, 0.0, drawer_scrim_color(&t));
            response
        })
        .inner;
    if scrim.clicked() {
        app.state.workbench.close_drawer();
        return_drawer_focus(ctx, drawer);
        return;
    }

    match drawer {
        Drawer::Navigator => {
            let opened_this_pass = begin_side_drawer_focus(ctx, Drawer::Navigator);
            if opened_this_pass {
                app.state.workbench.focus_navigator_search = true;
                ctx.memory_mut(|memory| memory.move_focus(FocusDirection::None));
            }
            let width = if layout.compact_shell {
                340.0_f32.min((screen.width() - 42.0).max(220.0))
            } else {
                360.0_f32.min((screen.width() - 54.0).max(220.0))
            };
            let area = Area::new(Id::new("workbench.drawer.navigator"))
                .kind(UiKind::Modal)
                .sense(Sense::hover())
                .order(Order::Tooltip)
                .fixed_pos(egui::pos2(screen.left(), screen.top() + top));
            let drawer_layer = area.layer();
            let mut close_id = None;
            let shown = area.show(ctx, |ui| {
                drawer_frame(&t).show(ui, |ui| {
                    ui.set_width(width);
                    ui.set_height(available_height);
                    super::navigator::show(ui, app);
                    let close = drawer_close_button(ui, "navigator", layout);
                    close_id = Some(close.id);
                    let _ = take_drawer_focus_pending(ctx, Drawer::Navigator);
                    if close.clicked() {
                        app.state.workbench.close_drawer();
                        return_drawer_focus(ctx, Drawer::Navigator);
                    }
                });
            });
            if app.state.workbench.drawer == Some(Drawer::Navigator) {
                reclaim_side_drawer_focus(ctx, drawer_layer, close_id);
            }
            ctx.accesskit_node_builder(shown.response.id, |node| {
                node.set_role(egui::accesskit::Role::Complementary);
                node.set_label("Workspace navigator");
                node.set_modal();
            });
        }
        Drawer::Inspector => {
            let opened_this_pass = begin_side_drawer_focus(ctx, Drawer::Inspector);
            if opened_this_pass {
                ctx.memory_mut(|memory| memory.move_focus(FocusDirection::None));
            }
            let maximum: f32 = if layout.compact_shell { 340.0 } else { 330.0 };
            let gutter: f32 = if layout.compact_shell { 42.0 } else { 54.0 };
            let width = maximum.min((screen.width() - gutter).max(220.0));
            let area = Area::new(Id::new("workbench.drawer.inspector"))
                .kind(UiKind::Modal)
                .sense(Sense::hover())
                .order(Order::Tooltip)
                .fixed_pos(egui::pos2(screen.right() - width, screen.top() + top));
            let drawer_layer = area.layer();
            let mut close_id = None;
            let shown = area.show(ctx, |ui| {
                drawer_frame(&t).show(ui, |ui| {
                    ui.set_width(width);
                    ui.set_height(available_height);
                    super::inspector::show(ui, app);
                    let close = drawer_close_button(ui, "inspector", layout);
                    close_id = Some(close.id);
                    let pending = take_drawer_focus_pending(ctx, Drawer::Inspector);
                    if opened_this_pass || pending {
                        close.request_focus();
                    }
                    if close.clicked() {
                        app.state.workbench.close_drawer();
                        return_drawer_focus(ctx, Drawer::Inspector);
                    }
                });
            });
            if app.state.workbench.drawer == Some(Drawer::Inspector) {
                reclaim_side_drawer_focus(ctx, drawer_layer, close_id);
            }
            ctx.accesskit_node_builder(shown.response.id, |node| {
                node.set_role(egui::accesskit::Role::Complementary);
                node.set_label("Inspector");
                node.set_modal();
            });
        }
        Drawer::Workspaces => {
            mobile_navigation_dialog(ctx, app, screen, &t);
        }
    }
}

#[derive(Clone, Copy)]
struct SideDrawerFocusSession {
    drawer: Drawer,
    last_seen_pass: u64,
}

fn side_drawer_focus_session_id() -> Id {
    Id::new("workbench.side_drawer.focus_session")
}

fn begin_side_drawer_focus(ctx: &Context, drawer: Drawer) -> bool {
    let pass = ctx.cumulative_pass_nr();
    ctx.data_mut(|data| {
        let key = side_drawer_focus_session_id();
        let previous = data.get_temp::<SideDrawerFocusSession>(key);
        let continuing = previous.is_some_and(|state| {
            state.drawer == drawer && pass <= state.last_seen_pass.saturating_add(1)
        });
        data.insert_temp(
            key,
            SideDrawerFocusSession {
                drawer,
                last_seen_pass: pass,
            },
        );
        !continuing
    })
}

fn clear_side_drawer_focus_session(ctx: &Context) {
    ctx.data_mut(|data| data.remove::<SideDrawerFocusSession>(side_drawer_focus_session_id()));
}

fn reclaim_side_drawer_focus(ctx: &Context, drawer_layer: egui::LayerId, fallback: Option<Id>) {
    if focus_is_within_drawer(ctx, drawer_layer) {
        return;
    }
    if let Some(fallback) = fallback {
        ctx.memory_mut(|memory| memory.request_focus(fallback));
    }
}

fn focus_is_within_drawer(ctx: &Context, drawer_layer: egui::LayerId) -> bool {
    let Some(focused) = ctx.memory(|memory| memory.focused()) else {
        return false;
    };
    let Some(response) = ctx.read_response(focused) else {
        return false;
    };
    ctx.memory(|memory| {
        let mut layer = response.layer_id;
        loop {
            if layer == drawer_layer {
                return true;
            }
            let Some(parent) = memory.areas().parent_layer(layer) else {
                return false;
            };
            layer = parent;
        }
    })
}

#[derive(Clone, Copy)]
struct MobileNavigationAction {
    command: Command,
    icon: WorkbenchIcon,
    label: &'static str,
    detail: &'static str,
}

const WORKSPACE_ACTIONS: [MobileNavigationAction; 7] = [
    MobileNavigationAction {
        command: Command::OpenWorkspace(Workspace::Project),
        icon: WorkbenchIcon::Project,
        label: "Current project",
        detail: "Libraries, recovery and configuration",
    },
    MobileNavigationAction {
        command: Command::OpenWorkspace(Workspace::Design),
        icon: WorkbenchIcon::Design,
        label: "Design",
        detail: "Schematic and hierarchy",
    },
    MobileNavigationAction {
        command: Command::OpenWorkspace(Workspace::Simulate),
        icon: WorkbenchIcon::Simulate,
        label: "Simulation",
        detail: "Setup, runs and resources",
    },
    MobileNavigationAction {
        command: Command::OpenWorkspace(Workspace::Results),
        icon: WorkbenchIcon::Results,
        label: "Results",
        detail: "Plots, calculator and measurements",
    },
    MobileNavigationAction {
        command: Command::OpenWorkspace(Workspace::Verify),
        icon: WorkbenchIcon::Verify,
        label: "Verify",
        detail: "PVT, yield, reliability and regression",
    },
    MobileNavigationAction {
        command: Command::OpenWorkspace(Workspace::Models),
        icon: WorkbenchIcon::Models,
        label: "Model & libraries",
        detail: "Sources, sections, symbols and qualification",
    },
    MobileNavigationAction {
        command: Command::OpenWorkspace(Workspace::Netlist),
        icon: WorkbenchIcon::Netlist,
        label: "Code & automation",
        detail: "Netlists, Verilog-A and scripts",
    },
];

const SPECIALIST_ACTIONS: [MobileNavigationAction; 1] = [MobileNavigationAction {
    command: Command::FeatureAvailability,
    icon: WorkbenchIcon::More,
    label: "Specialist tool browser",
    detail: "AMS, RF, physical, SI/PI, models, automation and emerging domains",
}];

const UTILITY_ACTIONS: [MobileNavigationAction; 2] = [
    MobileNavigationAction {
        command: Command::OpenConsole,
        icon: WorkbenchIcon::Console,
        label: "Console",
        detail: "Diagnostics, measurements and task log",
    },
    MobileNavigationAction {
        command: Command::ProjectLauncher,
        icon: WorkbenchIcon::Project,
        label: "Project launcher",
        detail: "Recent, shared, recovered and template projects",
    },
];

const SETTINGS_ACTIONS: [MobileNavigationAction; 2] = [
    MobileNavigationAction {
        command: Command::Preferences,
        icon: WorkbenchIcon::Settings,
        label: "Preferences",
        detail: "Personal appearance, files and shortcuts",
    },
    MobileNavigationAction {
        command: Command::License,
        icon: WorkbenchIcon::Verify,
        label: "Organization & licensing",
        detail: "Identity, roles, managed policy and entitlements",
    },
];

fn mobile_navigation_natural_height(columns: usize) -> f32 {
    let card_rows = [
        WORKSPACE_ACTIONS.len(),
        SPECIALIST_ACTIONS.len(),
        UTILITY_ACTIONS.len(),
        SETTINGS_ACTIONS.len(),
    ]
    .into_iter()
    .map(|count| count.div_ceil(columns))
    .sum::<usize>();
    let rows = card_rows + 4;
    MOBILE_NAV_HEADER_HEIGHT
        + MOBILE_NAV_GRID_PADDING * 2.0
        + MOBILE_NAV_GROUP_HEIGHT * 4.0
        + MOBILE_NAV_CARD_HEIGHT * card_rows as f32
        + MOBILE_NAV_GRID_GAP * rows.saturating_sub(1) as f32
}

fn mobile_navigation_dialog(
    ctx: &Context,
    app: &mut RSpiceApp,
    screen: egui::Rect,
    tokens: &Tokens,
) {
    let width = MOBILE_NAV_MAX_WIDTH.min((screen.width() - MOBILE_NAV_GUTTER * 2.0).max(280.0));
    let columns = if width <= MOBILE_NAV_BREAKPOINT { 1 } else { 2 };
    let natural_height = mobile_navigation_natural_height(columns);
    let height = natural_height
        .min((screen.height() - MOBILE_NAV_GUTTER * 2.0).max(MOBILE_NAV_HEADER_HEIGHT));
    let position = egui::pos2(
        screen.center().x - width * 0.5,
        screen.center().y - height * 0.5,
    );
    let mut chosen = None;
    let mut close = false;
    let mut focus_ids = Vec::new();
    let mut first_card_id = None;
    let shown = Area::new(Id::new("workbench.drawer.workspaces"))
        .order(Order::Tooltip)
        .fixed_pos(position)
        .show(ctx, |ui| {
            Frame::new()
                .fill(tokens.color.bg_elevated)
                .stroke(egui::Stroke::new(1.0, tokens.color.border_strong))
                .corner_radius(8.0)
                .shadow(tokens.shadow())
                .show(ui, |ui| {
                    ui.set_min_size(Vec2::new(width, height));
                    ui.set_max_size(Vec2::new(width, height));
                    let (header, _) = ui.allocate_exact_size(
                        Vec2::new(width, MOBILE_NAV_HEADER_HEIGHT),
                        Sense::hover().difference(Sense::focusable_noninteractive()),
                    );
                    ui.painter().rect_filled(header, 0.0, tokens.color.bg_panel);
                    ui.painter().hline(
                        header.x_range(),
                        header.bottom(),
                        egui::Stroke::new(1.0, tokens.color.border),
                    );
                    ui.painter().text(
                        egui::pos2(header.left() + 15.0, header.top() + 17.0),
                        egui::Align2::LEFT_CENTER,
                        "RSPICE",
                        theme::mono(tokens::FS_0, FontWeight::Medium),
                        tokens.color.text_faint,
                    );
                    ui.painter().text(
                        egui::pos2(header.left() + 15.0, header.bottom() - 16.0),
                        egui::Align2::LEFT_CENTER,
                        "Workspaces & tools",
                        theme::sans(tokens::FS_3, FontWeight::SemiBold),
                        tokens.color.text,
                    );
                    let close_response = mobile_navigation_close(ui, header);
                    close = close_response.clicked();
                    focus_ids.push(close_response.id);

                    ui.allocate_ui_with_layout(
                        Vec2::new(width, height - MOBILE_NAV_HEADER_HEIGHT),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            egui::ScrollArea::vertical()
                                .id_salt("workbench.mobile_navigation.scroll")
                                .show(ui, |ui| {
                                    ui.set_width(width);
                                    ui.spacing_mut().item_spacing.y = MOBILE_NAV_GRID_GAP;
                                    ui.add_space(MOBILE_NAV_GRID_PADDING);
                                    let mut group_state = MobileNavigationGroupState {
                                        chosen: &mut chosen,
                                        focus_ids: &mut focus_ids,
                                        first_card_id: &mut first_card_id,
                                    };
                                    mobile_navigation_group(
                                        ui,
                                        app,
                                        "Workspaces",
                                        &WORKSPACE_ACTIONS,
                                        columns,
                                        &mut group_state,
                                    );
                                    mobile_navigation_group(
                                        ui,
                                        app,
                                        "Specialist engineering",
                                        &SPECIALIST_ACTIONS,
                                        columns,
                                        &mut group_state,
                                    );
                                    mobile_navigation_group(
                                        ui,
                                        app,
                                        "Utilities",
                                        &UTILITY_ACTIONS,
                                        columns,
                                        &mut group_state,
                                    );
                                    mobile_navigation_group(
                                        ui,
                                        app,
                                        "Settings",
                                        &SETTINGS_ACTIONS,
                                        columns,
                                        &mut group_state,
                                    );
                                    ui.add_space(MOBILE_NAV_GRID_PADDING);
                                });
                        },
                    );
                })
        });
    ctx.accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Dialog);
        node.set_label("Workspaces & tools");
        node.set_description(
            "Choose a workspace, specialist tool, utility, or preference surface.",
        );
        node.set_modal();
    });
    maintain_mobile_navigation_focus(ctx, &focus_ids, first_card_id);
    if close {
        app.state.workbench.close_drawer();
        return_drawer_focus(ctx, Drawer::Workspaces);
    } else if let Some(command) = chosen {
        app.state.workbench.close_drawer();
        release_current_focus(ctx);
        command.execute(app);
    }
}

fn mobile_navigation_close(ui: &mut egui::Ui, header: egui::Rect) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let rect = egui::Rect::from_min_size(
        egui::pos2(header.right() - 15.0 - 44.0, header.center().y - 44.0 * 0.5),
        Vec2::splat(44.0),
    );
    let response = ui.interact(
        rect,
        Id::new("workbench.mobile_navigation.close"),
        Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Close navigation",
        )
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 3.0, t.color.bg_hover);
    }
    WorkbenchIcon::Close.paint(
        ui.painter(),
        egui::Rect::from_center_size(rect.center(), Vec2::splat(17.0)),
        t.color.text_dim,
    );
    theme::paint_focus_ring(ui, &response, rect);
    response
}

struct MobileNavigationGroupState<'a> {
    chosen: &'a mut Option<Command>,
    focus_ids: &'a mut Vec<Id>,
    first_card_id: &'a mut Option<Id>,
}

fn mobile_navigation_group(
    ui: &mut egui::Ui,
    app: &RSpiceApp,
    label: &str,
    actions: &[MobileNavigationAction],
    columns: usize,
    state: &mut MobileNavigationGroupState<'_>,
) {
    let t = Tokens::get(ui.ctx());
    let (label_rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width() - 24.0, MOBILE_NAV_GROUP_HEIGHT),
        Sense::hover().difference(Sense::focusable_noninteractive()),
    );
    let label_rect = label_rect.translate(Vec2::new(12.0, 0.0));
    ui.painter().hline(
        label_rect.x_range(),
        label_rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        egui::pos2(label_rect.left() + 2.0, label_rect.bottom() - 8.0),
        egui::Align2::LEFT_CENTER,
        label.to_uppercase(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );

    for row in actions.chunks(columns) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = MOBILE_NAV_GRID_GAP;
            ui.add_space(MOBILE_NAV_GRID_PADDING);
            let available = ui.available_width() - MOBILE_NAV_GRID_PADDING;
            let card_width = (available - MOBILE_NAV_GRID_GAP * (columns.saturating_sub(1) as f32))
                / columns as f32;
            for action in row {
                let enabled = action.command.is_enabled(app);
                let response = ui
                    .add_enabled_ui(enabled, |ui| {
                        mobile_navigation_card(ui, *action, card_width)
                    })
                    .inner;
                if enabled {
                    state.focus_ids.push(response.id);
                    state.first_card_id.get_or_insert(response.id);
                }
                if response.clicked() {
                    *state.chosen = Some(action.command);
                }
            }
        });
    }
}

fn maintain_mobile_navigation_focus(ctx: &Context, focus_ids: &[Id], first_card_id: Option<Id>) {
    let Some(first_card_id) = first_card_id else {
        return;
    };
    let focused = ctx.memory(|memory| memory.focused());
    let backwards =
        ctx.input_mut(|input| input.consume_key(egui::Modifiers::SHIFT, egui::Key::Tab));
    let forwards = !backwards
        && ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Tab));
    let target = if backwards || forwards {
        let current = focused
            .and_then(|id| focus_ids.iter().position(|candidate| *candidate == id))
            .unwrap_or_else(|| {
                focus_ids
                    .iter()
                    .position(|candidate| *candidate == first_card_id)
                    .unwrap_or(0)
            });
        let next = if backwards {
            (current + focus_ids.len() - 1) % focus_ids.len()
        } else {
            (current + 1) % focus_ids.len()
        };
        Some(focus_ids[next])
    } else if focused.is_none_or(|id| !focus_ids.contains(&id)) {
        Some(first_card_id)
    } else {
        None
    };
    if let Some(target) = target {
        ctx.memory_mut(|memory| memory.request_focus(target));
    }
}

fn drawer_owner(drawer: Drawer) -> &'static str {
    match drawer {
        Drawer::Navigator => "navigator",
        Drawer::Inspector => "inspector",
        Drawer::Workspaces => "workspaces",
    }
}

fn take_drawer_focus_pending(ctx: &Context, drawer: Drawer) -> bool {
    let key = Id::new(("workbench.drawer.focus_pending", drawer_owner(drawer)));
    ctx.data_mut(|data| {
        let pending = data.get_temp::<bool>(key).unwrap_or(false);
        data.remove::<bool>(key);
        pending
    })
}

fn return_drawer_focus(ctx: &Context, drawer: Drawer) {
    let key = if drawer == Drawer::Workspaces {
        Id::new("workbench.mobile_navigation.invoker")
    } else {
        Id::new(("workbench.drawer.invoker", drawer_owner(drawer)))
    };
    if let Some(invoker) = ctx.data(|data| data.get_temp::<Id>(key)) {
        ctx.memory_mut(|memory| memory.request_focus(invoker));
    } else {
        release_current_focus(ctx);
    }
}

fn release_current_focus(ctx: &Context) {
    if let Some(focused) = ctx.memory(|memory| memory.focused()) {
        ctx.memory_mut(|memory| memory.surrender_focus(focused));
    }
}

fn mobile_navigation_card(
    ui: &mut egui::Ui,
    action: MobileNavigationAction,
    width: f32,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let enabled = ui.is_enabled();
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(width, MOBILE_NAV_CARD_HEIGHT), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), action.label)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(action.detail);
    });
    ui.painter().rect(
        rect,
        t.radius,
        if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        },
        egui::Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    action.icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 22.0, rect.center().y),
            Vec2::splat(24.0),
        ),
        if enabled {
            t.color.accent
        } else {
            t.color.text_faint
        },
    );
    let text_clip = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 53.0, rect.top() + 8.0),
        egui::pos2(rect.right() - 10.0, rect.bottom() - 8.0),
    );
    let text_painter = ui.painter().with_clip_rect(text_clip);
    text_painter.text(
        egui::pos2(rect.left() + 53.0, rect.top() + 25.0),
        egui::Align2::LEFT_CENTER,
        action.label,
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        if enabled {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    text_painter.text(
        egui::pos2(rect.left() + 53.0, rect.bottom() - 22.0),
        egui::Align2::LEFT_CENTER,
        action.detail,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if enabled {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response
}

fn drawer_frame(tokens: &Tokens) -> Frame {
    Frame::new()
        .fill(tokens.color.bg_panel)
        .stroke(egui::Stroke::new(1.0, tokens.color.border_strong))
        .shadow(tokens.shadow())
}

fn drawer_scrim_color(tokens: &Tokens) -> egui::Color32 {
    if tokens.mode == Mode::Light {
        egui::Color32::from_rgba_unmultiplied(41, 46, 50, 97)
    } else {
        egui::Color32::from_rgba_unmultiplied(2, 6, 8, 158)
    }
}

fn drawer_close_button(
    ui: &mut egui::Ui,
    owner: &'static str,
    layout: LayoutSpec,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let panel = ui.max_rect();
    let (size, inset) = drawer_close_geometry(layout);
    let rect = egui::Rect::from_min_size(
        egui::pos2(panel.right() - inset - size, panel.top() + inset),
        Vec2::splat(size),
    );
    let response = ui.interact(
        rect,
        Id::new(("workbench.drawer.close", owner)),
        Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!("Close {owner}"),
        )
    });
    ui.painter().rect_filled(
        rect,
        3.0,
        if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_panel
        },
    );
    WorkbenchIcon::Close.paint(
        ui.painter(),
        egui::Rect::from_center_size(rect.center(), Vec2::splat(17.0)),
        if response.hovered() {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response
}

fn drawer_close_geometry(layout: LayoutSpec) -> (f32, f32) {
    let size = if layout.compact_shell || layout.coarse_pointer {
        44.0
    } else {
        34.0
    };
    let inset = if layout.coarse_pointer { 0.0 } else { 5.0 };
    (size, inset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::state::WorkbenchState;

    #[derive(Clone, Copy)]
    struct ModalFocusFixtureIds {
        background: Id,
        first: Id,
        disabled: Id,
        last: Id,
    }

    fn focus_test_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            events,
            ..Default::default()
        }
    }

    fn focus_test_key(shift: bool) -> egui::Event {
        egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: Some(egui::Key::Tab),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers {
                shift,
                ..egui::Modifiers::NONE
            },
        }
    }

    fn run_modal_focus_fixture(
        ctx: &Context,
        input: egui::RawInput,
        request_first: bool,
    ) -> ModalFocusFixtureIds {
        let mut rendered = None;
        let _output = ctx.run(input, |ctx| {
            let background = egui::CentralPanel::default()
                .show(ctx, |ui| ui.button("Background action"))
                .inner;

            let floor = Area::new(Id::new("drawer-focus-test.floor"))
                .kind(UiKind::Modal)
                .sense(Sense::hover())
                .order(Order::Foreground)
                .fixed_pos(egui::Pos2::ZERO);
            ctx.memory_mut(|memory| memory.set_modal_layer(floor.layer()));
            floor.show(ctx, |ui| {
                ui.allocate_exact_size(egui::vec2(800.0, 600.0), Sense::hover());
            });

            let controls = Area::new(Id::new("drawer-focus-test.controls"))
                .kind(UiKind::Modal)
                .sense(Sense::hover())
                .order(Order::Tooltip)
                .fixed_pos(egui::pos2(20.0, 20.0))
                .show(ctx, |ui| {
                    let first = ui.button("First action");
                    let disabled = ui.add_enabled(false, egui::Button::new("Unavailable action"));
                    let last = ui.button("Last action");
                    if request_first {
                        first.request_focus();
                    }
                    (first.id, disabled.id, last.id)
                })
                .inner;
            rendered = Some(ModalFocusFixtureIds {
                background: background.id,
                first: controls.0,
                disabled: controls.1,
                last: controls.2,
            });
        });
        rendered.expect("focus fixture must render")
    }

    #[test]
    fn drawer_scrims_match_the_mockup_dark_and_light_tokens() {
        let mut dark = Tokens::default();
        dark.mode = Mode::Dark;
        assert_eq!(
            drawer_scrim_color(&dark),
            egui::Color32::from_rgba_unmultiplied(2, 6, 8, 158)
        );

        let mut light = dark.clone();
        light.mode = Mode::Light;
        assert_eq!(
            drawer_scrim_color(&light),
            egui::Color32::from_rgba_unmultiplied(41, 46, 50, 97)
        );
    }

    #[test]
    fn mobile_navigation_grid_height_tracks_its_real_rows() {
        assert_eq!(mobile_navigation_natural_height(2), 757.0);
        assert_eq!(mobile_navigation_natural_height(1), 1_157.0);
        assert_eq!(MOBILE_NAV_MAX_WIDTH, 700.0);
        assert_eq!(MOBILE_NAV_GUTTER, 9.0);
        assert_eq!(MOBILE_NAV_CARD_HEIGHT, 72.0);
        assert_eq!(MOBILE_NAV_GRID_PADDING, 12.0);
        assert_eq!(MOBILE_NAV_GRID_GAP, 8.0);
    }

    #[test]
    fn drawer_close_targets_follow_pointer_and_breakpoint_cascade() {
        let compact = LayoutSpec::resolve(390.0, 844.0, &WorkbenchState::default());
        assert_eq!(drawer_close_geometry(compact), (44.0, 5.0));

        let intermediate = LayoutSpec::resolve(834.0, 1_112.0, &WorkbenchState::default());
        assert_eq!(drawer_close_geometry(intermediate), (34.0, 5.0));

        let coarse =
            LayoutSpec::resolve_with_pointer(834.0, 1_112.0, true, &WorkbenchState::default());
        assert_eq!(drawer_close_geometry(coarse), (44.0, 0.0));
    }

    #[test]
    fn side_drawer_modal_focus_skips_unavailable_controls_and_wraps_both_directions() {
        let ctx = Context::default();
        let ids = run_modal_focus_fixture(&ctx, focus_test_input(Vec::new()), true);
        let _ = run_modal_focus_fixture(&ctx, focus_test_input(Vec::new()), false);
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(ids.first));

        let current =
            run_modal_focus_fixture(&ctx, focus_test_input(vec![focus_test_key(false)]), false);
        assert_eq!(current.first, ids.first);
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(ids.last));
        assert_ne!(ctx.memory(|memory| memory.focused()), Some(ids.disabled));

        let _ = run_modal_focus_fixture(&ctx, focus_test_input(vec![focus_test_key(false)]), false);
        let _ = run_modal_focus_fixture(&ctx, focus_test_input(Vec::new()), false);
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(ids.first));

        let _ = run_modal_focus_fixture(&ctx, focus_test_input(vec![focus_test_key(true)]), false);
        let _ = run_modal_focus_fixture(&ctx, focus_test_input(Vec::new()), false);
        let focused = ctx.memory(|memory| memory.focused());
        assert_eq!(focused, Some(ids.last));
        assert_ne!(focused, Some(ids.background));
        assert_ne!(focused, Some(ids.disabled));
    }
}
