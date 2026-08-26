//! What a registry page states when it cannot state anything.
//!
//! Three of the four registry routes resolved their plan payload with
//! `unwrap_or_default()`, so a plan that will not resolve reached the page as a
//! plan holding nothing — an empty table with an empty-registry explanation
//! under it, in a workspace where no run could be described at all. The two
//! states have opposite fixes and the page said the same sentence for both.
//!
//! The same confusion in the other direction is what the record editors did
//! with a refused edit: the transaction was rejected, the plan was left exactly
//! as it was, and the field was cleared back to the stored value anyway — so
//! the reader was told their expression was wrong and simultaneously shown the
//! one it was meant to replace.

use egui::{Pos2, Rect, pos2, vec2};

use super::{RSpiceApp, SimulationPage, plan_id, render_with};

/// The routes that resolve a plan payload before drawing a registry, each with
/// a phrase from its own refusal card and column headings that exist only when
/// there is a real registry to head. The headings are spelled as `ledger_head`
/// paints them, which is upper case.
///
/// The headings are the assertion that matters: a page that renders its refusal
/// *and* its table has not fixed anything, because the table is still there to
/// be read as the plan's contents.
fn registries() -> [(SimulationPage, &'static str, [&'static str; 2]); 3] {
    [
        (
            SimulationPage::Outputs,
            "it is a plan nothing can be saved to",
            ["CAPTURE GROUP", "EST. SIZE"],
        ),
        (
            SimulationPage::Specifications,
            "it is a plan nothing can be authored against",
            ["DEFINITION", "MARGIN"],
        ),
        (
            SimulationPage::Save,
            "it is a plan with no retention to state",
            ["FORECAST", "INDETERMINATE"],
        ),
    ]
}

/// A plan that fails `stable_analysis_plan()` is a broken plan, not an empty
/// one, and each registry has to say so in its own subject's words.
#[test]
fn a_registry_over_an_unresolvable_plan_refuses_rather_than_reading_as_empty() {
    for (page, refusal, headings) in registries() {
        let rendered = render_with(page, 1200.0, |app| {
            app.state.sim_setup.analysis_plan = None;
        });
        assert!(
            rendered.contains("plan unavailable"),
            "{page:?} must badge the unresolved plan as an error:\n{rendered}"
        );
        assert!(
            rendered.contains(refusal),
            "{page:?} must say what it cannot state, in its own subject's words:\n{rendered}"
        );
        assert!(
            rendered.contains("has not been migrated to stable analysis-instance identity"),
            "{page:?} must carry the reason the plan would not resolve:\n{rendered}"
        );
        for heading in headings {
            assert!(
                !rendered.contains(heading),
                "{page:?} painted the {heading:?} column over a plan it could not resolve, so \
                 the refusal sits above a table that still reads as the plan's contents:\n\
                 {rendered}"
            );
        }
    }
}

/// The converse, which is what makes the refusal mean anything: a plan that
/// resolves to a payload holding nothing is a registry with nothing in it, and
/// those rows and that copy are deliberate.
#[test]
fn an_empty_registry_over_a_resolvable_plan_still_paints_its_table() {
    for (page, refusal, headings) in registries() {
        let rendered = render_with(page, 1200.0, |_| {});
        assert!(
            !rendered.contains(refusal),
            "{page:?} refused a plan it resolved:\n{rendered}"
        );
        for heading in headings {
            assert!(
                rendered.contains(heading),
                "{page:?} stopped painting its {heading:?} column over a resolvable plan:\n\
                 {rendered}"
            );
        }
    }
}

/// Specifications commits through `commit_plan_change` like every other
/// registry, and was the only route that never showed the log it wrote to — so
/// a successful edit here had no surface at all.
#[test]
fn the_specifications_route_shows_the_receipts_it_writes() {
    use crate::state::{SpecEntry, SpecPointScope};

    let rendered = render_with(SimulationPage::Specifications, 1200.0, |app| {
        let id = plan_id(app);
        app.state.workspace.replace_active_specs(
            id,
            vec![SpecEntry {
                measurement: "gain".to_owned(),
                expression: "meas ac gain max V(out)".to_owned(),
                min: Some(10.0),
                max: None,
                unit: "dB".to_owned(),
                scope: SpecPointScope::AllPoints,
            }],
        );
        super::super::page_specs::commit_scope(app, "gain", SpecPointScope::Nominal);
    });

    assert!(
        rendered.contains("Plan configuration receipts"),
        "the Specifications route must show the plan's receipt log:\n{rendered}"
    );
    assert!(
        rendered.contains("Scoped specification gain to nominal only."),
        "and the change this route just committed must be a row in it:\n{rendered}"
    );
}

/// One registry route, driven a frame at a time with real input events.
///
/// The rule under test is about focus, and focus only exists once the field is
/// a real widget on a real surface — so the fields are found by the name they
/// announce to AccessKit and reached by clicking where they were painted.
struct Registry {
    ctx: egui::Context,
    app: RSpiceApp,
    fields: Vec<(String, Rect)>,
}

impl Registry {
    const SURFACE: (f32, f32) = (1280.0, 2600.0);

    /// Takes the seeded app by value rather than a seeding closure: a
    /// `&mut RSpiceApp` parameter is exactly what the layering ratchet counts,
    /// and a test harness has no more claim on the whole application than a
    /// handler does.
    fn open(page: SimulationPage, mut app: RSpiceApp) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        app.state.workbench.simulation_page = page;
        let mut registry = Self {
            ctx,
            app,
            fields: Vec::new(),
        };
        // Two settling passes: the surface resolves its content width against
        // the scrollbar track it reserves, which it only knows on a second.
        registry.pass(Vec::new());
        registry.pass(Vec::new());
        registry
    }

    fn pass(&mut self, events: Vec<egui::Event>) {
        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    Pos2::ZERO,
                    vec2(Self::SURFACE.0, Self::SURFACE.1),
                )),
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
                    "the record announces no field called {field:?}; it announces {:?}",
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
}

/// A design variable bounded to 1k…10k, so an expression outside that range is
/// refused by the workspace's own validation rather than by anything this test
/// arranges — the exact refusal the page's "Out of bounds · the edit is
/// refused" rule promises.
fn bounded_variable() -> crate::state::DesignVariable {
    use crate::state::{
        DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableRange,
        DesignVariableScope, DesignVariableSweepEligibility,
    };

    DesignVariable::new(
        "rload",
        "1kohm",
        DesignVariableQuantity::Resistance,
        DesignVariableScope::Testbench,
        "load resistance",
        Some(DesignVariableRange {
            minimum: "1kohm".to_owned(),
            maximum: "10kohm".to_owned(),
        }),
        DesignVariableSweepEligibility::FixedParameter,
        DesignVariableOverridePolicy::InheritOwnerOnly,
    )
    .expect("a bounded design variable")
}

/// An app whose plan owns one bounded variable, with that variable selected.
fn app_with_bounded_variable() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let id = plan_id(&app);
    app.state
        .workspace
        .add_design_variable(id, bounded_variable())
        .expect("the plan accepts a bounded variable");
    app.state.workbench.selected_design_variable = Some("rload".to_owned());
    app
}

/// A refused inline edit keeps the text that was typed.
///
/// The commit is one validated transaction, so a rejected expression leaves the
/// plan byte-for-byte as it was — and the editor then cleared its draft
/// regardless, snapping the field back to the stored value. The refusal toast
/// was the only surviving trace of what the engineer had written, and the fix
/// for a one-character mistake was to type the whole expression again.
#[test]
fn a_refused_variable_expression_stays_on_the_field_that_was_typed_into() {
    let mut registry = Registry::open(SimulationPage::Variables, app_with_bounded_variable());

    registry.retype("Expression", "100kohm");

    let stored = |registry: &Registry| {
        let id = plan_id(&registry.app);
        registry
            .app
            .state
            .workspace
            .plan_data(id)
            .expect("the plan owns its payload")
            .design_variables[0]
            .expression
            .clone()
    };

    assert!(
        registry
            .app
            .state
            .workbench
            .analysis_lifecycle_status
            .is_refusal(),
        "100kohm is outside the variable's own 1k…10k bound, so the workspace must refuse it"
    );
    assert_eq!(
        stored(&registry),
        "1kohm",
        "a refused transaction leaves the plan exactly as it was"
    );
    assert_eq!(
        registry
            .app
            .state
            .workbench
            .design_variable_expression_draft
            .as_deref(),
        Some("100kohm"),
        "and the field keeps the refused text, so correcting it is an edit rather than a retype"
    );

    // The draft has to survive the frames that follow the refusal too: it is
    // read back into the field on every one of them, and a clear on the next
    // frame is the same loss one frame later.
    registry.pass(Vec::new());
    assert_eq!(
        registry
            .app
            .state
            .workbench
            .design_variable_expression_draft
            .as_deref(),
        Some("100kohm"),
        "the refused text must still be there on the next frame"
    );
    assert_eq!(stored(&registry), "1kohm");
}

/// An accepted edit still clears its draft, which is what keeps the field
/// showing the stored record rather than a copy of it that can drift.
#[test]
fn an_accepted_variable_expression_clears_the_draft_it_committed() {
    let mut registry = Registry::open(SimulationPage::Variables, app_with_bounded_variable());

    registry.retype("Expression", "4700ohm");

    let id = plan_id(&registry.app);
    assert_eq!(
        registry
            .app
            .state
            .workspace
            .plan_data(id)
            .expect("the plan owns its payload")
            .design_variables[0]
            .expression,
        "4700ohm",
        "a value inside the bound commits"
    );
    assert_eq!(
        registry
            .app
            .state
            .workbench
            .design_variable_expression_draft,
        None,
        "an adopted edit leaves no draft behind"
    );
    assert!(
        !registry
            .app
            .state
            .workbench
            .analysis_lifecycle_status
            .is_refusal()
    );
}

/// The same rule on the Outputs record editor, whose name and expression are
/// one transaction and so share one outcome.
#[test]
fn a_refused_saved_output_edit_stays_on_the_fields_that_were_typed_into() {
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };

    let mut app = RSpiceApp::test_instance();
    let id = plan_id(&app);
    for name in ["vout", "vin"] {
        let output = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            name,
            "V(out)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("a valid saved output");
        app.state
            .workspace
            .add_saved_output(id, output)
            .expect("the plan accepts it");
    }
    app.state.workbench.selected_saved_output = Some("vout".to_owned());
    let mut registry = Registry::open(SimulationPage::Outputs, app);

    // A name the registry already holds: the workspace refuses a duplicate
    // case-insensitively, which is a refusal the engineer has to correct on the
    // field rather than re-derive from a cleared one.
    registry.retype("Name", "VIN");

    let id = plan_id(&registry.app);
    let stored: Vec<String> = registry
        .app
        .state
        .workspace
        .plan_data(id)
        .expect("the plan owns its payload")
        .saved_outputs
        .iter()
        .map(|output| output.name.clone())
        .collect();

    assert!(
        registry
            .app
            .state
            .workbench
            .analysis_lifecycle_status
            .is_refusal(),
        "a name that duplicates another output's must be refused"
    );
    assert_eq!(
        stored,
        vec!["vout".to_owned(), "vin".to_owned()],
        "a refused transaction leaves the registry exactly as it was"
    );
    assert_eq!(
        registry
            .app
            .state
            .workbench
            .saved_output_name_draft
            .as_deref(),
        Some("VIN"),
        "and the field keeps the refused name"
    );
    assert_eq!(
        registry
            .app
            .state
            .workbench
            .selected_saved_output
            .as_deref(),
        Some("vout"),
        "the selection stays on the record that is still there"
    );

    registry.pass(Vec::new());
    assert_eq!(
        registry
            .app
            .state
            .workbench
            .saved_output_name_draft
            .as_deref(),
        Some("VIN"),
        "the refused name must still be there on the next frame"
    );
}
