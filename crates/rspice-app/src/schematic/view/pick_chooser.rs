//! Choosing among overlapping objects (D1 section 9). A plain click again on
//! the same spot takes the next object under it, and the hover shows which
//! one that will be; a long click on a spot two or more objects claim opens
//! a chooser that lists them in pick order.

use super::{AppState, SchematicSymbolContext, viewport::Viewport};
use crate::state::Point;
use egui::{Context, Id, Pos2, Response};
use rspice_design::connectivity::summary::projection_nets;
use rspice_schematic_editor::requests::{EditorAction, EditorRequest};
use rspice_schematic_editor::view::{
    pick_chooser::{ChooserOutcome, ChooserRow, PickChooser},
    pointer_target::{self, PointerQuery, PointerTarget},
    tool_input::ToolPointerView,
};

/// A repeat click, and a long click, stay within this many points of where
/// they started.
const SAME_SPOT: f32 = 4.0;
/// A press held this long is a long click.
const LONG_CLICK_SECONDS: f64 = 0.5;

/// The last plain click's candidates and which of them it took.
#[derive(Debug, Clone, PartialEq)]
struct PickCycle {
    anchor: Pos2,
    time: f64,
    candidates: Vec<PointerTarget>,
    index: usize,
}

/// A primary press on the canvas that may become a long click.
#[derive(Debug, Clone, Copy, PartialEq)]
struct LongPress {
    origin: Pos2,
    start: f64,
    strayed: bool,
}

fn cycle_id() -> Id {
    Id::new("rspice.schematic.pick_cycle")
}

fn press_id() -> Id {
    Id::new("rspice.schematic.long_press")
}

fn chooser_id() -> Id {
    Id::new("rspice.schematic.pick_chooser.state")
}

/// Every object under `query`, in pick order.
fn candidates_at(
    ctx: &Context,
    state: &AppState,
    query: PointerQuery,
    symbol_context: &SchematicSymbolContext,
    viewport: &Viewport,
) -> Vec<PointerTarget> {
    pointer_target::pointer_targets_all(
        &super::schematic_design_view(state),
        query,
        state.ui.schematic_selection_filter,
        symbol_context,
        ctx,
        viewport,
        || state.workspace.content.active_view.display_path(),
    )
}

/// Which candidate a plain click takes: the next one when it repeats the
/// last plain click on the same spot over the same candidates, after the
/// double-click interval; else the first.
fn cycle_index(
    previous: Option<&PickCycle>,
    position: Pos2,
    time: f64,
    double_click_delay: f64,
    candidates: &[PointerTarget],
) -> usize {
    match previous {
        Some(last)
            if !candidates.is_empty()
                && last.anchor.distance(position) <= SAME_SPOT
                && time - last.time > double_click_delay
                && last.candidates == candidates =>
        {
            (last.index + 1) % candidates.len()
        }
        _ => 0,
    }
}

/// What a click takes. An additive click takes the first object and ends
/// any cycle; a plain click again on the same spot takes the next object.
pub(super) fn click_target(
    ctx: &Context,
    state: &AppState,
    query: PointerQuery,
    symbol_context: &SchematicSymbolContext,
    viewport: &Viewport,
    additive: bool,
) -> Option<PointerTarget> {
    let candidates = candidates_at(ctx, state, query, symbol_context, viewport);
    if additive || candidates.is_empty() {
        ctx.data_mut(|data| data.remove::<PickCycle>(cycle_id()));
        return candidates.first().copied();
    }
    let time = ctx.input(|input| input.time);
    let delay = ctx.options(|options| options.input_options.max_double_click_delay);
    let previous = ctx.data(|data| data.get_temp::<PickCycle>(cycle_id()));
    let index = cycle_index(previous.as_ref(), query.position, time, delay, &candidates);
    let target = candidates[index];
    ctx.data_mut(|data| {
        data.insert_temp(
            cycle_id(),
            PickCycle {
                anchor: query.position,
                time,
                candidates,
                index,
            },
        );
    });
    Some(target)
}

/// What the next plain click at the pointer would take when it would cycle,
/// for the hover. A pointer that strays from the last click ends the cycle.
pub(super) fn cycle_hover(ctx: &Context) -> Option<PointerTarget> {
    let cycle = ctx.data(|data| data.get_temp::<PickCycle>(cycle_id()))?;
    let near = ctx
        .pointer_latest_pos()
        .is_some_and(|position| position.distance(cycle.anchor) <= SAME_SPOT);
    if !near {
        ctx.data_mut(|data| data.remove::<PickCycle>(cycle_id()));
        return None;
    }
    if cycle.candidates.len() < 2 || ctx.input(|input| !input.modifiers.is_none()) {
        return None;
    }
    cycle
        .candidates
        .get((cycle.index + 1) % cycle.candidates.len())
        .copied()
}

/// Watch the primary button on the canvas. On the release of a press held
/// at least 500 ms that never strayed 4 pt, return where it was released.
/// Touch has its own long press (the context menu), so it is not watched.
pub(super) fn observe_long_click(response: &Response) -> Option<Pos2> {
    let ctx = &response.ctx;
    let (pressed, released, down, position, time) = ctx.input(|input| {
        (
            input.pointer.primary_pressed(),
            input.pointer.primary_released(),
            input.pointer.primary_down(),
            input.pointer.latest_pos(),
            input.time,
        )
    });
    if pressed {
        let press = (response.hovered() && !pointer_target::canvas_touch_mode(ctx))
            .then_some(position)
            .flatten()
            .map(|origin| LongPress {
                origin,
                start: time,
                strayed: false,
            });
        ctx.data_mut(|data| match press {
            Some(press) => {
                data.insert_temp(press_id(), press);
            }
            None => data.remove::<LongPress>(press_id()),
        });
        return None;
    }
    let mut press = ctx.data(|data| data.get_temp::<LongPress>(press_id()))?;
    let strayed = position.is_none_or(|position| position.distance(press.origin) > SAME_SPOT);
    if down {
        if strayed && !press.strayed {
            press.strayed = true;
            ctx.data_mut(|data| data.insert_temp(press_id(), press));
        }
        return None;
    }
    ctx.data_mut(|data| data.remove::<LongPress>(press_id()));
    (released && !press.strayed && !strayed && time - press.start >= LONG_CLICK_SECONDS)
        .then_some(position)
        .flatten()
}

fn closed_id() -> Id {
    Id::new("rspice.schematic.pick_chooser.closed_pass")
}

/// The open chooser, if any.
pub(super) fn open_chooser(ctx: &Context) -> Option<PickChooser> {
    ctx.data(|data| data.get_temp::<PickChooser>(chooser_id()))
}

/// Whether a canvas click this frame belongs to the chooser: it is open (the
/// long click that opened it), or it closed this frame (a click outside
/// closes it with no change).
pub(super) fn chooser_takes_click(ctx: &Context) -> bool {
    open_chooser(ctx).is_some()
        || ctx.data(|data| data.get_temp::<u64>(closed_id())) == Some(ctx.cumulative_pass_nr())
}

/// Open the chooser over the objects a long click at `position` hits, when
/// two or more do.
pub(super) fn open_chooser_at(
    ctx: &Context,
    state: &AppState,
    position: Pos2,
    symbol_context: &SchematicSymbolContext,
    viewport: &Viewport,
) {
    let query = ToolPointerView {
        design: super::schematic_design_view(state),
        snap_engine: &state.schematic.session.editor.snap_engine,
        can_edit: !state.schematic_edit_read_only(),
    }
    .selection_query(viewport, position);
    let candidates = candidates_at(ctx, state, query, symbol_context, viewport);
    if candidates.len() < 2 {
        return;
    }
    let rows = chooser_rows(state, &candidates);
    if let Some(chooser) = PickChooser::open(position, rows, pointer_target::canvas_touch_mode(ctx))
    {
        ctx.data_mut(|data| {
            data.remove::<PickCycle>(cycle_id());
            data.insert_temp(chooser_id(), chooser);
        });
    }
}

/// Show the open chooser for this frame and apply what the person chose.
/// Runs before the canvas tools, so its keys (arrows, Enter, Esc) never
/// reach them.
pub(super) fn show_chooser(ctx: &Context, state: &mut AppState) {
    let Some(mut chooser) = open_chooser(ctx) else {
        return;
    };
    let outcome = chooser.show(ctx);
    let (target, additive) = match outcome {
        None => {
            ctx.data_mut(|data| data.insert_temp(chooser_id(), chooser));
            return;
        }
        Some(ChooserOutcome::Dismiss) => (None, false),
        Some(ChooserOutcome::Replace(target)) => (Some(target), false),
        Some(ChooserOutcome::Toggle(target)) => (Some(target), true),
    };
    let pass = ctx.cumulative_pass_nr();
    ctx.data_mut(|data| {
        data.remove::<PickChooser>(chooser_id());
        data.insert_temp(closed_id(), pass);
    });
    if let Some(target) = target {
        let request = EditorRequest {
            source: super::requests::editor_request_source(state),
            selection: state.schematic.session.editor.selection.clone(),
            action: EditorAction::SelectPointer {
                target: Some(target),
                additive,
                alt_held: false,
            },
        };
        super::requests::apply_editor_request(state, request);
    }
}

/// The chooser's rows: each candidate's kind and identity.
fn chooser_rows(state: &AppState, candidates: &[PointerTarget]) -> Vec<ChooserRow> {
    let document = state.schematic.document();
    let nets = candidates
        .iter()
        .any(|target| matches!(target, PointerTarget::Wire(_)))
        .then(|| {
            state
                .workspace
                .design_projection(
                    &state.library_manager,
                    &state.workspace.content.active_view,
                    &state.schematic,
                )
                .ok()
                .map(|projection| {
                    projection_nets(
                        state.library_manager.catalog(),
                        &projection,
                        &state.workspace.content.active_view.key(),
                    )
                })
        })
        .flatten();
    candidates
        .iter()
        .map(|&target| {
            let identity = match target {
                PointerTarget::Component(id) => document
                    .components
                    .iter()
                    .find(|component| component.id == id)
                    .map(|component| component.name.clone())
                    .filter(|name| !name.is_empty())
                    .unwrap_or_else(|| format!("instance {id}")),
                PointerTarget::Wire(id) => {
                    let net = nets.as_ref().and_then(|nets| {
                        nets.iter()
                            .find(|net| net.wire_ids.contains(&id))
                            .map(|net| net.name.clone())
                    });
                    match net {
                        Some(net) => format!("wire {id} · {net}"),
                        None => format!("wire {id}"),
                    }
                }
                PointerTarget::Bus(id) => format!("bus {id}"),
                PointerTarget::BusTap(id) => format!("tap {id}"),
                PointerTarget::Junction(Point { x, y }) => format!("({x}, {y})"),
                PointerTarget::NetLabel(id) => document
                    .net_labels
                    .iter()
                    .find(|label| label.id == id)
                    .map_or_else(|| format!("label {id}"), |label| label.name.clone()),
                PointerTarget::DesignNote(id) => format!("note {id}"),
                PointerTarget::DocumentationShape(id) => format!("shape {id}"),
                PointerTarget::Probe(id) => document
                    .probes
                    .iter()
                    .find(|probe| probe.id == id)
                    .map_or_else(|| format!("P{id}"), |probe| probe.reference.clone()),
            };
            ChooserRow { target, identity }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cycle(index: usize) -> PickCycle {
        PickCycle {
            anchor: Pos2::new(100.0, 100.0),
            time: 1.0,
            candidates: vec![
                PointerTarget::Component(2),
                PointerTarget::Component(1),
                PointerTarget::Wire(7),
            ],
            index,
        }
    }

    /// A plain click again on the same spot, after the double-click
    /// interval, takes the next candidate and wraps; a double click, a
    /// click elsewhere, or different candidates start again at the first.
    #[test]
    fn a_repeat_click_on_the_same_spot_cycles_the_candidates() {
        let candidates = cycle(0).candidates;
        let here = Pos2::new(102.0, 103.0);
        assert_eq!(cycle_index(None, here, 2.0, 0.3, &candidates), 0);
        assert_eq!(cycle_index(Some(&cycle(0)), here, 2.0, 0.3, &candidates), 1);
        assert_eq!(cycle_index(Some(&cycle(2)), here, 2.0, 0.3, &candidates), 0);
        assert_eq!(
            cycle_index(Some(&cycle(0)), here, 1.2, 0.3, &candidates),
            0,
            "the second click of a double click does not cycle"
        );
        assert_eq!(
            cycle_index(
                Some(&cycle(0)),
                Pos2::new(105.0, 100.0),
                2.0,
                0.3,
                &candidates
            ),
            0,
            "5 pt away is another spot"
        );
        assert_eq!(
            cycle_index(Some(&cycle(0)), here, 2.0, 0.3, &candidates[..2]),
            0,
            "other candidates start again"
        );
    }

    /// One frame of a 400-pt canvas at `time` with `events`, through the
    /// long-click watcher.
    fn frame(ctx: &Context, time: f64, events: Vec<egui::Event>) -> Option<Pos2> {
        let mut released = None;
        let _ = ctx.run_ui(
            egui::RawInput {
                time: Some(time),
                events,
                ..Default::default()
            },
            |ui| {
                let response = ui.interact(
                    egui::Rect::from_min_size(Pos2::ZERO, egui::vec2(400.0, 400.0)),
                    Id::new("canvas"),
                    egui::Sense::click_and_drag(),
                );
                released = observe_long_click(&response);
            },
        );
        released
    }

    fn button(position: Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    /// A press held for `held` seconds, the pointer moving `drift` points
    /// while it is down: what the watcher reports on release.
    fn long_click(held: f64, drift: f32) -> Option<Pos2> {
        let ctx = Context::default();
        let here = Pos2::new(100.0, 100.0);
        frame(&ctx, 0.0, vec![egui::Event::PointerMoved(here)]);
        frame(&ctx, 0.1, vec![button(here, true)]);
        let moved = here + egui::vec2(drift, 0.0);
        frame(
            &ctx,
            0.1 + held / 2.0,
            vec![egui::Event::PointerMoved(moved)],
        );
        frame(
            &ctx,
            0.1 + held,
            vec![egui::Event::PointerMoved(here), button(here, false)],
        )
    }

    /// A press held 500 ms or more that stays within 4 pt is a long click; a
    /// short press, or one that strays and comes back, is not.
    #[test]
    fn a_long_click_is_a_still_press_held_half_a_second() {
        assert_eq!(long_click(0.6, 2.0), Some(Pos2::new(100.0, 100.0)));
        assert_eq!(long_click(1.5, 0.0), Some(Pos2::new(100.0, 100.0)));
        assert_eq!(long_click(0.3, 0.0), None, "a short press is a click");
        assert_eq!(
            long_click(0.6, 6.0),
            None,
            "a press that strayed 6 pt is a drag"
        );
    }
}
