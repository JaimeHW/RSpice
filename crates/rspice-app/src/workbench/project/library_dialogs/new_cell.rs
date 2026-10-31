//! New Cell — small modal on the dialog primitive: target library, name,
//! description, and the views to seed the cell with.

use egui::Context;

use crate::state::workspace::VERILOGA_LIBRARY_NAME;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, input_row, switch_row};
use crate::workbench::app::dialogs::library::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::{ProjectRequest, ReservedLibraryRevision};

use super::new_view::seeded_symbol_view;

impl ProjectFeature {
    /// Draw New Cell. What a created cell leaves outside its catalog change
    /// goes to the shell, which carries it out as the dialog returns.
    pub(in crate::workbench) fn process_new_cell_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        if !cx.state.new_cell_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_create = false;

        let lib_names: Vec<String> = cx
            .model
            .library_manager
            .libraries_sorted()
            .iter()
            .filter(|lib| !lib.read_only)
            .map(|lib| lib.name.clone())
            .collect();
        if cx.state.new_cell_library.is_empty() && !lib_names.is_empty() {
            cx.state.new_cell_library = lib_names[0].clone();
        }
        let catalog_current =
            cx.model.library_manager.revision() == cx.state.new_cell_library_revision;
        if !catalog_current {
            cx.state.new_cell_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }

        let dialogs = &mut *cx.state;
        let choice = Dialog::new("New cell", "Create")
            .description(
                "Create a named cell in a writable library with the selected initial views.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .primary_enabled(catalog_current && !dialogs.new_cell_name.trim().is_empty())
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                let t = Tokens::get(ui.ctx());
                let c = t.color;

                // Target library.
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), t.metrics.row_h),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        let (label_rect, _) = ui.allocate_exact_size(
                            egui::vec2(92.0, t.metrics.row_h),
                            egui::Sense::hover(),
                        );
                        ui.painter().text(
                            egui::pos2(label_rect.left(), label_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            "Library",
                            theme::sans(tokens::FS_1, FontWeight::Regular),
                            c.text_dim,
                        );
                        egui::ComboBox::from_id_salt("cell_library_combo")
                            .selected_text(&dialogs.new_cell_library)
                            .width(ui.available_width())
                            .show_ui(ui, |ui| {
                                for name in &lib_names {
                                    ui.selectable_value(
                                        &mut dialogs.new_cell_library,
                                        name.clone(),
                                        name,
                                    );
                                }
                            });
                    },
                );

                input_row(ui, "Cell name", &mut dialogs.new_cell_name);
                input_row(ui, "Description", &mut dialogs.new_cell_description);

                ui.add_space(8.0);
                let mut kicker = egui::text::LayoutJob::default();
                kicker.append(
                    "VIEWS TO CREATE",
                    0.0,
                    egui::TextFormat {
                        font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                        color: c.text_faint,
                        extra_letter_spacing: 0.08 * tokens::FS_0,
                        ..Default::default()
                    },
                );
                ui.label(kicker);
                switch_row(ui, "Schematic", &mut dialogs.new_cell_create_schematic);
                switch_row(ui, "Symbol", &mut dialogs.new_cell_create_symbol);
                switch_row(ui, "Testbench", &mut dialogs.new_cell_create_testbench);
                switch_row(ui, "Verilog-A", &mut dialogs.new_cell_create_veriloga);

                if let Some(error) = dialogs.new_cell_error.clone() {
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(error)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.err),
                    );
                }
            });

        match choice {
            DialogChoice::Primary => should_create = true,
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_create {
            should_close |= create_new_cell(cx);
        }

        if should_close {
            cx.state.new_cell_dialog = false;
            cx.state.new_cell_name.clear();
            cx.state.new_cell_description.clear();
            cx.state.new_cell_error = None;
            cx.state.new_cell_create_schematic = true;
            cx.state.new_cell_create_symbol = false;
            cx.state.new_cell_create_testbench = false;
            cx.state.new_cell_create_veriloga = false;
        }
    }
}

/// Create the cell the dialog names, and whether it was created.
///
/// The cell and every view it starts with are made on copies of the
/// catalog, so a refusal anywhere creates nothing; the catalog then takes
/// the copies, and what the creation leaves outside it goes to the shell
/// in today's order. A refusal is stated in the dialog.
fn create_new_cell(cx: &mut Cx<'_, ProjectFeature>) -> bool {
    let name = cx.state.new_cell_name.trim().to_string();
    let library = cx.state.new_cell_library.clone();

    if cx.model.library_manager.revision() != cx.state.new_cell_library_revision {
        cx.state.new_cell_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        return false;
    }
    if let Some(error) = validate_lcv_name(&name, "Cell name") {
        cx.state.new_cell_error = Some(error);
        return false;
    }
    if library.is_empty() {
        cx.state.new_cell_error = Some("Please select a library".to_string());
        return false;
    }

    let Some(lib_ro) = cx.model.library_manager.get_library(&library) else {
        cx.state.new_cell_error = Some(format!("Library '{}' not found", library));
        return false;
    };
    if lib_ro.read_only {
        cx.state.new_cell_error = Some(format!(
            "Library '{library}' is read only; create the cell in an editable library"
        ));
        return false;
    }
    let requested_identity = crate::state::canonical_cell_view_owner_key(&library, &name, "");
    if lib_ro.cells.values().any(|cell| {
        crate::state::canonical_cell_view_owner_key(&library, &cell.name, "") == requested_identity
    }) {
        cx.state.new_cell_error = Some(format!(
            "Cell '{}' clashes with an existing cell in library '{}' (cell names ignore letter case)",
            name, library
        ));
        return false;
    }

    use crate::state::{Cell, View, ViewType};

    let mut cell = Cell::new(&name);
    cell.description = cx.state.new_cell_description.clone();

    if cx.state.new_cell_create_schematic {
        cell.add_view(View::new("schematic", ViewType::Schematic));
    }
    if cx.state.new_cell_create_symbol {
        // A cell created here starts on an empty schematic, so its symbol
        // starts empty too and grows with the interface the user draws.
        match seeded_symbol_view("symbol", &[]) {
            Ok(view) => cell.add_view(view),
            Err(error) => {
                cx.state.new_cell_error = Some(error);
                return false;
            }
        }
    }
    if cx.state.new_cell_create_testbench {
        cell.add_view(View::new("testbench", ViewType::Testbench));
    }

    // Every view is made on copies, the Verilog-A view's source and
    // compiled interface included, so a refusal anywhere creates nothing.
    let mut candidate_libraries = cx.model.library_manager.clone();
    let Some(mut candidate_library) = candidate_libraries.edit_library(&library) else {
        cx.state.new_cell_error = Some(format!("Library '{}' not found", library));
        return false;
    };
    candidate_library.add_cell(cell);
    let veriloga =
        crate::state::CellViewRef::new(&library, &name, crate::state::workspace::VERILOGA_VIEW);
    let seeded_veriloga = if cx.state.new_cell_create_veriloga {
        let mut candidate_sources = cx.model.workspace.content.project_sources.clone();
        match crate::workbench::app::dialogs::library::new_veriloga_device::seed_veriloga_view(
            cx.model,
            &mut candidate_libraries,
            &mut candidate_sources,
            &veriloga,
            &[],
        ) {
            Ok(receipt) => Some((candidate_sources, receipt)),
            Err(error) => {
                cx.state.new_cell_error = Some(error);
                return false;
            }
        }
    } else {
        None
    };

    let project_mutation = match cx.model.preflight_project_library_mutation(
        crate::state::ProjectLibraryMutation::CreateCell {
            library: library.clone(),
            cell: name.clone(),
        },
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            cx.state.new_cell_error = Some(error);
            return false;
        }
    };
    cx.model.library_manager = candidate_libraries;
    let receipt = seeded_veriloga.map(|(sources, receipt)| {
        cx.model.workspace.content.project_sources = sources;
        cx.model.workspace.content.project_sources_dirty = true;
        receipt
    });
    let schematics = [
        cx.state.new_cell_create_schematic.then_some("schematic"),
        cx.state.new_cell_create_testbench.then_some("testbench"),
    ]
    .into_iter()
    .flatten()
    .map(|view| crate::state::CellViewRef::new(&library, &name, view))
    .collect();
    // A schematic defines the cell's ports, so it opens first; a Verilog-A
    // view without one is the module to write next.
    let view_to_open = if cx.state.new_cell_create_schematic {
        Some("schematic")
    } else if receipt.is_some() {
        Some(veriloga.view.as_str())
    } else if cx.state.new_cell_create_symbol {
        Some("symbol")
    } else if cx.state.new_cell_create_testbench {
        Some("testbench")
    } else {
        None
    };
    let open = view_to_open.map(|view| crate::state::CellViewRef::new(&library, &name, view));
    cx.wb.outbox.push(ProjectRequest::CellViewsCreated {
        schematics,
        mutation: ReservedLibraryRevision::new(project_mutation),
        notice: format!("Created cell '{name}' in library '{library}'."),
        open,
        veriloga: receipt.map(|receipt| Box::new((veriloga, receipt))),
        persist_global_veriloga: library == VERILOGA_LIBRARY_NAME,
    });
    cx.state.new_cell_error = None;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        CellViewRef, ComponentType, Library, NetNamingPolicy, Point, SchematicGridPitch,
        SymbolDocument,
    };
    use crate::workbench::ChoicePreference;
    use crate::workbench::RSpiceApp;
    use crate::workbench::project::library_dialogs::{in_front, published, shown};
    use crate::workbench::project::tests::with_cx;

    /// Create the cell the dialog names in the feature's context, with
    /// what it raised answered as the dialog step answers it, and whether
    /// it was created.
    fn create(app: &mut RSpiceApp) -> bool {
        let created = with_cx(app, create_new_cell);
        app.after_contribution();
        created
    }

    fn cell_symbol_view(app: &RSpiceApp, library: &str, cell: &str) -> crate::state::View {
        app.state
            .model
            .library_manager
            .get_library(library)
            .and_then(|library| library.get_cell(cell))
            .and_then(|cell| cell.get_view("symbol"))
            .cloned()
            .expect("the created cell owns a symbol view")
    }

    #[test]
    fn creating_cell_seeds_every_schematic_like_view_at_creation_time() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .model
            .library_manager
            .add_library(Library::new("policy_test"));
        app.state
            .model
            .preferences
            .set_choice(ChoicePreference::SchematicGrid, 2)
            .unwrap();
        app.state
            .model
            .preferences
            .set_choice(ChoicePreference::NetNamingPolicy, 1)
            .unwrap();
        app.state.project.new_cell_library = "policy_test".to_owned();
        app.state.project.new_cell_name = "amp".to_owned();
        app.state.project.new_cell_create_schematic = true;
        app.state.project.new_cell_create_testbench = true;
        app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
        let project_revision_before = app.state.model.workspace.content.project.revision().get();

        let created = create(&mut app);

        assert!(created);
        assert_eq!(
            app.state.model.workspace.content.project.revision().get(),
            project_revision_before + 1
        );
        assert!(matches!(
            app.state
                .model.workspace
                .content.project
                .library_mutation_audit()
                .last()
                .map(|receipt| receipt.mutation()),
            Some(crate::state::ProjectLibraryMutation::CreateCell { library, cell })
                if library == "policy_test" && cell == "amp"
        ));
        assert!(app.state.model.workspace.content.project_metadata_dirty);
        for view in ["schematic", "testbench"] {
            let key = crate::state::CellViewRef::new("policy_test", "amp", view).key();
            let document = app
                .state
                .model
                .workspace
                .content
                .schematic_buffers
                .get(&key)
                .expect("document seeded when view is created");
            assert_eq!(
                document.document().document_policy.grid_pitch,
                SchematicGridPitch::Metric
            );
            assert_eq!(
                document.document().grid_size,
                SchematicGridPitch::Metric.canvas_grid_size()
            );
            assert_eq!(
                document.document().document_policy.net_naming,
                NetNamingPolicy::SpiceCompatibleRelaxed
            );
        }
    }

    #[test]
    fn new_cell_symbol_view_follows_the_schematic_interface_until_first_edited() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .model
            .library_manager
            .add_library(Library::new("tracking_test"));
        app.state.project.new_cell_library = "tracking_test".to_owned();
        app.state.project.new_cell_name = "amp".to_owned();
        app.state.project.new_cell_create_schematic = true;
        app.state.project.new_cell_create_symbol = true;
        app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();

        let created = create(&mut app);

        assert!(created);
        assert_eq!(
            app.state.model.workspace.content.active_view,
            CellViewRef::new("tracking_test", "amp", "schematic")
        );
        let created = cell_symbol_view(&app, "tracking_test", "amp");
        assert_eq!(
            created.metadata.get("generated").map(String::as_str),
            Some("ports"),
            "a symbol created with the cell is the schematic's until hand edited"
        );
        assert!(
            SymbolDocument::load_from_view(&created)
                .expect("the seeded symbol document parses")
                .pins
                .is_empty()
        );

        let port_id = app
            .state
            .model
            .schematic
            .add_component(ComponentType::Port, Point::origin());
        app.state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == port_id)
            .expect("the placed interface port")
            .value = "IN".to_owned();
        app.state.sync_active_schematic_to_workspace();

        let refreshed = cell_symbol_view(&app, "tracking_test", "amp");
        assert_eq!(
            refreshed.metadata.get("ports").map(String::as_str),
            Some("IN:inout")
        );
        assert_eq!(
            SymbolDocument::load_from_view(&refreshed)
                .expect("the refreshed symbol document parses")
                .pins
                .iter()
                .map(|pin| pin.name.as_str())
                .collect::<Vec<_>>(),
            ["IN"]
        );
    }

    /// The Verilog-A switch makes the view New Verilog-A device… makes, by
    /// the same path: a starter module, its source, and the symbol its
    /// compile generates, in the one cell transaction.
    #[test]
    fn new_cell_verilog_a_switch_creates_the_module_view_and_its_generated_symbol() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .model
            .library_manager
            .add_library(Library::new("behavioral"));
        app.state.project.new_cell_library = "behavioral".to_owned();
        app.state.project.new_cell_name = "comparator".to_owned();
        app.state.project.new_cell_create_schematic = false;
        app.state.project.new_cell_create_veriloga = true;
        app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
        let project_revision_before = app.state.model.workspace.content.project.revision().get();

        let created = create(&mut app);

        assert!(created, "{:?}", app.state.project.new_cell_error);
        assert_eq!(
            app.state.model.workspace.content.project.revision().get(),
            project_revision_before + 1
        );
        let reference = CellViewRef::new("behavioral", "comparator", "veriloga");
        let cell = app
            .state
            .model
            .library_manager
            .get_library("behavioral")
            .and_then(|library| library.get_cell("comparator"))
            .expect("the created cell");
        let veriloga = cell.get_view("veriloga").expect("the Verilog-A view");
        assert_eq!(
            veriloga.metadata.get("veriloga.module").map(String::as_str),
            Some("comparator")
        );
        assert_eq!(
            veriloga.metadata.get("veriloga.ports").map(String::as_str),
            Some("[]")
        );
        let symbol = cell.get_view("symbol").expect("the generated symbol");
        assert!(rspice_design::symbol::generated::is_generated_symbol(
            symbol
        ));
        assert!(
            app.state
                .model
                .workspace
                .content
                .project_sources
                .bundle_for_owner(&crate::state::ProjectSourceOwner::cell_view(
                    reference.clone()
                ))
                .is_some_and(|bundle| bundle.root().logical_path() == "comparator.va"
                    && bundle.root().content().contains("module comparator;"))
        );
        assert_eq!(app.state.model.workspace.content.active_view, reference);
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Design
        );
        assert!(app.state.code.code_workspace.veriloga.receipt.is_some());
    }

    #[test]
    fn new_cell_rejects_accented_canonical_collision_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("identity_test");
        library.add_cell(crate::state::Cell::new("\u{c9}tage"));
        app.state.model.library_manager.add_library(library);
        app.state.project.new_cell_library = "identity_test".to_owned();
        app.state.project.new_cell_name = "\u{e9}TAGE".to_owned();
        app.state.project.new_cell_create_schematic = true;
        app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
        let buffers_before = app.state.model.workspace.content.schematic_buffers.len();

        let created = create(&mut app);

        assert!(!created);
        let library = app
            .state
            .model
            .library_manager
            .get_library("identity_test")
            .expect("identity library remains");
        assert_eq!(library.cell_count(), 1);
        assert!(library.get_cell("\u{c9}tage").is_some());
        assert_eq!(
            app.state.model.workspace.content.schematic_buffers.len(),
            buffers_before
        );
        assert!(
            app.state
                .project
                .new_cell_error
                .as_deref()
                .is_some_and(|error| error.contains("clashes with an existing cell"))
        );
    }

    #[test]
    fn new_cell_rejects_read_only_library_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("read_only_test");
        library.read_only = true;
        app.state.model.library_manager.add_library(library);
        app.state.project.new_cell_library = "read_only_test".to_owned();
        app.state.project.new_cell_name = "amp".to_owned();
        app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
        let revision_before = app.state.model.workspace.content.project.revision();

        let created = create(&mut app);

        assert!(!created);
        assert!(
            app.state
                .model
                .library_manager
                .get_library("read_only_test")
                .is_some_and(|library| library.get_cell("amp").is_none())
        );
        assert_eq!(
            app.state.model.workspace.content.project.revision(),
            revision_before
        );
        assert!(
            app.state
                .project
                .new_cell_error
                .as_deref()
                .is_some_and(|error| error.contains("read only"))
        );
    }

    #[test]
    fn new_cell_rejects_catalog_change_after_dialog_open_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .model
            .library_manager
            .add_library(Library::new("stale_test"));
        app.state.project.new_cell_library = "stale_test".to_owned();
        app.state.project.new_cell_name = "amp".to_owned();
        app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
        app.state
            .model
            .library_manager
            .add_library(Library::new("intervening_change"));
        let project_revision_before = app.state.model.workspace.content.project.revision();

        let created = create(&mut app);

        assert!(!created);
        assert!(
            app.state
                .model
                .library_manager
                .get_library("stale_test")
                .is_some_and(|library| library.get_cell("amp").is_none())
        );
        assert_eq!(
            app.state.model.workspace.content.project.revision(),
            project_revision_before
        );
        assert_eq!(
            app.state.project.new_cell_error.as_deref(),
            Some(LIBRARY_CATALOG_STALE_MESSAGE)
        );
    }

    /// A cell in a library of its own typed into New Cell, with a Verilog-A
    /// view: with a schematic and a testbench, which open first, or alone,
    /// when the module is what opens.
    fn creating(veriloga_only: bool) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.state
            .model
            .library_manager
            .add_library(Library::new("behavioral"));
        let project = &mut app.state.project;
        project.new_cell_library = "behavioral".to_owned();
        project.new_cell_name = "comparator".to_owned();
        project.new_cell_create_schematic = !veriloga_only;
        project.new_cell_create_testbench = !veriloga_only;
        project.new_cell_create_veriloga = true;
        app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
        app
    }

    /// Carry out what the creation handed over in the order the creation
    /// once did it in place, with the new document it read before it
    /// changed the catalog.
    fn carry_out_in_place(app: &mut RSpiceApp, seeded: crate::state::SchematicState) {
        let Some(ProjectRequest::CellViewsCreated {
            schematics,
            mutation,
            notice,
            open,
            veriloga,
            persist_global_veriloga,
        }) = app.state.workbench.outbox.take::<ProjectRequest>()
        else {
            panic!("the creation hands the shell what it left");
        };
        assert!(
            !persist_global_veriloga,
            "the cell is not the Verilog-A library's"
        );
        let state = &mut app.state;
        for reference in schematics {
            state
                .model
                .workspace
                .insert_schematic_editor(reference.key(), seeded.clone());
        }
        state.model.publish_project_library_mutation(
            mutation
                .take()
                .expect("the reserved revision is published once"),
        );
        state.record(
            crate::diagnostics::LogSeverity::Info,
            crate::diagnostics::LogSource::Studio,
            notice,
        );
        if let Some(view) = open {
            state.open_workspace_view(view);
        }
        if let Some((view, receipt)) = veriloga.map(|seeded| *seeded)
            && state.model.workspace.content.active_view == view
        {
            state
                .workbench
                .activate(crate::workbench::state::Workspace::Design);
            state.code.code_workspace.veriloga.receipt = Some(receipt);
        }
    }

    /// A cell created in the feature's context, with what it left handed to
    /// the shell, ends where the creation carried out in place did: the new
    /// documents, then the revision, then the creation's line in the log,
    /// then the view it opens, and the Verilog-A receipt kept only when the
    /// module is the view open.
    #[test]
    fn a_created_cell_ends_where_the_creation_carried_out_in_place_did() {
        for veriloga_only in [false, true] {
            let mut in_place = creating(veriloga_only);
            let seeded = in_place.state.new_schematic_document();
            assert!(with_cx(&mut in_place, create_new_cell));
            carry_out_in_place(&mut in_place, seeded);

            let mut handed = creating(veriloga_only);
            assert!(create(&mut handed));

            assert_eq!(
                shown(&handed),
                shown(&in_place),
                "Verilog-A only: {veriloga_only}"
            );
            assert_eq!(published(&handed), published(&in_place));
            assert_eq!(in_front(&handed), in_front(&in_place));
            assert_eq!(in_front(&handed).1, veriloga_only);
        }
    }
}
