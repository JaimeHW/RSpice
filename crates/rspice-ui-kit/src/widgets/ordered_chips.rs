//! An ordered list of short names, one chip each: a view search order, a
//! stop list. Each chip moves earlier or later or is removed, and a trailing
//! dropdown adds a name from the ones offered.
//!
//! Every control is its own focusable button, so the list is operable from
//! the keyboard the way any row of buttons is: Tab walks the controls and
//! Enter or Space presses one.

use egui::{Rect, Sense, Stroke, Ui, WidgetInfo, WidgetType, pos2, vec2};

use crate::icons::Icon;
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

/// Width of one chip control: the earlier, later and remove buttons.
const CONTROL_W: f32 = 16.0;
const CHIP_PAD_X: f32 = 7.0;
const ADD_WIDTH: f32 = 86.0;

/// What the reader did to the list this frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderedChipsEdit {
    MoveEarlier(usize),
    MoveLater(usize),
    Remove(usize),
    Add(String),
}

impl OrderedChipsEdit {
    /// Apply the edit to `values`. Returns whether the list changed; an
    /// edit for an index the list no longer has, or a name it already
    /// holds, changes nothing.
    pub fn apply(&self, values: &mut Vec<String>) -> bool {
        match self {
            Self::MoveEarlier(index) if *index > 0 && *index < values.len() => {
                values.swap(*index - 1, *index);
                true
            }
            Self::MoveLater(index) if index + 1 < values.len() => {
                values.swap(*index, *index + 1);
                true
            }
            Self::Remove(index) if *index < values.len() => {
                values.remove(*index);
                true
            }
            Self::Add(name)
                if !name.is_empty()
                    && !values
                        .iter()
                        .any(|existing| existing.eq_ignore_ascii_case(name)) =>
            {
                values.push(name.clone());
                true
            }
            _ => false,
        }
    }
}

/// The chips for `values`, in order, then "+ `add_label`" offering
/// `addable`. With `enabled` false the chips are read-only facts and no
/// control is drawn.
pub fn ordered_chips(
    ui: &mut Ui,
    id_salt: &str,
    accessible_label: &str,
    add_label: &str,
    values: &[String],
    addable: &[String],
    enabled: bool,
) -> Option<OrderedChipsEdit> {
    chip_list(
        ui,
        ChipList {
            id_salt,
            accessible_label,
            add_label,
            values,
            addable,
            enabled,
            keep_one: false,
        },
    )
}

/// [`ordered_chips`] for a list that may never be empty: while it holds one
/// name, that chip offers no Remove.
pub fn required_ordered_chips(
    ui: &mut Ui,
    id_salt: &str,
    accessible_label: &str,
    add_label: &str,
    values: &[String],
    addable: &[String],
    enabled: bool,
) -> Option<OrderedChipsEdit> {
    chip_list(
        ui,
        ChipList {
            id_salt,
            accessible_label,
            add_label,
            values,
            addable,
            enabled,
            keep_one: true,
        },
    )
}

struct ChipList<'a> {
    id_salt: &'a str,
    accessible_label: &'a str,
    add_label: &'a str,
    values: &'a [String],
    addable: &'a [String],
    enabled: bool,
    keep_one: bool,
}

fn chip_list(ui: &mut Ui, list: ChipList<'_>) -> Option<OrderedChipsEdit> {
    let ChipList {
        id_salt,
        accessible_label,
        add_label,
        values,
        addable,
        enabled,
        keep_one,
    } = list;
    let removable = !(keep_one && values.len() == 1);
    let id = ui.id().with(("ordered-chips", id_salt));
    let mut edit = None;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 4.0);
        for (index, value) in values.iter().enumerate() {
            if let Some(chosen) = chip(
                ui,
                id.with(index),
                accessible_label,
                (index, values.len()),
                value,
                enabled.then_some(removable),
            ) {
                edit = Some(chosen);
            }
        }
        if enabled && !addable.is_empty() {
            let width = ADD_WIDTH.min(ui.available_width().max(60.0));
            if let Some(picked) = crate::widgets::select(
                ui,
                &format!("{id_salt}.add"),
                &format!("Add to {accessible_label}"),
                &format!("+ {add_label}"),
                addable,
                width,
            ) {
                edit = Some(OrderedChipsEdit::Add(addable[picked].clone()));
            }
        }
    });
    edit
}

/// One chip. `controls` is `None` for a read-only chip, otherwise whether it
/// offers Remove.
fn chip(
    ui: &mut Ui,
    id: egui::Id,
    list: &str,
    (index, count): (usize, usize),
    value: &str,
    controls: Option<bool>,
) -> Option<OrderedChipsEdit> {
    let t = Tokens::get(ui.ctx());
    let c = &t.color;
    let galley = ui.painter().layout_no_wrap(
        value.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text,
    );
    let control_count = match controls {
        None => 0.0,
        Some(true) => 3.0,
        Some(false) => 2.0,
    };
    // A finger gets a target it can hit for each control, so a press meant
    // for "later" never lands on the "remove" beside it.
    let (control_w, height) = if t.metrics.is_touch() {
        (t.metrics.touch_target(), t.metrics.touch_target())
    } else {
        (CONTROL_W, 22.0)
    };
    let controls_width = if control_count > 0.0 {
        control_count * control_w + 2.0
    } else {
        0.0
    };
    let size = vec2(galley.size().x + 2.0 * CHIP_PAD_X + controls_width, height);
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            ui.is_enabled(),
            format!("{list} {}: {value}", index + 1),
        )
    });
    if !ui.is_rect_visible(rect) {
        return None;
    }
    ui.painter().rect(
        rect,
        t.radius,
        c.bg_panel_2,
        Stroke::new(1.0, c.border),
        egui::StrokeKind::Inside,
    );
    ui.painter().galley(
        pos2(
            rect.left() + CHIP_PAD_X,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        c.text,
    );
    let removable = controls?;
    let mut left = rect.right() - CHIP_PAD_X * 0.5 - control_count * control_w;
    let mut edit = None;
    let offered = [
        (
            Icon::ChevronLeft,
            "earlier",
            index > 0,
            OrderedChipsEdit::MoveEarlier(index),
        ),
        (
            Icon::ChevronRight,
            "later",
            index + 1 < count,
            OrderedChipsEdit::MoveLater(index),
        ),
        (Icon::Close, "remove", true, OrderedChipsEdit::Remove(index)),
    ];
    for (icon, verb, available, chosen) in offered
        .into_iter()
        .filter(|(_, verb, _, _)| removable || *verb != "remove")
    {
        let control = Rect::from_min_size(pos2(left, rect.top()), vec2(control_w, rect.height()));
        left += control_w;
        let label = if verb == "remove" {
            format!("Remove {value} from {list}")
        } else {
            format!("Move {value} {verb} in {list}")
        };
        let response = ui.interact(
            control,
            id.with(verb),
            if available {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, available, &label));
        if available && response.hovered() {
            ui.painter()
                .rect_filled(control.shrink(2.0), t.radius, c.bg_hover);
        }
        icon.paint(
            ui.painter(),
            Rect::from_center_size(control.center(), vec2(11.0, 11.0)),
            if !available {
                c.text_faint.gamma_multiply(0.5)
            } else if response.hovered() || response.has_focus() {
                c.text
            } else {
                c.text_dim
            },
        );
        if available {
            theme::paint_focus_ring(ui, &response, control);
            if response.clicked() {
                edit = Some(chosen);
            }
            response.on_hover_text(label);
        }
    }
    edit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn edits_reorder_remove_and_add_without_duplicates() {
        let mut values = names(&["schematic", "extracted", "spice"]);
        assert!(OrderedChipsEdit::MoveLater(0).apply(&mut values));
        assert_eq!(values, names(&["extracted", "schematic", "spice"]));
        assert!(OrderedChipsEdit::MoveEarlier(2).apply(&mut values));
        assert_eq!(values, names(&["extracted", "spice", "schematic"]));
        assert!(OrderedChipsEdit::Remove(1).apply(&mut values));
        assert_eq!(values, names(&["extracted", "schematic"]));
        assert!(!OrderedChipsEdit::Add("Schematic".to_owned()).apply(&mut values));
        assert!(OrderedChipsEdit::Add("veriloga".to_owned()).apply(&mut values));
        assert_eq!(values, names(&["extracted", "schematic", "veriloga"]));
        for out_of_range in [
            OrderedChipsEdit::MoveEarlier(0),
            OrderedChipsEdit::MoveLater(2),
            OrderedChipsEdit::Remove(3),
        ] {
            assert!(!out_of_range.apply(&mut values), "{out_of_range:?}");
        }
    }

    #[test]
    fn every_control_is_a_named_button_and_read_only_chips_have_none() {
        let values = names(&["schematic", "spice"]);
        // The first chip cannot move earlier and the last cannot move later.
        for (enabled, expected_buttons) in [(true, 4), (false, 0)] {
            let ctx = egui::Context::default();
            crate::Theme::default().apply(&ctx);
            ctx.enable_accesskit();
            let output = ctx.run_ui(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ordered_chips(ui, "views", "View list", "View", &values, &[], enabled);
                });
            });
            let nodes = output.platform_output.accesskit_update.unwrap().nodes;
            let buttons = nodes
                .iter()
                .filter(|(_, node)| {
                    node.role() == egui::accesskit::Role::Button && !node.is_disabled()
                })
                .count();
            assert_eq!(buttons, expected_buttons, "enabled = {enabled}");
            assert!(
                nodes
                    .iter()
                    .any(|(_, node)| node.value() == Some("View list 2: spice"))
            );
        }
    }

    /// Under a finger each control of a chip is a touch target wide, so the
    /// three of them side by side are three targets, not one.
    #[test]
    fn each_control_takes_the_touch_target_under_a_finger() {
        let values = names(&["schematic", "spice"]);
        let ctx = egui::Context::default();
        crate::Theme::default().apply_responsive_metrics_with_target(&ctx, Some(48.0));
        ctx.enable_accesskit();
        let output = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ordered_chips(ui, "views", "View list", "View", &values, &[], true);
            });
        });
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        let controls = nodes
            .iter()
            .filter(|(_, node)| {
                node.role() == egui::accesskit::Role::Button
                    && node.label().is_some_and(|label| {
                        label.starts_with("Move ") || label.starts_with("Remove ")
                    })
            })
            .filter_map(|(_, node)| node.bounds())
            .collect::<Vec<_>>();
        assert_eq!(controls.len(), 6, "three controls on each of two chips");
        for bounds in controls {
            assert!(
                bounds.width() >= 48.0 - 0.5 && bounds.height() >= 48.0 - 0.5,
                "a control is {} by {}",
                bounds.width(),
                bounds.height()
            );
        }
    }

    /// A list that may never be empty offers no Remove on its only chip, and
    /// one on each chip while it holds more.
    #[test]
    fn a_required_list_offers_no_remove_on_its_last_chip() {
        for (values, removes) in [
            (names(&["schematic"]), 0),
            (names(&["schematic", "spice"]), 2),
        ] {
            let ctx = egui::Context::default();
            crate::Theme::default().apply(&ctx);
            ctx.enable_accesskit();
            let output = ctx.run_ui(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    required_ordered_chips(ui, "views", "View list", "View", &values, &[], true);
                });
            });
            let nodes = output.platform_output.accesskit_update.unwrap().nodes;
            let offered = nodes
                .iter()
                .filter(|(_, node)| {
                    node.label()
                        .is_some_and(|label| label.starts_with("Remove "))
                })
                .count();
            assert_eq!(offered, removes, "{values:?}");
        }
    }
}
