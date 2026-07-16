//! The RSpice modal primitive.
//!
//! Every dialog in the application is the same three-part surface over a
//! scrim:
//!
//! - **Scrim** — full-viewport canvas-black wash that blocks interaction
//!   with everything underneath. Clicking it does not dismiss; `Esc` does.
//! - **Surface** — each callsite declares a mockup-owned purpose: transaction,
//!   manager, wide workflow, or capability review. Purpose controls exact
//!   geometry, fill, corner radius, viewport gutter, and narrow behavior; no
//!   retired generic small/medium/large shell remains.
//! - **Header / footer** — mono uppercase kicker + semibold title + close
//!   on top; footer is `[ghost] [secondary] [primary]` with the primary
//!   always rightmost, always exactly one, accent-filled (or `err` when
//!   destructive), plus an optional mono hint on the left.
//!
//! Keys: `Esc` cancels, `Enter` activates the primary when it is enabled.
//! Dialogs edit a draft and commit on the primary — the primary and
//! cancel are never the same operation.

use egui::{
    Context, FocusDirection, Frame, Id, Key, Margin, Modifiers, Order, Popup, Rect, Sense, Stroke,
    Ui, UiKind, WidgetInfo, WidgetType, vec2,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Mockup-owned dialog surface purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogSize {
    /// Transactional edits and confirmations: 760 pt wide, content-height,
    /// capped at 760 pt, with the full-viewport shell applied at 560 pt.
    Transaction,
    /// Mockup-authored Simulation Studio transactions: 760 pt wide and
    /// content-height, with the two-column workflow retained through tablet
    /// widths and the full-viewport shell applied at 560 pt.
    SimulationWorkflow,
    /// Browsers and settings managers: 760 × 530 pt, with the mockup's
    /// 28/34 pt viewport gutters and 8 pt phone inset.
    Manager,
    /// Simulation Studio's mockup-owned analysis/workflow catalog: 1180 ×
    /// 780 pt, anchored 12 pt below the viewport top, and reduced to a 4 pt
    /// perimeter gutter on phone-sized viewports.
    AnalysisCatalog,
    /// Wide numerical/setup workflows: 980 pt wide, content-height capped at
    /// 760 pt, and edge-to-edge at the mockup's 820 pt breakpoint.
    WideWorkflow,
    /// Governed capability matrices: 1040 pt wide, content-height capped at
    /// 760 pt, edge-to-edge at the mockup's 820 pt breakpoint.
    CapabilityReview,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DialogSurfaceSpec {
    width: f32,
    max_height: f32,
    horizontal_inset: f32,
    vertical_inset: f32,
    narrow_max_width: f32,
    narrow_inset: f32,
    narrow_vertical_inset: f32,
    cap_narrow_height: bool,
    edge_to_edge_narrow: bool,
    fill_narrow_viewport: bool,
    fill_height: bool,
    app_background: bool,
    radius: f32,
    top_anchored: bool,
}

impl DialogSize {
    const fn spec(self) -> DialogSurfaceSpec {
        match self {
            Self::Transaction => DialogSurfaceSpec {
                width: 760.0,
                max_height: 760.0,
                horizontal_inset: 24.0,
                vertical_inset: 24.0,
                narrow_max_width: 560.0,
                narrow_inset: 0.0,
                narrow_vertical_inset: 0.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: true,
                fill_narrow_viewport: true,
                fill_height: false,
                app_background: true,
                radius: 4.0,
                top_anchored: false,
            },
            Self::SimulationWorkflow => DialogSurfaceSpec {
                width: 760.0,
                max_height: 760.0,
                horizontal_inset: 24.0,
                vertical_inset: 24.0,
                narrow_max_width: 560.0,
                narrow_inset: 0.0,
                narrow_vertical_inset: 0.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: true,
                fill_narrow_viewport: true,
                fill_height: false,
                app_background: true,
                radius: 4.0,
                top_anchored: false,
            },
            Self::Manager => DialogSurfaceSpec {
                width: 760.0,
                max_height: 530.0,
                horizontal_inset: 28.0,
                vertical_inset: 34.0,
                narrow_max_width: 560.0,
                narrow_inset: 8.0,
                narrow_vertical_inset: 8.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: false,
                fill_narrow_viewport: true,
                fill_height: true,
                app_background: false,
                radius: 8.0,
                top_anchored: false,
            },
            Self::AnalysisCatalog => DialogSurfaceSpec {
                width: 1_180.0,
                max_height: 780.0,
                horizontal_inset: 24.0,
                vertical_inset: 24.0,
                narrow_max_width: 560.0,
                narrow_inset: 8.0,
                narrow_vertical_inset: 24.0,
                cap_narrow_height: true,
                edge_to_edge_narrow: false,
                fill_narrow_viewport: true,
                fill_height: true,
                app_background: false,
                radius: 8.0,
                top_anchored: true,
            },
            Self::WideWorkflow => DialogSurfaceSpec {
                width: 980.0,
                max_height: 760.0,
                horizontal_inset: 24.0,
                vertical_inset: 24.0,
                narrow_max_width: 820.0,
                narrow_inset: 0.0,
                narrow_vertical_inset: 0.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: true,
                fill_narrow_viewport: true,
                fill_height: false,
                app_background: true,
                radius: 4.0,
                top_anchored: false,
            },
            Self::CapabilityReview => DialogSurfaceSpec {
                width: 1040.0,
                max_height: 760.0,
                horizontal_inset: 24.0,
                vertical_inset: 24.0,
                narrow_max_width: 820.0,
                narrow_inset: 0.0,
                narrow_vertical_inset: 0.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: true,
                fill_narrow_viewport: true,
                fill_height: false,
                app_background: true,
                radius: 4.0,
                top_anchored: false,
            },
        }
    }
}

const WORKFLOW_HEADER_MIN_HEIGHT: f32 = 57.0;
const WORKFLOW_HEADER_HORIZONTAL_MARGIN: i8 = 15;
const WORKFLOW_FOOTER_HORIZONTAL_MARGIN: i8 = 12;
const WORKFLOW_FOOTER_VERTICAL_MARGIN: i8 = 10;
const DIALOG_CLOSE_TARGET_WIDTH: f32 = 28.0;
const DIALOG_CLOSE_TARGET_HEIGHT: f32 = 27.0;
const TOUCH_TARGET_SIDE: f32 = 44.0;

/// Geometry resolved before the dialog is painted. Keeping this calculation
/// independent of egui's layout pass makes every mockup breakpoint and gutter
/// explicit and testable.
#[derive(Debug, Clone, Copy, PartialEq)]
struct DialogLayout {
    surface_rect: Rect,
    fill_height: bool,
    app_background: bool,
    radius: f32,
    narrow: bool,
}

impl DialogLayout {
    fn resolve(size: DialogSize, screen: Rect, measured_height: Option<f32>) -> Self {
        let spec = size.spec();
        let narrow = screen.width() <= spec.narrow_max_width;
        let surface_rect = if narrow && spec.edge_to_edge_narrow {
            screen
        } else {
            let horizontal_inset = if narrow {
                spec.narrow_inset
            } else {
                spec.horizontal_inset
            };
            let vertical_inset = if narrow {
                spec.narrow_vertical_inset
            } else {
                spec.vertical_inset
            };
            let surface_size = vec2(
                if narrow && spec.fill_narrow_viewport {
                    (screen.width() - horizontal_inset).max(1.0)
                } else {
                    spec.width.min((screen.width() - horizontal_inset).max(1.0))
                },
                if narrow && spec.fill_narrow_viewport {
                    let available = (screen.height() - vertical_inset).max(1.0);
                    if spec.cap_narrow_height {
                        available.min(spec.max_height)
                    } else {
                        available
                    }
                } else {
                    measured_height
                        .filter(|_| !spec.fill_height)
                        .unwrap_or(spec.max_height)
                        .min(spec.max_height)
                        .min((screen.height() - vertical_inset).max(1.0))
                },
            );
            if spec.top_anchored {
                Rect::from_min_size(
                    egui::pos2(
                        screen.center().x - surface_size.x * 0.5,
                        screen.top() + vertical_inset * 0.5,
                    ),
                    surface_size,
                )
            } else {
                Rect::from_center_size(screen.center(), surface_size)
            }
        };

        Self {
            surface_rect,
            fill_height: spec.fill_height,
            app_background: spec.app_background,
            radius: if narrow && spec.edge_to_edge_narrow {
                0.0
            } else {
                spec.radius
            },
            narrow,
        }
    }
}

/// Temporarily raises the design-system control height for coarse-pointer and
/// narrow workflow compositions, then restores the caller's selected density.
struct ControlHeightOverride {
    ctx: Context,
    original: Option<Tokens>,
}

impl ControlHeightOverride {
    fn new(ctx: &Context, minimum: Option<f32>) -> Self {
        let original = (*Tokens::get(ctx)).clone();
        let Some(minimum) = minimum.filter(|minimum| original.metrics.ctl_h < *minimum) else {
            return Self {
                ctx: ctx.clone(),
                original: None,
            };
        };
        let mut adjusted = original.clone();
        adjusted.metrics.ctl_h = minimum;
        adjusted.install(ctx);
        Self {
            ctx: ctx.clone(),
            original: Some(original),
        }
    }
}

impl Drop for ControlHeightOverride {
    fn drop(&mut self) {
        if let Some(original) = self.original.take() {
            original.install(&self.ctx);
        }
    }
}

/// What the user chose this frame.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DialogChoice {
    /// Still open, nothing chosen.
    #[default]
    None,
    /// The accent (or destructive) primary action.
    Primary,
    /// The plain secondary button, when present.
    Secondary,
    /// The ghost button, when present.
    Ghost,
    /// Dismissed: `Esc` or the ✕ close control.
    Cancelled,
}

/// Control that receives focus when a dialog first opens.
///
/// [`BodyControl`](Self::BodyControl) is resolved by
/// [`Dialog::show_with_initial_body_focus`], after the body has returned the
/// exact response id of its preferred control. Every unavailable target falls
/// back to the modal container, so focus can never escape to the obscured
/// workspace.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DialogInitialFocus {
    /// Focus the modal container. This preserves the established default.
    #[default]
    Container,
    /// Focus the body control returned by `show_with_initial_body_focus`.
    BodyControl,
    /// Focus the header close control.
    Close,
    /// Focus the primary footer action.
    Primary,
    /// Focus the optional secondary footer action.
    Secondary,
    /// Focus the optional ghost footer action.
    Ghost,
}

#[derive(Debug, Clone, Copy, Default)]
struct DialogFocusState {
    prior_focus: Option<Id>,
    last_seen_pass: u64,
}

#[derive(Debug, Clone, Copy, Default)]
struct DialogRenderedFocus {
    close: Option<Id>,
    primary: Option<Id>,
    secondary: Option<Id>,
    ghost: Option<Id>,
    body: Option<Id>,
}

impl DialogRenderedFocus {
    fn requested(self, target: DialogInitialFocus) -> Option<Id> {
        match target {
            DialogInitialFocus::Container => None,
            DialogInitialFocus::BodyControl => self.body,
            DialogInitialFocus::Close => self.close,
            DialogInitialFocus::Primary => self.primary,
            DialogInitialFocus::Secondary => self.secondary,
            DialogInitialFocus::Ghost => self.ghost,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct DialogHeaderOutput {
    closed: bool,
    close_id: Option<Id>,
}

#[derive(Debug, Clone, Copy, Default)]
struct DialogFooterOutput {
    choice: DialogChoice,
    primary_id: Option<Id>,
    secondary_id: Option<Id>,
    ghost_id: Option<Id>,
}

/// Mockup transaction strip rendered between a workflow body and its footer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogTransactionTone {
    Progress,
    Complete,
    Error,
}

#[derive(Debug, Clone, Copy)]
struct DialogTransactionState<'a> {
    tone: DialogTransactionTone,
    title: &'a str,
    detail: &'a str,
}

/// Declarative description of one modal frame.
pub struct Dialog<'a> {
    kicker: &'a str,
    title: &'a str,
    description: Option<&'a str>,
    size: DialogSize,
    primary: &'a str,
    primary_enabled: bool,
    primary_on_enter: bool,
    destructive: bool,
    secondary: Option<&'a str>,
    ghost: Option<&'a str>,
    ghost_enabled: bool,
    hint: Option<&'a str>,
    transaction_state: Option<DialogTransactionState<'a>>,
    body_scroll_offset: Option<&'a mut f32>,
    flush_body: bool,
    manual_body_scroll: bool,
    note_only_footer: bool,
    initial_focus: DialogInitialFocus,
}

impl<'a> Dialog<'a> {
    /// A dialog with the given header kicker (domain, e.g. "Simulate"),
    /// title, and primary-action label.
    pub fn new(kicker: &'a str, title: &'a str, primary: &'a str) -> Self {
        Self {
            kicker,
            title,
            description: None,
            size: DialogSize::Transaction,
            primary,
            primary_enabled: true,
            primary_on_enter: true,
            destructive: false,
            secondary: None,
            ghost: None,
            ghost_enabled: true,
            hint: None,
            transaction_state: None,
            body_scroll_offset: None,
            flush_body: false,
            manual_body_scroll: false,
            note_only_footer: false,
            initial_focus: DialogInitialFocus::Container,
        }
    }

    /// Describe the dialog's purpose and scope to assistive technology.
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    /// Choose the control that receives focus when the dialog opens.
    pub fn initial_focus(mut self, target: DialogInitialFocus) -> Self {
        self.initial_focus = target;
        self
    }

    /// Disable the Enter→primary mapping — for dialogs whose body owns the
    /// Enter key (multiline editors, where Enter must insert a newline).
    pub fn primary_on_enter(mut self, on: bool) -> Self {
        self.primary_on_enter = on;
        self
    }

    /// Surface width.
    pub fn size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }

    /// Disable the primary action (validation pending). `Enter` is inert
    /// while disabled.
    pub fn primary_enabled(mut self, enabled: bool) -> Self {
        self.primary_enabled = enabled;
        self
    }

    /// Render the primary in the destructive (`err`) treatment.
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    /// Plain secondary button, left of the primary.
    pub fn secondary(mut self, label: &'a str) -> Self {
        self.secondary = Some(label);
        self
    }

    /// Ghost button, leftmost in the footer (Revert, Cancel).
    pub fn ghost(mut self, label: &'a str) -> Self {
        self.ghost = Some(label);
        self
    }

    /// Enable or disable the optional ghost action.
    pub fn ghost_enabled(mut self, enabled: bool) -> Self {
        self.ghost_enabled = enabled;
        self
    }

    /// Mono footer hint (validation count, shortcut reminder).
    pub fn hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Show the canonical workflow transaction strip immediately above the
    /// footer. It is intentionally absent in the normal idle state.
    pub fn transaction_state(
        mut self,
        tone: DialogTransactionTone,
        title: &'a str,
        detail: &'a str,
    ) -> Self {
        self.transaction_state = Some(DialogTransactionState {
            tone,
            title,
            detail,
        });
        self
    }

    /// Restore and retain the body's vertical scroll offset. Managers whose
    /// route can be left and revisited use this to preserve review context.
    pub fn body_scroll_offset(mut self, offset: &'a mut f32) -> Self {
        self.body_scroll_offset = Some(offset);
        self
    }

    /// Remove the standard body inset. Wide workflow managers use this when
    /// their first child owns its own edge-to-edge grid and section padding.
    pub fn flush_body(mut self) -> Self {
        self.flush_body = true;
        self
    }

    /// Give the body its exact available track instead of wrapping it in the
    /// standard dialog scroll area. Catalogs use this to keep search controls
    /// fixed while their result region owns the only scrollbar.
    pub fn manual_body_scroll(mut self) -> Self {
        self.manual_body_scroll = true;
        self
    }

    /// Render the footer as the mockup's informational note strip, without a
    /// redundant action button. The header close control and Escape remain
    /// the dialog's dismissal affordances.
    pub fn note_only_footer(mut self) -> Self {
        self.note_only_footer = true;
        self
    }

    /// Show the dialog and render `body` into the scrollable middle
    /// region. Returns what the user chose this frame; the caller owns
    /// open/close state and reacts to the choice.
    pub fn show(self, ctx: &Context, body: impl FnOnce(&mut Ui)) -> DialogChoice {
        self.show_with_initial_body_focus(ctx, |ui| {
            body(ui);
            None
        })
    }

    /// Show the dialog and let its body identify the exact control to focus on
    /// first. The returned id is used only when
    /// [`DialogInitialFocus::BodyControl`] is selected and only on the opening
    /// pass; subsequent renders preserve the user's current focus.
    pub fn show_with_initial_body_focus(
        mut self,
        ctx: &Context,
        body: impl FnOnce(&mut Ui) -> Option<Id>,
    ) -> DialogChoice {
        let t = Tokens::get(ctx);
        let c = t.color;
        let screen = ctx.screen_rect();
        let id = Id::new(("rspice.dialog", self.title));
        let measured_height_id = id.with("measured-surface-height");
        let measured_height = ctx.data(|data| data.get_temp::<f32>(measured_height_id));
        let layout = DialogLayout::resolve(self.size, screen, measured_height);
        let large_targets = layout.narrow
            || (self.size == DialogSize::AnalysisCatalog && screen.width() <= 820.0)
            || ctx.input(|input| input.has_touch_screen());
        let hide_close_only_footer = self.size == DialogSize::CapabilityReview;
        let _control_height_override =
            ControlHeightOverride::new(ctx, large_targets.then_some(TOUCH_TARGET_SIDE));
        let focus_id = id.with("move");
        let focus_state_id = id.with("focus-state");

        let area = egui::Area::new(id)
            .kind(UiKind::Modal)
            .sense(Sense::focusable_noninteractive())
            .order(Order::Foreground)
            .fixed_pos(screen.min);
        let modal_layer = area.layer();
        let any_popup_open = Popup::is_any_open(ctx);
        let is_top_modal = ctx.memory_mut(|memory| {
            memory.set_modal_layer(modal_layer);
            memory.top_modal_layer() == Some(modal_layer)
        });

        let opened_this_pass = begin_dialog_focus(ctx, focus_state_id);
        if opened_this_pass || !focus_is_within_modal(ctx, modal_layer) {
            // Claim focus before any dialog fields are rendered. A body field
            // may intentionally request focus later in this pass.
            ctx.memory_mut(|memory| {
                memory.request_focus(focus_id);
                if opened_this_pass {
                    // Do not let the Tab direction that triggered opening
                    // carry a stale target from the underlying focus chain.
                    memory.move_focus(FocusDirection::None);
                }
            });
        }

        let mut choice = DialogChoice::None;
        let mut rendered_focus = DialogRenderedFocus::default();
        let mut rendered_surface_height = None;
        let area_response = area.show(ctx, |ui| {
            // Scrim: swallow pointer interaction with everything below.
            // accessibility-pointer-shim: the scrim consumes pointer
            // gestures but is deliberately absent from keyboard/AT order.
            ui.allocate_rect(
                screen,
                Sense::click_and_drag().difference(Sense::focusable_noninteractive()),
            );
            let backdrop = if t.mode == tokens::Mode::Dark {
                egui::Color32::from_rgba_unmultiplied(2, 6, 8, 158)
            } else {
                egui::Color32::from_rgba_unmultiplied(41, 46, 50, 97)
            };
            ui.painter().rect_filled(screen, 0.0, backdrop);

            let width = layout.surface_rect.width();
            let max_height = layout.surface_rect.height();
            let mut surface = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(layout.surface_rect)
                    .layout(egui::Layout::top_down(egui::Align::Min)),
            );
            surface.set_width(width);
            if layout.fill_height {
                surface.set_min_height(max_height);
            }

            let surface_output = Frame::NONE
                .fill(if layout.app_background {
                    c.bg_app
                } else {
                    c.bg_elevated
                })
                .stroke(Stroke::new(1.0, c.border_strong))
                .rounding(layout.radius)
                // Every currently implemented mockup dialog inherits
                // `--shadow`. The stronger `--shadow-dialog` elevation is
                // reserved for DRC workflows, which are not represented by
                // this generic surface purpose.
                .shadow(t.shadow())
                .show(&mut surface, |ui| {
                    ui.set_width(width);
                    if layout.fill_height {
                        ui.set_min_height(max_height);
                    }
                    let header_top = ui.cursor().top();
                    let header = self.header(ui, &t, large_targets);
                    rendered_focus.close = header.close_id;
                    if header.closed {
                        choice = DialogChoice::Cancelled;
                    }
                    let header_height = ui.cursor().top() - header_top;
                    let footer_height = self.footer_height(hide_close_only_footer, large_targets);
                    let transaction_height = if self.transaction_state.is_some() {
                        37.0
                    } else {
                        0.0
                    };
                    let body_max_height =
                        (max_height - header_height - footer_height - transaction_height).max(1.0);
                    let initial_scroll_offset = self
                        .body_scroll_offset
                        .as_deref()
                        .copied()
                        .unwrap_or_default();
                    if self.manual_body_scroll {
                        let body_output = ui.allocate_ui_with_layout(
                            vec2(ui.available_width(), body_max_height),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                Frame::NONE
                                    .fill(if layout.app_background {
                                        c.bg_app
                                    } else {
                                        c.bg_elevated
                                    })
                                    .inner_margin(if self.flush_body {
                                        Margin::same(0)
                                    } else {
                                        Margin::same(12)
                                    })
                                    .show(ui, body)
                                    .inner
                            },
                        );
                        rendered_focus.body = body_output.inner;
                    } else {
                        let body_scroll = egui::ScrollArea::vertical()
                            .id_salt(id.with("body"))
                            .vertical_scroll_offset(initial_scroll_offset)
                            .max_height(body_max_height)
                            .min_scrolled_height(if layout.fill_height {
                                body_max_height
                            } else {
                                64.0
                            })
                            .auto_shrink([false, !layout.fill_height]);
                        let body_output = body_scroll.show(ui, |ui| {
                            Frame::NONE
                                .fill(if layout.app_background {
                                    c.bg_app
                                } else {
                                    c.bg_elevated
                                })
                                .inner_margin(if self.flush_body {
                                    Margin::same(0)
                                } else {
                                    Margin::same(12)
                                })
                                .show(ui, body)
                                .inner
                        });
                        rendered_focus.body = body_output.inner;
                        if let Some(offset) = self.body_scroll_offset.as_deref_mut() {
                            *offset = body_output.state.offset.y;
                        }
                    }
                    self.transaction_strip(ui, &t);
                    let footer = self.footer(ui, &t, hide_close_only_footer, large_targets, width);
                    rendered_focus.primary = footer.primary_id;
                    rendered_focus.secondary = footer.secondary_id;
                    rendered_focus.ghost = footer.ghost_id;
                    match footer.choice {
                        DialogChoice::None => {}
                        chosen => choice = chosen,
                    }
                });
            rendered_surface_height = Some(surface_output.response.rect.height());
        });

        if !layout.fill_height
            && let Some(height) = rendered_surface_height
        {
            let maximum = self.size.spec().max_height.min(screen.height().max(1.0));
            let height = height.clamp(1.0, maximum);
            if measured_height.is_none_or(|previous| (previous - height).abs() > 0.5) {
                ctx.data_mut(|data| data.insert_temp(measured_height_id, height));
                ctx.request_repaint();
            }
        }

        let response = area_response.response;
        let enabled = response.enabled();
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Window, enabled, self.title));
        ctx.accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Dialog);
            node.set_label(self.title);
            if let Some(description) = self.description {
                node.set_description(description);
            }
            node.set_modal();
        });

        if opened_this_pass && self.initial_focus != DialogInitialFocus::Container {
            let target = rendered_focus
                .requested(self.initial_focus)
                .unwrap_or(focus_id);
            ctx.memory_mut(|memory| memory.request_focus(target));
        }
        if choice == DialogChoice::None && response.should_close() {
            choice = DialogChoice::Cancelled;
        }

        // Handle keys after the contents. This lets a focused secondary,
        // ghost, close, or body button own Enter without also firing the
        // primary action. Single-line text fields retain the canonical
        // Enter-to-submit behavior; multiline owners opt out explicitly with
        // `primary_on_enter(false)`.
        if choice == DialogChoice::None && is_top_modal && !any_popup_open {
            if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape)) {
                choice = DialogChoice::Cancelled;
            } else if self.primary_enabled
                && self.primary_on_enter
                && !focused_control_owns_enter(ctx, focus_id)
                && ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Enter))
            {
                choice = DialogChoice::Primary;
            }
        }

        if choice != DialogChoice::None {
            // Re-measure content-height surfaces every time they are opened.
            // Keeping a previous session's height could otherwise force a
            // later workflow state into an unnecessarily short scroll area.
            ctx.data_mut(|data| data.remove_temp::<f32>(measured_height_id));
            restore_dialog_focus(ctx, focus_state_id, focus_id, modal_layer);
        }

        choice
    }

    /// Header strip; returns `true` when the close control fired.
    fn header(&self, ui: &mut Ui, t: &Tokens, large_targets: bool) -> DialogHeaderOutput {
        let c = t.color;
        let mut closed = false;
        let mut close_id = None;
        Frame::NONE
            .fill(if self.size == DialogSize::AnalysisCatalog {
                c.bg_elevated
            } else {
                c.bg_panel
            })
            .inner_margin(Margin::symmetric(WORKFLOW_HEADER_HORIZONTAL_MARGIN, 0))
            .show(ui, |ui| {
                let header_width = ui.available_width();
                ui.allocate_ui_with_layout(
                    vec2(header_width, WORKFLOW_HEADER_MIN_HEIGHT),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 10.0;
                        let close_size = if large_targets {
                            vec2(TOUCH_TARGET_SIDE, TOUCH_TARGET_SIDE)
                        } else {
                            vec2(DIALOG_CLOSE_TARGET_WIDTH, DIALOG_CLOSE_TARGET_HEIGHT)
                        };
                        let text_width = (ui.available_width() - close_size.x - 10.0).max(1.0);
                        let mut eyebrow = egui::text::LayoutJob::default();
                        eyebrow.append(
                            &self.kicker.to_uppercase(),
                            0.0,
                            egui::TextFormat {
                                font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
                                color: c.text_faint,
                                extra_letter_spacing: 0.09 * tokens::FS_0,
                                ..Default::default()
                            },
                        );
                        ui.allocate_ui_with_layout(
                            vec2(text_width, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.set_width(text_width);
                                ui.spacing_mut().item_spacing.y = 3.0;
                                ui.add(egui::Label::new(eyebrow).wrap());
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(self.title)
                                            .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                                            .color(c.text),
                                    )
                                    .wrap(),
                                );
                            },
                        );
                        let close_response =
                            crate::ui::widgets::IconButton::new(crate::ui::icons::Icon::Close)
                                .size(close_size.x, close_size.y)
                                .tooltip("Close (Esc)")
                                .show(ui);
                        close_id = Some(close_response.id);
                        if close_response.clicked() {
                            closed = true;
                        }
                    },
                );
            });
        let line_y = ui.cursor().top();
        ui.painter().hline(
            ui.max_rect().x_range(),
            line_y,
            Stroke::new(1.0, t.color.border),
        );
        DialogHeaderOutput { closed, close_id }
    }

    /// Footer strip with the canonical button order.
    fn footer_height(&self, hide_close_only_footer: bool, _large_targets: bool) -> f32 {
        if self.note_only_footer {
            return 48.0;
        }
        if self.hides_close_only_footer(hide_close_only_footer) {
            return 0.0;
        }
        48.0
    }

    fn transaction_strip(&self, ui: &mut Ui, t: &Tokens) {
        let Some(transaction) = self.transaction_state else {
            return;
        };
        let color = match transaction.tone {
            DialogTransactionTone::Progress => t.color.accent,
            DialogTransactionTone::Complete => t.color.ok,
            DialogTransactionTone::Error => t.color.err,
        };
        let response = Frame::NONE
            .fill(t.color.bg_panel)
            .inner_margin(Margin::symmetric(12, 6))
            .show(ui, |ui| {
                let width = ui.available_width();
                ui.allocate_ui_with_layout(
                    vec2(width, 25.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        let (indicator, _) =
                            ui.allocate_exact_size(vec2(14.0, 14.0), Sense::hover());
                        ui.painter().circle_stroke(
                            indicator.center(),
                            4.0,
                            Stroke::new(1.0, color),
                        );
                        if transaction.tone == DialogTransactionTone::Complete {
                            ui.painter().circle_filled(indicator.center(), 2.0, color);
                        }
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 1.0;
                            ui.label(
                                egui::RichText::new(transaction.title)
                                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                    .color(t.color.text),
                            );
                            ui.label(
                                egui::RichText::new(transaction.detail)
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_faint),
                            );
                        });
                    },
                );
            });
        ui.painter().hline(
            response.response.rect.x_range(),
            response.response.rect.top(),
            Stroke::new(1.0, t.color.border_strong),
        );
        ui.ctx()
            .accesskit_node_builder(response.response.id, |node| {
                node.set_role(if transaction.tone == DialogTransactionTone::Error {
                    egui::accesskit::Role::Alert
                } else {
                    egui::accesskit::Role::Status
                });
                node.set_label(transaction.title);
                node.set_description(transaction.detail);
            });
    }

    fn footer(
        &self,
        ui: &mut Ui,
        t: &Tokens,
        hide_close_only_footer: bool,
        large_targets: bool,
        surface_width: f32,
    ) -> DialogFooterOutput {
        if self.note_only_footer {
            // The catalog body contains a two-column horizontal layout. Its
            // retained minimum width can make `available_width()` report one
            // column here, so a content-sized Frame only paints half of the
            // note strip. Allocate and paint against the resolved dialog
            // surface instead of inheriting body content geometry.
            let footer_rect =
                dialog_note_footer_rect(ui.max_rect().left(), ui.cursor().top(), surface_width);
            ui.painter().rect_filled(footer_rect, 0.0, t.color.bg_panel);
            ui.painter().hline(
                footer_rect.x_range(),
                footer_rect.top(),
                Stroke::new(1.0, t.color.border),
            );
            let content_rect = Rect::from_min_max(
                footer_rect.min + vec2(12.0, 0.0),
                footer_rect.max - vec2(12.0, 0.0),
            );
            let mut footer_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(content_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            if let Some(hint) = self.hint {
                footer_ui.add(
                    egui::Label::new(
                        egui::RichText::new(hint)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    )
                    .wrap(),
                );
            }
            ui.allocate_rect(footer_rect, Sense::hover());
            return DialogFooterOutput::default();
        }
        if self.hides_close_only_footer(hide_close_only_footer) {
            return DialogFooterOutput::default();
        }
        let c = t.color;
        let mut choice = DialogChoice::None;
        let mut primary_id = None;
        let mut secondary_id = None;
        let mut ghost_id = None;
        let line_y = ui.cursor().top();
        ui.painter()
            .hline(ui.max_rect().x_range(), line_y, Stroke::new(1.0, c.border));
        let horizontal_margin = WORKFLOW_FOOTER_HORIZONTAL_MARGIN;
        let vertical_margin = if large_targets {
            2
        } else {
            WORKFLOW_FOOTER_VERTICAL_MARGIN
        };
        Frame::NONE
            .fill(c.bg_panel)
            .inner_margin(Margin::symmetric(horizontal_margin, vertical_margin))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;
                    if let Some(hint) = self.hint {
                        ui.label(
                            egui::RichText::new(hint)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(c.text_faint),
                        );
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let primary = crate::ui::widgets::Button::new(self.primary)
                            .accent()
                            .destructive(self.destructive)
                            .enabled(self.primary_enabled)
                            .show(ui);
                        primary_id = self.primary_enabled.then_some(primary.id);
                        if primary.clicked() {
                            choice = DialogChoice::Primary;
                        }
                        if let Some(label) = self.secondary {
                            let secondary = crate::ui::widgets::Button::new(label).show(ui);
                            secondary_id = Some(secondary.id);
                            if secondary.clicked() {
                                choice = DialogChoice::Secondary;
                            }
                        }
                        if let Some(label) = self.ghost {
                            let ghost = crate::ui::widgets::Button::new(label)
                                .ghost()
                                .enabled(self.ghost_enabled)
                                .show(ui);
                            ghost_id = self.ghost_enabled.then_some(ghost.id);
                            if ghost.clicked() {
                                choice = DialogChoice::Ghost;
                            }
                        }
                    });
                });
            });
        DialogFooterOutput {
            choice,
            primary_id,
            secondary_id,
            ghost_id,
        }
    }

    fn hides_close_only_footer(&self, hide_close_only_footer: bool) -> bool {
        hide_close_only_footer
            && self.primary == "Close"
            && self.secondary.is_none()
            && self.ghost.is_none()
            && self.hint.is_none()
    }
}

fn dialog_note_footer_rect(left: f32, top: f32, surface_width: f32) -> Rect {
    Rect::from_min_size(egui::pos2(left, top), vec2(surface_width.max(1.0), 48.0))
}

fn begin_dialog_focus(ctx: &Context, state_id: Id) -> bool {
    let pass = ctx.cumulative_pass_nr();
    let current_focus = ctx.memory(|memory| memory.focused());
    ctx.data_mut(|data| {
        let previous = data.get_temp::<DialogFocusState>(state_id);
        let continuing =
            previous.is_some_and(|state| pass <= state.last_seen_pass.saturating_add(1));
        let state = match previous {
            Some(mut state) if continuing => {
                state.last_seen_pass = pass;
                state
            }
            _ => DialogFocusState {
                prior_focus: current_focus,
                last_seen_pass: pass,
            },
        };
        data.insert_temp(state_id, state);
        !continuing
    })
}

fn focused_control_owns_enter(ctx: &Context, dialog_focus_id: Id) -> bool {
    let Some(focused) = ctx.memory(|memory| memory.focused()) else {
        return false;
    };
    if focused == dialog_focus_id {
        return false;
    }

    // Buttons and button-like controls already synthesize their own click for
    // Enter. Drag-capable fields include egui text edits; a single-line edit
    // deliberately falls through to the dialog's default submit behavior.
    ctx.read_response(focused)
        .is_some_and(|response| response.sense.senses_click() && !response.sense.senses_drag())
}

fn focus_is_within_modal(ctx: &Context, dialog_layer: egui::LayerId) -> bool {
    let Some(focused) = ctx.memory(|memory| memory.focused()) else {
        return false;
    };
    let Some(response) = ctx.read_response(focused) else {
        return false;
    };
    response.layer_id == dialog_layer
        || ctx.memory(|memory| memory.is_above_modal_layer(response.layer_id))
}

fn restore_dialog_focus(
    ctx: &Context,
    state_id: Id,
    dialog_focus_id: Id,
    dialog_layer: egui::LayerId,
) {
    let state = ctx.data_mut(|data| data.remove_temp::<DialogFocusState>(state_id));
    let prior_focus = state.and_then(|state| state.prior_focus);
    let restorable = prior_focus.filter(|prior| {
        *prior != dialog_focus_id
            && ctx.read_response(*prior).is_some_and(|response| {
                response.layer_id != dialog_layer
                    && response.enabled()
                    && response.sense.is_focusable()
            })
    });

    ctx.memory_mut(|memory| {
        // Never leave focus attached to a dialog control that is about to
        // disappear. Invalid or disabled prior targets safely yield no focus.
        if let Some(current) = memory.focused() {
            memory.surrender_focus(current);
        }
        if let Some(prior) = restorable {
            memory.request_focus(prior);
        }
    });
}

/// Center a header strip text baseline helper used by dialog tabs (mono
/// uppercase underline tabs, as in the results docbar).
pub fn dialog_tabs(ui: &mut Ui, tabs: &[&str], active: &mut usize) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        for (index, label) in tabs.iter().enumerate() {
            let selected = *active == index;
            let mut job = egui::text::LayoutJob::default();
            job.append(
                &label.to_uppercase(),
                0.0,
                egui::TextFormat {
                    font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                    color: egui::Color32::PLACEHOLDER,
                    extra_letter_spacing: 0.06 * tokens::FS_0,
                    ..Default::default()
                },
            );
            let galley = ui.fonts_mut(|f| f.layout_job(job));
            let (rect, response) =
                ui.allocate_exact_size(vec2(galley.size().x + 22.0, 24.0), Sense::click());
            response.widget_info(|| {
                WidgetInfo::labeled(WidgetType::SelectableLabel, ui.is_enabled(), label)
            });
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::Tab);
                if selected {
                    node.set_selected(true);
                } else {
                    node.set_selected(false);
                }
            });
            let hover = ui.ctx().animate_bool_with_time(
                response.id,
                response.hovered() && !selected,
                ui.style().animation_time,
            );
            let color = if selected {
                c.accent
            } else {
                crate::ui::theme::mix(c.text_dim, c.text, hover)
            };
            ui.painter().galley(
                egui::pos2(rect.left() + 11.0, rect.center().y - galley.size().y * 0.5),
                galley,
                color,
            );
            if selected {
                ui.painter().hline(
                    egui::Rangef::new(rect.left() + 6.0, rect.right() - 6.0),
                    rect.bottom() - 1.0,
                    Stroke::new(2.0, c.accent),
                );
            }
            theme::paint_focus_ring(ui, &response, rect);
            if response.clicked() {
                *active = index;
            }
        }
    });
    let y = ui.cursor().top();
    ui.painter()
        .hline(ui.max_rect().x_range(), y, Stroke::new(1.0, t.color.border));
    ui.add_space(10.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TITLE: &str = "Modal behavior";
    const TEST_DESCRIPTION: &str = "Review this modal operation and its available actions.";

    fn dialog_id() -> Id {
        Id::new(("rspice.dialog", TEST_TITLE))
    }

    fn dialog_focus_id() -> Id {
        dialog_id().with("move")
    }

    fn underlying_id() -> Id {
        Id::new("dialog-test-underlying-editor")
    }

    fn raw_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_000.0, 800.0))),
            events,
            ..Default::default()
        }
    }

    fn key_event(key: Key, modifiers: Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers,
        }
    }

    fn focus_underlying_editor(ctx: &Context, underlying: &mut String) {
        let _output = ctx.run(raw_input(Vec::new()), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(
                    egui::TextEdit::singleline(underlying)
                        .id(underlying_id())
                        .desired_width(240.0),
                )
                .request_focus();
            });
        });
    }

    fn run_dialog(
        ctx: &Context,
        input: egui::RawInput,
        underlying: &mut String,
        primary_on_enter: bool,
        mut body: impl FnMut(&mut Ui),
    ) -> (DialogChoice, egui::FullOutput) {
        let mut choice = DialogChoice::None;
        let output = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = ui.add(
                    egui::TextEdit::singleline(underlying)
                        .id(underlying_id())
                        .desired_width(240.0),
                );
            });
            choice = Dialog::new("TEST", TEST_TITLE, "Accept")
                .description(TEST_DESCRIPTION)
                .primary_on_enter(primary_on_enter)
                .show(ctx, |ui| body(ui));
        });
        (choice, output)
    }

    fn assert_focus_is_on_dialog_layer(ctx: &Context) {
        let focused = ctx
            .memory(|memory| memory.focused())
            .expect("dialog should own keyboard focus");
        assert_ne!(focused, underlying_id());
        let response = ctx
            .read_response(focused)
            .expect("focused dialog widget should still exist");
        assert_eq!(
            response.layer_id,
            egui::LayerId::new(Order::Foreground, dialog_id())
        );
    }

    #[test]
    fn capability_review_phone_layout_is_full_viewport() {
        let screen = Rect::from_min_size(egui::pos2(7.0, 11.0), vec2(390.0, 844.0));
        let layout = DialogLayout::resolve(DialogSize::CapabilityReview, screen, None);

        assert_eq!(layout.surface_rect, screen);
        assert_eq!(layout.radius, 0.0);
        assert!(layout.app_background);
    }

    #[test]
    fn capability_review_desktop_layout_uses_mockup_caps_and_is_centered() {
        let screen = Rect::from_min_size(egui::pos2(20.0, 30.0), vec2(1_440.0, 900.0));
        let layout = DialogLayout::resolve(DialogSize::CapabilityReview, screen, None);

        assert_eq!(layout.surface_rect.width(), 1_040.0);
        assert_eq!(layout.surface_rect.height(), 760.0);
        assert_eq!(layout.surface_rect.center(), screen.center());
        assert_eq!(layout.surface_rect.left(), 220.0);
        assert_eq!(layout.surface_rect.top(), 100.0);
    }

    #[test]
    fn transaction_short_narrow_viewport_is_full_viewport() {
        let screen = Rect::from_min_size(egui::pos2(3.0, 5.0), vec2(390.0, 300.0));
        let layout = DialogLayout::resolve(DialogSize::Transaction, screen, None);

        assert_eq!(layout.surface_rect, screen);
    }

    #[test]
    fn transaction_tablet_retains_mockup_gutters_until_the_phone_breakpoint() {
        let screen = Rect::from_min_size(egui::pos2(10.0, 20.0), vec2(800.0, 900.0));
        let layout = DialogLayout::resolve(DialogSize::Transaction, screen, Some(500.0));

        assert_eq!(layout.surface_rect.size(), vec2(760.0, 500.0));
        assert_eq!(layout.surface_rect.center(), screen.center());
        assert_eq!(layout.surface_rect.left(), 30.0);
        assert_eq!(layout.radius, 4.0);
        assert!(!layout.narrow);

        let phone_breakpoint = Rect::from_min_size(egui::pos2(10.0, 20.0), vec2(560.0, 900.0));
        let phone = DialogLayout::resolve(DialogSize::Transaction, phone_breakpoint, Some(500.0));
        assert_eq!(phone.surface_rect, phone_breakpoint);
        assert_eq!(phone.radius, 0.0);
        assert!(phone.narrow);
    }

    #[test]
    fn manager_phone_layout_uses_the_mockup_four_point_gutter() {
        let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(390.0, 844.0));
        let layout = DialogLayout::resolve(DialogSize::Manager, screen, None);

        assert_eq!(layout.surface_rect.left(), 4.0);
        assert_eq!(layout.surface_rect.width(), 382.0);
        assert_eq!(layout.surface_rect.top(), 4.0);
        assert_eq!(layout.surface_rect.height(), 836.0);
        assert!(layout.fill_height);
        assert!(!layout.app_background);
    }

    #[test]
    fn manager_and_wide_workflow_desktop_geometry_match_the_mockup() {
        let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_440.0, 900.0));
        let manager = DialogLayout::resolve(DialogSize::Manager, screen, None);
        let workflow = DialogLayout::resolve(DialogSize::WideWorkflow, screen, Some(612.0));

        assert_eq!(manager.surface_rect.size(), vec2(760.0, 530.0));
        assert_eq!(manager.surface_rect.center(), screen.center());
        assert_eq!(manager.radius, 8.0);
        assert_eq!(workflow.surface_rect.size(), vec2(980.0, 612.0));
        assert_eq!(workflow.surface_rect.center(), screen.center());
        assert_eq!(workflow.radius, 4.0);
        assert!(!workflow.fill_height);
    }

    #[test]
    fn analysis_catalog_uses_mockup_top_anchor_caps_and_phone_gutter() {
        let desktop_screen = Rect::from_min_size(egui::pos2(20.0, 30.0), vec2(1_440.0, 900.0));
        let desktop = DialogLayout::resolve(DialogSize::AnalysisCatalog, desktop_screen, None);
        assert_eq!(desktop.surface_rect.size(), vec2(1_180.0, 780.0));
        assert_eq!(desktop.surface_rect.left(), 150.0);
        assert_eq!(desktop.surface_rect.top(), 42.0);
        assert_eq!(desktop.radius, 8.0);
        assert!(desktop.fill_height);
        assert!(!desktop.app_background);

        let phone_screen = Rect::from_min_size(egui::pos2(7.0, 11.0), vec2(390.0, 844.0));
        let phone = DialogLayout::resolve(DialogSize::AnalysisCatalog, phone_screen, None);
        assert_eq!(phone.surface_rect.min, egui::pos2(11.0, 23.0));
        assert_eq!(phone.surface_rect.size(), vec2(382.0, 780.0));
        assert_eq!(phone.radius, 8.0);
    }

    #[test]
    fn wide_workflow_becomes_edge_to_edge_at_the_mockup_breakpoint() {
        let screen = Rect::from_min_size(egui::pos2(5.0, 7.0), vec2(820.0, 900.0));
        let layout = DialogLayout::resolve(DialogSize::WideWorkflow, screen, Some(500.0));

        assert_eq!(layout.surface_rect, screen);
        assert_eq!(layout.radius, 0.0);
    }

    #[test]
    fn flush_body_is_opt_in() {
        let padded = Dialog::new("Test", TEST_TITLE, "Accept");
        assert!(!padded.flush_body);
        let flush = Dialog::new("Test", TEST_TITLE, "Accept").flush_body();
        assert!(flush.flush_body);
    }

    #[test]
    fn catalog_body_and_note_footer_are_explicit_opt_ins() {
        let dialog = Dialog::new("Test", TEST_TITLE, "Close")
            .manual_body_scroll()
            .note_only_footer()
            .hint("Catalog classification note");
        assert!(dialog.manual_body_scroll);
        assert!(dialog.note_only_footer);
        assert_eq!(dialog.footer_height(false, false), 48.0);
    }

    #[test]
    fn catalog_note_footer_uses_the_complete_1280_viewport_surface_width() {
        let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_280.0, 720.0));
        let layout = DialogLayout::resolve(DialogSize::AnalysisCatalog, screen, None);
        let footer = dialog_note_footer_rect(
            layout.surface_rect.left(),
            layout.surface_rect.bottom() - 48.0,
            layout.surface_rect.width(),
        );

        assert_eq!(
            layout.surface_rect,
            Rect::from_min_max(egui::pos2(50.0, 12.0), egui::pos2(1_230.0, 708.0),)
        );
        assert_eq!(footer.left(), layout.surface_rect.left());
        assert_eq!(footer.right(), layout.surface_rect.right());
        assert_eq!(footer.height(), 48.0);
    }

    #[test]
    fn transaction_state_is_absent_when_idle_and_exposes_an_assertive_strip_on_error() {
        let idle = Dialog::new("Test", TEST_TITLE, "Accept");
        assert!(idle.transaction_state.is_none());

        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run(raw_input(Vec::new()), |ctx| {
            let _ = Dialog::new("Test", TEST_TITLE, "Accept")
                .transaction_state(
                    DialogTransactionTone::Error,
                    "Unsaved dialog changes",
                    "Choose Discard changes again to close.",
                )
                .show(ctx, |ui| {
                    ui.label("Dialog body");
                });
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Alert
                && node.label() == Some("Unsaved dialog changes")
                && node.description() == Some("Choose Discard changes again to close.")
        }));
    }

    #[test]
    fn close_only_workflow_hides_redundant_footer() {
        let dialog = Dialog::new("Test", TEST_TITLE, "Close").size(DialogSize::CapabilityReview);

        assert_eq!(dialog.footer_height(true, false), 0.0);
        assert!(dialog.hides_close_only_footer(true));
    }

    #[test]
    fn capability_review_phone_footer_stays_horizontal_and_compact() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(390.0, 844.0))),
            ..Default::default()
        };

        let output = ctx.run(input, |ctx| {
            let _ = Dialog::new(
                "A deliberately long governed workflow",
                TEST_TITLE,
                "Accept",
            )
            .size(DialogSize::CapabilityReview)
            .ghost("Cancel")
            .secondary("Later")
            .show(ctx, |ui| {
                ui.label("Dialog body");
            });
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;

        let bounds = ["Cancel", "Later", "Accept"].map(|label| {
            nodes
                .iter()
                .find(|(_, node)| {
                    node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
                })
                .and_then(|(_, node)| node.bounds())
                .unwrap_or_else(|| panic!("missing bounds for {label}"))
        });
        for bounds in &bounds {
            assert_eq!(bounds.y1 - bounds.y0, TOUCH_TARGET_SIDE as f64);
        }
        assert_eq!(bounds[0].y0, bounds[1].y0);
        assert_eq!(bounds[1].y0, bounds[2].y0);
        assert!(bounds[0].x0 < bounds[1].x0);
        assert!(bounds[1].x0 < bounds[2].x0);
        assert!(bounds[2].x1 >= 377.0, "actions must be trailing-grouped");

        let close_bounds = nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some("Close (Esc)")
            })
            .and_then(|(_, node)| node.bounds())
            .expect("missing workflow close control bounds");
        assert_eq!(close_bounds.x1 - close_bounds.x0, TOUCH_TARGET_SIDE as f64);
        assert_eq!(close_bounds.y1 - close_bounds.y0, TOUCH_TARGET_SIDE as f64);
    }

    #[test]
    fn capability_review_desktop_close_control_uses_mockup_icon_button_target() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_440.0, 900.0))),
            ..Default::default()
        };

        let output = ctx.run(input, |ctx| {
            let _ = Dialog::new("TEST", TEST_TITLE, "Accept")
                .size(DialogSize::CapabilityReview)
                .show(ctx, |ui| {
                    ui.label("Dialog body");
                });
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        let bounds = nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some("Close (Esc)")
            })
            .and_then(|(_, node)| node.bounds())
            .expect("missing workflow close control bounds");

        assert_eq!(bounds.x1 - bounds.x0, DIALOG_CLOSE_TARGET_WIDTH as f64);
        assert_eq!(bounds.y1 - bounds.y0, DIALOG_CLOSE_TARGET_HEIGHT as f64);
    }

    #[test]
    fn dialog_publishes_modal_accessibility_semantics() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut underlying = String::new();

        let (_, output) = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
            let _ = ui.label("Dialog body");
        });

        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Dialog
                && node.label() == Some(TEST_TITLE)
                && node.description() == Some(TEST_DESCRIPTION)
                && node.is_modal()
        }));
    }

    #[test]
    fn requested_body_control_receives_initial_focus_once() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut preferred_id = None;

        let _output = ctx.run(raw_input(Vec::new()), |ctx| {
            let _ = Dialog::new("TEST", TEST_TITLE, "Accept")
                .initial_focus(DialogInitialFocus::BodyControl)
                .show_with_initial_body_focus(ctx, |ui| {
                    let response = ui.button("Preferred body control");
                    preferred_id = Some(response.id);
                    Some(response.id)
                });
        });

        assert_eq!(ctx.memory(|memory| memory.focused()), preferred_id);
    }

    #[test]
    fn unavailable_initial_focus_target_falls_back_to_modal_container() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);

        let _output = ctx.run(raw_input(Vec::new()), |ctx| {
            let _ = Dialog::new("TEST", TEST_TITLE, "Accept")
                .primary_enabled(false)
                .initial_focus(DialogInitialFocus::Primary)
                .show(ctx, |ui| {
                    ui.label("Dialog body");
                });
        });

        assert_eq!(
            ctx.memory(|memory| memory.focused()),
            Some(dialog_focus_id())
        );
    }

    #[test]
    fn modal_focus_is_trapped_and_restored_without_leaking_text() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut underlying = "baseline".to_owned();
        focus_underlying_editor(&ctx, &mut underlying);
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id()));

        let _ = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
            let _ = ui.button("Body action");
        });
        assert_eq!(
            ctx.memory(|memory| memory.focused()),
            Some(dialog_focus_id())
        );

        let _ = run_dialog(
            &ctx,
            raw_input(vec![egui::Event::Text("blocked".to_owned())]),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.button("Body action");
            },
        );
        assert_eq!(underlying, "baseline");

        let _ = run_dialog(
            &ctx,
            raw_input(vec![key_event(Key::Tab, Modifiers::NONE)]),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.button("Body action");
            },
        );
        assert_focus_is_on_dialog_layer(&ctx);

        let _ = run_dialog(
            &ctx,
            raw_input(vec![key_event(
                Key::Tab,
                Modifiers {
                    shift: true,
                    ..Modifiers::NONE
                },
            )]),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.button("Body action");
            },
        );
        assert_focus_is_on_dialog_layer(&ctx);

        let (choice, _) = run_dialog(
            &ctx,
            raw_input(vec![key_event(Key::Escape, Modifiers::NONE)]),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.button("Body action");
            },
        );
        assert_eq!(choice, DialogChoice::Cancelled);
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id()));
    }

    #[test]
    fn focused_body_button_owns_enter_instead_of_primary() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut underlying = String::new();
        let mut activated = false;

        let _ = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
            ui.button("Body action").request_focus();
        });
        let (choice, _) = run_dialog(
            &ctx,
            raw_input(vec![key_event(Key::Enter, Modifiers::NONE)]),
            &mut underlying,
            true,
            |ui| {
                let response = ui.button("Body action");
                if response.clicked() {
                    activated = true;
                }
                response.request_focus();
            },
        );

        assert!(activated);
        assert_eq!(choice, DialogChoice::None);
    }

    #[test]
    fn body_close_request_cancels_and_restores_prior_focus() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut underlying = String::new();
        focus_underlying_editor(&ctx, &mut underlying);

        let (choice, _) = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
            ui.close_kind(UiKind::Modal)
        });

        assert_eq!(choice, DialogChoice::Cancelled);
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id()));
    }

    #[test]
    fn enter_submits_by_default_but_multiline_opt_out_keeps_newlines() {
        let submit_ctx = Context::default();
        crate::ui::Theme::default().apply(&submit_ctx);
        let mut underlying = String::new();
        let _ = run_dialog(
            &submit_ctx,
            raw_input(Vec::new()),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.label("Ready");
            },
        );
        let (choice, _) = run_dialog(
            &submit_ctx,
            raw_input(vec![key_event(Key::Enter, Modifiers::NONE)]),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.label("Ready");
            },
        );
        assert_eq!(choice, DialogChoice::Primary);
        assert_eq!(submit_ctx.memory(|memory| memory.focused()), None);

        let multiline_ctx = Context::default();
        crate::ui::Theme::default().apply(&multiline_ctx);
        let mut underlying = String::new();
        let mut body_text = String::new();
        let _ = run_dialog(
            &multiline_ctx,
            raw_input(Vec::new()),
            &mut underlying,
            false,
            |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut body_text).id(Id::new("dialog-test-multiline")),
                )
                .request_focus();
            },
        );
        let (choice, _) = run_dialog(
            &multiline_ctx,
            raw_input(vec![key_event(Key::Enter, Modifiers::NONE)]),
            &mut underlying,
            false,
            |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut body_text).id(Id::new("dialog-test-multiline")),
                )
                .request_focus();
            },
        );

        assert_eq!(choice, DialogChoice::None);
        assert_eq!(body_text, "\n");
    }

    fn collect_rust_sources(directory: &std::path::Path, sources: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(directory).expect("read source directory") {
            let path = entry.expect("read source entry").path();
            if path.is_dir() {
                collect_rust_sources(&path, sources);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }

    #[test]
    fn every_production_dialog_callsite_supplies_a_description() {
        let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut sources = Vec::new();
        collect_rust_sources(&source_root, &mut sources);
        sources.sort();

        let mut audited = 0;
        let mut missing = Vec::new();
        for path in sources {
            let relative = path
                .strip_prefix(&source_root)
                .expect("source beneath crate root")
                .to_string_lossy()
                .replace('\\', "/");
            // The primitive contains deliberately partial construction tests;
            // feature-availability descriptions have their own route-specific
            // source contract because their purpose text depends on the route.
            if matches!(
                relative.as_str(),
                "ui/widgets/dialog.rs" | "workbench/feature_availability.rs"
            ) {
                continue;
            }

            let source = std::fs::read_to_string(&path).expect("read Rust source");
            let production_source = source.split("\n#[cfg(test)]").next().unwrap_or(&source);
            for (offset, _) in production_source.match_indices("Dialog::new(") {
                if offset > 0
                    && (production_source.as_bytes()[offset - 1].is_ascii_alphanumeric()
                        || production_source.as_bytes()[offset - 1] == b'_')
                {
                    // Exclude other types whose names end in `Dialog`, such
                    // as native `FileDialog` and persisted dialog-state data.
                    continue;
                }
                audited += 1;
                let tail = &production_source[offset..];
                let chain_end = [
                    tail.find(".show("),
                    tail.find(".show_with_initial_body_focus("),
                    tail.find(';'),
                ]
                .into_iter()
                .flatten()
                .min()
                .unwrap_or(tail.len());
                if !tail[..chain_end].contains(".description(") {
                    let line = production_source[..offset]
                        .bytes()
                        .filter(|byte| *byte == b'\n')
                        .count()
                        + 1;
                    missing.push(format!("{relative}:{line}"));
                }
            }
        }

        assert!(audited > 0, "source audit did not find a production dialog");
        assert!(
            missing.is_empty(),
            "production dialogs must publish explicit purpose text:\n{}",
            missing.join("\n")
        );
    }
}
