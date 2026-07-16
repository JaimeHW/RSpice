//! Dedicated Preferences shell from the workbench mockup.
//!
//! Preferences are not a generic form dialog: the reference surface has a
//! fixed header/footer, a persistent category rail, and a compact category
//! selector on touch/narrow layouts.  Keeping that composition here prevents
//! application preference pages from rebuilding shell geometry ad hoc.

use egui::{
    Context, Id, Order, Popup, Rect, Sense, Stroke, Ui, UiKind, WidgetInfo, WidgetType, vec2,
};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, IconButton, select};

const DESKTOP_WIDTH: f32 = 1000.0;
const DESKTOP_HEIGHT: f32 = 680.0;
const DESKTOP_HORIZONTAL_INSET: f32 = 28.0;
const DESKTOP_VERTICAL_INSET: f32 = 34.0;
const PHONE_INSET: f32 = 8.0;
const PHONE_MAX_WIDTH: f32 = 560.0;
const NARROW_MAX_WIDTH: f32 = 820.0;
const SCOPE_NARROW_MAX_WIDTH: f32 = 760.0;
const SECTION_LABEL_NARROW_MAX_WIDTH: f32 = 620.0;
const HEADER_HEIGHT: f32 = 57.0;
const FOOTER_HEIGHT: f32 = 48.0;
const CATEGORY_RAIL_WIDTH: f32 = 208.0;
const CATEGORY_ROW_HEIGHT: f32 = 36.0;
const TOUCH_TARGET: f32 = 44.0;
const SEGMENT_MIN_WIDTH: f32 = 54.0;
const SEGMENT_HORIZONTAL_PADDING: f32 = 8.0;
const HEADER_COPY_GAP: f32 = 3.0;
const FOOTER_ITEM_GAP: f32 = 6.0;
const SWITCH_TRACK_WIDTH: f32 = 30.0;
const SWITCH_TRACK_HEIGHT: f32 = 17.0;
const SWITCH_KNOB_DIAMETER: f32 = 11.0;
const SWITCH_KNOB_TRANSLATION: f32 = 13.0;
const SETTING_ROW_DESKTOP_CONTENT_HEIGHT: f32 = 52.0;
const SETTING_ROW_COLUMN_GAP: f32 = 24.0;
const SETTING_ROW_PHONE_VALUE_GAP: f32 = 10.0;
const SHELL_ID: &str = "rspice.preferences.shell";

/// Preference categories whose controls are fully connected to RSpice.
///
/// The mockup contains additional future categories. They deliberately stay
/// absent until their underlying product behavior exists; exposing a dead
/// category would make the Preferences surface lie about product readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) enum PreferenceCategory {
    #[default]
    Appearance,
    Workspace,
    Files,
}

impl PreferenceCategory {
    pub(super) const ALL: [Self; 3] = [Self::Appearance, Self::Workspace, Self::Files];

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Appearance => "Appearance",
            Self::Workspace => "Workspace",
            Self::Files => "Files & storage",
        }
    }

    const fn stable_id(self) -> &'static str {
        match self {
            Self::Appearance => "appearance",
            Self::Workspace => "workspace",
            Self::Files => "files",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct PreferencesShellResponse {
    pub(super) close_requested: bool,
    pub(super) license_activation_requested: bool,
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

/// Temporarily raises controls to the mockup's 44 px narrow/coarse target.
struct LargeControlTargetOverride {
    ctx: Context,
    original: Option<Tokens>,
}

impl LargeControlTargetOverride {
    fn new(ctx: &Context, enabled: bool) -> Self {
        let original = (*Tokens::get(ctx)).clone();
        if !enabled || original.metrics.ctl_h >= TOUCH_TARGET {
            return Self {
                ctx: ctx.clone(),
                original: None,
            };
        }
        let mut adjusted = original.clone();
        adjusted.metrics.ctl_h = TOUCH_TARGET;
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
pub(super) fn show(
    ctx: &Context,
    category: &mut PreferenceCategory,
    allow_escape_close: bool,
    render_page: impl FnOnce(&mut Ui, PreferenceCategory),
) -> PreferencesShellResponse {
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
        if render_footer(&mut surface, footer) {
            response.license_activation_requested = true;
        }
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
                "Personal and device appearance, workspace, and file preferences. Changes apply immediately; project settings remain versioned.",
            );
            node.set_modal();
        });
    }

    if allow_escape_close
        && !popup_was_open
        && ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
    {
        response.close_requested = true;
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
    let mut ui = surface.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(15.0, 0.0)))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let heading_response = ui
        .vertical(|ui| {
            ui.spacing_mut().item_spacing.y = HEADER_COPY_GAP;
            ui.label(
                egui::RichText::new("WORKSPACE")
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.text_faint)
                    .extra_letter_spacing(0.09 * tokens::FS_0),
            );
            ui.label(
                egui::RichText::new("Preferences")
                    .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                    .color(t.color.text),
            )
        })
        .inner;
    surface
        .ctx()
        .accesskit_node_builder(heading_response.id, |node| {
            node.set_role(egui::accesskit::Role::Heading);
            node.set_label("Preferences");
            node.set_level(2);
        });
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.push_id("neutral-close-preferences-1af3c6ff0446", |ui| {
            IconButton::new(Icon::Close)
                .side(if large_targets { TOUCH_TARGET } else { 28.0 })
                .tooltip("Close preferences")
                .show(ui)
        })
        .inner
    })
    .inner
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
    render_page: impl FnOnce(&mut Ui, PreferenceCategory),
) -> Option<Id> {
    if layout.narrow {
        let selector_height = Tokens::get(surface.ctx()).metrics.ctl_h + 16.0;
        let selector_rect = Rect::from_min_max(
            rect.min,
            egui::pos2(
                rect.right(),
                (rect.top() + selector_height).min(rect.bottom()),
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
    let mut active_category_id = None;
    for candidate in PreferenceCategory::ALL {
        rail.push_id(("preferences.category", candidate.stable_id()), |ui| {
            let active = *category == candidate;
            let response = category_button(ui, candidate.label(), active);
            if active {
                active_category_id = Some(response.id);
            }
            if response.clicked() {
                *category = candidate;
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
        egui::pos2(rect.left() + 18.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if active {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
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
            .max_rect(rect.shrink2(vec2(10.0, 8.0)))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    ui.spacing_mut().item_spacing.x = 10.0;
    ui.label(
        egui::RichText::new("Preference category")
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
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
    render_page: impl FnOnce(&mut Ui, PreferenceCategory),
) {
    let mut content = surface.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    content.set_clip_rect(rect);
    egui::ScrollArea::vertical()
        .id_salt(("preferences.page", category.stable_id()))
        .auto_shrink([false, false])
        .show(&mut content, |ui| {
            ui.set_min_width(ui.available_width());
            // CSS settings rows are contiguous. Every vertical margin in the
            // mockup is explicit, so egui's global 4 px item gap must not be
            // added between rows or on top of those margins.
            ui.spacing_mut().item_spacing.y = 0.0;
            render_page(ui, category);
        });
}

fn render_footer(surface: &mut Ui, rect: Rect) -> bool {
    let t = Tokens::get(surface.ctx());
    surface.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    surface
        .painter()
        .hline(rect.x_range(), rect.top(), Stroke::new(1.0, t.color.border));
    let mut ui = surface.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(12.0, 0.0)))
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
    );
    ui.spacing_mut().item_spacing.x = FOOTER_ITEM_GAP;
    let clicked = ui
        .push_id("license-activation", |ui| {
            Button::new("License activation…").show(ui).clicked()
        })
        .inner;
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.add(
            egui::Label::new(
                egui::RichText::new(
                    "Personal and device preferences apply immediately. Project settings remain versioned.",
                )
                .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                .color(t.color.text_dim),
            )
            .wrap(),
        );
    });
    clicked
}

pub(super) fn page_heading(ui: &mut Ui, title: &str, description: &str) {
    let t = Tokens::get(ui.ctx());
    let phone = ui.ctx().content_rect().width() <= PHONE_MAX_WIDTH;
    let horizontal = if phone { 16.0 } else { 24.0 };
    ui.add_space(if phone { 16.0 } else { 18.0 });
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(horizontal as i8, 0))
        .show(ui, |ui| {
            let heading_response = ui.label(
                egui::RichText::new(title)
                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.ctx()
                .accesskit_node_builder(heading_response.id, |node| {
                    node.set_role(egui::accesskit::Role::Heading);
                    node.set_label(title);
                    node.set_level(3);
                });
            ui.add_space(4.0);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(description)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
    ui.add_space(if phone { 8.0 } else { 12.0 });
}

/// Read-only ownership strip shown above each settings page. The mockup's
/// policy-review action remains absent until RSpice has a resolved-policy
/// engine, but the authoritative scope copy is still useful and truthful.
pub(super) fn scope_strip(ui: &mut Ui, scope: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    let narrow = ui.ctx().content_rect().width() <= SCOPE_NARROW_MAX_WIDTH;
    let horizontal = if narrow { 12 } else { 24 };
    let response = egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(horizontal, 7))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_min_height(28.0);
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.label(
                    egui::RichText::new(scope)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.add_space(2.0);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(detail)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    )
                    .wrap(),
                );
            });
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    // CSS collapses the strip's 14 px bottom margin with the 14–18 px top
    // margin of the next heading/section. Every runtime-backed page has that
    // next block, so it owns the single resolved gap here.
}

pub(super) fn section_label(ui: &mut Ui, title: &str) {
    let t = Tokens::get(ui.ctx());
    let narrow = ui.ctx().content_rect().width() <= SECTION_LABEL_NARROW_MAX_WIDTH;
    let horizontal = if narrow { 16.0 } else { 24.0 };
    ui.add_space(14.0);
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 31.0), Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), title));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(title);
        node.set_level(4);
    });
    let line = Rect::from_min_max(
        egui::pos2(rect.left() + horizontal, rect.bottom()),
        egui::pos2(rect.right() - horizontal, rect.bottom()),
    );
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &title.to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::SemiBold),
            color: t.color.text_dim,
            extra_letter_spacing: 0.06 * tokens::FS_0,
            ..Default::default()
        },
    );
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    ui.painter().galley(
        egui::pos2(line.left(), rect.center().y - galley.size().y * 0.5),
        galley,
        t.color.text_dim,
    );
    ui.painter().hline(
        line.x_range(),
        line.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

/// Mockup setting row: two weighted columns on desktop, one stacked column
/// with 16 px gutters on phones.
pub(super) fn setting_row<R>(
    ui: &mut Ui,
    title: &str,
    help: &str,
    add_value: impl FnOnce(&mut Ui) -> R,
) -> R {
    let phone = ui.ctx().content_rect().width() <= PHONE_MAX_WIDTH;
    let horizontal = if phone { 16 } else { 24 };
    let vertical = if phone { 13 } else { 9 };
    let response = egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(horizontal, vertical))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            if phone {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = SETTING_ROW_PHONE_VALUE_GAP;
                    ui.scope(|ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        setting_copy(ui, title, help);
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
                let left = (available * 0.45).max(220.0);
                let right = (available - left).max(240.0);
                let (row_rect, _) = ui.allocate_exact_size(
                    vec2(ui.available_width(), SETTING_ROW_DESKTOP_CONTENT_HEIGHT),
                    Sense::hover(),
                );
                let mut row = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(row_rect)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                );
                row.spacing_mut().item_spacing.x = gap;
                row.vertical(|ui| {
                    ui.set_width(left);
                    setting_copy(ui, title, help);
                });
                row.vertical(|ui| {
                    ui.set_width(right);
                    add_value(ui)
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

fn setting_copy(ui: &mut Ui, title: &str, help: &str) {
    let t = Tokens::get(ui.ctx());
    let title_response = ui.label(
        egui::RichText::new(title)
            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text),
    );
    ui.ctx().accesskit_node_builder(title_response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_label(title);
    });
    if !help.is_empty() {
        ui.add_space(3.0);
        let help_font = theme::sans(tokens::FS_0, FontWeight::Regular);
        let help_width = ui
            .fonts_mut(|fonts| fonts.glyph_width(&help_font, '0') * 38.0)
            .min(ui.available_width());
        ui.scope(|ui| {
            ui.set_max_width(help_width);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(help)
                        .font(help_font)
                        .color(t.color.text_faint),
                )
                .wrap(),
            );
        });
    }
}

pub(super) fn right_aligned<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    let phone = ui.ctx().content_rect().width() <= PHONE_MAX_WIDTH;
    ui.with_layout(
        egui::Layout::top_down(if phone {
            egui::Align::Min
        } else {
            egui::Align::Max
        }),
        add,
    )
    .inner
}

/// Contiguous option buttons matching the mockup `.segmented` control.
pub(super) fn segmented(
    ui: &mut Ui,
    id_salt: &'static str,
    options: &[&str],
    selected: &mut usize,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let height = t.metrics.ctl_h;
    let natural_widths = options
        .iter()
        .map(|label| {
            let text_width = ui.fonts_mut(|fonts| {
                fonts
                    .layout_no_wrap(
                        (*label).to_owned(),
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text,
                    )
                    .size()
                    .x
            });
            (text_width + SEGMENT_HORIZONTAL_PADDING * 2.0).max(SEGMENT_MIN_WIDTH)
        })
        .collect::<Vec<_>>();
    let desired_width = natural_widths.iter().sum::<f32>();
    let phone = ui.ctx().content_rect().width() <= PHONE_MAX_WIDTH;
    // At the phone breakpoint the grid item stretches, while its flex
    // children keep their natural CSS widths. The remaining inset area is
    // deliberately blank rather than distributing extra width to buttons.
    let width = if phone {
        ui.available_width()
    } else {
        desired_width.min(ui.available_width())
    };
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let scale = if desired_width > 0.0 {
        (rect.width() / desired_width).min(1.0)
    } else {
        1.0
    };
    let mut changed = false;
    ui.painter().rect(
        rect,
        t.radius,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let mut cell_left = rect.left();
    for (index, label) in options.iter().enumerate() {
        let cell_width = natural_widths[index] * scale;
        let cell = Rect::from_min_max(
            egui::pos2(cell_left, rect.top()),
            egui::pos2(
                if index + 1 == options.len() && rect.width() <= desired_width {
                    rect.right()
                } else {
                    cell_left + cell_width
                },
                rect.bottom(),
            ),
        );
        cell_left = cell.right();
        let response = ui.interact(
            cell,
            ui.make_persistent_id((id_salt, index)),
            Sense::click(),
        );
        response.widget_info(|| {
            WidgetInfo::selected(WidgetType::RadioButton, true, *selected == index, *label)
        });
        let active = *selected == index;
        let fill = if active {
            t.color.bg_active
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            egui::Color32::TRANSPARENT
        };
        if fill != egui::Color32::TRANSPARENT {
            ui.painter().rect_filled(cell.shrink(1.0), 0.0, fill);
        }
        if index > 0 {
            ui.painter().vline(
                cell.left(),
                cell.y_range(),
                Stroke::new(1.0, t.color.border),
            );
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
            egui::Align2::CENTER_CENTER,
            *label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if active {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        theme::paint_focus_ring(ui, &response, cell);
        let keyboard_previous =
            response.has_focus() && ui.input(|input| input.key_pressed(egui::Key::ArrowLeft));
        let keyboard_next =
            response.has_focus() && ui.input(|input| input.key_pressed(egui::Key::ArrowRight));
        let next = if keyboard_previous {
            index.checked_sub(1)
        } else if keyboard_next {
            (index + 1 < options.len()).then_some(index + 1)
        } else {
            None
        };
        if let Some(next) = next {
            *selected = next;
            let next_id = ui.make_persistent_id((id_salt, next));
            ui.memory_mut(|memory| memory.request_focus(next_id));
            changed = true;
        } else if response.clicked() && !active {
            *selected = index;
            changed = true;
        }
    }
    changed
}

/// Compact switch with a 44 px touch hit target and checkbox semantics.
pub(super) fn preference_switch(
    ui: &mut Ui,
    id_salt: &'static str,
    accessible_label: &'static str,
    value: &mut bool,
) -> bool {
    let large_target = ui.ctx().content_rect().width() <= NARROW_MAX_WIDTH
        || ui.ctx().input(|input| input.has_touch_screen());
    let hit = if large_target {
        vec2(TOUCH_TARGET, TOUCH_TARGET)
    } else {
        vec2(SWITCH_TRACK_WIDTH, SWITCH_TRACK_HEIGHT)
    };
    let (rect, _) = ui.allocate_exact_size(hit, Sense::hover());
    let response = ui.interact(rect, ui.make_persistent_id(id_salt), Sense::click());
    response
        .widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, true, *value, accessible_label));
    let t = Tokens::get(ui.ctx());
    let track =
        Rect::from_center_size(rect.center(), vec2(SWITCH_TRACK_WIDTH, SWITCH_TRACK_HEIGHT));
    ui.painter().rect(
        track,
        8.5,
        if *value {
            t.color.accent
        } else {
            t.color.bg_inset
        },
        Stroke::new(
            1.0,
            if *value {
                t.color.accent
            } else {
                t.color.border_strong
            },
        ),
        egui::StrokeKind::Inside,
    );
    let knob_radius = SWITCH_KNOB_DIAMETER * 0.5;
    let knob_x =
        track.left() + 2.0 + knob_radius + if *value { SWITCH_KNOB_TRANSLATION } else { 0.0 };
    ui.painter().circle_filled(
        egui::pos2(knob_x, track.center().y),
        knob_radius,
        if *value {
            t.color.accent_ink
        } else {
            t.color.text_dim
        },
    );
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
        let output = ctx.run(input, |ctx| {
            let _ = show(ctx, &mut category, true, |_, _| {});
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
        let output = ctx.run(
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
    fn desktop_geometry_matches_preferences_mockup() {
        let layout =
            PreferencesLayout::resolve(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0)));
        assert_eq!(layout.surface.size(), vec2(1000.0, 680.0));
        assert!(!layout.narrow);
        assert!(!layout.phone);
    }

    #[test]
    fn shell_structure_matches_preferences_mockup_tokens() {
        assert_eq!(HEADER_HEIGHT, 57.0);
        assert_eq!(FOOTER_HEIGHT, 48.0);
        assert_eq!(CATEGORY_RAIL_WIDTH, 208.0);
        assert_eq!(CATEGORY_ROW_HEIGHT, 36.0);
        assert_eq!(NARROW_MAX_WIDTH, 820.0);
        assert_eq!(SCOPE_NARROW_MAX_WIDTH, 760.0);
        assert_eq!(SECTION_LABEL_NARROW_MAX_WIDTH, 620.0);
        assert_eq!(PHONE_MAX_WIDTH, 560.0);
        assert_eq!(TOUCH_TARGET, 44.0);
        assert_eq!(SEGMENT_MIN_WIDTH, 54.0);
        assert_eq!(SEGMENT_HORIZONTAL_PADDING, 8.0);
        assert_eq!(HEADER_COPY_GAP, 3.0);
        assert_eq!(FOOTER_ITEM_GAP, 6.0);
        assert_eq!(SWITCH_TRACK_WIDTH, 30.0);
        assert_eq!(SWITCH_TRACK_HEIGHT, 17.0);
        assert_eq!(SWITCH_KNOB_DIAMETER, 11.0);
        assert_eq!(SWITCH_KNOB_TRANSLATION, 13.0);
        assert_eq!(SETTING_ROW_DESKTOP_CONTENT_HEIGHT, 52.0);
        assert_eq!(SETTING_ROW_COLUMN_GAP, 24.0);
        assert_eq!(SETTING_ROW_PHONE_VALUE_GAP, 10.0);
        assert_eq!(SETTING_ROW_DESKTOP_CONTENT_HEIGHT + 9.0 * 2.0, 70.0);
    }

    #[test]
    fn phone_geometry_keeps_four_point_viewport_gutters() {
        let layout =
            PreferencesLayout::resolve(Rect::from_min_size(egui::Pos2::ZERO, vec2(390.0, 844.0)));
        assert_eq!(layout.surface.size(), vec2(382.0, 836.0));
        assert!(layout.narrow);
        assert!(layout.phone);
    }

    #[test]
    fn tablet_uses_category_selector_without_phone_full_height_override() {
        let layout =
            PreferencesLayout::resolve(Rect::from_min_size(egui::Pos2::ZERO, vec2(768.0, 1024.0)));
        assert_eq!(layout.surface.size(), vec2(740.0, 680.0));
        assert!(layout.narrow);
        assert!(!layout.phone);
    }

    #[test]
    fn narrow_or_coarse_inputs_raise_targets_at_the_exact_mockup_boundary() {
        assert_eq!(category_control_height(false), 44.0);
        assert_eq!(category_control_height(true), 44.0);
        assert_eq!(switch_control_height(821.0, false), 17.0);
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

    #[test]
    fn only_runtime_backed_categories_are_exposed() {
        assert_eq!(
            PreferenceCategory::ALL.map(PreferenceCategory::label),
            ["Appearance", "Workspace", "Files & storage"]
        );
    }

    #[test]
    fn segmented_options_keep_mockup_natural_widths() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(800.0, 600.0))),
            ..Default::default()
        };
        let mut selected = 0;

        let output = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = segmented(
                    ui,
                    "preferences.test.density",
                    &["Compact", "Comfortable"],
                    &mut selected,
                );
            });
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        let widths = ["Compact", "Comfortable"].map(|label| {
            nodes
                .iter()
                .find(|(_, node)| {
                    node.role() == egui::accesskit::Role::RadioButton && node.label() == Some(label)
                })
                .and_then(|(_, node)| node.bounds())
                .map(|bounds| bounds.x1 - bounds.x0)
                .unwrap_or_else(|| panic!("missing segmented option {label}"))
        });

        assert!(widths[0] >= SEGMENT_MIN_WIDTH as f64);
        assert!(widths[1] > widths[0]);
    }

    #[test]
    fn desktop_setting_rows_are_contiguous_seventy_pixel_tracks() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))),
            ..Default::default()
        };
        let mut category = PreferenceCategory::Appearance;
        let output = ctx.run(input, |ctx| {
            let _ = show(ctx, &mut category, true, |ui, _| {
                setting_row(ui, "First fidelity row", "", |_| {});
                setting_row(ui, "Second fidelity row", "", |_| {});
            });
        });
        let update = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update");
        let first = node_bounds(
            &update.nodes,
            egui::accesskit::Role::Label,
            "First fidelity row",
        );
        let second = node_bounds(
            &update.nodes,
            egui::accesskit::Role::Label,
            "Second fidelity row",
        );

        assert_eq!(second.y0 - first.y0, 70.0);
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

        let output = ctx.run(input, |ctx| {
            let _ = show(ctx, &mut category, true, |ui, _| {
                page_heading(ui, "Appearance", "Appearance preferences");
                section_label(ui, "Visual system");
            });
        });
        let update = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update");
        let nodes = &update.nodes;

        let dialog_bounds = node_bounds(nodes, egui::accesskit::Role::Dialog, "Preferences");
        assert_eq!(dialog_bounds.x1 - dialog_bounds.x0, 1000.0);
        assert_eq!(dialog_bounds.y1 - dialog_bounds.y0, 680.0);

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Dialog
                && node.label() == Some("Preferences")
                && node.description().is_some_and(|description| {
                    description.contains("appearance, workspace, and file preferences")
                })
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
        for (label, level) in [("Preferences", 2), ("Appearance", 3), ("Visual system", 4)] {
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
        let output = ctx.run(input, |ctx| {
            let _ = show(ctx, &mut category, true, |ui, _| {
                page_heading(ui, "Appearance", "Appearance preferences");
            });
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        let dialog = node_bounds(&nodes, egui::accesskit::Role::Dialog, "Preferences");
        let current_category = node_bounds(&nodes, egui::accesskit::Role::Button, "Appearance");
        let page_heading = node_bounds(&nodes, egui::accesskit::Role::Heading, "Appearance");

        assert_eq!(dialog.x1 - dialog.x0, DESKTOP_WIDTH as f64);
        assert_eq!(
            current_category.x1 - current_category.x0,
            CATEGORY_RAIL_WIDTH as f64
        );
        assert!(current_category.x0 >= dialog.x0);
        assert!(current_category.x1 <= dialog.x1);
        assert!(current_category.y0 >= dialog.y0 + HEADER_HEIGHT as f64);
        assert!(page_heading.x0 >= current_category.x1);
        assert!(page_heading.x1 <= dialog.x1);
    }
}
