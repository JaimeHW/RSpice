//! Shared visual and transaction contracts for mockup-owned schematic commands.
//!
//! Command modules retain their own typed authority and commit semantics. This
//! module only centralizes the exact field treatment and copy that the restored
//! design applies to every schematic transform dialog.

use egui::{Frame, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

pub(crate) const FOOTER_NOTE: &str = "Pointer, touch, stylus, and keyboard entry resolve to the same exact coordinates. Escape cancels without modifying the document.";
pub(crate) const DISCARD_TITLE: &str = "Unsaved dialog changes";
pub(crate) const DISCARD_DETAIL: &str = "Choose Discard changes again to close, or continue editing. No project or result data has been changed.";

pub(crate) fn snap_label(pitch: crate::state::SchematicGridPitch) -> &'static str {
    match pitch {
        crate::state::SchematicGridPitch::Mil50 => "50 mil",
        crate::state::SchematicGridPitch::Mil25 => "25 mil",
        crate::state::SchematicGridPitch::Metric => "Metric",
    }
}

pub(crate) fn field_label<R>(ui: &mut Ui, label: &str, content: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label.to_ascii_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
    ui.add_space(4.0);
    content(ui)
}

pub(crate) fn read_only_value(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    field_label(ui, label, |ui| {
        Frame::new()
            .fill(t.color.bg_panel)
            .stroke(egui::Stroke::new(1.0, t.color.border))
            .corner_radius(5.0)
            .inner_margin(egui::Margin::symmetric(8, 6))
            .show(ui, |ui| {
                ui.set_min_width((ui.available_width() - 16.0).max(1.0));
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(value)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text),
                    )
                    .wrap(),
                );
            });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snap_copy_tracks_every_document_pitch() {
        assert_eq!(
            snap_label(crate::state::SchematicGridPitch::Mil50),
            "50 mil"
        );
        assert_eq!(
            snap_label(crate::state::SchematicGridPitch::Mil25),
            "25 mil"
        );
        assert_eq!(
            snap_label(crate::state::SchematicGridPitch::Metric),
            "Metric"
        );
    }
}
