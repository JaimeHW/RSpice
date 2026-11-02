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

/// What the line under Expression says before anything is typed.
const EXPRESSION_MISSING: &str = "Type an expression that reduces to one number.";

pub fn show(
    ctx: &Context,
    draft: &mut MeasurementDraft,
    preview: &MeasurementPreview,
) -> DialogChoice {
    let field = Id::new("rspice.add-measurement.expression");
    let refusal = preview.refusal.as_deref();
    CommandForm::new("Add measurement", "Add")
        .describe(
            "Evaluate a scalar expression on the selected pane and keep its value in the document.",
        )
        .primary_enabled(refusal.is_none() && matches!(preview.value, Ok(Some(_))))
        .status(refusal.map_or(Ok(None), Err))
        .show(ctx, |rows| {
            rows.value("Pane", &preview.pane);
            rows.text("Expression", field, &mut draft.expression, "rms(V(out))");
            rows.field_line(
                Some(field),
                // The line is never blank: before anything is typed it
                // says what Add waits for, so the form keeps one pitch.
                match &preview.value {
                    Ok(None) => FieldLine::Derived(EXPRESSION_MISSING),
                    Ok(Some(value)) => FieldLine::Value(value),
                    Err(reason) => FieldLine::Error(reason),
                },
            );
            Some(field)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Before anything is typed, the line under Expression says what Add
    /// waits for, so no blank slot stands between the field and the footer.
    #[test]
    fn an_empty_expression_says_what_add_waits_for() {
        let ctx = Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut draft = MeasurementDraft::default();
        let preview = MeasurementPreview {
            pane: "Run 1 (8:31:04 PM)".to_owned(),
            value: Ok(None),
            refusal: None,
        };
        let mut said = false;
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    show(ui.ctx(), &mut draft, &preview);
                },
            );
            said |= output
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes
                .iter()
                .any(|(_, node)| {
                    node.label() == Some(EXPRESSION_MISSING)
                        || node.value() == Some(EXPRESSION_MISSING)
                });
        }
        assert!(said, "the line under Expression is blank");
    }
}
