//! Materializing an executable binding from a placed instance.
//!
//! A placed schematic binding is never used as execution authority: the
//! executable form is rebuilt from the resolved Library/Cell/View and its
//! authoritative view metadata, so editing a symbol's placement cannot change
//! what gets netlisted. Configuration overrides are matched by pattern
//! specificity, and two patterns that overlap at equal specificity are an
//! error rather than a silent first-match win.

use super::*;

pub(super) fn is_project_virtual_source_path(path: &Path) -> bool {
    path.to_str()
        .is_some_and(|value| value.starts_with("__rspice_project__/"))
}

pub(super) fn translated_point(
    point: crate::state::Point,
    delta: crate::state::Point,
) -> Result<crate::state::Point, crate::state::DesignManagementError> {
    Ok(crate::state::Point::new(
        point
            .x
            .checked_add(delta.x)
            .ok_or(crate::state::DesignManagementError::NumericRange(
                "materialized sheet x coordinate",
            ))?,
        point
            .y
            .checked_add(delta.y)
            .ok_or(crate::state::DesignManagementError::NumericRange(
                "materialized sheet y coordinate",
            ))?,
    ))
}

/// Resolve one typed cross-sheet endpoint against the authored topology and
/// then project it into the endpoint sheet's execution namespace. A wire
/// point must still lie on its retained conductor; a component terminal must
/// still own a canonical wire connection. Stale contracts fail before DRC or
/// netlisting rather than silently connecting a label to a component origin.
pub(super) fn projected_cross_sheet_anchor(
    source: &SchematicState,
    projected: &SchematicState,
    endpoint: &crate::state::CrossSheetPortEndpoint,
    delta: crate::state::Point,
) -> Result<crate::state::Point, crate::state::DesignManagementError> {
    let authored_point = match &endpoint.anchor {
        crate::state::CrossSheetPortAnchor::WirePoint { wire_id, point } => {
            let wire = source
                .wires
                .iter()
                .find(|wire| wire.id == *wire_id)
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet wire anchor",
                    identity: wire_id.to_string(),
                })?;
            if !wire.contains_point(*point) {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet wire anchor point",
                    identity: format!("{}@{},{}", wire_id, point.x, point.y),
                });
            }
            *point
        }
        crate::state::CrossSheetPortAnchor::ComponentTerminal {
            component_id,
            terminal_name,
        } => {
            if !source
                .components
                .iter()
                .any(|component| component.id == *component_id)
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component anchor",
                    identity: component_id.to_string(),
                });
            }
            let connection = source
                .connections
                .iter()
                .find(|connection| {
                    connection.component_id == *component_id
                        && connection.terminal_name.eq_ignore_ascii_case(terminal_name)
                })
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component terminal connection",
                    identity: format!("{}:{}", component_id, terminal_name),
                })?;
            source
                .wires
                .iter()
                .find(|wire| wire.id == connection.wire_id)
                .and_then(|wire| wire.points.get(connection.point_index))
                .copied()
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component terminal wire point",
                    identity: format!("{}:{}", connection.wire_id, connection.point_index),
                })?
        }
    };
    let anchor = translated_point(authored_point, delta)?;
    match &endpoint.anchor {
        crate::state::CrossSheetPortAnchor::WirePoint { wire_id, .. } => {
            if !projected
                .wires
                .iter()
                .any(|wire| wire.id == *wire_id && wire.contains_point(anchor))
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "projected cross-sheet wire anchor",
                    identity: wire_id.to_string(),
                });
            }
        }
        crate::state::CrossSheetPortAnchor::ComponentTerminal { component_id, .. } => {
            if !projected
                .components
                .iter()
                .any(|component| component.id == *component_id)
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "projected cross-sheet component anchor",
                    identity: component_id.to_string(),
                });
            }
        }
    }
    Ok(anchor)
}

pub(super) fn materialize_schematic_binding(
    placed: &LibraryCellInstance,
    reference: &CellViewRef,
    schematic: &SchematicState,
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

pub(super) fn materialize_authoritative_source_binding(
    placed: &LibraryCellInstance,
    library: &Library,
    cell: &Cell,
    view: &View,
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
) -> Result<LibraryCellInstance, String> {
    let reference = CellViewRef::new(&library.name, &cell.name, &view.name);
    let project_veriloga = (view.view_type == ViewType::VerilogA)
        .then(|| project_veriloga_binding_for_view(workspace, libraries, &reference))
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
            view.metadata
                .get("veriloga.module")
                .or_else(|| view.metadata.get("netlist.module"))
                .or_else(|| cell.metadata.get("veriloga.module"))
                .or_else(|| cell.metadata.get("netlist.module"))
                .cloned()
        });
    materialized.netlist_template = metadata_value(
        [&view.metadata, &cell.metadata],
        &["netlist.template", "netlist_template"],
    );
    materialized.model_section = metadata_value(
        [&view.metadata, &cell.metadata],
        &["netlist.section", "model.section"],
    );
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
        .map(|name| crate::state::PortSpec {
            name,
            direction: crate::state::PortDirection::InOut,
        })
        .collect::<Vec<_>>();
    materialized.bind_interface(&ports);
    Ok(materialized)
}

pub(crate) fn project_veriloga_binding_for_view(
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
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
    let bundle = workspace
        .project_sources
        .bundle_for_owner(&owner)
        .ok_or_else(|| {
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
    let source_key = super::super::project_sources::project_veriloga_bundle_source_key(
        workspace.project.id(),
        bundle,
        &selected_module,
    )
    .map_err(|error| error.to_string())?;
    let netlist_alias =
        super::super::project_sources::project_veriloga_bundle_alias(bundle, &selected_module)
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

pub(super) fn metadata_value<const N: usize>(
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

/// Whether a placed interface still presents the authoritative one.
///
/// This is the crate's one answer to that question: the resolver asks it when
/// it materializes a binding, and netlist generation asks it again — through
/// its own typed defect — before it emits an instance. Two spellings of the
/// comparison would let a binding pass materialization and fail emission for
/// reasons that disagree.
pub(crate) fn same_terminal_contract<L, R>(placed: &[L], authoritative: &[R]) -> bool
where
    L: AsRef<str>,
    R: AsRef<str>,
{
    placed.len() == authoritative.len()
        && placed
            .iter()
            .zip(authoritative)
            .all(|(left, right)| left.as_ref().eq_ignore_ascii_case(right.as_ref()))
}

pub(super) fn metadata_source_path(metadata: &HashMap<String, String>) -> Option<&Path> {
    metadata
        .get("netlist.source_path")
        .or_else(|| metadata.get("veriloga.source_path"))
        .filter(|path| !path.trim().is_empty())
        .map(Path::new)
}

pub(super) fn source_paths_match(left: &Path, right: &Path) -> bool {
    if left == right {
        return true;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        match (std::fs::canonicalize(left), std::fs::canonicalize(right)) {
            (Ok(left), Ok(right)) => left == right,
            _ => false,
        }
    }
    #[cfg(target_arch = "wasm32")]
    false
}

pub(super) fn configured_source_identity(path: &Path) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    #[cfg(target_arch = "wasm32")]
    let path = path.to_path_buf();
    path.to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase()
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn validate_source_file(
    source_path: &Path,
    view_type: ViewType,
    binding: &LibraryCellInstance,
) -> Result<(), String> {
    let source = if let Some(section) = binding.model_section.as_deref() {
        let mut processor = rspice_core::netlist::IncludeProcessor::new(source_path);
        processor
            .process_lib(&source_path.to_string_lossy(), Some(section))
            .map_err(|error| {
                format!(
                    "source-backed binding {}/{}/{} cannot resolve model section '{}' from {}: {error}",
                    binding.library,
                    binding.cell,
                    binding.view,
                    section,
                    source_path.display()
                )
            })?
    } else {
        std::fs::read_to_string(source_path).map_err(|error| {
            format!(
                "source-backed binding {}/{}/{} cannot read {}: {error}",
                binding.library,
                binding.cell,
                binding.view,
                source_path.display()
            )
        })?
    };
    let master = binding
        .module_name
        .as_deref()
        .filter(|name| !name.trim().is_empty())
        .unwrap_or(&binding.cell);
    let declaration_found = match view_type {
        ViewType::Verilog | ViewType::VerilogA => source.lines().any(|line| {
            let code = line.split("//").next().unwrap_or_default();
            let mut tokens = code
                .split(|character: char| !character.is_alphanumeric() && character != '_')
                .filter(|token| !token.is_empty());
            tokens.any(|token| token.eq_ignore_ascii_case("module"))
                && tokens.any(|token| token.eq_ignore_ascii_case(master))
        }),
        ViewType::Spice | ViewType::Extracted => source.lines().any(|line| {
            let mut tokens = line.split_ascii_whitespace();
            let directive = tokens.next();
            let declared_name = tokens.next();
            let subcircuit_matches = directive
                .is_some_and(|token| token.eq_ignore_ascii_case(".subckt"))
                && declared_name.is_some_and(|token| token.eq_ignore_ascii_case(master));
            let model_matches = binding.netlist_template.is_some()
                && directive.is_some_and(|token| token.eq_ignore_ascii_case(".model"))
                && declared_name.is_some_and(|token| token.eq_ignore_ascii_case(master));
            subcircuit_matches || model_matches
        }),
        _ => false,
    };
    if declaration_found {
        Ok(())
    } else {
        Err(format!(
            "source-backed binding {}/{}/{} does not declare executable master {master}",
            binding.library, binding.cell, binding.view
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn validate_configured_model_section(
    source_path: &Path,
    section: &str,
) -> Result<(), String> {
    let mut processor = rspice_core::netlist::IncludeProcessor::new(source_path);
    processor
        .process_lib(&source_path.to_string_lossy(), Some(section))
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[cfg(target_arch = "wasm32")]
pub(super) fn validate_configured_model_section(
    _source_path: &Path,
    _section: &str,
) -> Result<(), String> {
    Err("filesystem-backed model sections are unavailable in this browser session".to_owned())
}

#[cfg(target_arch = "wasm32")]
pub(super) fn validate_source_file(
    _source_path: &Path,
    _view_type: ViewType,
    binding: &LibraryCellInstance,
) -> Result<(), String> {
    Err(format!(
        "source-backed binding {}/{}/{} references a desktop path unavailable in this browser session",
        binding.library, binding.cell, binding.view
    ))
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
    overrides: &'a [crate::state::ConfigurationSetOverride],
    instance_path: &str,
) -> Option<&'a crate::state::ConfigurationSetOverride> {
    let instance = crate::state::InstancePath::parse_legacy(instance_path).ok()?;
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
    configuration: &crate::state::ConfigurationSet,
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

fn configured_scope(pattern: &str) -> Result<crate::state::InstancePathPattern, String> {
    crate::state::InstancePathPattern::parse_legacy(pattern)
        .map_err(|error| format!("configured scope '{pattern}' is not a hierarchy scope: {error}"))
}

pub(super) fn hierarchy_stop_view(view_type: ViewType) -> bool {
    matches!(
        view_type,
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA | ViewType::Extracted
    )
}

pub(super) fn hierarchy_model_section(
    libraries: &LibraryManager,
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
