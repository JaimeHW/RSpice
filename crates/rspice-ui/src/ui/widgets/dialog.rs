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
    /// Mockup-authored workflow transactions: 760 pt wide and
    /// content-height, with the full-viewport workflow shell applied at the
    /// shared 820 pt dialog breakpoint; individual fields stack by local width.
    SimulationWorkflow,
    /// Mockup-owned specialist workspace discovery manager: 760 pt wide on
    /// desktop and full-viewport at the 820 pt one-column breakpoint.
    SpecialistToolBrowser,
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
    /// Drawing-sheet supporting workflows: 1160 pt wide, content-height
    /// capped at 760 pt, and edge-to-edge at the shared 820 pt breakpoint.
    DrawingSheetWorkflow,
    /// Account, organization, and licensing manager: the mockup's 920 pt
    /// desktop surface, content-height capped at 820 pt so the final license
    /// action clears the fixed footer, and edge-to-edge at the shared 820 pt
    /// manager breakpoint.
    AccountManager,
    /// Governed capability matrices: 1040 pt wide, content-height capped at
    /// 760 pt, edge-to-edge at the mockup's 820 pt breakpoint.
    CapabilityReview,
    /// Execution queue, target, and retained-run manager: 1120 × 680 pt with
    /// the mockup's 12 pt desktop perimeter and 4 pt phone perimeter.
    JobsManager,
    /// Permanent authored drawing-sheet studio: 1280 x 760 pt with a fixed
    /// section navigator, editable sheet form, and live effect preview.
    SchematicPageSetup,
    /// Schematic instance editor: the mockup's fixed 880 x 680 pt two-pane
    /// surface with 24/32 pt desktop viewport gutters.
    ComponentEditor,
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
            Self::SpecialistToolBrowser => DialogSurfaceSpec {
                width: 760.0,
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
            Self::AccountManager => DialogSurfaceSpec {
                width: 920.0,
                max_height: 820.0,
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
            Self::JobsManager => DialogSurfaceSpec {
                width: 1_120.0,
                max_height: 680.0,
                horizontal_inset: 24.0,
                vertical_inset: 24.0,
                narrow_max_width: 560.0,
                narrow_inset: 8.0,
                narrow_vertical_inset: 8.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: false,
                fill_narrow_viewport: true,
                fill_height: true,
                app_background: true,
                radius: 4.0,
                top_anchored: false,
            },
            Self::DrawingSheetWorkflow => DialogSurfaceSpec {
                width: 1_160.0,
                max_height: 760.0,
                horizontal_inset: 12.0,
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
            Self::SchematicPageSetup => DialogSurfaceSpec {
                width: 1_280.0,
                max_height: 760.0,
                horizontal_inset: 12.0,
                vertical_inset: 12.0,
                narrow_max_width: 820.0,
                narrow_inset: 0.0,
                narrow_vertical_inset: 0.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: true,
                fill_narrow_viewport: true,
                fill_height: true,
                app_background: true,
                radius: 4.0,
                top_anchored: false,
            },
            Self::ComponentEditor => DialogSurfaceSpec {
                width: 880.0,
                max_height: 680.0,
                horizontal_inset: 48.0,
                vertical_inset: 64.0,
                narrow_max_width: 760.0,
                narrow_inset: 48.0,
                narrow_vertical_inset: 64.0,
                cap_narrow_height: true,
                edge_to_edge_narrow: false,
                fill_narrow_viewport: false,
                fill_height: true,
                app_background: false,
                radius: 8.0,
                top_anchored: false,
            },
        }
    }
}

const WORKFLOW_HEADER_MIN_HEIGHT: f32 = 57.0;
const WORKFLOW_HEADER_TEXT_MIN_HEIGHT: f32 = 33.0;
/// The single measure of lead a title-first header uses above the title,
/// between the two lines, and below the subtitle.
const TITLE_FIRST_HEADER_LEAD: f32 = 9.0;
const WORKFLOW_HEADER_HORIZONTAL_MARGIN: i8 = 15;
const WORKFLOW_FOOTER_HORIZONTAL_MARGIN: i8 = 12;
const WORKFLOW_FOOTER_VERTICAL_MARGIN: i8 = 10;
const DIALOG_CLOSE_TARGET_WIDTH: f32 = 28.0;
const DIALOG_CLOSE_TARGET_HEIGHT: f32 = 27.0;
const DIALOG_TAB_HORIZONTAL_PADDING: f32 = 18.0;
const DIALOG_TAB_MIN_HORIZONTAL_PADDING: f32 = 6.0;
#[cfg(test)]
const DIALOG_TAB_SCROLL_VIEWPORT_RESERVE: f32 = 12.0;
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
            fill_height: spec.fill_height || (narrow && spec.fill_narrow_viewport),
            app_background: spec.app_background,
            radius: if narrow && spec.edge_to_edge_narrow {
                0.0
            } else {
                spec.radius
            },
            narrow,
        }
    }

    fn fills_surface_height(self, fixed_height: Option<f32>) -> bool {
        self.fill_height || fixed_height.is_some()
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

#[derive(Debug, Clone)]
struct DialogTransactionState {
    tone: DialogTransactionTone,
    title: String,
    detail: String,
}

/// Declarative description of one modal frame.
pub struct Dialog<'a> {
    kicker: String,
    title: String,
    description: Option<String>,
    size: DialogSize,
    primary: String,
    primary_enabled: bool,
    interaction_enabled: bool,
    primary_on_enter: bool,
    destructive: bool,
    secondary: Option<String>,
    secondary_enabled: bool,
    ghost: Option<String>,
    ghost_enabled: bool,
    hint: Option<String>,
    transaction_state: Option<DialogTransactionState>,
    body_scroll_offset: Option<&'a mut f32>,
    flush_body: bool,
    manual_body_scroll: bool,
    note_only_footer: bool,
    header_visible: bool,
    initial_focus: DialogInitialFocus,
    retained_cancel_focus: Option<DialogInitialFocus>,
    initial_height: Option<f32>,
    fixed_height: Option<f32>,
}

impl<'a> Dialog<'a> {
    /// A dialog with the given header kicker (domain, e.g. "Simulate"),
    /// title, and primary-action label.
    pub fn new(
        kicker: impl Into<String>,
        title: impl Into<String>,
        primary: impl Into<String>,
    ) -> Self {
        Self {
            kicker: kicker.into(),
            title: title.into(),
            description: None,
            size: DialogSize::Transaction,
            primary: primary.into(),
            primary_enabled: true,
            interaction_enabled: true,
            primary_on_enter: true,
            destructive: false,
            secondary: None,
            secondary_enabled: true,
            ghost: None,
            ghost_enabled: true,
            hint: None,
            transaction_state: None,
            body_scroll_offset: None,
            flush_body: false,
            manual_body_scroll: false,
            note_only_footer: false,
            header_visible: true,
            initial_focus: DialogInitialFocus::Container,
            retained_cancel_focus: None,
            initial_height: None,
            fixed_height: None,
        }
    }

    /// Describe the dialog's purpose and scope to assistive technology.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Supply the mockup-authored total surface height for the first frame.
    /// Content-height dialogs still measure and retain their actual height;
    /// this removes the provisional max-height jump before that measurement.
    pub fn initial_height(mut self, height: f32) -> Self {
        self.initial_height = height.is_finite().then(|| height.max(1.0));
        self
    }

    /// Hold a workflow surface at one authored desktop height across validation,
    /// error, and progress states. Narrow edge-to-edge layouts still use the
    /// available viewport height, and overflowing body content remains scrollable.
    pub fn fixed_height(mut self, height: f32) -> Self {
        self.fixed_height = height.is_finite().then(|| height.max(1.0));
        self
    }

    /// Choose the control that receives focus when the dialog opens.
    pub fn initial_focus(mut self, target: DialogInitialFocus) -> Self {
        self.initial_focus = target;
        self
    }

    /// Keep a dialog transaction open after its first cancel choice and move
    /// focus to a control that explains the retained state. Dirty workflows
    /// use this for their first Escape/Cancel pass; the following pass omits
    /// this option so a confirmed dismissal restores the prior workspace
    /// focus normally.
    pub fn retain_on_cancel_focus(mut self, target: DialogInitialFocus) -> Self {
        self.retained_cancel_focus = Some(target);
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

    /// Keep a parent workflow visible but inert while a nested modal owns
    /// pointer, keyboard, and assistive-technology interaction.
    pub fn interaction_enabled(mut self, enabled: bool) -> Self {
        self.interaction_enabled = enabled;
        self
    }

    /// Render the primary in the destructive (`err`) treatment.
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    /// Plain secondary button, left of the primary.
    pub fn secondary(mut self, label: impl Into<String>) -> Self {
        self.secondary = Some(label.into());
        self
    }

    /// Enable or disable the optional secondary action.
    pub fn secondary_enabled(mut self, enabled: bool) -> Self {
        self.secondary_enabled = enabled;
        self
    }

    /// Ghost button, leftmost in the footer (Revert, Cancel).
    pub fn ghost(mut self, label: impl Into<String>) -> Self {
        self.ghost = Some(label.into());
        self
    }

    /// Enable or disable the optional ghost action.
    pub fn ghost_enabled(mut self, enabled: bool) -> Self {
        self.ghost_enabled = enabled;
        self
    }

    /// Mono footer hint (validation count, shortcut reminder).
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Show the canonical workflow transaction strip immediately above the
    /// footer. It is intentionally absent in the normal idle state.
    pub fn transaction_state(
        mut self,
        tone: DialogTransactionTone,
        title: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        self.transaction_state = Some(DialogTransactionState {
            tone,
            title: title.into(),
            detail: detail.into(),
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

    /// Suppress the standard workflow header while preserving the modal's
    /// accessible name. Specialist editors use this when their body owns an
    /// identity-rich header that cannot be represented by the generic title.
    pub fn without_header(mut self) -> Self {
        self.header_visible = false;
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
        let screen = ctx.content_rect();
        let id = Id::new(("rspice.dialog", self.title.as_str()));
        let measured_height_id =
            id.with(("measured-surface-height", self.transaction_state.is_some()));
        let measured_height = self.fixed_height.or_else(|| {
            ctx.data(|data| data.get_temp::<f32>(measured_height_id))
                .or(self.initial_height)
        });
        let layout = DialogLayout::resolve(self.size, screen, measured_height);
        // `fixed_height` is a workflow-stability contract, not merely an
        // initial measurement hint. Its surface and body viewport must retain
        // the resolved height while body content scrolls beneath the fixed
        // footer, including when the requested height is clamped to the screen.
        let fill_surface_height = layout.fills_surface_height(self.fixed_height);
        let large_targets = (layout.narrow && self.size != DialogSize::ComponentEditor)
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
            if !self.interaction_enabled {
                surface.disable();
            }
            surface.set_width(width);
            if fill_surface_height {
                surface.set_min_height(max_height);
            }

            let surface_output = Frame::NONE
                .fill(if layout.app_background {
                    c.bg_app
                } else {
                    c.bg_elevated
                })
                .stroke(Stroke::new(1.0, c.border_strong))
                .corner_radius(layout.radius)
                // Every currently implemented mockup dialog inherits
                // `--shadow`. The stronger `--shadow-dialog` elevation is
                // reserved for DRC workflows, which are not represented by
                // this generic surface purpose.
                .shadow(t.shadow())
                .show(&mut surface, |ui| {
                    ui.set_width(width);
                    if fill_surface_height {
                        ui.set_min_height(max_height);
                    }
                    // Header, body, transaction state, and footer are one
                    // continuous structural stack. Their own borders and
                    // insets own every seam; egui's ordinary inter-widget gap
                    // must never open a strip of background between them.
                    let body_item_spacing_y = ui.spacing().item_spacing.y;
                    ui.spacing_mut().item_spacing.y = 0.0;
                    let header_top = ui.cursor().top();
                    let header = if self.header_visible {
                        self.header(ui, &t, large_targets)
                    } else {
                        DialogHeaderOutput {
                            closed: false,
                            close_id: None,
                        }
                    };
                    rendered_focus.close = header.close_id;
                    if header.closed {
                        choice = DialogChoice::Cancelled;
                    }
                    let header_height = ui.cursor().top() - header_top;
                    let footer_height =
                        self.footer_height(hide_close_only_footer, large_targets, width);
                    let transaction_height = if self.transaction_state.is_some() {
                        37.0
                    } else {
                        0.0
                    };
                    let body_max_height =
                        (max_height - header_height - footer_height - transaction_height).max(1.0);
                    let requested_scroll_offset = self.body_scroll_offset.as_deref().copied();
                    let flush_body = self.flush_body;
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
                                    .show(ui, |ui| {
                                        ui.spacing_mut().item_spacing.y =
                                            if flush_body { 0.0 } else { body_item_spacing_y };
                                        body(ui)
                                    })
                                    .inner
                            },
                        );
                        rendered_focus.body = body_output.inner;
                    } else {
                        let body_scroll = egui::ScrollArea::vertical()
                            .id_salt(id.with("body"))
                            .max_height(body_max_height)
                            .min_scrolled_height(if fill_surface_height {
                                body_max_height
                            } else {
                                64.0
                            })
                            .auto_shrink([false, !fill_surface_height]);
                        // Without an externally managed offset, leave egui's
                        // retained ScrollArea state untouched. Supplying a
                        // default zero here would snap the body to the top on
                        // every frame and make wheel/scrollbar input inert.
                        let body_scroll =
                            if let Some(requested_scroll_offset) = requested_scroll_offset {
                                body_scroll.vertical_scroll_offset(requested_scroll_offset)
                            } else {
                                body_scroll
                            };
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
                                .show(ui, |ui| {
                                    ui.spacing_mut().item_spacing.y =
                                        if flush_body { 0.0 } else { body_item_spacing_y };
                                    body(ui)
                                })
                                .inner
                        });
                        rendered_focus.body = body_output.inner;
                        if let Some(offset) = self.body_scroll_offset.as_deref_mut() {
                            *offset = body_output.state.offset.y;
                        }
                    }
                    // The strip and the command row answer to ids of their own.
                    // Taken from the count of what was laid out first, the
                    // row's id would move every time a transaction state opened
                    // or closed the strip above it — while the row itself, held
                    // on the surface's bottom edge, has not moved a point — and
                    // its buttons would lose the focus and hover continuity that
                    // id carries.
                    ui.scope_builder(egui::UiBuilder::new().id(id.with("transaction")), |ui| {
                        self.transaction_strip(ui, &t, width);
                    });
                    let footer = ui
                        .scope_builder(egui::UiBuilder::new().id(id.with("footer")), |ui| {
                            self.footer(ui, &t, hide_close_only_footer, large_targets, width)
                        })
                        .inner;
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

        if self.fixed_height.is_none()
            && !layout.fill_height
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
        response
            .widget_info(|| WidgetInfo::labeled(WidgetType::Window, enabled, self.title.as_str()));
        ctx.accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Dialog);
            node.set_label(self.title.as_str());
            if let Some(description) = self.description.as_deref() {
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
        if choice == DialogChoice::None
            && self.interaction_enabled
            && is_top_modal
            && !any_popup_open
        {
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
            let retained_target = matches!(choice, DialogChoice::Ghost | DialogChoice::Cancelled)
                .then_some(self.retained_cancel_focus)
                .flatten();
            if let Some(target) = retained_target {
                let target = rendered_focus.requested(target).unwrap_or(focus_id);
                ctx.memory_mut(|memory| memory.request_focus(target));
            } else {
                restore_dialog_focus(ctx, focus_state_id, focus_id, modal_layer);
            }
        }

        choice
    }

    /// Header strip; returns `true` when the close control fired.
    fn header(&self, ui: &mut Ui, t: &Tokens, large_targets: bool) -> DialogHeaderOutput {
        let c = t.color;
        let mut closed = false;
        let mut close_id = None;
        // Title-first headers set the operation above its identity subtitle and
        // give the block one measure of lead on all three sides — above the
        // title, between the lines, and below the subtitle — instead of
        // centring a fixed text stack in a taller strip.
        let title_first = self.size == DialogSize::SchematicPageSetup;
        Frame::NONE
            .fill(if self.size == DialogSize::AnalysisCatalog {
                c.bg_elevated
            } else {
                c.bg_panel
            })
            .inner_margin(Margin::symmetric(
                WORKFLOW_HEADER_HORIZONTAL_MARGIN,
                if title_first {
                    TITLE_FIRST_HEADER_LEAD as i8
                } else {
                    0
                },
            ))
            .show(ui, |ui| {
                let header_width = ui.available_width();
                ui.allocate_ui_with_layout(
                    vec2(
                        header_width,
                        if title_first {
                            0.0
                        } else {
                            WORKFLOW_HEADER_MIN_HEIGHT
                        },
                    ),
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
                                // Line box hugging the glyphs. With the font's
                                // own leading, three geometrically equal gaps
                                // still read as a larger one at the top,
                                // because part of it sits inside the line box.
                                line_height: title_first.then_some(tokens::FS_0),
                                ..Default::default()
                            },
                        );
                        let mut title_job = egui::text::LayoutJob::default();
                        title_job.append(
                            self.title.as_str(),
                            0.0,
                            egui::TextFormat {
                                font_id: theme::sans(tokens::FS_3, FontWeight::SemiBold),
                                color: c.text,
                                line_height: title_first.then_some(tokens::FS_3),
                                ..Default::default()
                            },
                        );
                        ui.allocate_ui_with_layout(
                            // A zero-height child is aligned by its empty
                            // rectangle and then grows downward, which pushed
                            // every two-line header below the mockup center and
                            // enlarged the surface. Give the authored text
                            // stack its real minimum height; wrapped narrow
                            // titles may still grow it when necessary. A
                            // title-first header sizes to its own lead instead.
                            vec2(
                                text_width,
                                if title_first {
                                    0.0
                                } else {
                                    WORKFLOW_HEADER_TEXT_MIN_HEIGHT
                                },
                            ),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.set_width(text_width);
                                ui.spacing_mut().item_spacing.y = if title_first {
                                    TITLE_FIRST_HEADER_LEAD
                                } else {
                                    3.0
                                };
                                if title_first {
                                    ui.add(egui::Label::new(title_job).wrap());
                                    ui.add(egui::Label::new(eyebrow).wrap());
                                } else {
                                    ui.add(egui::Label::new(eyebrow).wrap());
                                    ui.add(egui::Label::new(title_job).wrap());
                                }
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
    fn footer_height(
        &self,
        hide_close_only_footer: bool,
        large_targets: bool,
        surface_width: f32,
    ) -> f32 {
        if self.note_only_footer {
            return 48.0;
        }
        if self.hides_close_only_footer(hide_close_only_footer) {
            return 0.0;
        }
        if self.size == DialogSize::ComponentEditor {
            return 48.0;
        }
        if !self.footer_stacks(surface_width) {
            return 48.0;
        }

        let action_count =
            1 + usize::from(self.secondary.is_some()) + usize::from(self.ghost.is_some());
        let action_height = if large_targets {
            TOUCH_TARGET_SIDE
        } else {
            29.0
        };
        let hint_height = if self.hint.is_some() { 19.0 } else { 0.0 };
        let row_count = action_count + usize::from(self.hint.is_some());
        20.0 + action_count as f32 * action_height
            + hint_height
            + row_count.saturating_sub(1) as f32 * 6.0
    }

    fn footer_stacks(&self, surface_width: f32) -> bool {
        if self.size == DialogSize::ComponentEditor {
            return false;
        }
        let button_width = |label: &str| label.chars().count() as f32 * 6.4 + 20.0;
        let mut required = button_width(&self.primary);
        let mut items: usize = 1;
        for label in [self.secondary.as_deref(), self.ghost.as_deref()]
            .into_iter()
            .flatten()
        {
            required += button_width(label);
            items += 1;
        }
        if let Some(hint) = self.hint.as_deref() {
            required += hint.chars().count() as f32 * 6.4;
            items += 1;
        }
        required += items.saturating_sub(1) as f32 * 6.0;
        required > (surface_width - 2.0 * WORKFLOW_FOOTER_HORIZONTAL_MARGIN as f32).max(1.0)
    }

    fn transaction_strip(&self, ui: &mut Ui, t: &Tokens, surface_width: f32) {
        let Some(transaction) = self.transaction_state.as_ref() else {
            return;
        };
        let color = match transaction.tone {
            DialogTransactionTone::Progress => t.color.accent,
            DialogTransactionTone::Complete => t.color.ok,
            DialogTransactionTone::Error => t.color.err,
        };
        // A preceding split-pane body may leave `available_width()` at its
        // first column's retained minimum. The transaction state is a dialog-
        // level strip, so resolve its rectangle from the surface instead of
        // inheriting the last body's content geometry.
        let strip_rect = Rect::from_min_size(
            egui::pos2(ui.max_rect().left(), ui.cursor().top()),
            vec2(surface_width, 37.0),
        );
        let response = ui.allocate_rect(strip_rect, Sense::hover());
        ui.painter().rect_filled(strip_rect, 0.0, t.color.bg_panel);
        ui.painter().hline(
            strip_rect.x_range(),
            strip_rect.top(),
            Stroke::new(1.0, t.color.border_strong),
        );
        let content_rect = Rect::from_min_max(
            strip_rect.min + vec2(12.0, 6.0),
            strip_rect.max - vec2(12.0, 6.0),
        );
        let mut strip_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(content_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        strip_ui.spacing_mut().item_spacing.x = 8.0;
        let (indicator, _) = strip_ui.allocate_exact_size(vec2(14.0, 14.0), Sense::hover());
        strip_ui
            .painter()
            .circle_stroke(indicator.center(), 4.0, Stroke::new(1.0, color));
        if transaction.tone == DialogTransactionTone::Complete {
            strip_ui
                .painter()
                .circle_filled(indicator.center(), 2.0, color);
        }
        strip_ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 1.0;
            ui.label(
                egui::RichText::new(transaction.title.as_str())
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.label(
                egui::RichText::new(transaction.detail.as_str())
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(if transaction.tone == DialogTransactionTone::Error {
                egui::accesskit::Role::Alert
            } else {
                egui::accesskit::Role::Status
            });
            node.set_label(transaction.title.as_str());
            node.set_description(transaction.detail.as_str());
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
            if let Some(hint) = self.hint.as_deref() {
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
        let component_editor = self.size == DialogSize::ComponentEditor;
        let stack_footer = self.footer_stacks(surface_width);
        let line_y = ui.cursor().top();
        let footer_rect = dialog_footer_rect(
            ui.max_rect().left(),
            line_y,
            surface_width,
            self.footer_height(hide_close_only_footer, large_targets, surface_width),
        );
        ui.painter()
            .hline(footer_rect.x_range(), line_y, Stroke::new(1.0, c.border));
        let mut footer_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(footer_rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        let horizontal_margin = if component_editor {
            16
        } else {
            WORKFLOW_FOOTER_HORIZONTAL_MARGIN
        };
        let vertical_margin = if large_targets {
            2
        } else {
            WORKFLOW_FOOTER_VERTICAL_MARGIN
        };
        Frame::NONE
            .fill(if component_editor {
                c.bg_panel_2
            } else {
                c.bg_panel
            })
            .inner_margin(Margin::symmetric(
                horizontal_margin,
                if stack_footer {
                    WORKFLOW_FOOTER_VERTICAL_MARGIN
                } else {
                    vertical_margin
                },
            ))
            .show(&mut footer_ui, |ui| {
                if stack_footer {
                    ui.spacing_mut().item_spacing.y = 6.0;
                    if let Some(hint) = self.hint.as_deref() {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(hint)
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(c.text_faint),
                            )
                            .wrap(),
                        );
                    }
                    let action_width = ui.available_width();
                    let action_height = if large_targets {
                        TOUCH_TARGET_SIDE
                    } else {
                        29.0
                    };
                    let primary = crate::ui::widgets::Button::new(&self.primary)
                        .accent()
                        .destructive(self.destructive)
                        .enabled(self.primary_enabled)
                        .min_width(action_width)
                        .max_width(action_width)
                        .min_height(action_height)
                        .show(ui);
                    primary_id = self.primary_enabled.then_some(primary.id);
                    if primary.clicked() {
                        choice = DialogChoice::Primary;
                    }
                    if let Some(label) = self.secondary.as_deref() {
                        let secondary = crate::ui::widgets::Button::new(label)
                            .enabled(self.secondary_enabled)
                            .min_width(action_width)
                            .max_width(action_width)
                            .min_height(action_height)
                            .show(ui);
                        secondary_id = self.secondary_enabled.then_some(secondary.id);
                        if secondary.clicked() {
                            choice = DialogChoice::Secondary;
                        }
                    }
                    if let Some(label) = self.ghost.as_deref() {
                        let ghost = crate::ui::widgets::Button::new(label)
                            .ghost()
                            .enabled(self.ghost_enabled)
                            .min_width(action_width)
                            .max_width(action_width)
                            .min_height(action_height)
                            .show(ui);
                        ghost_id = self.ghost_enabled.then_some(ghost.id);
                        if ghost.clicked() {
                            choice = DialogChoice::Ghost;
                        }
                    }
                    return;
                }
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;
                    if let Some(hint) = self.hint.as_deref() {
                        let label = egui::Label::new(
                            egui::RichText::new(hint)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(if component_editor {
                                    c.warn
                                } else {
                                    c.text_faint
                                }),
                        );
                        if component_editor {
                            ui.add_sized(
                                [(ui.available_width() - 190.0).max(40.0), 18.0],
                                label.truncate(),
                            )
                            .on_hover_text(hint);
                        } else {
                            ui.add(label);
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let primary = crate::ui::widgets::Button::new(&self.primary)
                            .accent()
                            .destructive(self.destructive)
                            .enabled(self.primary_enabled)
                            .show(ui);
                        primary_id = self.primary_enabled.then_some(primary.id);
                        if primary.clicked() {
                            choice = DialogChoice::Primary;
                        }
                        if let Some(label) = self.secondary.as_deref() {
                            let secondary = crate::ui::widgets::Button::new(label)
                                .enabled(self.secondary_enabled)
                                .show(ui);
                            secondary_id = self.secondary_enabled.then_some(secondary.id);
                            if secondary.clicked() {
                                choice = DialogChoice::Secondary;
                            }
                        }
                        if let Some(label) = self.ghost.as_deref() {
                            let button =
                                crate::ui::widgets::Button::new(label).enabled(self.ghost_enabled);
                            let ghost = if component_editor {
                                button.show(ui)
                            } else {
                                button.ghost().show(ui)
                            };
                            ghost_id = self.ghost_enabled.then_some(ghost.id);
                            if ghost.clicked() {
                                choice = DialogChoice::Ghost;
                            }
                        }
                    });
                });
            });
        ui.allocate_rect(footer_rect, Sense::hover());
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
    dialog_footer_rect(left, top, surface_width, 48.0)
}

fn dialog_footer_rect(left: f32, top: f32, surface_width: f32, height: f32) -> Rect {
    Rect::from_min_size(
        egui::pos2(left, top),
        vec2(surface_width.max(1.0), height.max(1.0)),
    )
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
    dialog_tabs_impl(ui, tabs, active, None);
}

fn dialog_tabs_impl(ui: &mut Ui, tabs: &[&str], active: &mut usize, available_width: Option<f32>) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let standard_galleys = dialog_tab_galleys(ui, tabs, 0.06 * tokens::FS_0);
    let standard_text_width = dialog_tab_text_width(&standard_galleys);
    let use_compact_tracking = available_width
        .is_some_and(|width| dialog_tab_required_width(standard_text_width, tabs.len()) > width);
    let galleys = if use_compact_tracking {
        // The mockup's narrow panel tabs retain their complete labels and
        // remove tracking before allowing overflow. Keeping the font and
        // labels unchanged avoids both illegible scaling and invented
        // abbreviations in compact property panes.
        dialog_tab_galleys(ui, tabs, 0.0)
    } else {
        standard_galleys
    };
    let text_width = dialog_tab_text_width(&galleys);
    let horizontal_padding = dialog_tab_horizontal_padding(available_width, text_width, tabs.len());
    let text_inset = horizontal_padding * 0.5;
    let underline_inset = text_inset.min(5.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        for (index, (label, galley)) in tabs.iter().zip(galleys).enumerate() {
            let selected = *active == index;
            let (rect, response) = ui.allocate_exact_size(
                vec2(galley.size().x + horizontal_padding, 24.0),
                Sense::click(),
            );
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
                egui::pos2(
                    rect.left() + text_inset,
                    rect.center().y - galley.size().y * 0.5,
                ),
                galley,
                color,
            );
            if selected {
                ui.painter().hline(
                    egui::Rangef::new(
                        rect.left() + underline_inset,
                        rect.right() - underline_inset,
                    ),
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

fn dialog_tab_galleys(
    ui: &mut Ui,
    tabs: &[&str],
    extra_letter_spacing: f32,
) -> Vec<std::sync::Arc<egui::Galley>> {
    tabs.iter()
        .map(|label| {
            let mut job = egui::text::LayoutJob::default();
            job.append(
                &label.to_uppercase(),
                0.0,
                egui::TextFormat {
                    font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                    color: egui::Color32::PLACEHOLDER,
                    extra_letter_spacing,
                    ..Default::default()
                },
            );
            ui.fonts_mut(|fonts| fonts.layout_job(job))
        })
        .collect()
}

fn dialog_tab_text_width(galleys: &[std::sync::Arc<egui::Galley>]) -> f32 {
    galleys.iter().map(|galley| galley.size().x).sum()
}

fn dialog_tab_required_width(text_width: f32, tab_count: usize) -> f32 {
    let gaps = tab_count.saturating_sub(1) as f32 * 2.0;
    text_width + gaps + tab_count as f32 * DIALOG_TAB_MIN_HORIZONTAL_PADDING
}

fn dialog_tab_horizontal_padding(
    available_width: Option<f32>,
    text_width: f32,
    tab_count: usize,
) -> f32 {
    let gaps = tab_count.saturating_sub(1) as f32 * 2.0;
    available_width
        .filter(|_| tab_count > 0)
        .map(|width| ((width - text_width - gaps) / tab_count as f32).floor())
        .unwrap_or(DIALOG_TAB_HORIZONTAL_PADDING)
        .clamp(
            DIALOG_TAB_MIN_HORIZONTAL_PADDING,
            DIALOG_TAB_HORIZONTAL_PADDING,
        )
}

#[cfg(test)]
mod tests;
