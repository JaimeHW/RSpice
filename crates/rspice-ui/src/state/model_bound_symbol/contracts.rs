//! Symbol identity.
//!
//! What makes two symbol definitions the same definition. Identity is by
//! declared name and parameter contract, not by drawing, so restyling a body
//! does not orphan the instances that use it.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolIdentity {
    pub library: String,
    pub cell: String,
    pub revision: u64,
    pub binding_id: String,
}

impl SymbolIdentity {
    pub fn new(
        library: impl Into<String>,
        cell: impl Into<String>,
        revision: u64,
        binding_id: impl Into<String>,
    ) -> Self {
        Self {
            library: library.into(),
            cell: cell.into(),
            revision,
            binding_id: binding_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolModelReference {
    pub library: String,
    pub model: String,
    pub section: Option<String>,
    pub revision: Option<String>,
    /// Authoritative implementation source. Model-bound definitions require
    /// an absolute path; geometry imports never fabricate one.
    pub source_path: Option<String>,
    pub implementation_view: SymbolImplementationView,
    pub module_name: Option<String>,
}

impl SymbolModelReference {
    pub fn new(library: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            library: library.into(),
            model: model.into(),
            section: None,
            revision: None,
            source_path: None,
            implementation_view: SymbolImplementationView::Spice,
            module_name: None,
        }
    }

    pub fn with_source_path(mut self, source_path: impl Into<String>) -> Self {
        self.source_path = Some(source_path.into());
        self
    }

    pub fn with_implementation_view(mut self, view: SymbolImplementationView) -> Self {
        self.implementation_view = view;
        self
    }

    pub fn with_module_name(mut self, module_name: impl Into<String>) -> Self {
        self.module_name = Some(module_name.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SymbolImplementationView {
    #[default]
    Spice,
    VerilogA,
}

impl SymbolImplementationView {
    pub const fn view_name(self) -> &'static str {
        match self {
            Self::Spice => "spice",
            Self::VerilogA => "veriloga",
        }
    }

    pub const fn view_type(self) -> ViewType {
        match self {
            Self::Spice => ViewType::Spice,
            Self::VerilogA => ViewType::VerilogA,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SymbolSourceContract {
    Model {
        model: SymbolModelReference,
        ports: Vec<PortSpec>,
    },
    ExistingSchematicPins {
        schematic_view: String,
        ports: Vec<PortSpec>,
    },
    BlankExplicitContract,
}

impl SymbolSourceContract {
    pub fn model(model: SymbolModelReference, ports: Vec<PortSpec>) -> Self {
        Self::Model { model, ports }
    }

    pub fn existing_schematic_pins(
        schematic_view: impl Into<String>,
        ports: Vec<PortSpec>,
    ) -> Self {
        Self::ExistingSchematicPins {
            schematic_view: schematic_view.into(),
            ports,
        }
    }

    pub const fn blank() -> Self {
        Self::BlankExplicitContract
    }


    pub const fn is_explicitly_unbound_for_review(&self) -> bool {
        matches!(self, Self::BlankExplicitContract)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolElectricalType {
    Analog,
    Logic,
    Power,
    Ground,
    Passive,
}

impl SymbolElectricalType {
    pub const ALL: [Self; 5] = [
        Self::Analog,
        Self::Logic,
        Self::Power,
        Self::Ground,
        Self::Passive,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Analog => "analog",
            Self::Logic => "logic",
            Self::Power => "power",
            Self::Ground => "ground",
            Self::Passive => "passive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolPinSide {
    Left,
    Right,
    Top,
    Bottom,
}

impl SymbolPinSide {
    pub const ALL: [Self; 4] = [Self::Left, Self::Right, Self::Top, Self::Bottom];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolPinDefinition {
    pub name: String,
    pub electrical_type: SymbolElectricalType,
    pub direction: PortDirection,
    pub side: SymbolPinSide,
    /// One-based positional netlist order.
    pub order: usize,
}

impl SymbolPinDefinition {
    pub fn new(
        name: impl Into<String>,
        electrical_type: SymbolElectricalType,
        direction: PortDirection,
        side: SymbolPinSide,
        order: usize,
    ) -> Self {
        Self {
            name: name.into(),
            electrical_type,
            direction,
            side,
            order,
        }
    }

    pub fn port_spec(&self) -> PortSpec {
        PortSpec {
            name: self.name.clone(),
            direction: self.direction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolGraphicTemplate {
    OperationalAmplifier5Pin,
    RectangularIc,
    RfNPort,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterInheritance {
    InstanceOverride,
    CellDefault,
    ModelDefault,
}

impl ParameterInheritance {
    pub const ALL: [Self; 3] = [
        Self::InstanceOverride,
        Self::CellDefault,
        Self::ModelDefault,
    ];

    /// Instance overrides are the only values an instance editor may mutate.
    pub const fn instance_editable(self) -> bool {
        matches!(self, Self::InstanceOverride)
    }

    /// Model defaults are omitted unless the model itself supplies them.
    /// Instance overrides and immutable cell defaults are emitted by RSpice.
    pub const fn emitted_by_rspice(self) -> bool {
        !matches!(self, Self::ModelDefault)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolParameterVisibility {
    Visible,
    Advanced,
    Hidden,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SymbolParameterDefault {
    Number {
        engineering: String,
        unit: Option<String>,
    },
    String {
        value: String,
    },
    Expression {
        value: String,
    },
    Enum {
        selected: String,
    },
    Boolean {
        value: bool,
    },
}

impl SymbolParameterDefault {
    pub fn display_string(&self) -> String {
        match self {
            Self::Number { engineering, unit } => {
                format!("{}{}", engineering, unit.as_deref().unwrap_or_default())
            }
            Self::String { value } | Self::Expression { value } => value.clone(),
            Self::Enum { selected } => selected.clone(),
            Self::Boolean { value } => if *value { "yes" } else { "no" }.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct SymbolParameterConstraints {
    /// Inclusive lower engineering value for numeric parameters.
    pub minimum: Option<String>,
    /// Inclusive upper engineering value for numeric parameters.
    pub maximum: Option<String>,
    pub enum_values: Vec<String>,
    pub max_length: Option<usize>,
}
