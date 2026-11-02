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
        .describe("Set the significant digits values are shown with and how phase is drawn.")
        .show(ctx, |rows| {
            rows.value("Applies to", scope);
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
            // Phase spans the control column Digits spans, so the row does
            // not stop short of the one above it.
            if rows.segmented_columns(
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Phase's two choices span the column the Digits list spans.
    #[test]
    fn phase_spans_the_column_digits_spans() {
        use egui::accesskit::Role;
        let ctx = Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut draft = PropertiesDraft {
            significant_digits: 7,
            phase_continuous: false,
        };
        let mut nodes = std::collections::HashMap::new();
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
                    show(ui.ctx(), &mut draft, "Quick view");
                },
            );
            nodes.extend(
                output
                    .platform_output
                    .accesskit_update
                    .expect("AccessKit tree update")
                    .nodes,
            );
        }
        let control = |name: &str| {
            nodes
                .values()
                .find(|node| node.label() == Some(name) && node.role() != Role::Label)
                .and_then(egui::accesskit::Node::bounds)
                .unwrap_or_else(|| panic!("{name} is published"))
        };
        let (digits, wrapped, continuous) =
            (control("Digits"), control("Wrapped"), control("Continuous"));
        assert!(
            (wrapped.x0 - digits.x0).abs() < 1.0 && (continuous.x1 - digits.x1).abs() < 1.0,
            "{wrapped:?}..{continuous:?} against {digits:?}"
        );
    }
}
