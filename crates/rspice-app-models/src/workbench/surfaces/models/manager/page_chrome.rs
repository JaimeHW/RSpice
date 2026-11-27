//! What every page of the workspace draws the same way: a line a screen
//! reader can reach, a marked line, and the refusal a page states when the
//! action taken from it was refused.

use super::*;

/// One line of banner prose that a screen reader can actually reach.
///
/// A plain `ui.label` in this workspace publishes no accessibility node, which
/// would leave an error banner unreadable by exactly the reader who most needs
/// it read aloud. Every line of the banner goes through here instead.
pub(super) fn announced(ui: &mut Ui, text: RichText, label: &str) {
    announced_widget(ui, egui::Label::new(text), label);
}

/// One line of banner prose the reader has to notice: the mark its severity
/// earns, then the words in the ink every other line on the page is in.
///
/// The announced text is the sentence alone, because the sentences this page
/// marks already say what is wrong in their own words and a screen reader
/// reading "Error:" in front of "the library file could not be read" hears
/// the severity twice.
pub(super) fn marked_line(ui: &mut Ui, problem: Problem, text: &str) {
    ui.horizontal(|ui| {
        // The kit's own advance, so a marked line and an unmarked one beside
        // it start their words the same distance apart wherever both appear.
        ui.spacing_mut().item_spacing.x = MARK_ADVANCE - MARK_SIDE;
        let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(MARK_SIDE), Sense::hover());
        paint_problem_mark(ui.painter(), rect, problem);
        announced(ui, RichText::new(text).small(), text);
    });
}

/// The same, for a line that has to be told how to behave when it runs out of
/// room — truncating rather than pushing the control beside it off the pane.
///
/// What truncates is the painted glyphs. The announced label is the whole
/// sentence either way, which is the point of there being one door.
pub(super) fn announced_widget(ui: &mut Ui, widget: egui::Label, label: &str) {
    let response = ui.add(widget.sense(Sense::hover()));
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
    // `widget_info` alone publishes nothing for a non-interactive widget, so
    // the node is declared outright — the same route the preflight report's
    // panel headings take.
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_label(label);
    });
}

/// A refused action on any manager page, stated on the page it was taken
/// from: the failure mark and the reason, wrapped to the page. The pages name
/// their actions in their own toolbars, and offer no retry here.
pub(super) fn page_refusal(ui: &mut Ui, state: &ModelsState) {
    let Some(Err(reason)) = &state.models_view.action_receipt else {
        return;
    };
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .inner_margin(egui::Margin::symmetric(12, 7))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = MARK_ADVANCE - MARK_SIDE;
                let (rect, _) =
                    ui.allocate_exact_size(egui::Vec2::splat(MARK_SIDE), Sense::hover());
                paint_problem_mark(ui.painter(), rect, Problem::Failure);
                announced_widget(
                    ui,
                    egui::Label::new(RichText::new(reason).small().color(t.color.text)).wrap(),
                    reason,
                );
            });
        });
}
