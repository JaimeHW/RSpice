//! Schematic context-menu presentation and source-bound input requests.

use crate::{requests::EditorRequestSource, session::tool::Tool};
use egui::{
    Align2, Color32, Context, CornerRadius, Frame, Key, Modifiers, Popup, Rect, RectAlign,
    Response, ScrollArea, Sense, Shadow, Stroke, StrokeKind, Ui, WidgetInfo, WidgetType, pos2,
    vec2,
};
use rspice_design::schematic::selection::Selection;
use rspice_design_model::Point;
use rspice_ui_kit::{
    icons::Icon,
    panels::WorkbenchIcon,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};

const DESKTOP_WIDTH: f32 = 286.0;
/// Tall enough for the tallest menu there is without a scroller: a placed
/// source's, at sixteen rows and five separators — 526 px with the header and
/// the border. Every other target's menu is one separator shorter.
const DESKTOP_MAX_HEIGHT: f32 = 528.0;
const DESKTOP_VIEWPORT_INSET: f32 = 6.0;
/// The mockup's `.menu-item { min-height: 27px }`. The row was drawn three
/// pixels taller here, which cost the surface a row's worth of height for no
/// design reason and pushed the last entries below the ceiling.
const DESKTOP_ROW_HEIGHT: f32 = 27.0;
const DESKTOP_RADIUS: u8 = 3;
const TOUCH_MAX_WIDTH: f32 = 420.0;
const TOUCH_VIEWPORT_INSET: f32 = 8.0;
const TOUCH_MAX_HEIGHT: f32 = 560.0;
const TOUCH_VIEWPORT_FRACTION: f32 = 0.70;
const TOUCH_ROW_HEIGHT: f32 = 44.0;
const TOUCH_RADIUS: u8 = 7;
const HEADER_HEIGHT: f32 = 47.0;
const SEPARATOR_HEIGHT: f32 = 9.0;
const ICON_SIDE: f32 = 17.0;
const ROW_HORIZONTAL_PADDING: f32 = 7.0;
const ICON_LABEL_GAP: f32 = 7.0;
const SURFACE_BORDER_WIDTH: f32 = 2.0;
const KEYBOARD_ANCHOR_OFFSET: f32 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextInvocation {
    Pointer,
    Keyboard,
    TouchSheet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextAction {
    Properties,
    Rotate,
    Mirror,
    AdoptStimulus,
    ReadoptStimulus,
    SaveStimulus,
    OpenStimulusDefinition,
    Copy,
    Duplicate,
    Delete,
    DescendHierarchy,
    UpdateInstanceInterface,
    ReplaceInstance,
    CreateHierarchy,
    CreateSymbolFromPorts,
    PageSetup,
    FitContent,
    ShowInNetlist,
    Probe,
    OperatingPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextIcon {
    Sliders,
    Rotate,
    Mirror,
    Copy,
    Trash,
    Hierarchy,
    Sheet,
    Fit,
    Code,
    Probe,
    Waveform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextTarget {
    Component(u64),
    Wire(u64),
    Canvas,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextMenuBinding {
    pub source: EditorRequestSource,
    pub selection: Selection,
    pub tool: Tool,
    pub target: ContextTarget,
    pub click_pos: Point,
}

#[derive(Debug, Clone)]
pub struct ContextMenuRequest {
    pub binding: ContextMenuBinding,
    pub action: ContextAction,
}

#[derive(Debug, Clone)]
pub struct ContextCommand {
    pub action: ContextAction,
    pub icon: ContextIcon,
    pub label: &'static str,
    pub shortcut: String,
    pub enabled: bool,
    pub disabled_reason: &'static str,
}

/// Shared row/separator structure for app command bindings and resolved display rows.
#[derive(Debug, Clone, Copy)]
pub enum ContextEntry<T = ContextCommand> {
    Command(T),
    Separator,
}

#[derive(Debug, Clone)]
pub struct ContextMenuView {
    pub binding: ContextMenuBinding,
    pub summary: String,
    pub entries: Vec<ContextEntry>,
}

#[derive(Debug, Clone, Copy)]
pub struct ContextMenuOpening {
    pub secondary_clicked: bool,
    pub keyboard_open: bool,
    pub was_open: bool,
}

impl ContextMenuOpening {
    pub fn needs_contents(self) -> bool {
        self.was_open || self.secondary_clicked || self.keyboard_open
    }
}

pub fn opening(response: &Response, browser_keyboard_open: bool) -> ContextMenuOpening {
    ContextMenuOpening {
        secondary_clicked: response.secondary_clicked(),
        keyboard_open: (response.has_focus()
            && response
                .ctx
                .input_mut(|input| input.consume_key(Modifiers::SHIFT, Key::F10)))
            || browser_keyboard_open,
        was_open: Popup::is_id_open(&response.ctx, Popup::default_response_id(response)),
    }
}

#[derive(Debug, Default)]
pub struct ContextMenuOutput {
    pub request: Option<ContextMenuRequest>,
    pub open: bool,
}

pub fn close(response: &Response) {
    let popup_id = Popup::default_response_id(response);
    Popup::close_id(&response.ctx, popup_id);
    response.ctx.data_mut(|data| {
        data.remove::<ContextInvocation>(popup_id.with("invocation"));
        data.remove::<egui::Pos2>(popup_id.with("surface-anchor"));
        data.remove::<ContextMenuBinding>(popup_id.with("source"));
    });
}

pub fn show(
    response: &Response,
    opening: ContextMenuOpening,
    anchor: Option<egui::Pos2>,
    view: &ContextMenuView,
) -> ContextMenuOutput {
    let ctx = &response.ctx;
    let popup_id = Popup::default_response_id(response);
    let invocation_id = popup_id.with("invocation");
    let surface_anchor_id = popup_id.with("surface-anchor");
    let source_id = popup_id.with("source");
    let opened_this_frame = opening.secondary_clicked || opening.keyboard_open;
    if opened_this_frame {
        ctx.data_mut(|data| data.insert_temp(source_id, view.binding.clone()));
        if opening.secondary_clicked {
            if let Some(anchor) = anchor {
                let invocation = if response.long_touched() {
                    ContextInvocation::TouchSheet
                } else {
                    ContextInvocation::Pointer
                };
                ctx.data_mut(|data| {
                    data.insert_temp(invocation_id, invocation);
                    data.insert_temp(surface_anchor_id, anchor);
                });
            }
        } else {
            ctx.data_mut(|data| {
                data.insert_temp(invocation_id, ContextInvocation::Keyboard);
                data.insert_temp(surface_anchor_id, keyboard_surface_anchor(response.rect));
            });
        }
    } else if ctx
        .data(|data| data.get_temp::<ContextMenuBinding>(source_id))
        .as_ref()
        != Some(&view.binding)
    {
        close(response);
        return ContextMenuOutput::default();
    }
    let mut request = None;
    let invocation = ctx
        .data(|data| data.get_temp::<ContextInvocation>(invocation_id))
        .unwrap_or(ContextInvocation::Pointer);
    let geometry = SurfaceGeometry::resolve(ctx, invocation, &view.entries);
    let t = Tokens::get(ctx);
    let frame = Frame::new()
        .fill(t.color.bg_elevated)
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .corner_radius(CornerRadius::same(geometry.radius))
        .shadow(context_shadow(&t));

    let mut popup = Popup::context_menu(response)
        .id(popup_id)
        .width(geometry.width)
        .frame(frame);
    if opening.keyboard_open {
        popup = popup.open_memory(Some(egui::SetOpenCommand::Bool(true)));
    }
    match invocation {
        ContextInvocation::Pointer | ContextInvocation::Keyboard => {
            if let Some(requested) = ctx.data(|data| data.get_temp::<egui::Pos2>(surface_anchor_id))
            {
                let anchor = clamp_desktop_surface_origin(ctx.content_rect(), requested, geometry);
                popup = popup
                    .at_position(anchor)
                    .align(RectAlign::BOTTOM_START)
                    .align_alternatives(&[]);
            }
        }
        ContextInvocation::TouchSheet => {
            let screen = ctx.content_rect();
            let anchor = pos2(screen.center().x, screen.bottom() - TOUCH_VIEWPORT_INSET);
            popup = popup
                .at_position(anchor)
                .align(RectAlign::TOP)
                .align_alternatives(&[]);
        }
    }

    let restore_focus = opening.was_open && ctx.input(|input| input.key_pressed(Key::Escape));
    let popup_response = popup.show(|ui| {
        let content_width = (geometry.width - 2.0).max(1.0);
        ui.set_min_width(content_width);
        ui.set_max_width(content_width);
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

        ScrollArea::vertical()
            .id_salt(popup_id.with("scroll"))
            // CSS max-height includes the one-pixel border on both sides;
            // egui's Frame adds those outside its content UI.
            .max_height((geometry.max_height - SURFACE_BORDER_WIDTH).max(1.0))
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.set_min_width(content_width);
                ui.set_max_width(content_width);
                request = render_context_contents(ui, view, geometry.row_height, opened_this_frame);
            });
    });
    if let Some(popup_response) = &popup_response {
        ctx.accesskit_node_builder(popup_response.response.id, |node| {
            node.set_role(egui::accesskit::Role::Menu);
            node.set_label("Schematic selection");
        });
    }

    if restore_focus {
        response.request_focus();
    }
    let open = Popup::is_id_open(ctx, popup_id);
    if !open {
        close(response);
    }
    ContextMenuOutput { request, open }
}

#[derive(Clone)]
struct ContextRow {
    response: Response,
    enabled: bool,
}

pub fn render_context_contents(
    ui: &mut Ui,
    view: &ContextMenuView,
    row_height: f32,
    focus_first: bool,
) -> Option<ContextMenuRequest> {
    menu_header(ui, &view.summary);

    let mut rows = Vec::with_capacity(16);
    let mut keyboard_or_pointer_action = None;
    for entry in &view.entries {
        match entry {
            ContextEntry::Separator => menu_separator(ui),
            ContextEntry::Command(command) => {
                let enabled = command.enabled;
                let reason = command.disabled_reason;
                let shortcut = &command.shortcut;
                let response = ui
                    .push_id(("schematic-context-command", command.label), |ui| {
                        menu_item(ui, command, shortcut, enabled, reason, row_height)
                    })
                    .inner;
                // Consume a focused Enter/Space before consulting the egui
                // click synthesis. If `clicked` is checked first, the
                // short-circuit can execute the command while leaving the
                // same key event available to the canvas behind the menu.
                let activated =
                    enabled && (menu_row_keyboard_activated(ui, &response) || response.clicked());
                rows.push(ContextRow { response, enabled });
                if activated && keyboard_or_pointer_action.is_none() {
                    keyboard_or_pointer_action = Some(command.action);
                }
            }
        }
    }
    manage_menu_focus(ui, &rows, focus_first);
    keyboard_or_pointer_action.map(|action| {
        ui.close();
        ContextMenuRequest {
            binding: view.binding.clone(),
            action,
        }
    })
}

fn keyboard_surface_anchor(trigger: Rect) -> egui::Pos2 {
    pos2(
        trigger.left() + KEYBOARD_ANCHOR_OFFSET.min(trigger.width() * 0.5),
        trigger.top() + KEYBOARD_ANCHOR_OFFSET.min(trigger.height() * 0.5),
    )
}

fn clamp_desktop_surface_origin(
    screen: Rect,
    requested: egui::Pos2,
    geometry: SurfaceGeometry,
) -> egui::Pos2 {
    let min_x = screen.left() + DESKTOP_VIEWPORT_INSET;
    let min_y = screen.top() + DESKTOP_VIEWPORT_INSET;
    let max_x = (screen.right() - geometry.width - DESKTOP_VIEWPORT_INSET).max(min_x);
    let max_y = (screen.bottom() - geometry.outer_height() - DESKTOP_VIEWPORT_INSET).max(min_y);
    pos2(
        requested.x.clamp(min_x, max_x),
        requested.y.clamp(min_y, max_y),
    )
}

fn menu_row_keyboard_activated(ui: &Ui, response: &Response) -> bool {
    response.has_focus()
        && ui.input_mut(|input| {
            input.consume_key(Modifiers::NONE, Key::Enter)
                || input.consume_key(Modifiers::NONE, Key::Space)
        })
}

fn menu_header(ui: &mut Ui, summary: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), HEADER_HEIGHT), Sense::hover());
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            true,
            format!("Schematic selection: {summary}"),
        )
    });
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, t.color.bg_panel);
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border_strong),
    );
    painter.text(
        pos2(rect.left() + 10.0, rect.top() + 8.0),
        Align2::LEFT_TOP,
        "Schematic selection",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let summary_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let summary = fit_text(
        painter,
        summary,
        &summary_font,
        (rect.width() - 20.0).max(1.0),
        t.color.text_faint,
    );
    painter.text(
        pos2(rect.left() + 10.0, rect.top() + 25.0),
        Align2::LEFT_TOP,
        summary,
        summary_font,
        t.color.text_faint,
    );
}

fn menu_item(
    ui: &mut Ui,
    command: &ContextCommand,
    shortcut: &str,
    enabled: bool,
    disabled_reason: &'static str,
    row_height: f32,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), row_height),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, command.label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::MenuItem);
        node.set_label(command.label);
        if !enabled {
            node.set_disabled();
            node.set_description(disabled_reason);
        }
        if enabled && !shortcut.is_empty() {
            node.set_keyboard_shortcut(shortcut);
        }
    });

    let focused = enabled && response.has_focus();
    let hovered = enabled && response.hovered();
    if response.is_pointer_button_down_on() && enabled {
        ui.painter().rect_filled(rect, 3.0, t.color.bg_active);
    } else if hovered || focused {
        ui.painter().rect_filled(rect, 3.0, t.color.bg_hover);
    }

    let row_color = if hovered || focused {
        t.color.text
    } else {
        t.color.text_dim
    };
    let row_color = if enabled {
        row_color
    } else {
        row_color.gamma_multiply(0.4)
    };
    let faint = if enabled {
        t.color.text_faint
    } else {
        t.color.text_faint.gamma_multiply(0.4)
    };
    let icon_rect = Rect::from_min_size(
        pos2(
            rect.left() + ROW_HORIZONTAL_PADDING,
            rect.center().y - ICON_SIDE * 0.5,
        ),
        vec2(ICON_SIDE, ICON_SIDE),
    );
    command.icon.paint(ui.painter(), icon_rect, row_color);

    let shortcut_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let shortcut_width = if shortcut.is_empty() {
        0.0
    } else {
        ui.painter()
            .layout_no_wrap(shortcut.to_owned(), shortcut_font.clone(), faint)
            .size()
            .x
    };
    if !shortcut.is_empty() {
        ui.painter().text(
            pos2(rect.right() - ROW_HORIZONTAL_PADDING, rect.center().y),
            Align2::RIGHT_CENTER,
            shortcut,
            shortcut_font,
            faint,
        );
    }

    let label_left = icon_rect.right() + ICON_LABEL_GAP;
    let label_right = if !shortcut.is_empty() {
        rect.right() - ROW_HORIZONTAL_PADDING - shortcut_width - ICON_LABEL_GAP
    } else {
        rect.right() - ROW_HORIZONTAL_PADDING
    };
    let label_clip = Rect::from_min_max(
        pos2(label_left, rect.top()),
        pos2(label_right.max(label_left), rect.bottom()),
    );
    ui.painter().with_clip_rect(label_clip).text(
        pos2(label_left, rect.center().y),
        Align2::LEFT_CENTER,
        command.label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        row_color,
    );

    theme::paint_focus_ring(ui, &response, rect);

    if enabled {
        response
    } else {
        response.on_hover_text(disabled_reason)
    }
}

fn menu_separator(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), SEPARATOR_HEIGHT), Sense::hover());
    response.widget_info(|| WidgetInfo::new(WidgetType::Other));
    ui.painter().hline(
        egui::Rangef::new(rect.left() + 5.0, rect.right() - 5.0),
        rect.center().y,
        Stroke::new(1.0, t.color.border),
    );
}

fn manage_menu_focus(ui: &mut Ui, rows: &[ContextRow], focus_first: bool) {
    let enabled: Vec<&ContextRow> = rows.iter().filter(|row| row.enabled).collect();
    let Some(first) = enabled.first() else {
        return;
    };
    if focus_first {
        first.response.request_focus();
        return;
    }

    let current = enabled.iter().position(|row| row.response.has_focus());
    let movement = ui.input_mut(|input| {
        let modifiers = input.modifiers;
        if input.consume_key(modifiers, Key::ArrowDown) {
            Some(FocusMove::Next)
        } else if input.consume_key(modifiers, Key::ArrowUp) {
            Some(FocusMove::Previous)
        } else if input.consume_key(modifiers, Key::Home) {
            Some(FocusMove::First)
        } else if input.consume_key(modifiers, Key::End) {
            Some(FocusMove::Last)
        } else {
            None
        }
    });
    let Some(movement) = movement else {
        return;
    };
    let next = match movement {
        FocusMove::Next => current.map_or(0, |index| (index + 1) % enabled.len()),
        FocusMove::Previous => current.map_or(enabled.len() - 1, |index| {
            (index + enabled.len() - 1) % enabled.len()
        }),
        FocusMove::First => 0,
        FocusMove::Last => enabled.len() - 1,
    };
    enabled[next].response.request_focus();
    enabled[next].response.scroll_to_me(None);
}

fn paint_copy_icon(painter: &egui::Painter, rect: Rect, color: Color32) {
    let scale = rect.width().min(rect.height()) / 24.0;
    let origin = rect.center() - vec2(12.0, 12.0) * scale;
    let map = |x: f32, y: f32| origin + vec2(x, y) * scale;
    let stroke = Stroke::new((1.7 * rect.width() / 16.0).max(1.0), color);
    painter.rect_stroke(
        Rect::from_min_max(map(8.0, 4.0), map(20.0, 16.0)),
        1.0,
        stroke,
        StrokeKind::Middle,
    );
    painter.rect_stroke(
        Rect::from_min_max(map(4.0, 8.0), map(16.0, 20.0)),
        1.0,
        stroke,
        StrokeKind::Middle,
    );
}

fn fit_text(
    painter: &egui::Painter,
    text: &str,
    font: &egui::FontId,
    max_width: f32,
    color: Color32,
) -> String {
    if painter
        .layout_no_wrap(text.to_owned(), font.clone(), color)
        .size()
        .x
        <= max_width
    {
        return text.to_owned();
    }
    let mut output: String = text.chars().collect();
    while !output.is_empty() {
        output.pop();
        let candidate = format!("{}…", output.trim_end());
        if painter
            .layout_no_wrap(candidate.clone(), font.clone(), color)
            .size()
            .x
            <= max_width
        {
            return candidate;
        }
    }
    "…".to_owned()
}

fn context_shadow(active_tokens: &Tokens) -> Shadow {
    let color = if active_tokens.mode == tokens::Mode::Light {
        // rgb(41 46 50 / 22%), premultiplied to egui's Color32 storage.
        Color32::from_rgba_premultiplied(9, 10, 11, 56)
    } else {
        Color32::from_rgba_premultiplied(0, 0, 0, 97)
    };
    Shadow {
        offset: [0, 16],
        blur: 40,
        spread: 0,
        color,
    }
}

#[derive(Debug, Clone, Copy)]
enum FocusMove {
    Next,
    Previous,
    First,
    Last,
}

impl ContextIcon {
    fn paint(self, painter: &egui::Painter, rect: Rect, color: Color32) {
        match self {
            Self::Sliders => WorkbenchIcon::Sliders.paint(painter, rect, color),
            Self::Rotate => WorkbenchIcon::Rotate.paint(painter, rect, color),
            Self::Mirror => WorkbenchIcon::Mirror.paint(painter, rect, color),
            Self::Copy => paint_copy_icon(painter, rect, color),
            Self::Trash => Icon::Trash.paint(painter, rect, color),
            Self::Hierarchy => WorkbenchIcon::Instance.paint(painter, rect, color),
            Self::Sheet => WorkbenchIcon::Layers.paint(painter, rect, color),
            Self::Fit => WorkbenchIcon::ZoomFit.paint(painter, rect, color),
            Self::Code => WorkbenchIcon::Code.paint(painter, rect, color),
            Self::Probe => WorkbenchIcon::Probe.paint(painter, rect, color),
            Self::Waveform => WorkbenchIcon::Simulate.paint(painter, rect, color),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SurfaceGeometry {
    width: f32,
    max_height: f32,
    row_height: f32,
    radius: u8,
    /// How many rows and rules the menu being placed is made of.
    commands: u32,
    separators: u32,
}

impl SurfaceGeometry {
    fn resolve<T>(
        ctx: &Context,
        invocation: ContextInvocation,
        entries: &[ContextEntry<T>],
    ) -> Self {
        Self::for_viewport(ctx.content_rect().size(), invocation, entries)
    }

    fn for_viewport<T>(
        viewport: egui::Vec2,
        invocation: ContextInvocation,
        entries: &[ContextEntry<T>],
    ) -> Self {
        let (commands, separators) = entries.iter().fold(
            (0_u32, 0_u32),
            |(commands, separators), entry| match entry {
                ContextEntry::Command(_) => (commands + 1, separators),
                ContextEntry::Separator => (commands, separators + 1),
            },
        );
        if invocation == ContextInvocation::TouchSheet {
            Self {
                width: (viewport.x - 2.0 * TOUCH_VIEWPORT_INSET).clamp(1.0, TOUCH_MAX_WIDTH),
                max_height: (viewport.y * TOUCH_VIEWPORT_FRACTION).min(TOUCH_MAX_HEIGHT),
                row_height: TOUCH_ROW_HEIGHT,
                radius: TOUCH_RADIUS,
                commands,
                separators,
            }
        } else {
            Self {
                width: DESKTOP_WIDTH,
                max_height: DESKTOP_MAX_HEIGHT.min((viewport.y - 12.0).max(1.0)),
                row_height: DESKTOP_ROW_HEIGHT,
                radius: DESKTOP_RADIUS,
                commands,
                separators,
            }
        }
    }

    fn outer_height(self) -> f32 {
        (HEADER_HEIGHT
            + self.commands as f32 * self.row_height
            + self.separators as f32 * SEPARATOR_HEIGHT
            + SURFACE_BORDER_WIDTH)
            .min(self.max_height)
    }
}

#[cfg(test)]
mod tests;
