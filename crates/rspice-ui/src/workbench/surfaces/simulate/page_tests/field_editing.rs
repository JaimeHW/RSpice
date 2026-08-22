//! What an analysis form's fields hold after they are typed into.
//!
//! Driven through the route rather than through the row functions: the rule
//! under test is about focus, and focus only exists once the field is a real
//! widget on a real surface. The fields are found by the name they announce,
//! which is the same name the accessibility sweep next door asserts they have.

use egui::{Pos2, Rect, pos2, vec2};

use super::{RSpiceApp, SimulationPage};
use crate::simulation::plan::{AnalysisDraft, AnalysisInstance, AnalysisKind};

const SURFACE: (f32, f32) = (1280.0, 1400.0);

/// One studio route, driven a frame at a time with real input events.
struct Route {
    ctx: egui::Context,
    app: RSpiceApp,
    fields: Vec<(String, Rect)>,
}

impl Route {
    fn analyses(kind: AnalysisKind) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.simulation_page = SimulationPage::Analyses;
        let selected = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .ok()
            .and_then(|plan| {
                plan.instances()
                    .iter()
                    .find(|instance| instance.kind() == kind)
                    .map(AnalysisInstance::id)
            })
            .unwrap_or_else(|| panic!("the test plan holds a {kind:?}"));
        app.state.workbench.active_analysis_instance = Some(selected);
        let mut route = Self {
            ctx,
            app,
            fields: Vec::new(),
        };
        // Two settling passes: the surface resolves its content width against
        // the scrollbar track it reserves, which it only knows on a second.
        route.pass(Vec::new());
        route.pass(Vec::new());
        route
    }

    fn pass(&mut self, events: Vec<egui::Event>) {
        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(SURFACE.0, SURFACE.1))),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| super::super::show(ui, app));
            },
        );
        self.fields = output
            .platform_output
            .accesskit_update
            .map(|update| {
                update
                    .nodes
                    .iter()
                    .filter(|(_, node)| node.role() == egui::accesskit::Role::TextInput)
                    .filter_map(|(_, node)| {
                        let label = node.label()?.to_owned();
                        let bounds = node.bounds()?;
                        Some((
                            label,
                            Rect::from_min_max(
                                pos2(bounds.x0 as f32, bounds.y0 as f32),
                                pos2(bounds.x1 as f32, bounds.y1 as f32),
                            ),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();
    }

    fn click(&mut self, at: Pos2) {
        self.pass(vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ]);
        self.pass(Vec::new());
    }

    /// Select the whole of the named field, replace it with `text`, and move
    /// focus off it the way Tab does.
    fn retype(&mut self, field: &str, text: &str) {
        let rect = self
            .fields
            .iter()
            .find(|(label, _)| label == field)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| {
                panic!(
                    "the form announces no field called {field:?}; it announces {:?}",
                    self.fields
                        .iter()
                        .map(|(label, _)| label.as_str())
                        .collect::<Vec<_>>()
                )
            });
        self.click(rect.center());
        self.pass(vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            },
            egui::Event::Text(text.to_owned()),
        ]);
        self.pass(vec![egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }]);
        self.pass(Vec::new());
    }

    fn transient_stop(&self) -> String {
        let plan = self
            .app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the plan resolves");
        let instance = plan
            .instances()
            .iter()
            .find(|instance| instance.kind() == AnalysisKind::Transient)
            .expect("the plan holds a transient");
        match instance.draft() {
            AnalysisDraft::Transient(setup) => setup.stop.clone(),
            other => panic!("a transient instance holds {other:?}"),
        }
    }
}

/// The two input policies a reader can be working under.
///
/// `StrictUnitsRequired` is the default and makes a bare number invalid in a
/// time or frequency field; `InferFromFieldQuantity` accepts one. Both reach
/// the same normalization, and each makes a different half of it reachable, so
/// both are asked.
fn policies() -> [crate::quantity::QuantityPresentationPolicy; 2] {
    use crate::quantity::{QuantityPresentationPolicy, TimeFrequencyInput};
    let strict = QuantityPresentationPolicy::default();
    let inferring = QuantityPresentationPolicy {
        time_frequency_input: TimeFrequencyInput::InferFromFieldQuantity,
        ..strict
    };
    [strict, inferring]
}

/// A spelling the deck already reads is kept exactly as it was typed.
///
/// Blurring rewrote every quantity field that parsed to `{value:.17e}`, so a
/// reader working under `InferFromFieldQuantity` — where a bare number is a
/// valid time — typed `1e-3` into Stop time and got `1.00000000000000005e-3`
/// back the moment focus moved. The row's own helper text says "engineering
/// notation"; a field that will not hold it is the one thing that cannot be
/// true there.
#[test]
fn a_spelling_the_deck_already_reads_survives_the_blur() {
    use crate::quantity::{QuantityInputKind, UiNumberLocale};

    for policy in policies() {
        for spelling in ["1e-3", "0.25", "2500u", "10n"] {
            let mut value = spelling.to_owned();
            let rewritten = super::super::analysis_form::normalize_quantity(
                &mut value,
                QuantityInputKind::Time,
                policy,
                UiNumberLocale::default(),
            );
            assert!(
                !rewritten && value == spelling,
                "{spelling:?} became {value:?}: the deck reads it as typed, so nothing here \
                 may restate it"
            );
        }
    }
}

/// A spelling only the form understands is rewritten — in the deck's own
/// notation, never a 17-digit exponent.
///
/// The field holds the text the deck is generated from, and
/// `parse_spice_value_checked` does not know the unit letters the form accepts:
/// `5ms` reaches it as an unsupported suffix, so a field left holding it reads
/// one value and runs none.
#[test]
fn a_spelling_the_deck_cannot_read_is_rewritten_in_engineering_notation() {
    let mut route = Route::analyses(AnalysisKind::Transient);
    route.retype("Stop time", "5ms");
    let rewritten = route.transient_stop();
    assert_eq!(
        rewritten,
        crate::state::format_engineering(5.0e-3),
        "the rewrite goes through the deck's own formatter"
    );
    assert!(
        !rewritten.contains('e'),
        "a 17-digit exponent is not engineering notation: {rewritten}"
    );
    assert!(
        crate::simulation::spice_value::parse_spice_value_checked(&rewritten).is_ok(),
        "whatever is left behind has to be a value the deck can read: {rewritten}"
    );
}

/// A field the form cannot parse at all keeps what was typed, so the reader can
/// see the thing they have to correct.
#[test]
fn an_unparseable_field_is_left_for_its_reader_to_correct() {
    use crate::quantity::{QuantityInputKind, UiNumberLocale};

    let mut value = "banana".to_owned();
    assert!(!super::super::analysis_form::normalize_quantity(
        &mut value,
        QuantityInputKind::Time,
        crate::quantity::QuantityPresentationPolicy::default(),
        UiNumberLocale::default(),
    ));
    assert_eq!(value, "banana");
}

/// The temperature field's draft is bare Celsius, so a unit typed into it is
/// converted rather than kept.
#[test]
fn a_temperature_with_a_unit_is_converted_to_the_draft_the_deck_reads() {
    use crate::quantity::{QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale};

    let mut value = "25 \u{b0}C".to_owned();
    assert!(
        super::super::analysis_form::normalize_quantity(
            &mut value,
            QuantityInputKind::Temperature,
            QuantityPresentationPolicy::default(),
            UiNumberLocale::default(),
        ),
        "a unit the deck cannot read has to be rewritten"
    );
    assert_eq!(value, crate::state::format_engineering(25.0));
}
