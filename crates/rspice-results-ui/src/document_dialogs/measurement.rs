//! Add a scalar measurement to the selected pane of a result document.

use egui::{Context, Id};
use rspice_ui_kit::widgets::{CommandForm, DialogChoice, FieldLine};

/// The expression being measured, as the reader typed it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MeasurementDraft {
    pub expression: String,
}

/// What the host resolved the draft to on this frame.
pub struct MeasurementPreview {
    /// The pane the measurement is kept on, or the result a quick view shows.
    pub pane: String,
    /// `Ok(None)` while nothing is typed, the formatted value once the
    /// expression reduces to a number, or why it does not.
    pub value: Result<Option<String>, String>,
    /// Why Add is refused whatever the expression: a quick view keeps no
    /// measurements.
    pub refusal: Option<String>,
}

pub fn show(
    ctx: &Context,
    draft: &mut MeasurementDraft,
    preview: &MeasurementPreview,
) -> DialogChoice {
    let field = Id::new("rspice.add-measurement.expression");
    let refusal = preview.refusal.as_deref();
    CommandForm::new("Add measurement", "Add")
        .context(Some(preview.pane.as_str()))
        .describe(
            "Evaluate a scalar expression on the selected pane and keep its value in the document.",
        )
        .primary_enabled(refusal.is_none() && matches!(preview.value, Ok(Some(_))))
        .status(refusal.map_or(Ok(None), Err))
        .status_fixed_at_open()
        .show(ctx, |rows| {
            rows.text("Expression", field, &mut draft.expression, "rms(V(out))");
            rows.field_line(
                Some(field),
                match &preview.value {
                    Ok(None) => FieldLine::Blank,
                    Ok(Some(value)) => FieldLine::Derived(value),
                    Err(reason) => FieldLine::Error(reason),
                },
            );
            Some(field)
        })
}
