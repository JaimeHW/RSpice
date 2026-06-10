//! The VOLTA modal primitive.
//!
//! Every dialog in the application is the same three-part surface over a
//! scrim (see `design/volta-dialogs.html` §01):
//!
//! - **Scrim** — full-viewport canvas-black wash that blocks interaction
//!   with everything underneath. Clicking it does not dismiss; `Esc` does.
//! - **Surface** — `bg_panel`, 1 px `border_strong`, large radius, pop
//!   shadow. Three widths (Sm 400 / Md 560 / Lg 780); the body scrolls
//!   once the surface reaches 82 % of the viewport height.
//! - **Header / footer** — mono uppercase kicker + semibold title + close
//!   on top; footer is `[ghost] [secondary] [primary]` with the primary
//!   always rightmost, always exactly one, accent-filled (or `err` when
//!   destructive), plus an optional mono hint on the left.
//!
//! Keys: `Esc` cancels, `Enter` activates the primary when it is enabled.
//! Dialogs edit a draft and commit on the primary — the primary and
//! cancel are never the same operation.

use egui::{Context, Frame, Id, Key, Margin, Order, Rect, Sense, Stroke, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Dialog surface width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogSize {
    /// Confirmations, small forms (400 pt).
    Sm,
    /// Standard forms (560 pt).
    Md,
    /// Multi-pane surfaces — browsers, options (780 pt).
    Lg,
}

impl DialogSize {
    fn width(self) -> f32 {
        match self {
            Self::Sm => 400.0,
            Self::Md => 560.0,
            Self::Lg => 780.0,
        }
    }
}

/// What the user chose this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogChoice {
    /// Still open, nothing chosen.
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

/// Declarative description of one modal frame.
pub struct Dialog<'a> {
    kicker: &'a str,
    title: &'a str,
    size: DialogSize,
    primary: &'a str,
    primary_enabled: bool,
    destructive: bool,
    secondary: Option<&'a str>,
    ghost: Option<&'a str>,
    hint: Option<&'a str>,
}

impl<'a> Dialog<'a> {
    /// A dialog with the given header kicker (domain, e.g. "Simulate"),
    /// title, and primary-action label.
    pub fn new(kicker: &'a str, title: &'a str, primary: &'a str) -> Self {
        Self {
            kicker,
            title,
            size: DialogSize::Md,
            primary,
            primary_enabled: true,
            destructive: false,
            secondary: None,
            ghost: None,
            hint: None,
        }
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

    /// Mono footer hint (validation count, shortcut reminder).
    pub fn hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Show the dialog and render `body` into the scrollable middle
    /// region. Returns what the user chose this frame; the caller owns
    /// open/close state and reacts to the choice.
    pub fn show(self, ctx: &Context, body: impl FnOnce(&mut Ui)) -> DialogChoice {
        let t = Tokens::get(ctx);
        let c = t.color;
        let screen = ctx.screen_rect();
        let id = Id::new(("volta.dialog", self.title));

        let mut choice = DialogChoice::None;

        // Keys first so they work regardless of focus inside the body.
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            choice = DialogChoice::Cancelled;
        } else if self.primary_enabled && ctx.input(|i| i.key_pressed(Key::Enter)) {
            choice = DialogChoice::Primary;
        }

        egui::Area::new(id)
            .order(Order::Foreground)
            .fixed_pos(screen.min)
            .show(ctx, |ui| {
                // Scrim: swallow pointer interaction with everything below.
                ui.allocate_rect(screen, Sense::click_and_drag());
                ui.painter()
                    .rect_filled(screen, 0.0, c.canvas_bg.gamma_multiply(0.55));

                let width = self.size.width().min(screen.width() - 32.0);
                let max_height = screen.height() * 0.82;
                // Dialogs sit at a fixed optical position — top edge at 18 %
                // of the viewport — so confirmations and browsers alike open
                // in the same place.
                let top_left = egui::pos2(
                    screen.center().x - width * 0.5,
                    screen.top() + screen.height() * 0.18,
                );

                let mut surface = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(Rect::from_min_size(top_left, vec2(width, max_height)))
                        .layout(egui::Layout::top_down(egui::Align::Min)),
                );
                surface.set_width(width);

                Frame::none()
                    .fill(c.bg_panel)
                    .stroke(Stroke::new(1.0, c.border_strong))
                    .rounding(t.radius_lg)
                    .shadow(t.shadow())
                    .show(&mut surface, |ui| {
                        ui.set_width(width);
                        if self.header(ui, &t) {
                            choice = DialogChoice::Cancelled;
                        }
                        egui::ScrollArea::vertical()
                            .id_salt(id.with("body"))
                            .max_height(max_height - 2.0 * 46.0)
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                Frame::none()
                                    .inner_margin(Margin::symmetric(16.0, 14.0))
                                    .show(ui, body);
                            });
                        match self.footer(ui, &t) {
                            DialogChoice::None => {}
                            chosen => choice = chosen,
                        }
                    });
            });

        choice
    }

    /// Header strip; returns `true` when the close control fired.
    fn header(&self, ui: &mut Ui, t: &Tokens) -> bool {
        let c = t.color;
        let mut closed = false;
        Frame::none()
            .inner_margin(Margin::symmetric(16.0, 11.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    let mut kicker = egui::text::LayoutJob::default();
                    kicker.append(
                        &self.kicker.to_uppercase(),
                        0.0,
                        egui::TextFormat {
                            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                            color: if self.destructive { c.err } else { c.accent },
                            extra_letter_spacing: 0.14 * tokens::FS_0,
                            ..Default::default()
                        },
                    );
                    ui.label(kicker);
                    ui.label(
                        egui::RichText::new(self.title)
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(c.text),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if crate::ui::widgets::IconButton::new(crate::ui::icons::Icon::Close)
                            .side(24.0)
                            .tooltip("Close (Esc)")
                            .show(ui)
                            .clicked()
                        {
                            closed = true;
                        }
                    });
                });
            });
        let line_y = ui.cursor().top();
        ui.painter().hline(
            ui.max_rect().x_range(),
            line_y,
            Stroke::new(1.0, t.color.border),
        );
        closed
    }

    /// Footer strip with the canonical button order.
    fn footer(&self, ui: &mut Ui, t: &Tokens) -> DialogChoice {
        let c = t.color;
        let mut choice = DialogChoice::None;
        let line_y = ui.cursor().top();
        ui.painter()
            .hline(ui.max_rect().x_range(), line_y, Stroke::new(1.0, c.border));
        Frame::none()
            .inner_margin(Margin::symmetric(16.0, 11.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    if let Some(label) = self.ghost {
                        if crate::ui::widgets::Button::new(label).ghost().show(ui).clicked() {
                            choice = DialogChoice::Ghost;
                        }
                    }
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
                        if primary.clicked() {
                            choice = DialogChoice::Primary;
                        }
                        if let Some(label) = self.secondary {
                            if crate::ui::widgets::Button::new(label).show(ui).clicked() {
                                choice = DialogChoice::Secondary;
                            }
                        }
                    });
                });
            });
        choice
    }
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
            let galley = ui.fonts(|f| f.layout_job(job));
            let (rect, response) = ui.allocate_exact_size(
                vec2(galley.size().x + 22.0, 24.0),
                Sense::click(),
            );
            let hover = ui
                .ctx()
                .animate_bool_with_time(response.id, response.hovered() && !selected, 0.16);
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
