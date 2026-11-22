//! The inspector's parameter form drawn through the whole application: the
//! component editor's groups and fold, its typed editors, a refusal under the
//! row it belongs to, and Browse beside a model.

use crate::workbench::RSpiceApp;
use rspice_app_design::workbench::design::inspector::component::{
    BROWSE_MODELS, SOURCE_FUNCTION, identity_section, parameters_section,
};
use rspice_app_design::workbench::design::inspector::inline_instance::field_value;
use rspice_app_design::workbench::design::inspector::model_evidence::component_model_evidence;
use rspice_app_state::state::{Component, ComponentType, Point};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::state::console_views::InlineEditField;
use std::collections::HashMap;

/// One painted run of text and where it was painted.
type Painted = Vec<(String, egui::Rect)>;

fn painted_text(output: &egui::FullOutput) -> Painted {
    fn walk(shape: &egui::epaint::Shape, into: &mut Painted) {
        match shape {
            egui::epaint::Shape::Text(text) => into.push((
                text.galley.text().to_owned(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, into);
                }
            }
            _ => {}
        }
    }
    let mut painted = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut painted);
    }
    painted
}

/// A component's Identity and Parameters sections at the dock's width,
/// driven by pointer and keyboard events.
struct Panel {
    ctx: egui::Context,
    app: RSpiceApp,
    id: u64,
    controls: HashMap<String, egui::Rect>,
    painted: Painted,
}

impl Panel {
    fn new(mut app: RSpiceApp, id: u64) -> Self {
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        app.state.workbench.activate(Workspace::Design);
        let mut panel = Self {
            ctx,
            app,
            id,
            controls: HashMap::new(),
            painted: Vec::new(),
        };
        // The first pass only measures.
        panel.pass(Vec::new());
        panel.pass(Vec::new());
        panel
    }

    fn with_part(kind: ComponentType) -> Self {
        let mut app = RSpiceApp::test_instance();
        let id = app
            .state
            .model
            .schematic
            .add_component(kind, Point::origin());
        app.state.model.schematic.clear_undo_history();
        Self::new(app, id)
    }

    fn component(&self) -> Component {
        self.app
            .state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == self.id)
            .cloned()
            .expect("the inspected part")
    }

    fn pass(&mut self, events: Vec<egui::Event>) {
        let component = self.component();
        let evidence = component_model_evidence(&self.app.state.model, &component);
        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(312.0, 4_000.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        identity_section(ui, &mut app.design_cx(), &component);
                        parameters_section(ui, &mut app.design_cx(), &component, &evidence);
                    });
            },
        );
        self.painted = painted_text(&output);
        if let Some(update) = output.platform_output.accesskit_update {
            for (_, node) in update.nodes {
                if let (Some(label), Some(bounds)) = (node.label(), node.bounds()) {
                    self.controls.insert(
                        label.to_owned(),
                        egui::Rect::from_min_max(
                            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                        ),
                    );
                }
            }
        }
    }

    /// The control whose accessible name starts with `label`: a captioned
    /// field is named with its unit after it.
    fn control(&self, label: &str) -> egui::Rect {
        self.controls
            .iter()
            .filter(|(name, _)| name.starts_with(label))
            .min_by_key(|(name, _)| name.len())
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("no control named {label}: {:?}", self.controls.keys()))
    }

    fn click_at(&mut self, pos: egui::Pos2) {
        self.pass(vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ]);
        self.pass(vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }]);
        self.pass(Vec::new());
    }

    fn click(&mut self, label: &str) {
        let pos = self.control(label).center();
        self.click_at(pos);
    }

    fn type_text(&mut self, text: &str) {
        self.pass(vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: Some(egui::Key::A),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            },
            egui::Event::Text(text.to_owned()),
        ]);
        self.pass(Vec::new());
    }

    fn painted(&self, text: &str) -> egui::Rect {
        self.painted
            .iter()
            .find(|(painted, _)| painted == text)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("{text:?} was not painted: {:?}", self.texts()))
    }

    fn texts(&self) -> Vec<String> {
        self.painted.iter().map(|(text, _)| text.clone()).collect()
    }
}

/// One list read in two overlapping views of it: `first`, then what `second`
/// shows past the longest run where the end of one is the start of the other.
fn joined(first: Vec<String>, second: Vec<String>) -> Vec<String> {
    let overlap = (0..=first.len().min(second.len()))
        .rev()
        .find(|&count| first[first.len() - count..] == second[..count])
        .unwrap_or(0);
    first
        .into_iter()
        .chain(second.into_iter().skip(overlap))
        .collect()
}

/// The inspector shows a MOSFET in the component editor's groups, in its
/// order, with the same fields folded under Advanced: one function builds
/// both, over the same values and the same account of what the bound card
/// reads.
#[test]
fn the_inspector_shows_a_mosfet_in_the_component_editors_groups() {
    let mut app = RSpiceApp::test_instance();
    rspice_app_state::workbench::examples::load_example(
        "CMOS Inverter",
        &mut app.state.model.schematic,
    );
    let mn = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name == "MN")
        .expect("the inverter has an MN")
        .id;
    let sheet = app
        .state
        .design
        .property_registry
        .get(ComponentType::Nmos)
        .cloned()
        .expect("the MOSFET sheet");
    let names: std::collections::HashSet<String> = sheet
        .iter()
        .filter(|definition| definition.name != "name")
        .flat_map(|definition| [definition.category.clone(), definition.display_name.clone()])
        .chain(["Advanced".to_owned()])
        .collect();
    let named = |painted: Vec<String>| {
        painted
            .into_iter()
            .map(|text| text.trim_end_matches(" *").to_owned())
            .filter(|text| names.contains(text))
            .collect::<Vec<_>>()
    };

    // The component editor, with its fold open, read down its whole form:
    // its body scrolls, so the form is read a view at a time.
    app.state.open_property_editor(mn);
    assert!(app.state.design.tabbed_property_dialog.open);
    app.state.design.tabbed_property_dialog.draft.show_advanced = true;
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut view = |events: Vec<egui::Event>| {
        let mut painted = Vec::new();
        for events in [events, Vec::new(), Vec::new()] {
            let output = ctx.run_ui(super::property_dialog_drawn::dialog_input(events), |ctx| {
                super::property_dialog_drawn::render_property_dialog(ctx, &mut app.state);
            });
            painted = painted_text(&output);
        }
        painted
    };
    let first = view(Vec::new());
    let over_the_form = first
        .iter()
        .find(|(text, _)| text.starts_with("Width"))
        .map(|(_, rect)| rect.center())
        .expect("the form shows a width");
    let mut editor = named(first.into_iter().map(|(text, _)| text).collect());
    for _ in 0..12 {
        let next = named(
            view(vec![
                egui::Event::PointerMoved(over_the_form),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, -240.0),
                    phase: egui::TouchPhase::Move,
                    modifiers: egui::Modifiers::NONE,
                },
            ])
            .into_iter()
            .map(|(text, _)| text)
            .collect(),
        );
        editor = joined(editor, next);
    }
    app.state.design.tabbed_property_dialog.close();

    // The inspector, with its fold open.
    let mut panel = Panel::new(app, mn);
    if let Some(fold) = panel
        .controls
        .keys()
        .find(|name| name.starts_with("Advanced"))
        .cloned()
    {
        panel.click(&fold);
    }
    let inspector = named(panel.texts());

    for expected in ["Model", "Geometry", "Width", "Length", "Temperature"] {
        assert!(editor.iter().any(|text| text == expected), "{editor:?}");
    }
    assert!(
        !inspector.iter().any(|text| text == "# fingers"),
        "a LEVEL=1 card reads no finger count: {inspector:?}"
    );
    assert_eq!(inspector, editor);
}

/// A refused value's reason is painted under the row it belongs to, not
/// after the last row of the section.
#[test]
fn a_refused_value_is_explained_under_its_own_row() {
    let mut panel = Panel::with_part(ComponentType::Resistor);
    panel.click("Resistance");
    panel.type_text("1k+");
    let reason = panel
        .app
        .state
        .workbench
        .inline_edit
        .session()
        .and_then(|session| session.error.clone())
        .expect("1k+ is refused");
    let row = panel.painted("Resistance");
    let strip = panel.painted(&reason);
    let next = panel.painted("Multiplier");
    assert!(
        strip.top() >= row.bottom() && strip.bottom() <= next.top(),
        "the reason {strip:?} sits between its row {row:?} and the next {next:?}"
    );
    assert_eq!(
        panel.component().value,
        "1k",
        "a refused value is never written"
    );
}

/// A yes/no property is a check box and a property with options is a
/// drop-down, as in the component editor, and each commits when it is
/// clicked, as one step of the design's history.
#[test]
fn a_check_box_and_a_drop_down_commit_when_clicked() {
    let mut panel = Panel::with_part(ComponentType::Resistor);
    let noisy = InlineEditField::Parameter("noisy".to_owned());
    let before = field_value(&panel.component(), &noisy);
    panel.click("Noisy");
    let after = field_value(&panel.component(), &noisy);
    assert!(
        ["0", "no", "false"].contains(&after.as_str()),
        "the resistor is no longer noisy: {before:?} became {after:?}"
    );
    assert!(panel.app.state.model.schematic.undo());
    assert_eq!(field_value(&panel.component(), &noisy), before);
    assert!(!panel.app.state.model.schematic.can_undo());

    let mut panel = Panel::with_part(ComponentType::Capacitor);
    assert_eq!(panel.component().symbol_variant, None);
    panel.click("Symbol");
    panel.click("polarized");
    assert_eq!(
        panel.component().symbol_variant.as_deref(),
        Some("polarized")
    );
    assert!(panel.app.state.model.schematic.undo());
    assert_eq!(panel.component().symbol_variant, None);
    assert!(!panel.app.state.model.schematic.can_undo());
}

/// The part's value is the row of the property it is, under that property's
/// name: a sine source's offset says so, and the Identity group has no row
/// named Value to type an offset into by mistake.
#[test]
fn the_value_row_is_named_by_its_property() {
    let panel = Panel::with_part(ComponentType::VoltageSourceSin);
    let texts = panel.texts();
    assert!(
        !texts.iter().any(|text| text == "Value"),
        "no row is named Value: {texts:?}"
    );
    let sheet = panel
        .app
        .state
        .design
        .property_registry
        .get(ComponentType::VoltageSourceSin)
        .cloned()
        .expect("the sine sheet");
    let offset = sheet.get("vo").expect("the offset").display_name.clone();
    assert!(texts.contains(&offset), "{offset} is offered: {texts:?}");
}

/// Browse beside a transistor's model opens the component editor with its
/// model browser over it, so a model is picked from the libraries, not typed
/// from memory.
#[test]
fn browse_beside_a_model_opens_the_model_browser() {
    let mut panel = Panel::with_part(ComponentType::Nmos);
    assert!(
        panel.texts().iter().any(|text| text == "Model"),
        "{:?}",
        panel.texts()
    );
    panel.click(BROWSE_MODELS);
    let dialog = &panel.app.state.design.tabbed_property_dialog;
    assert!(dialog.open, "the component editor opens");
    assert!(dialog.model_browser.open, "its model browser opens");
}

/// A source's function is a property: choosing SIN on a DC source swaps it
/// in place, one undo step, keeping its name, position and wiring and the
/// values both functions take.
#[test]
fn a_sources_function_changes_in_place() {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::VoltageSource, Point::origin());
    let placed = app.state.model.schematic.document().components[0].clone();
    let (_, pin) = placed.terminal_positions()[0];
    let wire = app
        .state
        .model
        .schematic
        .add_wire(vec![pin, Point::new(pin.x - 40, pin.y)])
        .expect("a wire on the pin");
    {
        let source = &mut app.state.model.schematic.document_mut_for_test().components[0];
        source.value = "3.3".to_owned();
        source.params = "ac=1 rs=50".to_owned();
    }
    app.state.model.schematic.clear_undo_history();
    let before = app.state.model.schematic.document().clone();
    let mut panel = Panel::new(app, id);

    panel.click(SOURCE_FUNCTION);
    panel.click("SIN");
    let sine = panel.component();
    assert_eq!(sine.kind, ComponentType::VoltageSourceSin);
    assert_eq!(
        (sine.id, sine.name.as_str()),
        (placed.id, placed.name.as_str())
    );
    assert_eq!(sine.pos, placed.pos);
    assert_eq!(sine.terminal_positions()[0].1, pin);
    let document = panel.app.state.model.schematic.document();
    let kept = document
        .wires
        .iter()
        .find(|candidate| candidate.id == wire)
        .expect("the wire stays");
    assert!(kept.points.contains(&pin), "{kept:?}");
    let params = rspice_app_state::state::parse_params_string(&sine.params);
    assert_eq!(params.get("ac").map(String::as_str), Some("1"));
    assert!(
        !params.contains_key("dc"),
        "a sine has no DC level: {params:?}"
    );

    assert!(panel.app.state.model.schematic.undo());
    assert_eq!(
        panel.app.state.model.schematic.document().components,
        before.components
    );
    assert!(!panel.app.state.model.schematic.can_undo());
}
