//! The Preferences shell: a modal with a category rail and a settings grid.
//!
//! Preferences are not a generic form dialog, so the composition lives here
//! rather than being rebuilt per page: a fixed header and footer, a grouped
//! category rail that becomes a selector on narrow and touch layouts, and one
//! setting-row grid every page writes into.
//!
//! The grid is the whole design. A setting is a label, an optional line of
//! help under it, and a control — and the control starts at the same x on
//! every row of every page, so a page reads as one column of labels against
//! one column of controls instead of a list of individually centred widgets.
//! Values are left-aligned at that x for the same reason a schematic property
//! editor aligns them: a reader comparing two rows compares their first
//! character, not the gap in front of it.

use egui::{
    Context, Id, Order, Popup, Rect, Sense, Stroke, Ui, UiKind, WidgetInfo, WidgetType, vec2,
};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, IconButton, SWITCH_WIDTH, paint_switch, select};

const DESKTOP_WIDTH: f32 = 1000.0;
const DESKTOP_HEIGHT: f32 = 680.0;
const DESKTOP_HORIZONTAL_INSET: f32 = 28.0;
const DESKTOP_VERTICAL_INSET: f32 = 34.0;
const PHONE_INSET: f32 = 8.0;
const PHONE_MAX_WIDTH: f32 = 560.0;
const NARROW_MAX_WIDTH: f32 = 820.0;
const HEADER_HEIGHT: f32 = 44.0;
const FOOTER_HEIGHT: f32 = 48.0;
const CATEGORY_RAIL_WIDTH: f32 = 196.0;
const CATEGORY_ROW_HEIGHT: f32 = 30.0;
const CATEGORY_GROUP_HEIGHT: f32 = 28.0;
/// The smaller of the two minimum touch targets Accessibility offers, used
/// when no session has said which one the reader picked.
const DEFAULT_TOUCH_TARGET: f32 = 44.0;
const FOOTER_ITEM_GAP: f32 = 8.0;

/// One inset for the header, every page row and the footer, so a title, a
/// label and a button all start on the same vertical line.
const CONTENT_INSET: f32 = 18.0;
/// The same inset where a phone has given up a quarter of its width to it.
const PHONE_CONTENT_INSET: f32 = 14.0;
/// Width of the label column a page's controls line up against.
///
/// Wide enough that most of a row's help is one line: a column that wraps
/// every other row's help to two lines makes the list's rhythm depend on
/// sentence length rather than on what is on the row.
const LABEL_COLUMN_WIDTH: f32 = 400.0;
/// Gap between the label column and the control that answers it.
const SETTING_ROW_COLUMN_GAP: f32 = 20.0;
/// Widest a select or numeric field grows to, so neighbouring rows agree.
const CONTROL_MAX_WIDTH: f32 = 300.0;
/// Padding above and below a row's tallest element.
///
/// The density's own bar padding rather than a number of this dialog's own:
/// a 28 px control inside nine pixels either side is a row that is half air,
/// and the design system already states how much space a control of this
/// density sits in.
fn setting_row_padding(ui: &Ui) -> f32 {
    Tokens::get(ui.ctx()).metrics.bar_pad
}
/// Gap between a row's label and the help under it.
const SETTING_COPY_HELP_GAP: f32 = 2.0;
/// Gap between a stacked label and its control on a phone.
const SETTING_ROW_PHONE_VALUE_GAP: f32 = 8.0;
const SHELL_ID: &str = "rspice.preferences.shell";
/// Height the rail gives the filter field, including the space around it.
const SEARCH_FIELD_BAND: f32 = 12.0;

/// The settings filter.
///
/// Forty-odd settings across eight categories is past the point where a reader
/// can be expected to know which page holds the one they want, so a query
/// searches every page at once and the content area answers with a flat list
/// of matching rows. Nothing here indexes the pages: a row's own label and
/// help are what it is matched on, as it renders, which is why the list cannot
/// drift out of step with the settings that exist.
#[derive(Debug, Clone, Default)]
struct SettingFilter {
    /// What the reader typed. Persists across frames.
    query: String,
    /// Per-pass scratch: the category whose heading has not been painted yet.
    /// The body sets it before each page and the first matching row on that
    /// page takes it, so a page that matches nothing announces nothing.
    pending_category: Option<&'static str>,
    /// Per-pass scratch: how many rows matched, to tell an empty result from
    /// a result the reader has simply not scrolled to.
    matches: usize,
}

fn filter_id() -> Id {
    Id::new(SHELL_ID).with("settings-filter")
}

fn read_filter(ctx: &Context) -> SettingFilter {
    ctx.data(|data| data.get_temp::<SettingFilter>(filter_id()))
        .unwrap_or_default()
}

fn write_filter(ctx: &Context, filter: SettingFilter) {
    ctx.data_mut(|data| data.insert_temp(filter_id(), filter));
}

/// The query every row is tested against, or `None` when the pages are being
/// read normally and no row is to be hidden.
fn active_query(ctx: &Context) -> Option<String> {
    let query = read_filter(ctx).query.trim().to_lowercase();
    (!query.is_empty()).then_some(query)
}

/// Whether a row answers `query`, on its label, the help under it, or one of
/// its `terms`: the labels of the options it offers and the words a reader
/// types for it ("dark" for a colour mode, "zoom" for a scale).
///
/// The query is folded here rather than trusted to arrive folded: a predicate
/// that is only correct when its caller remembered to lowercase is one
/// refactor away from matching nothing, and silently.
fn row_matches(title: &str, help: &str, terms: &[&str], query: &str) -> bool {
    let query = query.trim().to_lowercase();
    [title, help]
        .into_iter()
        .chain(terms.iter().copied())
        .any(|text| text.to_lowercase().contains(&query))
}

/// The two things a reader is looking for when they open Preferences: how the
/// application behaves, or how engineering quantities are read and written.
///
/// A rail of ten entries does not need groups to be navigable, but it does
/// need them to be *predictable*: a reader after the grid size looks under
/// engineering, and one after the colour mode does not read past Interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CategoryGroup {
    Interface,
    Engineering,
}

impl CategoryGroup {
    const fn label(self) -> &'static str {
        match self {
            Self::Interface => "Interface",
            Self::Engineering => "Engineering",
        }
    }
}

/// The preference pages this build can actually configure.
///
/// Three categories were removed rather than shipped empty: Security and
/// privacy and Plug-ins and integrations each rendered a heading and a
/// sentence saying nothing on the page was configurable, and Remote compute
/// held one read-only count of the host's logical processors. A rail entry
/// that leads to an absence advertises the absence; these come back when a
/// credential store, an extension loader or an execution service does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) enum PreferenceCategory {
    #[default]
    Appearance,
    Accessibility,
    Workspace,
    Shortcuts,
    Units,
    Schematic,
    Projects,
    Results,
    Simulation,
    Navigation,
}

impl PreferenceCategory {
    pub(super) const ALL: [Self; 10] = [
        Self::Appearance,
        Self::Accessibility,
        Self::Workspace,
        Self::Navigation,
        Self::Shortcuts,
        Self::Units,
        Self::Simulation,
        Self::Schematic,
        Self::Projects,
        Self::Results,
    ];

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Appearance => "Appearance",
            Self::Accessibility => "Accessibility",
            Self::Workspace => "Workspace",
            Self::Shortcuts => "Shortcuts",
            Self::Units => "Units and notation",
            Self::Schematic => "Schematic editor",
            Self::Projects => "Projects and files",
            Self::Results => "Results and precision",
            Self::Simulation => "Simulation",
            Self::Navigation => "Navigation",
        }
    }

    pub(super) const fn group(self) -> CategoryGroup {
        match self {
            Self::Appearance
            | Self::Accessibility
            | Self::Workspace
            | Self::Navigation
            | Self::Shortcuts => CategoryGroup::Interface,
            Self::Units | Self::Simulation | Self::Schematic | Self::Projects | Self::Results => {
                CategoryGroup::Engineering
            }
        }
    }

    pub(super) const fn stable_id(self) -> &'static str {
        match self {
            Self::Appearance => "appearance",
            Self::Accessibility => "accessibility",
            Self::Workspace => "workspace",
            Self::Shortcuts => "shortcuts",
            Self::Units => "units",
            Self::Schematic => "schematic",
            Self::Projects => "projects",
            Self::Results => "results",
            Self::Simulation => "simulation",
            Self::Navigation => "navigation",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct PreferencesShellResponse {
    pub(super) close_requested: bool,
    /// The footer's page-scoped reset. The shell owns the control because the
    /// footer is shell geometry; the page owns which settings it covers.
    pub(super) restore_defaults_requested: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct PreferencesFocusState {
    prior_focus: Option<Id>,
    last_seen_pass: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PreferencesLayout {
    surface: Rect,
    narrow: bool,
    phone: bool,
}

impl PreferencesLayout {
    fn resolve(viewport: Rect) -> Self {
        let phone = viewport.width() <= PHONE_MAX_WIDTH;
        let (width, height) = if phone {
            (
                (viewport.width() - PHONE_INSET).max(1.0),
                (viewport.height() - PHONE_INSET).max(1.0),
            )
        } else {
            (
                DESKTOP_WIDTH.min((viewport.width() - DESKTOP_HORIZONTAL_INSET).max(1.0)),
                DESKTOP_HEIGHT.min((viewport.height() - DESKTOP_VERTICAL_INSET).max(1.0)),
            )
        };
        Self {
            surface: Rect::from_center_size(viewport.center(), vec2(width, height)),
            narrow: viewport.width() <= NARROW_MAX_WIDTH,
            phone,
        }
    }
}

fn touch_target_id() -> Id {
    Id::new(SHELL_ID).with("touch-target")
}

/// The minimum touch target the reader chose under Accessibility, as the
/// shell was last told it. Every raised control in the dialog reads this one
/// number, so the close button, the switches and the rows agree with the
/// rest of the application on a coarse pointer.
fn touch_target(ctx: &Context) -> f32 {
    ctx.data(|data| data.get_temp::<f32>(touch_target_id()))
        .unwrap_or(DEFAULT_TOUCH_TARGET)
}

/// Temporarily raises controls to the touch target on narrow and coarse
/// layouts.
struct LargeControlTargetOverride {
    ctx: Context,
    original: Option<Tokens>,
}

impl LargeControlTargetOverride {
    fn new(ctx: &Context, enabled: bool) -> Self {
        let original = (*Tokens::get(ctx)).clone();
        let target = touch_target(ctx);
        if !enabled || original.metrics.ctl_h >= target {
            return Self {
                ctx: ctx.clone(),
                original: None,
            };
        }
        let mut adjusted = original.clone();
        adjusted.metrics.ctl_h = target;
        adjusted.install(ctx);
        Self {
            ctx: ctx.clone(),
            original: Some(original),
        }
    }
}

impl Drop for LargeControlTargetOverride {
    fn drop(&mut self) {
        if let Some(original) = self.original.take() {
            original.install(&self.ctx);
        }
    }
}

/// Render the complete Preferences shell around one category page.
///
/// `touch_target` is the minimum target from Accessibility, which the shell
/// raises its controls to on narrow and coarse layouts.
pub(super) fn show(
    ctx: &Context,
    category: &mut PreferenceCategory,
    allow_escape_close: bool,
    touch_target: f32,
    render_page: impl FnMut(&mut Ui, PreferenceCategory),
) -> PreferencesShellResponse {
    ctx.data_mut(|data| data.insert_temp(touch_target_id(), touch_target));
    let viewport = ctx.content_rect();
    let layout = PreferencesLayout::resolve(viewport);
    // The mockup raises interactive targets at either side of its combined
    // `max-width: 820px, pointer: coarse` media query. This deliberately also
    // covers a narrow desktop window controlled by a mouse.
    let large_targets = layout.narrow || ctx.input(|input| input.has_touch_screen());
    let _target_override = LargeControlTargetOverride::new(ctx, large_targets);
    let t = Tokens::get(ctx);
    let c = t.color;
    let area_id = Id::new(SHELL_ID);
    let focus_state_id = area_id.with("focus-state");
    let opened_this_pass = begin_focus_session(ctx, focus_state_id);
    let popup_was_open = Popup::is_any_open(ctx);
    let area = egui::Area::new(area_id)
        .kind(UiKind::Modal)
        .order(Order::Foreground)
        .fixed_pos(viewport.min)
        .sense(Sense::focusable_noninteractive());
    let modal_layer = area.layer();
    ctx.memory_mut(|memory| memory.set_modal_layer(modal_layer));

    let mut response = PreferencesShellResponse::default();
    let mut close_control_id = None;
    let mut initial_focus_control_id = None;
    let mut dialog_surface_id = None;
    let _ = area.show(ctx, |ui| {
        // accessibility-pointer-shim: the modal backdrop consumes pointer
        // gestures but is deliberately absent from keyboard and AT order.
        ui.allocate_rect(
            viewport,
            Sense::click_and_drag().difference(Sense::focusable_noninteractive()),
        );
        let backdrop = if t.mode == tokens::Mode::Dark {
            egui::Color32::from_rgba_unmultiplied(2, 6, 8, 158)
        } else {
            egui::Color32::from_rgba_unmultiplied(41, 46, 50, 97)
        };
        ui.painter().rect_filled(viewport, 0.0, backdrop);
        ui.painter().add(t.shadow().as_shape(layout.surface, 4.0));
        ui.painter().rect(
            layout.surface,
            4.0,
            c.bg_app,
            Stroke::new(1.0, c.border_strong),
            egui::StrokeKind::Inside,
        );
        let mut dialog_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("dialog-surface")
                .max_rect(layout.surface)
                .layout(egui::Layout::top_down(egui::Align::Min))
                .sense(Sense::focusable_noninteractive()),
        );
        dialog_ui.set_min_size(layout.surface.size());
        let mut surface = dialog_ui.new_child(
            egui::UiBuilder::new()
                .id_salt("dialog-content")
                .max_rect(layout.surface.shrink(1.0))
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        let inner = surface.max_rect();
        surface.set_min_size(inner.size());
        let header = Rect::from_min_max(
            inner.min,
            egui::pos2(inner.right(), inner.top() + HEADER_HEIGHT),
        );
        let footer = Rect::from_min_max(
            egui::pos2(inner.left(), inner.bottom() - FOOTER_HEIGHT),
            inner.max,
        );
        let body = Rect::from_min_max(
            egui::pos2(inner.left(), header.bottom()),
            egui::pos2(inner.right(), footer.top()),
        );

        let header_response = render_header(&mut surface, header, large_targets);
        close_control_id = Some(header_response.id);
        if header_response.clicked() {
            response.close_requested = true;
        }
        initial_focus_control_id = render_body(&mut surface, body, layout, category, render_page);
        let footer_response = render_footer(&mut surface, footer);
        response.close_requested |= footer_response.close;
        response.restore_defaults_requested = footer_response.restore_defaults;
        let dialog_response = dialog_ui.response();
        dialog_response
            .widget_info(|| WidgetInfo::labeled(WidgetType::Window, true, "Preferences"));
        dialog_surface_id = Some(dialog_response.id);
    });
    if let Some(dialog_surface_id) = dialog_surface_id {
        ctx.accesskit_node_builder(dialog_surface_id, |node| {
            node.set_role(egui::accesskit::Role::Dialog);
            node.set_label("Preferences");
            node.set_bounds(egui::accesskit::Rect {
                x0: f64::from(layout.surface.left()),
                y0: f64::from(layout.surface.top()),
                x1: f64::from(layout.surface.right()),
                y1: f64::from(layout.surface.bottom()),
            });
            node.set_description(
                "Personal, device, accessibility, and recovery preferences. Changes apply immediately; project settings remain versioned.",
            );
            node.set_modal();
        });
    }

    if allow_escape_close
        && !popup_was_open
        && ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
    {
        // Escape undoes the narrowing before it closes the dialog. Dropping a
        // filter and dismissing the surface the filter was applied to are not
        // the same request, and losing the second to the first would make a
        // reader reopen Preferences to get out of a search.
        if active_query(ctx).is_some() {
            clear_filter(ctx);
        } else {
            response.close_requested = true;
        }
    }
    if let Some(close_control_id) = close_control_id {
        if opened_this_pass || !focus_is_within_modal(ctx, modal_layer) {
            ctx.memory_mut(|memory| {
                memory.request_focus(initial_focus_control_id.unwrap_or(close_control_id));
            });
        }
        if response.close_requested {
            Popup::close_all(ctx);
            restore_focus(ctx, focus_state_id, close_control_id, modal_layer);
        }
    }
    response
}

/// End a Preferences focus session when canonical navigation removes the
/// route without going through the dialog's Close control (for example,
/// browser Back). This mirrors normal modal teardown instead of leaving a
/// hidden control focused for the next shortcut-resolution pass.
pub(super) fn unmount(ctx: &Context) {
    // A query is a reading position, not a stored preference: Preferences
    // opens on the pages, never on someone's last search.
    ctx.data_mut(|data| data.remove_temp::<SettingFilter>(filter_id()));
    let area_id = Id::new(SHELL_ID);
    let modal_layer = egui::LayerId::new(Order::Foreground, area_id);
    let focus_state =
        ctx.data_mut(|data| data.remove_temp::<PreferencesFocusState>(area_id.with("focus-state")));

    let current_focus = ctx.memory(|memory| memory.focused());
    let focused_in_preferences = current_focus.is_some_and(|focused| {
        ctx.read_response(focused)
            .is_some_and(|response| response.layer_id == modal_layer)
    });
    if focus_state.is_none() && !focused_in_preferences {
        return;
    }
    Popup::close_all(ctx);
    let prior_focus = focus_state
        .and_then(|state| state.prior_focus)
        .filter(|prior| {
            ctx.read_response(*prior).is_some_and(|response| {
                response.layer_id != modal_layer
                    && response.enabled()
                    && response.sense.is_focusable()
            })
        });

    ctx.memory_mut(|memory| {
        if focused_in_preferences && let Some(current_focus) = current_focus {
            memory.surrender_focus(current_focus);
        }
        if focused_in_preferences && let Some(prior_focus) = prior_focus {
            memory.request_focus(prior_focus);
        }
    });
}

fn render_header(surface: &mut Ui, rect: Rect, large_targets: bool) -> egui::Response {
    let t = Tokens::get(surface.ctx());
    surface.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    surface.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let heading_font = theme::sans(tokens::FS_3, FontWeight::SemiBold);
    let heading_size = surface
        .painter()
        .layout_no_wrap("Preferences".to_owned(), heading_font.clone(), t.color.text)
        .size();
    let copy_rect = centered_header_copy_rect(rect, heading_size + vec2(1.0, 0.0), CONTENT_INSET);
    let mut copy_ui = surface.new_child(
        egui::UiBuilder::new()
            .max_rect(copy_rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    let heading_response = copy_ui.label(
        egui::RichText::new("Preferences")
            .font(heading_font)
            .color(t.color.text),
    );
    surface
        .ctx()
        .accesskit_node_builder(heading_response.id, |node| {
            node.set_role(egui::accesskit::Role::Heading);
            node.set_label("Preferences");
            node.set_level(2);
        });
    let close_side = if large_targets {
        touch_target(surface.ctx())
    } else {
        28.0
    };
    let mut close_ui = surface.new_child(
        egui::UiBuilder::new()
            // The button frame is wider than the glyph inside it, so insetting
            // the frame by the content inset would push the glyph in past the
            // title. Inset by the difference and the glyph's own right edge
            // lands on `CONTENT_INSET`, where the title's left edge is.
            .max_rect(rect.shrink2(vec2(CONTENT_INSET - 6.0, 0.0)))
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
    );
    close_ui
        .with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.push_id("neutral-close-preferences-1af3c6ff0446", |ui| {
                IconButton::new(Icon::Close)
                    .side(close_side)
                    .tooltip("Close preferences")
                    .show(ui)
            })
            .inner
        })
        .inner
}

fn centered_header_copy_rect(header: Rect, copy_size: egui::Vec2, left_inset: f32) -> Rect {
    Rect::from_min_size(
        egui::pos2(
            header.left() + left_inset,
            header.center().y - copy_size.y * 0.5,
        ),
        copy_size,
    )
}

fn begin_focus_session(ctx: &Context, state_id: Id) -> bool {
    let pass = ctx.cumulative_pass_nr();
    let current_focus = ctx.memory(|memory| memory.focused());
    ctx.data_mut(|data| {
        let previous = data.get_temp::<PreferencesFocusState>(state_id);
        let continuing =
            previous.is_some_and(|state| pass <= state.last_seen_pass.saturating_add(1));
        let state = match previous {
            Some(mut state) if continuing => {
                state.last_seen_pass = pass;
                state
            }
            _ => PreferencesFocusState {
                prior_focus: current_focus,
                last_seen_pass: pass,
            },
        };
        data.insert_temp(state_id, state);
        !continuing
    })
}

fn focus_is_within_modal(ctx: &Context, modal_layer: egui::LayerId) -> bool {
    let Some(focused) = ctx.memory(|memory| memory.focused()) else {
        return false;
    };
    let Some(response) = ctx.read_response(focused) else {
        return false;
    };
    response.layer_id == modal_layer
        || ctx.memory(|memory| memory.is_above_modal_layer(response.layer_id))
}

fn restore_focus(ctx: &Context, state_id: Id, close_control_id: Id, modal_layer: egui::LayerId) {
    let state = ctx.data_mut(|data| data.remove_temp::<PreferencesFocusState>(state_id));
    let prior_focus = state.and_then(|state| state.prior_focus).filter(|prior| {
        *prior != close_control_id
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
        if let Some(prior_focus) = prior_focus {
            memory.request_focus(prior_focus);
        }
    });
}

fn render_body(
    surface: &mut Ui,
    rect: Rect,
    layout: PreferencesLayout,
    category: &mut PreferenceCategory,
    render_page: impl FnMut(&mut Ui, PreferenceCategory),
) -> Option<Id> {
    if layout.narrow {
        let control_height = Tokens::get(surface.ctx()).metrics.ctl_h;
        let selector_height = control_height + 16.0;
        let search_height = control_height + SEARCH_FIELD_BAND;
        let search_rect = Rect::from_min_max(
            rect.min,
            egui::pos2(
                rect.right(),
                (rect.top() + search_height).min(rect.bottom()),
            ),
        );
        let inset = content_inset(surface);
        let mut search = surface.new_child(
            egui::UiBuilder::new()
                .id_salt("preferences.search.narrow")
                .max_rect(search_rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        search.add_space(SEARCH_FIELD_BAND * 0.5);
        search.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add_space(inset);
            render_search_field(ui, (search_rect.width() - inset * 2.0).max(1.0));
        });
        let selector_rect = Rect::from_min_max(
            egui::pos2(rect.left(), search_rect.bottom()),
            egui::pos2(
                rect.right(),
                (search_rect.bottom() + selector_height).min(rect.bottom()),
            ),
        );
        let active_category_id = render_mobile_category(surface, selector_rect, category);
        let content_rect =
            Rect::from_min_max(egui::pos2(rect.left(), selector_rect.bottom()), rect.max);
        render_scrolling_page(surface, content_rect, *category, render_page);
        Some(active_category_id)
    } else {
        let rail_rect = Rect::from_min_max(
            rect.min,
            egui::pos2(
                (rect.left() + CATEGORY_RAIL_WIDTH).min(rect.right()),
                rect.bottom(),
            ),
        );
        surface
            .painter()
            .rect_filled(rail_rect, 0.0, Tokens::get(surface.ctx()).color.bg_panel);
        surface.painter().vline(
            rail_rect.right(),
            rail_rect.y_range(),
            Stroke::new(1.0, Tokens::get(surface.ctx()).color.border),
        );
        let active_category_id = render_category_rail(surface, rail_rect, category);
        let content_rect = Rect::from_min_max(egui::pos2(rail_rect.right(), rect.top()), rect.max);
        render_scrolling_page(surface, content_rect, *category, render_page);
        active_category_id
    }
}

/// The rail's filter field: the one control that searches every page.
///
/// It sits at the head of the rail because searching is navigating here — it
/// answers the same question the category list answers, for a reader who knows
/// the setting's name but not which page holds it.
fn render_search_field(ui: &mut Ui, width: f32) {
    let t = Tokens::get(ui.ctx());
    let mut filter = read_filter(ui.ctx());
    let before = filter.query.clone();
    let response = ui.add_sized(
        [width, t.metrics.ctl_h],
        egui::TextEdit::singleline(&mut filter.query)
            .id(Id::new(SHELL_ID).with("filter-field"))
            .hint_text("Search settings")
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .margin(egui::Margin {
                left: 29,
                right: 8,
                top: 5,
                bottom: 5,
            }),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("Search settings");
        node.set_description("Filter every Preferences page by setting name");
    });
    crate::workbench::design_system::WorkbenchIcon::Search.paint(
        ui.painter(),
        Rect::from_center_size(
            egui::pos2(response.rect.left() + 15.0, response.rect.center().y),
            vec2(16.0, 16.0),
        ),
        t.color.text_faint,
    );
    if filter.query != before {
        write_filter(ui.ctx(), filter);
    }
}

/// Put a query in the field without typing it, so a review render and the
/// tests can show the filtered dialog as a reader would meet it.
#[cfg(test)]
pub(super) fn set_filter_for_review(ctx: &Context, query: &str) {
    write_filter(
        ctx,
        SettingFilter {
            query: query.to_owned(),
            ..SettingFilter::default()
        },
    );
}

/// Drop the query, so the rail and the pages read normally again.
fn clear_filter(ctx: &Context) {
    let mut filter = read_filter(ctx);
    filter.query.clear();
    write_filter(ctx, filter);
}

fn render_category_rail(
    surface: &mut Ui,
    rect: Rect,
    category: &mut PreferenceCategory,
) -> Option<Id> {
    let mut rail = surface.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    rail.spacing_mut().item_spacing.y = 0.0;
    let inset = content_inset(&rail);
    rail.add_space(SEARCH_FIELD_BAND * 0.5);
    rail.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(inset);
        render_search_field(ui, (rect.width() - inset * 2.0).max(1.0));
    });
    rail.add_space(SEARCH_FIELD_BAND * 0.5);
    let mut active_category_id = None;
    let mut rendered_group = None;
    for candidate in PreferenceCategory::ALL {
        if rendered_group != Some(candidate.group()) {
            rendered_group = Some(candidate.group());
            category_group_label(&mut rail, candidate.group().label());
        }
        rail.push_id(("preferences.category", candidate.stable_id()), |ui| {
            let active = *category == candidate;
            let response = category_button(ui, candidate.label(), active);
            if active {
                active_category_id = Some(response.id);
            }
            if response.clicked() {
                *category = candidate;
                // Picking a page is a request to read that page, not to read
                // it filtered down to a query about something else.
                clear_filter(ui.ctx());
            }
        });
    }
    let nav_response = rail.response();
    surface
        .ctx()
        .accesskit_node_builder(nav_response.id, |node| {
            node.set_role(egui::accesskit::Role::Navigation);
            node.set_label("Preference categories");
        });
    active_category_id
}

/// A rail group name: the one piece of text in the dialog that is not a
/// setting, a value or a control, so it is the faintest thing on the rail and
/// carries no rule, no eyebrow and no tracking.
fn category_group_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), CATEGORY_GROUP_HEIGHT),
        Sense::hover(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(label);
        node.set_level(3);
    });
    ui.painter().text(
        egui::pos2(rect.left() + CONTENT_INSET, rect.center().y + 3.0),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );
}

fn category_button(ui: &mut Ui, label: &str, active: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(
            ui.available_width(),
            CATEGORY_ROW_HEIGHT.max(t.metrics.ctl_h),
        ),
        Sense::click(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, true, label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_aria_current(if active {
            egui::accesskit::AriaCurrent::Page
        } else {
            egui::accesskit::AriaCurrent::False
        });
    });
    if active || response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    if active {
        ui.painter().rect_filled(
            Rect::from_min_size(rect.min, vec2(2.0, rect.height())),
            0.0,
            t.color.accent,
        );
    }
    ui.painter().text(
        egui::pos2(rect.left() + CONTENT_INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(
            tokens::FS_1,
            if active {
                FontWeight::Medium
            } else {
                FontWeight::Regular
            },
        ),
        if active {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    // Opening Preferences puts keyboard focus on the category you are on, so
    // the shell's shortcuts work without a click. Ringing it for that reason
    // frames the rail the moment the dialog appears and says the reader did
    // something they did not do — the page is already marked as current by
    // its fill, its accent edge and its weight. The ring is for a reader who
    // tabbed here, which is what `focus_is_visible` distinguishes.
    if theme::focus_is_visible(ui.ctx(), &response) {
        theme::paint_focus_ring(ui, &response, rect);
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn render_mobile_category(surface: &mut Ui, rect: Rect, category: &mut PreferenceCategory) -> Id {
    let t = Tokens::get(surface.ctx());
    surface.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    surface.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let mut ui = surface.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(PHONE_CONTENT_INSET, 8.0)))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    ui.spacing_mut().item_spacing.x = 10.0;
    ui.label(
        egui::RichText::new("Preference category")
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    let options = PreferenceCategory::ALL
        .iter()
        .map(|candidate| candidate.label().to_owned())
        .collect::<Vec<_>>();
    let select_width = ui.available_width();
    // `select` allocates the next automatic widget id. Capturing it makes the
    // responsive category control the modal's initial focus target, matching
    // the desktop rail's current-page focus behavior.
    let select_id = ui.next_auto_id();
    if let Some(index) = select(
        &mut ui,
        "preferences.preference-category",
        "Preference category",
        category.label(),
        &options,
        select_width,
    ) {
        *category = PreferenceCategory::ALL[index];
    }
    select_id
}

fn render_scrolling_page(
    surface: &mut Ui,
    rect: Rect,
    category: PreferenceCategory,
    mut render_page: impl FnMut(&mut Ui, PreferenceCategory),
) {
    let filtering = active_query(surface.ctx()).is_some();
    let mut content = surface.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    content.set_clip_rect(rect);
    egui::ScrollArea::vertical()
        // A filtered list is one surface over every page, so it keeps one
        // scroll position rather than inheriting whichever page was open.
        .id_salt(if filtering {
            "preferences.search-results"
        } else {
            category.stable_id()
        })
        .auto_shrink([false, false])
        .show(&mut content, |ui| {
            ui.set_min_width(ui.available_width());
            // CSS settings rows are contiguous. Every vertical margin in the
            // mockup is explicit, so egui's global 4 px item gap must not be
            // added between rows or on top of those margins.
            ui.spacing_mut().item_spacing.y = 0.0;
            if filtering {
                render_filtered_pages(ui, render_page);
            } else {
                render_page(ui, category);
            }
        });
}

/// Every page in rail order, each showing only the rows that matched.
///
/// All eight run because a query is about the settings, not about the page
/// that happens to be open; the rows themselves decide whether they appear,
/// and a page contributing nothing contributes no heading either.
fn render_filtered_pages(ui: &mut Ui, mut render_page: impl FnMut(&mut Ui, PreferenceCategory)) {
    let mut filter = read_filter(ui.ctx());
    filter.matches = 0;
    filter.pending_category = None;
    write_filter(ui.ctx(), filter);

    for candidate in PreferenceCategory::ALL {
        let mut filter = read_filter(ui.ctx());
        filter.pending_category = Some(candidate.label());
        write_filter(ui.ctx(), filter);
        ui.push_id(("preferences.filtered", candidate.stable_id()), |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            render_page(ui, candidate);
        });
    }

    let mut filter = read_filter(ui.ctx());
    let matches = filter.matches;
    filter.pending_category = None;
    write_filter(ui.ctx(), filter);
    if matches == 0 {
        no_matches_row(ui);
    }
}

/// What the content area says when nothing matched: the row grid's own inset
/// and padding, so an empty result is still a row of the page it replaces.
fn no_matches_row(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let horizontal = content_inset(ui) as i8;
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(
            horizontal,
            setting_row_padding(ui) as i8,
        ))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add(
                egui::Label::new(
                    egui::RichText::new("No setting matches this search.")
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
}

/// The footer: the page's reset on the left, the close on the right.
///
/// It used to carry a sentence explaining that preferences apply immediately
/// and that project settings are versioned, plus a second route to the account
/// surface the title bar, the drawers and the command palette already reach.
/// Neither is a decision a reader makes here.
fn render_footer(surface: &mut Ui, rect: Rect) -> FooterResponse {
    let t = Tokens::get(surface.ctx());
    surface.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    surface
        .painter()
        .hline(rect.x_range(), rect.top(), Stroke::new(1.0, t.color.border));
    let mut response = FooterResponse::default();
    let mut ui = surface.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(CONTENT_INSET, 0.0)))
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
    );
    ui.spacing_mut().item_spacing.x = FOOTER_ITEM_GAP;
    response.close = ui
        .push_id("preferences.done", |ui| {
            Button::new("Done").accent().show(ui).clicked()
        })
        .inner;
    // A search shows rows from every page at once, so there is no one page
    // for the reset to mean; offering it would reset the rail page the reader
    // cannot currently see.
    if active_query(surface.ctx()).is_none() {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            response.restore_defaults = ui
                .push_id("preferences.restore-defaults", |ui| {
                    Button::new("Restore defaults").ghost().show(ui).clicked()
                })
                .inner;
        });
    }
    response
}

#[derive(Debug, Clone, Copy, Default)]
struct FooterResponse {
    close: bool,
    restore_defaults: bool,
}

/// The page inset at this width: a phone gives up less of it to the gutter.
fn content_inset(ui: &Ui) -> f32 {
    if ui.ctx().content_rect().width() <= PHONE_MAX_WIDTH {
        PHONE_CONTENT_INSET
    } else {
        CONTENT_INSET
    }
}

/// A heading over a run of related settings, in the case it is written in.
///
/// It was an upper-cased, letter-spaced eyebrow over a full-width rule. Two
/// typographic devices and a case change to separate one group of rows from
/// the next is more emphasis than the distinction carries, and the product
/// settled on sentence-case headers with no tracking.
///
/// It is a row of the same grid, not a label floating in whitespace. Its
/// inset, its padding and the rule it closes with are a setting row's, and
/// the tint is the one the design system keeps for table heads and section
/// labels — so it separates two runs of settings without any spacing that
/// belongs to it alone. Every number here is read off the row beside it.
pub(super) fn section_heading(ui: &mut Ui, title: &str) {
    if active_query(ui.ctx()).is_some() {
        // A filtered page is a flat list of the rows that matched, so its
        // sections are no longer contiguous. A band naming one would sit above
        // rows that are not in it.
        return;
    }
    heading_band(ui, title, 4);
}

/// Which page a run of filtered matches came from.
///
/// Painted above the first match on each page, because a flat result list
/// otherwise states forty settings without saying where any of them live. It
/// is the section band's own geometry at a level above it, rather than a
/// second heading style invented for the filtered view.
fn category_heading(ui: &mut Ui, title: &str) {
    heading_band(ui, title, 3);
}

/// A full-width band carrying one heading.
///
/// Every number in it comes from the row grid — the same inset a label starts
/// at, the same padding above and below, and the rule a row closes with — so
/// the band is a row of the grid rather than a box of chosen size with a word
/// in it. Its height is measured from the text it was given.
fn heading_band(ui: &mut Ui, title: &str, level: usize) {
    let t = Tokens::get(ui.ctx());
    let horizontal = content_inset(ui);
    let font = theme::sans(tokens::FS_1, FontWeight::SemiBold);
    let text_height = ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(title.to_owned(), font.clone(), t.color.text)
            .size()
            .y
    });
    let (rect, response) = ui.allocate_exact_size(
        vec2(
            ui.available_width(),
            text_height + setting_row_padding(ui) * 2.0,
        ),
        Sense::hover(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), title));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(title);
        node.set_level(level);
    });
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().text(
        egui::pos2(rect.left() + horizontal, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title,
        font,
        t.color.text,
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

/// One setting on a Preferences page, hidden when a filter excludes it.
///
/// Every page row goes through here rather than through `setting_row` so the
/// filter is applied in exactly one place. `setting_row` stays the grid
/// primitive the sibling dialogs in this module use, and they are not
/// filtered by a query typed into this one.
///
/// A filtered-out row does not render, so its closure never runs and a
/// control that is not on screen cannot report having been used. That is why
/// this returns nothing: a row reporting what the reader did with it captures
/// that itself, as `action_row` does, instead of reading it back from here.
pub(super) fn preference_row<R>(
    ui: &mut Ui,
    title: &str,
    help: &str,
    add_value: impl FnOnce(&mut Ui) -> R,
) {
    searchable_row(ui, title, help, &[], add_value);
}

/// A [`preference_row`] a search also finds by `terms`: the labels of the
/// options it offers and the words people type for it. The terms sit at the
/// row's own call, so they cannot drift from the row they describe.
pub(super) fn searchable_row<R>(
    ui: &mut Ui,
    title: &str,
    help: &str,
    terms: &[&str],
    add_value: impl FnOnce(&mut Ui) -> R,
) {
    if let Some(query) = active_query(ui.ctx()) {
        if !row_matches(title, help, terms, &query) {
            return;
        }
        accept_filtered_row(ui);
    }
    setting_row(ui, title, help, add_value);
}

/// Count a match and, if this is the first on its page, name the page.
fn accept_filtered_row(ui: &mut Ui) {
    let mut filter = read_filter(ui.ctx());
    filter.matches += 1;
    let pending = filter.pending_category.take();
    write_filter(ui.ctx(), filter);
    if let Some(label) = pending {
        category_heading(ui, label);
    }
}

/// One setting: a label, optional help under it, and the control that sets it.
///
/// The label column is a fixed width rather than a fraction of the page, so
/// the control column begins at the same x whatever the dialog is sized to and
/// whichever page is open — the property-editor convention, and the reason a
/// reader can run an eye down the values. A row is as tall as its tallest
/// element plus one padding, so a plain select row is a control high and only
/// a row carrying wrapped help or a stack of controls grows.
///
/// A phone stacks the two: the label, then the control at full width.
pub(super) fn setting_row<R>(
    ui: &mut Ui,
    title: &str,
    help: &str,
    add_value: impl FnOnce(&mut Ui) -> R,
) -> R {
    let phone = ui.ctx().content_rect().width() <= PHONE_MAX_WIDTH;
    let horizontal = content_inset(ui) as i8;
    let response = egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(
            horizontal,
            setting_row_padding(ui) as i8,
        ))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let control_height = Tokens::get(ui.ctx()).metrics.ctl_h;
            if phone {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = SETTING_ROW_PHONE_VALUE_GAP;
                    ui.scope(|ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        setting_copy(ui, title, help, ui.available_width());
                    });
                    ui.scope(|ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.set_width(ui.available_width());
                        add_value(ui)
                    })
                    .inner
                })
                .inner
            } else {
                let gap = SETTING_ROW_COLUMN_GAP;
                let available = (ui.available_width() - gap).max(0.0);
                // The label column holds its width until the page is too
                // narrow for a usable control beside it, and only then gives
                // ground — never the other way round, which is what let a
                // 30 px switch sit in half a page of empty row.
                let label_width = LABEL_COLUMN_WIDTH.min(available * 0.6);
                let control_width = (available - label_width).max(1.0);
                // Both columns are centred on the row rather than hung from
                // its top. A control pinned to the top of a row whose label
                // carries two lines of help sits above the row's middle by
                // half the help, and by a different amount on the next row,
                // so a column of controls drifts against its own rows.
                let copy_height = setting_copy_height(ui, title, help, label_width);
                // Every row reserves the height of a label with one line of
                // help, which is what most of them carry, and grows past it
                // only for content that needs more. Without the floor a row
                // whose setting needs no explaining is four pixels shorter
                // than its neighbours, and a page of them reads as a list
                // measured by hand.
                let row_height = copy_height
                    .max(control_height)
                    .max(standard_copy_height(ui, label_width));
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    ui.set_min_height(row_height);
                    ui.vertical(|ui| {
                        ui.set_width(label_width);
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.add_space((row_height - copy_height) * 0.5);
                        setting_copy(ui, title, help, label_width);
                    });
                    ui.vertical(|ui| {
                        ui.set_width(control_width);
                        ui.add_space((row_height - control_height) * 0.5);
                        add_value(ui)
                    })
                    .inner
                })
                .inner
            }
        });
    ui.painter().hline(
        response.response.rect.x_range(),
        response.response.rect.bottom(),
        Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
    response.inner
}

/// A row's label, and its help where the setting needs one.
///
/// Help wraps to the label column rather than to a character count, so the
/// text block has the same right edge as the label above it instead of ending
/// wherever 38 zeroes happened to reach.
/// The height of the page's standard row content: a one-line label over one
/// line of help.
fn standard_copy_height(ui: &mut Ui, width: f32) -> f32 {
    setting_copy_height(ui, "A", "A", width)
}

/// How tall the label and its help will be at `width`, before either is laid
/// out, so the row can centre both columns against each other.
fn setting_copy_height(ui: &mut Ui, title: &str, help: &str, width: f32) -> f32 {
    let measure = |ui: &mut Ui, text: &str, size: f32, weight: FontWeight| {
        ui.fonts_mut(|fonts| {
            fonts
                .layout(
                    text.to_owned(),
                    theme::sans(size, weight),
                    egui::Color32::WHITE,
                    width,
                )
                .size()
                .y
        })
    };
    let mut height = measure(ui, title, tokens::FS_1, FontWeight::Regular);
    if !help.is_empty() {
        height += SETTING_COPY_HELP_GAP + measure(ui, help, tokens::FS_0, FontWeight::Regular);
    }
    height
}

fn setting_copy(ui: &mut Ui, title: &str, help: &str, width: f32) {
    let t = Tokens::get(ui.ctx());
    let title_response = ui.add(
        egui::Label::new(
            egui::RichText::new(title)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text),
        )
        .wrap(),
    );
    ui.ctx().accesskit_node_builder(title_response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_label(title);
    });
    if !help.is_empty() {
        ui.add_space(SETTING_COPY_HELP_GAP);
        ui.scope(|ui| {
            ui.set_max_width(width);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(help)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                )
                .wrap(),
            );
        });
    }
}

/// A row's control column: left-aligned, so every value on the page starts at
/// one x. Selects and numeric fields take [`control_width`]; a switch or a
/// segmented control takes its own.
pub(super) fn value_column<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.with_layout(egui::Layout::top_down(egui::Align::Min), add)
        .inner
}

/// A row's buttons, `(id, label)` each, left to right from the control column
/// like every other value. They are as tall as the page's controls, or as the
/// Accessibility touch target on a phone. Returns the index of the button
/// pressed this frame.
pub(super) fn action_buttons(
    ui: &mut Ui,
    buttons: &[(&'static str, &'static str)],
) -> Option<usize> {
    value_column(ui, |ui| {
        let height = if ui.ctx().content_rect().width() <= PHONE_MAX_WIDTH {
            touch_target(ui.ctx())
        } else {
            Tokens::get(ui.ctx()).metrics.ctl_h
        };
        let mut pressed = None;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), height),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                for (index, &(id, label)) in buttons.iter().enumerate() {
                    let clicked = ui
                        .push_id(id, |ui| {
                            Button::new(label).min_height(height).show(ui).clicked()
                        })
                        .inner;
                    if clicked {
                        pressed = Some(index);
                    }
                }
            },
        );
        pressed
    })
}

/// How wide a select or numeric field is on this row.
///
/// Capped so a wide dialog does not stretch one page's selects to twice the
/// width of the next page's, which is what makes a rail of pages read as one
/// dialog rather than eight.
pub(super) fn control_width(ui: &Ui) -> f32 {
    ui.available_width().min(CONTROL_MAX_WIDTH)
}

/// The `.segmented` control moved to `ui::widgets` when the results data
/// browser needed the same object; preference pages keep reaching it by the
/// name they already use.
pub(super) fn segmented(
    ui: &mut Ui,
    id_salt: &'static str,
    options: &[&str],
    selected: &mut usize,
) -> bool {
    crate::ui::widgets::segmented(
        ui,
        id_salt,
        options,
        selected,
        // The preferences grid item stretches at the phone breakpoint.
        crate::ui::widgets::SegmentedWidth::PhoneStretch,
    )
}

/// A boolean setting's control: the design system's switch, on a hit target a
/// coarse pointer can reach.
///
/// The shape is painted by [`paint_switch`] rather than here. This dialog used
/// to carry its own copy of the track, the knob and their colours, which is a
/// second switch in the product the moment either changes.
pub(super) fn preference_switch(
    ui: &mut Ui,
    id_salt: &'static str,
    accessible_label: &'static str,
    value: &mut bool,
) -> bool {
    let large_target = ui.ctx().content_rect().width() <= NARROW_MAX_WIDTH
        || ui.ctx().input(|input| input.has_touch_screen());
    let hit = if large_target {
        let target = touch_target(ui.ctx()).max(ui.spacing().interact_size.y);
        vec2(target, target)
    } else {
        vec2(SWITCH_WIDTH, Tokens::get(ui.ctx()).metrics.ctl_h)
    };
    let (rect, _) = ui.allocate_exact_size(hit, Sense::hover());
    let response = ui.interact(rect, ui.make_persistent_id(id_salt), Sense::click());
    response
        .widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, true, *value, accessible_label));
    paint_switch(ui, rect.center(), *value, response.hovered(), rect);
    theme::paint_focus_ring(ui, &response, rect);
    if response.clicked() {
        *value = !*value;
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_bounds(
        nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
        role: egui::accesskit::Role,
        label: &str,
    ) -> egui::accesskit::Rect {
        nodes
            .iter()
            .find(|(_, node)| node.role() == role && node.label() == Some(label))
            .and_then(|(_, node)| node.bounds())
            .unwrap_or_else(|| panic!("missing {role:?} node {label}"))
    }

    fn category_control_height(has_touch: bool) -> f64 {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut events = Vec::new();
        if has_touch {
            events.push(egui::Event::Touch {
                device_id: egui::TouchDeviceId(1),
                id: egui::TouchId(1),
                phase: egui::TouchPhase::Start,
                pos: egui::Pos2::ZERO,
                force: None,
            });
        }
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(768.0, 1024.0))),
            events,
            ..Default::default()
        };
        let mut category = PreferenceCategory::Appearance;
        let output = ctx.run_ui(input, |ctx| {
            let _ = show(ctx, &mut category, true, DEFAULT_TOUCH_TARGET, |_, _| {});
        });
        let update = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update");
        assert_eq!(
            update
                .nodes
                .iter()
                .find(|(id, _)| *id == update.focus)
                .and_then(|(_, node)| node.label()),
            Some("Preference category")
        );
        let bounds = node_bounds(
            &update.nodes,
            egui::accesskit::Role::ComboBox,
            "Preference category",
        );
        bounds.y1 - bounds.y0
    }

    fn switch_control_height(viewport_width: f32, has_touch: bool) -> f64 {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut events = Vec::new();
        if has_touch {
            events.push(egui::Event::Touch {
                device_id: egui::TouchDeviceId(1),
                id: egui::TouchId(1),
                phase: egui::TouchPhase::Start,
                pos: egui::Pos2::ZERO,
                force: None,
            });
        }
        let mut value = false;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(viewport_width, 600.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    preference_switch(ui, "preferences.test.switch", "Test switch", &mut value);
                });
            },
        );
        let bounds = node_bounds(
            &output
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes,
            egui::accesskit::Role::CheckBox,
            "Test switch",
        );
        bounds.y1 - bounds.y0
    }

    #[test]
    fn a_desktop_window_gets_the_full_fixed_surface() {
        let layout =
            PreferencesLayout::resolve(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0)));
        assert_eq!(layout.surface.size(), vec2(DESKTOP_WIDTH, DESKTOP_HEIGHT));
        assert!(!layout.narrow);
        assert!(!layout.phone);
    }

    /// One inset runs through the header, the rail, every row and the footer.
    ///
    /// The header used to be inset 15 px over a body inset 24 and a footer
    /// inset 12, so the dialog's title, its first label and its buttons each
    /// started on a different vertical line.
    #[test]
    fn one_inset_governs_the_header_the_rail_the_rows_and_the_footer() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    assert_eq!(content_inset(ui), CONTENT_INSET);
                });
            },
        );
        let header = Rect::from_min_size(egui::pos2(11.0, 11.0), vec2(998.0, HEADER_HEIGHT));
        let copy = centered_header_copy_rect(header, vec2(112.0, 31.0), CONTENT_INSET);
        assert_eq!(copy.left() - header.left(), CONTENT_INSET);
        assert_eq!(copy.center().y, header.center().y);
        assert!(copy.top() > header.top());
        assert!(copy.bottom() < header.bottom());
    }

    /// A row is as tall as the control on it plus one padding either side.
    ///
    /// The old row reserved a 52 px content track for a 28 px control, so
    /// every row was 70 px tall whatever was on it: an 11 px label and a
    /// 17 px switch inside two and a half control heights of air.
    #[test]
    fn a_plain_row_is_one_control_tall() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let standard = standard_row_height(&ctx);
        assert!(
            standard >= Tokens::get(&ctx).metrics.ctl_h + Tokens::get(&ctx).metrics.bar_pad * 2.0,
            "the page's row height clears a control and its padding"
        );
        assert!(
            standard < 70.0,
            "a row is shorter than the fixed 70 px track it replaces: {standard}"
        );
    }

    /// The height every row on the page holds to: a label with one line of
    /// help, padded. A row that carries no help is the same height as its
    /// neighbours rather than four pixels shorter.
    fn standard_row_height(ctx: &Context) -> f32 {
        let mut height = 0.0;
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    height = standard_copy_height(ui, LABEL_COLUMN_WIDTH)
                        .max(Tokens::get(ui.ctx()).metrics.ctl_h)
                        + setting_row_padding(ui) * 2.0;
                });
            },
        );
        height
    }

    /// What the dialog publishes to assistive technology, by role, after a
    /// few passes. The filter decides what exists rather than what is merely
    /// scrolled out of view, so the AT tree is the honest oracle for it: a
    /// row that is not in here was not rendered at all.
    fn published_labels(query: &str) -> Vec<(egui::accesskit::Role, String)> {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        if !query.is_empty() {
            write_filter(
                &ctx,
                SettingFilter {
                    query: query.to_owned(),
                    ..SettingFilter::default()
                },
            );
        }
        let mut category = PreferenceCategory::Appearance;
        let mut update = None;
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_060.0, 760.0))),
                    ..Default::default()
                },
                |ctx| {
                    let _ = show(
                        ctx,
                        &mut category,
                        true,
                        DEFAULT_TOUCH_TARGET,
                        |ui, category| {
                            // Two pages' worth of rows, enough to prove a query
                            // reaches past the page that happens to be open.
                            match category {
                                PreferenceCategory::Appearance => {
                                    section_heading(ui, "Appearance section");
                                    preference_row(ui, "Color mode", "", |_| {});
                                    preference_row(ui, "Canvas contrast", "", |_| {});
                                }
                                PreferenceCategory::Results => {
                                    preference_row(ui, "Cursor interpolation", "", |_| {});
                                }
                                _ => {}
                            }
                        },
                    );
                },
            );
            update = output.platform_output.accesskit_update;
        }
        update
            .expect("AccessKit tree update")
            .nodes
            .into_iter()
            .filter_map(|(_, node)| {
                // egui publishes a plain `Label`'s text as the node's value
                // and a widget's accessible name as its label, so a scan that
                // reads only one of the two misses half the page.
                let text = node
                    .label()
                    .or_else(|| node.value())
                    .map(std::string::ToString::to_string)?;
                Some((node.role(), text))
            })
            .collect()
    }

    fn labelled(labels: &[(egui::accesskit::Role, String)], label: &str) -> bool {
        labels.iter().any(|(_, published)| published == label)
    }

    #[test]
    fn with_no_query_a_page_shows_its_own_rows_and_its_sections() {
        let labels = published_labels("");
        assert!(labelled(&labels, "Color mode"));
        assert!(labelled(&labels, "Canvas contrast"));
        assert!(
            labelled(&labels, "Appearance section"),
            "an unfiltered page keeps its section bands"
        );
        assert!(
            !labelled(&labels, "Cursor interpolation"),
            "an unfiltered dialog shows one page, not all of them"
        );
    }

    #[test]
    fn a_query_reaches_rows_on_pages_that_are_not_open() {
        let labels = published_labels("cursor");
        assert!(
            labelled(&labels, "Cursor interpolation"),
            "a match on Results is reachable while Appearance is the open page"
        );
        assert!(
            !labelled(&labels, "Color mode"),
            "a row that does not match is not rendered"
        );
    }

    #[test]
    fn a_match_is_published_under_the_page_it_came_from() {
        let labels = published_labels("cursor");
        assert!(
            labels.iter().any(|(role, label)| {
                *role == egui::accesskit::Role::Heading && label == "Results and precision"
            }),
            "a flat result list names the page each run of matches belongs to"
        );
        assert!(
            !labelled(&labels, "Appearance section"),
            "a filtered list drops section bands, whose runs are no longer contiguous"
        );
    }

    #[test]
    fn a_query_matches_a_rows_help_as_well_as_its_label() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        assert!(row_matches(
            "Solver threads",
            "Concurrent workers",
            &[],
            "worker"
        ));
        assert!(row_matches("Solver threads", "", &[], "SOLVER"));
        assert!(!row_matches(
            "Solver threads",
            "Concurrent workers",
            &[],
            "grid"
        ));
    }

    /// A row is also found by the options it offers and by the words people
    /// type for it, neither of which is in its label.
    #[test]
    fn a_query_matches_a_rows_options_and_keywords() {
        assert!(row_matches("Color mode", "", &["Dark", "theme"], "dark"));
        assert!(row_matches("Color mode", "", &["Dark", "theme"], "THEME"));
        assert!(!row_matches("Color mode", "", &["Dark", "theme"], "grid"));
    }

    /// A search shows rows from every page, so the footer's page reset has
    /// no page to mean and is withdrawn until the search is cleared.
    #[test]
    fn restore_defaults_is_withdrawn_while_a_search_is_active() {
        assert!(labelled(&published_labels(""), "Restore defaults"));
        assert!(!labelled(&published_labels("cursor"), "Restore defaults"));
    }

    #[test]
    fn a_query_that_matches_nothing_says_so_instead_of_showing_an_empty_page() {
        let labels = published_labels("xyzzy");
        assert!(labelled(&labels, "No setting matches this search."));
        assert!(!labelled(&labels, "Color mode"));
        assert!(!labelled(&labels, "Cursor interpolation"));
    }

    #[test]
    fn the_filter_field_is_published_once_and_reaches_every_page() {
        let labels = published_labels("");
        let fields = labels
            .iter()
            .filter(|(_, label)| label == "Search settings")
            .count();
        assert_eq!(fields, 1, "one filter field, in the rail");
    }

    /// Whether a pixel is carrying the accent rather than a panel grey.
    ///
    /// Tested by how far red runs ahead of blue, not by equality with the
    /// token: the modal fades in over several passes, so accent reaches the
    /// canvas blended toward the backdrop and never matches exactly. Every
    /// surface behind it is a neutral whose channels sit together, while the
    /// accent is `rgb(242, 184, 36)` — 206 apart — so the gap survives the
    /// blend by a wide margin.
    fn carries_accent(pixel: egui::Color32) -> bool {
        i16::from(pixel.r()) - i16::from(pixel.b()) > 40
    }

    /// Paint the shell and report the first rail row's top edge and left edge.
    ///
    /// The two edges tell the marks apart: a focus ring strokes the whole row
    /// and so puts accent along the top, while the current-page marker is a
    /// bar down the left only. Sampling the top away from both corners
    /// therefore sees the ring and nothing else.
    fn active_row_edges() -> (Vec<egui::Color32>, Vec<egui::Color32>) {
        let mut category = PreferenceCategory::Appearance;
        let canvas = crate::ui::raster::render(vec2(1_060.0, 760.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    let _ = show(
                        ui.ctx(),
                        &mut category,
                        true,
                        DEFAULT_TOUCH_TARGET,
                        |_, _| {},
                    );
                });
        });
        // Appearance is the first category, so its row opens the rail under
        // that group's label. Located from the rail's own constants rather
        // than from numbers read off a picture.
        let rail_left = 31.0;
        let row_top = 153.0;
        let row_height = CATEGORY_ROW_HEIGHT;
        let top = canvas
            .pixels_in(Rect::from_min_max(
                egui::pos2(rail_left + 40.0, row_top),
                egui::pos2(rail_left + 120.0, row_top + 2.0),
            ))
            .collect();
        let left = canvas
            .pixels_in(Rect::from_min_max(
                egui::pos2(rail_left, row_top + row_height * 0.25),
                egui::pos2(rail_left + 2.0, row_top + row_height * 0.75),
            ))
            .collect();
        (top, left)
    }

    /// Opening Preferences focuses the category you are on so the dialog's
    /// keys work without a click. That is not the reader navigating, and it
    /// must not draw a ring around the rail the instant the dialog appears.
    #[test]
    fn opening_the_dialog_marks_the_current_page_without_ringing_it() {
        let (top, left) = active_row_edges();

        assert!(!top.is_empty() && !left.is_empty(), "sampled off-canvas");
        assert!(
            !top.iter().copied().any(carries_accent),
            "the active row's top edge carries accent, so a ring was painted \
             around a row the reader never navigated to"
        );
        assert!(
            left.iter().copied().any(carries_accent),
            "the current page still has to be marked, by its left accent edge"
        );
    }

    /// The ring is withheld by the `:focus-visible` rule, not deleted.
    ///
    /// `focus_is_visible` is the kit's own helper and carries its own tests
    /// for telling a tabbed-to focus from a pointer-given one. What belongs
    /// here is that the rail asks it before ringing anything, because the
    /// failure this guards against is someone restoring the unconditional
    /// call and taking the dialog back to ringing itself on open.
    #[test]
    fn the_rail_rings_a_row_only_when_the_keyboard_put_focus_there() {
        let production = crate::source_guard::production_half(include_str!("preferences_shell.rs"));
        let ring = production
            .match_indices("paint_focus_ring")
            .next()
            .expect("the rail still paints a keyboard ring");
        assert!(
            production[..ring.0].ends_with("        theme::"),
            "the ring call moved; check it is still inside a focus_is_visible gate"
        );
        assert!(
            production.contains("if theme::focus_is_visible(ui.ctx(), &response) {"),
            "the rail must ask whether focus is visible before ringing a row"
        );
    }

    #[test]
    fn phone_geometry_keeps_four_point_viewport_gutters() {
        let layout =
            PreferencesLayout::resolve(Rect::from_min_size(egui::Pos2::ZERO, vec2(390.0, 844.0)));
        assert_eq!(layout.surface.size(), vec2(382.0, 836.0));
        assert!(layout.narrow);
        assert!(layout.phone);
        assert_eq!(FOOTER_HEIGHT, 48.0);
    }

    #[test]
    fn tablet_uses_category_selector_without_phone_full_height_override() {
        let layout =
            PreferencesLayout::resolve(Rect::from_min_size(egui::Pos2::ZERO, vec2(768.0, 1024.0)));
        assert_eq!(layout.surface.size(), vec2(740.0, DESKTOP_HEIGHT));
        assert!(layout.narrow);
        assert!(!layout.phone);
        assert_eq!(FOOTER_HEIGHT, 48.0);
    }

    #[test]
    fn narrow_or_coarse_inputs_raise_every_target_to_the_touch_minimum() {
        assert_eq!(category_control_height(false), 44.0);
        assert_eq!(category_control_height(true), 44.0);
        assert_eq!(switch_control_height(821.0, false), 28.0);
        assert_eq!(switch_control_height(820.0, false), 44.0);
        assert_eq!(switch_control_height(1440.0, true), 44.0);
    }

    #[test]
    fn unmount_without_a_preferences_session_preserves_foreign_popups() {
        let ctx = Context::default();
        let popup_id = Id::new("foreign.application.popup");
        Popup::open_id(&ctx, popup_id);
        assert!(Popup::is_id_open(&ctx, popup_id));

        unmount(&ctx);

        assert!(Popup::is_id_open(&ctx, popup_id));
        Popup::close_id(&ctx, popup_id);
    }

    /// The rail, in order, and grouped so that each group is contiguous —
    /// otherwise the rail paints a group name twice and a reader reads past
    /// the heading they were looking under.
    #[test]
    fn the_rail_lists_its_categories_in_contiguous_groups() {
        assert_eq!(
            PreferenceCategory::ALL.map(PreferenceCategory::label),
            [
                "Appearance",
                "Accessibility",
                "Workspace",
                "Navigation",
                "Shortcuts",
                "Units and notation",
                "Simulation",
                "Schematic editor",
                "Projects and files",
                "Results and precision",
            ]
        );
        let mut seen: Vec<CategoryGroup> = Vec::new();
        for category in PreferenceCategory::ALL {
            if seen.last() != Some(&category.group()) {
                assert!(
                    !seen.contains(&category.group()),
                    "{} reopens a group the rail already closed",
                    category.label()
                );
                seen.push(category.group());
            }
        }
        assert_eq!(seen.len(), 2);
    }

    /// The three categories that were a heading over an absence, and the one
    /// row of fiction each of two of them carried.
    #[test]
    fn the_rail_offers_no_category_without_a_setting_on_it() {
        for absent in [
            "Security & privacy",
            "Security and privacy",
            "Integrations",
            "Remote compute",
        ] {
            assert!(
                !PreferenceCategory::ALL
                    .iter()
                    .any(|category| category.label() == absent),
                "{absent} leads to a page with nothing to configure"
            );
        }
    }

    /// Rows are contiguous and one control tall, and every control on the
    /// page starts at the same x — which is the whole reason the label column
    /// is a fixed width rather than a fraction of the row.
    #[test]
    fn rows_are_one_control_tall_and_share_one_control_column() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))),
            ..Default::default()
        };
        let mut category = PreferenceCategory::Appearance;
        let mut switch = false;
        let output = ctx.run_ui(input, |ctx| {
            let _ = show(ctx, &mut category, true, DEFAULT_TOUCH_TARGET, |ui, _| {
                setting_row(ui, "First row", "", |ui| {
                    value_column(ui, |ui| {
                        preference_switch(ui, "first", "First switch", &mut switch);
                    });
                });
                setting_row(ui, "Second row", "", |ui| {
                    value_column(ui, |ui| {
                        let width = control_width(ui);
                        ui.add_sized(
                            [width, Tokens::get(ui.ctx()).metrics.ctl_h],
                            egui::Slider::new(&mut 0, 0..=100).show_value(false),
                        )
                        .widget_info(|| {
                            WidgetInfo::labeled(WidgetType::Slider, true, "Second control")
                        });
                    });
                });
            });
        });
        let update = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update");
        let first = node_bounds(&update.nodes, egui::accesskit::Role::Label, "First row");
        let second = node_bounds(&update.nodes, egui::accesskit::Role::Label, "Second row");
        let switch_bounds = node_bounds(
            &update.nodes,
            egui::accesskit::Role::CheckBox,
            "First switch",
        );
        let slider = node_bounds(
            &update.nodes,
            egui::accesskit::Role::Slider,
            "Second control",
        );

        assert_eq!(
            second.y0 - first.y0,
            f64::from(standard_row_height(&ctx)),
            "two rows carrying no help are the page's one row height apart"
        );
        assert_eq!(first.x0, second.x0, "the two labels share one left edge");
        assert_eq!(
            switch_bounds.x0, slider.x0,
            "a switch and a slider on two rows start at one control column"
        );
        assert_eq!(
            switch_bounds.x0 - first.x0,
            f64::from(LABEL_COLUMN_WIDTH + SETTING_ROW_COLUMN_GAP),
            "the control column begins one label column and one gap in"
        );
    }

    #[test]
    fn desktop_setting_rows_grow_for_wrapped_copy_and_stacked_values() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))),
            ..Default::default()
        };
        let mut category = PreferenceCategory::Appearance;
        let output = ctx.run_ui(input, |ctx| {
            let _ = show(ctx, &mut category, true, DEFAULT_TOUCH_TARGET, |ui, _| {
                setting_row(
                    ui,
                    "Expandable fidelity row",
                    "This deliberately long preference explanation wraps across several lines so the row must measure content instead of clipping it into a fixed-height track.",
                    |ui| {
                        ui.label("Stacked value");
                        ui.label("Second stacked value");
                        ui.label("Third stacked value");
                    },
                );
                setting_row(ui, "Following fidelity row", "", |_| {});
            });
        });
        let update = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update");
        let first = node_bounds(
            &update.nodes,
            egui::accesskit::Role::Label,
            "Expandable fidelity row",
        );
        let second = node_bounds(
            &update.nodes,
            egui::accesskit::Role::Label,
            "Following fidelity row",
        );

        assert!(second.y0 - first.y0 > 46.0);
    }

    #[test]
    fn shell_publishes_dialog_navigation_heading_and_current_page_semantics() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))),
            ..Default::default()
        };
        let mut category = PreferenceCategory::Appearance;

        let output = ctx.run_ui(input, |ctx| {
            let _ = show(ctx, &mut category, true, DEFAULT_TOUCH_TARGET, |ui, _| {
                section_heading(ui, "Visual system");
            });
        });
        let update = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update");
        let nodes = &update.nodes;

        let dialog_bounds = node_bounds(nodes, egui::accesskit::Role::Dialog, "Preferences");
        assert_eq!(dialog_bounds.x1 - dialog_bounds.x0, 1000.0);
        assert_eq!(
            dialog_bounds.y1 - dialog_bounds.y0,
            f64::from(DESKTOP_HEIGHT)
        );

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Dialog
                && node.label() == Some("Preferences")
                && node
                    .description()
                    .is_some_and(|description| description.contains("accessibility, and recovery"))
                && node.is_modal()
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Navigation
                && node.label() == Some("Preference categories")
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Button
                && node.label() == Some("Appearance")
                && node.aria_current() == Some(egui::accesskit::AriaCurrent::Page)
        }));
        for (label, level) in [
            ("Preferences", 2),
            ("Interface", 3),
            ("Engineering", 3),
            ("Visual system", 4),
        ] {
            assert!(
                nodes.iter().any(|(_, node)| {
                    node.role() == egui::accesskit::Role::Heading
                        && node.label() == Some(label)
                        && node.level() == Some(level)
                }),
                "missing heading {label} at level {level}; actual headings: {:?}",
                nodes
                    .iter()
                    .filter(|(_, node)| node.role() == egui::accesskit::Role::Heading)
                    .map(|(_, node)| (node.label(), node.level()))
                    .collect::<Vec<_>>()
            );
        }
        assert_eq!(
            nodes
                .iter()
                .find(|(id, _)| *id == update.focus)
                .and_then(|(_, node)| node.label()),
            Some("Appearance")
        );
    }

    #[test]
    fn desktop_rail_and_page_are_contained_by_the_full_preferences_surface() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_089.0, 900.0))),
            ..Default::default()
        };
        let mut category = PreferenceCategory::Appearance;
        let output = ctx.run_ui(input, |ctx| {
            let _ = show(ctx, &mut category, true, DEFAULT_TOUCH_TARGET, |ui, _| {
                setting_row(ui, "Contained row", "", |_| {});
            });
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        let dialog = node_bounds(&nodes, egui::accesskit::Role::Dialog, "Preferences");
        let current_category = node_bounds(&nodes, egui::accesskit::Role::Button, "Appearance");
        let row = node_bounds(&nodes, egui::accesskit::Role::Label, "Contained row");

        assert_eq!(dialog.x1 - dialog.x0, DESKTOP_WIDTH as f64);
        assert_eq!(
            current_category.x1 - current_category.x0,
            CATEGORY_RAIL_WIDTH as f64
        );
        assert!(current_category.x0 >= dialog.x0);
        assert!(current_category.x1 <= dialog.x1);
        assert!(current_category.y0 >= dialog.y0 + HEADER_HEIGHT as f64);
        assert!(row.x0 >= current_category.x1);
        assert!(row.x1 <= dialog.x1);
    }
}
