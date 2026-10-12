//! Shared, deterministic library/cell/view placement catalog.
//!
//! The component shelf and replacement workflows consume this one resolver so
//! source, interface, readiness, and CDF metadata cannot drift between UI
//! surfaces.

use std::collections::HashMap;

use rspice_design::hierarchy::source_view_source;
use rspice_design::library::metadata::{CellParameter, cell_parameter_contract, metadata_ports};

use super::{LibraryManager, ViewType};
use crate::state::{
    CellViewRef, LibraryCellInstance, ProjectWorkspace, validate_library_netlist_template,
};

#[derive(Debug, Clone)]
pub struct LibraryCellPlacementCandidate {
    pub library: String,
    pub cell: String,
    pub view: String,
    pub binding: LibraryCellInstance,
    pub parameters: Vec<CellParameter>,
    pub parameter_contract_error: Option<String>,
    pub ready: bool,
    pub unavailable_reason: String,
}

pub fn library_cell_placement_candidates(
    libraries: &LibraryManager,
    workspace: &ProjectWorkspace,
) -> Vec<LibraryCellPlacementCandidate> {
    let active = &workspace.content.active_view;
    let mut candidates = Vec::new();
    for library in libraries.libraries_sorted() {
        for cell in library.cells_sorted() {
            let mut views = cell
                .views_sorted()
                .into_iter()
                .filter(|view| {
                    matches!(
                        view.view_type,
                        ViewType::Schematic | ViewType::VerilogA | ViewType::Spice
                    )
                })
                .collect::<Vec<_>>();
            views.sort_by(|left, right| {
                view_preference(left.view_type)
                    .cmp(&view_preference(right.view_type))
                    .then_with(|| left.name.cmp(&right.name))
            });
            for view in views {
                let mut binding = LibraryCellInstance::new(&library.name, &cell.name, &view.name);
                let is_current = library.name == active.library && cell.name == active.cell;
                let (mut ready, mut unavailable_reason) = match view.view_type {
                    ViewType::Schematic => {
                        let reference = CellViewRef::new(&library.name, &cell.name, &view.name);
                        if let Some(master) =
                            workspace.content.schematic_buffers.get(&reference.key())
                        {
                            binding.bind_interface(&master.document().interface_ports());
                            (
                                !is_current,
                                if is_current {
                                    "current cell".to_owned()
                                } else {
                                    String::new()
                                },
                            )
                        } else {
                            (false, "open master first".to_owned())
                        }
                    }
                    ViewType::VerilogA | ViewType::Spice => {
                        // The netlister's own rule: a Verilog-A view runs the
                        // project bundle it owns, any other source view the
                        // file it names. A shelf that judged a Verilog-A view
                        // by a file path offered views no deck could bind and
                        // refused every one the project owns.
                        let source = source_view_source(
                            library,
                            cell,
                            view,
                            workspace.content.project.id(),
                            &workspace.content.project_sources,
                            libraries.catalog(),
                        )
                        .map(|source| source.path);
                        let ports = metadata_ports(&view.metadata)
                            .or_else(|| metadata_ports(&cell.metadata))
                            .unwrap_or_default();
                        if !ports.is_empty() {
                            binding.bind_interface(&ports);
                        }
                        binding.source_path = source.as_ref().ok().cloned();
                        binding.module_name = view
                            .metadata
                            .get("veriloga.module")
                            .or_else(|| view.metadata.get("netlist.model"))
                            .or_else(|| view.metadata.get("netlist.master"))
                            .or_else(|| view.metadata.get("netlist.module"))
                            .or_else(|| cell.metadata.get("veriloga.module"))
                            .or_else(|| cell.metadata.get("netlist.model"))
                            .or_else(|| cell.metadata.get("netlist.master"))
                            .or_else(|| cell.metadata.get("netlist.module"))
                            .or_else(|| cell.metadata.get("model.family"))
                            .cloned();
                        binding.netlist_template = metadata_owned(
                            [&view.metadata, &cell.metadata],
                            &["netlist.template", "netlist_template"],
                        );
                        binding.model_section = metadata_owned(
                            [&view.metadata, &cell.metadata],
                            &["netlist.section", "model.section"],
                        );
                        binding.reference_prefix = metadata_owned(
                            [&view.metadata, &cell.metadata],
                            &["reference.prefix", "reference_prefix"],
                        );
                        let template_error = binding
                            .netlist_template
                            .as_deref()
                            .map(validate_library_netlist_template)
                            .transpose()
                            .err();
                        let ready = source.is_ok()
                            && !binding.terminal_order.is_empty()
                            && template_error.is_none();
                        (
                            ready,
                            match template_error {
                                Some(error) => {
                                    format!("invalid model-bound netlist template: {error}")
                                }
                                None if ready => String::new(),
                                None if source.is_err() => "missing source".to_owned(),
                                None => "no ports".to_owned(),
                            },
                        )
                    }
                    _ => unreachable!("candidate view was filtered"),
                };
                let (parameters, parameter_contract_error) =
                    match cell_parameter_contract(cell, Some(view)) {
                        Ok(names) => (names, None),
                        Err(error) => (Vec::new(), Some(error)),
                    };
                if let Some(error) = parameter_contract_error.as_deref() {
                    ready = false;
                    unavailable_reason = format!("invalid typed parameter contract: {error}");
                }
                binding.parameter_order = parameters
                    .iter()
                    .map(|parameter| parameter.name.clone())
                    .collect();
                candidates.push(LibraryCellPlacementCandidate {
                    library: library.name.clone(),
                    cell: cell.name.clone(),
                    view: view.name.clone(),
                    binding,
                    parameters,
                    parameter_contract_error,
                    ready,
                    unavailable_reason,
                });
            }
        }
    }
    candidates
}

fn metadata_owned<const N: usize>(
    maps: [&HashMap<String, String>; N],
    keys: &[&str],
) -> Option<String> {
    maps.into_iter()
        .find_map(|metadata| {
            keys.iter()
                .find_map(|key| metadata.get(*key))
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
        })
        .map(str::to_owned)
}

const fn view_preference(view_type: ViewType) -> u8 {
    match view_type {
        ViewType::Schematic => 0,
        ViewType::VerilogA => 1,
        ViewType::Spice => 2,
        _ => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, Library, ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceOwner, View,
    };
    use std::path::PathBuf;

    #[test]
    fn a_schematic_cell_publishes_its_declared_parameters_onto_every_placement() {
        let mut cell = Cell::new("amp");
        cell.metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"r","default":"1k"},{"name":"c","required":true}]"#.to_owned(),
        );
        cell.add_view(View::new("schematic", ViewType::Schematic));
        let mut library = Library::new("work");
        library.add_cell(cell);
        let mut libraries = LibraryManager::new();
        libraries.add_library(library);

        let candidate = library_cell_placement_candidates(&libraries, &ProjectWorkspace::default())
            .pop()
            .expect("schematic candidate");

        assert_eq!(candidate.binding.parameter_order, ["r", "c"]);
        assert_eq!(
            candidate
                .parameters
                .iter()
                .map(|parameter| (parameter.name.as_str(), parameter.required))
                .collect::<Vec<_>>(),
            [("r", false), ("c", true)]
        );
        assert!(candidate.parameter_contract_error.is_none());
    }

    #[test]
    fn a_schematic_cell_with_an_unusable_contract_cannot_be_placed() {
        let mut cell = Cell::new("amp");
        cell.metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"bad-name"}]"#.to_owned(),
        );
        cell.add_view(View::new("schematic", ViewType::Schematic));
        let mut library = Library::new("work");
        library.add_cell(cell);
        let mut libraries = LibraryManager::new();
        libraries.add_library(library);

        let candidate = library_cell_placement_candidates(&libraries, &ProjectWorkspace::default())
            .pop()
            .expect("schematic candidate is retained for diagnosis");

        assert!(!candidate.ready);
        assert!(
            candidate
                .unavailable_reason
                .contains("invalid typed parameter contract"),
            "{}",
            candidate.unavailable_reason
        );
        assert!(candidate.binding.parameter_order.is_empty());
    }

    /// `analog/amp` with a `veriloga` view stating `ports` and naming module
    /// `amp`, whose project bundle `workspace` owns.
    fn project_veriloga_cell(workspace: &mut ProjectWorkspace, ports: &str) -> LibraryManager {
        let mut view = View::new("veriloga", ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "amp".to_owned());
        view.metadata
            .insert("veriloga.ports".to_owned(), ports.to_owned());
        let mut cell = Cell::new("amp");
        cell.add_view(view);
        let mut library = Library::new("analog");
        library.add_cell(cell);
        let mut libraries = LibraryManager::new();
        libraries.add_library(library);
        workspace
            .content
            .project_sources
            .insert_bundle(
                ProjectSourceBundle::try_new(
                    ProjectSourceOwner::cell_view(CellViewRef::new("analog", "amp", "veriloga")),
                    ProjectSourceLanguage::VerilogA,
                    "amp.va",
                    "module amp(inp, out);\n  input inp;\n  output out;\n  electrical inp, out;\nendmodule\n",
                    [],
                    [],
                )
                .expect("a cell-view bundle"),
            )
            .expect("one bundle per cell view");
        libraries
    }

    #[test]
    fn catalog_enumerates_every_ready_implementation_view_deterministically() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = project_veriloga_cell(&mut workspace, r#"["in","out"]"#);
        let mut fast =
            View::new("fast", ViewType::Spice).with_path(PathBuf::from("models/fast.sp"));
        fast.metadata
            .insert("netlist.ports".to_owned(), "in,out".to_owned());
        libraries
            .get_library_mut("analog")
            .and_then(|library| library.get_cell_mut("amp"))
            .expect("the cell")
            .add_view(fast);

        let candidates = library_cell_placement_candidates(&libraries, &workspace);
        assert_eq!(
            candidates
                .iter()
                .map(|candidate| candidate.view.as_str())
                .collect::<Vec<_>>(),
            ["veriloga", "fast"]
        );
        assert!(candidates.iter().all(|candidate| candidate.ready));
    }

    /// A Verilog-A cell the project owns has no file path: its source is the
    /// bundle its view owns, which is what the netlister binds. The shelf used
    /// to ask for a path and called every such cell "missing source or ports".
    #[test]
    fn a_project_owned_veriloga_cell_with_ports_is_ready_to_place() {
        let mut workspace = ProjectWorkspace::default();
        let libraries = project_veriloga_cell(
            &mut workspace,
            r#"[{"name":"inp","direction":"in","discipline":"electrical","domain":"continuous"},{"name":"out","direction":"out","discipline":"electrical","domain":"continuous"}]"#,
        );

        let candidate = library_cell_placement_candidates(&libraries, &workspace)
            .pop()
            .expect("the Verilog-A candidate");

        assert!(candidate.ready, "{}", candidate.unavailable_reason);
        assert_eq!(candidate.binding.terminal_order, ["inp", "out"]);
        assert!(
            candidate
                .binding
                .source_path
                .as_deref()
                .and_then(std::path::Path::to_str)
                .is_some_and(|key| key.starts_with("__rspice_project__/")),
            "{:?}",
            candidate.binding.source_path
        );
        assert_eq!(candidate.binding.module_name.as_deref(), Some("amp"));
    }

    /// The same rule refuses what the netlister would: a Verilog-A view that
    /// names a file but owns no bundle has nothing a deck can bind, and one
    /// whose module has no ports has nothing to connect.
    #[test]
    fn a_veriloga_view_without_a_bundle_or_ports_is_not_ready() {
        let mut workspace = ProjectWorkspace::default();
        let libraries = project_veriloga_cell(&mut workspace, "[]");
        let portless = library_cell_placement_candidates(&libraries, &workspace)
            .pop()
            .expect("the Verilog-A candidate");
        assert!(!portless.ready);
        assert_eq!(portless.unavailable_reason, "no ports");

        let mut file_only =
            View::new("veriloga", ViewType::VerilogA).with_path(PathBuf::from("models/amp.va"));
        file_only
            .metadata
            .insert("veriloga.module".to_owned(), "amp".to_owned());
        file_only
            .metadata
            .insert("veriloga.ports".to_owned(), "in,out".to_owned());
        let mut cell = Cell::new("amp");
        cell.add_view(file_only);
        let mut library = Library::new("analog");
        library.add_cell(cell);
        let mut libraries = LibraryManager::new();
        libraries.add_library(library);
        let unbound = library_cell_placement_candidates(&libraries, &ProjectWorkspace::default())
            .pop()
            .expect("the Verilog-A candidate");
        assert!(!unbound.ready);
        assert_eq!(unbound.unavailable_reason, "missing source");
    }

    #[test]
    fn catalog_projects_model_bound_execution_contract_and_rejects_bad_templates() {
        let mut spice =
            View::new("spice", ViewType::Spice).with_path(PathBuf::from("C:/models/foundry.lib"));
        spice
            .metadata
            .insert("netlist.ports".to_owned(), "d,g,s,b".to_owned());
        spice.metadata.insert(
            "netlist.template".to_owned(),
            "M{name} {nodes} {model} {params}".to_owned(),
        );
        spice
            .metadata
            .insert("netlist.model".to_owned(), "nmos_18".to_owned());
        spice
            .metadata
            .insert("netlist.section".to_owned(), "TT".to_owned());
        spice
            .metadata
            .insert("reference.prefix".to_owned(), "M".to_owned());
        let mut cell = Cell::new("nmos_18");
        cell.add_view(spice);
        let mut library = Library::new("models");
        library.add_cell(cell);
        let mut libraries = LibraryManager::new();
        libraries.add_library(library);

        let candidate = library_cell_placement_candidates(&libraries, &ProjectWorkspace::default())
            .pop()
            .expect("model candidate");
        assert!(candidate.ready, "{}", candidate.unavailable_reason);
        assert_eq!(candidate.binding.module_name.as_deref(), Some("nmos_18"));
        assert_eq!(candidate.binding.model_section.as_deref(), Some("TT"));
        assert_eq!(candidate.binding.reference_prefix.as_deref(), Some("M"));

        libraries
            .get_library_mut("models")
            .and_then(|library| library.get_cell_mut("nmos_18"))
            .and_then(|cell| cell.get_view_mut("spice"))
            .expect("model implementation")
            .metadata
            .insert(
                "netlist.template".to_owned(),
                ".include {nodes} {model}".to_owned(),
            );
        let invalid = library_cell_placement_candidates(&libraries, &ProjectWorkspace::default())
            .pop()
            .expect("invalid model candidate is retained for diagnosis");
        assert!(!invalid.ready);
        assert!(invalid.unavailable_reason.contains("invalid model-bound"));

        let implementation = libraries
            .get_library_mut("models")
            .and_then(|library| library.get_cell_mut("nmos_18"))
            .and_then(|cell| cell.get_view_mut("spice"))
            .expect("model implementation");
        implementation.metadata.insert(
            "netlist.template".to_owned(),
            "M{name} {nodes} {model} {params}".to_owned(),
        );
        implementation.metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"bad-name"}]"#.to_owned(),
        );
        let invalid_form =
            library_cell_placement_candidates(&libraries, &ProjectWorkspace::default())
                .pop()
                .expect("invalid parameter form remains diagnosable");
        assert!(!invalid_form.ready);
        assert!(
            invalid_form
                .unavailable_reason
                .contains("invalid typed parameter contract")
        );
    }
}
