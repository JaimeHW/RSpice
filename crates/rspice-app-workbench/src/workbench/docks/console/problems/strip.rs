//! The filter strip: what the grid is narrowed to, and what each control
//! would list if the reader moved it.
//!
//! Every count printed here is the count of rows that control would list with
//! every *other* control exactly where it stands. That is the only definition
//! under which the numbers stay true as the reader works: a severity count
//! taken over the whole list would promise rows that the producer filter beside
//! it then withholds.
//!
//! A severity with nothing in it and nothing hidden earns no control. One that
//! is switched off keeps its control whatever its count, because a reader who
//! hid a rung has to be able to find it again.
//!
//! The strip is one line or one menu. Where every control fits on the line at
//! the width it needs to say its longest choice whole, the controls stand
//! there. Where they do not, the toggles and both choices fold into one
//! Filters menu beside the text filter, so no control prints a cut word.

use egui::containers::menu::{MenuButton, MenuConfig};
use egui::{Align, Layout, PopupCloseBehavior, Ui};

use super::super::CONSOLE_COLUMN_GAP;
use super::arrange::Tally;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    chip_at_width, chip_width, menu_button, select_mono_width, select_mono_with_response,
};
use crate::workbench::app_state::diagnostic_store::DiagnosticLevel;
use crate::workbench::state::problems_view::{ProblemGrouping, ProblemsView};

/// What the producer control says when the grid is narrowed to none.
const EVERY_PRODUCER: &str = "All producers";

/// What the text filter says while it is empty: what it matches.
const FILTER_HINT: &str = "rule, subject, message";

/// What the menu the strip folds into says.
const FILTERS: &str = "Filters";

/// The text filter's margin around its text: egui's own, stated, because the
/// field's width is reckoned from it.
const FILTER_MARGIN: egui::Margin = egui::Margin::symmetric(4, 2);

/// The space between two controls on the strip's line.
const CONTROL_GAP: f32 = 6.0;

/// The space between the strip's line and the line under it.
const LINE_GAP: f32 = 4.0;

/// A toggle on the strip: what it says, whether what it names is listed, and
/// what pressing it switches.
struct Toggle {
    label: String,
    on: bool,
    switches: Switch,
}

#[derive(Clone, Copy)]
enum Switch {
    Severity(DiagnosticLevel),
    Waived,
}

impl Switch {
    fn flip(self, view: &mut ProblemsView) {
        match self {
            Self::Severity(level) => view.severities.toggle(level),
            Self::Waived => view.waived = !view.waived,
        }
    }
}

/// Every control of the strip, what it says, and how wide it stands.
///
/// Siblings stand at one width: every toggle at the widest toggle's, both
/// choices at the width of the longest thing either can say. A choice is
/// measured in the face it prints in, so none prints a cut word and picking
/// one never moves the controls beside it.
struct Controls {
    toggles: Vec<Toggle>,
    toggle_width: f32,
    /// Every producer choice: none first, then each producer present.
    producers: Vec<String>,
    /// The producer choice standing, spelled the way the list spells it.
    producer: String,
    groupings: Vec<String>,
    choice_width: f32,
    /// The hint whole, inside the field's margins, with a character to spare
    /// for the caret.
    field_width: f32,
}

impl Controls {
    fn measure(ui: &Ui, view: &ProblemsView, tally: &Tally) -> Self {
        let mut toggles: Vec<Toggle> = DiagnosticLevel::ALL
            .into_iter()
            .filter_map(|level| {
                let count = tally.severity(level);
                let listed = view.severities.lists(level);
                (count > 0 || !listed).then(|| Toggle {
                    label: format!("{} {count}", level.display_name()),
                    on: listed,
                    switches: Switch::Severity(level),
                })
            })
            .collect();
        if tally.waived > 0 || !view.waived {
            toggles.push(Toggle {
                label: format!("Waived {}", tally.waived),
                on: view.waived,
                switches: Switch::Waived,
            });
        }
        let toggle_width = toggles
            .iter()
            .map(|toggle| chip_width(ui, &toggle.label))
            .fold(0.0, f32::max);

        let mut producers = Vec::with_capacity(tally.producers.len() + 1);
        producers.push(EVERY_PRODUCER.to_owned());
        producers.extend(
            tally
                .producers
                .iter()
                .map(|(producer, count)| format!("{} {count}", super::producer_name(producer))),
        );
        // The field shows the option the reader picked, spelled the way the
        // list spells it, so the open list has something to match it against.
        let producer = view.producer.as_deref().map_or_else(
            || EVERY_PRODUCER.to_owned(),
            |producer| {
                tally
                    .producers
                    .iter()
                    .find(|(listed, _)| *listed == producer)
                    .map_or_else(
                        || super::producer_name(producer).into_owned(),
                        |(listed, count)| format!("{} {count}", super::producer_name(listed)),
                    )
            },
        );
        let groupings: Vec<String> = ProblemGrouping::ALL
            .iter()
            .map(|grouping| grouping.label().to_owned())
            .collect();
        let choice_width = select_mono_width(ui, &producers)
            .max(select_mono_width(ui, &groupings))
            .max(select_mono_width(ui, std::slice::from_ref(&producer)));

        let hint = ui
            .painter()
            .layout_no_wrap(
                format!("{FILTER_HINT}M"),
                theme::field_mono(),
                egui::Color32::PLACEHOLDER,
            )
            .size()
            .x;
        Self {
            toggles,
            toggle_width,
            producers,
            producer,
            groupings,
            choice_width,
            field_width: hint.ceil() + FILTER_MARGIN.sum().x,
        }
    }

    /// How wide the strip's line is with every control standing on it.
    fn line_width(&self) -> f32 {
        let toggles = self.toggles.len() as f32;
        let gaps = toggles + 2.0;
        self.toggle_width * toggles
            + 2.0 * self.choice_width
            + self.field_width
            + CONTROL_GAP * gaps
    }
}

/// Draw the strip, editing the reading position it is drawn from: every
/// control on one line where they all fit whole, else the toggles and choices
/// in one Filters menu beside the text filter.
pub(super) fn filter_strip(ui: &mut Ui, view: &mut ProblemsView, tally: &Tally) {
    let controls = Controls::measure(ui, view, tally);
    let fits = controls.line_width() <= ui.available_width();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = CONTROL_GAP;
        if fits {
            toggles(ui, view, &controls);
            producer_select(ui, view, tally, &controls);
            text_filter(ui, view, controls.field_width);
            grouping_select(ui, view, &controls);
        } else {
            filters_menu(ui, view, tally, &controls);
            text_filter(ui, view, controls.field_width);
        }
    });
}

fn toggles(ui: &mut Ui, view: &mut ProblemsView, controls: &Controls) {
    for toggle in &controls.toggles {
        if chip_at_width(ui, &toggle.label, toggle.on, controls.toggle_width).clicked() {
            toggle.switches.flip(view);
        }
    }
}

fn producer_select(ui: &mut Ui, view: &mut ProblemsView, tally: &Tally, controls: &Controls) {
    let picked = select_mono_with_response(
        ui,
        "workbench.problems.producer",
        "Producer",
        &controls.producer,
        &controls.producers,
        controls.choice_width,
    )
    .picked;
    if let Some(picked) = picked {
        view.producer = producer_at(tally, picked);
    }
}

fn grouping_select(ui: &mut Ui, view: &mut ProblemsView, controls: &Controls) {
    let picked = select_mono_with_response(
        ui,
        "workbench.problems.grouping",
        "Grouping",
        view.grouping.label(),
        &controls.groupings,
        controls.choice_width,
    )
    .picked;
    if let Some(picked) = picked
        && let Some(grouping) = ProblemGrouping::ALL.get(picked)
    {
        view.grouping = *grouping;
    }
}

/// The producer the `picked` choice narrows to: none for the first.
fn producer_at(tally: &Tally, picked: usize) -> Option<String> {
    picked
        .checked_sub(1)
        .and_then(|index| tally.producers.get(index))
        .map(|(producer, _)| producer.clone())
}

/// The text filter, in the face and well of the Console's find field.
fn text_filter(ui: &mut Ui, view: &mut ProblemsView, width: f32) {
    let response = ui.add_sized(
        [width, Tokens::get(ui.ctx()).metrics.ctl_h],
        egui::TextEdit::singleline(&mut view.text)
            .id(egui::Id::new("workbench.problems.text"))
            .hint_text(FILTER_HINT)
            .margin(FILTER_MARGIN)
            .vertical_align(egui::Align::Center)
            .font(theme::field_mono()),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("Filter findings by text");
    });
}

/// The strip folded into one menu: the toggles on a line of their own, then
/// the producer and the grouping, each a submenu of its choices. It stays
/// open while the reader works through it, as the line does, and closes on a
/// press outside it.
fn filters_menu(ui: &mut Ui, view: &mut ProblemsView, tally: &Tally, controls: &Controls) {
    let height = Tokens::get(ui.ctx()).metrics.ctl_h;
    let config = MenuConfig::new().close_behavior(PopupCloseBehavior::CloseOnClickOutside);
    let _ = MenuButton::from_button(egui::Button::new(FILTERS).min_size(egui::vec2(0.0, height)))
        .config(config)
        .ui(ui, |ui| {
            if !controls.toggles.is_empty() {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = CONTROL_GAP;
                    toggles(ui, view, controls);
                });
                ui.separator();
            }
            menu_button(ui, "Producer", |ui| {
                for (index, choice) in controls.producers.iter().enumerate() {
                    let row = ui.add(egui::Button::selectable(
                        *choice == controls.producer,
                        choice.as_str(),
                    ));
                    if row.clicked() {
                        view.producer = producer_at(tally, index);
                    }
                }
            });
            menu_button(ui, "Grouping", |ui| {
                for (grouping, choice) in ProblemGrouping::ALL.iter().zip(&controls.groupings) {
                    let row = ui.add(egui::Button::selectable(
                        view.grouping == *grouping,
                        choice.as_str(),
                    ));
                    if row.clicked() {
                        view.grouping = *grouping;
                    }
                }
            });
        });
}

/// What the strip says about a list the filters have emptied, and the control
/// that undoes it. A page that said only "no findings" here would be claiming
/// the design is clean when the reader had hidden what is wrong with it.
///
/// It is the toolbar's last line, as far under the controls as their own
/// lines are apart.
pub(super) fn filtered_out(ui: &mut Ui, listed: usize, total: usize) -> bool {
    let tokens = Tokens::get(ui.ctx());
    let mut cleared = false;
    ui.add_space(LINE_GAP);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = CONSOLE_COLUMN_GAP;
        ui.label(
            egui::RichText::new(format!(
                "{listed} of {total} finding{} listed",
                if total == 1 { "" } else { "s" }
            ))
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(tokens.color.text_dim),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let clear = ui.button("List everything");
            clear.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    ui.is_enabled(),
                    "Clear every Problems filter",
                )
            });
            cleared = clear.clicked();
        });
    });
    cleared
}

#[cfg(test)]
mod tests {
    use egui::accesskit;

    use super::*;

    /// Findings at every rung, some waived, from two producers.
    fn tally() -> Tally {
        Tally::of(
            [1, 12, 30, 4, 2],
            3,
            vec![
                ("rspice.netlist.parser".to_owned(), 41),
                ("rspice.design.rules".to_owned(), 8),
            ],
        )
    }

    /// What the strip, drawn alone `width` wide, printed and told assistive
    /// technology on its last pass.
    struct Drawn {
        texts: Vec<String>,
        nodes: Vec<(accesskit::NodeId, accesskit::Node)>,
    }

    impl Drawn {
        fn prints(&self, text: &str) -> bool {
            self.texts.iter().any(|printed| printed == text)
        }

        /// The width of the control assistive technology knows as `label`.
        fn width(&self, label: &str) -> f64 {
            let bounds = self
                .nodes
                .iter()
                .find(|(_, node)| node.label() == Some(label))
                .and_then(|(_, node)| node.bounds())
                .unwrap_or_else(|| panic!("no control is called {label}"));
            bounds.x1 - bounds.x0
        }
    }

    fn drawn(width: f32) -> Drawn {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut view = ProblemsView::default();
        let tally = tally();
        let mut output = None;
        // The first pass loads the faces; the strip is read once it settles.
        for _ in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(width, 200.0),
                )),
                ..Default::default()
            };
            output = Some(ctx.run_ui(input, |ui| filter_strip(ui, &mut view, &tally)));
        }
        let output = output.expect("a pass ran");
        let mut texts = Vec::new();
        for clipped in &output.shapes {
            texts_of(&clipped.shape, &mut texts);
        }
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("an AccessKit update")
            .nodes;
        Drawn { texts, nodes }
    }

    fn texts_of(shape: &egui::Shape, texts: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => texts.push(text.galley.text().to_owned()),
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    texts_of(shape, texts);
                }
            }
            _ => {}
        }
    }

    /// Every control on the line prints whole: a choice is as wide as its
    /// longest choice in the face it prints in, not in the smaller face of
    /// the toggles beside it.
    #[test]
    fn every_control_on_the_line_prints_whole() {
        let drawn = drawn(1400.0);
        for whole in ["All producers", "No grouping", FILTER_HINT, "Warning 30"] {
            assert!(drawn.prints(whole), "{whole} is cut: {:?}", drawn.texts);
        }
        assert!(
            !drawn.texts.iter().any(|text| text.contains('…')),
            "{:?}",
            drawn.texts
        );
        assert!(!drawn.prints(FILTERS), "{:?}", drawn.texts);
    }

    /// A strip too narrow for its line folds the toggles and both choices
    /// into one menu beside the text filter, rather than cutting what any of
    /// them says.
    #[test]
    fn a_strip_too_narrow_for_its_line_folds_its_filters_into_one_menu() {
        let drawn = drawn(560.0);
        assert!(drawn.prints(FILTERS), "{:?}", drawn.texts);
        assert!(drawn.prints(FILTER_HINT), "{:?}", drawn.texts);
        for folded in ["All producers", "No grouping", "Warning 30"] {
            assert!(!drawn.prints(folded), "{folded} is on the line");
        }
        assert!(
            !drawn.texts.iter().any(|text| text.contains('…')),
            "{:?}",
            drawn.texts
        );
    }

    /// Siblings stand at one width: every toggle at the widest toggle's, and
    /// the two choices at the width of the longest thing either can say.
    #[test]
    fn the_toggles_share_one_width_and_the_choices_another() {
        let drawn = drawn(1400.0);
        let toggles = [
            "Critical 1",
            "Error 12",
            "Warning 30",
            "Info 4",
            "Hint 2",
            "Waived 3",
        ]
        .map(|label| drawn.width(label));
        assert!(
            toggles.iter().all(|width| (width - toggles[0]).abs() < 0.5),
            "{toggles:?}"
        );
        assert!((drawn.width("Producer") - drawn.width("Grouping")).abs() < 0.5);
    }
}
