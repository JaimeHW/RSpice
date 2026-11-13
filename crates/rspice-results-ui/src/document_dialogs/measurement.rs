//! Add a scalar measurement to the selected pane of a result document.
//!
//! The reader picks a measurement from the catalogue — searchable, grouped
//! by what it measures — and fills its labelled arguments; the fields write
//! the expression. The expression stays editable, and a catalogued call
//! typed into it fills the fields back, so the two never describe different
//! measurements. A signal argument picks from the pane's traces.

use egui::{Context, Id};
use rspice_results::calculator::catalogue::{
    self, ArgumentDefault, ArgumentKind, ArgumentSpec, MEASUREMENTS, MeasureGroup, MeasureSpec,
};
use rspice_ui_kit::widgets::{CommandForm, DialogChoice, FieldLine, FormRows, SelectGroup};

/// The measurement being built, as the reader left it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MeasurementDraft {
    pub expression: String,
    /// The catalogued measurement whose fields write the expression, while
    /// the expression is one call to it.
    pub function: Option<&'static MeasureSpec>,
    /// One field per argument of `function`, as typed or picked.
    pub arguments: Vec<String>,
    /// What the measurement list is narrowed to.
    pub search: String,
    /// The signal argument waiting for a trace the reader clicks in the
    /// strip. The form steps aside while it waits.
    pub picking: Option<usize>,
}

/// What the host resolved the draft to on this frame.
pub struct MeasurementPreview {
    /// The pane the measurement is kept on, or the result a quick view shows.
    pub pane: String,
    /// The traces the pane shows, which a signal argument picks from.
    pub signals: Vec<String>,
    /// The unit each of `signals` reads in, in the same order, which a level
    /// of it is typed in.
    pub signal_units: Vec<String>,
    /// The unit of the pane's x-axis, which a point on it is typed in.
    pub x_unit: String,
    /// `Ok(None)` while nothing is typed, the formatted value once the
    /// expression reduces to a number, or why it does not.
    pub value: Result<Option<String>, String>,
    /// Why Add is refused whatever the expression: a quick view keeps no
    /// measurements.
    pub refusal: Option<String>,
}

/// What the line under Expression says before anything is typed.
const EXPRESSION_MISSING: &str = "Type an expression that reduces to one number.";
/// What the Measurement field shows while the expression is not one call
/// to a catalogued measurement.
const CUSTOM: &str = "Expression";
const FUNCTION_SALT: &str = "rspice.add-measurement.function";
/// The columns every choice row's segments are cut from, so the rows of one
/// form keep a shared option in one column: an edge type's rising, falling
/// and either, and a second edge type's same four with "same" last.
const CHOICE_COLUMNS: usize = 4;
/// The option a second edge type offers for "as the first".
const SAME: &str = "same";

/// The label of the action that takes a signal argument from the strip.
const PICK: &str = "Pick";

pub fn show(
    ctx: &Context,
    draft: &mut MeasurementDraft,
    preview: &MeasurementPreview,
) -> DialogChoice {
    if let Some(index) = draft.picking {
        picking_prompt(ctx, draft, index);
        return DialogChoice::None;
    }
    let field = Id::new("rspice.add-measurement.expression");
    let refusal = preview.refusal.as_deref();
    let incomplete = draft
        .function
        .and_then(|spec| spec.call(&draft.arguments).err());
    CommandForm::new("Add measurement", "Add")
        .wide()
        .describe(
            "Pick a measurement and fill its arguments, or type any expression that reduces to \
             one number. Its value on the selected pane is kept in the document.",
        )
        .primary_enabled(
            refusal.is_none() && incomplete.is_none() && matches!(preview.value, Ok(Some(_))),
        )
        .status(refusal.map_or(Ok(None), Err))
        .show(ctx, |rows| {
            rows.value("Pane", &preview.pane);
            let groups = picker_groups();
            let selected = draft
                .function
                .map_or_else(|| CUSTOM.to_owned(), MeasureSpec::short_signature);
            if let Some((group, option)) = rows.searchable_select(
                "Measurement",
                FUNCTION_SALT,
                &selected,
                &groups,
                &mut draft.search,
            ) {
                pick(
                    draft,
                    in_group(MeasureGroup::ALL[group])[option],
                    &preview.signals,
                );
            }
            if let Some(spec) = draft.function {
                rows.derived_line(spec.description);
                let mut edited = false;
                for (index, argument) in spec.arguments.iter().enumerate() {
                    edited |= argument_row(rows, spec, index, argument, draft, preview);
                }
                if edited && let Ok(call) = spec.call(&draft.arguments) {
                    draft.expression = call;
                }
            }
            if rows
                .text("Expression", field, &mut draft.expression, "rms(V(out))")
                .changed()
            {
                reread(draft);
            }
            rows.field_line(
                Some(field),
                // The line is never blank: before anything is typed it
                // says what Add waits for, so the form keeps one pitch.
                match (&incomplete, &preview.value) {
                    (Some(problem), _) => FieldLine::Error(problem),
                    (None, Ok(None)) => FieldLine::Derived(EXPRESSION_MISSING),
                    (None, Ok(Some(value))) => FieldLine::Value(value),
                    (None, Err(reason)) => FieldLine::Error(reason),
                },
            );
            Some(field)
        })
}

/// While the reader picks a trace for the argument at `index`: the form
/// steps aside so the strip can be clicked, and one line says what the click
/// will do and how to come back without one. The line sits at the foot of
/// the strip, over its time axis, where a status line would read; at the
/// head it would cover the legend entries a click can also take.
fn picking_prompt(ctx: &Context, draft: &mut MeasurementDraft, index: usize) {
    let label = draft
        .function
        .and_then(|spec| spec.arguments.get(index))
        .map_or("the signal", |argument| argument.label);
    let t = rspice_ui_kit::tokens::Tokens::get(ctx);
    let mut back =
        ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
    let over = crate::waves::stack::last_strip_area(ctx).unwrap_or_else(|| ctx.content_rect());
    egui::Area::new(Id::new("rspice.add-measurement.picking"))
        .order(egui::Order::Foreground)
        .pivot(egui::Align2::CENTER_BOTTOM)
        .fixed_pos(egui::pos2(over.center().x, over.bottom() - 12.0))
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(t.color.bg_elevated)
                .stroke(egui::Stroke::new(1.0, t.color.border_strong))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(12, 6))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 12.0;
                        ui.label(
                            egui::RichText::new(format!("Click a trace in the strip for {label}."))
                                .font(rspice_ui_kit::theme::sans(
                                    rspice_ui_kit::tokens::FS_1,
                                    rspice_ui_kit::theme::FontWeight::Regular,
                                ))
                                .color(t.color.text),
                        );
                        back |= rspice_ui_kit::widgets::Button::new("Back to form")
                            .show(ui)
                            .on_hover_text("Esc")
                            .clicked();
                    });
                });
        });
    if back {
        draft.picking = None;
    }
}

/// Fill the argument waiting for a pick with the trace the reader clicked:
/// `source`, the waveform it draws, as the pane's traces name it, else the
/// `name` the strip shows it by. The form comes back with the call
/// rewritten.
pub fn fill_picked(
    draft: &mut MeasurementDraft,
    preview: &MeasurementPreview,
    source: &str,
    name: &str,
) {
    let Some(index) = draft.picking.take() else {
        return;
    };
    let signal = preview
        .signals
        .iter()
        .find(|signal| *signal == source)
        .or_else(|| {
            preview
                .signals
                .iter()
                .find(|signal| signal.eq_ignore_ascii_case(source) || *signal == name)
        })
        .map_or_else(|| source.to_owned(), Clone::clone);
    if let Some(value) = draft.arguments.get_mut(index) {
        *value = signal;
    }
    if let Some(call) = draft
        .function
        .and_then(|spec| spec.call(&draft.arguments).ok())
    {
        draft.expression = call;
    }
}

fn in_group(group: MeasureGroup) -> Vec<&'static MeasureSpec> {
    MEASUREMENTS
        .iter()
        .filter(|spec| spec.group == group)
        .collect()
}

fn picker_groups() -> Vec<SelectGroup<'static>> {
    MeasureGroup::ALL
        .iter()
        .map(|&group| SelectGroup {
            heading: group.label(),
            options: in_group(group)
                .into_iter()
                .map(|spec| (spec.short_signature(), spec.summary))
                .collect(),
        })
        .collect()
}

/// Switch the fields to `spec`, keeping the signals already chosen and
/// filling any others from the pane's traces in order.
fn pick(draft: &mut MeasurementDraft, spec: &'static MeasureSpec, signals: &[String]) {
    let mut chosen: Vec<String> = draft
        .function
        .map(|old| {
            old.arguments
                .iter()
                .zip(&draft.arguments)
                .filter(|(argument, value)| {
                    argument.kind == ArgumentKind::Signal && !value.trim().is_empty()
                })
                .map(|(_, value)| value.clone())
                .collect()
        })
        .unwrap_or_default();
    for signal in signals {
        if !chosen.contains(signal) {
            chosen.push(signal.clone());
        }
    }
    let mut chosen = chosen.into_iter();
    draft.function = Some(spec);
    draft.arguments = spec
        .arguments
        .iter()
        .map(|argument| match argument.kind {
            ArgumentKind::Signal => chosen.next().unwrap_or_default(),
            _ => String::new(),
        })
        .collect();
    // A measurement that still needs a number shows no stale call; the line
    // under the expression names the field it waits for.
    draft.expression = spec.call(&draft.arguments).unwrap_or_default();
}

/// Follow a hand edit of the expression: a call to a catalogued measurement
/// fills its fields. While the reader is still typing a call to the same
/// measurement the fields stay as they were rather than vanishing.
fn reread(draft: &mut MeasurementDraft) {
    if let Some((spec, mut fields)) = catalogue::split_call(&draft.expression) {
        fields.resize(spec.arguments.len(), String::new());
        draft.function = Some(spec);
        draft.arguments = fields;
        return;
    }
    let typing_same = draft.function.is_some_and(|spec| {
        let text = draft.expression.trim_start();
        text.len() > spec.name.len()
            && text[..spec.name.len()].eq_ignore_ascii_case(spec.name)
            && text[spec.name.len()..].trim_start().starts_with('(')
    });
    if !typing_same {
        draft.function = None;
        draft.arguments.clear();
    }
}

/// What an empty number field means, as `(hint, note)`: a value short
/// enough for the field is its hint, and words that are not go after it.
fn default_words(spec: &MeasureSpec, argument: &ArgumentSpec) -> (String, String) {
    match argument.default {
        ArgumentDefault::Required => ("required".to_owned(), String::new()),
        ArgumentDefault::Literal(word) => (word.to_owned(), String::new()),
        ArgumentDefault::Derived(words) => (String::new(), words.to_owned()),
        ArgumentDefault::SameAs(other) => (
            String::new(),
            format!("same as {}", spec.arguments[other].label.to_lowercase()),
        ),
    }
}

/// The unit `signal` reads in: the pane's own word for a trace it shows,
/// else what its accessor says (`V(...)` is volts), else none.
fn unit_of(signal: &str, preview: &MeasurementPreview) -> String {
    let signal = signal.trim();
    if signal.is_empty() {
        return String::new();
    }
    preview
        .signals
        .iter()
        .position(|known| known == signal)
        .and_then(|index| preview.signal_units.get(index))
        .cloned()
        .unwrap_or_else(|| crate::waves::browser_signal_unit(signal, None, "").to_owned())
}

/// The unit a frequency along an x-axis read in `x_unit` is typed in: hertz
/// along time, else the reciprocal of the axis's unit.
fn axis_frequency_unit(x_unit: &str) -> String {
    match x_unit {
        "" => String::new(),
        "s" => "Hz".to_owned(),
        unit => format!("1/{unit}"),
    }
}

/// One argument's row. Returns whether the reader changed it.
fn argument_row(
    rows: &mut FormRows<'_>,
    spec: &MeasureSpec,
    index: usize,
    argument: &ArgumentSpec,
    draft: &mut MeasurementDraft,
    preview: &MeasurementPreview,
) -> bool {
    let salt = format!("rspice.add-measurement.argument.{index}");
    let signals = &preview.signals;
    // A number states the unit it is typed in inside its field: a level its
    // signal's, a point on the x-axis the axis's.
    let unit = match argument.kind {
        ArgumentKind::Level(signal) => draft
            .arguments
            .get(signal)
            .map_or_else(String::new, |signal| unit_of(signal, preview)),
        ArgumentKind::AxisPoint => preview.x_unit.clone(),
        ArgumentKind::AxisFrequency => axis_frequency_unit(&preview.x_unit),
        _ => String::new(),
    };
    let value = &mut draft.arguments[index];
    match argument.kind {
        ArgumentKind::Signal if !signals.is_empty() => {
            let shown = if value.trim().is_empty() {
                "Choose a trace"
            } else {
                value.as_str()
            };
            // A signal picks from the pane's traces here, or from the strip
            // itself: Pick steps the form aside until a trace is clicked.
            let (picked, pick) =
                rows.select_with_action(argument.label, &salt, shown, signals, PICK);
            if let Some(trace) = picked {
                *value = signals[trace].clone();
            }
            if pick {
                draft.picking = Some(index);
            }
            picked.is_some()
        }
        ArgumentKind::Signal => rows
            .text(argument.label, Id::new(&salt), value, "required")
            .changed(),
        ArgumentKind::Choice(words) => {
            // An empty field is the default, so the expression only spells a
            // choice the reader changed. "As the first" comes last, so the
            // words both rows offer stand in the same columns.
            let (same, default) = match argument.default {
                ArgumentDefault::SameAs(_) => (true, None),
                ArgumentDefault::Literal(word) => (false, Some(word)),
                _ => (false, None),
            };
            let mut options = words.to_vec();
            if same {
                options.push(SAME);
            }
            let current = if value.is_empty() {
                default.unwrap_or(SAME)
            } else {
                value.as_str()
            };
            let mut selected = options
                .iter()
                .position(|word| *word == current)
                .unwrap_or(0);
            if !rows.segmented_in_columns(
                argument.label,
                &salt,
                &options,
                &mut selected,
                CHOICE_COLUMNS,
            ) {
                return false;
            }
            let word = options[selected];
            *value = if (same && word == SAME) || Some(word) == default {
                String::new()
            } else {
                word.to_owned()
            };
            true
        }
        ArgumentKind::Number
        | ArgumentKind::Level(_)
        | ArgumentKind::AxisPoint
        | ArgumentKind::AxisFrequency => {
            let (hint, note) = default_words(spec, argument);
            // The note says what an empty field stands for; a typed value
            // says it for itself.
            let note = if value.trim().is_empty() {
                note
            } else {
                String::new()
            };
            rows.quantity(argument.label, Id::new(&salt), value, &hint, &unit, &note)
                .changed()
        }
    }
}

#[cfg(test)]
mod tests;
