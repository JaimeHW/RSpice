//! Authoritative schematic and retained-source binding materialization.

use super::*;
use crate::schematic::interface_repair::same_terminal_contract;

pub(super) fn is_project_virtual_source_path(path: &Path) -> bool {
    path.to_str()
        .is_some_and(|value| value.starts_with("__rspice_project__/"))
}

pub(super) fn materialize_schematic_binding(
    placed: &LibraryCellInstance,
    reference: &CellViewRef,
    schematic: &SchematicDocument,
) -> Result<LibraryCellInstance, String> {
    let ports = schematic.interface_ports();
    let authoritative = ports
        .iter()
        .map(|port| port.name.as_str())
        .collect::<Vec<_>>();
    if !placed.terminal_order.is_empty()
        && !same_terminal_contract(&placed.terminal_order, &authoritative)
    {
        return Err(format!(
            "placed interface for {}/{} is incompatible with authoritative schematic view '{}'",
            placed.library, placed.cell, reference.view
        ));
    }
    let mut materialized =
        LibraryCellInstance::new(&reference.library, &reference.cell, &reference.view);
    materialized.bind_interface(&ports);
    Ok(materialized)
}

pub fn materialize_authoritative_source_binding(
    placed: &LibraryCellInstance,
    library: &Library,
    cell: &Cell,
    view: &View,
    project_id: ProjectId,
    project_sources: &ProjectSourceRegistry,
    libraries: &LibraryCatalog,
) -> Result<LibraryCellInstance, String> {
    let reference = CellViewRef::new(&library.name, &cell.name, &view.name);
    let project_veriloga = (view.view_type == ViewType::VerilogA)
        .then(|| {
            project_veriloga_binding_for_view(project_id, project_sources, libraries, &reference)
        })
        .transpose()?;
    let source_path = if let Some(binding) = project_veriloga.as_ref() {
        PathBuf::from(binding.source_key())
    } else {
        view.file_path
            .clone()
            .or_else(|| metadata_source_path(&view.metadata).map(Path::to_path_buf))
            .or_else(|| metadata_source_path(&cell.metadata).map(Path::to_path_buf))
            .ok_or_else(|| {
                format!(
                    "authoritative source view {}/{}/{} has no source identity",
                    library.name, cell.name, view.name
                )
            })?
    };
    let terminal_order = metadata_terminal_names(&view.metadata)
        .or_else(|| metadata_terminal_names(&cell.metadata))
        .ok_or_else(|| {
            format!(
                "authoritative source view {}/{}/{} has no terminal contract",
                library.name, cell.name, view.name
            )
        })?;
    if !placed.terminal_order.is_empty()
        && !same_terminal_contract(&placed.terminal_order, &terminal_order)
    {
        return Err(format!(
            "placed interface for {}/{} is incompatible with authoritative source view '{}'",
            placed.library, placed.cell, view.name
        ));
    }
    let mut materialized = LibraryCellInstance::new(&library.name, &cell.name, &view.name);
    materialized.source_path = Some(source_path);
    materialized.module_name = project_veriloga
        .as_ref()
        .map(|binding| binding.netlist_alias().to_owned())
        .or_else(|| {
            metadata_value(
                [&view.metadata, &cell.metadata],
                &[
                    "veriloga.module",
                    "netlist.model",
                    "netlist.master",
                    "netlist.module",
                    "model.family",
                ],
            )
        });
    materialized.netlist_template = metadata_value(
        [&view.metadata, &cell.metadata],
        &["netlist.template", "netlist_template"],
    );
    materialized.inherit_variant_model_section(placed);
    materialized.model_section = placed
        .variant_model_section()
        .map(str::to_owned)
        .or_else(|| {
            metadata_value(
                [&view.metadata, &cell.metadata],
                &["netlist.section", "model.section"],
            )
        });
    materialized.reference_prefix = metadata_value(
        [&view.metadata, &cell.metadata],
        &["reference.prefix", "reference_prefix"],
    );
    materialized.parameter_order = metadata_terminal_names_for_keys(
        [&view.metadata, &cell.metadata],
        &["netlist.parameter_order"],
    )
    .unwrap_or_default();
    let ports = terminal_order
        .into_iter()
        .map(|name| PortSpec {
            name,
            direction: PortDirection::InOut,
        })
        .collect::<Vec<_>>();
    materialized.bind_interface(&ports);
    Ok(materialized)
}

pub fn project_veriloga_binding_for_view(
    project_id: ProjectId,
    project_sources: &ProjectSourceRegistry,
    libraries: &LibraryCatalog,
    reference: &CellViewRef,
) -> Result<ConfigurationVerilogABinding, String> {
    let library = find_library(libraries, &reference.library).ok_or_else(|| {
        format!(
            "project Verilog-A source owner {} has no authoritative library",
            reference.display_path()
        )
    })?;
    let cell = find_cell(library, &reference.cell).ok_or_else(|| {
        format!(
            "project Verilog-A source owner {} has no authoritative cell",
            reference.display_path()
        )
    })?;
    let view = find_view(cell, &reference.view).ok_or_else(|| {
        format!(
            "project Verilog-A source owner {} has no authoritative view",
            reference.display_path()
        )
    })?;
    if view.view_type != ViewType::VerilogA {
        return Err(format!(
            "project source owner {} is not a Verilog-A view",
            reference.display_path()
        ));
    }
    let owner = ProjectSourceOwner::cell_view(reference.clone());
    let bundle = project_sources.bundle_for_owner(&owner).ok_or_else(|| {
        format!(
            "Verilog-A view {} has no project-owned source bundle",
            reference.display_path()
        )
    })?;
    let selected_module = view
        .metadata
        .get("veriloga.module")
        .or_else(|| view.metadata.get("netlist.module"))
        .or_else(|| cell.metadata.get("veriloga.module"))
        .or_else(|| cell.metadata.get("netlist.module"))
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            format!(
                "Verilog-A view {} has no explicit module binding",
                reference.display_path()
            )
        })?
        .to_owned();
    let source_key = crate::project_sources::project_veriloga_bundle_source_key(
        project_id,
        bundle,
        &selected_module,
    )
    .map_err(|error| error.to_string())?;
    let netlist_alias =
        crate::project_sources::project_veriloga_bundle_alias(bundle, &selected_module)
            .map_err(|error| error.to_string())?;
    Ok(ConfigurationVerilogABinding {
        source_bundle_id: bundle.id(),
        source_closure_digest: bundle.closure_digest(),
        selected_module,
        source_key,
        netlist_alias,
    })
}

pub(super) fn metadata_terminal_names(metadata: &HashMap<String, String>) -> Option<Vec<String>> {
    let encoded = metadata
        .get("netlist.ports")
        .or_else(|| metadata.get("netlist.terminals"))
        .or_else(|| metadata.get("veriloga.ports"))?;
    let names = serde_json::from_str::<Vec<String>>(encoded).unwrap_or_else(|_| {
        encoded
            .split([',', ' ', '\t', '\n'])
            .filter_map(|name| {
                let name = name.trim();
                (!name.is_empty()).then(|| name.to_owned())
            })
            .collect()
    });
    (!names.is_empty()).then_some(names)
}

/// The first value any of these metadata maps states for any of these keys.
///
/// Maps are read in order and, within a map, keys in order, so a caller
/// spells precedence by argument order: the view's own metadata before the
/// cell it instantiates, the canonical key before the legacy spelling it
/// replaced.
///
/// A key present with a blank value is not a value. This metadata arrives
/// from imported libraries and hand-edited cells, where clearing a field is
/// how an author removes it, so a blank `netlist.template` falls through to
/// `netlist_template` rather than shadowing it. The result is trimmed for the
/// same reason: surrounding whitespace was never part of what was written.
///
/// The Models & PDKs symbol-contract table used to carry its own copy of
/// this, which read the keys in the same order but kept the untrimmed text.
pub fn metadata_value<const N: usize>(
    maps: [&HashMap<String, String>; N],
    keys: &[&str],
) -> Option<String> {
    maps.into_iter()
        .flat_map(|metadata| keys.iter().filter_map(|key| metadata.get(*key)))
        .map(|value| value.trim())
        .find(|value| !value.is_empty())
        .map(str::to_owned)
}

pub(super) fn metadata_terminal_names_for_keys<const N: usize>(
    maps: [&HashMap<String, String>; N],
    keys: &[&str],
) -> Option<Vec<String>> {
    let encoded = maps
        .into_iter()
        .find_map(|metadata| keys.iter().find_map(|key| metadata.get(*key)))?;
    let values = serde_json::from_str::<Vec<String>>(encoded).unwrap_or_else(|_| {
        encoded
            .split([',', ' ', '\t', '\n'])
            .filter_map(|value| {
                let value = value.trim();
                (!value.is_empty()).then(|| value.to_owned())
            })
            .collect()
    });
    (!values.is_empty()).then_some(values)
}

pub(super) fn metadata_source_path(metadata: &HashMap<String, String>) -> Option<&Path> {
    metadata
        .get("netlist.source_path")
        .or_else(|| metadata.get("veriloga.source_path"))
        .filter(|path| !path.trim().is_empty())
        .map(Path::new)
}

pub(super) fn hierarchy_identity(reference: &CellViewRef) -> String {
    reference.key().to_ascii_lowercase()
}

pub(super) fn hierarchy_display_path(reference: &CellViewRef) -> String {
    format!("{}/{}", reference.library, reference.cell)
}

pub(super) fn hierarchy_view_search_order(requested: &str, is_root: bool) -> Vec<String> {
    let requested = if requested.eq_ignore_ascii_case("symbol") {
        DEFAULT_SCHEMATIC_VIEW
    } else {
        requested
    };
    let mut order = vec![requested.to_owned()];
    match ViewType::from_name(requested) {
        ViewType::Schematic | ViewType::Testbench if !is_root => {
            order.push("extracted".to_owned());
            order.push("spice".to_owned());
        }
        ViewType::Schematic | ViewType::Testbench => order.push("spice".to_owned()),
        ViewType::Extracted | ViewType::Verilog | ViewType::VerilogA => {
            order.push("spice".to_owned());
        }
        ViewType::Spice => {}
        _ => order.push("spice".to_owned()),
    }
    order.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    order
}

pub(super) fn deduplicate_view_order(order: &mut Vec<String>) {
    let mut seen = HashSet::with_capacity(order.len());
    order.retain(|view| seen.insert(view.to_ascii_lowercase()));
}

/// The override that governs one instance: the matching scope that pins the
/// most positions by name.
///
/// Both operands are read with the grammar's legacy parser, so a scope stored
/// before the design root became implicit and one stored after it resolve to
/// the same instance rather than to two.
pub(super) fn selected_configuration_override<'a>(
    overrides: &'a [ConfigurationSetOverride],
    instance_path: &str,
) -> Option<&'a ConfigurationSetOverride> {
    let instance = InstancePath::parse_legacy(instance_path).ok()?;
    overrides
        .iter()
        .filter_map(|scoped| {
            let pattern = configured_scope(&scoped.instance_path).ok()?;
            pattern
                .matches(&instance)
                .then(|| (scoped, pattern.specificity()))
        })
        .max_by_key(|(_, specificity)| *specificity)
        .map(|(scoped, _)| scoped)
}

pub(super) fn validate_override_pattern_authority(
    configuration: &ConfigurationSet,
) -> Result<(), String> {
    let mut scopes = Vec::with_capacity(configuration.overrides().len());
    for scoped in configuration.overrides() {
        scopes.push(configured_scope(&scoped.instance_path)?);
    }
    for (index, left) in scopes.iter().enumerate() {
        for (offset, right) in scopes.iter().enumerate().skip(index + 1) {
            if left.specificity() == right.specificity() && left.overlaps(right) {
                return Err(format!(
                    "configuration overrides '{}' and '{}' overlap with equal specificity",
                    configuration.overrides()[index].instance_path,
                    configuration.overrides()[offset].instance_path
                ));
            }
        }
    }
    Ok(())
}

fn configured_scope(pattern: &str) -> Result<InstancePathPattern, String> {
    InstancePathPattern::parse_legacy(pattern)
        .map_err(|error| format!("configured scope '{pattern}' is not a hierarchy scope: {error}"))
}

pub fn hierarchy_stop_view(view_type: ViewType) -> bool {
    matches!(
        view_type,
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA | ViewType::Extracted
    )
}

pub(super) fn hierarchy_model_section(
    libraries: &LibraryCatalog,
    reference: &CellViewRef,
    binding: Option<&LibraryCellInstance>,
) -> String {
    let library = find_library(libraries, &reference.library);
    let cell = library.and_then(|library| find_cell(library, &reference.cell));
    let view = cell.and_then(|cell| find_view(cell, &reference.view));
    for metadata in [
        view.map(|view| &view.metadata),
        cell.map(|cell| &cell.metadata),
        library.map(|library| &library.metadata),
    ]
    .into_iter()
    .flatten()
    {
        for key in ["model_sections", "model_section", "sections", "section"] {
            if let Some(value) = metadata.get(key).filter(|value| !value.trim().is_empty()) {
                return value.clone();
            }
        }
    }
    if binding
        .and_then(|value| value.source_path.as_ref())
        .is_some()
    {
        "source-defined".to_owned()
    } else {
        "inherit PVT".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_override_patterns_use_most_specific_segment_match() {
        let overrides = vec![
            ConfigurationSetOverride {
                instance_path: "/top/*".to_owned(),
                executable_views: vec!["spice".to_owned()],
                stop_view: Some("spice".to_owned()),
                model_section: None,
                eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
            },
            ConfigurationSetOverride {
                instance_path: "/top/Xcritical".to_owned(),
                executable_views: vec!["schematic".to_owned()],
                stop_view: None,
                model_section: None,
                eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
            },
        ];
        let selected = selected_configuration_override(&overrides, "/top/xCRITICAL")
            .expect("specific override matches");
        assert_eq!(selected.instance_path, "/top/Xcritical");
        let wildcard = selected_configuration_override(&overrides, "/top/Xother")
            .expect("wildcard override matches");
        assert_eq!(wildcard.instance_path, "/top/*");
    }
}
