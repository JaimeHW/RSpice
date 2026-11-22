//! Notes on a strip's plot: words the reader writes at a place of the plot,
//! as ViVA's and ADS's plot text and EZwave's annotations are. A note stands
//! on a place in the strip's frame, on the point of a curve, or on a point
//! of the data. The plot's menu adds one where it was opened; the reader
//! writes it there, and edits it where it stands with a double click. A
//! drag moves it, and its own menu pins it to another kind of place or
//! deletes it.

use egui::{Key, Pos2, Rect, Ui, Vec2, pos2, vec2};
use rspice_results::result_presentation::AnalysisPresentationKey;
use rspice_ui_kit::{
    plot::{PlotResponse, SampleInterpolation, XScale},
    theme::{self, FontWeight},
    tokens::Tokens,
};

use super::{SHARED_X_SLOT, StripModel, StripTrace, UnitPane, anchor_key, pane::SecondaryPlot};
use crate::session::{DocumentNote, NoteAnchor, NoteDraft, NoteSelector, ResultViewerState};

/// What the reader did to a note, for the host to keep in its view.
#[derive(Debug, Clone, PartialEq)]
pub enum NoteEdit {
    /// A note written at `anchor` on the strip of `analysis`.
    Add {
        analysis: AnalysisPresentationKey,
        anchor: NoteAnchor,
        text: String,
    },
    /// A note moved, pinned to another kind of place, or rewritten.
    Set {
        note: NoteSelector,
        anchor: NoteAnchor,
        text: String,
    },
    Delete {
        note: NoteSelector,
    },
}

/// How a strip's plot was drawn this frame: where on screen, and over what.
pub(crate) struct NoteFrame<'a> {
    pub plot_rect: Rect,
    pub x: (f64, f64),
    pub x_scale: XScale,
    /// The first axis' range and scale.
    pub y: (f64, f64),
    pub y_scale: XScale,
    /// The axes after the first, each linear.
    pub secondary: &'a [SecondaryPlot<'a>],
    pub interpolation: SampleInterpolation,
    /// The plot's own id, which every note's widgets derive theirs from.
    pub id: egui::Id,
}

impl<'a> NoteFrame<'a> {
    pub(crate) fn of(
        response: &PlotResponse,
        model: &StripModel,
        log_y: bool,
        secondary: &'a [SecondaryPlot<'a>],
        interpolation: SampleInterpolation,
    ) -> Self {
        Self {
            plot_rect: response.plot_rect,
            x: response.axes.0,
            x_scale: model.x_scale,
            y: response.axes.1,
            y_scale: if log_y { XScale::Log10 } else { XScale::Linear },
            secondary,
            interpolation,
            id: response.response.id,
        }
    }

    fn screen_x(&self, x: f64) -> f32 {
        let rect = self.plot_rect;
        rect.left() + self.x_scale.normalize(x, self.x.0, self.x.1) as f32 * rect.width()
    }

    pub(crate) fn data_x(&self, screen: f32) -> f64 {
        let rect = self.plot_rect;
        let share = f64::from((screen - rect.left()) / rect.width().max(1.0));
        self.x_scale
            .denormalize(share.clamp(0.0, 1.0), self.x.0, self.x.1)
    }

    /// The range and scale of the axis of `pane` that reads `unit`.
    fn axis(&self, pane: &UnitPane, unit: &str) -> Option<((f64, f64), XScale)> {
        if unit == pane.unit {
            return Some((self.y, self.y_scale));
        }
        self.secondary
            .iter()
            .find(|axis| axis.unit == unit)
            .map(|axis| (axis.range, XScale::Linear))
    }

    fn screen_y(&self, (range, scale): ((f64, f64), XScale), y: f64) -> f32 {
        let rect = self.plot_rect;
        rect.bottom() - scale.normalize(y, range.0, range.1) as f32 * rect.height()
    }

    pub(crate) fn data_y(&self, (range, scale): ((f64, f64), XScale), screen: f32) -> f64 {
        let rect = self.plot_rect;
        let share = f64::from((rect.bottom() - screen) / rect.height().max(1.0));
        scale.denormalize(share.clamp(0.0, 1.0), range.0, range.1)
    }
}

/// The curve of `pane` that `anchor` names, with the unit of the axis it
/// reads on.
fn anchored_trace<'m>(
    model: &'m StripModel,
    pane: &'m UnitPane,
    anchor: &rspice_results::result_presentation::WaveformPresentationKey,
) -> Option<(&'m StripTrace, &'m str)> {
    std::iter::once((&*pane.traces, pane.unit))
        .chain(pane.secondary.iter().map(|axis| (&*axis.traces, axis.unit)))
        .flat_map(|(traces, unit)| traces.iter().map(move |index| (*index, unit)))
        .filter_map(|(index, unit)| model.traces.get(index).map(|trace| (trace, unit)))
        .find(|(trace, _)| !trace.overlay && anchor_key(model, trace) == *anchor)
}

/// Where `anchor` stands on the plot of `pane`; `None` when the note stands
/// on another of the strip's panes, or on a point this frame has no place
/// for.
fn point(
    model: &StripModel,
    pane: &UnitPane,
    frame: &NoteFrame<'_>,
    anchor: &NoteAnchor,
) -> Option<Pos2> {
    let rect = frame.plot_rect;
    let at = match anchor {
        NoteAnchor::Frame { unit, x, y } => {
            let here = unit
                .as_deref()
                .map_or(pane.slot == SHARED_X_SLOT, |unit| unit == pane.unit);
            if !here {
                return None;
            }
            pos2(
                rect.left() + x.clamp(0.0, 1.0) * rect.width(),
                rect.top() + y.clamp(0.0, 1.0) * rect.height(),
            )
        }
        NoteAnchor::Trace { anchor, x, .. } => {
            let (trace, unit) = anchored_trace(model, pane, anchor)?;
            let y = model.trace_value(trace, *x, frame.interpolation);
            pos2(
                frame.screen_x(*x),
                frame.screen_y(frame.axis(pane, unit)?, y),
            )
        }
        NoteAnchor::Data { unit, x, y } => pos2(
            frame.screen_x(*x),
            frame.screen_y(frame.axis(pane, unit)?, *y),
        ),
    };
    (at.x.is_finite() && at.y.is_finite()).then_some(at)
}

/// `anchor` of the same kind, standing at `at` on screen instead. A note on
/// a curve slides along it: only its X follows the pointer.
fn moved(pane: &UnitPane, frame: &NoteFrame<'_>, anchor: &NoteAnchor, at: Pos2) -> NoteAnchor {
    let rect = frame.plot_rect;
    match anchor {
        NoteAnchor::Frame { unit, .. } => NoteAnchor::Frame {
            unit: unit.clone(),
            x: ((at.x - rect.left()) / rect.width().max(1.0)).clamp(0.0, 1.0),
            y: ((at.y - rect.top()) / rect.height().max(1.0)).clamp(0.0, 1.0),
        },
        NoteAnchor::Trace {
            anchor, trace_name, ..
        } => NoteAnchor::Trace {
            anchor: anchor.clone(),
            trace_name: trace_name.clone(),
            x: frame.data_x(at.x),
        },
        NoteAnchor::Data { unit, x, y } => match frame.axis(pane, unit) {
            Some(axis) => NoteAnchor::Data {
                unit: unit.clone(),
                x: frame.data_x(at.x),
                y: frame.data_y(axis, at.y),
            },
            None => NoteAnchor::Data {
                unit: unit.clone(),
                x: *x,
                y: *y,
            },
        },
    }
}

/// The curve of `pane` drawn nearest `at` on screen, at its X.
fn nearest_curve<'m>(
    model: &'m StripModel,
    pane: &'m UnitPane,
    frame: &NoteFrame<'_>,
    at: Pos2,
) -> Option<&'m StripTrace> {
    let x = frame.data_x(at.x);
    std::iter::once((&*pane.traces, pane.unit))
        .chain(pane.secondary.iter().map(|axis| (&*axis.traces, axis.unit)))
        .flat_map(|(traces, unit)| traces.iter().map(move |index| (*index, unit)))
        .filter_map(|(index, unit)| {
            let trace = model.traces.get(index).filter(|trace| !trace.overlay)?;
            let y = model.trace_value(trace, x, frame.interpolation);
            let screen = frame.screen_y(frame.axis(pane, unit)?, y);
            screen.is_finite().then(|| (trace, (screen - at.y).abs()))
        })
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(trace, _)| trace)
}

/// The anchor a note added from the plot's menu at (`x`, `y`) stands on: the
/// point of the data clicked, or the point of `trace` at `x`.
pub(crate) fn new_anchor(
    model: &StripModel,
    pane: &UnitPane,
    (x, y): (f64, f64),
    trace: Option<&StripTrace>,
) -> NoteAnchor {
    match trace {
        Some(trace) => NoteAnchor::Trace {
            anchor: anchor_key(model, trace),
            trace_name: trace.name.clone(),
            x,
        },
        None => NoteAnchor::Data {
            unit: pane.unit.to_owned(),
            x,
            y,
        },
    }
}

/// Start writing a note where the plot's menu was opened: at `at` on the
/// first axis of `pane`, or on `trace` at its X.
pub(crate) fn begin(
    ui: &Ui,
    session: &mut ResultViewerState,
    model: &StripModel,
    pane: &UnitPane,
    at: (f64, f64),
    trace: Option<&StripTrace>,
) {
    start(
        ui,
        session,
        NoteDraft {
            analysis: model.analysis_key,
            note: None,
            anchor: new_anchor(model, pane, at, trace),
            text: String::new(),
        },
    );
}

/// Write `draft` in a field of its own where it stands, which takes the
/// keyboard when it first shows.
fn start(ui: &Ui, session: &mut ResultViewerState, draft: NoteDraft) {
    ui.data_mut(|data| data.remove::<bool>(field_id(draft.analysis).with("seeded")));
    session.note_draft = Some(draft);
}

/// The id of the field a note is written in on the strip of `analysis`;
/// one note is written at a time.
fn field_id(analysis: AnalysisPresentationKey) -> egui::Id {
    egui::Id::new(("rspice.results.note-field", analysis))
}

/// What a note's own menu was asked.
enum NoteChoice {
    Edit,
    Pin(NoteAnchor),
    Delete,
}

/// The room a note's words take inside its box, either side.
const PAD: Vec2 = vec2(6.0, 3.0);
/// How far a note's box stands from the point it is on.
const STAND_OFF: f32 = 8.0;
/// The least width the field a note is written in takes.
const FIELD_WIDTH: f32 = 160.0;

/// Where a note's box of `size` goes for a note standing at `at`: above and
/// right of a point, turned to whichever side keeps it inside the plot; with
/// its corner on a place in the frame.
fn note_box(rect: Rect, at: Pos2, size: Vec2, on_point: bool) -> Rect {
    let min = if on_point {
        let mut min = pos2(at.x + STAND_OFF, at.y - STAND_OFF - size.y);
        if min.x + size.x > rect.right() - 2.0 {
            min.x = at.x - STAND_OFF - size.x;
        }
        if min.y < rect.top() + 2.0 {
            min.y = at.y + STAND_OFF;
        }
        min
    } else {
        at
    };
    let x = min.x.clamp(
        rect.left() + 2.0,
        (rect.right() - size.x - 2.0).max(rect.left() + 2.0),
    );
    let y = min.y.clamp(
        rect.top() + 2.0,
        (rect.bottom() - size.y - 2.0).max(rect.top() + 2.0),
    );
    Rect::from_min_size(pos2(x, y), size)
}

/// Draw the notes of `pane`'s plot, drawn this frame as `frame` says, and
/// hand what the reader does to them to `host`, which keeps them in its view.
pub(crate) fn show(
    ui: &mut Ui,
    host: &mut impl super::stack::StackHost,
    model: &StripModel,
    pane: &UnitPane,
    frame: &NoteFrame<'_>,
) {
    let mut edits = Vec::new();
    paint(ui, host.session_mut(), model, pane, frame, &mut |edit| {
        edits.push(edit);
    });
    for edit in edits {
        host.edit_note(model, edit);
    }
}

fn paint(
    ui: &mut Ui,
    session: &mut ResultViewerState,
    model: &StripModel,
    pane: &UnitPane,
    frame: &NoteFrame<'_>,
    apply: &mut impl FnMut(NoteEdit),
) {
    let t = Tokens::get(ui.ctx());
    let c = &t.color;
    let font = theme::sans(11.0, FontWeight::Regular);
    let rect = frame.plot_rect;
    let painter = ui.painter().with_clip_rect(rect);
    let drafting = session
        .note_draft
        .as_ref()
        .filter(|draft| draft.analysis == model.analysis_key)
        .and_then(|draft| draft.note);
    let notes: Vec<DocumentNote> = session
        .strip_notes(model.analysis_key)
        .filter(|note| Some(note.selector()) != drafting)
        .cloned()
        .collect();
    for note in notes {
        let Some(at) = point(model, pane, frame, &note.anchor) else {
            continue;
        };
        let on_point = !matches!(note.anchor, NoteAnchor::Frame { .. });
        if on_point && !rect.expand(0.5).contains(at) {
            continue;
        }
        let galley = painter.layout_no_wrap(note.text.clone(), font.clone(), c.text);
        let size = galley.size() + 2.0 * PAD;
        let id = frame
            .id
            .with(("note", note.document_id, note.retained_id.get()));
        // Where the note stood when the frame began; a drag moves it from
        // there, and the box follows what the drag would make it.
        let placed = note_box(rect, at, size, on_point);
        let response = ui.interact(placed, id, egui::Sense::click_and_drag());
        let drag_id = id.with("drag");
        if response.dragged() {
            let offset = ui
                .data(|data| data.get_temp::<Vec2>(drag_id))
                .unwrap_or_default()
                + response.drag_delta();
            ui.data_mut(|data| data.insert_temp(drag_id, offset));
        }
        let offset = ui
            .data(|data| data.get_temp::<Vec2>(drag_id))
            .unwrap_or_default();
        let (anchor, at) = if offset == Vec2::ZERO {
            (note.anchor.clone(), at)
        } else {
            let anchor = moved(pane, frame, &note.anchor, at + offset);
            let at = point(model, pane, frame, &anchor).unwrap_or(at + offset);
            (anchor, at)
        };
        if response.drag_stopped() {
            ui.data_mut(|data| data.remove::<Vec2>(drag_id));
            if anchor != note.anchor {
                apply(NoteEdit::Set {
                    note: note.selector(),
                    anchor: anchor.clone(),
                    text: note.text.clone(),
                });
            }
        }
        let drawn = note_box(rect, at, size, on_point);
        if on_point {
            let corner = pos2(
                if drawn.center().x < at.x {
                    drawn.right()
                } else {
                    drawn.left()
                },
                if drawn.center().y < at.y {
                    drawn.bottom()
                } else {
                    drawn.top()
                },
            );
            painter.line_segment([at, corner], egui::Stroke::new(1.0, c.text_dim));
            painter.circle_filled(at, 2.5, c.text);
        }
        let ink = if response.hovered() || response.dragged() {
            c.border_strong
        } else {
            c.border
        };
        painter.rect(
            drawn,
            t.radius,
            c.bg_elevated,
            egui::Stroke::new(1.0, ink),
            egui::StrokeKind::Inside,
        );
        painter.galley(drawn.min + PAD, galley, c.text);
        if response.hovered() && !response.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
        }
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Label,
                true,
                format!("Note: {}", note.text),
            )
        });

        let mut choice = None;
        if response.double_clicked() {
            choice = Some(NoteChoice::Edit);
        }
        crate::context_menu::context_menu(&response, |ui| {
            if ui.button("Edit text").clicked() {
                choice = Some(NoteChoice::Edit);
                ui.close();
            }
            ui.separator();
            if !matches!(note.anchor, NoteAnchor::Frame { .. })
                && ui
                    .button("Pin to the frame")
                    .on_hover_text("Keep the note where it is in the strip as the view moves")
                    .clicked()
            {
                let place = NoteAnchor::Frame {
                    unit: Some(pane.unit.to_owned()),
                    x: 0.0,
                    y: 0.0,
                };
                choice = Some(NoteChoice::Pin(moved(pane, frame, &place, drawn.min)));
                ui.close();
            }
            if !matches!(note.anchor, NoteAnchor::Data { .. })
                && ui
                    .button("Pin to the data")
                    .on_hover_text("Keep the note on this point of the axes as the view moves")
                    .clicked()
            {
                let unit = match &note.anchor {
                    NoteAnchor::Trace { anchor, .. } => {
                        anchored_trace(model, pane, anchor).map_or(pane.unit, |(_, unit)| unit)
                    }
                    _ => pane.unit,
                };
                let place = NoteAnchor::Data {
                    unit: unit.to_owned(),
                    x: 0.0,
                    y: 0.0,
                };
                choice = Some(NoteChoice::Pin(moved(pane, frame, &place, at)));
                ui.close();
            }
            if !matches!(note.anchor, NoteAnchor::Trace { .. })
                && let Some(trace) = nearest_curve(model, pane, frame, at)
                && ui
                    .button(format!("Pin to {}", trace.name))
                    .on_hover_text("Keep the note on this curve at its X, wherever the curve goes")
                    .clicked()
            {
                choice = Some(NoteChoice::Pin(NoteAnchor::Trace {
                    anchor: anchor_key(model, trace),
                    trace_name: trace.name.clone(),
                    x: frame.data_x(at.x),
                }));
                ui.close();
            }
            ui.separator();
            if ui.button("Delete").clicked() {
                choice = Some(NoteChoice::Delete);
                ui.close();
            }
        });
        match choice {
            Some(NoteChoice::Edit) => start(
                ui,
                session,
                NoteDraft {
                    analysis: model.analysis_key,
                    note: Some(note.selector()),
                    anchor: note.anchor.clone(),
                    text: note.text.clone(),
                },
            ),
            Some(NoteChoice::Pin(anchor)) => apply(NoteEdit::Set {
                note: note.selector(),
                anchor,
                text: note.text.clone(),
            }),
            Some(NoteChoice::Delete) => apply(NoteEdit::Delete {
                note: note.selector(),
            }),
            None => {}
        }
    }
    write_draft(ui, session, model, pane, frame, apply);
}

/// The field a note is written in, where the note stands, while the reader
/// writes one on this pane. Enter or a click elsewhere keeps what was
/// written, Escape keeps nothing; a note emptied is deleted.
fn write_draft(
    ui: &mut Ui,
    session: &mut ResultViewerState,
    model: &StripModel,
    pane: &UnitPane,
    frame: &NoteFrame<'_>,
    apply: &mut impl FnMut(NoteEdit),
) {
    let Some(draft) = session
        .note_draft
        .as_mut()
        .filter(|draft| draft.analysis == model.analysis_key)
    else {
        return;
    };
    let Some(at) = point(model, pane, frame, &draft.anchor) else {
        return;
    };
    let on_point = !matches!(draft.anchor, NoteAnchor::Frame { .. });
    let font = theme::sans(11.0, FontWeight::Regular);
    let width = ui
        .painter()
        .layout_no_wrap(draft.text.clone(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        .max(FIELD_WIDTH)
        + 2.0 * PAD.x;
    let height = ui.fonts_mut(|fonts| fonts.row_height(&font)) + 2.0 * PAD.y + 2.0;
    let rect = note_box(frame.plot_rect, at, vec2(width, height), on_point);
    let id = field_id(model.analysis_key);
    let seeded = id.with("seeded");
    let response = ui.put(
        rect,
        egui::TextEdit::singleline(&mut draft.text)
            .id(id)
            .font(font)
            .hint_text("Note")
            .margin(PAD)
            .desired_width(width - 2.0 * PAD.x),
    );
    if ui.data(|data| data.get_temp::<bool>(seeded)).is_none() {
        response.request_focus();
        ui.data_mut(|data| data.insert_temp(seeded, true));
        return;
    }
    if !response.lost_focus() {
        return;
    }
    ui.data_mut(|data| data.remove::<bool>(seeded));
    let draft = session
        .note_draft
        .take()
        .expect("the draft was just written in");
    if ui.input(|input| input.key_pressed(Key::Escape)) {
        return;
    }
    let text = draft.text.trim().to_owned();
    match (draft.note, text.is_empty()) {
        (None, true) => {}
        (None, false) => apply(NoteEdit::Add {
            analysis: draft.analysis,
            anchor: draft.anchor,
            text,
        }),
        (Some(note), true) => apply(NoteEdit::Delete { note }),
        (Some(note), false) => apply(NoteEdit::Set {
            note,
            anchor: draft.anchor,
            text,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derived::DerivedSeries;
    use crate::waves::ProjectionOptions;
    use rspice_results::{
        analysis_result::AnalysisResult, analysis_type::AnalysisType,
        executed_deck::ExecutedDeckArchive, waveform::RetainedTrace,
    };
    use std::collections::HashSet;

    /// One strip on a logarithmic X axis from 100 to 1e6, whose curve falls a
    /// volt a decade from 4 V.
    fn model() -> StripModel {
        let x = vec![1.0e2, 1.0e3, 1.0e4, 1.0e5, 1.0e6];
        let y = vec![4.0, 3.0, 2.0, 1.0, 0.0];
        let mut run = rspice_results::run::SimulationRun::new(
            1,
            0.0,
            rspice_results::run::ExecutionTarget::LocalDesktop,
        );
        run.analyses.push(
            AnalysisResult::new(1, AnalysisType::Transient, "Tran", 0.0).with_waveforms(vec![
                RetainedTrace::new("V(out)", x, y, "#fff").with_unit("V"),
            ]),
        );
        let mut models = crate::waves::build_models(
            &[&run],
            &ExecutedDeckArchive::default(),
            &mut DerivedSeries::default(),
            &Tokens::default(),
            ProjectionOptions {
                phase_continuous: false,
                complex_display: Default::default(),
                ac: Default::default(),
                selection: None,
                hidden_family_traces: &HashSet::new(),
                waveform_visibility: &Default::default(),
                signals: None,
            },
            |_| None,
        );
        crate::waves::extent::resolve_x_ranges(&mut models);
        let mut model = models.remove(0);
        model.x_scale = XScale::Log10;
        model
    }

    fn pane() -> UnitPane<'static> {
        UnitPane {
            unit: "V",
            axis: "V",
            traces: vec![0].into(),
            expressions: Vec::new(),
            hidden_expressions: Vec::new(),
            unlabeled: false,
            secondary: Vec::new(),
            slot: SHARED_X_SLOT,
            weight: 1.0,
            locked: false,
            arranged: false,
        }
    }

    /// The plot drawn 400 by 200 over X 100 to 1e6 and Y 0 to 4 V.
    fn frame() -> NoteFrame<'static> {
        NoteFrame {
            plot_rect: Rect::from_min_size(Pos2::ZERO, vec2(400.0, 200.0)),
            x: (1.0e2, 1.0e6),
            x_scale: XScale::Log10,
            y: (0.0, 4.0),
            y_scale: XScale::Linear,
            secondary: &[],
            interpolation: SampleInterpolation::Linear,
            id: egui::Id::new("plot"),
        }
    }

    fn near(at: Pos2, expected: Pos2) -> bool {
        (at - expected).length() < 1.0e-3
    }

    /// A note on a curve stands on the line the curve is drawn with, which
    /// runs straight on screen between samples of a logarithmic X axis, and
    /// a drag slides it along the curve to the X under the pointer.
    #[test]
    fn a_note_on_a_curve_stands_on_its_drawn_line_and_slides_along_it() {
        let (model, pane, frame) = (model(), pane(), frame());
        let note = NoteAnchor::Trace {
            anchor: anchor_key(&model, &model.traces[0]),
            trace_name: "V(out)".to_owned(),
            x: 10f64.powf(3.5),
        };
        // Three eighths of the way along four decades; 2.5 V, halfway
        // between the samples either side.
        let at = point(&model, &pane, &frame, &note).expect("the note is on this pane");
        assert!(near(at, pos2(150.0, 75.0)), "{at:?}");
        let dragged = moved(&pane, &frame, &note, pos2(250.0, 20.0));
        let at = point(&model, &pane, &frame, &dragged).expect("the note is on this pane");
        assert!(near(at, pos2(250.0, 125.0)), "{at:?}");
    }

    /// A note on a point of the data stands where the axes put that point,
    /// and a drag puts it on the point under the pointer.
    #[test]
    fn a_note_on_the_data_stands_on_its_point_and_moves_to_the_pointer() {
        let (model, pane, frame) = (model(), pane(), frame());
        let note = NoteAnchor::Data {
            unit: "V".to_owned(),
            x: 1.0e4,
            y: 1.0,
        };
        let at = point(&model, &pane, &frame, &note).expect("the note is on this pane");
        assert!(near(at, pos2(200.0, 150.0)), "{at:?}");
        let NoteAnchor::Data { x, y, .. } = moved(&pane, &frame, &note, pos2(100.0, 50.0)) else {
            panic!("a note on the data stays on the data");
        };
        assert!((x / 1.0e3 - 1.0).abs() < 1.0e-9, "{x}");
        assert!((y - 3.0).abs() < 1.0e-9, "{y}");
        // A note on another strip's axis is not this pane's.
        let elsewhere = NoteAnchor::Data {
            unit: "A".to_owned(),
            x: 1.0e4,
            y: 1.0,
        };
        assert_eq!(point(&model, &pane, &frame, &elsewhere), None);
    }
}
