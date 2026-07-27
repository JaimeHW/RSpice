use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolDefinitionError {
    UnsupportedSchema(u32),
    EmptyIdentity(&'static str),
    InvalidIdentity(&'static str),
    DuplicatePin(String),
    DuplicatePinOrder(usize),
    NonContiguousPinOrder { expected: usize, observed: usize },
    InvalidPin(String),
    SourcePinMismatch(String),
    InvalidNetlist(String),
    DuplicateParameterSection(String),
    DuplicateParameter(String),
    UnknownParameterSection(String),
    UnknownParameter(String),
    InvalidForm(String),
    NoGeneratedViews,
    Serialization(String),
    Import(String),
    ReadOnlyLibrary(String),
    LibraryIdentityMismatch { expected: String, actual: String },
    NonMonotonicRevision { current: u64, proposed: u64 },
    StaleTarget(String),
}

impl fmt::Display for SymbolDefinitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchema(version) => {
                write!(formatter, "unsupported symbol schema {version}")
            }
            Self::EmptyIdentity(field) => write!(formatter, "symbol {field} is required"),
            Self::InvalidIdentity(field) => {
                write!(formatter, "symbol {field} contains unsupported characters")
            }
            Self::DuplicatePin(name) => write!(formatter, "duplicate symbol pin `{name}`"),
            Self::DuplicatePinOrder(order) => {
                write!(formatter, "duplicate symbol pin order {order}")
            }
            Self::NonContiguousPinOrder { expected, observed } => write!(
                formatter,
                "pin order must be contiguous: expected {expected}, observed {observed}"
            ),
            Self::InvalidPin(message) => write!(formatter, "invalid symbol pin: {message}"),
            Self::SourcePinMismatch(message) => {
                write!(formatter, "source pin contract mismatch: {message}")
            }
            Self::InvalidNetlist(message) => {
                write!(formatter, "invalid netlist binding: {message}")
            }
            Self::DuplicateParameterSection(key) => {
                write!(formatter, "duplicate parameter section `{key}`")
            }
            Self::DuplicateParameter(key) => {
                write!(formatter, "duplicate parameter or alias `{key}`")
            }
            Self::UnknownParameterSection(key) => {
                write!(formatter, "unknown parameter section `{key}`")
            }
            Self::UnknownParameter(key) => write!(formatter, "unknown parameter `{key}`"),
            Self::InvalidForm(message) => write!(formatter, "invalid parameter form: {message}"),
            Self::NoGeneratedViews => formatter.write_str("select at least one generated view"),
            Self::Serialization(message) => {
                write!(formatter, "symbol serialization failed: {message}")
            }
            Self::Import(message) => formatter.write_str(message),
            Self::ReadOnlyLibrary(library) => write!(formatter, "library `{library}` is read-only"),
            Self::LibraryIdentityMismatch { expected, actual } => write!(
                formatter,
                "definition targets library `{expected}`, not `{actual}`"
            ),
            Self::NonMonotonicRevision { current, proposed } => write!(
                formatter,
                "symbol revision {proposed} must be greater than current revision {current}"
            ),
            Self::StaleTarget(cell) => write!(
                formatter,
                "cell `{cell}` changed after the symbol plan was validated"
            ),
        }
    }
}

impl std::error::Error for SymbolDefinitionError {}

pub(super) fn validate_identity(identity: &SymbolIdentity) -> Result<(), SymbolDefinitionError> {
    for (field, value) in [
        ("library", identity.library.as_str()),
        ("cell", identity.cell.as_str()),
        ("binding ID", identity.binding_id.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(SymbolDefinitionError::EmptyIdentity(field));
        }
        if value.len() > 256 || value.chars().any(char::is_control) {
            return Err(SymbolDefinitionError::InvalidIdentity(field));
        }
    }
    if identity.revision == 0 {
        return Err(SymbolDefinitionError::InvalidIdentity("revision"));
    }

    Ok(())
}

pub(super) fn validate_pins(pins: &[SymbolPinDefinition]) -> Result<(), SymbolDefinitionError> {
    if pins.is_empty() {
        return Err(SymbolDefinitionError::InvalidPin(
            "at least one explicit pin is required".to_owned(),
        ));
    }
    let mut names = HashSet::new();
    let mut orders = HashSet::new();
    for pin in pins {
        if validate_key(&pin.name).is_err() {
            return Err(SymbolDefinitionError::InvalidPin(format!(
                "`{}` is not a portable terminal name",
                pin.name
            )));
        }
        if !names.insert(pin.name.to_ascii_lowercase()) {
            return Err(SymbolDefinitionError::DuplicatePin(pin.name.clone()));
        }
        if pin.order == 0 {
            return Err(SymbolDefinitionError::InvalidPin(
                "pin order is one-based".to_owned(),
            ));
        }
        if !orders.insert(pin.order) {
            return Err(SymbolDefinitionError::DuplicatePinOrder(pin.order));
        }
    }
    let mut ordered = pins.iter().map(|pin| pin.order).collect::<Vec<_>>();
    ordered.sort_unstable();
    for (index, observed) in ordered.into_iter().enumerate() {
        let expected = index + 1;
        if observed != expected {
            return Err(SymbolDefinitionError::NonContiguousPinOrder { expected, observed });
        }
    }
    Ok(())
}

pub(super) fn validate_source(
    source: &SymbolSourceContract,
    pins: &[SymbolPinDefinition],
) -> Result<(), SymbolDefinitionError> {
    match source {
        SymbolSourceContract::Model { model, ports } => {
            if model.library.trim().is_empty() || model.model.trim().is_empty() {
                return Err(SymbolDefinitionError::SourcePinMismatch(
                    "model library and name are required".to_owned(),
                ));
            }
            let source_path = model.source_path.as_deref().ok_or_else(|| {
                SymbolDefinitionError::SourcePinMismatch(
                    "model source path is required for an executable implementation".to_owned(),
                )
            })?;
            if source_path.trim().is_empty() || !Path::new(source_path).is_absolute() {
                return Err(SymbolDefinitionError::SourcePinMismatch(
                    "model source path must be absolute".to_owned(),
                ));
            }
            if model.implementation_view == SymbolImplementationView::VerilogA
                && model
                    .module_name
                    .as_deref()
                    .is_none_or(|name| name.trim().is_empty())
            {
                return Err(SymbolDefinitionError::SourcePinMismatch(
                    "Verilog-A implementations require a module name".to_owned(),
                ));
            }
            compare_source_ports(ports, pins)
        }
        SymbolSourceContract::ExistingSchematicPins {
            schematic_view,
            ports,
        } => {
            if schematic_view.trim().is_empty() {
                return Err(SymbolDefinitionError::SourcePinMismatch(
                    "schematic view name is required".to_owned(),
                ));
            }
            compare_source_ports(ports, pins)
        }
        SymbolSourceContract::BlankExplicitContract => Ok(()),
    }
}

fn compare_source_ports(
    ports: &[PortSpec],
    pins: &[SymbolPinDefinition],
) -> Result<(), SymbolDefinitionError> {
    if ports.len() != pins.len() {
        return Err(SymbolDefinitionError::SourcePinMismatch(format!(
            "source has {} terminals but symbol declares {}",
            ports.len(),
            pins.len()
        )));
    }

    let mut ordered = pins.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|pin| pin.order);
    for (index, (port, pin)) in ports.iter().zip(ordered).enumerate() {
        if !port.name.eq_ignore_ascii_case(&pin.name) || port.direction != pin.direction {
            return Err(SymbolDefinitionError::SourcePinMismatch(format!(
                "terminal {} is `{} {}` in the source and `{} {}` in the symbol",
                index + 1,
                port.name,
                port.direction.keyword(),
                pin.name,
                pin.direction.keyword()
            )));
        }
    }
    Ok(())
}

pub(super) fn validate_netlist(
    netlist: &SymbolNetlistBinding,
    source: &SymbolSourceContract,
    pins: &[SymbolPinDefinition],
    form: &SymbolParameterForm,
) -> Result<(), SymbolDefinitionError> {
    if source.is_explicitly_unbound_for_review() {
        if netlist.is_executable() || netlist.model.is_some() || !netlist.parameter_order.is_empty()
        {
            return Err(SymbolDefinitionError::InvalidNetlist(
                "an explicitly unbound review symbol must use a non-executable empty binding"
                    .to_owned(),
            ));
        }
        return Ok(());
    }
    if let SymbolSourceContract::Model { model, .. } = source
        && netlist.model.as_ref() != Some(model)
    {
        return Err(SymbolDefinitionError::InvalidNetlist(
            "model source and executable netlist model must match exactly".to_owned(),
        ));
    }
    validate_executable_template(netlist, pins, form)?;
    Ok(())
}

pub(super) fn validate_executable_template(
    netlist: &SymbolNetlistBinding,
    _pins: &[SymbolPinDefinition],
    form: &SymbolParameterForm,
) -> Result<(), SymbolDefinitionError> {
    if netlist.device_prefix.trim().is_empty()
        || !netlist
            .device_prefix
            .chars()
            .all(|character| character.is_ascii_alphabetic())
    {
        return Err(SymbolDefinitionError::InvalidNetlist(
            "device prefix must contain ASCII letters".to_owned(),
        ));
    }
    crate::state::validate_library_netlist_template(&netlist.template)
        .map_err(SymbolDefinitionError::InvalidNetlist)?;
    if let Some(prefix) = netlist
        .template
        .split_ascii_whitespace()
        .next()
        .and_then(|token| token.strip_suffix("{name}"))
        && !prefix.eq_ignore_ascii_case(&netlist.device_prefix)
    {
        return Err(SymbolDefinitionError::InvalidNetlist(format!(
            "template prefix `{prefix}` does not match reference prefix `{}`",
            netlist.device_prefix
        )));
    }
    let mut seen = HashSet::new();
    for parameter in &netlist.parameter_order {
        if !seen.insert(parameter.to_ascii_lowercase()) {
            return Err(SymbolDefinitionError::InvalidNetlist(format!(
                "duplicate parameter `{parameter}`"
            )));
        }
        if form.field(parameter).is_none() {
            return Err(SymbolDefinitionError::InvalidNetlist(format!(
                "parameter `{parameter}` is not in the typed form"
            )));
        }
    }
    let expected = form.netlist_parameter_order();
    if netlist.parameter_order != expected {
        return Err(SymbolDefinitionError::InvalidNetlist(format!(
            "parameter order must match the form's emitted fields exactly: expected {}",
            expected.join(", ")
        )));
    }
    Ok(())
}

pub(super) fn validate_parameter_field(
    section: &SymbolParameterSection,
    field: &SymbolParameterField,
    spellings: &mut HashMap<String, String>,
    diagnostics: &mut Vec<SymbolFormDiagnostic>,
) {
    if validate_key(&field.key).is_err() {
        form_diag(
            diagnostics,
            section,
            Some(field),
            "invalid stable field key",
        );
    }
    if field.label.trim().is_empty() || field.help.trim().is_empty() {
        form_diag(
            diagnostics,
            section,
            Some(field),
            "field label and help are required",
        );
    }
    for spelling in std::iter::once(&field.key).chain(&field.aliases) {
        if validate_key(spelling).is_err() {
            form_diag(diagnostics, section, Some(field), "invalid parameter alias");
        }
        if let Some(owner) = spellings.insert(spelling.to_ascii_lowercase(), field.key.clone()) {
            form_diag(
                diagnostics,
                section,
                Some(field),
                &format!("parameter spelling is already owned by `{owner}`"),
            );
        }
    }
    let kind_matches = matches!(
        (&field.property_type, &field.default),
        (PropertyType::Number, SymbolParameterDefault::Number { .. })
            | (PropertyType::String, SymbolParameterDefault::String { .. })
            | (
                PropertyType::Expression,
                SymbolParameterDefault::Expression { .. }
            )
            | (PropertyType::Enum, SymbolParameterDefault::Enum { .. })
            | (
                PropertyType::Boolean,
                SymbolParameterDefault::Boolean { .. }
            )
    );
    if !kind_matches {
        form_diag(
            diagnostics,
            section,
            Some(field),
            "typed default does not match the field type",
        );
    }
    if field.property_type == PropertyType::Number {
        let default = match &field.default {
            SymbolParameterDefault::Number { engineering, .. } => parse_engineering(engineering),
            _ => return,
        };
        let minimum = field.constraints.minimum.as_deref().map(parse_engineering);
        let maximum = field.constraints.maximum.as_deref().map(parse_engineering);
        if default.is_err()
            || minimum.as_ref().is_some_and(|value| value.is_err())
            || maximum.as_ref().is_some_and(|value| value.is_err())
        {
            form_diag(
                diagnostics,
                section,
                Some(field),
                "engineering default or bound is invalid",
            );
        } else if let Ok(default) = default {
            let minimum = minimum.and_then(Result::ok);
            let maximum = maximum.and_then(Result::ok);
            if minimum.zip(maximum).is_some_and(|(min, max)| min > max)
                || minimum.is_some_and(|minimum| default < minimum)
                || maximum.is_some_and(|maximum| default > maximum)
            {
                form_diag(
                    diagnostics,
                    section,
                    Some(field),
                    "numeric default and inclusive bounds are inconsistent",
                );
            }
        }
    } else if field.constraints.minimum.is_some() || field.constraints.maximum.is_some() {
        form_diag(
            diagnostics,
            section,
            Some(field),
            "numeric bounds are only valid for numeric fields",
        );
    }
    if field.property_type == PropertyType::Enum {
        let selected = match &field.default {
            SymbolParameterDefault::Enum { selected } => selected,
            _ => return,
        };
        let mut values = HashSet::new();
        if field.constraints.enum_values.is_empty()
            || field
                .constraints
                .enum_values
                .iter()
                .any(|value| value.trim().is_empty() || !values.insert(value.to_ascii_lowercase()))
            || !field.constraints.enum_values.contains(selected)
        {
            form_diag(
                diagnostics,
                section,
                Some(field),
                "enum choices must be unique and contain the default",
            );
        }
    } else if !field.constraints.enum_values.is_empty() {
        form_diag(
            diagnostics,
            section,
            Some(field),
            "enum choices are only valid for enum fields",
        );
    }
    if field.unit.as_ref().is_some_and(|unit| {
        unit.trim().is_empty() || unit.len() > 32 || unit.chars().any(char::is_control)
    }) {
        form_diag(diagnostics, section, Some(field), "unit is invalid");
    }
    if field.unit.is_some() && field.property_type != PropertyType::Number {
        form_diag(
            diagnostics,
            section,
            Some(field),
            "units are only valid for numeric fields",
        );
    }
    if let Some(max_length) = field.constraints.max_length {
        if !matches!(
            field.property_type,
            PropertyType::String | PropertyType::Expression
        ) {
            form_diag(
                diagnostics,
                section,
                Some(field),
                "maximum length is only valid for string or expression fields",
            );
        } else if max_length == 0 || max_length > 1_048_576 {
            form_diag(
                diagnostics,
                section,
                Some(field),
                "maximum length must be between 1 and 1048576 characters",
            );
        } else {
            let default_length = match &field.default {
                SymbolParameterDefault::String { value }
                | SymbolParameterDefault::Expression { value } => value.chars().count(),
                _ => 0,
            };
            if default_length > max_length {
                form_diag(
                    diagnostics,
                    section,
                    Some(field),
                    "default exceeds the configured maximum length",
                );
            }
        }
    }
}

pub(super) fn form_diag(
    diagnostics: &mut Vec<SymbolFormDiagnostic>,
    section: &SymbolParameterSection,
    field: Option<&SymbolParameterField>,
    message: &str,
) {
    diagnostics.push(SymbolFormDiagnostic {
        section_key: section.key.clone(),
        field_key: field.map(|field| field.key.clone()),
        message: message.to_owned(),
    });
}

pub(super) fn validate_key(value: &str) -> Result<(), ()> {
    let mut chars = value.chars();
    if !chars
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
        || chars.any(|character| !character.is_ascii_alphanumeric() && character != '_')
    {
        return Err(());
    }
    Ok(())
}

pub(super) fn parse_engineering(value: &str) -> Result<f64, SymbolDefinitionError> {
    crate::quantity::parse_engineering_value(value).map_err(|error| {
        SymbolDefinitionError::InvalidForm(format!("invalid engineering value `{value}`: {error}"))
    })
}
