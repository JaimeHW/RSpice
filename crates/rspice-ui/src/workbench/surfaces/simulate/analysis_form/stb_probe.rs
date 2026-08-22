//! The loop-stability forms' probe field.
//!
//! Loop-gain extraction is defined at one element: a 0 V source standing in
//! the feedback path, which the engine looks up by name in the circuit's
//! voltage-source table. For most of this studio's life the field that named
//! it was free text, so the one input the analysis cannot do without was also
//! the one input nothing checked. A name that matched no element produced a
//! run that failed in the solver, naming a source the schematic had never
//! heard of.
//!
//! The field therefore offers the loop probes the drawing actually holds, and
//! records which of the two kinds of answer it was given. A chosen probe is a
//! reference to an object that can be deleted, and a plan still pointing at a
//! deleted one is refused by name. A typed name is a claim about a deck this
//! application did not draw and cannot check — it is kept because hand-written
//! decks are a real workflow, and refusing it here would break them.
//!
//! Both loop-stability analyses use it. PSTB breaks the same loop at the same
//! element — a 0 V source in the feedback path, looked up by name — and its
//! field stayed free text long after STB's stopped being one, so two forms
//! answered the same question two different ways. The row takes the name and
//! the reference rather than a dialog state, which is what lets one widget
//! serve two drafts without either growing a copy of the other's fields.

use egui::{Align, Layout, Ui, vec2};

use crate::simulation::dialog::StbProbeReference;
use crate::ui::tokens::Tokens;

use super::{
    ENVELOPE_INLINE_CONTROL_GAP, field_cell, full_width_field, mono_input,
    noise_sweep_control_widths, select_mono_with_response, uses_two_column_fields,
};

/// The selector entry that hands the name back to the engineer.
const ENTERED_CHOICE: &str = "Enter name";

/// What the field says about where its name came from, beside the label.
fn hint(placed: &[String], reference: StbProbeReference) -> String {
    if reference == StbProbeReference::Entered {
        "entered by hand".to_owned()
    } else if placed.is_empty() {
        "no loop probe placed".to_owned()
    } else {
        format!("{} placed", placed.len())
    }
}

/// A placed loop probe chosen from the drawing, or a name entered by hand.
///
/// A stale reference is shown as the name it still holds rather than being
/// quietly folded into the entered form. The engineer has to see which probe
/// went missing, and the plan refuses under that same name.
fn control(
    ui: &mut Ui,
    label: &str,
    placed: &[String],
    name: &mut String,
    reference: &mut StbProbeReference,
) {
    let mut options = placed.to_vec();
    options.push(ENTERED_CHOICE.to_owned());
    let current = if *reference == StbProbeReference::Entered {
        ENTERED_CHOICE.to_owned()
    } else {
        name.trim().to_owned()
    };
    let width = ui.available_width();
    let (selector_width, editor_width) = noise_sweep_control_widths(width);
    ui.allocate_ui_with_layout(
        vec2(width, Tokens::get(ui.ctx()).metrics.ctl_h),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = ENVELOPE_INLINE_CONTROL_GAP;
            let salt = format!("analysis-stb-probe-{}", ui.id().value());
            if let Some(index) =
                select_mono_with_response(ui, &salt, label, &current, &options, selector_width)
                    .picked
            {
                match placed.get(index) {
                    Some(chosen) => {
                        name.clone_from(chosen);
                        *reference = StbProbeReference::Placed;
                    }
                    None => *reference = StbProbeReference::Entered,
                }
            }
            ui.add_enabled_ui(*reference == StbProbeReference::Entered, |ui| {
                mono_input(ui, name, editor_width);
            });
        },
    );
}

/// Draw the probe row in whichever column layout the form is using.
pub(super) fn row(
    ui: &mut Ui,
    placed: &[String],
    name: &mut String,
    reference: &mut StbProbeReference,
) {
    let label = "Probe";
    let hint = hint(placed, *reference);
    if uses_two_column_fields(ui) {
        field_cell(ui, label, Some(&hint), |ui| {
            control(ui, label, placed, name, reference);
        });
    } else {
        full_width_field(
            ui,
            label,
            Some(&hint),
            Tokens::get(ui.ctx()).metrics.ctl_h,
            |ui| control(ui, label, placed, name, reference),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_hint_distinguishes_an_entered_name_from_an_empty_drawing() {
        assert_eq!(hint(&[], StbProbeReference::Entered), "entered by hand");
        assert_eq!(hint(&[], StbProbeReference::Placed), "no loop probe placed");
        assert_eq!(
            hint(&["VLOOP1".to_owned()], StbProbeReference::Placed),
            "1 placed"
        );
    }
}
