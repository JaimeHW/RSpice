//! Shared, deterministic library/cell/view placement catalog.
//!
//! The component shelf and replacement workflows consume this one resolver so
//! source, interface, readiness, and CDF metadata cannot drift between UI
//! surfaces.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use serde::Deserialize;

use super::{Cell, LibraryManager, View, ViewType};
use crate::state::{
    CellViewRef, InstanceMultiplicity, LibraryCellInstance, PortDirection, PortSpec,
    ProjectWorkspace, validate_library_netlist_template,
};

#[derive(Debug, Clone)]
pub struct LibraryCellPlacementCandidate {
    pub library: String,
    pub cell: String,
    pub view: String,
    pub binding: LibraryCellInstance,
    pub parameters: Vec<LibraryCellPlacementParameter>,
    pub parameter_contract_error: Option<String>,
    pub ready: bool,
    pub unavailable_reason: String,
}

/// Lossless parameter contract authored by a cell or concrete implementation
/// view.  Legacy name lists remain supported as optional parameters, while a
/// structured contract carries aliases, requiredness, and exact defaults.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryCellPlacementParameter {
    pub name: String,
    pub aliases: Vec<String>,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct EncodedParameterContract {
    name: String,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    required: bool,
    #[serde(default, alias = "default_value")]
    default: Option<String>,
}

pub fn library_cell_placement_candidates(
    libraries: &LibraryManager,
    workspace: &ProjectWorkspace,
) -> Vec<LibraryCellPlacementCandidate> {
    let active = &workspace.active_view;
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
                        if let Some(master) = workspace.schematic_buffers.get(&reference.key()) {
                            binding.bind_interface(&master.interface_ports());
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
                        let source = view
                            .file_path
                            .clone()
                            .or_else(|| metadata_path(&view.metadata))
                            .or_else(|| metadata_path(&cell.metadata));
                        let ports = metadata_ports(&view.metadata)
                            .or_else(|| metadata_ports(&cell.metadata))
                            .unwrap_or_default();
                        if !ports.is_empty() {
                            binding.bind_interface(&ports);
                        }
                        binding.source_path = source;
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
                        let ready = binding.source_path.is_some()
                            && !binding.terminal_order.is_empty()
                            && template_error.is_none();
                        (
                            ready,
                            match template_error {
                                Some(error) => {
                                    format!("invalid model-bound netlist template: {error}")
                                }
                                None if ready => String::new(),
                                None => "missing source or ports".to_owned(),
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

fn metadata_path(metadata: &HashMap<String, String>) -> Option<PathBuf> {
    metadata
        .get("netlist.source_path")
        .or_else(|| metadata.get("veriloga.source_path"))
        .filter(|path| !path.trim().is_empty())
        .map(PathBuf::from)
}

fn metadata_ports(metadata: &HashMap<String, String>) -> Option<Vec<PortSpec>> {
    let encoded = metadata
        .get("netlist.ports")
        .or_else(|| metadata.get("netlist.terminals"))
        .or_else(|| metadata.get("veriloga.ports"))?;
    let names = parse_name_list(encoded);
    Some(
        names
            .into_iter()
            .map(|name| PortSpec {
                name,
                direction: PortDirection::InOut,
            })
            .collect(),
    )
}

/// The parameters one cellview declares: the cell's contract with the view's
/// laid over it, name by name.
///
/// This is the single reader of the CDF contract metadata. Placement writes
/// the resulting order onto every instance and the hierarchy resolver writes
/// the same declarations into `.subckt … params:`, so neither can invent a
/// parameter the other has never heard of.
pub(crate) fn cell_parameter_contract(
    cell: &Cell,
    view: Option<&View>,
) -> Result<Vec<LibraryCellPlacementParameter>, String> {
    merge_parameter_contracts(
        metadata_parameter_contract(&cell.metadata),
        view.map_or_else(
            || Ok(Vec::new()),
            |view| metadata_parameter_contract(&view.metadata),
        ),
    )
}

fn metadata_parameter_contract(
    metadata: &HashMap<String, String>,
) -> Result<Vec<LibraryCellPlacementParameter>, String> {
    if let Some(encoded) = metadata
        .get("cdf.parameter_contract")
        .or_else(|| metadata.get("netlist.parameter_contract"))
        .or_else(|| metadata.get("veriloga.parameter_contract"))
    {
        let encoded = serde_json::from_str::<Vec<EncodedParameterContract>>(encoded)
            .map_err(|error| format!("parameter contract JSON is invalid: {error}"))?;
        let parameters = encoded
            .into_iter()
            .map(|parameter| LibraryCellPlacementParameter {
                name: parameter.name,
                aliases: parameter.aliases,
                required: parameter.required,
                default_value: parameter.default,
            })
            .collect::<Vec<_>>();
        validate_parameter_contract(&parameters)?;
        return Ok(parameters);
    }
    metadata
        .get("cdf.parameters")
        .or_else(|| metadata.get("netlist.parameters"))
        .or_else(|| metadata.get("veriloga.parameters"))
        .map_or_else(
            || Ok(Vec::new()),
            |encoded| {
                parse_parameter_names(encoded).map(|names| {
                    names
                        .into_iter()
                        .map(|name| LibraryCellPlacementParameter {
                            name,
                            aliases: Vec::new(),
                            required: false,
                            default_value: None,
                        })
                        .collect()
                })
            },
        )
}

fn parse_name_list(encoded: &str) -> Vec<String> {
    let parsed = serde_json::from_str::<Vec<String>>(encoded).unwrap_or_else(|_| {
        encoded
            .split([',', ' ', '\t', '\n'])
            .filter_map(|name| {
                let name = name.trim();
                (!name.is_empty()).then(|| name.to_owned())
            })
            .collect()
    });
    let mut seen = HashSet::new();
    parsed
        .into_iter()
        .filter(|name| seen.insert(name.to_ascii_lowercase()))
        .collect()
}

fn parse_parameter_names(encoded: &str) -> Result<Vec<String>, String> {
    let names = parse_name_list(encoded);
    if names.is_empty() && !encoded.trim().is_empty() {
        return Err("parameter metadata does not contain a valid name".to_owned());
    }
    for name in &names {
        validate_parameter_name(name)?;
    }
    Ok(names)
}

fn validate_parameter_contract(parameters: &[LibraryCellPlacementParameter]) -> Result<(), String> {
    let mut owners = HashMap::<String, String>::new();
    for parameter in parameters {
        for name in std::iter::once(&parameter.name).chain(&parameter.aliases) {
            validate_parameter_name(name)?;
            let key = name.to_ascii_lowercase();
            if let Some(owner) = owners.insert(key.clone(), parameter.name.clone()) {
                return Err(format!(
                    "parameter spelling `{key}` is owned by both `{owner}` and `{}`",
                    parameter.name
                ));
            }
        }
    }
    Ok(())
}

fn validate_parameter_name(name: &str) -> Result<(), String> {
    let mut chars = name.chars();
    let valid_first = chars
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_');
    if !valid_first || chars.any(|character| !character.is_ascii_alphanumeric() && character != '_')
    {
        return Err(format!("`{name}` is not a valid parameter name"));
    }
    if name.eq_ignore_ascii_case(InstanceMultiplicity::PARAMETER_NAME) {
        return Err(InstanceMultiplicity::RESERVED_GUIDANCE.to_owned());
    }
    Ok(())
}

fn merge_parameter_contracts(
    left: Result<Vec<LibraryCellPlacementParameter>, String>,
    right: Result<Vec<LibraryCellPlacementParameter>, String>,
) -> Result<Vec<LibraryCellPlacementParameter>, String> {
    let mut merged = left?;
    for parameter in right? {
        if let Some(existing) = merged
            .iter_mut()
            .find(|existing| existing.name.eq_ignore_ascii_case(&parameter.name))
        {
            *existing = parameter;
        } else {
            merged.push(parameter);
        }
    }
    validate_parameter_contract(&merged)?;
    Ok(merged)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Cell, Library, View};

    #[test]
    fn port_metadata_accepts_json_and_legacy_lists() {
        let mut metadata = HashMap::new();
        metadata.insert("veriloga.ports".to_owned(), r#"["in","out"]"#.to_owned());
        assert_eq!(metadata_ports(&metadata).unwrap().len(), 2);
        metadata.insert("veriloga.ports".to_owned(), "in, out vss".to_owned());
        assert_eq!(metadata_ports(&metadata).unwrap().len(), 3);
    }

    #[test]
    fn invalid_parameter_contract_is_not_silently_coerced() {
        assert!(parse_parameter_names("gain bad-name").is_err());
    }

    #[test]
    fn structured_parameter_contract_preserves_required_defaults_and_aliases() {
        let mut metadata = HashMap::new();
        metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"gain","aliases":["av"],"required":true,"default":"10"}]"#.to_owned(),
        );
        let parameters = metadata_parameter_contract(&metadata).unwrap();
        assert_eq!(
            parameters,
            [LibraryCellPlacementParameter {
                name: "gain".to_owned(),
                aliases: vec!["av".to_owned()],
                required: true,
                default_value: Some("10".to_owned()),
            }]
        );
    }

    #[test]
    fn structured_parameter_contract_rejects_ambiguous_aliases_and_unknown_fields() {
        let mut metadata = HashMap::new();
        metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"gain","aliases":["av"]},{"name":"av"}]"#.to_owned(),
        );
        assert!(metadata_parameter_contract(&metadata).is_err());
        metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"gain","type":"real"}]"#.to_owned(),
        );
        assert!(metadata_parameter_contract(&metadata).is_err());
    }

    #[test]
    fn the_multiplicity_spelling_cannot_be_declared_as_a_cell_parameter() {
        let mut metadata = HashMap::new();
        metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"M"}]"#.to_owned(),
        );
        assert_eq!(
            metadata_parameter_contract(&metadata).unwrap_err(),
            InstanceMultiplicity::RESERVED_GUIDANCE
        );

        metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"gain","aliases":["m"]}]"#.to_owned(),
        );
        assert_eq!(
            metadata_parameter_contract(&metadata).unwrap_err(),
            InstanceMultiplicity::RESERVED_GUIDANCE
        );

        let legacy = HashMap::from([("cdf.parameters".to_owned(), "gain, m".to_owned())]);
        assert_eq!(
            metadata_parameter_contract(&legacy).unwrap_err(),
            InstanceMultiplicity::RESERVED_GUIDANCE
        );
    }

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

    #[test]
    fn catalog_enumerates_every_ready_implementation_view_deterministically() {
        let mut fast =
            View::new("fast", ViewType::Spice).with_path(PathBuf::from("models/fast.sp"));
        fast.metadata
            .insert("netlist.ports".to_owned(), "in,out".to_owned());
        let mut accurate = View::new("accurate", ViewType::VerilogA)
            .with_path(PathBuf::from("models/accurate.va"));
        accurate
            .metadata
            .insert("veriloga.ports".to_owned(), "in,out".to_owned());
        let mut cell = Cell::new("amp");
        cell.add_view(fast);
        cell.add_view(accurate);
        let mut library = Library::new("analog");
        library.add_cell(cell);
        let mut libraries = LibraryManager::new();
        libraries.add_library(library);

        let candidates =
            library_cell_placement_candidates(&libraries, &ProjectWorkspace::default());
        assert_eq!(
            candidates
                .iter()
                .map(|candidate| candidate.view.as_str())
                .collect::<Vec<_>>(),
            ["accurate", "fast"]
        );
        assert!(candidates.iter().all(|candidate| candidate.ready));
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
