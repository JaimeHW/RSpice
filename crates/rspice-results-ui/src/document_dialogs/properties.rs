//! The formatting a result document carries: displayed significant digits and
//! whether phase traces are unwrapped.

use egui::Context;
use rspice_ui_kit::widgets::{CommandForm, DialogChoice};

/// The fewest and most significant digits a displayed value may carry, as
/// the document model bounds them.
const MIN_DIGITS: u8 = 3;
const MAX_DIGITS: u8 = 17;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertiesDraft {
    pub significant_digits: u8,
    /// Draw phase as a continuous (unwrapped) curve instead of wrapped to
    /// ±180°.
    pub phase_continuous: bool,
}

/// `scope` names what the properties apply to: the open document, or the
/// quick view, whose digits are the user's own default.
pub fn show(ctx: &Context, draft: &mut PropertiesDraft, scope: &str) -> DialogChoice {
    CommandForm::new("Plot properties", "Apply")
        .context(Some(scope))
        .describe("Set the significant digits values are shown with and how phase is drawn.")
        .status_fixed_at_open()
        .show(ctx, |rows| {
            let digits = (MIN_DIGITS..=MAX_DIGITS).collect::<Vec<_>>();
            let options = digits.iter().map(u8::to_string).collect::<Vec<_>>();
            if let Some(index) = rows.select(
                "Digits",
                "rspice.plot-properties.digits",
                &draft.significant_digits.to_string(),
                &options,
            ) {
                draft.significant_digits = digits[index];
            }
            let mut phase = usize::from(draft.phase_continuous);
            if rows.segmented(
                "Phase",
                "rspice.plot-properties.phase",
                &["Wrapped", "Continuous"],
                &mut phase,
            ) {
                draft.phase_continuous = phase == 1;
            }
            None
        })
}
