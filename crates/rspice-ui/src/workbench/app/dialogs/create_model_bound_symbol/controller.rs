//! Create-symbol dialog flow.

use crate::diagnostics::ConsoleMessage;
use crate::state::model_library::ModelType;
use crate::state::{
    CellViewRef, GeneratedSymbolViews, ModelBoundSymbolDefinition, ParameterInheritance,
    PortDirection, PortSpec, PropertyType, SymbolGraphicTemplate, SymbolIdentity,
    SymbolImplementationView, SymbolModelReference, SymbolNetlistBinding,
    SymbolParameterConstraints, SymbolParameterDefault, SymbolParameterField, SymbolParameterForm,
    SymbolParameterSection, SymbolParameterVisibility, SymbolPinDefinition, SymbolSourceContract,
};

use super::state::*;
use crate::workbench::app_state::{
    AppState, SymbolDefinitionFixtureDelta, publish_symbol_definition_candidate_with_fixture,
};

pub(crate) fn open_create_model_bound_symbol_dialog(state: &mut AppState) {
    let library = state
        .library_manager
        .selected_library
        .as_deref()
        .and_then(|name| {
            state
                .library_manager
                .get_library(name)
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
        })
        .or_else(|| {
            state
                .library_manager
                .libraries_sorted()
                .into_iter()
                .find(|library| !library.read_only)
                .map(|library| library.name.clone())
        });
    let Some(library) = library else {
        state.push_user_message(ConsoleMessage::warning(
            "Create symbol requires a writable design library.".to_owned(),
        ));
        return;
    };

    let selected_model = state
        .workbench
        .selected_model
        .as_deref()
        .and_then(|selected| {
            state
                .model_library_manager
                .search_models("")
                .into_iter()
                .find(|(_, model)| model.name.eq_ignore_ascii_case(selected))
        })
        .or_else(|| {
            state
                .model_library_manager
                .search_models("")
                .into_iter()
                .next()
        });
    let model_source = selected_model.map(|(source_library, model)| {
        let pins = inferred_model_pins(model.model_type);
        let section = source_library
            .selected_corner
            .as_deref()
            .and_then(|name| source_library.corners.get(name))
            .filter(|corner| corner.file_path.is_some())
            .map(|corner| corner.name.clone());
        let source_path = if section.is_some() {
            source_library.root_path.clone()
        } else {
            model
                .file_path
                .clone()
                .or_else(|| source_library.root_path.clone())
        };
        CreateSymbolModelSource {
            library: source_library.name.clone(),
            model: model.name.clone(),
            model_type: model.model_type,
            family: model_family_label(model.model_type).to_owned(),
            source_path,
            section,
            instance_parameters: model_instance_parameters(model),
            requires_pin_review: true,
            pins,
        }
    });

    let schematic_reference = state.workspace.active_schematic_reference();
    let schematic_ports = state.schematic.interface_ports();
    let schematic_source = (!schematic_ports.is_empty()).then(|| CreateSymbolSchematicSource {
        reference: schematic_reference,
        pins: schematic_ports
            .into_iter()
            .map(pin_from_port)
            .collect::<Vec<_>>(),
    });

    let source_mode = if model_source.is_some() {
        CreateSymbolSourceMode::Model
    } else if schematic_source.is_some() {
        CreateSymbolSourceMode::ExistingSchematicPins
    } else {
        CreateSymbolSourceMode::BlankExplicitContract
    };
    let pins = match source_mode {
        CreateSymbolSourceMode::Model => model_source
            .as_ref()
            .map(|source| source.pins.clone())
            .unwrap_or_default(),
        CreateSymbolSourceMode::ExistingSchematicPins => schematic_source
            .as_ref()
            .map(|source| source.pins.clone())
            .unwrap_or_default(),
        CreateSymbolSourceMode::BlankExplicitContract => Vec::new(),
    };
    let cell = model_source.as_ref().map_or_else(
        || "precision_opamp".to_owned(),
        |source| default_cell_name(&source.model),
    );
    let template = model_source
        .as_ref()
        .map_or(CreateSymbolTemplate::RectangularIc, |source| {
            default_template(source.model_type, &source.model)
        });

    state.dialogs.create_model_bound_symbol = CreateModelBoundSymbolDialogState {
        open: true,
        target: format!("{library} / {cell}"),
        source_mode,
        model_source,
        schematic_source,
        pins,
        template,
        pin_contract_reviewed: !matches!(source_mode, CreateSymbolSourceMode::Model),
        expected_library_revision: state.library_manager.revision(),
        ..CreateModelBoundSymbolDialogState::default()
    };
}

#[cfg(test)]
pub(crate) fn open_create_subcircuit_bound_symbol_dialog(
    state: &mut AppState,
    source_library: String,
    subcircuit: String,
    source_path: std::path::PathBuf,
    ordered_ports: Vec<String>,
    section: Option<String>,
    parameter_defaults: std::collections::BTreeMap<String, String>,
) -> Result<(), String> {
    let target_library = state
        .library_manager
        .selected_library
        .as_deref()
        .and_then(|name| {
            state
                .library_manager
                .get_library(name)
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
        })
        .or_else(|| {
            state
                .library_manager
                .libraries_sorted()
                .into_iter()
                .find(|library| !library.read_only)
                .map(|library| library.name.clone())
        })
        .ok_or_else(|| "Subcircuit symbol import requires a writable design library".to_owned())?;
    if ordered_ports.is_empty() {
        return Err(format!(
            "Subcircuit '{subcircuit}' has no explicit ordered terminal contract"
        ));
    }
    let pins = ordered_ports
        .into_iter()
        .enumerate()
        .map(|(index, port)| {
            CreateSymbolPinDraft::new(
                port,
                CreateSymbolPinType::AnalogBidirectional,
                if index.is_multiple_of(2) {
                    CreateSymbolPinSide::Left
                } else {
                    CreateSymbolPinSide::Right
                },
            )
        })
        .collect::<Vec<_>>();
    let cell = default_cell_name(&subcircuit);
    let model_source = CreateSymbolModelSource {
        library: source_library,
        model: subcircuit,
        model_type: ModelType::Other,
        family: "subcircuit".to_owned(),
        source_path: Some(source_path),
        section,
        instance_parameters: subcircuit_instance_parameters(parameter_defaults),
        requires_pin_review: true,
        pins: pins.clone(),
    };
    state.dialogs.create_model_bound_symbol = CreateModelBoundSymbolDialogState {
        open: true,
        target: format!("{target_library} / {cell}"),
        source_mode: CreateSymbolSourceMode::Model,
        model_source: Some(model_source),
        pins,
        template: CreateSymbolTemplate::RectangularIc,
        pin_contract_reviewed: false,
        expected_library_revision: state.library_manager.revision(),
        ..CreateModelBoundSymbolDialogState::default()
    };
    Ok(())
}

pub(super) fn validate_create_symbol_draft(state: &AppState) -> Result<(), String> {
    let draft = &state.dialogs.create_model_bound_symbol;
    if state.library_manager.revision() != draft.expected_library_revision {
        return Err(
            "The design library changed after this transaction opened. Close and reopen Create symbol."
                .to_owned(),
        );
    }
    let (library_name, cell_name) = parse_target(&draft.target)?;
    let library = state
        .library_manager
        .get_library(library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))?;
    if library.read_only {
        return Err(format!("Library '{library_name}' is read-only."));
    }
    validate_identifier(cell_name, "Cell name")?;
    if !draft.symbol && !draft.parameter_form && !draft.simulation_test_fixture {
        return Err("Select at least one generated view for this revision.".to_owned());
    }
    if draft.pins.is_empty() {
        return Err(
            "Define at least one explicit pin before creating the symbol revision.".to_owned(),
        );
    }
    let mut names = std::collections::HashSet::new();
    for (index, pin) in draft.pins.iter().enumerate() {
        validate_identifier(pin.name.trim(), &format!("Pin {} name", index + 1))?;
        if !names.insert(pin.name.trim().to_ascii_lowercase()) {
            return Err(format!("Pin name '{}' is repeated.", pin.name.trim()));
        }
    }
    if matches!(draft.source_mode, CreateSymbolSourceMode::Model) && !draft.pin_contract_reviewed {
        return Err(
            "Review and confirm the inferred model pin order and electrical types before commit."
                .to_owned(),
        );
    }
    match draft.source_mode {
        CreateSymbolSourceMode::Model => {
            let source = draft
                .model_source
                .as_ref()
                .ok_or_else(|| "The selected model contract is no longer available.".to_owned())?;
            if source.source_path.is_none() {
                return Err(format!(
                    "Model '{}' has no retained implementation source. The definition is unbound and non-placeable; import or attach its authenticated source before creating a model-bound symbol.",
                    source.model
                ));
            }
        }
        CreateSymbolSourceMode::ExistingSchematicPins => {
            let source = draft.schematic_source.as_ref().ok_or_else(|| {
                "The retained schematic pin contract is no longer available.".to_owned()
            })?;
            if !source.reference.library.eq_ignore_ascii_case(library_name)
                || !source.reference.cell.eq_ignore_ascii_case(cell_name)
            {
                return Err(format!(
                    "Existing schematic pins belong to '{} / {}'. Use that exact target cell so the symbol remains placeable.",
                    source.reference.library, source.reference.cell
                ));
            }
        }
        CreateSymbolSourceMode::BlankExplicitContract => {
            if draft.simulation_test_fixture {
                return Err(
                    "A blank explicit contract cannot generate a simulation test fixture because it has no executable DUT implementation."
                        .to_owned(),
                );
            }
        }
    }
    if let Some(error) = draft.validation_error.as_ref() {
        return Err(error.clone());
    }
    build_create_symbol_definition(state)?;
    Ok(())
}

pub(super) fn parse_target(target: &str) -> Result<(&str, &str), String> {
    let mut parts = target.split('/').map(str::trim);
    let library = parts.next().unwrap_or_default();
    let cell = parts.next().unwrap_or_default();
    if library.is_empty() || cell.is_empty() || parts.next().is_some() {
        return Err("Enter one destination as 'library / cell'.".to_owned());
    }
    validate_identifier(library, "Library name")?;
    validate_identifier(cell, "Cell name")?;
    Ok((library, cell))
}

pub(super) fn target_syntax_error(target: &str) -> Option<String> {
    parse_target(target).err()
}

fn validate_identifier(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err(format!("{label} is required."));
    }
    if value.chars().count() > 128 {
        return Err(format!("{label} is limited to 128 characters."));
    }
    let mut chars = value.chars();
    let valid_first = chars
        .next()
        .is_some_and(|character| character == '_' || character.is_ascii_alphabetic());
    if !valid_first
        || chars.any(|character| !(character == '_' || character.is_ascii_alphanumeric()))
    {
        return Err(format!(
            "{label} must start with a letter or underscore and contain only letters, digits, and underscores."
        ));
    }
    Ok(())
}

pub(super) fn inferred_model_pins(model_type: ModelType) -> Vec<CreateSymbolPinDraft> {
    use CreateSymbolPinSide::{Bottom, Left, Right, Top};
    use CreateSymbolPinType::{AnalogInput, Passive, Power};
    match model_type {
        ModelType::Nmos | ModelType::Pmos => vec![
            CreateSymbolPinDraft::new("D", Passive, Top),
            CreateSymbolPinDraft::new("G", AnalogInput, Left),
            CreateSymbolPinDraft::new("S", Passive, Bottom),
            CreateSymbolPinDraft::new("B", Power, Bottom),
        ],
        ModelType::Npn | ModelType::Pnp => vec![
            CreateSymbolPinDraft::new("C", Passive, Top),
            CreateSymbolPinDraft::new("B", AnalogInput, Left),
            CreateSymbolPinDraft::new("E", Passive, Bottom),
        ],
        ModelType::Diode | ModelType::Varactor => vec![
            CreateSymbolPinDraft::new("A", Passive, Left),
            CreateSymbolPinDraft::new("K", Passive, Right),
        ],
        ModelType::Resistor | ModelType::Capacitor | ModelType::Inductor => vec![
            CreateSymbolPinDraft::new("P", Passive, Left),
            CreateSymbolPinDraft::new("N", Passive, Right),
        ],
        ModelType::Rf => vec![
            CreateSymbolPinDraft::new("P", Passive, Left),
            CreateSymbolPinDraft::new("N", Passive, Right),
        ],
        ModelType::Esd | ModelType::Other => Vec::new(),
    }
}

fn model_family_label(model_type: ModelType) -> &'static str {
    match model_type {
        ModelType::Nmos | ModelType::Pmos => "MOS model",
        ModelType::Npn | ModelType::Pnp => "bipolar model",
        ModelType::Diode | ModelType::Varactor => "junction model",
        ModelType::Resistor | ModelType::Capacitor | ModelType::Inductor => "passive model",
        ModelType::Rf => "RF model",
        ModelType::Esd => "ESD model",
        ModelType::Other => "model contract",
    }
}

fn default_template(model_type: ModelType, model: &str) -> CreateSymbolTemplate {
    if model.to_ascii_lowercase().contains("opamp") || model.to_ascii_lowercase().contains("opa") {
        CreateSymbolTemplate::OperationalAmplifier5Pin
    } else if model_type == ModelType::Rf {
        CreateSymbolTemplate::RfNPort
    } else {
        CreateSymbolTemplate::RectangularIc
    }
}

fn pin_from_port(port: PortSpec) -> CreateSymbolPinDraft {
    let (electrical_type, side) = match port.direction {
        PortDirection::In => (CreateSymbolPinType::AnalogInput, CreateSymbolPinSide::Left),
        PortDirection::Out => (
            CreateSymbolPinType::AnalogOutput,
            CreateSymbolPinSide::Right,
        ),
        PortDirection::InOut => (
            CreateSymbolPinType::AnalogBidirectional,
            CreateSymbolPinSide::Right,
        ),
        PortDirection::Supply => (CreateSymbolPinType::Power, CreateSymbolPinSide::Top),
    };
    CreateSymbolPinDraft::new(port.name, electrical_type, side)
}

fn default_cell_name(model: &str) -> String {
    if model.eq_ignore_ascii_case("OPA189_A") {
        return "precision_opamp".to_owned();
    }
    let mut name = model
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    while name.contains("__") {
        name = name.replace("__", "_");
    }
    name = name.trim_matches('_').to_owned();
    if name.is_empty() || name.starts_with(|character: char| character.is_ascii_digit()) {
        name.insert_str(0, "model_");
    }
    name
}

pub(super) fn build_create_symbol_definition(
    state: &AppState,
) -> Result<ModelBoundSymbolDefinition, String> {
    let draft = &state.dialogs.create_model_bound_symbol;
    let (library_name, cell_name) = parse_target(&draft.target)?;
    let library = state
        .library_manager
        .get_library(library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))?;

    let mut current = None;
    if let Some(cell) = library.get_cell(cell_name) {
        let views = cell.views_sorted();
        for view in views {
            if let Some(definition) = ModelBoundSymbolDefinition::load_from_view(view)
                .map_err(|error| error.to_string())?
            {
                current = Some(definition);
                break;
            }
        }
    }
    let revision = current.as_ref().map_or(Ok(1), |definition| {
        definition.identity.revision.checked_add(1).ok_or_else(|| {
            "The existing symbol revision is exhausted and cannot be advanced.".to_owned()
        })
    })?;
    let binding_id = current.map_or_else(
        || {
            let material = format!(
                "rspice:model-bound-symbol:{}:{library_name}/{cell_name}",
                state.workspace.project.id()
            );
            uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, material.as_bytes()).to_string()
        },
        |definition| definition.identity.binding_id,
    );

    let pins = draft
        .pins
        .iter()
        .enumerate()
        .map(|(index, pin)| {
            SymbolPinDefinition::new(
                pin.name.trim(),
                pin.electrical_type.electrical_type(),
                pin.electrical_type.direction(),
                pin.side.domain(),
                index + 1,
            )
        })
        .collect::<Vec<_>>();

    let (source, parameter_form, netlist) = match draft.source_mode {
        CreateSymbolSourceMode::Model => {
            let source = draft
                .model_source
                .as_ref()
                .ok_or_else(|| "The selected model contract is no longer available.".to_owned())?;
            let source_path = source.source_path.as_ref().ok_or_else(|| {
                format!(
                    "Model '{}' has no retained implementation source.",
                    source.model
                )
            })?;
            let mut model = SymbolModelReference::new(&source.library, &source.model)
                .with_source_path(source_path.to_string_lossy())
                .with_implementation_view(SymbolImplementationView::Spice)
                .with_module_name(&source.model);
            model.section.clone_from(&source.section);

            let fields = source
                .instance_parameters
                .iter()
                .map(|parameter| {
                    validate_identifier(
                        &parameter.key,
                        &format!("Instance parameter '{}'", parameter.key),
                    )?;
                    let (property_type, default) = match parameter.property_type {
                        PropertyType::Number => (
                            PropertyType::Number,
                            SymbolParameterDefault::Number {
                                engineering: parameter.default.clone(),
                                unit: parameter.unit.clone(),
                            },
                        ),
                        PropertyType::String => (
                            PropertyType::String,
                            SymbolParameterDefault::String {
                                value: parameter.default.clone(),
                            },
                        ),
                        PropertyType::Expression => (
                            PropertyType::Expression,
                            SymbolParameterDefault::Expression {
                                value: parameter.default.clone(),
                            },
                        ),
                        PropertyType::Enum | PropertyType::Boolean => {
                            return Err(format!(
                                "Instance parameter '{}' has an unsupported generated type",
                                parameter.key
                            ));
                        }
                    };
                    Ok(SymbolParameterField {
                        key: parameter.key.clone(),
                        label: parameter.label.clone(),
                        help: parameter.help.clone(),
                        property_type,
                        default,
                        unit: parameter.unit.clone(),
                        constraints: SymbolParameterConstraints {
                            minimum: parameter.minimum.clone(),
                            maximum: parameter.maximum.clone(),
                            ..SymbolParameterConstraints::default()
                        },
                        inheritance: ParameterInheritance::InstanceOverride,
                        visibility: SymbolParameterVisibility::Advanced,
                        required: parameter.required,
                        aliases: Vec::new(),
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let parameter_order = fields
                .iter()
                .map(|field| field.key.clone())
                .collect::<Vec<_>>();
            let parameter_form = SymbolParameterForm {
                revision,
                sections: (!fields.is_empty())
                    .then(|| SymbolParameterSection {
                        key: "model_parameters".to_owned(),
                        label: "Model parameters".to_owned(),
                        help: format!("Typed instance overrides inherited from {}.", source.model),
                        fields,
                    })
                    .into_iter()
                    .collect(),
            };
            let ports = pins.iter().map(SymbolPinDefinition::port_spec).collect();
            let device_prefix = model_device_prefix(source.model_type);
            let netlist = SymbolNetlistBinding {
                device_prefix: device_prefix.to_owned(),
                model: Some(model.clone()),
                template: format!("{device_prefix}{{name}} {{nodes}} {{model}} {{params}}"),
                parameter_order,
            };
            (
                SymbolSourceContract::model(model, ports),
                parameter_form,
                netlist,
            )
        }
        CreateSymbolSourceMode::ExistingSchematicPins => {
            let source = draft.schematic_source.as_ref().ok_or_else(|| {
                "The retained schematic pin contract is no longer available.".to_owned()
            })?;
            let ports = source.pins.iter().map(CreateSymbolPinDraft::port).collect();
            (
                SymbolSourceContract::existing_schematic_pins(&source.reference.view, ports),
                SymbolParameterForm {
                    revision,
                    sections: Vec::new(),
                },
                SymbolNetlistBinding {
                    device_prefix: "X".to_owned(),
                    model: None,
                    template: "X{name} {nodes} {model} {params}".to_owned(),
                    parameter_order: Vec::new(),
                },
            )
        }
        CreateSymbolSourceMode::BlankExplicitContract => (
            SymbolSourceContract::blank(),
            SymbolParameterForm {
                revision,
                sections: Vec::new(),
            },
            SymbolNetlistBinding::unbound(),
        ),
    };

    let graphic_template = match draft.template {
        CreateSymbolTemplate::OperationalAmplifier5Pin => {
            SymbolGraphicTemplate::OperationalAmplifier5Pin
        }
        CreateSymbolTemplate::RectangularIc => SymbolGraphicTemplate::RectangularIc,
        CreateSymbolTemplate::RfNPort => SymbolGraphicTemplate::RfNPort,
    };
    let definition = ModelBoundSymbolDefinition::new(
        SymbolIdentity::new(library_name, cell_name, revision, binding_id),
        source,
        pins,
        graphic_template,
        parameter_form,
        netlist,
        GeneratedSymbolViews {
            symbol: draft.symbol,
            parameter_form: draft.parameter_form,
            simulation_test_fixture: draft.simulation_test_fixture,
        },
    );
    definition.validate().map_err(|error| error.to_string())?;
    definition
        .build_plan(library)
        .map_err(|error| error.to_string())?;
    Ok(definition)
}

#[cfg(test)]
fn subcircuit_instance_parameters(
    defaults: std::collections::BTreeMap<String, String>,
) -> Vec<CreateSymbolParameterDraft> {
    defaults
        .into_iter()
        .map(|(key, raw)| {
            let trimmed = raw.trim();
            let (property_type, default) =
                if crate::quantity::parse_engineering_value(trimmed).is_ok() {
                    (PropertyType::Number, trimmed.to_owned())
                } else if let Some(value) = trimmed
                    .strip_prefix('{')
                    .and_then(|value| value.strip_suffix('}'))
                {
                    (PropertyType::Expression, value.trim().to_owned())
                } else if let Some(value) = trimmed
                    .strip_prefix('"')
                    .and_then(|value| value.strip_suffix('"'))
                    .or_else(|| {
                        trimmed
                            .strip_prefix('\'')
                            .and_then(|value| value.strip_suffix('\''))
                    })
                {
                    let mut decoded = String::with_capacity(value.len());
                    let mut characters = value.chars();
                    while let Some(character) = characters.next() {
                        if character == '\\' {
                            decoded.push(characters.next().unwrap_or('\\'));
                        } else {
                            decoded.push(character);
                        }
                    }
                    (PropertyType::String, decoded)
                } else {
                    (PropertyType::Expression, trimmed.to_owned())
                };
            CreateSymbolParameterDraft {
                label: key.clone(),
                help: format!(
                    "Instance override declared by the imported subcircuit parameter '{key}'."
                ),
                key,
                property_type,
                default,
                unit: None,
                minimum: None,
                maximum: None,
                required: false,
            }
        })
        .collect()
}

fn model_instance_parameters(
    model: &crate::state::model_library::DeviceModel,
) -> Vec<CreateSymbolParameterDraft> {
    // Keeping the complete numeric contract in one constructor prevents a
    // caller from accidentally dropping bounds or requiredness while adding
    // a model-family field.
    fn numeric(
        key: &str,
        label: &str,
        help: &str,
        default: impl Into<String>,
        unit: Option<&str>,
        minimum: Option<String>,
        maximum: Option<String>,
        required: bool,
    ) -> CreateSymbolParameterDraft {
        CreateSymbolParameterDraft {
            key: key.to_owned(),
            label: label.to_owned(),
            help: help.to_owned(),
            property_type: PropertyType::Number,
            default: default.into(),
            unit: unit.map(str::to_owned),
            minimum,
            maximum,
            required,
        }
    }

    fn finite(value: Option<f64>) -> Option<String> {
        value
            .filter(|value| value.is_finite())
            .map(|value| format!("{value:.17}"))
    }

    fn geometry(
        key: &str,
        label: &str,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> CreateSymbolParameterDraft {
        let minimum = finite(minimum);
        let maximum = finite(maximum);
        let default = minimum
            .clone()
            .or_else(|| maximum.clone())
            .unwrap_or_else(|| "1u".to_owned());
        numeric(
            key,
            label,
            &format!("Instance {label} constrained by the selected model geometry."),
            default,
            Some("m"),
            minimum.clone(),
            maximum.clone(),
            minimum.is_none() && maximum.is_none(),
        )
    }

    let multiplicity = || {
        numeric(
            "m",
            "Multiplicity",
            "Parallel instance multiplier.",
            "1",
            None,
            Some("1".to_owned()),
            None,
            false,
        )
    };
    match model.model_type {
        ModelType::Nmos | ModelType::Pmos => vec![
            geometry("w", "channel width", model.w_min, model.w_max),
            geometry("l", "channel length", model.l_min, model.l_max),
            multiplicity(),
            numeric(
                "nf",
                "Finger count",
                "Number of device fingers.",
                "1",
                None,
                Some("1".to_owned()),
                None,
                false,
            ),
            numeric(
                "ad",
                "Drain area",
                "Drain diffusion area.",
                "0",
                Some("m^2"),
                Some("0".to_owned()),
                None,
                false,
            ),
            numeric(
                "as",
                "Source area",
                "Source diffusion area.",
                "0",
                Some("m^2"),
                Some("0".to_owned()),
                None,
                false,
            ),
            numeric(
                "pd",
                "Drain perimeter",
                "Drain diffusion perimeter.",
                "0",
                Some("m"),
                Some("0".to_owned()),
                None,
                false,
            ),
            numeric(
                "ps",
                "Source perimeter",
                "Source diffusion perimeter.",
                "0",
                Some("m"),
                Some("0".to_owned()),
                None,
                false,
            ),
            numeric(
                "nrd",
                "Drain squares",
                "Drain diffusion resistance in squares.",
                "0",
                None,
                Some("0".to_owned()),
                None,
                false,
            ),
            numeric(
                "nrs",
                "Source squares",
                "Source diffusion resistance in squares.",
                "0",
                None,
                Some("0".to_owned()),
                None,
                false,
            ),
        ],
        ModelType::Npn | ModelType::Pnp => vec![
            numeric(
                "area",
                "Emitter area",
                "Emitter area multiplier.",
                "1",
                None,
                Some("0".to_owned()),
                None,
                false,
            ),
            multiplicity(),
        ],
        ModelType::Diode | ModelType::Varactor => vec![
            numeric(
                "area",
                "Junction area",
                "Junction area multiplier.",
                "1",
                None,
                Some("0".to_owned()),
                None,
                false,
            ),
            multiplicity(),
        ],
        ModelType::Resistor => vec![
            numeric(
                "r",
                "Resistance",
                "Instance resistance override.",
                "1k",
                Some("Ohm"),
                Some("0".to_owned()),
                None,
                false,
            ),
            multiplicity(),
        ],
        ModelType::Capacitor => vec![
            numeric(
                "c",
                "Capacitance",
                "Instance capacitance override.",
                "1p",
                Some("F"),
                Some("0".to_owned()),
                None,
                false,
            ),
            multiplicity(),
        ],
        ModelType::Inductor => vec![
            numeric(
                "l",
                "Inductance",
                "Instance inductance override.",
                "1n",
                Some("H"),
                Some("0".to_owned()),
                None,
                false,
            ),
            multiplicity(),
        ],
        ModelType::Rf | ModelType::Esd | ModelType::Other => Vec::new(),
    }
}

const fn model_device_prefix(model_type: ModelType) -> &'static str {
    match model_type {
        ModelType::Nmos | ModelType::Pmos => "M",
        ModelType::Npn | ModelType::Pnp => "Q",
        ModelType::Diode | ModelType::Varactor => "D",
        ModelType::Resistor => "R",
        ModelType::Capacitor => "C",
        ModelType::Inductor => "L",
        ModelType::Rf | ModelType::Esd | ModelType::Other => "X",
    }
}

// The domain-owned construction plan is the final authority after every
// retained UI authority check. Conversion happens once at the transaction
// boundary, so controls edit typed draft input and never mutate a live cell.
pub(super) fn commit_create_model_bound_symbol(state: &mut AppState) -> Result<(), String> {
    validate_create_symbol_draft(state)?;
    let definition = build_create_symbol_definition(state)?;
    let library_name = definition.identity.library.clone();
    let cell_name = definition.identity.cell.clone();
    let revision = definition.identity.revision;
    let digest = definition
        .validation_digest()
        .map_err(|error| error.to_string())?;
    let mut candidate = state.library_manager.clone();
    let library = candidate
        .get_library_mut(&library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))?;
    definition
        .build_plan(library)
        .map_err(|error| error.to_string())?
        .commit(library)
        .map_err(|error| error.to_string())?;

    let fixture = if definition.generated_views.simulation_test_fixture {
        let reference = CellViewRef::new(&library_name, &cell_name, "testbench");
        let before = if state.workspace.active_view == reference {
            Some(state.schematic.clone())
        } else {
            state
                .workspace
                .schematic_buffers
                .get(&reference.key())
                .cloned()
        };
        let after = definition
            .build_test_fixture_schematic()
            .map_err(|error| error.to_string())?;
        Some(SymbolDefinitionFixtureDelta {
            reference,
            before,
            after: Some(after),
        })
    } else {
        None
    };

    publish_symbol_definition_candidate_with_fixture(
        state,
        candidate,
        &library_name,
        &cell_name,
        format!("Create symbol revision {revision} for {library_name}/{cell_name}"),
        fixture,
    )?;
    state.dialogs.create_model_bound_symbol.close();
    let view_name = if definition.generated_views.symbol {
        "symbol"
    } else if definition.generated_views.parameter_form {
        "parameter_form"
    } else if definition.generated_views.simulation_test_fixture {
        "testbench"
    } else {
        unreachable!("the validated definition owns at least one generated view")
    };
    state.open_workspace_view(CellViewRef::new(&library_name, &cell_name, view_name));
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    state.push_user_message(ConsoleMessage::info(format!(
        "Created {library_name}/{cell_name}/symbol revision {revision} ({digest})"
    )));
    Ok(())
}
