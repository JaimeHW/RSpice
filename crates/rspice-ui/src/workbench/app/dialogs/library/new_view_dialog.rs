//! New View — small modal on the dialog primitive: the target
//! library/cell context, a view name, and the view type as chips.

use super::shared::{DialogActionOutcome, validate_lcv_name};
use super::{Context, RSpiceApp, VERILOGA_LIBRARY_NAME};
use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, chip, input_row, kv_row};

/// Cell-view kinds backed by a complete, editable production surface.
///
/// Other persisted `ViewType` variants remain valid for project interchange,
/// but must not be offered by the creation workflow until their editor and
/// storage contract are implemented end to end.
const CREATABLE_VIEW_TYPES: [crate::state::ViewType; 4] = [
    crate::state::ViewType::Schematic,
    crate::state::ViewType::Testbench,
    crate::state::ViewType::Symbol,
    crate::state::ViewType::VerilogA,
];

const fn is_creatable_view_type(view_type: crate::state::ViewType) -> bool {
    matches!(
        view_type,
        crate::state::ViewType::Schematic
            | crate::state::ViewType::Testbench
            | crate::state::ViewType::Symbol
            | crate::state::ViewType::VerilogA
    )
}

impl RSpiceApp {
    pub(in crate::workbench::app) fn process_new_view_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.new_view_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_create = false;
        let mut persist_global_veriloga = false;

        let dialogs = &mut self.state.dialogs;
        let can_create = !dialogs.new_view_name.trim().is_empty()
            && !dialogs.new_view_library.is_empty()
            && !dialogs.new_view_cell.is_empty()
            && is_creatable_view_type(dialogs.new_view_type);

        let choice = Dialog::new("Library", "New view", "Create")
            .description("Create a named view of the selected type in this library cell.")
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .primary_enabled(can_create)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                let t = Tokens::get(ui.ctx());
                let c = t.color;

                kv_row(ui, "Library", &dialogs.new_view_library);
                kv_row(ui, "Cell", &dialogs.new_view_cell);
                ui.add_space(4.0);

                input_row(ui, "View name", &mut dialogs.new_view_name);

                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), t.metrics.row_h),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        let (label_rect, _) = ui.allocate_exact_size(
                            egui::vec2(96.0, t.metrics.row_h),
                            egui::Sense::hover(),
                        );
                        ui.painter().text(
                            egui::pos2(label_rect.left(), label_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            "Type",
                            theme::sans(tokens::FS_1, FontWeight::Regular),
                            c.text_dim,
                        );
                        for view_type in CREATABLE_VIEW_TYPES {
                            if chip(
                                ui,
                                view_type.display_name(),
                                dialogs.new_view_type == view_type,
                            )
                            .clicked()
                            {
                                dialogs.new_view_type = view_type;
                            }
                        }
                    },
                );

                if let Some(error) = dialogs.new_view_error.clone() {
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
            let outcome = self.handle_new_view_create_action();
            should_close |= outcome.close;
            persist_global_veriloga |= outcome.persist_global_veriloga;
        }

        if persist_global_veriloga {
            self.persist_global_veriloga_library_with_feedback();
        }

        if should_close {
            self.state.dialogs.new_view_dialog = false;
            self.state.dialogs.new_view_name.clear();
            self.state.dialogs.new_view_error = None;
        }
    }

    pub(super) fn handle_new_view_create_action(&mut self) -> DialogActionOutcome {
        let mut outcome = DialogActionOutcome::default();
        let view_name = self.state.dialogs.new_view_name.trim().to_string();
        let library = self.state.dialogs.new_view_library.clone();
        let cell = self.state.dialogs.new_view_cell.clone();

        if let Some(error) = validate_lcv_name(&view_name, "View name") {
            self.state.dialogs.new_view_error = Some(error);
            return outcome;
        }
        if library.is_empty() {
            self.state.dialogs.new_view_error = Some("Library cannot be empty".to_string());
            return outcome;
        }
        if cell.is_empty() {
            self.state.dialogs.new_view_error = Some("Cell cannot be empty".to_string());
            return outcome;
        }
        let view_type = self.state.dialogs.new_view_type;
        if !is_creatable_view_type(view_type) {
            self.state.dialogs.new_view_error = Some(format!(
                "{} views cannot be created until their production editor and storage workflow are available",
                view_type.display_name()
            ));
            return outcome;
        }

        let Some(lib_ro) = self.state.library_manager.get_library(&library) else {
            self.state.dialogs.new_view_error = Some(format!("Library '{}' not found", library));
            return outcome;
        };
        if lib_ro.read_only {
            self.state.dialogs.new_view_error = Some(format!(
                "Library '{library}' is read only; create the view in an editable library"
            ));
            return outcome;
        }
        let Some(cell_ro) = lib_ro.get_cell(&cell) else {
            self.state.dialogs.new_view_error = Some(format!(
                "Cell '{}' not found in library '{}'",
                cell, library
            ));
            return outcome;
        };
        let requested_identity =
            crate::state::canonical_cell_view_owner_key(&library, &cell, &view_name);
        if cell_ro.views.values().any(|view| {
            crate::state::canonical_cell_view_owner_key(&library, &cell, &view.name)
                == requested_identity
        }) {
            self.state.dialogs.new_view_error = Some(format!(
                "View '{}' conflicts with an existing canonical view identity in cell '{}'",
                view_name, cell
            ));
            return outcome;
        }

        if view_type == crate::state::ViewType::VerilogA {
            return self.create_veriloga_cell_view(&library, &cell, &view_name);
        }

        use crate::state::View;

        let seeded_schematic = self.state.new_schematic_document();
        if let Some(lib) = self.state.library_manager.get_library_mut(&library) {
            if let Some(cell_ref) = lib.get_cell_mut(&cell) {
                cell_ref.add_view(View::new(&view_name, view_type));
                if matches!(
                    view_type,
                    crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
                ) {
                    let reference = crate::state::CellViewRef::new(&library, &cell, &view_name);
                    self.state
                        .workspace
                        .schematic_buffers
                        .insert(reference.key(), seeded_schematic);
                }
                self.state.push_user_message(ConsoleMessage::info(format!(
                    "Created view '{}' in cell '{}'",
                    view_name, cell
                )));
                if matches!(
                    view_type,
                    crate::state::ViewType::Schematic
                        | crate::state::ViewType::Testbench
                        | crate::state::ViewType::Symbol
                ) {
                    self.state
                        .open_workspace_view(crate::state::CellViewRef::new(
                            library.clone(),
                            cell.clone(),
                            view_name.clone(),
                        ));
                }
                self.state.dialogs.new_view_error = None;
                outcome.close = true;
                outcome.persist_global_veriloga = library == VERILOGA_LIBRARY_NAME;
            } else {
                self.state.dialogs.new_view_error = Some(format!(
                    "Cell '{}' not found in library '{}'",
                    cell, library
                ));
            }
        } else {
            self.state.dialogs.new_view_error = Some(format!("Library '{}' not found", library));
        }

        outcome
    }

    fn create_veriloga_cell_view(
        &mut self,
        library: &str,
        cell: &str,
        view_name: &str,
    ) -> DialogActionOutcome {
        let mut outcome = DialogActionOutcome::default();
        let reference = crate::state::CellViewRef::new(library, cell, view_name);
        let ports = match derive_cell_interface(self, library, cell) {
            Ok(ports) => ports,
            Err(error) => {
                self.state.dialogs.new_view_error = Some(error);
                return outcome;
            }
        };
        let module_name = verilog_module_name(cell);
        if let Some(invalid) = ports.iter().find(|port| !is_verilog_identifier(&port.name)) {
            self.state.dialogs.new_view_error = Some(format!(
                "Interface port '{}' is not a valid Verilog-A identifier; rename the cell port before creating this view",
                invalid.name
            ));
            return outcome;
        }
        let source = starter_veriloga_source(&module_name, &ports);
        let root_path = format!("{module_name}.va");
        let mut bundle = match crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::cell_view(reference.clone()),
            crate::state::ProjectSourceLanguage::VerilogA,
            root_path,
            source,
            [],
            [],
        ) {
            Ok(bundle) => bundle,
            Err(error) => {
                self.state.dialogs.new_view_error = Some(format!(
                    "The Verilog-A source bundle could not be created: {error}"
                ));
                return outcome;
            }
        };
        let receipt = match crate::workbench::documents::code_workspace::compile_project_bundle_receipt(
            self.state.workspace.project.id(),
            &bundle,
            Some(&module_name),
        ) {
            Ok(receipt) => receipt,
            Err(diagnostics) => {
                let detail = diagnostics
                    .first()
                    .map(|diagnostic| format!("{}: {}", diagnostic.message, diagnostic.detail))
                    .unwrap_or_else(|| "the compiler returned no diagnostic".to_owned());
                self.state.dialogs.new_view_error = Some(format!(
                    "The generated Verilog-A starter did not pass semantic compilation: {detail}"
                ));
                return outcome;
            }
        };
        if let Err(error) = bundle.mark_validated() {
            self.state.dialogs.new_view_error = Some(format!(
                "The compiled Verilog-A source could not retain its validation identity: {error}"
            ));
            return outcome;
        }

        // Prepare both durable stores on clones. No live project mutation is
        // visible until the complete view/source transaction has succeeded.
        let mut candidate_sources = self.state.workspace.project_sources.clone();
        if let Err(error) = candidate_sources.insert_bundle(bundle) {
            self.state.dialogs.new_view_error = Some(format!(
                "The Verilog-A source bundle could not be retained: {error}"
            ));
            return outcome;
        }
        let port_names = ports
            .iter()
            .map(|port| port.name.as_str())
            .collect::<Vec<_>>();
        let encoded_ports = match serde_json::to_string(&port_names) {
            Ok(encoded) => encoded,
            Err(error) => {
                self.state.dialogs.new_view_error = Some(format!(
                    "The Verilog-A interface contract could not be retained: {error}"
                ));
                return outcome;
            }
        };
        let mut view = crate::state::View::new(view_name, crate::state::ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), module_name.clone());
        view.metadata
            .insert("veriloga.ports".to_owned(), encoded_ports);
        let mut candidate_libraries = self.state.library_manager.clone();
        let Some(candidate_library) = candidate_libraries.get_library_mut(library) else {
            self.state.dialogs.new_view_error = Some(format!("Library '{library}' not found"));
            return outcome;
        };
        if candidate_library.read_only {
            self.state.dialogs.new_view_error = Some(format!(
                "Library '{library}' became read only before the view could be created"
            ));
            return outcome;
        }
        let Some(candidate_cell) = candidate_library.get_cell_mut(cell) else {
            self.state.dialogs.new_view_error =
                Some(format!("Cell '{cell}' not found in library '{library}'"));
            return outcome;
        };
        let requested_identity =
            crate::state::canonical_cell_view_owner_key(library, cell, view_name);
        if candidate_cell.views.values().any(|view| {
            crate::state::canonical_cell_view_owner_key(library, cell, &view.name)
                == requested_identity
        }) {
            self.state.dialogs.new_view_error = Some(format!(
                "View '{view_name}' conflicts with an existing canonical view identity in cell '{cell}'"
            ));
            return outcome;
        }
        candidate_cell.add_view(view);

        self.state.workspace.project_sources = candidate_sources;
        self.state.workspace.project_sources_dirty = true;
        self.state.library_manager = candidate_libraries;
        self.state.dialogs.new_view_error = None;
        self.state.open_workspace_view(reference);
        self.state
            .workbench
            .activate(crate::workbench::state::Workspace::Netlist);
        self.state.ui.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA;
        self.state.ui.code_workspace.veriloga.receipt = Some(receipt);
        self.state.push_user_message(ConsoleMessage::info(format!(
            "Created Verilog-A view '{view_name}' for cell '{cell}' and opened its project-owned source."
        )));
        outcome.close = true;
        outcome.persist_global_veriloga = library == VERILOGA_LIBRARY_NAME;
        outcome
    }
}

fn derive_cell_interface(
    app: &RSpiceApp,
    library_name: &str,
    cell_name: &str,
) -> Result<Vec<crate::state::PortSpec>, String> {
    let cell = app
        .state
        .library_manager
        .get_library(library_name)
        .and_then(|library| library.get_cell(cell_name))
        .ok_or_else(|| format!("Cell '{cell_name}' not found in library '{library_name}'"))?;
    let mut views = cell.views.values().collect::<Vec<_>>();
    views.sort_by(|left, right| {
        interface_view_rank(left)
            .cmp(&interface_view_rank(right))
            .then_with(|| left.name.cmp(&right.name))
    });

    let mut contracts: Vec<(String, Vec<crate::state::PortSpec>)> = Vec::new();
    for view in &views {
        if !matches!(
            view.view_type,
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        ) {
            continue;
        }
        let reference = crate::state::CellViewRef::new(library_name, cell_name, &view.name);
        let schematic = if app.state.workspace.active_view == reference {
            Some(&app.state.schematic)
        } else {
            app.state.workspace.schematic_buffers.get(&reference.key())
        };
        if let Some(schematic) = schematic {
            let ports = schematic.interface_ports();
            if !ports.is_empty() {
                contracts.push((reference.display_path(), ports));
            }
        }
    }
    for view in views {
        if view.view_type != crate::state::ViewType::Symbol {
            continue;
        }
        let document = crate::state::SymbolDocument::load_from_view(view).map_err(|error| {
            format!(
                "The symbol interface in {library_name}/{cell_name}/{} is invalid: {error}",
                view.name
            )
        })?;
        if document.pins.is_empty() {
            continue;
        }
        let ports = document
            .pins
            .into_iter()
            .map(|pin| crate::state::PortSpec {
                name: pin.name,
                direction: pin.direction,
            })
            .collect::<Vec<_>>();
        contracts.push((format!("{library_name}/{cell_name}/{}", view.name), ports));
    }

    let Some((authority_name, authority)) = contracts.first() else {
        return Ok(vec![
            crate::state::PortSpec {
                name: "p".to_owned(),
                direction: crate::state::PortDirection::InOut,
            },
            crate::state::PortSpec {
                name: "n".to_owned(),
                direction: crate::state::PortDirection::InOut,
            },
        ]);
    };
    validate_unique_ports(authority_name, authority)?;
    for (source_name, candidate) in contracts.iter().skip(1) {
        validate_unique_ports(source_name, candidate)?;
        if candidate != authority {
            return Err(format!(
                "The existing cell interfaces disagree between '{authority_name}' and '{source_name}'. Reconcile their ordered port names and directions before creating a Verilog-A view."
            ));
        }
    }
    Ok(authority.clone())
}

fn interface_view_rank(view: &crate::state::View) -> (u8, u8) {
    let kind = match view.view_type {
        crate::state::ViewType::Schematic => 0,
        crate::state::ViewType::Testbench => 1,
        crate::state::ViewType::Symbol => 2,
        _ => 3,
    };
    let canonical_name = u8::from(!matches!(
        view.name.as_str(),
        "schematic" | "testbench" | "symbol"
    ));
    (kind, canonical_name)
}

fn validate_unique_ports(source: &str, ports: &[crate::state::PortSpec]) -> Result<(), String> {
    let mut names = std::collections::HashSet::new();
    for port in ports {
        let key = port.name.to_ascii_lowercase();
        if !names.insert(key) {
            return Err(format!(
                "The interface in '{source}' declares port '{}' more than once",
                port.name
            ));
        }
    }
    Ok(())
}

fn verilog_module_name(cell_name: &str) -> String {
    let mut sanitized = String::with_capacity(cell_name.len().saturating_add(11));
    for character in cell_name.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            sanitized.push(character);
        } else {
            sanitized.push('_');
        }
    }
    if sanitized.is_empty() {
        sanitized.push_str("model");
    }
    format!("rspice_{sanitized}_va")
}

fn is_verilog_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn starter_veriloga_source(module_name: &str, ports: &[crate::state::PortSpec]) -> String {
    use crate::state::PortDirection;

    let names = ports
        .iter()
        .map(|port| port.name.as_str())
        .collect::<Vec<_>>();
    let mut source = String::from("`include \"constants.vams\"\n`include \"disciplines.vams\"\n\n");
    if names.is_empty() {
        source.push_str(&format!("module {module_name};\nendmodule\n"));
        return source;
    }
    source.push_str(&format!("module {module_name}({});\n", names.join(", ")));
    for (keyword, direction) in [
        ("input", PortDirection::In),
        ("output", PortDirection::Out),
        ("inout", PortDirection::InOut),
    ] {
        let group = ports
            .iter()
            .filter(|port| {
                port.direction == direction
                    || (direction == PortDirection::InOut
                        && port.direction == PortDirection::Supply)
            })
            .map(|port| port.name.as_str())
            .collect::<Vec<_>>();
        if !group.is_empty() {
            source.push_str(&format!("    {keyword} {};\n", group.join(", ")));
        }
    }
    source.push_str(&format!("    electrical {};\n", names.join(", ")));
    source.push_str("endmodule\n");
    source
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, Library, PortDirection, ProjectSourceBundle, ProjectSourceLanguage,
        ProjectSourceOwner, PropertyCommitPolicy, SymbolDocument, SymbolPin, View, ViewType,
    };
    use crate::workbench::ChoicePreference;

    #[test]
    fn creating_schematic_view_freezes_current_defaults_into_its_buffer() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("view_policy_test");
        library.add_cell(Cell::new("filter"));
        app.state.library_manager.add_library(library);
        app.state
            .ui
            .preferences
            .set_choice(ChoicePreference::PropertyCommitPolicy, 1)
            .unwrap();
        app.state.dialogs.new_view_library = "view_policy_test".to_owned();
        app.state.dialogs.new_view_cell = "filter".to_owned();
        app.state.dialogs.new_view_name = "schematic".to_owned();
        app.state.dialogs.new_view_type = crate::state::ViewType::Schematic;

        let outcome = app.handle_new_view_create_action();

        assert!(outcome.close);
        let key = crate::state::CellViewRef::new("view_policy_test", "filter", "schematic").key();
        let document = app
            .state
            .workspace
            .schematic_buffers
            .get(&key)
            .expect("new schematic view owns an explicit document buffer");
        assert_eq!(
            document.document_policy.property_commit,
            PropertyCommitPolicy::ApplyValidFields
        );
    }

    #[test]
    fn creation_catalog_contains_only_views_with_production_editors() {
        assert_eq!(
            CREATABLE_VIEW_TYPES,
            [
                crate::state::ViewType::Schematic,
                crate::state::ViewType::Testbench,
                crate::state::ViewType::Symbol,
                crate::state::ViewType::VerilogA,
            ]
        );
        assert!(CREATABLE_VIEW_TYPES.into_iter().all(is_creatable_view_type));
        assert!(
            crate::state::ViewType::ALL
                .into_iter()
                .filter(|view_type| !is_creatable_view_type(*view_type))
                .all(|view_type| !CREATABLE_VIEW_TYPES.contains(&view_type))
        );
    }

    #[test]
    fn unsupported_view_type_is_rejected_before_project_mutation() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("view_policy_test");
        library.add_cell(Cell::new("filter"));
        app.state.library_manager.add_library(library);
        app.state.dialogs.new_view_library = "view_policy_test".to_owned();
        app.state.dialogs.new_view_cell = "filter".to_owned();
        app.state.dialogs.new_view_name = "layout".to_owned();
        app.state.dialogs.new_view_type = crate::state::ViewType::Layout;

        let outcome = app.handle_new_view_create_action();

        assert!(!outcome.close);
        assert!(
            app.state
                .library_manager
                .get_library("view_policy_test")
                .and_then(|library| library.get_cell("filter"))
                .is_some_and(|cell| cell.get_view("layout").is_none())
        );
        assert!(
            app.state
                .dialogs
                .new_view_error
                .as_deref()
                .is_some_and(|error| error.contains("production editor"))
        );
    }

    #[test]
    fn new_view_rejects_accented_canonical_collision_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("identity_test");
        let mut cell = Cell::new("filter");
        cell.add_view(View::new("Mod\u{e8}le", ViewType::Symbol));
        library.add_cell(cell);
        app.state.library_manager.add_library(library);
        app.state.dialogs.new_view_library = "identity_test".to_owned();
        app.state.dialogs.new_view_cell = "filter".to_owned();
        app.state.dialogs.new_view_name = "MOD\u{c8}LE".to_owned();
        app.state.dialogs.new_view_type = ViewType::Schematic;
        let buffers_before = app.state.workspace.schematic_buffers.len();

        let outcome = app.handle_new_view_create_action();

        assert!(!outcome.close);
        let cell = app
            .state
            .library_manager
            .get_library("identity_test")
            .and_then(|library| library.get_cell("filter"))
            .expect("identity cell remains");
        assert_eq!(cell.view_count(), 1);
        assert!(cell.get_view("Mod\u{e8}le").is_some());
        assert_eq!(app.state.workspace.schematic_buffers.len(), buffers_before);
        assert!(
            app.state
                .dialogs
                .new_view_error
                .as_deref()
                .is_some_and(|error| error.contains("canonical view identity"))
        );
    }

    #[test]
    fn creating_veriloga_view_atomically_retains_contract_and_opens_exact_editor() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("behavioral_models");
        let mut cell = Cell::new("precision_amp");
        let mut symbol = View::new("symbol", ViewType::Symbol);
        let symbol_document = SymbolDocument {
            pins: vec![
                SymbolPin::new("inp", PortDirection::In, None),
                SymbolPin::new("inn", PortDirection::In, None),
                SymbolPin::new("out", PortDirection::Out, None),
                SymbolPin::new("vss", PortDirection::Supply, None),
            ],
            ..SymbolDocument::default()
        };
        symbol_document.store_in_view(&mut symbol).unwrap();
        cell.add_view(symbol);
        library.add_cell(cell);
        app.state.library_manager.add_library(library);
        app.state.dialogs.new_view_library = "behavioral_models".to_owned();
        app.state.dialogs.new_view_cell = "precision_amp".to_owned();
        app.state.dialogs.new_view_name = "veriloga".to_owned();
        app.state.dialogs.new_view_type = ViewType::VerilogA;

        let outcome = app.handle_new_view_create_action();

        assert!(outcome.close, "{:?}", app.state.dialogs.new_view_error);
        let reference =
            crate::state::CellViewRef::new("behavioral_models", "precision_amp", "veriloga");
        let owner = ProjectSourceOwner::cell_view(reference.clone());
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .expect("created view owns a sealed source bundle");
        assert_eq!(bundle.language(), ProjectSourceLanguage::VerilogA);
        assert!(
            bundle
                .root()
                .content()
                .contains("module rspice_precision_amp_va(inp, inn, out, vss);")
        );
        assert!(bundle.root().content().contains("input inp, inn;"));
        assert!(bundle.root().content().contains("output out;"));
        assert!(bundle.root().content().contains("inout vss;"));
        let view = app
            .state
            .library_manager
            .get_library("behavioral_models")
            .and_then(|library| library.get_cell("precision_amp"))
            .and_then(|cell| cell.get_view("veriloga"))
            .unwrap();
        assert_eq!(
            view.metadata.get("veriloga.module").map(String::as_str),
            Some("rspice_precision_amp_va")
        );
        assert_eq!(
            view.metadata.get("veriloga.ports").map(String::as_str),
            Some(r#"["inp","inn","out","vss"]"#)
        );
        assert_eq!(app.state.workspace.active_view, reference);
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Netlist
        );
        assert_eq!(
            app.state.ui.code_workspace.page,
            crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA
        );
        let selected = crate::workbench::documents::code_workspace::selected_veriloga_source(&app).unwrap();
        assert_eq!(selected.bundle().id(), bundle.id());
        assert_eq!(selected.selected_module(), Some("rspice_precision_amp_va"));
        assert!(selected.bundle().validation_is_current());
        assert!(app.state.ui.code_workspace.veriloga.receipt.is_some());
    }

    #[test]
    fn duplicate_cell_view_source_owner_rolls_back_view_creation() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("behavioral_models");
        library.add_cell(Cell::new("gain_stage"));
        app.state.library_manager.add_library(library);
        let reference =
            crate::state::CellViewRef::new("behavioral_models", "gain_stage", "veriloga");
        app.state
            .workspace
            .project_sources
            .insert_bundle(
                ProjectSourceBundle::try_new(
                    ProjectSourceOwner::cell_view(reference.clone()),
                    ProjectSourceLanguage::VerilogA,
                    "existing.va",
                    "module existing; endmodule\n",
                    [],
                    [],
                )
                .unwrap(),
            )
            .unwrap();
        let sources_before = app.state.workspace.project_sources.iter_bundles().count();
        app.state.dialogs.new_view_library = "behavioral_models".to_owned();
        app.state.dialogs.new_view_cell = "gain_stage".to_owned();
        app.state.dialogs.new_view_name = "veriloga".to_owned();
        app.state.dialogs.new_view_type = ViewType::VerilogA;

        let outcome = app.handle_new_view_create_action();

        assert!(!outcome.close);
        assert_eq!(
            app.state.workspace.project_sources.iter_bundles().count(),
            sources_before
        );
        assert!(
            app.state
                .library_manager
                .get_library("behavioral_models")
                .and_then(|library| library.get_cell("gain_stage"))
                .is_some_and(|cell| cell.get_view("veriloga").is_none())
        );
        assert!(app.state.dialogs.new_view_error.is_some());
    }
}
