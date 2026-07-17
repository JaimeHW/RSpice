//! Application title bar and complete implemented menu taxonomy.

use egui::{
    Align, Align2, Context, Frame, Id, Key, Layout, Modifiers, PointerButton, Popup, Rect,
    Response, Sense, TopBottomPanel, Ui, Vec2,
};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::{Command, CommandAvailability};
use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::{ModelsPage, VerificationPage, Workspace};

const DESCEND_MENU_LABEL: &str = "Descend into selected instance…";
#[cfg(test)]
const COMMAND_REFERENCE_MENU_LABEL: &str = "Command reference";
const MENU_OUTER_WIDTH: f32 = 244.0;
const MENU_MARGIN: f32 = 5.0;
const MENU_BORDER_WIDTH: f32 = 1.0;
const MENU_CONTENT_WIDTH: f32 = MENU_OUTER_WIDTH - 2.0 * (MENU_MARGIN + MENU_BORDER_WIDTH);
const MENU_ROW_HEIGHT: f32 = 27.0;
const MENU_TOUCH_ROW_HEIGHT: f32 = 44.0;
const MENU_POPUP_GAP: f32 = 2.0;
const SEARCH_KEYCAP_HEIGHT: f32 = 18.0;
const SEARCH_KEYCAP_MIN_WIDTH: f32 = 19.0;
const SEARCH_KEYCAP_INLINE_PADDING: f32 = 4.0;
const SEARCH_KEYCAP_BORDER_WIDTH: f32 = 1.0;
const MENU_RENDER_STATE_ID: &str = "workbench.title_menu.render_state";
const MENU_RETURN_FOCUS_ID: &str = "workbench.title_menu.return_focus";
const MENU_TYPEAHEAD_ID: &str = "workbench.title_menu.typeahead";

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let viewport_width = ctx.content_rect().width();
    let menu_projection = MenuProjection::for_layout(viewport_width, layout.compact_shell);
    let large_targets = viewport_width <= 820.0 || layout.coarse_pointer;
    TopBottomPanel::top("workbench.title_bar")
        .exact_height(layout.title_bar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let account_initials = account_initials(app);
            // Keep both one-pixel rules fully inside the panel clip. A stroke
            // centered exactly on `rect.top()` loses half its coverage in the
            // browser renderer and can disappear at common display scales.
            let (separator_top, separator_bottom) = title_bar_separator_positions(rect);
            ui.painter().hline(
                rect.x_range(),
                separator_top,
                egui::Stroke::new(1.0, t.color.border_strong),
            );
            ui.painter().hline(
                rect.x_range(),
                separator_bottom,
                egui::Stroke::new(1.0, t.color.border),
            );
            ui.ctx().accesskit_node_builder(ui.id(), |node| {
                node.set_role(egui::accesskit::Role::Banner);
                node.set_label("Application title bar");
            });
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                if viewport_width <= 820.0 {
                    ui.add_space(5.0);
                }
                brand(
                    ui,
                    app,
                    menu_projection.shows_title_context(),
                    layout.title_bar_height,
                    large_targets,
                    layout.compact_shell && viewport_width > 820.0,
                );
                menus(
                    ui,
                    app,
                    menu_projection,
                    large_targets,
                    layout.title_bar_height,
                );
                let context_left = ui.available_rect_before_wrap().left();
                let mut context_right = ui.max_rect().right();

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.add_space(if viewport_width <= 820.0 { 5.0 } else { 7.0 });
                    if viewport_width > 560.0 {
                        if account_action(ui, &account_initials, large_targets) {
                            Command::AccountOrganization.execute(app);
                        }
                        if notification_action(
                            ui,
                            app.state.ui.toasts.unread_count(),
                            large_targets,
                        ) {
                            app.state.workbench.notification_center_open = true;
                        }
                        if icon_action(
                            ui,
                            WorkbenchIcon::Settings,
                            "Open preferences",
                            large_targets,
                        ) {
                            Command::Preferences.execute(app);
                        }
                    }
                    if search_button(ui, app, viewport_width, large_targets) {
                        Command::CommandPalette.execute(app);
                    }
                    context_right = ui.available_rect_before_wrap().right();
                });
                paint_title_context(
                    ui,
                    app,
                    egui::Rect::from_x_y_ranges(
                        context_left..=context_right,
                        ui.max_rect().y_range(),
                    ),
                    !menu_projection.shows_title_context(),
                    title_context_is_left_aligned(
                        menu_projection,
                        viewport_width,
                        layout.compact_shell,
                    ),
                );
            });
        });
}

fn title_bar_separator_positions(rect: egui::Rect) -> (f32, f32) {
    (
        (rect.top() + 0.5).min(rect.bottom()),
        (rect.bottom() - 0.5).max(rect.top()),
    )
}

fn title_context_is_left_aligned(
    projection: MenuProjection,
    viewport_width: f32,
    compact_shell: bool,
) -> bool {
    projection == MenuProjection::ThroughSimulate || (compact_shell && viewport_width > 820.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ApplicationMenu {
    File,
    Edit,
    View,
    Design,
    Simulate,
    Results,
    Verify,
    Models,
    Automation,
    Window,
    Help,
}

impl ApplicationMenu {
    const fn label(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Design => "Design",
            Self::Simulate => "Simulate",
            Self::Results => "Results",
            Self::Verify => "Verify",
            Self::Models => "Models",
            Self::Automation => "Automation",
            Self::Window => "Window",
            Self::Help => "Help",
        }
    }

    fn show(self, ui: &mut Ui, app: &mut RSpiceApp) {
        match self {
            Self::File => file_menu(ui, app),
            Self::Edit => edit_menu(ui, app),
            Self::View => view_menu(ui, app),
            Self::Design => design_menu(ui, app),
            Self::Simulate => simulate_menu(ui, app),
            Self::Results => results_menu(ui, app),
            Self::Verify => verify_menu(ui, app),
            Self::Models => models_menu(ui, app),
            Self::Automation => automation_menu(ui, app),
            Self::Window => window_menu(ui, app),
            Self::Help => help_menu(ui, app),
        }
    }
}

const ALL_MENUS: [ApplicationMenu; 11] = [
    ApplicationMenu::File,
    ApplicationMenu::Edit,
    ApplicationMenu::View,
    ApplicationMenu::Design,
    ApplicationMenu::Simulate,
    ApplicationMenu::Results,
    ApplicationMenu::Verify,
    ApplicationMenu::Models,
    ApplicationMenu::Automation,
    ApplicationMenu::Window,
    ApplicationMenu::Help,
];

const THROUGH_MODELS_MENUS: [ApplicationMenu; 8] = [
    ApplicationMenu::File,
    ApplicationMenu::Edit,
    ApplicationMenu::View,
    ApplicationMenu::Design,
    ApplicationMenu::Simulate,
    ApplicationMenu::Results,
    ApplicationMenu::Verify,
    ApplicationMenu::Models,
];

const THROUGH_SIMULATE_MENUS: [ApplicationMenu; 5] = [
    ApplicationMenu::File,
    ApplicationMenu::Edit,
    ApplicationMenu::View,
    ApplicationMenu::Design,
    ApplicationMenu::Simulate,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuProjection {
    Hidden,
    ThroughSimulate,
    ThroughModels,
    All,
}

impl MenuProjection {
    const fn for_layout(viewport_width: f32, compact_shell: bool) -> Self {
        if compact_shell {
            Self::Hidden
        } else {
            Self::for_width(viewport_width)
        }
    }

    const fn for_width(viewport_width: f32) -> Self {
        if viewport_width <= 820.0 {
            Self::Hidden
        } else if viewport_width <= 1020.0 {
            Self::ThroughSimulate
        } else if viewport_width <= 1360.0 {
            Self::ThroughModels
        } else {
            Self::All
        }
    }

    const fn visible_menus(self) -> &'static [ApplicationMenu] {
        match self {
            Self::Hidden => &[],
            Self::ThroughSimulate => &THROUGH_SIMULATE_MENUS,
            Self::ThroughModels => &THROUGH_MODELS_MENUS,
            Self::All => &ALL_MENUS,
        }
    }

    const fn has_overflow(self) -> bool {
        matches!(self, Self::ThroughSimulate | Self::ThroughModels)
    }

    #[cfg(test)]
    const fn overflow_trigger_label(self) -> &'static str {
        match self {
            // The mockup adds a text label only in its 1021-1360 px range.
            Self::ThroughModels => "⋯  More",
            Self::ThroughSimulate => "⋯",
            Self::Hidden | Self::All => "",
        }
    }

    const fn shows_title_context(self) -> bool {
        !matches!(self, Self::Hidden)
    }
}

fn brand(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    show_wordmark: bool,
    height: f32,
    large_target: bool,
    retain_desktop_width: bool,
) {
    let t = Tokens::get(ui.ctx());
    let width = if show_wordmark || retain_desktop_width {
        112.0
    } else if large_target {
        44.0
    } else {
        34.0
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "RSpice project overview",
        )
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    let icon_center_x = if show_wordmark || retain_desktop_width {
        rect.left() + 21.0
    } else {
        rect.left() + 17.0
    };
    WorkbenchIcon::Brand.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(icon_center_x, rect.center().y),
            Vec2::splat(20.0),
        ),
        t.color.accent,
    );
    if show_wordmark {
        ui.painter().text(
            egui::Pos2::new(rect.left() + 38.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            "RSpice",
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        );
    }
    if response.clicked() {
        Command::OpenWorkspace(Workspace::Project).execute(app);
    }
    theme::paint_focus_ring_outset(ui, &response, rect);
    response.on_hover_text("RSpice project overview");
}

fn menus(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    projection: MenuProjection,
    large_targets: bool,
    title_bar_height: f32,
) {
    if projection == MenuProjection::Hidden {
        return;
    }

    let bar_width = menu_bar_width(ui, projection, large_targets);
    let mut triggers = Vec::new();
    let menu_bar = ui.allocate_ui_with_layout(
        Vec2::new(bar_width, title_bar_height),
        Layout::left_to_right(Align::Center),
        |ui| {
            // These triggers own their painting and their popup contract. A
            // second `egui::MenuBar` wrapper adds an implicit horizontal row
            // around the already exact-height title track; under some scale
            // factors that row can round beyond the 35 px panel and clip the
            // menu labels to a narrow strip. Keep one exact geometry owner,
            // matching the mockup's fixed-height `.menu-bar` flex track.
            ui.spacing_mut().item_spacing.x = 0.0;
            for &menu in projection.visible_menus() {
                triggers.push(top_menu(
                    ui,
                    app,
                    menu,
                    large_targets,
                    title_bar_height,
                    |ui, app| menu.show(ui, app),
                ));
            }
            if projection.has_overflow() {
                triggers.push(overflow_menu_button(
                    ui,
                    app,
                    projection,
                    large_targets,
                    title_bar_height,
                ));
            }
        },
    );
    ui.ctx()
        .accesskit_node_builder(menu_bar.response.id, |node| {
            node.set_role(egui::accesskit::Role::Menu);
            node.set_label("Application menu");
        });
    handle_menu_bar_keyboard(ui.ctx(), &triggers, !app.state.application_modal_open());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MenuTriggerKey {
    Application(ApplicationMenu),
    Overflow,
}

impl MenuTriggerKey {
    fn label(self) -> &'static str {
        match self {
            Self::Application(menu) => menu.label(),
            Self::Overflow => "More application menus",
        }
    }
}

#[derive(Clone)]
struct MenuTriggerResponse {
    key: MenuTriggerKey,
    popup_id: Id,
    response: Response,
}

#[derive(Debug, Clone, Copy)]
struct MenuTriggerPresentation {
    width: f32,
    height: f32,
    large_targets: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InitialMenuFocus {
    First,
    Last,
}

#[derive(Debug, Clone)]
struct MenuRowRecord {
    id: Id,
    label: String,
    enabled: bool,
}

#[derive(Debug, Clone, Default)]
struct MenuRenderState {
    rows: Vec<MenuRowRecord>,
    initial_focus: Option<InitialMenuFocus>,
    initial_focus_claimed: bool,
}

#[derive(Debug, Clone, Default)]
struct MenuTypeaheadState {
    value: String,
    last_input_time: f64,
}

fn menu_bar_width(ui: &Ui, projection: MenuProjection, large_targets: bool) -> f32 {
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let menu_width = projection
        .visible_menus()
        .iter()
        .map(|menu| {
            ui.painter()
                .layout_no_wrap(menu.label().to_owned(), font.clone(), egui::Color32::WHITE)
                .size()
                .x
                + 14.0
        })
        .sum::<f32>();
    menu_width
        + if projection.has_overflow() {
            overflow_trigger_width(projection, large_targets)
        } else {
            0.0
        }
}

fn top_menu(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    menu: ApplicationMenu,
    large_targets: bool,
    title_bar_height: f32,
    contents: impl FnOnce(&mut Ui, &mut RSpiceApp),
) -> MenuTriggerResponse {
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let width = ui
        .painter()
        .layout_no_wrap(menu.label().to_owned(), font, egui::Color32::WHITE)
        .size()
        .x
        + 14.0;
    menu_trigger(
        ui,
        app,
        MenuTriggerKey::Application(menu),
        MenuTriggerPresentation {
            width,
            height: title_bar_height,
            large_targets,
        },
        contents,
    )
}

fn overflow_menu_button(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    projection: MenuProjection,
    large_targets: bool,
    title_bar_height: f32,
) -> MenuTriggerResponse {
    let height = if large_targets {
        MENU_TOUCH_ROW_HEIGHT
    } else {
        27.0_f32.min(title_bar_height)
    };
    menu_trigger(
        ui,
        app,
        MenuTriggerKey::Overflow,
        MenuTriggerPresentation {
            width: overflow_trigger_width(projection, large_targets),
            height,
            large_targets,
        },
        move |ui, app| overflow_menu(ui, app, projection),
    )
}

const fn overflow_trigger_width(projection: MenuProjection, large_targets: bool) -> f32 {
    if large_targets {
        44.0
    } else if matches!(projection, MenuProjection::ThroughModels) {
        54.0
    } else {
        28.0
    }
}

fn menu_trigger(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    key: MenuTriggerKey,
    presentation: MenuTriggerPresentation,
    contents: impl FnOnce(&mut Ui, &mut RSpiceApp),
) -> MenuTriggerResponse {
    let MenuTriggerPresentation {
        width,
        height,
        large_targets,
    } = presentation;
    let t = Tokens::get(ui.ctx());
    let id = Id::new(("workbench.title_menu.trigger", key));
    let (_, rect) = ui.allocate_space(Vec2::new(width, height));
    let response = ui
        .interact(rect, id, Sense::click())
        .on_hover_text(key.label());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), key.label())
    });
    let popup_id = Popup::default_response_id(&response);
    let was_open = Popup::is_id_open(ui.ctx(), popup_id);
    let another_application_menu_is_open = known_application_popup_ids()
        .into_iter()
        .any(|candidate| candidate != popup_id && Popup::is_id_open(ui.ctx(), candidate));
    let keyboard_focus = if response.has_focus() {
        if ui
            .ctx()
            .input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowDown))
        {
            Some(InitialMenuFocus::First)
        } else if ui
            .ctx()
            .input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowUp))
        {
            Some(InitialMenuFocus::Last)
        } else if ui.ctx().input_mut(|input| {
            input.consume_key(Modifiers::NONE, Key::Enter)
                || input.consume_key(Modifiers::NONE, Key::Space)
        }) || (response.clicked() && !response.clicked_by(PointerButton::Primary))
        {
            Some(InitialMenuFocus::First)
        } else {
            None
        }
    } else {
        None
    };
    if let Some(initial_focus) = keyboard_focus {
        ui.ctx().data_mut(|data| {
            data.insert_temp(popup_id.with("initial_focus"), initial_focus);
        });
    }

    let open_for_paint = was_open
        || response.clicked()
        || keyboard_focus.is_some()
        || (response.hovered() && another_application_menu_is_open);
    if response.hovered() || response.has_focus() || open_for_paint {
        ui.painter().rect_filled(
            rect,
            if key == MenuTriggerKey::Overflow {
                t.radius
            } else {
                0.0
            },
            if open_for_paint {
                t.color.bg_active
            } else {
                t.color.bg_hover
            },
        );
    }
    let ink = if response.hovered() || response.has_focus() || open_for_paint {
        t.color.text
    } else {
        t.color.text_dim
    };
    match key {
        MenuTriggerKey::Application(menu) => {
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                menu.label(),
                theme::sans(tokens::FS_1, FontWeight::Regular),
                ink,
            );
        }
        MenuTriggerKey::Overflow => {
            let show_label = width >= 54.0;
            let icon_center = if show_label {
                egui::pos2(rect.left() + 15.0, rect.center().y)
            } else {
                rect.center()
            };
            WorkbenchIcon::More.paint(
                ui.painter(),
                egui::Rect::from_center_size(icon_center, Vec2::splat(16.0)),
                ink,
            );
            if show_label {
                ui.painter().text(
                    egui::pos2(rect.left() + 27.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    "More",
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    ink,
                );
            }
        }
    }
    theme::paint_focus_ring_outset(ui, &response, rect);

    let popup = Popup::menu(&response).gap(MENU_POPUP_GAP).show(|ui| {
        configure_menu_popup(ui, large_targets);
        ui.ctx().accesskit_node_builder(popup_id, |node| {
            node.set_role(egui::accesskit::Role::Menu);
            node.set_label(format!("{} menu", key.label()));
        });
        begin_menu_render(ui.ctx(), popup_id);
        contents(ui, app);
        let render_state = finish_menu_render(ui.ctx());
        handle_popup_keyboard(ui.ctx(), popup_id, &response, &render_state.rows);
    });
    if keyboard_focus.is_some() || (response.hovered() && another_application_menu_is_open) {
        // Opening after `Popup::show` keeps egui's toggle policy from closing a
        // custom keyboard/hover request during the same pass. Memory owns the
        // popup from the next pass onward, exactly like adjacent-menu
        // navigation handled after all triggers render.
        Popup::open_id(ui.ctx(), popup_id);
    }
    let is_open = popup.is_some() || Popup::is_id_open(ui.ctx(), popup_id);
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::MenuItem);
        node.set_label(key.label());
        node.set_expanded(is_open);
        node.set_controls([popup_id.accesskit_id()]);
    });

    MenuTriggerResponse {
        key,
        popup_id,
        response,
    }
}

fn known_application_popup_ids() -> Vec<Id> {
    ALL_MENUS
        .into_iter()
        .map(|menu| {
            Id::new((
                "workbench.title_menu.trigger",
                MenuTriggerKey::Application(menu),
            ))
            .with("popup")
        })
        .chain(std::iter::once(
            Id::new(("workbench.title_menu.trigger", MenuTriggerKey::Overflow)).with("popup"),
        ))
        .collect()
}

fn begin_menu_render(ctx: &Context, popup_id: Id) {
    let initial_focus = ctx.data_mut(|data| {
        let id = popup_id.with("initial_focus");
        let focus = data.get_temp::<InitialMenuFocus>(id);
        data.remove::<InitialMenuFocus>(id);
        focus
    });
    ctx.data_mut(|data| {
        data.insert_temp(
            Id::new(MENU_RENDER_STATE_ID),
            MenuRenderState {
                initial_focus,
                ..MenuRenderState::default()
            },
        );
    });
}

fn record_menu_row(ctx: &Context, response: &Response, label: &str, enabled: bool) {
    let request_row_focus = ctx.data_mut(|data| {
        let id = Id::new(MENU_RENDER_STATE_ID);
        let Some(mut state) = data.get_temp::<MenuRenderState>(id) else {
            return false;
        };
        let request_row_focus = if enabled {
            match state.initial_focus {
                Some(InitialMenuFocus::First) if !state.initial_focus_claimed => {
                    state.initial_focus_claimed = true;
                    true
                }
                Some(InitialMenuFocus::Last) => {
                    // Every enabled row requests focus while rendering; the last
                    // enabled row therefore owns the final request.
                    state.initial_focus_claimed = true;
                    true
                }
                Some(InitialMenuFocus::First) | None => false,
            }
        } else {
            false
        };
        state.rows.push(MenuRowRecord {
            id: response.id,
            label: label.to_owned(),
            enabled,
        });
        data.insert_temp(id, state);
        request_row_focus
    });
    // `ctx.data_mut` is backed by egui's Memory lock. Requesting focus from
    // inside that closure recursively acquires the same lock and deadlocks on
    // native and WASM builds; apply the recorded focus request after release.
    if request_row_focus {
        response.request_focus();
    }
}

fn finish_menu_render(ctx: &Context) -> MenuRenderState {
    ctx.data_mut(|data| {
        data.remove_temp::<MenuRenderState>(Id::new(MENU_RENDER_STATE_ID))
            .unwrap_or_default()
    })
}

fn request_focus(ctx: &Context, id: Id) {
    ctx.memory_mut(|memory| memory.request_focus(id));
}

fn handle_popup_keyboard(ctx: &Context, popup_id: Id, trigger: &Response, rows: &[MenuRowRecord]) {
    let enabled_rows = rows.iter().filter(|row| row.enabled).collect::<Vec<_>>();
    if enabled_rows.is_empty() {
        return;
    }
    let focused = ctx.memory(|memory| memory.focused());
    let focused_index = focused.and_then(|id| enabled_rows.iter().position(|row| row.id == id));

    let destination = if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowDown)) {
        Some(focused_index.map_or(0, |index| (index + 1) % enabled_rows.len()))
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowUp)) {
        Some(focused_index.map_or(enabled_rows.len() - 1, |index| {
            (index + enabled_rows.len() - 1) % enabled_rows.len()
        }))
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Home)) {
        Some(0)
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::End)) {
        Some(enabled_rows.len() - 1)
    } else {
        None
    };
    if let Some(index) = destination {
        request_focus(ctx, enabled_rows[index].id);
        return;
    }

    if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape)) {
        Popup::close_id(ctx, popup_id);
        trigger.request_focus();
        return;
    }
    if ctx.input(|input| input.key_pressed(Key::Tab)) {
        Popup::close_id(ctx, popup_id);
        return;
    }

    if focused_index.is_some()
        && let Some(query) = menu_typeahead_query(ctx)
    {
        let start = focused_index.unwrap_or(0);
        if let Some(row) = enabled_rows
            .iter()
            .cycle()
            .skip(start + 1)
            .take(enabled_rows.len())
            .find(|row| row.label.to_lowercase().starts_with(&query))
        {
            request_focus(ctx, row.id);
        }
    }
}

fn handle_menu_bar_keyboard(
    ctx: &Context,
    triggers: &[MenuTriggerResponse],
    keyboard_enabled: bool,
) {
    if !keyboard_enabled || triggers.is_empty() {
        return;
    }
    let focused = ctx.memory(|memory| memory.focused());
    let focused_trigger = focused.and_then(|id| {
        triggers
            .iter()
            .position(|trigger| trigger.response.id == id)
    });
    let open_trigger = triggers
        .iter()
        .position(|trigger| Popup::is_id_open(ctx, trigger.popup_id));

    if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::F10)) {
        if focused_trigger.is_some() || open_trigger.is_some() {
            Popup::close_all(ctx);
            let return_focus = ctx.data_mut(|data| {
                data.get_temp::<Option<Id>>(Id::new(MENU_RETURN_FOCUS_ID))
                    .flatten()
            });
            if let Some(id) = return_focus {
                request_focus(ctx, id);
            }
        } else {
            ctx.data_mut(|data| {
                data.insert_temp(Id::new(MENU_RETURN_FOCUS_ID), focused);
            });
            triggers[0].response.request_focus();
        }
        return;
    }

    if let Some(open_index) = open_trigger {
        let direction =
            if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowRight)) {
                Some(1_isize)
            } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowLeft)) {
                Some(-1_isize)
            } else {
                None
            };
        if let Some(direction) = direction {
            let next =
                (open_index as isize + direction).rem_euclid(triggers.len() as isize) as usize;
            Popup::open_id(ctx, triggers[next].popup_id);
            ctx.data_mut(|data| {
                data.insert_temp(
                    triggers[next].popup_id.with("initial_focus"),
                    if direction < 0 {
                        InitialMenuFocus::Last
                    } else {
                        InitialMenuFocus::First
                    },
                );
            });
            triggers[next].response.request_focus();
            return;
        }
    }

    let Some(current) = focused_trigger else {
        return;
    };
    let destination = if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowRight))
    {
        Some((current + 1) % triggers.len())
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowLeft)) {
        Some((current + triggers.len() - 1) % triggers.len())
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Home)) {
        Some(0)
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::End)) {
        Some(triggers.len() - 1)
    } else {
        None
    };
    if let Some(index) = destination {
        triggers[index].response.request_focus();
        return;
    }

    if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape)) {
        let return_focus = ctx.data_mut(|data| {
            data.get_temp::<Option<Id>>(Id::new(MENU_RETURN_FOCUS_ID))
                .flatten()
        });
        if let Some(id) = return_focus {
            request_focus(ctx, id);
        } else {
            ctx.memory_mut(|memory| {
                memory.surrender_focus(focused.unwrap_or(triggers[current].response.id))
            });
        }
        return;
    }

    if let Some(query) = menu_typeahead_query(ctx)
        && let Some(index) = triggers
            .iter()
            .enumerate()
            .cycle()
            .skip(current + 1)
            .take(triggers.len())
            .find_map(|(index, trigger)| {
                trigger
                    .key
                    .label()
                    .to_lowercase()
                    .starts_with(&query)
                    .then_some(index)
            })
    {
        triggers[index].response.request_focus();
    }
}

fn menu_typeahead_query(ctx: &Context) -> Option<String> {
    let typed = ctx.input(|input| {
        input.events.iter().rev().find_map(|event| match event {
            egui::Event::Text(text) if text.chars().any(|character| !character.is_whitespace()) => {
                Some(text.to_lowercase())
            }
            _ => None,
        })
    })?;
    let now = ctx.input(|input| input.time);
    ctx.data_mut(|data| {
        let id = Id::new(MENU_TYPEAHEAD_ID);
        let mut state = data.get_temp::<MenuTypeaheadState>(id).unwrap_or_default();
        if now - state.last_input_time > 0.7 {
            state.value.clear();
        }
        state.value.push_str(&typed);
        state.last_input_time = now;
        let repeated = state.value.chars().all(|character| {
            state
                .value
                .chars()
                .next()
                .is_some_and(|first| character == first)
        });
        let query = if repeated {
            state
                .value
                .chars()
                .next()
                .map(|character| character.to_string())
        } else {
            Some(state.value.clone())
        };
        data.insert_temp(id, state);
        query
    })
}

fn configure_menu_popup(ui: &mut Ui, large_targets: bool) {
    ui.set_min_width(MENU_CONTENT_WIDTH);
    ui.set_max_width(MENU_CONTENT_WIDTH);
    ui.spacing_mut().item_spacing.y = 0.0;
    ui.spacing_mut().interact_size.y = if large_targets {
        MENU_TOUCH_ROW_HEIGHT
    } else {
        MENU_ROW_HEIGHT
    };
}

fn command_item(ui: &mut Ui, app: &mut RSpiceApp, command: Command) {
    let spec = command.spec();
    command_item_as(ui, app, command, spec.label, None);
}

fn command_item_as(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    command: Command,
    label: &str,
    shortcut_override: Option<&str>,
) {
    let availability = command.availability(app);
    if availability == CommandAvailability::Hidden {
        return;
    }
    let enabled = availability.is_available();
    let shortcut = shortcut_for_occurrence(
        app.state.ui.preferences.shortcuts(),
        crate::common::app::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
        command,
        shortcut_override,
    );
    let row_height = ui.spacing().interact_size.y.max(MENU_ROW_HEIGHT);
    let (rect, response) = ui
        .add_enabled_ui(enabled, |ui| {
            ui.allocate_exact_size(Vec2::new(ui.available_width(), row_height), Sense::click())
        })
        .inner;
    let response = match availability {
        CommandAvailability::Disabled(reason) => {
            response.on_hover_text(format!("Unavailable: {reason}"))
        }
        CommandAvailability::Available | CommandAvailability::Hidden => response,
    };
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::MenuItem);
        node.set_label(label);
        if let CommandAvailability::Disabled(reason) = availability {
            node.set_description(reason);
        }
        if enabled && !shortcut.is_empty() {
            node.set_keyboard_shortcut(shortcut.as_str());
        }
    });
    record_menu_row(ui.ctx(), &response, label, enabled);

    let t = Tokens::get(ui.ctx());
    let highlighted = enabled && (response.hovered() || response.has_focus());
    if highlighted {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    let ink = if enabled {
        if highlighted {
            t.color.text
        } else {
            t.color.text_dim
        }
    } else {
        t.color.text_dim.gamma_multiply(0.4)
    };
    let icon_rect = egui::Rect::from_min_size(
        egui::Pos2::new(rect.left() + 7.0, rect.center().y - 8.5),
        Vec2::splat(17.0),
    );
    command_icon(command).paint(ui.painter(), icon_rect, ink);
    let label_left = rect.left() + 31.0;
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let shortcut_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let shortcut_width = if shortcut.is_empty() {
        0.0
    } else {
        ui.painter()
            .layout_no_wrap(shortcut.to_owned(), shortcut_font.clone(), ink)
            .size()
            .x
            + 7.0
    };
    let label_right = rect.right() - 7.0 - shortcut_width;
    let label_width = (label_right - label_left).max(8.0);
    let painted_label = ellipsize_to_width(ui.painter(), label, &label_font, label_width);
    // Keep the label in its mockup grid column even if a platform font has
    // wider metrics than the bundled face. The measured ellipsis provides a
    // readable ending; the clip is the final invariant that prevents a long
    // localized label from ever painting through the shortcut column.
    ui.painter()
        .with_clip_rect(egui::Rect::from_min_max(
            egui::pos2(label_left, rect.top()),
            egui::pos2(label_right, rect.bottom()),
        ))
        .text(
            egui::Pos2::new(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            painted_label,
            label_font,
            ink,
        );
    if !shortcut.is_empty() {
        ui.painter().text(
            egui::Pos2::new(rect.right() - 7.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            shortcut,
            shortcut_font,
            if enabled {
                t.color.text_faint
            } else {
                t.color.text_faint.gamma_multiply(0.4)
            },
        );
    }
    theme::paint_focus_ring_outset(ui, &response, rect);

    let response = match availability {
        CommandAvailability::Disabled(reason) => response.on_disabled_hover_text(reason),
        CommandAvailability::Available | CommandAvailability::Hidden => response,
    };
    if response.clicked() {
        command.execute(app);
        ui.close();
    }
}

fn command_icon(command: Command) -> WorkbenchIcon {
    match command {
        Command::OpenWorkspace(Workspace::Project) | Command::ProjectLauncher => {
            WorkbenchIcon::Project
        }
        Command::OpenWorkspace(Workspace::Design) => WorkbenchIcon::Design,
        Command::OpenWorkspace(Workspace::Simulate) => WorkbenchIcon::Simulate,
        Command::OpenWorkspace(Workspace::Results)
        | Command::ResultViewer(_)
        | Command::WaveformCalculator => WorkbenchIcon::Results,
        Command::OpenWorkspace(Workspace::Verify)
        | Command::VerificationPage(_)
        | Command::EditSpecifications => WorkbenchIcon::Verify,
        Command::OpenWorkspace(Workspace::Models)
        | Command::ModelsPage(_)
        | Command::ModelBrowser
        | Command::PdkSettings => WorkbenchIcon::Models,
        Command::OpenWorkspace(Workspace::Netlist) => WorkbenchIcon::Netlist,
        Command::AutomationConsole => WorkbenchIcon::Terminal,
        Command::NewProject => WorkbenchIcon::File,
        Command::NewCell => WorkbenchIcon::Add,
        Command::OpenProject | Command::OpenDocument => WorkbenchIcon::Folder,
        Command::RecentProjects => WorkbenchIcon::History,
        Command::Save | Command::SaveAs | Command::SaveAll => WorkbenchIcon::Save,
        Command::RevertActiveDocument | Command::ResetActiveView | Command::ResetLayout => {
            WorkbenchIcon::Refresh
        }
        Command::ImportNetlist
        | Command::ImportVerilogA
        | Command::ExportSchematicSvg
        | Command::ExportWaveformsCsv
        | Command::ExportNetlist(_) => WorkbenchIcon::Export,
        Command::Undo => WorkbenchIcon::Undo,
        Command::Redo => WorkbenchIcon::Redo,
        Command::Cut | Command::Copy | Command::Paste | Command::Duplicate => WorkbenchIcon::Copy,
        Command::Delete | Command::ClearConsole => WorkbenchIcon::Trash,
        Command::SelectAll => WorkbenchIcon::Grid,
        Command::ObjectProperties | Command::SimulationOptions => WorkbenchIcon::Sliders,
        Command::FindInDesign | Command::CommandPalette => WorkbenchIcon::Search,
        Command::Preferences => WorkbenchIcon::Settings,
        Command::ZoomIn => WorkbenchIcon::ZoomIn,
        Command::ZoomOut => WorkbenchIcon::ZoomOut,
        Command::ZoomFit => WorkbenchIcon::ZoomFit,
        Command::CycleGrid => WorkbenchIcon::Grid,
        Command::ToggleNavigator => WorkbenchIcon::Navigator,
        Command::ToggleInspector => WorkbenchIcon::Inspector,
        Command::ToggleConsole => WorkbenchIcon::Console,
        Command::ToggleFocusMode | Command::ToggleFullScreen => WorkbenchIcon::Focus,
        Command::PlaceInstance => WorkbenchIcon::Component,
        Command::Place(_) => WorkbenchIcon::Add,
        Command::PlaceWire => WorkbenchIcon::Wire,
        Command::PlaceJunction => WorkbenchIcon::Grid,
        Command::PlaceLabel => WorkbenchIcon::Label,
        Command::PlaceProbe => WorkbenchIcon::Probe,
        Command::AscendHierarchy => WorkbenchIcon::ArrowLeft,
        Command::DescendHierarchy => WorkbenchIcon::Folder,
        Command::RunSimulation => WorkbenchIcon::Run,
        Command::StopSimulation => WorkbenchIcon::Stop,
        Command::PreflightChecks | Command::RunChecks | Command::CheckAndSave => {
            WorkbenchIcon::Check
        }
        Command::CompileVerilogA => WorkbenchIcon::Code,
        Command::AccountOrganization | Command::License => WorkbenchIcon::User,
        Command::KeyboardShortcuts => WorkbenchIcon::Search,
        Command::InteroperabilityMatrix => WorkbenchIcon::Compare,
        Command::SpecialistToolBrowser => WorkbenchIcon::Grid,
        Command::VisualizationStudio => WorkbenchIcon::Results,
        Command::AddVisualizationPane => WorkbenchIcon::Add,
        Command::VisualizationTraceManager => WorkbenchIcon::Sliders,
        Command::VisualizationCursorManager => WorkbenchIcon::Target,
        Command::VisualizationDocumentProperties => WorkbenchIcon::Settings,
        Command::ExportVisualizationDocument => WorkbenchIcon::Export,
        Command::FeatureAvailability | Command::About => WorkbenchIcon::Info,
        Command::Exit => WorkbenchIcon::Stop,
        Command::CloseActiveDocument => WorkbenchIcon::File,
        Command::CloseProject => WorkbenchIcon::Folder,
        Command::Cancel => WorkbenchIcon::Close,
        _ => WorkbenchIcon::File,
    }
}

fn menu_separator(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 9.0), Sense::hover());
    ui.painter().hline(
        (rect.left() + 5.0)..=(rect.right() - 5.0),
        rect.top() + 4.5,
        egui::Stroke::new(1.0, t.color.border),
    );
}

fn shortcut_for_occurrence(
    shortcuts: &crate::workbench::ShortcutPreferences,
    platform: crate::workbench::commands::CommandPlatform,
    operating_system: egui::os::OperatingSystem,
    command: Command,
    shortcut_override: Option<&str>,
) -> String {
    shortcut_override.map_or_else(
        || shortcuts.resolved_label(command, platform, operating_system),
        str::to_owned,
    )
}

fn file_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::ProjectLauncher);
    menu_separator(ui);
    command_item(ui, app, Command::NewProject);
    command_item(ui, app, Command::OpenProject);
    command_item(ui, app, Command::RecentProjects);
    command_item(ui, app, Command::NewCell);
    command_item(ui, app, Command::OpenDocument);
    menu_separator(ui);
    command_item(ui, app, Command::Save);
    command_item(ui, app, Command::SaveAs);
    command_item(ui, app, Command::SaveAll);
    command_item(ui, app, Command::RevertActiveDocument);
    menu_separator(ui);
    command_item(ui, app, Command::ImportNetlist);
    command_item(ui, app, Command::ImportVerilogA);
    menu_separator(ui);
    command_item(ui, app, Command::ExportSchematicSvg);
    command_item(ui, app, Command::ExportWaveformsCsv);
    command_item(
        ui,
        app,
        Command::ExportNetlist(crate::io::NetlistFormat::Spice),
    );
    menu_separator(ui);
    command_item(ui, app, Command::CloseActiveDocument);
    command_item(ui, app, Command::CloseProject);
    menu_separator(ui);
    command_item(ui, app, Command::Exit);
}

fn edit_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    for command in [Command::Undo, Command::Redo] {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    for command in [
        Command::Cut,
        Command::Copy,
        Command::Paste,
        Command::Duplicate,
        Command::Delete,
        Command::SelectAll,
    ] {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    command_item(ui, app, Command::ObjectProperties);
    command_item(ui, app, Command::FindInDesign);
    menu_separator(ui);
    command_item(ui, app, Command::Preferences);
}

fn view_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    for command in [
        Command::ZoomIn,
        Command::ZoomOut,
        Command::ZoomFit,
        Command::ZoomOneToOne,
        Command::CycleGrid,
    ] {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    command_item_as(
        ui,
        app,
        Command::ToggleFullScreen,
        if app.state.workbench.full_screen {
            "Exit full screen"
        } else {
            "Enter full screen"
        },
        None,
    );
    command_item(ui, app, Command::ResetActiveView);
    menu_separator(ui);
    command_item(ui, app, Command::ToggleNavigator);
    command_item(ui, app, Command::ToggleInspector);
    command_item(ui, app, Command::ToggleConsole);
}

fn design_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::OpenWorkspace(Workspace::Design),
        "Open active schematic",
        None,
    );
    command_item(ui, app, Command::AscendHierarchy);
    command_item_as(ui, app, Command::DescendHierarchy, DESCEND_MENU_LABEL, None);
    menu_separator(ui);
    for command in [
        Command::PlaceInstance,
        Command::PlaceWire,
        Command::PlaceJunction,
        Command::PlaceLabel,
        Command::PlaceProbe,
    ] {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    for command in [
        Command::RotateSelection,
        Command::MirrorSelectionHorizontal,
        Command::MirrorSelectionVertical,
        Command::ObjectProperties,
        Command::FindInDesign,
    ] {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    command_item(ui, app, Command::RunChecks);
    command_item(ui, app, Command::CheckAndSave);
    command_item(ui, app, Command::ClearChecks);
    command_item(ui, app, Command::GenerateNetlist);
}

fn simulate_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::RunSimulation);
    command_item(ui, app, Command::StopSimulation);
    command_item(ui, app, Command::JobsManager);
    command_item(ui, app, Command::PreflightChecks);
    menu_separator(ui);
    command_item_as(
        ui,
        app,
        Command::OpenWorkspace(Workspace::Simulate),
        "Simulation Studio",
        None,
    );
    command_item(ui, app, Command::SimulationOptions);
    command_item(ui, app, Command::GenerateNetlist);
    command_item(ui, app, Command::EditSpecifications);
}

fn results_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::ResultViewer(crate::workbench::ResultViewer::Waves),
        "Open results workspace",
        None,
    );
    menu_separator(ui);
    for (viewer, label) in [
        (crate::workbench::ResultViewer::Bode, "Bode / stability"),
        (crate::workbench::ResultViewer::Fft, "FFT / spectrum"),
        (crate::workbench::ResultViewer::Eye, "Eye diagram"),
        (crate::workbench::ResultViewer::Hist, "Distribution"),
        (crate::workbench::ResultViewer::Op, "Operating point"),
        (
            crate::workbench::ResultViewer::NoiseContrib,
            "Noise contributors",
        ),
        (
            crate::workbench::ResultViewer::Contribution,
            "Sensitivity contributions",
        ),
        (
            crate::workbench::ResultViewer::Specs,
            "Measurements & specifications",
        ),
        (crate::workbench::ResultViewer::Nyquist, "Nyquist"),
        (crate::workbench::ResultViewer::Smith, "Smith chart"),
        (crate::workbench::ResultViewer::PoleZero, "Pole-zero"),
    ] {
        command_item_as(ui, app, Command::ResultViewer(viewer), label, None);
    }
    menu_separator(ui);
    command_item_as(ui, app, Command::WaveformCalculator, "Calculator…", None);
    command_item(ui, app, Command::ExportWaveformsCsv);
    command_item(ui, app, Command::ClearResults);
}

fn verify_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::Yield),
        "Verification cockpit",
        None,
    );
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::Corners),
        "Corner matrix",
        None,
    );
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::Optimization),
        "Optimization",
        None,
    );
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::Reliability),
        "Reliability, fault and SOA",
        None,
    );
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::Regression),
        "Regression plan",
        None,
    );
    menu_separator(ui);
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::Drc),
        "Physical DRC",
        None,
    );
}

fn models_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::ModelsPage(ModelsPage::Catalog),
        "Model & library catalog",
        None,
    );
    menu_separator(ui);
    command_item_as(
        ui,
        app,
        Command::CompileVerilogA,
        "Verilog-A/AMS compiler",
        None,
    );
}

fn automation_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::AutomationConsole,
        "Automation workspace",
        None,
    );
}

fn overflow_menu(ui: &mut Ui, app: &mut RSpiceApp, projection: MenuProjection) {
    if projection == MenuProjection::ThroughSimulate {
        command_item_as(
            ui,
            app,
            Command::ResultViewer(crate::workbench::ResultViewer::Waves),
            "Results workspace",
            Some(""),
        );
        command_item_as(
            ui,
            app,
            Command::VerificationPage(VerificationPage::Yield),
            "Verification workspace",
            Some(""),
        );
        command_item_as(
            ui,
            app,
            Command::ModelsPage(ModelsPage::Catalog),
            "Models workspace",
            Some(""),
        );
    }
    command_item_as(
        ui,
        app,
        Command::AutomationConsole,
        "Automation workspace",
        Some(""),
    );
    menu_separator(ui);
    command_item(ui, app, Command::ResetLayout);
    command_item(ui, app, Command::ToggleFocusMode);
    command_item(ui, app, Command::ToggleConsole);
    menu_separator(ui);
    command_item(ui, app, Command::KeyboardShortcuts);
    command_item(ui, app, Command::FeatureAvailability);
}

fn window_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::ToggleNavigator);
    command_item(ui, app, Command::ToggleInspector);
    command_item(ui, app, Command::ToggleConsole);
    command_item(ui, app, Command::ToggleFocusMode);
    menu_separator(ui);
    command_item(ui, app, Command::PreviousWorkspace);
    command_item(ui, app, Command::NextWorkspace);
    command_item(ui, app, Command::ResetLayout);
    command_item(ui, app, Command::ToggleFullScreen);
}

fn help_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::CommandPalette);
    command_item(ui, app, Command::KeyboardShortcuts);
    command_item(ui, app, Command::FeatureAvailability);
    command_item(ui, app, Command::InteroperabilityMatrix);
    command_item(ui, app, Command::License);
    menu_separator(ui);
    command_item(ui, app, Command::About);
}

fn paint_title_context(
    ui: &mut Ui,
    app: &RSpiceApp,
    bounds: egui::Rect,
    compact: bool,
    left_aligned: bool,
) {
    if bounds.width() < 40.0 {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let dirty = app.state.schematic.is_dirty || app.state.workspace.any_dirty();
    let cell = active_title_cell(app);
    let full = if compact {
        cell.clone()
    } else {
        app.state.workspace.project.display_name().to_owned()
    };
    let available_text_width = (bounds.width() - 26.0).max(12.0);
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let painter = ui
        .painter()
        .with_clip_rect(bounds.shrink2(egui::vec2(5.0, 0.0)));
    let text = ellipsize_to_width(&painter, &full, &font, available_text_width);
    let galley = painter.layout_no_wrap(text, font, t.color.text);
    let total_width = 13.0 + galley.size().x;
    let left = if left_aligned {
        bounds.left() + 12.0
    } else {
        (bounds.center().x - total_width * 0.5).max(bounds.left() + 5.0)
    };
    let state_color = if dirty { t.color.warn } else { t.color.ok };
    painter.circle_filled(
        egui::pos2(left + 3.0, bounds.center().y),
        6.0,
        state_color.gamma_multiply(0.11),
    );
    painter.circle_filled(egui::pos2(left + 3.0, bounds.center().y), 3.0, state_color);
    painter.galley(
        egui::pos2(left + 13.0, bounds.center().y - galley.size().y * 0.5),
        galley,
        t.color.text,
    );

    let status = ui.interact(
        bounds,
        Id::new("workbench.title_context.status"),
        Sense::hover(),
    );
    ui.ctx().accesskit_node_builder(status.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label(format!(
            "Active project: {}; {}; {}",
            app.state.workspace.project.display_name(),
            cell,
            if dirty {
                "unsaved changes"
            } else {
                "all changes saved"
            }
        ));
    });
}

fn ellipsize_to_width(
    painter: &egui::Painter,
    value: &str,
    font: &egui::FontId,
    maximum_width: f32,
) -> String {
    if painter
        .layout_no_wrap(value.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        <= maximum_width
    {
        return value.to_owned();
    }
    let characters = value.chars().collect::<Vec<_>>();
    let mut low = 0_usize;
    let mut high = characters.len();
    while low < high {
        let mid = (low + high).div_ceil(2);
        let candidate = characters[..mid]
            .iter()
            .copied()
            .chain(std::iter::once('…'))
            .collect::<String>();
        let fits = painter
            .layout_no_wrap(candidate, font.clone(), egui::Color32::WHITE)
            .size()
            .x
            <= maximum_width;
        if fits {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    characters[..low]
        .iter()
        .copied()
        .chain(std::iter::once('…'))
        .collect()
}

fn active_title_cell(app: &RSpiceApp) -> String {
    match app.state.workbench.workspace {
        Workspace::Project => "Project overview".to_owned(),
        Workspace::Design => format!(
            "{} · {}",
            app.state.workspace.active_view.cell, app.state.workspace.active_view.view
        ),
        Workspace::Simulate => "Simulation plan".to_owned(),
        Workspace::Results => "Results".to_owned(),
        Workspace::Verify => "Verification".to_owned(),
        Workspace::Models => "Model & Library Manager".to_owned(),
        Workspace::Netlist => "top.sp · generated".to_owned(),
    }
}

fn ellipsize(value: &str, maximum_characters: usize) -> String {
    let count = value.chars().count();
    if count <= maximum_characters {
        return value.to_owned();
    }
    if maximum_characters <= 1 {
        return "…".to_owned();
    }
    let mut shortened = value
        .chars()
        .take(maximum_characters - 1)
        .collect::<String>();
    shortened.push('…');
    shortened
}

fn search_button(ui: &mut Ui, app: &RSpiceApp, viewport_width: f32, large_target: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
        Command::CommandPalette,
        crate::common::app::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
    );
    let keycap = shortcut.replace('+', " ");
    let width = search_button_width(viewport_width, large_target);
    let height = if large_target { 44.0 } else { 25.0 };
    let icon_only = width <= 60.0;
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Search and run a command",
        )
    });
    if !shortcut.is_empty() {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_keyboard_shortcut(shortcut.as_str());
        });
    }
    if !icon_only {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_inset);
    }
    ui.painter().rect_stroke(
        rect,
        t.radius,
        egui::Stroke::new(
            1.0,
            if response.hovered() || response.has_focus() {
                t.color.border_strong
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    WorkbenchIcon::Search.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(
                if icon_only {
                    rect.center().x
                } else {
                    rect.left() + 15.0
                },
                rect.center().y,
            ),
            Vec2::splat(16.0),
        ),
        if response.hovered() || response.has_focus() {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    if !icon_only {
        // CSS `kbd` uses border-box sizing with 4 px inline padding and a
        // 1 px border. Measure the invariant shortcut instead of forcing it
        // into a 36 px box; that fixed width clips the final glyph at the
        // workstation font metrics used by the mockup.
        let shortcut_width = search_keycap_width(ui, &keycap);
        let shortcut_rect = egui::Rect::from_center_size(
            egui::Pos2::new(rect.right() - 7.0 - shortcut_width * 0.5, rect.center().y),
            Vec2::new(shortcut_width, SEARCH_KEYCAP_HEIGHT),
        );
        let label_width = (shortcut_rect.left() - 7.0 - (rect.left() + 30.0)).max(8.0);
        let label = ellipsize(
            "Search or run a command",
            (label_width / 5.9).floor().max(1.0) as usize,
        );
        ui.painter().text(
            egui::Pos2::new(rect.left() + 30.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if response.hovered() || response.has_focus() {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        ui.painter()
            .rect_filled(shortcut_rect, 3.0, t.color.bg_panel_2);
        ui.painter().rect_stroke(
            shortcut_rect,
            3.0,
            egui::Stroke::new(1.0, t.color.border),
            egui::StrokeKind::Inside,
        );
        ui.painter().hline(
            shortcut_rect.x_range().shrink(3.0),
            shortcut_rect.bottom() - 1.0,
            egui::Stroke::new(1.0, t.color.border_strong),
        );
        ui.painter().text(
            shortcut_rect.center(),
            egui::Align2::CENTER_CENTER,
            &keycap,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    theme::paint_focus_ring_outset(ui, &response, rect);
    let tooltip = if shortcut.is_empty() {
        "Search and run a command".to_owned()
    } else {
        format!("Search and run a command ({shortcut})")
    };
    response.on_hover_text(tooltip).clicked()
}

fn search_keycap_width(ui: &Ui, shortcut: &str) -> f32 {
    let text_width = ui
        .painter()
        .layout_no_wrap(
            shortcut.to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            egui::Color32::WHITE,
        )
        .size()
        .x;
    (text_width + SEARCH_KEYCAP_INLINE_PADDING * 2.0 + SEARCH_KEYCAP_BORDER_WIDTH * 2.0)
        .max(SEARCH_KEYCAP_MIN_WIDTH)
}

fn search_button_width(viewport_width: f32, large_target: bool) -> f32 {
    if large_target {
        44.0
    } else if viewport_width <= 1020.0 {
        31.0
    } else {
        (viewport_width * 0.18).min(230.0)
    }
}

fn icon_action(ui: &mut Ui, icon: WorkbenchIcon, label: &str, large_target: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let size = if large_target {
        Vec2::splat(44.0)
    } else {
        Vec2::new(28.0, 27.0)
    };
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    let active = response.hovered() || response.has_focus();
    if response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    icon.paint(
        ui.painter(),
        rect.shrink(6.0),
        if active {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    response.on_hover_text(label).clicked()
}

fn account_initials(app: &RSpiceApp) -> String {
    let Some(name) = app
        .state
        .license
        .as_ref()
        .map(|license| license.licensed_to.trim())
        .filter(|name| !name.is_empty())
    else {
        return "RS".to_owned();
    };
    let words = name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<Vec<_>>();
    match words.as_slice() {
        [] => "RS".to_owned(),
        [first] => first.to_uppercase().collect(),
        [first, .., last] => format!("{}{}", first.to_uppercase(), last.to_uppercase()),
    }
}

fn account_action(ui: &mut Ui, initials: &str, large_target: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let target_size = if large_target {
        Vec2::splat(44.0)
    } else {
        Vec2::new(28.0, 27.0)
    };
    let (target_rect, response) = ui.allocate_exact_size(target_size, Sense::click());
    let label = "Open account, organization, and licensing";
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    if response.hovered() {
        ui.painter()
            .rect_filled(target_rect, t.radius, t.color.bg_hover);
    }
    let avatar_rect = Rect::from_center_size(
        target_rect.center(),
        Vec2::splat(if large_target { 28.0 } else { 24.0 }),
    );
    ui.painter().circle_filled(
        avatar_rect.center(),
        avatar_rect.width() * 0.5,
        t.color.bg_panel_2,
    );
    ui.painter().circle_stroke(
        avatar_rect.center(),
        avatar_rect.width() * 0.5,
        egui::Stroke::new(1.0, t.color.border_strong),
    );
    ui.painter().text(
        avatar_rect.center(),
        Align2::CENTER_CENTER,
        initials,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        if response.hovered() || response.has_focus() {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring_outset(ui, &response, target_rect);
    response.on_hover_text(label).clicked()
}

fn notification_action(ui: &mut Ui, unread_count: usize, large_target: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let size = if large_target {
        Vec2::splat(44.0)
    } else {
        Vec2::new(28.0, 27.0)
    };
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let accessible_label = if unread_count == 0 {
        "Notifications and activity".to_owned()
    } else {
        format!("Notifications and activity · {unread_count} unread")
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            accessible_label.clone(),
        )
    });
    let active = response.hovered() || response.has_focus();
    if response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    WorkbenchIcon::Bell.paint(
        ui.painter(),
        rect.shrink(6.0),
        if active {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    if unread_count > 0 {
        let badge_text = unread_count.to_string();
        let badge_width = (ui
            .painter()
            .layout_no_wrap(
                badge_text.clone(),
                theme::mono(tokens::FS_0, FontWeight::SemiBold),
                t.color.accent_ink,
            )
            .size()
            .x
            + 8.0)
            .max(14.0);
        let badge_rect = egui::Rect::from_min_size(
            egui::pos2(rect.right() - 3.0 - badge_width, rect.top() + 3.0),
            egui::vec2(badge_width, 14.0),
        );
        ui.painter().rect_filled(badge_rect, 7.0, t.color.accent);
        ui.painter().rect_stroke(
            badge_rect,
            7.0,
            egui::Stroke::new(1.0, t.color.bg_app),
            egui::StrokeKind::Inside,
        );
        ui.painter().text(
            badge_rect.center(),
            egui::Align2::CENTER_CENTER,
            badge_text,
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            t.color.accent_ink,
        );
    }
    theme::paint_focus_ring_outset(ui, &response, rect);
    response.on_hover_text(accessible_label).clicked()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_arch = "wasm32"))]
    fn title_test_app() -> RSpiceApp {
        RSpiceApp::test_instance()
    }

    fn title_key_event(key: Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn run_title_menu_frame(ctx: &Context, app: &mut RSpiceApp, events: Vec<egui::Event>) {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_440.0, 900.0),
            )),
            events,
            ..Default::default()
        };
        let _ = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                menus(ui, app, MenuProjection::All, false, 35.0);
            });
        });
    }

    fn labels(projection: MenuProjection) -> Vec<&'static str> {
        projection
            .visible_menus()
            .iter()
            .map(|menu| menu.label())
            .collect()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn rendered_results_menu_labels() -> Vec<String> {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = title_test_app();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(MENU_OUTER_WIDTH, 720.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(Frame::NONE)
                    .show(ctx, |ui| results_menu(ui, &mut app));
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("AccessKit results-menu tree")
            .nodes
            .into_iter()
            .filter_map(|(_, node)| {
                if node.role() == egui::accesskit::Role::MenuItem {
                    node.label().map(str::to_owned)
                } else {
                    None
                }
            })
            .collect()
    }

    #[test]
    fn menu_projection_matches_mockup_breakpoints() {
        assert_eq!(MenuProjection::for_width(820.0), MenuProjection::Hidden);
        assert_eq!(
            MenuProjection::for_width(821.0),
            MenuProjection::ThroughSimulate
        );
        assert_eq!(
            MenuProjection::for_width(1020.0),
            MenuProjection::ThroughSimulate
        );
        assert_eq!(
            MenuProjection::for_width(1021.0),
            MenuProjection::ThroughModels
        );
        assert_eq!(
            MenuProjection::for_width(1360.0),
            MenuProjection::ThroughModels
        );
        assert_eq!(MenuProjection::for_width(1361.0), MenuProjection::All);
        assert_eq!(
            MenuProjection::for_layout(844.0, true),
            MenuProjection::Hidden
        );
        assert!(title_context_is_left_aligned(
            MenuProjection::Hidden,
            844.0,
            true
        ));
        assert!(!title_context_is_left_aligned(
            MenuProjection::Hidden,
            820.0,
            true
        ));
    }

    #[test]
    fn title_bar_rules_remain_inside_the_browser_clip() {
        let rect = egui::Rect::from_min_size(egui::pos2(0.0, 7.0), egui::vec2(1_440.0, 35.0));
        let (top, bottom) = title_bar_separator_positions(rect);
        assert_eq!(top, 7.5);
        assert_eq!(bottom, 41.5);
        assert!(top > rect.top());
        assert!(bottom < rect.bottom());
    }

    #[test]
    fn each_projection_exposes_only_the_mockup_menu_prefix() {
        assert!(labels(MenuProjection::Hidden).is_empty());
        assert_eq!(
            labels(MenuProjection::ThroughSimulate),
            ["File", "Edit", "View", "Design", "Simulate"]
        );
        assert_eq!(
            labels(MenuProjection::ThroughModels),
            [
                "File", "Edit", "View", "Design", "Simulate", "Results", "Verify", "Models"
            ]
        );
        assert_eq!(
            labels(MenuProjection::All),
            [
                "File",
                "Edit",
                "View",
                "Design",
                "Simulate",
                "Results",
                "Verify",
                "Models",
                "Automation",
                "Window",
                "Help"
            ]
        );
    }

    #[test]
    fn overflow_and_compact_title_labels_match_the_mockup() {
        assert_eq!(
            MenuProjection::ThroughModels.overflow_trigger_label(),
            "⋯  More"
        );
        assert_eq!(
            MenuProjection::ThroughSimulate.overflow_trigger_label(),
            "⋯"
        );
        assert!(!MenuProjection::Hidden.has_overflow());
        assert!(!MenuProjection::Hidden.shows_title_context());
        assert_eq!(search_button_width(1020.0, false), 31.0);
        assert!(search_button_width(1021.0, false) > 31.0);
        assert_eq!(search_button_width(1440.0, true), 44.0);
    }

    #[test]
    fn menu_geometry_matches_the_mockup_contract() {
        assert_eq!(MENU_OUTER_WIDTH, 244.0);
        assert_eq!(MENU_CONTENT_WIDTH, 232.0);
        assert_eq!(MENU_ROW_HEIGHT, 27.0);
        assert_eq!(MENU_TOUCH_ROW_HEIGHT, 44.0);
        assert_eq!(MENU_MARGIN, 5.0);
        assert_eq!(MENU_BORDER_WIDTH, 1.0);
        assert_eq!(MENU_POPUP_GAP, 2.0);
        assert_eq!(SEARCH_KEYCAP_HEIGHT, 18.0);
        assert_eq!(SEARCH_KEYCAP_MIN_WIDTH, 19.0);
        assert_eq!(SEARCH_KEYCAP_INLINE_PADDING, 4.0);
        assert_eq!(SEARCH_KEYCAP_BORDER_WIDTH, 1.0);
        assert_eq!(
            overflow_trigger_width(MenuProjection::ThroughSimulate, false),
            28.0
        );
        assert_eq!(
            overflow_trigger_width(MenuProjection::ThroughModels, false),
            54.0
        );
        assert_eq!(
            overflow_trigger_width(MenuProjection::ThroughModels, true),
            44.0
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn desktop_menu_triggers_fill_the_title_track_without_vertical_clipping() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = title_test_app();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_440.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(Frame::NONE)
                    .show(ctx, |ui| {
                        menus(ui, &mut app, MenuProjection::All, false, 35.0);
                    });
            },
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        for label in ["File", "Edit", "View", "Design", "Simulate", "Help"] {
            let bounds = nodes
                .iter()
                .find(|(_, node)| {
                    node.role() == egui::accesskit::Role::MenuItem && node.label() == Some(label)
                })
                .and_then(|(_, node)| node.bounds())
                .unwrap_or_else(|| panic!("missing application menu trigger {label}"));
            assert_eq!(bounds.y1 - bounds.y0, 35.0, "clipped trigger {label}");
        }
    }

    #[test]
    fn command_keycap_uses_canonical_intrinsic_border_box_width() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut width = 0.0;
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                width = search_keycap_width(ui, "Ctrl K");
                let text_width = ui
                    .painter()
                    .layout_no_wrap(
                        "Ctrl K".to_owned(),
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        egui::Color32::WHITE,
                    )
                    .size()
                    .x;
                assert_eq!(
                    width,
                    (text_width
                        + SEARCH_KEYCAP_INLINE_PADDING * 2.0
                        + SEARCH_KEYCAP_BORDER_WIDTH * 2.0)
                        .max(SEARCH_KEYCAP_MIN_WIDTH)
                );
            });
        });
        assert!(width >= SEARCH_KEYCAP_MIN_WIDTH);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn f10_then_enter_opens_the_focused_application_menu() {
        let ctx = Context::default();
        let mut app = title_test_app();
        run_title_menu_frame(&ctx, &mut app, vec![title_key_event(Key::F10)]);
        run_title_menu_frame(&ctx, &mut app, Vec::new());

        let trigger_id = Id::new((
            "workbench.title_menu.trigger",
            MenuTriggerKey::Application(ApplicationMenu::File),
        ));
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(trigger_id));

        run_title_menu_frame(&ctx, &mut app, vec![title_key_event(Key::Enter)]);
        let popup_id = trigger_id.with("popup");
        assert!(Popup::is_id_open(&ctx, popup_id));
    }

    #[test]
    fn occurrence_specific_shortcuts_can_be_suppressed_or_overridden() {
        let shortcuts = crate::workbench::ShortcutPreferences::default();
        assert_eq!(
            shortcut_for_occurrence(
                &shortcuts,
                crate::workbench::commands::CommandPlatform::Desktop,
                egui::os::OperatingSystem::Windows,
                Command::KeyboardShortcuts,
                Some("")
            ),
            ""
        );
        assert_eq!(
            shortcut_for_occurrence(
                &shortcuts,
                crate::workbench::commands::CommandPlatform::Desktop,
                egui::os::OperatingSystem::Windows,
                Command::KeyboardShortcuts,
                Some("Ctrl+Alt+R")
            ),
            "Ctrl+Alt+R"
        );
        assert_eq!(DESCEND_MENU_LABEL, "Descend into selected instance…");
        assert_eq!(COMMAND_REFERENCE_MENU_LABEL, "Command reference");
    }

    #[test]
    fn menu_shortcuts_are_projected_from_the_typed_registry() {
        let platform = crate::workbench::commands::CommandPlatform::Desktop;
        let shortcuts = crate::workbench::ShortcutPreferences::default();
        for command in [
            Command::OpenProject,
            Command::Save,
            Command::RunSimulation,
            Command::ToggleFullScreen,
            Command::GenerateNetlist,
            Command::ToggleConsole,
        ] {
            assert_eq!(
                shortcut_for_occurrence(
                    &shortcuts,
                    platform,
                    egui::os::OperatingSystem::Windows,
                    command,
                    None
                ),
                command.default_shortcut_label(platform)
            );
        }
        assert_eq!(
            shortcut_for_occurrence(
                &shortcuts,
                platform,
                egui::os::OperatingSystem::Windows,
                Command::KeyboardShortcuts,
                None
            ),
            ""
        );
    }

    #[test]
    fn core_menu_commands_are_all_real_dispatch_commands() {
        let commands = [
            Command::OpenProject,
            Command::Save,
            Command::Undo,
            Command::PlaceWire,
            Command::PlaceJunction,
            Command::RunSimulation,
            Command::WaveformCalculator,
            Command::PdkSettings,
            Command::AutomationConsole,
        ];
        assert!(commands.iter().all(|command| !command.spec().id.is_empty()));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn results_menu_exposes_only_truthful_completed_result_workflows() {
        let labels = rendered_results_menu_labels();
        for expected in [
            "Open results workspace",
            "Bode / stability",
            "FFT / spectrum",
            "Eye diagram",
            "Distribution",
            "Operating point",
            "Noise contributors",
            "Measurements & specifications",
            "Nyquist",
            "Smith chart",
            "Pole-zero",
            "Calculator…",
            "Export result data…",
        ] {
            assert!(
                labels.iter().any(|label| label == expected),
                "missing completed Results route: {expected}"
            );
        }

        // These are specified by the mockup, but their complete product
        // contracts are broader than the currently retained executors. Keep
        // them absent rather than advertising a partial or differently scoped
        // action under the production label.
        for incomplete in [
            "Dataset and manifest browser…",
            "Create result document…",
            "Add result comparison…",
            "Report page and datasheet editor…",
            "Trace and family manager…",
            "Cursor and linked-probe manager…",
            "Plot markers and annotations…",
            "Review notes…",
            "Expression and unit diagnostics…",
            "Measurement library…",
            "Family slicing and pivot…",
            "Plot document properties…",
            "Measurement and calibration hub…",
            "Import result dataset…",
            "Export dataset…",
        ] {
            assert!(
                labels.iter().all(|label| label != incomplete),
                "partial mockup route was exposed: {incomplete}"
            );
        }
    }

    #[test]
    fn canonical_menu_icons_do_not_fall_back_to_generic_file_glyphs() {
        for (command, icon) in [
            (Command::NewProject, WorkbenchIcon::File),
            (Command::RevertActiveDocument, WorkbenchIcon::Refresh),
            (Command::CloseActiveDocument, WorkbenchIcon::File),
            (Command::CloseProject, WorkbenchIcon::Folder),
            (Command::Exit, WorkbenchIcon::Stop),
            (Command::Copy, WorkbenchIcon::Copy),
            (Command::Delete, WorkbenchIcon::Trash),
            (Command::SelectAll, WorkbenchIcon::Grid),
            (Command::ObjectProperties, WorkbenchIcon::Sliders),
            (Command::ResetActiveView, WorkbenchIcon::Refresh),
            (Command::AscendHierarchy, WorkbenchIcon::ArrowLeft),
            (Command::DescendHierarchy, WorkbenchIcon::Folder),
            (Command::PlaceInstance, WorkbenchIcon::Component),
            (Command::CompileVerilogA, WorkbenchIcon::Code),
            (Command::AutomationConsole, WorkbenchIcon::Terminal),
            (Command::KeyboardShortcuts, WorkbenchIcon::Search),
            (Command::InteroperabilityMatrix, WorkbenchIcon::Compare),
        ] {
            assert_eq!(command_icon(command), icon, "{}", command.spec().id);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn title_context_ellipsis_is_measured_instead_of_character_estimated() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut fitted = String::new();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let font = theme::sans(tokens::FS_0, FontWeight::Regular);
                fitted = ellipsize_to_width(
                    ui.painter(),
                    "WWW · precision-sensor-front-end",
                    &font,
                    52.0,
                );
                let width = ui
                    .painter()
                    .layout_no_wrap(fitted.clone(), font, egui::Color32::WHITE)
                    .size()
                    .x;
                assert!(width <= 52.0);
            });
        });
        assert!(fitted.ends_with('…'));
        assert_ne!(fitted, "WWW · precision-sensor-front-end");
    }
}
