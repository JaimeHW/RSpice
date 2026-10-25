//! Application title bar and complete implemented menu taxonomy.

use egui::{
    Align, Align2, Context, Frame, Id, Key, Layout, Modifiers, Panel, PointerButton, Popup, Rect,
    Response, Sense, Ui, Vec2,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::framework::host::{Host, Point};
use crate::workbench::framework::workbench_host::WorkbenchHost;

use super::super::layout::LayoutSpec;
use super::super::state::WorkbenchSession;
use crate::ui::panels::WorkbenchIcon;
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::command_registry::ids;

pub(in crate::workbench) mod application_menus;
pub(in crate::workbench) mod code;
pub(in crate::workbench) mod design;
mod keyboard;
pub(in crate::workbench) mod menu_entries;
mod menu_row;
#[cfg(test)]
use application_menus::menu_popup_height_for_viewport;
pub(in crate::workbench) use application_menus::source_editor_context_menu;
use application_menus::{
    ALL_MENUS, ApplicationMenu, configure_menu_popup, file_menu_shows_exit, menu_popup_max_height,
    shortcut_for_occurrence,
};
use keyboard::{
    InitialMenuFocus, MENU_RENDER_STATE_ID, MenuRenderState, MenuRowRecord, begin_menu_render,
    claim_menu_keys, finish_menu_render, handle_menu_bar_keyboard, handle_popup_keyboard,
    record_menu_row, step_menu_row_focus, typeahead_menu_row_focus,
};
use menu_entries::{MenuEntry, menu_rows, show_scrolled_menu_rows};
use menu_row::{MenuRowLead, MenuRowTrailing, paint_menu_row};

/// The narrowest a menu popup is; it widens to fit its longest row up to
/// [`MENU_MAX_OUTER_WIDTH`].
const MENU_OUTER_WIDTH: f32 = 244.0;
const MENU_MAX_OUTER_WIDTH: f32 = 360.0;
const MENU_MARGIN: f32 = 5.0;
const MENU_BORDER_WIDTH: f32 = 1.0;
const MENU_CONTENT_WIDTH: f32 = MENU_OUTER_WIDTH - 2.0 * (MENU_MARGIN + MENU_BORDER_WIDTH);
const MENU_MAX_CONTENT_WIDTH: f32 = MENU_MAX_OUTER_WIDTH - 2.0 * (MENU_MARGIN + MENU_BORDER_WIDTH);
const MENU_ROW_HEIGHT: f32 = 27.0;
const MENU_TOUCH_ROW_HEIGHT: f32 = 44.0;
pub(super) const MENU_POPUP_GAP: f32 = 2.0;
const SEARCH_KEYCAP_HEIGHT: f32 = 18.0;
const SEARCH_KEYCAP_MIN_WIDTH: f32 = 19.0;
const SEARCH_KEYCAP_INLINE_PADDING: f32 = 4.0;
const SEARCH_KEYCAP_BORDER_WIDTH: f32 = 1.0;
const BRAND_WORDMARK_WIDTH: f32 = 88.0;

pub fn show(
    root: &mut Ui,
    app: &mut impl WorkbenchHost<Menu = ApplicationMenu, MenuEntry = MenuEntry>,
    layout: LayoutSpec,
) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    let t = Tokens::get(ctx);
    let viewport_width = ctx.content_rect().width();
    let menu_projection = MenuProjection::for_layout(viewport_width, layout.compact_shell);
    let large_targets = viewport_width <= 820.0 || layout.coarse_pointer;
    Panel::top("workbench.title_bar")
        .exact_size(layout.title_bar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(false)
        .show(root, |ui| {
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
                        if account_action(ui, account_initials.as_deref(), large_targets) {
                            app.execute(ids::ACCOUNT_ORGANIZATION);
                        }
                        if notification_action(
                            ui,
                            app.workbench().toasts.unread_count(),
                            large_targets,
                            panel_hangs_here(ui.ctx(), app.workbench()),
                        ) {
                            press_bell(ui.ctx(), app.workbench_mut());
                        }
                        if icon_action(
                            ui,
                            WorkbenchIcon::Settings,
                            "Open preferences",
                            large_targets,
                        ) {
                            app.execute(ids::PREFERENCES);
                        }
                    }
                    if search_button(ui, app, viewport_width, large_targets) {
                        app.execute(ids::COMMAND_PALETTE);
                    }
                    app.contribute(Point::Presence { large_targets }, ui);
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
                    title_context_is_left_aligned(viewport_width, layout.compact_shell),
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

fn title_context_is_left_aligned(viewport_width: f32, compact_shell: bool) -> bool {
    compact_shell && viewport_width > 820.0
}

/// How the bar offers the application menus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuProjection {
    /// One Menu button holding every menu as a submenu, with touch rows:
    /// phone widths and the compact shell.
    Compact,
    /// Every menu on the bar. It never hides one: where space is short,
    /// the centred title shortens instead.
    All,
}

impl MenuProjection {
    const fn for_layout(viewport_width: f32, compact_shell: bool) -> Self {
        if compact_shell || viewport_width <= 820.0 {
            Self::Compact
        } else {
            Self::All
        }
    }

    const fn visible_menus(self) -> &'static [ApplicationMenu] {
        match self {
            Self::Compact => &[],
            Self::All => &ALL_MENUS,
        }
    }

    const fn shows_title_context(self) -> bool {
        matches!(self, Self::All)
    }
}

fn brand(
    ui: &mut Ui,
    app: &mut impl WorkbenchHost,
    show_wordmark: bool,
    height: f32,
    large_target: bool,
    retain_desktop_width: bool,
) {
    let t = Tokens::get(ui.ctx());
    let width = if show_wordmark || retain_desktop_width {
        // Icon (20), 7 px lockup gap, measured wordmark, and 10 px end
        // padding. The previous 112 px minimum came from the HTML mock's
        // generic lockup rule and left a conspicuous empty block before File.
        BRAND_WORDMARK_WIDTH
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
        app.execute(ids::PROJECT_WORKSPACE);
    }
    theme::paint_focus_ring_outset(ui, &response, rect);
    response.on_hover_text("RSpice project overview");
}

fn menus(
    ui: &mut Ui,
    app: &mut impl WorkbenchHost<Menu = ApplicationMenu, MenuEntry = MenuEntry>,
    projection: MenuProjection,
    large_targets: bool,
    title_bar_height: f32,
) {
    let bar_width = menu_bar_width(ui, projection);
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
                triggers.push(top_menu(ui, app, menu, large_targets, title_bar_height));
            }
            if projection == MenuProjection::Compact {
                triggers.push(compact_menu_button(ui, app, title_bar_height));
            }
        },
    );
    ui.ctx()
        .accesskit_node_builder(menu_bar.response.id, |node| {
            node.set_role(egui::accesskit::Role::Menu);
            node.set_label("Application menu");
        });
    handle_menu_bar_keyboard(ui.ctx(), &triggers, !app.application_modal_open());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MenuTriggerKey {
    Application(ApplicationMenu),
    /// The compact layout's one button, holding every menu.
    Compact,
}

impl MenuTriggerKey {
    fn label(self) -> &'static str {
        match self {
            Self::Application(menu) => menu.label(),
            Self::Compact => COMPACT_MENU_LABEL,
        }
    }
}

const COMPACT_MENU_LABEL: &str = "Menu";

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

fn menu_bar_width(ui: &Ui, projection: MenuProjection) -> f32 {
    match projection {
        MenuProjection::All => projection
            .visible_menus()
            .iter()
            .map(|menu| menu_trigger_width(ui, menu.label()))
            .sum::<f32>(),
        MenuProjection::Compact => menu_trigger_width(ui, COMPACT_MENU_LABEL),
    }
}

/// A bar trigger's width: its label and 7 pt either side.
fn menu_trigger_width(ui: &Ui, label: &str) -> f32 {
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    ui.painter()
        .layout_no_wrap(label.to_owned(), font, egui::Color32::WHITE)
        .size()
        .x
        + 14.0
}

fn top_menu(
    ui: &mut Ui,
    app: &mut impl WorkbenchHost<Menu = ApplicationMenu, MenuEntry = MenuEntry>,
    menu: ApplicationMenu,
    large_targets: bool,
    title_bar_height: f32,
) -> MenuTriggerResponse {
    let width = menu_trigger_width(ui, menu.label());
    let entries = app.menu_entries(menu);
    menu_trigger(
        ui,
        app,
        MenuTriggerKey::Application(menu),
        MenuTriggerPresentation {
            width,
            height: title_bar_height,
            large_targets,
        },
        entries,
    )
}

/// The compact layout's Menu button: every application menu as a submenu,
/// on touch rows whatever the pointer, since the layout is a touch one.
fn compact_menu_button(
    ui: &mut Ui,
    app: &mut impl WorkbenchHost<Menu = ApplicationMenu, MenuEntry = MenuEntry>,
    title_bar_height: f32,
) -> MenuTriggerResponse {
    let width = menu_trigger_width(ui, COMPACT_MENU_LABEL).max(MENU_TOUCH_ROW_HEIGHT);
    let entries = app.compact_menu();
    menu_trigger(
        ui,
        app,
        MenuTriggerKey::Compact,
        MenuTriggerPresentation {
            width,
            height: title_bar_height,
            large_targets: true,
        },
        entries,
    )
}

fn menu_trigger(
    ui: &mut Ui,
    app: &mut impl WorkbenchHost,
    key: MenuTriggerKey,
    presentation: MenuTriggerPresentation,
    entries: &[MenuEntry],
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
            0.0,
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
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        key.label(),
        theme::sans(tokens::FS_1, FontWeight::Regular),
        ink,
    );
    theme::paint_focus_ring_outset(ui, &response, rect);

    let popup = Popup::menu(&response).gap(MENU_POPUP_GAP).show(|ui| {
        let rows = menu_rows(entries, app, ui.ctx());
        configure_menu_popup(ui, large_targets, &rows);
        ui.ctx().accesskit_node_builder(popup_id, |node| {
            node.set_role(egui::accesskit::Role::Menu);
            node.set_label(format!("{} menu", key.label()));
        });
        begin_menu_render(ui.ctx(), popup_id);
        let choice = show_scrolled_menu_rows(ui, key, &rows);
        let render_state = finish_menu_render(ui.ctx());
        handle_popup_keyboard(ui.ctx(), popup_id, &response, &render_state.rows);
        choice
    });
    let popup_shown = popup.is_some();
    if let Some(choice) = popup.and_then(|shown| shown.inner) {
        choice.run(app);
    }
    if keyboard_focus.is_some() || (response.hovered() && another_application_menu_is_open) {
        // Opening after `Popup::show` keeps egui's toggle policy from closing a
        // custom keyboard/hover request during the same pass. Memory owns the
        // popup from the next pass onward, exactly like adjacent-menu
        // navigation handled after all triggers render.
        Popup::open_id(ui.ctx(), popup_id);
    }
    let is_open = popup_shown || Popup::is_id_open(ui.ctx(), popup_id);
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
            Id::new(("workbench.title_menu.trigger", MenuTriggerKey::Compact)).with("popup"),
        ))
        .collect()
}

fn paint_title_context(
    ui: &mut Ui,
    app: &impl WorkbenchHost,
    bounds: egui::Rect,
    compact: bool,
    left_aligned: bool,
) {
    if bounds.width() < 40.0 {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let dirty = app.platform().project_lifecycle.is_open()
        && (app.model().schematic.session.is_dirty || app.model().workspace.any_dirty());
    let cell = active_title_cell(app);
    let full = title_context_text(app, compact);
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let clip = bounds.shrink2(egui::vec2(5.0, 0.0));
    let painter = ui.painter().with_clip_rect(clip);
    let text_origin = if left_aligned {
        bounds.left() + 12.0
    } else {
        bounds.left() + 5.0
    };
    let available_text_width = (clip.right() - text_origin).max(12.0);
    // Unsaved project changes are a `*` after the title, as Virtuoso and ADS
    // mark them. The marker is set aside before the name is shortened, so a
    // narrow title loses letters of the name, never the marker.
    let marker_width = if dirty {
        painter
            .layout_no_wrap(UNSAVED_TITLE_MARKER.to_owned(), font.clone(), t.color.text)
            .size()
            .x
    } else {
        0.0
    };
    let mut text = ellipsize_to_width(
        &painter,
        &full,
        &font,
        (available_text_width - marker_width).max(12.0),
    );
    if dirty {
        text.push_str(UNSAVED_TITLE_MARKER);
    }
    let galley = painter.layout_no_wrap(text, font, t.color.text);
    let total_width = galley.size().x;
    let left = if left_aligned {
        text_origin
    } else {
        (bounds.center().x - total_width * 0.5).max(bounds.left() + 5.0)
    };
    painter.galley(
        egui::pos2(left, bounds.center().y - galley.size().y * 0.5),
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
        if app.platform().project_lifecycle.is_open() {
            node.set_label(format!(
                "Active project: {}; {}; {}",
                app.model().workspace.content.project.display_name(),
                cell,
                if dirty {
                    "unsaved changes"
                } else {
                    "all changes saved"
                }
            ));
        } else {
            node.set_label("No project open");
        }
    });
}

/// What follows the title while the project has unsaved changes.
const UNSAVED_TITLE_MARKER: &str = "*";

/// The text the centred title paints.
///
/// This carried `#[cfg(test)]` while the painter above inlined only its
/// project-name branch, so the closed-project rule was asserted by a test
/// against a function the application never called and the shipped title read
/// "Untitled Project" with nothing open.
fn title_context_text(app: &impl WorkbenchHost, compact: bool) -> String {
    if !app.platform().project_lifecycle.is_open() {
        return if compact {
            "No project open"
        } else {
            "RSpice Workbench"
        }
        .to_owned();
    }
    if compact {
        active_title_cell(app)
    } else {
        // On desktop the active cell/view already appears in both the
        // document tab and the local canvas breadcrumb. Keep the centered
        // title scoped to project identity so the shell does not repeat the
        // same context three times.
        app.model()
            .workspace
            .content
            .project
            .display_name()
            .to_owned()
    }
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

fn active_title_cell(app: &impl WorkbenchHost) -> String {
    if !app.platform().project_lifecycle.is_open() {
        return "No project open".to_owned();
    }
    app.title_cell(app.workbench().workspace)
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

fn search_button(
    ui: &mut Ui,
    app: &impl WorkbenchHost,
    viewport_width: f32,
    large_target: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let shortcut = app.shortcut_label(
        ids::COMMAND_PALETTE,
        crate::workbench::app_state::session::shortcuts::runtime_command_platform(ui.ctx()),
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
            theme::sans(tokens::FS_1, FontWeight::Regular),
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
            theme::mono(tokens::FS_1, FontWeight::Regular),
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
            theme::mono(tokens::FS_1, FontWeight::Regular),
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

/// The initials the account button shows, or `None` when no one is signed
/// in and no license names a holder: the button then shows a generic
/// person, not letters that name no one.
fn account_initials(app: &impl WorkbenchHost) -> Option<String> {
    let name = app.account_holder()?;
    let words = name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<Vec<_>>();
    match words.as_slice() {
        [] => None,
        [first] => Some(first.to_uppercase().collect()),
        [first, .., last] => Some(format!("{}{}", first.to_uppercase(), last.to_uppercase())),
    }
}

fn account_action(ui: &mut Ui, initials: Option<&str>, large_target: bool) -> bool {
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
    let ink = if response.hovered() || response.has_focus() {
        t.color.text
    } else {
        t.color.text_dim
    };
    match initials {
        Some(initials) => {
            ui.painter().text(
                avatar_rect.center(),
                Align2::CENTER_CENTER,
                initials,
                theme::sans(tokens::FS_0, FontWeight::SemiBold),
                ink,
            );
        }
        // Painted as a vector, inset so the head and shoulders sit inside
        // the ring.
        None => WorkbenchIcon::User.paint(
            ui.painter(),
            Rect::from_center_size(avatar_rect.center(), avatar_rect.size() * 0.65),
            ink,
        ),
    }
    theme::paint_focus_ring_outset(ui, &response, target_rect);
    response.on_hover_text(label).clicked()
}

/// Where the notification panel hangs from: the bell's rectangle as of the
/// last pass that drew one, and the bell's identity so closing the panel from
/// the keyboard can hand focus back to it. `None` wherever the title bar draws
/// no bell — a phone-width shell, full-screen presentation — and the panel
/// then anchors itself to the shell's trailing edge. Each window has its own
/// bell, so it is the bell of the window `ctx` is drawing.
pub(crate) fn notification_anchor(ctx: &egui::Context) -> Option<(Rect, egui::Id)> {
    let anchor_id = notification_anchor_id(ctx);
    let (pass, rect, id) = ctx.data(|data| data.get_temp::<(u64, Rect, egui::Id)>(anchor_id))?;
    // A rectangle from an older pass is a bell that is no longer on screen.
    (ctx.cumulative_pass_nr() <= pass + 1).then_some((rect, id))
}

const NOTIFICATION_BADGE_HEIGHT: f32 = 12.0;

fn notification_anchor_id(ctx: &egui::Context) -> egui::Id {
    egui::Id::new("rspice.title-bar.notification-anchor").with(ctx.viewport_id())
}

/// Which window's bell the open notification panel hangs from. The panel is
/// drawn in that window only, and claims one when no bell opened it.
pub(in crate::workbench) fn notification_window_id() -> egui::Id {
    egui::Id::new("rspice.title-bar.notification-window")
}

/// Whether the open panel hangs from the bell of the window `ctx` is drawing.
fn panel_hangs_here(ctx: &egui::Context, workbench: &WorkbenchSession) -> bool {
    let here = ctx.viewport_id();
    workbench.notification_center_showing()
        && ctx.data(|data| data.get_temp::<egui::ViewportId>(notification_window_id()))
            == Some(here)
}

/// The bell of the window `ctx` is drawing was pressed. The panel opens under
/// it, comes to it from another window's bell, or, hanging here already, goes
/// away.
pub(crate) fn press_bell(ctx: &egui::Context, workbench: &mut WorkbenchSession) {
    let here = ctx.viewport_id();
    if !workbench.notification_center_showing() || panel_hangs_here(ctx, workbench) {
        workbench.toggle_notification_center();
    }
    ctx.data_mut(|data| data.insert_temp(notification_window_id(), here));
}

fn notification_action(
    ui: &mut Ui,
    unread_count: usize,
    large_target: bool,
    showing: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let size = if large_target {
        Vec2::splat(44.0)
    } else {
        Vec2::new(28.0, 27.0)
    };
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let pass = ui.ctx().cumulative_pass_nr();
    let anchor_id = notification_anchor_id(ui.ctx());
    ui.ctx().data_mut(|data| {
        data.insert_temp(anchor_id, (pass, rect, response.id));
    });
    let accessible_label = if unread_count == 0 {
        "Notifications".to_owned()
    } else {
        format!("Notifications · {unread_count} unread")
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            accessible_label.clone(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_expanded(showing);
    });
    let active = showing || response.hovered() || response.has_focus();
    if showing {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_active);
    } else if response.hovered() {
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
        // The count rides the bell's shoulder rather than sitting on its
        // face: a badge the size of the icon leaves a number with no bell
        // under it, and the bell is what says the number is notifications.
        let badge_font = theme::mono(tokens::FS_MICRO, FontWeight::SemiBold);
        let badge_text =
            ui.painter()
                .layout_no_wrap(unread_count.to_string(), badge_font, t.color.accent_ink);
        let badge_width = (badge_text.size().x + 6.0).max(NOTIFICATION_BADGE_HEIGHT);
        let badge_rect = egui::Rect::from_min_size(
            egui::pos2(rect.right() - 2.0 - badge_width, rect.top() + 2.0),
            egui::vec2(badge_width, NOTIFICATION_BADGE_HEIGHT),
        );
        // A ring of whatever is behind the button, so the badge reads as
        // standing off the bell instead of being stuck to it.
        let surface = if showing {
            t.color.bg_active
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_panel
        };
        let radius = NOTIFICATION_BADGE_HEIGHT * 0.5 + 1.0;
        ui.painter()
            .rect_filled(badge_rect.expand(1.0), radius, surface);
        ui.painter()
            .rect_filled(badge_rect, radius - 1.0, t.color.accent);
        ui.painter().galley(
            badge_rect.center() - badge_text.size() * 0.5,
            badge_text,
            t.color.accent_ink,
        );
    }
    theme::paint_focus_ring_outset(ui, &response, rect);
    if showing {
        // The panel hangs where the tooltip would open, and already says it.
        return response.clicked();
    }
    response.on_hover_text(accessible_label).clicked()
}

#[cfg(test)]
mod tests;
