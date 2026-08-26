//! Attribution and detachment both resolve to one project revision.
//!
//! A bound consumer is attributed to the *effective* provider rather than to
//! whichever library happens to declare the name, and detaching a pack takes
//! every library it attached with it in a single revision — a partial detach
//! leaves bindings pointing at a provider the project no longer has.

use super::*;
use crate::state::model_library::{ModelQualificationState, ModelType};
use crate::state::{Component, ComponentType, Point};

#[test]
fn bound_consumers_are_attributed_only_to_the_effective_provider() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    for name in ["alpha", "beta"] {
        let mut library = ModelLibrary::new(name);
        library.add_model(DeviceModel::new("nch", ModelType::Nmos));
        state.model_library_manager.add_library(library);
    }
    state
        .model_library_manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "nch",
            "alpha",
            "provider-aware consumer test",
        )
        .expect("the contested provider can be resolved");

    state.workspace.ensure_active_buffer();
    let mut schematic = state
        .workspace
        .active_schematic()
        .cloned()
        .expect("an active schematic exists");
    let mut component = Component::new(1, ComponentType::Nmos, Point::origin());
    component.name = "M1".to_owned();
    component.params = "model=nch model_library=alpha".to_owned();
    schematic.components.push(component);
    state.workspace.save_active_schematic(&schematic);

    let mut pending_actions = Vec::new();
    let render = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending_actions,
    };
    let consumers = ConsumerIndex::build(&render);
    let scan = project_catalog_scan(&render, &consumers);
    let alpha = scan
        .rows
        .iter()
        .find(|row| row.library == "alpha" && row.model == "nch")
        .expect("alpha catalog row");
    let beta = scan
        .rows
        .iter()
        .find(|row| row.library == "beta" && row.model == "nch")
        .expect("beta catalog row");

    // The column carries the designator, not the whole consumer label: the
    // kind and the coordinates are what the where-used list is for.
    assert_eq!(alpha.usage.as_deref(), Some("M1"));
    assert_eq!(alpha.usage_count, 1);
    assert_eq!(beta.usage, None);
    assert_eq!(beta.usage_count, 0);
    assert!(scan.consumer_diagnostics.is_empty());
}

#[test]
fn detaching_a_pack_removes_every_attached_library_in_one_revision() {
    let mut state = AppState::default();
    for (library_name, model_name) in [("pack-a", "na"), ("pack-b", "nb")] {
        let mut library = ModelLibrary::new(library_name);
        library.pack_id = Some("shared-pack".to_owned());
        library.add_model(DeviceModel::new(model_name, ModelType::Nmos));
        state.model_library_manager.add_library(library);
    }
    let initial_revision = state.workspace.project.revision();
    let initial_epoch = state.design_execution_epoch;
    let mut pending_actions = Vec::new();
    let mut render = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending_actions,
    };

    detach_pack(&mut render, "shared-pack");

    assert!(
        render
            .state
            .model_library_manager
            .get_library("pack-a")
            .is_none()
    );
    assert!(
        render
            .state
            .model_library_manager
            .get_library("pack-b")
            .is_none()
    );
    assert_eq!(
        render.state.workspace.project.revision().get(),
        initial_revision.get() + 1
    );
    assert_eq!(
        render.state.design_execution_epoch,
        initial_epoch.wrapping_add(1)
    );
}

#[test]
fn signed_technology_symbol_variant_is_authored_in_one_project_revision() {
    let mut state = AppState::default();
    state.provision_test_project_symbol_technology_contract();
    state
        .library_manager
        .add_library(crate::state::Library::new("signed_variant_test"));
    let initial_revision = state.workspace.project.revision();
    let package_digest = state
        .project_signed_technology_package()
        .expect("exact package resolves")
        .expect("project has package")
        .archive_digest();
    let mut pending_actions = Vec::new();
    let mut render = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending_actions,
    };

    let receipt = author_technology_symbol_variant(
        &mut render,
        "demo180",
        "nmos_demo",
        "signed_variant_test",
        "nmos_custom",
    )
    .expect("variant commits");

    assert!(receipt.contains("signed technology symbol 'demo180/nmos_demo'"));
    assert_eq!(
        render.state.workspace.project.revision().get(),
        initial_revision.get() + 1
    );
    let view = render
        .state
        .library_manager
        .get_library("signed_variant_test")
        .and_then(|library| library.get_cell("nmos_custom"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("variant symbol view exists");
    let definition = ModelBoundSymbolDefinition::load_from_view(view)
        .expect("metadata loads")
        .expect("typed definition exists");
    assert_eq!(definition.identity.library, "signed_variant_test");
    assert_eq!(definition.identity.cell, "nmos_custom");
    assert_eq!(definition.identity.revision, 1);
    let model = definition
        .netlist
        .model
        .as_ref()
        .expect("signed model binding");
    assert_eq!(model.library, "signed-pdk:demo-models-tt");
    assert!(
        model
            .source_path
            .as_deref()
            .is_some_and(|path| path.contains(&package_digest.to_string()))
    );
    definition
        .validate()
        .expect("project variant stays executable");

    let committed_revision = render.state.workspace.project.revision();
    assert!(
        author_technology_symbol_variant(
            &mut render,
            "demo180",
            "nmos_demo",
            "signed_variant_test",
            "nmos_custom",
        )
        .is_err()
    );
    assert_eq!(
        render.state.workspace.project.revision(),
        committed_revision,
        "a rejected overwrite must not publish a partial transaction"
    );
}

#[test]
fn the_pin_action_names_whichever_reason_actually_blocks_it() {
    // Two conditions disable the pin and only one hover text existed, so a
    // user who had started an import was told the external library in front of
    // them was built in.
    const BUILT_IN: &str = "Built-in sources do not have an external file to pin.";
    const BUSY: &str = "Another model-source operation is still running.";

    assert_eq!(pin_source_block_reason(true, false), None);
    assert_eq!(pin_source_block_reason(true, true), Some(BUSY));
    assert_eq!(pin_source_block_reason(false, false), Some(BUILT_IN));
    assert_eq!(
        pin_source_block_reason(false, true),
        Some(BUILT_IN),
        "a built-in source never becomes pinnable, so waiting is not the advice"
    );
}

/// Ordering the catalog reorders the rows the page already derived, and never
/// leaves two of them in an order that depends on the frame.
///
/// The model identity is the final tie-break under every key, which is what
/// makes the order total: a column full of equal cells still lands the same way
/// twice, and reversing the direction reverses the whole order rather than
/// shuffling within its ties.
#[test]
fn ordering_the_catalog_is_total_under_every_column_and_reverses_whole() {
    fn row(model: &str, library: &str, family: &'static str, vectors: usize) -> ProjectModelRow {
        ProjectModelRow {
            library: library.to_owned(),
            model: model.to_owned(),
            family,
            source: format!("{library}.lib"),
            pinned: false,
            review: false,
            drifted: false,
            usage: None,
            usage_count: 0,
            vectors,
        }
    }

    let original = || {
        vec![
            row("nch", "beta", "NMOS", 4),
            row("pch", "alpha", "PMOS", 1),
            row("Nch", "alpha", "NMOS", 9),
        ]
    };
    let names = |rows: &[ProjectModelRow]| {
        rows.iter()
            .map(|row| format!("{}/{}", row.library, row.model))
            .collect::<Vec<_>>()
    };

    // The default is the order the table has always opened in: model name,
    // case-folded, library breaking ties.
    let mut rows = original();
    sort_catalog_rows(&mut rows, ModelsTableSort::default());
    assert_eq!(names(&rows), ["alpha/Nch", "beta/nch", "alpha/pch"]);

    let mut reversed = original();
    sort_catalog_rows(
        &mut reversed,
        ModelsTableSort {
            key: ModelsCatalogSortKey::Model,
            descending: true,
        },
    );
    let mut expected = names(&rows);
    expected.reverse();
    assert_eq!(names(&reversed), expected);

    // A numeric column orders by the number, not by its printed form.
    let mut by_vectors = original();
    sort_catalog_rows(
        &mut by_vectors,
        ModelsTableSort {
            key: ModelsCatalogSortKey::Vectors,
            descending: false,
        },
    );
    assert_eq!(
        by_vectors.iter().map(|row| row.vectors).collect::<Vec<_>>(),
        [1, 4, 9]
    );

    // A column whose cells are all equal still produces one order, and it is
    // the identity order.
    let mut tied = original();
    sort_catalog_rows(
        &mut tied,
        ModelsTableSort {
            key: ModelsCatalogSortKey::Status,
            descending: false,
        },
    );
    assert_eq!(names(&tied), names(&rows));
}

/// The STATUS column and the STATUS cell are one fact.
#[test]
fn the_status_column_orders_by_exactly_what_the_cell_says() {
    let mut row = ProjectModelRow {
        library: "alpha".to_owned(),
        model: "nch".to_owned(),
        family: "NMOS",
        source: "alpha.lib".to_owned(),
        pinned: false,
        review: false,
        drifted: false,
        usage: None,
        usage_count: 0,
        vectors: 0,
    };
    assert_eq!(catalog_status(&row).label(), "");
    row.pinned = true;
    assert_eq!(catalog_status(&row).label(), "pinned");
    row.review = true;
    assert_eq!(
        catalog_status(&row).label(),
        "review",
        "a finding outranks a pin: it is the thing an engineer has to act on"
    );
}

/// The status word and the tone it is painted in are one value.
///
/// A cell that says "review" in the tone of a settled state is worse than one
/// that says nothing, and the two used to be chosen in different places.
#[test]
fn every_status_the_column_states_carries_its_own_tone() {
    let t = Tokens::default();
    assert_eq!(CatalogStatus::Review.tone(&t), t.color.warn);
    assert_ne!(
        CatalogStatus::Pinned.tone(&t),
        t.color.warn,
        "a retained pin is a settled state; toning it like a finding is how a \
         warning colour comes to mean nothing"
    );
}

/// The USED BY cell names one instance and counts the rest.
///
/// A model bound eleven times and a model bound once showed the same single
/// designator, so the column could not be read for what it is for.
#[test]
fn the_used_by_cell_names_one_consumer_and_counts_the_others() {
    let mut row = ProjectModelRow {
        library: "alpha".to_owned(),
        model: "nch".to_owned(),
        family: "NMOS",
        source: "alpha.lib".to_owned(),
        pinned: false,
        review: false,
        drifted: false,
        usage: None,
        usage_count: 0,
        vectors: 0,
    };
    assert_eq!(used_by_cell(&row), "");
    row.usage = Some("M1".to_owned());
    row.usage_count = 1;
    assert_eq!(used_by_cell(&row), "M1");
    row.usage_count = 4;
    assert_eq!(used_by_cell(&row), "M1 · +3");
}

/// The catalog's USED BY cell reads the designator out of a consumer label,
/// and the where-used list reads the rest of it.
///
/// Both halves are spelled beside the code that writes the label, so a change
/// to its shape cannot leave one of them reading the wrong field.
#[test]
fn a_consumer_label_splits_into_a_designator_and_a_location() {
    let label = "M1 · NMOS · (120, 40)";
    assert_eq!(bindings::consumer_designator(label), "M1");
    assert_eq!(bindings::consumer_location(label), "NMOS · (120, 40)");

    // A label with no separator is all designator: the column still names
    // something rather than emptying itself.
    assert_eq!(bindings::consumer_designator("U1"), "U1");
    assert_eq!(bindings::consumer_location("U1"), "");
}

/// Every string one frame painted, with the colour it was painted in.
///
/// The catalog's cells are painter text, so reading the shapes back is the
/// only way to judge what a row says — and, for the status column, what it
/// means by the tone it says it in.
fn painted_text(output: &egui::FullOutput) -> Vec<(String, Color32)> {
    fn walk(shape: &egui::epaint::Shape, out: &mut Vec<(String, Color32)>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push((
                text.galley.text().to_owned(),
                text.galley
                    .job
                    .sections
                    .first()
                    .map_or(Color32::TRANSPARENT, |section| section.format.color),
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, out);
                }
            }
            _ => {}
        }
    }
    let mut found = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut found);
    }
    found
}

/// Paint one catalog row and read back what it drew.
fn catalog_row_paint(row: &ProjectModelRow) -> Vec<(String, Color32)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = AppState::default();
    let mut pending_actions = Vec::new();
    // Twice: the first pass builds the font set and the second lays out
    // against it, so a cell measured before the fonts exist is not the cell
    // the row ends up painting.
    let mut output = None;
    for _ in 0..2 {
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 120.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                let mut render = ManagerRenderContext {
                    state: &mut state,
                    pending_actions: &mut pending_actions,
                };
                egui::CentralPanel::default().show(ctx, |ui| {
                    project_model_row(ui, &mut render, row);
                });
            },
        ));
    }
    painted_text(&output.expect("two passes"))
}

fn catalog_row(review: bool, pinned: bool) -> ProjectModelRow {
    ProjectModelRow {
        library: "alpha".to_owned(),
        model: "nch".to_owned(),
        family: "NMOS",
        source: "alpha.lib".to_owned(),
        pinned,
        review,
        drifted: false,
        usage: Some("M1".to_owned()),
        usage_count: 4,
        vectors: 0,
    }
}

/// The status column paints its word in the tone of the state, and the USED BY
/// column names one consumer and counts the others.
///
/// Both cells are painted, so nothing that reads state covers either: the
/// column showed the same lone designator for a model bound once and a model
/// bound eleven times, and a row carrying a finding was the same grey as one
/// carrying none.
#[test]
fn the_catalog_row_paints_its_status_in_tone_and_counts_its_consumers() {
    let warn = Tokens::default().color.warn;

    let review = catalog_row_paint(&catalog_row(true, false));
    assert!(
        review.contains(&("review".to_owned(), warn)),
        "a row with a finding paints its status in the warning tone; it painted {review:?}"
    );
    assert!(
        review.iter().any(|(text, _)| text == "M1 · +3"),
        "the consumer cell names one instance and counts the rest; it painted {review:?}"
    );

    // A pin is a settled state, and reads as one.
    let pinned = catalog_row_paint(&catalog_row(false, true));
    let cell = pinned
        .iter()
        .find(|(text, _)| text == "pinned")
        .expect("a pinned row states that its bytes are retained");
    assert_ne!(cell.1, warn);
}

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
                    .show(ctx, |ui| show(ui, app));
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
    app.state.model_library_manager.clear();
    let mut library = ModelLibrary::new("alpha");
    let mut model = DeviceModel::new("nch", ModelType::Nmos);
    model.file_path = Some(PathBuf::from("alpha.lib"));
    model.source_line = Some(41);
    model.parameters.insert("vto".to_owned(), 0.7);
    library.add_model(model);
    library
        .model_qualification
        .insert("nch".to_owned(), ModelQualificationState::default());
    app.state.model_library_manager.add_library(library);
    app.state.select_model_library("alpha");
    app.state.workbench.selected_model = Some("nch".to_owned());
    app.state.workbench.models_page = ModelsPage::Models;

    let mut component = Component::new(1, ComponentType::Nmos, Point::origin());
    component.name = "M1".to_owned();
    component.params = "model=nch".to_owned();
    app.state.schematic.components.push(component);
    app.state.schematic.selection.select_only_component(1);
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
    app.state.schematic.selection.clear();
    let unselected = ModelsStudio::open(app);
    assert!(
        unselected.announces(|label| label == "Bind to selection…"),
        "the page announces {:#?}",
        unselected.announcements()
    );
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
        studio.app.state.workbench.models_page,
        ModelsPage::Models,
        "the studio opens on the catalog"
    );

    studio.click(|label| label.starts_with("Suites:"));

    assert_eq!(
        studio.app.state.workbench.models_page,
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
