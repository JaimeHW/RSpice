//! F6 and Shift+F6: the keyboard's way from one pane of the workbench to the
//! next.
//!
//! The panes are visited as they read on screen, left to right with the bottom
//! dock under the document it sits beneath, which is the order VS Code gives
//! its next-part and previous-part commands. A pane that is not drawn is passed
//! over, as is one with nothing in it to focus. Focus lands on the pane's first
//! control, or on the document's own editor or canvas when the document has
//! one, because that is what the reader came back to the document for. A
//! document with several, as Results' strips and panes are, is stepped
//! through one at a time before the keys go on to the next pane.

use egui::{Context, Id, LayerId, Rect, Response};

/// One press of the pane-cycling keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneStep {
    Next,
    Previous,
}

/// The panes of the workbench, in the order the keys visit them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    ActivityRail,
    Navigator,
    Document,
    BottomDock,
    Inspector,
}

const PANE_ORDER: [Pane; 5] = [
    Pane::ActivityRail,
    Pane::Navigator,
    Pane::Document,
    Pane::BottomDock,
    Pane::Inspector,
];

#[derive(Debug, Clone, Copy)]
struct DrawnPane {
    pane: Pane,
    rect: Rect,
    layer: LayerId,
}

/// Where the panes were drawn this frame, and the step waiting to be taken.
#[derive(Debug, Clone, Default)]
pub struct PaneFocus {
    drawn: Vec<DrawnPane>,
    pending: Option<PaneStep>,
}

impl PaneFocus {
    /// Forget the last frame's panes. A pane is only ever a target in the
    /// frame that drew it.
    pub fn begin_frame(&mut self) {
        self.drawn.clear();
    }

    /// Note where `pane` was drawn. Only the main window's panes count: a
    /// detached window draws the same docks, and the keys answer in the window
    /// that holds them.
    pub fn record(&mut self, ctx: &Context, pane: Pane, shown: &Response) {
        if ctx.viewport_id() == egui::ViewportId::ROOT {
            self.drawn.push(DrawnPane {
                pane,
                rect: shown.rect,
                layer: shown.layer_id,
            });
        }
    }

    /// Where `pane` was drawn this frame.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn drawn_rect(&self, pane: Pane) -> Option<Rect> {
        self.drawn
            .iter()
            .find(|drawn| drawn.pane == pane)
            .map(|drawn| drawn.rect)
    }

    pub fn request(&mut self, step: PaneStep) {
        self.pending = Some(step);
    }

    /// Take the waiting step. Runs after every pane has drawn, so the target
    /// is chosen from the controls actually on screen this frame.
    /// `document_stops` are the document's own editors or canvases in the
    /// order the keys visit them; none when it has none.
    pub fn apply(&mut self, ctx: &Context, document_stops: &[Id]) {
        let Some(step) = self.pending.take() else {
            return;
        };
        if let Some(target) = self.target(ctx, step, document_stops) {
            ctx.memory_mut(|memory| memory.request_focus(target));
            ctx.request_repaint();
        }
    }

    fn target(&self, ctx: &Context, step: PaneStep, document_stops: &[Id]) -> Option<Id> {
        let panes: Vec<DrawnPane> = PANE_ORDER
            .iter()
            .filter_map(|pane| self.drawn.iter().find(|drawn| drawn.pane == *pane).copied())
            .collect();
        if panes.is_empty() {
            return None;
        }
        let focused = ctx.memory(|memory| memory.focused());
        // On one of the document's stops, the keys take the next stop first.
        if let Some(document) = panes.iter().find(|drawn| drawn.pane == Pane::Document)
            && let Some(at) =
                focused.and_then(|id| document_stops.iter().position(|stop| *stop == id))
        {
            let further = match step {
                PaneStep::Next => &document_stops[at + 1..],
                PaneStep::Previous => &document_stops[..at],
            };
            let mut further = further.iter();
            let next = match step {
                PaneStep::Next => further.find(|stop| Self::usable(ctx, *document, **stop)),
                PaneStep::Previous => further.rfind(|stop| Self::usable(ctx, *document, **stop)),
            };
            if let Some(next) = next {
                return Some(*next);
            }
        }
        let current = focused.and_then(|id| Self::pane_holding(ctx, &panes, id));
        let count = panes.len();
        // With the keyboard nowhere in the panes, the first press goes to the
        // document whichever way it was meant to run.
        let start = current.unwrap_or_else(|| {
            let document = panes
                .iter()
                .position(|drawn| drawn.pane == Pane::Document)
                .unwrap_or(0);
            match step {
                PaneStep::Next => (document + count - 1) % count,
                PaneStep::Previous => (document + 1) % count,
            }
        });
        // Every other pane in turn: one with nothing to focus is passed over.
        (1..count)
            .map(|offset| match step {
                PaneStep::Next => (start + offset) % count,
                PaneStep::Previous => (start + count - offset) % count,
            })
            .chain(current.is_none().then_some(start))
            .find_map(|index| Self::landing_control(ctx, panes[index], step, document_stops))
    }

    /// Whether `id` is a control of `drawn` the keys can land on.
    fn usable(ctx: &Context, drawn: DrawnPane, id: Id) -> bool {
        ctx.viewport(|viewport| {
            viewport
                .this_pass
                .widgets
                .get(id)
                .is_some_and(|widget| Self::lands_in(drawn, widget))
        })
    }

    fn lands_in(drawn: DrawnPane, widget: &egui::WidgetRect) -> bool {
        widget.enabled
            && widget.sense.is_focusable()
            && widget.interact_rect.is_positive()
            // A splitter straddles the pane's edge; it is not inside.
            && drawn.rect.contains_rect(widget.interact_rect)
    }

    fn pane_holding(ctx: &Context, panes: &[DrawnPane], id: Id) -> Option<usize> {
        let widget = ctx.viewport(|viewport| viewport.this_pass.widgets.get(id).copied())?;
        panes.iter().position(|drawn| {
            drawn.layer == widget.layer_id && drawn.rect.contains(widget.rect.center())
        })
    }

    /// Where a step into `drawn` lands: the document's first stop going on,
    /// its last coming back, or else the pane's first control.
    fn landing_control(
        ctx: &Context,
        drawn: DrawnPane,
        step: PaneStep,
        document_stops: &[Id],
    ) -> Option<Id> {
        ctx.viewport(|viewport| {
            let widgets = &viewport.this_pass.widgets;
            let usable = |widget: &&egui::WidgetRect| Self::lands_in(drawn, widget);
            let mut stops = document_stops
                .iter()
                .filter(|_| drawn.pane == Pane::Document)
                .filter_map(|id| widgets.get(*id));
            let stop = match step {
                PaneStep::Next => stops.find(usable),
                PaneStep::Previous => stops.rfind(usable),
            };
            stop.or_else(|| widgets.get_layer(drawn.layer).find(usable))
                .map(|widget| widget.id)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN: egui::Vec2 = egui::vec2(800.0, 600.0);

    /// Two side panels, the first with `first_has_a_control`, and whatever
    /// focus results after `steps` presses of the key.
    fn run(first_has_a_control: bool, steps: &[PaneStep]) -> Option<&'static str> {
        let ctx = Context::default();
        let mut panes = PaneFocus::default();
        let mut landed = None;
        for step in steps.iter().map(Some).chain([None, None]) {
            if let Some(step) = step {
                panes.request(*step);
            }
            let raw = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
                ..Default::default()
            };
            let mut ids = [None, None];
            let _ = ctx.run_ui(raw, |ui| {
                panes.begin_frame();
                let left = egui::Panel::left("left").show(ui, |ui| {
                    if first_has_a_control {
                        ids[0] = Some(ui.button("left").id);
                    } else {
                        ui.label("nothing to press");
                    }
                });
                panes.record(ui.ctx(), Pane::ActivityRail, &left.response);
                let middle = egui::CentralPanel::default().show(ui, |ui| {
                    ids[1] = Some(ui.button("middle").id);
                });
                panes.record(ui.ctx(), Pane::Document, &middle.response);
                panes.apply(ui.ctx(), &[]);
            });
            let focused = ctx.memory(|memory| memory.focused());
            landed = if focused.is_some() && focused == ids[0] {
                Some("left")
            } else if focused.is_some() && focused == ids[1] {
                Some("middle")
            } else {
                None
            };
        }
        landed
    }

    #[test]
    fn the_first_press_goes_to_the_document_whichever_way_it_runs() {
        assert_eq!(run(true, &[PaneStep::Next]), Some("middle"));
        assert_eq!(run(true, &[PaneStep::Previous]), Some("middle"));
    }

    #[test]
    fn the_next_press_moves_on_and_the_last_wraps() {
        assert_eq!(run(true, &[PaneStep::Next, PaneStep::Next]), Some("left"));
        assert_eq!(
            run(true, &[PaneStep::Next, PaneStep::Next, PaneStep::Next]),
            Some("middle")
        );
        assert_eq!(
            run(true, &[PaneStep::Next, PaneStep::Previous]),
            Some("left")
        );
    }

    #[test]
    fn a_pane_with_nothing_to_focus_is_passed_over() {
        assert_eq!(
            run(false, &[PaneStep::Next, PaneStep::Next]),
            Some("middle")
        );
    }
}
