//! The Models workspace drawn through a whole application, a frame at a
//! time: the shell answers what the workspace raised once it has drawn, as
//! the frame does, so a control pressed here reaches whatever part carries
//! it out.

use std::path::PathBuf;

use egui::Color32;

use crate::state::model_library::{DeviceModel, ModelLibrary, ModelQualificationState, ModelType};
use crate::state::{Component, ComponentType, ModelsPage, Point};
use crate::workbench::models::ModelsFeature;
use crate::workbench::models::tests::painted_text;
use crate::workbench::models::workspace_state::ModelsOperationalState;
use crate::workbench::state::ModelsCatalogScope;
use crate::workbench::{RSpiceApp, features};

mod binned_family;
mod provider_dialog;

/// The Models page, rendered until its layout settles, holding both what it
/// published to AccessKit and what it painted.
///
/// A control found by the announcement it publishes and pressed in the middle
/// of the rectangle that announcement carried is what proves a painted cell is
/// both reachable and wired: this page's rows are drawn rather than built, so
/// a route with no node reaches nobody and a node with no handler reaches
/// nothing.
struct ModelsStudio {
    ctx: egui::Context,
    app: RSpiceApp,
    controls: Vec<(String, egui::Rect)>,
    painted: Vec<(String, Color32)>,
}

/// Wide enough for the mockup's four-column detail row, which is the layout
/// the approved design is drawn at.
const STUDIO_SIZE: egui::Vec2 = egui::vec2(1400.0, 1000.0);

impl ModelsStudio {
    fn open(app: RSpiceApp) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut studio = Self {
            ctx,
            app,
            controls: Vec::new(),
            painted: Vec::new(),
        };
        // Fonts build on the first pass, sizing settles on the second, and the
        // third is the steady-state frame these assertions read.
        for _ in 0..3 {
            studio.pass(Vec::new());
        }
        studio
    }

    fn pass(&mut self, events: Vec<egui::Event>) {
        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, STUDIO_SIZE)),
                events,
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        ModelsFeature::show_workspace(&mut features::cx(app), ui)
                    });
                // The shell answers what the workspace raised once it has
                // drawn.
                app.after_contribution();
            },
        );
        self.painted = painted_text(&output);
        self.controls = output
            .platform_output
            .accesskit_update
            .map(|update| {
                update
                    .nodes
                    .iter()
                    .filter_map(|(_, node)| {
                        let bounds = node.bounds()?;
                        Some((
                            node.label()?.to_owned(),
                            egui::Rect::from_min_max(
                                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                            ),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();
    }

    fn announcements(&self) -> Vec<&str> {
        self.controls
            .iter()
            .map(|(label, _)| label.as_str())
            .collect()
    }

    fn announces(&self, matches: impl Fn(&str) -> bool) -> bool {
        self.controls.iter().any(|(label, _)| matches(label))
    }

    /// Click the one control whose announcement satisfies `matches`.
    ///
    /// An ambiguous match is refused rather than resolved by taking the first:
    /// two controls answering one description means the test is not saying
    /// which of them it pressed.
    fn click(&mut self, matches: impl Fn(&str) -> bool) {
        let hits = self
            .controls
            .iter()
            .filter(|(label, _)| matches(label))
            .collect::<Vec<_>>();
        assert_eq!(
            hits.len(),
            1,
            "expected exactly one matching control; the page announces {:#?}",
            self.announcements()
        );
        let at = hits[0].1.center();
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
}

/// A project holding one file-backed model and one instance it can be bound
/// to, with the selection sitting on that instance.
fn one_model_project() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.model.model_library_manager.clear();
    let mut library = ModelLibrary::new("alpha");
    let mut model = DeviceModel::new("nch", ModelType::Nmos);
    model.file_path = Some(PathBuf::from("alpha.lib"));
    model.source_line = Some(41);
    model.parameters.insert("vto".to_owned(), 0.7);
    library.add_model(model);
    library
        .model_qualification
        .insert("nch".to_owned(), ModelQualificationState::default());
    app.state.model.model_library_manager.add_library(library);
    crate::workbench::models::workspace_state::select_library_or_prompt(
        &mut app.state.model,
        &mut app.state.workbench,
        "alpha",
    );
    app.state.models.selected_model = Some("nch".to_owned());
    app.state.models.models_page = ModelsPage::Models;

    let mut component = Component::new(1, ComponentType::Nmos, Point::origin());
    component.name = "M1".to_owned();
    component.params = "model=nch".to_owned();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(1);
    app
}

/// The page's one accent primary names the instance it would bind.
///
/// "Bind to selection…" beside a schematic with exactly one instance selected
/// makes a reader open the schematic to find out what the button is about. It
/// names the instance only while the binding would land, so the label never
/// claims a target the action is refusing.
#[test]
fn the_bind_primary_names_the_instance_it_would_bind() {
    let studio = ModelsStudio::open(one_model_project());
    assert!(
        studio.announces(|label| label == "Bind to M1…"),
        "the primary did not name the selected instance; the page announces {:#?}",
        studio.announcements()
    );

    // With nothing selected there is nothing to name, and the base spelling —
    // the one the control ratchet reads — is what comes back.
    let mut app = one_model_project();
    app.state.model.schematic.session.editor.selection.clear();
    let unselected = ModelsStudio::open(app);
    assert!(
        unselected.announces(|label| label == "Bind to selection…"),
        "the page announces {:#?}",
        unselected.announcements()
    );
}

/// A refused action is stated, once, on the page it was taken from.
///
/// Only the packs view drew a refusal, so a refused corner, symbol, bin or
/// include action was shown nowhere at all.
#[test]
fn a_refused_action_is_stated_on_the_page_it_was_taken_from() {
    let reason = "the system clock cannot timestamp model validation";
    for (page, scope) in [
        (ModelsPage::Models, ModelsCatalogScope::Project),
        (ModelsPage::Models, ModelsCatalogScope::InstalledPacks),
        (ModelsPage::Models, ModelsCatalogScope::RSpiceLibrary),
        (ModelsPage::Symbols, ModelsCatalogScope::Project),
        (ModelsPage::Corners, ModelsCatalogScope::Project),
        (ModelsPage::Bins, ModelsCatalogScope::Project),
        (ModelsPage::Include, ModelsCatalogScope::Project),
    ] {
        let mut app = RSpiceApp::test_instance();
        app.state.models.models_page = page;
        app.state.models.models_view.catalog_scope = scope;
        app.state
            .models
            .models_view
            .note_receipt(Err(reason.to_owned()));
        let consequence = app.state.models.models_view.operational_state.consequence();
        let studio = ModelsStudio::open(app);
        let stated = studio
            .announcements()
            .into_iter()
            .filter(|label| *label == reason)
            .count();
        assert_eq!(
            stated,
            1,
            "{page:?} / {} states the refusal once; the page announces {:#?}",
            scope.label(),
            studio.announcements()
        );
        assert!(
            studio.announces(|label| label == consequence),
            "{page:?} / {} says what the refusal left behind",
            scope.label()
        );
    }

    // An action that worked says nothing.
    let mut app = RSpiceApp::test_instance();
    app.state.models.models_page = ModelsPage::Corners;
    app.state
        .models
        .models_view
        .note_receipt(Ok("validated".to_owned()));
    let studio = ModelsStudio::open(app);
    assert!(!studio.announces(|label| label == ModelsOperationalState::Ready.consequence()));
}

/// A qualification count is a way into the page that holds it.
///
/// The card stated five numbers a reader could not act on where they were
/// shown, beside an action bar whose "Qualification" button went exactly
/// there. The counts are that same route rather than five new controls.
#[test]
fn a_qualification_count_opens_the_page_that_holds_it() {
    let mut studio = ModelsStudio::open(one_model_project());
    assert_eq!(
        studio.app.state.models.models_page,
        ModelsPage::Models,
        "the studio opens on the catalog"
    );

    studio.click(|label| label.starts_with("Suites:"));

    assert_eq!(
        studio.app.state.models.models_page,
        ModelsPage::Qualification
    );
}

/// A parameter states where the card that declares it is written.
///
/// "source card" alone is a claim a reader has no way to check; the file and
/// the line under it are what make the origin column worth its width.
#[test]
fn a_parameter_states_where_its_card_is_written() {
    let studio = ModelsStudio::open(one_model_project());
    for expected in ["source card", "alpha.lib:41"] {
        assert!(
            studio.painted.iter().any(|(text, _)| text == expected),
            "the parameter card never painted {expected:?}"
        );
    }
}
