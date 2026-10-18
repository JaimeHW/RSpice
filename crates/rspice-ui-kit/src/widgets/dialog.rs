//! The RSpice modal primitive.
//!
//! Every dialog in the application is the same three-part surface over a
//! scrim:
//!
//! - **Scrim** — full-viewport canvas-black wash that blocks interaction
//!   with everything underneath. Clicking it does not dismiss; `Esc` does.
//! - **Surface** — each callsite declares a mockup-owned purpose: confirmation,
//!   transaction, manager, wide workflow, or capability review. Purpose controls exact
//!   geometry, fill, corner radius, viewport gutter, and narrow behavior; no
//!   retired generic small/medium/large shell remains.
//! - **Header / footer** — the semibold title alone + close on top, as a
//!   native dialog has it; footer is
//!   `[ghost] [secondary] [primary]`, or `[secondary] … [ghost] [primary]`
//!   when the secondary is set apart, with the primary always rightmost,
//!   always exactly one, accent-filled (or `err` when destructive), plus an
//!   optional mono hint on the left.
//! - **Body** — the scrolling middle region, laid out inside a stable
//!   scrollbar gutter: the bar's track is withheld from the body whether or
//!   not a bar is showing, so the body's width answers to the surface's width
//!   and never to its own height. A content-height surface therefore reaches
//!   its content in one re-measure instead of creeping toward it.
//!
//! Keys: `Esc` cancels, `Enter` activates the primary when it is enabled.
//! Dialogs edit a draft and commit on the primary — the primary and
//! cancel are never the same operation.

use egui::{
    Color32, Context, FocusDirection, Frame, Id, Key, Margin, Modifiers, Order, Popup, Rect, Sense,
    Stroke, Ui, UiKind, WidgetInfo, WidgetType, vec2,
};

use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};
use crate::widgets::{PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE, Problem, paint_problem_mark};

mod keyboard;

/// Mockup-owned dialog surface purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogSize {
    /// Short confirmations — a few status lines over at most three verbs:
    /// 480 pt wide, content-height, capped at 760 pt. At 560 pt and below it
    /// stays a content-height card, centred with a 12 pt gutter on each side
    /// as a native alert is, rather than becoming a full-viewport sheet.
    Confirmation,
    /// Compact command forms: the three or four fields a schematic command
    /// needs before it arms its canvas tool. 420 pt wide, content-height,
    /// capped at 560 pt.
    ///
    /// It behaves like [`Self::Confirmation`] at every viewport — a
    /// content-height card, never a full-screen sheet — because a form of four
    /// rows that fills a phone is a form pretending to be a workspace. Build
    /// every one of these with [`Dialog::overlay_scrollbar`]: they are
    /// scroll-free by construction, and the standard body otherwise withholds a
    /// gutter that shows as bare surface down the trailing edge.
    CommandForm,
    /// Transactional edits and confirmations: 760 pt wide, content-height,
    /// capped at 760 pt, and a content-height card with the same 12 pt gutter
    /// as [`Self::Confirmation`] at 560 pt and below.
    Transaction,
    /// Mockup-authored workflow transactions: 760 pt wide and
    /// content-height, with the full-viewport workflow shell applied at the
    /// shared 820 pt dialog breakpoint; individual fields stack by local width.
    ///
    /// Below the breakpoint it gives up both side gutters and keeps its
    /// content's height. It took the whole viewport until the ceiling at a
    /// narrow width became the viewport rather than this size's own 760 pt:
    /// the tallest route on it, the plan manager's Create form, measures
    /// 890 pt of content, which the old ceiling would have scrolled the last
    /// section of and the viewport holds whole.
    SimulationWorkflow,
    /// Mockup-owned specialist workspace discovery manager: 760 pt wide on
    /// desktop and full-viewport at the 820 pt one-column breakpoint.
    SpecialistToolBrowser,
    /// Browsers and settings managers: 760 pt wide and content-height,
    /// capped at 530 pt, with the mockup's 28/34 pt viewport gutters and
    /// 8 pt phone inset.
    ///
    /// It used to be pinned at its cap at every size, and at a narrow
    /// viewport at the viewport. A window manager listing one window then
    /// left 527 points of bare surface above its footer in a 900 pt
    /// portrait and 196 on the desktop. A manager is as tall as what it is
    /// managing.
    Manager,
    /// Simulation Studio's mockup-owned analysis/workflow catalog: 1180 ×
    /// 780 pt, anchored 12 pt below the viewport top, and reduced to a 4 pt
    /// perimeter gutter on phone-sized viewports.
    AnalysisCatalog,
    /// Wide numerical/setup workflows: 980 pt wide, content-height capped at
    /// 760 pt, and a 12 pt gutter card at the mockup's 820 pt breakpoint.
    WideWorkflow,
    /// Two-pane pick-and-preview transactions: 860 pt wide, content-height
    /// capped at 620 pt, a 12 pt gutter card at the shared 820 pt breakpoint.
    ///
    /// Narrower than [`Self::WideWorkflow`] because the right pane holds
    /// stacked waveform strips rather than a table: past about 860 pt the
    /// strips stop gaining information and the list is left stranded from the
    /// preview it drives.
    StimulusLink,
    /// Drawing-sheet supporting workflows: 1160 pt wide, content-height
    /// capped at 760 pt, and edge-to-edge at the shared 820 pt breakpoint.
    DrawingSheetWorkflow,
    /// The drawing-sheet preset catalog: 880 pt wide and content-height at
    /// every viewport, otherwise [`Self::DrawingSheetWorkflow`].
    ///
    /// Narrower because its table states six short columns and four row
    /// verbs. Given the workflow width it spent a third of the surface on
    /// nothing at all, however the columns were sized.
    ///
    /// Content-height on a narrow viewport where the workflow fills it,
    /// because a catalog of presets is as tall as the presets there are: a
    /// project holding none spent two thirds of a phone screen saying so.
    DrawingSheetLibrary,
    /// Account, license, and privacy console: one 600 pt reading column,
    /// content-height capped at 760 pt, and a full-viewport sheet at 560 pt
    /// and below.
    AccountManager,
    /// Governed capability matrices: 1040 pt wide, content-height capped at
    /// 760 pt, edge-to-edge at the mockup's 820 pt breakpoint.
    ///
    /// Edge to edge rather than a gutter card, because at exactly 820 pt the
    /// 24 pt a gutter costs leaves a 794 pt body — one point class under the
    /// plan manager's split minimum, which stacks its records table over its
    /// aside, and stacked does not fit the 640 pt that shape has.
    CapabilityReview,
    /// Execution queue, target, and retained-run manager: 1120 pt wide and
    /// content-height, capped at 680 pt, with the mockup's 12 pt desktop
    /// perimeter and 4 pt phone perimeter.
    ///
    /// Content-height for the reason [`Self::Manager`] is: a queue holding
    /// two runs left 171 points of bare surface above the footer at the
    /// desktop width, and a queue is as tall as the queue.
    JobsManager,
    /// Permanent authored drawing-sheet studio: 1280 pt wide and
    /// content-height, capped at 760 pt, with a fixed section navigator,
    /// editable sheet form, and live effect preview.
    ///
    /// The studio's own callsite states a fixed 760 pt height, which is
    /// what holds its two columns to one measure; this purpose no longer
    /// adds a second pin on top of it. Below the breakpoint that pin took
    /// the whole viewport and left 298 points of bare surface in a 900 pt
    /// portrait, where the surface is now as tall as the studio.
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
            Self::Confirmation => DialogSurfaceSpec {
                width: 480.0,
                max_height: 760.0,
                horizontal_inset: 24.0,
                vertical_inset: 24.0,
                narrow_max_width: 560.0,
                narrow_inset: 24.0,
                narrow_vertical_inset: 24.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: false,
                fill_narrow_viewport: false,
                fill_height: false,
                app_background: true,
                radius: 4.0,
                top_anchored: false,
            },
            Self::CommandForm => DialogSurfaceSpec {
                width: 420.0,
                max_height: 560.0,
                horizontal_inset: 24.0,
                vertical_inset: 24.0,
                narrow_max_width: 560.0,
                narrow_inset: 24.0,
                narrow_vertical_inset: 24.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: false,
                fill_narrow_viewport: false,
                fill_height: false,
                app_background: true,
                radius: 4.0,
                top_anchored: false,
            },
            Self::Transaction => DialogSurfaceSpec {
                width: 760.0,
                max_height: 760.0,
                horizontal_inset: 24.0,
                vertical_inset: 24.0,
                narrow_max_width: 560.0,
                narrow_inset: 24.0,
                narrow_vertical_inset: 24.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: false,
                fill_narrow_viewport: false,
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
                fill_narrow_viewport: false,
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
                fill_narrow_viewport: false,
                fill_height: false,
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
                narrow_inset: 24.0,
                narrow_vertical_inset: 24.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: false,
                fill_narrow_viewport: false,
                fill_height: false,
                app_background: true,
                radius: 4.0,
                top_anchored: false,
            },
            Self::StimulusLink => DialogSurfaceSpec {
                width: 860.0,
                max_height: 620.0,
                horizontal_inset: 24.0,
                vertical_inset: 24.0,
                narrow_max_width: 820.0,
                narrow_inset: 24.0,
                narrow_vertical_inset: 24.0,
                cap_narrow_height: false,
                edge_to_edge_narrow: false,
                fill_narrow_viewport: false,
                fill_height: false,
                app_background: true,
                radius: 4.0,
                top_anchored: false,
            },
            Self::AccountManager => DialogSurfaceSpec {
                width: 600.0,
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
                fill_narrow_viewport: false,
                fill_height: false,
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
            Self::DrawingSheetLibrary => DialogSurfaceSpec {
                width: 880.0,
                narrow_inset: 24.0,
                narrow_vertical_inset: 24.0,
                edge_to_edge_narrow: false,
                fill_narrow_viewport: false,
                ..Self::DrawingSheetWorkflow.spec()
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
                fill_narrow_viewport: false,
                fill_height: false,
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

/// The header: the title alone, centred in a row as tall as the footer, so
/// the surface opens and closes on the same measure at every size.
const HEADER_MIN_HEIGHT: f32 = 48.0;
/// The narrowest surface on which a footer with its secondary set apart
/// stays split; narrower surfaces stack their verbs full-width.
const SPLIT_FOOTER_MIN_WIDTH: f32 = 440.0;
/// The header shares the body's and the footer's side inset, so a title, the
/// rows under it and the footer's verbs start on one left edge and the close
/// target ends on the right edge the primary ends on. At 15 the title sat
/// 3 pt inside everything below it.
const WORKFLOW_HEADER_HORIZONTAL_MARGIN: i8 = WORKFLOW_FOOTER_HORIZONTAL_MARGIN;
const WORKFLOW_FOOTER_HORIZONTAL_MARGIN: i8 = 12;
const WORKFLOW_FOOTER_VERTICAL_MARGIN: i8 = 10;
/// Inset above and below a single row of touch-height footer actions.
///
/// The row held a 44-point action in the 48-point footer a 29-point action gets,
/// which left two points either side and read as buttons wedged into the bar.
/// The footer grows with the action instead.
const TOUCH_FOOTER_VERTICAL_MARGIN: i8 = 8;
const DIALOG_CLOSE_TARGET_WIDTH: f32 = 28.0;
const DIALOG_CLOSE_TARGET_HEIGHT: f32 = 27.0;
/// The surface's own border, drawn inside the resolved surface rect. The
/// header, body, and footer share the rect less this measure on every side.
const SURFACE_BORDER_WIDTH: f32 = 1.0;

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
    #[cfg(test)]
    fn resolve(size: DialogSize, screen: Rect, measured_height: Option<f32>) -> Self {
        Self::resolve_with_width(size, screen, measured_height, None)
    }

    /// As [`Self::resolve`], with the width a content-width body stated.
    fn resolve_with_width(
        size: DialogSize,
        screen: Rect,
        measured_height: Option<f32>,
        content_width: Option<f32>,
    ) -> Self {
        let mut spec = size.spec();
        if let Some(width) = content_width {
            spec.width = width.min(spec.width);
        }
        let narrow = screen.width() <= spec.narrow_max_width;
        let surface_rect = if narrow && spec.edge_to_edge_narrow {
            if spec.fill_narrow_viewport {
                screen
            } else {
                // Edge to edge is a statement about the side gutters: this
                // surface spends none of a narrow viewport's width on one.
                // It used to carry the height with it, so a purpose that
                // reached both side edges also reached the top and the
                // bottom whatever its content held. The two are separate
                // here, and the height is the content's as it is everywhere
                // else, held to the viewport.
                let viewport = screen.height().max(1.0);
                let height = measured_height.unwrap_or(viewport).min(viewport);
                Rect::from_center_size(screen.center(), vec2(screen.width(), height))
            }
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
                    let viewport = (screen.height() - vertical_inset).max(1.0);
                    if spec.fill_height {
                        spec.max_height.min(viewport)
                    } else {
                        // `max_height` is a desktop ceiling: how tall this
                        // surface may grow inside a desktop window. At narrow
                        // the viewport is the only ceiling that means
                        // anything, and holding the desktop number there made
                        // a content-height route scroll its last row inside a
                        // portrait that had room for all of it.
                        let ceiling = if narrow { viewport } else { spec.max_height };
                        measured_height.unwrap_or(ceiling).min(ceiling)
                    }
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
    /// Focus a known body control before it processes opening-frame input.
    /// The body must return this id from `show_with_initial_body_focus`.
    Control(Id),
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
            DialogInitialFocus::Control(id) => self.body.filter(|body| *body == id),
            DialogInitialFocus::Close => self.close,
            DialogInitialFocus::Primary => self.primary,
            DialogInitialFocus::Ghost => self.ghost,
        }
    }
}

/// A rendered transaction whose cancellation focus is finalized on drop.
/// The caller can inspect the freshly edited draft before retaining focus
/// for a discard confirmation. Accepted cancellation restores the original
/// focus automatically, including on early returns.
pub struct DialogResponse {
    pub choice: DialogChoice,
    cancel_focus: Option<DialogCancelFocus>,
}

struct DialogCancelFocus {
    ctx: Context,
    state_id: Id,
    container_id: Id,
    layer: egui::LayerId,
    rendered: DialogRenderedFocus,
    retained_target: Option<DialogInitialFocus>,
}

impl DialogResponse {
    /// Hand the input that followed the cancelling Escape to the control the
    /// returning page opens with, which the caller renders in this same pass.
    /// Without it those keystrokes reach that control two frames later, after
    /// the reader has already seen the page they belong to carrying stale text.
    pub fn route_cancel_input(&self, target: Id) {
        let Some(focus) = &self.cancel_focus else {
            return;
        };
        if let Some(events) = keyboard::take_deferred(&focus.ctx) {
            crate::input::hand_opening_input(&focus.ctx, target, events);
        }
    }

    /// Keep the cancelled dialog open and focus its confirmation control.
    /// Call only after applying the workflow's current close policy.
    pub fn retain_cancel_focus(&mut self, target: DialogInitialFocus) {
        if let Some(focus) = &mut self.cancel_focus {
            focus.retained_target = Some(target);
        }
    }
}

impl Drop for DialogResponse {
    fn drop(&mut self) {
        if let Some(focus) = self.cancel_focus.take() {
            if let Some(target) = focus.retained_target {
                let target = focus
                    .rendered
                    .requested(target)
                    .unwrap_or(focus.container_id);
                focus.ctx.memory_mut(|memory| memory.request_focus(target));
            } else {
                keyboard::unregister(&focus.ctx, focus.layer);
                restore_dialog_focus(&focus.ctx, focus.state_id, focus.container_id, focus.layer);
            }
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

/// What a footer hint is saying.
///
/// A hint the reader has to notice carries a [`Problem`] mark in front of
/// neutral text, the grammar of a compiler's problem list: the mark holds the
/// severity and its colour, and the sentence is never coloured.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DialogHintTone {
    /// Metadata: a count, a shortcut, a reminder. No mark.
    #[default]
    Neutral,
    /// What the primary is about to do that a reader would want to know
    /// first — a re-place, a discard, a widening scope.
    Warn,
    /// Why the last attempt refused.
    Error,
}

impl DialogHintTone {
    /// The problem a marked hint states; a neutral hint states none.
    const fn problem(self) -> Option<Problem> {
        match self {
            Self::Neutral => None,
            Self::Warn => Some(Problem::Warning),
            Self::Error => Some(Problem::Failure),
        }
    }

    /// The sentence's ink: faint metadata, or dim beside a mark that carries
    /// the severity.
    const fn text_color(self, c: &crate::palette::Palette) -> Color32 {
        match self {
            Self::Neutral => c.text_faint,
            Self::Warn | Self::Error => c.text_dim,
        }
    }

    /// Paint the leading mark.
    ///
    /// A neutral hint has no mark, because there is nothing about it to
    /// notice: the sentence is the whole of it.
    fn mark(self, ui: &mut Ui) {
        let Some(problem) = self.problem() else {
            return;
        };
        let (rect, response) =
            ui.allocate_exact_size(vec2(PROBLEM_MARK_SIDE, PROBLEM_MARK_SIDE), Sense::hover());
        paint_problem_mark(ui.painter(), rect, problem);
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label(match self {
                Self::Warn => "Consequence",
                _ => "Refused",
            });
        });
    }

    /// The mark and its sentence, left to right in whatever track is left.
    ///
    /// The mark sits the problem line's advance from the sentence, inside this
    /// track only: they are one statement, and the actions beside them keep
    /// the footer's own rhythm.
    fn hint(self, ui: &mut Ui, hint: &str, color: Color32) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = HINT_MARK_GAP;
            self.mark(ui);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(hint)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(color),
                )
                .truncate(),
            );
        });
    }
}

/// Air between a footer hint's mark and its sentence: the problem line's, so
/// a marked hint and a marked row read as the same mark.
const HINT_MARK_GAP: f32 = PROBLEM_MARK_ADVANCE - PROBLEM_MARK_SIDE;

/// Mockup transaction strip rendered between a workflow body and its footer.
///
/// The strip's words are neutral. Only an error carries a mark — the
/// [`Problem::Failure`] cross in front of its title — and only an error is
/// announced as an alert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogTransactionTone {
    /// A transaction in flight. No mark: the busy primary is the live
    /// indicator.
    Progress,
    /// What a finished transaction changed, when the dialog stays open and
    /// nothing else on it says so. No mark: the title says it.
    Complete,
    /// Why the transaction stopped.
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
    secondary_leading: bool,
    ghost: Option<String>,
    ghost_enabled: bool,
    hint: Option<String>,
    hint_tone: DialogHintTone,
    transaction_state: Option<DialogTransactionState>,
    body_scroll_offset: Option<&'a mut f32>,
    flush_body: bool,
    overlay_scrollbar: bool,
    manual_body_scroll: bool,
    note_only_footer: bool,
    header_visible: bool,
    initial_focus: DialogInitialFocus,
    initial_height: Option<f32>,
    fixed_height: Option<f32>,
    content_width: Option<f32>,
}

impl<'a> Dialog<'a> {
    /// A dialog with the given title and primary-action label. The header
    /// carries the title alone.
    pub fn new(title: impl Into<String>, primary: impl Into<String>) -> Self {
        Self {
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
            secondary_leading: false,
            ghost: None,
            ghost_enabled: true,
            hint: None,
            hint_tone: DialogHintTone::Neutral,
            transaction_state: None,
            body_scroll_offset: None,
            flush_body: false,
            overlay_scrollbar: false,
            manual_body_scroll: false,
            note_only_footer: false,
            header_visible: true,
            initial_focus: DialogInitialFocus::Container,
            initial_height: None,
            fixed_height: None,
            content_width: None,
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

    /// Take the surface's width from the body instead of from the size.
    ///
    /// For a catalog whose columns are as wide as the widest thing in them: an
    /// authored width is either wider than that table, which leaves a band of
    /// surface no column reaches, or narrower, which scrolls it. The body is
    /// the only honest source for the width, so it states it. Still capped by
    /// the size's own width and by the viewport, and ignored by a narrow
    /// edge-to-edge layout, which is the viewport's width by definition.
    pub fn width_from_content(mut self, width: f32) -> Self {
        self.content_width = width.is_finite().then(|| width.max(1.0));
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

    /// Set the secondary action apart at the footer's leading edge, with the
    /// ghost and the primary together at the trailing edge: the save-changes
    /// grouping, where "Don't save" must not sit beside the primary it could
    /// be mistaken for. A stacked footer keeps the canonical order.
    pub fn secondary_leading(mut self) -> Self {
        self.secondary_leading = true;
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

    /// Mark the footer hint.
    ///
    /// A hint is metadata by default — a count, a shortcut — and stays in the
    /// faint register. A hint that states what the primary is about to *do*,
    /// or why it refused, is not metadata: it is the last thing a reader sees
    /// before committing, so it takes the warning or failure mark in front of
    /// it and steps up to dim text. The mark alone is coloured.
    pub fn hint_tone(mut self, tone: DialogHintTone) -> Self {
        self.hint_tone = tone;
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

    /// Float the body's scrollbar over its content instead of withholding a
    /// gutter for it.
    ///
    /// A flush body paints fills and rules to its own edges, so a withheld
    /// gutter shows as a strip of bare surface down the dialog's trailing edge.
    /// A floating bar allocates nothing, which leaves the body's width a function
    /// of the surface's width alone — the same stability the gutter buys.
    pub fn overlay_scrollbar(mut self) -> Self {
        self.overlay_scrollbar = true;
        self
    }

    /// Where a body that fills its surface ends, for content that wants to reach
    /// it.
    ///
    /// `Some` only inside the body of a dialog whose surface height is fixed by
    /// its layout rather than measured from its content. There the body is
    /// handed more height than its content may need, and a surface that closes
    /// itself with a rule leaves bare body between that rule and the footer. A
    /// content-height surface reports `None`: filling it would hold the surface
    /// at whatever height it was first laid out in.
    pub fn body_fill_floor(ui: &Ui) -> Option<f32> {
        ui.data(|data| data.get_temp::<Option<f32>>(body_fill_floor_id()))
            .flatten()
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
    /// [`DialogInitialFocus::BodyControl`] is selected, or to confirm the id
    /// supplied by [`DialogInitialFocus::Control`]. Subsequent renders preserve
    /// the user's current focus.
    pub fn show_with_initial_body_focus(
        self,
        ctx: &Context,
        body: impl FnOnce(&mut Ui) -> Option<Id>,
    ) -> DialogChoice {
        self.show_transaction(ctx, body).choice
    }

    /// Show a workflow that decides whether to accept cancellation after
    /// processing body edits. Keep the response local to this render call;
    /// dropping it completes cancellation focus handling. Other choices
    /// restore focus immediately, before the caller handles their action.
    pub fn show_transaction(
        mut self,
        ctx: &Context,
        body: impl FnOnce(&mut Ui) -> Option<Id>,
    ) -> DialogResponse {
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
        let layout = DialogLayout::resolve_with_width(
            self.size,
            screen,
            measured_height,
            self.content_width,
        );
        // `fixed_height` is a workflow-stability contract, not merely an
        // initial measurement hint. Its surface and body viewport must retain
        // the resolved height while body content scrolls beneath the fixed
        // footer, including when the requested height is clamped to the screen.
        let fill_surface_height = layout.fills_surface_height(self.fixed_height);
        // The side of a finger-sized control, when this dialog is laid out for
        // one: the theme's own touch target, so a raised Accessibility target
        // reaches the close button and the footer actions too.
        let large_targets = ((layout.narrow && self.size != DialogSize::ComponentEditor)
            || (self.size == DialogSize::AnalysisCatalog && screen.width() <= 820.0)
            || ctx.input(|input| input.has_touch_screen()))
        .then(|| t.metrics.touch_target());
        let hide_close_only_footer = self.size == DialogSize::CapabilityReview;
        let _control_height_override = ControlHeightOverride::new(ctx, large_targets);
        let focus_id = id.with("move");
        let focus_state_id = id.with("focus-state");

        let area = egui::Area::new(id)
            .kind(UiKind::Modal)
            .sense(Sense::focusable_noninteractive())
            .order(Order::Foreground)
            .fixed_pos(screen.min);
        let modal_layer = area.layer();
        keyboard::register(ctx, modal_layer, self.interaction_enabled);
        let any_popup_open = Popup::is_any_open(ctx);
        let is_top_modal = ctx.memory_mut(|memory| {
            memory.set_modal_layer(modal_layer);
            memory.top_modal_layer() == Some(modal_layer)
        });
        let is_top_modal =
            keyboard::top_layer(ctx).map_or(is_top_modal, |layer| layer == modal_layer);

        let opened_this_pass = begin_dialog_focus(ctx, focus_state_id);
        if opened_this_pass || !focus_is_within_modal(ctx, modal_layer) {
            let initial_target = match self.initial_focus {
                DialogInitialFocus::Control(id) if self.interaction_enabled => id,
                _ => focus_id,
            };
            // A known control can process text and editing keys immediately;
            // callbacks that discover their target retain the container fallback.
            ctx.memory_mut(|memory| {
                memory.request_focus(initial_target);
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

            let surface_frame = Frame::NONE
                .fill(if layout.app_background {
                    c.bg_app
                } else {
                    c.bg_elevated
                })
                .stroke(Stroke::new(SURFACE_BORDER_WIDTH, c.border_strong))
                .corner_radius(layout.radius)
                // Every currently implemented mockup dialog inherits
                // `--shadow`. The stronger `--shadow-dialog` elevation is
                // reserved for DRC workflows, which are not represented by
                // this generic surface purpose.
                .shadow(t.shadow());
            // The resolved surface rect measures the outer edge, border
            // included, while the frame hands its content that rect less the
            // border on every side. The structural stack is therefore solved
            // against this content box. Solved against the outer size, header
            // plus body plus footer overrun the surface by exactly the border
            // — the same offset on every dialog whatever its content — and the
            // footer, painted after the body, covers the body's last points.
            let surface_margin = surface_frame.total_margin().sum();
            let content = (vec2(width, max_height) - surface_margin).max(vec2(1.0, 1.0));
            // How much of the body the scroll area could not show this pass. A
            // content-height surface owes its next pass exactly this much more
            // room; a surface that fills its height ignores it and scrolls.
            let mut unshown_body_height = 0.0_f32;
            let surface_output = surface_frame.show(&mut surface, |ui| {
                ui.set_width(content.x);
                if fill_surface_height {
                    ui.set_min_height(content.y);
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
                    self.footer_height(hide_close_only_footer, large_targets, content.x);
                let transaction_height = if self.transaction_state.is_some() {
                    37.0
                } else {
                    0.0
                };
                let body_max_height =
                    (content.y - header_height - footer_height - transaction_height).max(1.0);
                let requested_scroll_offset = self.body_scroll_offset.as_deref().copied();
                let flush_body = self.flush_body;
                // The body's own track, with the scrollbar's gutter withheld
                // from it whether or not a bar is showing this pass.
                //
                // `allocated_width` is exactly what a shown bar takes out of a
                // scroll area's inner width, so withholding it makes the body's
                // width a function of the surface's width alone. That is the
                // whole of the fix: the body's height is a function of its
                // width, so while its width could still answer to whether a bar
                // was showing, and a bar shows when the body is too tall, the
                // two were solving each other. A body one point too tall took a
                // bar, the bar narrowed it, the narrower body wrapped taller
                // still, and the surface measured from it crept a point at a
                // time for a dozen frames before anything settled. With the
                // gutter always spent, no body ever re-wraps because a bar
                // arrived, and the surface reaches its content in one
                // re-measure. A floating scrollbar style allocates nothing and
                // is left with nothing withheld, which is equally stable.
                //
                // The manual-scroll path is handed the full content box: it
                // hosts no bar of its own, so its width never answered to its
                // height in the first place, and the region inside it that does
                // scroll is its owner's to lay out.
                if self.overlay_scrollbar {
                    ui.spacing_mut().scroll = egui::style::ScrollStyle::floating();
                }
                let body_width = (content.x - ui.spacing().scroll.allocated_width()).max(1.0);
                // The body is painted once, by the surface beneath it. Its
                // frame used to repeat the surface's fill over a rect only as
                // wide as its content; at full opacity the two coincide, but
                // the modal is drawn below full opacity while it fades in, and
                // there the doubled rect composites lighter — a band behind
                // the text as wide as the text.
                if self.manual_body_scroll {
                    let body_output = ui.allocate_ui_with_layout(
                        vec2(ui.available_width(), body_max_height),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            Frame::NONE
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
                    let body_scroll = if let Some(requested_scroll_offset) = requested_scroll_offset
                    {
                        body_scroll.vertical_scroll_offset(requested_scroll_offset)
                    } else {
                        body_scroll
                    };
                    let body_output = body_scroll.show(ui, |ui| {
                        ui.set_width(body_width);
                        // The viewport's bottom edge, while the surface rather
                        // than the content decides it.
                        let floor = fill_surface_height.then(|| ui.max_rect().bottom());
                        ui.data_mut(|data| data.insert_temp(body_fill_floor_id(), floor));
                        Frame::NONE
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
                    ui.data_mut(|data| data.insert_temp(body_fill_floor_id(), None::<f32>));
                    unshown_body_height =
                        (body_output.content_size.y - body_output.inner_rect.height()).max(0.0);
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
                    self.transaction_strip(ui, &t, content.x);
                });
                let footer = ui
                    .scope_builder(egui::UiBuilder::new().id(id.with("footer")), |ui| {
                        self.footer(ui, &t, hide_close_only_footer, large_targets, content.x)
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
            // The surface is as tall as the stack it holds, plus whatever of
            // that stack the body could not show.
            //
            // The frame reports its outer rect with the stroke already counted
            // in, so the border allowance must not be added a second time. It
            // was, and a body that overflows spends every point the surface
            // offers, so every pass measured a border more than the last and
            // handed it to the next: the dialog grew two points a frame until
            // it reached its ceiling and nothing on it ever stopped moving.
            //
            // Counting the border once alone would freeze an overflowing
            // surface at whatever height it happened to be, which is a
            // different defect: content that arrived after the surface settled
            // would be reachable only by scrolling, and a surface one point
            // short of its content would stay one point short forever. Adding
            // the unshown remainder back instead resolves the surface to the
            // height its content actually asked for, and reaches it in a single
            // re-measure — the body's height no longer moves when the scrollbar
            // does, so the height solved for is the height the next pass lays
            // out. Content past the size's ceiling is clamped below and scrolls,
            // as it must.
            rendered_surface_height =
                Some(surface_output.response.rect.height() + unshown_body_height);
        });

        if self.fixed_height.is_none()
            && !layout.fill_height
            && let Some(height) = rendered_surface_height
        {
            // The same ceiling `resolve_with_width` applies, for the same
            // reason: `max_height` is how tall this surface may grow inside a
            // desktop window, and at narrow the viewport is the only ceiling
            // that means anything. Clamping the measurement to the desktop
            // number instead made a 680 point help centre scroll inside a
            // 900 point portrait that had room for all of it.
            let viewport = screen.height().max(1.0);
            let maximum = if layout.narrow {
                viewport
            } else {
                self.size.spec().max_height.min(viewport)
            };
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
                .filter(|_| self.interaction_enabled)
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

        // The intercepted Escape follows all input rendered above, including
        // Enter or a button activation that may already have chosen an action.
        let accept_cancel = choice == DialogChoice::None
            && self.interaction_enabled
            && is_top_modal
            && !Popup::is_any_open(ctx);
        if keyboard::cancel(ctx, modal_layer, accept_cancel) {
            choice = DialogChoice::Cancelled;
        }

        let mut cancel_focus = None;
        if choice != DialogChoice::None {
            // Re-measure content-height surfaces every time they are opened.
            // Keeping a previous session's height could otherwise force a
            // later workflow state into an unnecessarily short scroll area.
            ctx.data_mut(|data| data.remove_temp::<f32>(measured_height_id));
            if matches!(choice, DialogChoice::Ghost | DialogChoice::Cancelled) {
                cancel_focus = Some(DialogCancelFocus {
                    ctx: ctx.clone(),
                    state_id: focus_state_id,
                    container_id: focus_id,
                    layer: modal_layer,
                    rendered: rendered_focus,
                    retained_target: None,
                });
            } else {
                keyboard::unregister(ctx, modal_layer);
                restore_dialog_focus(ctx, focus_state_id, focus_id, modal_layer);
            }
        }

        DialogResponse {
            choice,
            cancel_focus,
        }
    }

    /// Header strip; returns `true` when the close control fired.
    fn header(&self, ui: &mut Ui, t: &Tokens, large_targets: Option<f32>) -> DialogHeaderOutput {
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
                    vec2(header_width, HEADER_MIN_HEIGHT),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        // State the row's floor as a floor.
                        //
                        // The height requested above is only a request: egui
                        // advances the parent by the row's own `min_rect`, and
                        // the row reaches the requested height solely because a
                        // centre cross-alignment fills the track it was offered.
                        // An `Area` lays its first frame out as a sizing pass —
                        // invisible, and with every centre cross-alignment
                        // rewritten to `Align::Min` so widgets report the least
                        // they can live in — and under those rules this row
                        // collapses onto its title.
                        // The surface measured from it is short by the same
                        // amount, and it is that measurement the first *visible*
                        // frame is laid out against, so every dialog opened with
                        // a header jumped once more after the reader could
                        // already see it. A floor stated as a floor holds in both
                        // passes, and the sizing pass measures what the visible
                        // pass will draw.
                        ui.set_min_height(HEADER_MIN_HEIGHT);
                        ui.spacing_mut().item_spacing.x = 10.0;
                        let close_size = if let Some(side) = large_targets {
                            vec2(side, side)
                        } else {
                            vec2(DIALOG_CLOSE_TARGET_WIDTH, DIALOG_CLOSE_TARGET_HEIGHT)
                        };
                        let text_width = (ui.available_width() - close_size.x - 10.0).max(1.0);
                        let mut title_job = egui::text::LayoutJob::default();
                        title_job.append(
                            self.title.as_str(),
                            0.0,
                            egui::TextFormat {
                                font_id: theme::sans(tokens::FS_3, FontWeight::SemiBold),
                                color: c.text,
                                ..Default::default()
                            },
                        );
                        // The title is centred by its own laid-out height,
                        // wrapped or not.
                        let mut measured = title_job.clone();
                        measured.wrap.max_width = text_width;
                        let title_height =
                            ui.fonts_mut(|fonts| fonts.layout_job(measured)).size().y;
                        ui.allocate_ui_with_layout(
                            // A zero-height child is aligned by its empty
                            // rectangle and then grows downward, which pushed
                            // the title below the row's centre and enlarged the
                            // surface. Give the title its real height; a
                            // wrapped narrow title grows it.
                            vec2(text_width, title_height),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.set_width(text_width);
                                ui.add(egui::Label::new(title_job).wrap());
                            },
                        );
                        let close_response =
                            crate::widgets::IconButton::new(crate::icons::Icon::Close)
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
        large_targets: Option<f32>,
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
            return large_targets.map_or(48.0, |side| {
                side + 2.0 * f32::from(TOUCH_FOOTER_VERTICAL_MARGIN)
            });
        }

        let action_count =
            1 + usize::from(self.secondary.is_some()) + usize::from(self.ghost.is_some());
        let action_height = large_targets.unwrap_or(29.0);
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
        // A split footer reads as two groups only with room between them; on
        // a phone-width surface it stacks instead, primary first.
        if self.secondary_leading && surface_width < SPLIT_FOOTER_MIN_WIDTH {
            return true;
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
        let error = transaction.tone == DialogTransactionTone::Error;
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
        // The words start on the body's left edge. An error steps them in by
        // the problem line's advance and sets the failure cross on the title's
        // line, as a problem line does; nothing else on the strip is coloured.
        strip_ui.spacing_mut().item_spacing.x = 0.0;
        if error {
            strip_ui.add_space(PROBLEM_MARK_ADVANCE);
        }
        let title_rect = strip_ui
            .vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 1.0;
                let title = ui.label(
                    egui::RichText::new(transaction.title.as_str())
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.label(
                    egui::RichText::new(transaction.detail.as_str())
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                title.rect
            })
            .inner;
        if error {
            paint_problem_mark(
                strip_ui.painter(),
                Rect::from_center_size(
                    egui::pos2(
                        content_rect.left() + PROBLEM_MARK_SIDE * 0.5,
                        title_rect.center().y,
                    ),
                    egui::Vec2::splat(PROBLEM_MARK_SIDE),
                ),
                Problem::Failure,
            );
        }
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(if error {
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
        large_targets: Option<f32>,
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
        let vertical_margin = if large_targets.is_some() {
            TOUCH_FOOTER_VERTICAL_MARGIN
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
                        let tone = self.hint_tone;
                        let color = tone.text_color(&c);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = HINT_MARK_GAP;
                            tone.mark(ui);
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(hint)
                                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                        .color(color),
                                )
                                .wrap(),
                            );
                        });
                    }
                    let action_width = ui.available_width();
                    let action_height = large_targets.unwrap_or(29.0);
                    let primary = crate::widgets::Button::new(&self.primary)
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
                        let secondary = crate::widgets::Button::new(label)
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
                        let ghost = crate::widgets::Button::new(label)
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
                    if self.secondary_leading
                        && let Some(label) = self.secondary.as_deref()
                    {
                        let secondary = crate::widgets::Button::new(label)
                            .enabled(self.secondary_enabled)
                            .show(ui);
                        secondary_id = self.secondary_enabled.then_some(secondary.id);
                        if secondary.clicked() {
                            choice = DialogChoice::Secondary;
                        }
                    }
                    // The component editor's hint keeps the track it has always
                    // had, left of its three actions: the unapplied-changes
                    // notice in dim text, or a session error behind the
                    // failure mark. It starts at the footer's left edge, as
                    // every other dialog's hint does.
                    if component_editor && let Some(hint) = self.hint.as_deref() {
                        let track = (ui.available_width() - 190.0).max(40.0);
                        ui.allocate_ui_with_layout(
                            vec2(track, 18.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                // The child advances its parent by what it
                                // holds, not by the track it was offered.
                                ui.set_min_width(track);
                                ui.spacing_mut().item_spacing.x = HINT_MARK_GAP;
                                self.hint_tone.mark(ui);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(hint)
                                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                            .color(c.text_dim),
                                    )
                                    .truncate(),
                                )
                                .on_hover_text(hint);
                            },
                        );
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let primary = crate::widgets::Button::new(&self.primary)
                            .accent()
                            .destructive(self.destructive)
                            .enabled(self.primary_enabled)
                            .show(ui);
                        primary_id = self.primary_enabled.then_some(primary.id);
                        if primary.clicked() {
                            choice = DialogChoice::Primary;
                        }
                        if !self.secondary_leading
                            && let Some(label) = self.secondary.as_deref()
                        {
                            let secondary = crate::widgets::Button::new(label)
                                .enabled(self.secondary_enabled)
                                .show(ui);
                            secondary_id = self.secondary_enabled.then_some(secondary.id);
                            if secondary.clicked() {
                                choice = DialogChoice::Secondary;
                            }
                        }
                        if let Some(label) = self.ghost.as_deref() {
                            let button =
                                crate::widgets::Button::new(label).enabled(self.ghost_enabled);
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
                        // Every other dialog's hint takes the mark it declared
                        // and the track the actions left, measured rather than
                        // guessed: laid out after them, it is exactly as wide
                        // as what remains, so a long consequence sentence
                        // elides (and states itself whole on hover) instead of
                        // pushing the primary off the footer, and a short one
                        // starts at the footer's left edge behind its mark
                        // instead of floating in the middle.
                        if !component_editor && let Some(hint) = self.hint.as_deref() {
                            self.hint_tone.hint(ui, hint, self.hint_tone.text_color(&c));
                        }
                    });
                });
            });
        // The rule goes on after the footer's fill. Painted first, the fill
        // covered its lower half, so it showed or vanished depending on where
        // the surface's height left it against the pixel grid.
        ui.painter()
            .hline(footer_rect.x_range(), line_y, Stroke::new(1.0, c.border));
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

fn body_fill_floor_id() -> egui::Id {
    egui::Id::new("rspice.dialog.body-fill-floor")
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
    if let Some(prior) = restorable {
        keyboard::resume_focus(ctx, prior);
    }
}

#[cfg(test)]
mod tests;
