//! Create-symbol dialog state: the source the symbol is built from and the
//! body template it starts with.

use std::path::PathBuf;

use crate::state::model_library::ModelType;
use crate::state::{
    CellViewRef, PortDirection, PortSpec, PropertyType, SymbolElectricalType, SymbolPinSide,
};

pub(super) const EYEBROW: &str = "SYMBOL LIBRARY \u{00b7} PIN CONTRACT \u{00b7} GRAPHIC VIEW";
pub(super) const TITLE: &str = "Create model-bound symbol";
pub(super) const PRIMARY: &str = "Create symbol revision";
pub(super) const DESCRIPTION: &str = "Create one governed symbol definition from an explicit source contract, ordered electrical terminals, and versioned generated views.";
pub(super) const INITIAL_HEIGHT: f32 = 526.0;
pub(super) const SPLIT_BREAKPOINT: f32 = 640.0;
pub(super) const SECTION_HEADER_HEIGHT: f32 = 31.0;
pub(super) const PIN_HEADER_HEIGHT: f32 = 27.0;
pub(super) const PIN_ROW_HEIGHT: f32 = 30.0;
pub(super) const FIELD_GAP: f32 = 10.0;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum CreateSymbolSourceMode {
    #[default]
    Model,
    ExistingSchematicPins,
    BlankExplicitContract,
}

impl CreateSymbolSourceMode {
    pub(super) const ALL: [Self; 3] = [
        Self::Model,
        Self::ExistingSchematicPins,
        Self::BlankExplicitContract,
    ];
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum CreateSymbolTemplate {
    #[default]
    OperationalAmplifier5Pin,
    RectangularIc,
    RfNPort,
}

impl CreateSymbolTemplate {
    pub(super) const ALL: [Self; 3] = [
        Self::OperationalAmplifier5Pin,
        Self::RectangularIc,
        Self::RfNPort,
    ];

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::OperationalAmplifier5Pin => "Operational amplifier \u{00b7} 5 pin",
            Self::RectangularIc => "Rectangular IC",
            Self::RfNPort => "RF N-port",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum CreateSymbolPinType {
    #[default]
    AnalogInput,
    AnalogOutput,
    AnalogBidirectional,
    LogicInput,
    LogicOutput,
    LogicBidirectional,
    Power,
    Ground,
    Passive,
}

impl CreateSymbolPinType {
    pub(super) const ALL: [Self; 9] = [
        Self::AnalogInput,
        Self::AnalogOutput,
        Self::AnalogBidirectional,
        Self::LogicInput,
        Self::LogicOutput,
        Self::LogicBidirectional,
        Self::Power,
        Self::Ground,
        Self::Passive,
    ];

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::AnalogInput => "analog input",
            Self::AnalogOutput => "analog output",
            Self::AnalogBidirectional => "analog bidirectional",
            Self::LogicInput => "logic input",
            Self::LogicOutput => "logic output",
            Self::LogicBidirectional => "logic bidirectional",
            Self::Power => "power",
            Self::Ground => "ground",
            Self::Passive => "passive",
        }
    }

    pub(super) const fn direction(self) -> PortDirection {
        match self {
            Self::AnalogInput | Self::LogicInput => PortDirection::In,
            Self::AnalogOutput | Self::LogicOutput => PortDirection::Out,
            Self::Power | Self::Ground => PortDirection::Supply,
            Self::AnalogBidirectional | Self::LogicBidirectional | Self::Passive => {
                PortDirection::InOut
            }
        }
    }

    pub(super) const fn electrical_type(self) -> SymbolElectricalType {
        match self {
            Self::AnalogInput | Self::AnalogOutput | Self::AnalogBidirectional => {
                SymbolElectricalType::Analog
            }
            Self::LogicInput | Self::LogicOutput | Self::LogicBidirectional => {
                SymbolElectricalType::Logic
            }
            Self::Power => SymbolElectricalType::Power,
            Self::Ground => SymbolElectricalType::Ground,
            Self::Passive => SymbolElectricalType::Passive,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum CreateSymbolPinSide {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

impl CreateSymbolPinSide {
    pub(super) const ALL: [Self; 4] = [Self::Left, Self::Right, Self::Top, Self::Bottom];

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }

    pub(super) const fn domain(self) -> SymbolPinSide {
        match self {
            Self::Left => SymbolPinSide::Left,
            Self::Right => SymbolPinSide::Right,
            Self::Top => SymbolPinSide::Top,
            Self::Bottom => SymbolPinSide::Bottom,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CreateSymbolPinDraft {
    pub(crate) name: String,
    pub(crate) electrical_type: CreateSymbolPinType,
    pub(crate) side: CreateSymbolPinSide,
}

impl CreateSymbolPinDraft {
    pub(super) fn new(
        name: impl Into<String>,
        electrical_type: CreateSymbolPinType,
        side: CreateSymbolPinSide,
    ) -> Self {
        Self {
            name: name.into(),
            electrical_type,
            side,
        }
    }

    pub(super) fn port(&self) -> PortSpec {
        PortSpec {
            name: self.name.trim().to_owned(),
            direction: self.electrical_type.direction(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CreateSymbolModelSource {
    pub(crate) library: String,
    pub(crate) model: String,
    pub(crate) model_type: ModelType,
    pub(crate) family: String,
    pub(crate) source_path: Option<PathBuf>,
    pub(crate) section: Option<String>,
    pub(crate) instance_parameters: Vec<CreateSymbolParameterDraft>,
    pub(crate) pins: Vec<CreateSymbolPinDraft>,
    pub(crate) requires_pin_review: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CreateSymbolParameterDraft {
    pub(super) key: String,
    pub(super) label: String,
    pub(super) help: String,
    pub(super) property_type: PropertyType,
    pub(super) default: String,
    pub(super) unit: Option<String>,
    pub(super) minimum: Option<String>,
    pub(super) maximum: Option<String>,
    pub(super) required: bool,
}

impl CreateSymbolModelSource {
    pub(super) fn label(&self) -> String {
        if self.pins.is_empty() {
            format!("{} \u{00b7} explicit terminals required", self.model)
        } else {
            format!(
                "{} \u{00b7} {} terminal {}",
                self.model,
                self.pins.len(),
                self.family
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CreateSymbolSchematicSource {
    pub(crate) reference: CellViewRef,
    pub(crate) pins: Vec<CreateSymbolPinDraft>,
}

impl CreateSymbolSchematicSource {
    pub(super) fn label(&self) -> String {
        format!(
            "Existing schematic pins \u{00b7} {}",
            self.reference.display_path()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CreateModelBoundSymbolDialogState {
    pub(crate) open: bool,
    pub(crate) target: String,
    pub(crate) source_mode: CreateSymbolSourceMode,
    pub(crate) model_source: Option<CreateSymbolModelSource>,
    pub(crate) schematic_source: Option<CreateSymbolSchematicSource>,
    pub(crate) template: CreateSymbolTemplate,
    pub(crate) pins: Vec<CreateSymbolPinDraft>,
    pub(crate) symbol: bool,
    pub(crate) parameter_form: bool,
    pub(crate) simulation_test_fixture: bool,
    pub(crate) pin_contract_reviewed: bool,
    pub(crate) selected_pin: Option<usize>,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
    pub(crate) validation_error: Option<String>,
    pub(crate) expected_library_revision: u64,
}

impl Default for CreateModelBoundSymbolDialogState {
    fn default() -> Self {
        Self {
            open: false,
            target: String::new(),
            source_mode: CreateSymbolSourceMode::Model,
            model_source: None,
            schematic_source: None,
            template: CreateSymbolTemplate::OperationalAmplifier5Pin,
            pins: Vec::new(),
            symbol: true,
            parameter_form: true,
            simulation_test_fixture: false,
            pin_contract_reviewed: false,
            selected_pin: None,
            dirty: false,
            discard_confirm: false,
            validation_error: None,
            expected_library_revision: 0,
        }
    }
}

impl CreateModelBoundSymbolDialogState {
    pub(super) fn close(&mut self) {
        *self = Self::default();
    }

    pub(super) fn mark_edited(&mut self) {
        self.dirty = true;
        self.discard_confirm = false;
        self.validation_error = None;
    }

    pub(super) fn attempt_close(&mut self) {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
        } else {
            self.close();
        }
    }

    pub(super) fn source_label(&self, mode: CreateSymbolSourceMode) -> String {
        match mode {
            CreateSymbolSourceMode::Model => self.model_source.as_ref().map_or_else(
                || "Model contract unavailable".to_owned(),
                CreateSymbolModelSource::label,
            ),
            CreateSymbolSourceMode::ExistingSchematicPins => {
                self.schematic_source.as_ref().map_or_else(
                    || "Existing schematic pins unavailable".to_owned(),
                    |source| source.label(),
                )
            }
            CreateSymbolSourceMode::BlankExplicitContract => "Blank explicit contract".to_owned(),
        }
    }

    pub(super) fn source_available(&self, mode: CreateSymbolSourceMode) -> bool {
        match mode {
            CreateSymbolSourceMode::Model => self.model_source.is_some(),
            CreateSymbolSourceMode::ExistingSchematicPins => self.schematic_source.is_some(),
            CreateSymbolSourceMode::BlankExplicitContract => true,
        }
    }

    pub(super) fn select_source(&mut self, mode: CreateSymbolSourceMode) {
        if mode == self.source_mode || !self.source_available(mode) {
            return;
        }
        self.source_mode = mode;
        self.pins = match mode {
            CreateSymbolSourceMode::Model => self
                .model_source
                .as_ref()
                .map(|source| source.pins.clone())
                .unwrap_or_default(),
            CreateSymbolSourceMode::ExistingSchematicPins => self
                .schematic_source
                .as_ref()
                .map(|source| source.pins.clone())
                .unwrap_or_default(),
            CreateSymbolSourceMode::BlankExplicitContract => Vec::new(),
        };
        self.pin_contract_reviewed = !matches!(mode, CreateSymbolSourceMode::Model)
            || self
                .model_source
                .as_ref()
                .is_none_or(|source| !source.requires_pin_review);
        if mode == CreateSymbolSourceMode::BlankExplicitContract {
            self.simulation_test_fixture = false;
        }
        if mode == CreateSymbolSourceMode::ExistingSchematicPins
            && let Some(source) = self.schematic_source.as_ref()
        {
            self.target = format!("{} / {}", source.reference.library, source.reference.cell);
        }
        self.selected_pin = None;
        self.mark_edited();
    }
}
