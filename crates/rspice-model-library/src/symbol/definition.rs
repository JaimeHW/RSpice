//! Binding a symbol to a netlist device.
//!
//! Maps the symbol's pins onto the device's terminals in the order the
//! netlist line requires. The order is explicit because a SPICE card is
//! positional and a mismatched pin order is silently wrong.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolNetlistBinding {
    pub device_prefix: String,
    pub model: Option<SymbolModelReference>,
    pub template: String,
    pub parameter_order: Vec<String>,
}

impl SymbolNetlistBinding {
    pub fn unbound() -> Self {
        Self {
            device_prefix: String::new(),
            model: None,
            template: String::new(),
            parameter_order: Vec::new(),
        }
    }

    pub fn is_executable(&self) -> bool {
        !self.device_prefix.is_empty() && !self.template.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratedSymbolViews {
    pub symbol: bool,
    pub parameter_form: bool,
    pub simulation_test_fixture: bool,
}

impl Default for GeneratedSymbolViews {
    fn default() -> Self {
        Self {
            symbol: true,
            parameter_form: true,
            simulation_test_fixture: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedGraphicFormat {
    Svg,

    Edif,
    LtspiceAsy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportedPinAnchor {
    pub name: String,
    pub spice_order: usize,
    pub position: Point,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportedGraphicSource {
    pub format: ImportedGraphicFormat,
    pub source_name: String,
    pub source: String,
    pub primitive_count: usize,
    /// Validated, renderer-native geometry. Electrical pins are intentionally
    /// not derived from these shapes.
    pub shapes: Vec<SymbolShape>,
    pub pin_anchors: Vec<ImportedPinAnchor>,
    pub attributes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelBoundSymbolDefinition {
    pub schema_version: u32,
    pub identity: SymbolIdentity,
    pub source: SymbolSourceContract,
    pub pins: Vec<SymbolPinDefinition>,
    pub graphic_template: SymbolGraphicTemplate,
    pub parameter_form: SymbolParameterForm,
    pub netlist: SymbolNetlistBinding,
    pub generated_views: GeneratedSymbolViews,
    pub imported_graphic: Option<ImportedGraphicSource>,
}

impl ModelBoundSymbolDefinition {
    pub fn new(
        identity: SymbolIdentity,
        source: SymbolSourceContract,
        pins: Vec<SymbolPinDefinition>,
        graphic_template: SymbolGraphicTemplate,
        parameter_form: SymbolParameterForm,
        netlist: SymbolNetlistBinding,
        generated_views: GeneratedSymbolViews,
    ) -> Self {
        Self {
            schema_version: MODEL_BOUND_SYMBOL_SCHEMA_VERSION,
            identity,
            source,
            pins,
            graphic_template,
            parameter_form,
            netlist,
            generated_views,
            imported_graphic: None,
        }
    }

    /// Explicitly unbound import target. It persists for review but has no
    /// electrical interface or executable placement contract.
    pub fn review_only(library: impl Into<String>, cell: impl Into<String>) -> Self {
        let library = library.into();
        let cell = cell.into();
        Self::new(
            SymbolIdentity::new(&library, &cell, 1, format!("review:{library}/{cell}")),
            SymbolSourceContract::BlankExplicitContract,
            Vec::new(),
            SymbolGraphicTemplate::RectangularIc,
            SymbolParameterForm {
                revision: 1,
                sections: Vec::new(),
            },
            SymbolNetlistBinding::unbound(),
            GeneratedSymbolViews {
                symbol: true,
                parameter_form: false,
                simulation_test_fixture: false,
            },
        )
    }

    pub fn validate(&self) -> Result<(), SymbolDefinitionError> {
        if self.schema_version != MODEL_BOUND_SYMBOL_SCHEMA_VERSION {
            return Err(SymbolDefinitionError::UnsupportedSchema(
                self.schema_version,
            ));
        }
        validate_identity(&self.identity)?;
        if !self.source.is_explicitly_unbound_for_review() || !self.pins.is_empty() {
            validate_pins(&self.pins)?;
        }
        self.parameter_form.validate()?;

        validate_source(&self.source, &self.pins)?;
        validate_netlist(
            &self.netlist,
            &self.source,
            &self.pins,
            &self.parameter_form,
        )?;
        if !self.generated_views.symbol
            && !self.generated_views.parameter_form
            && !self.generated_views.simulation_test_fixture
        {
            return Err(SymbolDefinitionError::NoGeneratedViews);
        }
        if self.generated_views.simulation_test_fixture
            && self.source.is_explicitly_unbound_for_review()
        {
            return Err(SymbolDefinitionError::InvalidNetlist(
                "an unbound review symbol cannot generate a test fixture".to_owned(),
            ));
        }
        if let Some(imported) = &self.imported_graphic {
            validate_imported_graphic(imported)?;
            validate_import_pin_anchors(self)?;
        } else if self.source.is_explicitly_unbound_for_review() && self.pins.is_empty() {
            return Err(SymbolDefinitionError::Import(
                "a zero-pin review-only definition requires imported graphics".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn from_json_bytes(bytes: &[u8], source_name: &str) -> Result<Self, SymbolDefinitionError> {
        if bytes.len() > MAX_DEFINITION_BYTES {
            return Err(SymbolDefinitionError::Import(format!(
                "{source_name}: definition exceeds the {MAX_DEFINITION_BYTES}-byte limit"
            )));
        }
        let definition = serde_json::from_slice::<Self>(bytes).map_err(|error| {
            SymbolDefinitionError::Import(format!("{source_name}: invalid symbol JSON: {error}"))
        })?;
        definition.validate()?;
        Ok(definition)
    }

    pub fn replace_parameter_form(
        &self,
        replacement: SymbolParameterForm,
    ) -> Result<Self, SymbolDefinitionError> {
        replacement.validate()?;
        let expected_revision = self.parameter_form.revision.checked_add(1).ok_or_else(|| {
            SymbolDefinitionError::InvalidForm("form revision cannot be incremented".to_owned())
        })?;
        if replacement.revision != expected_revision {
            return Err(SymbolDefinitionError::InvalidForm(format!(
                "replacement form revision must be {expected_revision}"
            )));
        }
        let mut next = self.clone();
        next.identity.revision =
            next.identity.revision.checked_add(1).ok_or({
                SymbolDefinitionError::InvalidIdentity("revision cannot be incremented")
            })?;
        next.parameter_form = replacement;
        next.netlist.parameter_order = next.parameter_form.netlist_parameter_order();
        next.validate()?;
        Ok(next)
    }

    pub fn validation_digest(&self) -> Result<String, SymbolDefinitionError> {
        let canonical = serde_json::to_vec(self)
            .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?;
        Ok(stable_digest(&canonical))
    }

    /// Typed pin-access harness contract used by the application to publish
    /// the editable testbench buffer in the same history transaction as the
    /// library views. It contains no guessed source or analysis.
    pub fn test_fixture_contract(
        &self,
    ) -> Result<SymbolTestFixtureContract, SymbolDefinitionError> {
        self.validate()?;
        if self.source.is_explicitly_unbound_for_review() {
            return Err(SymbolDefinitionError::InvalidNetlist(
                "an unbound review symbol cannot generate a test fixture".to_owned(),
            ));
        }
        let mut pins = self.pins.clone();
        pins.sort_by_key(|pin| pin.order);
        let implementation_view = match &self.source {
            SymbolSourceContract::Model { model, .. } => model.implementation_view.view_name(),
            SymbolSourceContract::ExistingSchematicPins { schematic_view, .. } => schematic_view,
            SymbolSourceContract::BlankExplicitContract => unreachable!(),
        };
        Ok(SymbolTestFixtureContract {
            schema_version: 1,
            library: self.identity.library.clone(),
            cell: self.identity.cell.clone(),
            implementation_view: implementation_view.to_owned(),
            dut_instance_name: format!("{}DUT", self.netlist.device_prefix),
            accesses: pins
                .into_iter()
                .map(|pin| SymbolTestFixtureAccess {
                    port_name: pin.name,
                    order: pin.order,
                    electrical_type: pin.electrical_type,
                    direction: pin.direction,
                    ground: pin.electrical_type == SymbolElectricalType::Ground,
                })
                .collect(),
        })
    }
}

fn stable_digest(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolTestFixtureAccess {
    pub port_name: String,
    pub order: usize,
    pub electrical_type: SymbolElectricalType,
    pub direction: PortDirection,
    pub ground: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolTestFixtureContract {
    pub schema_version: u32,
    pub library: String,
    pub cell: String,
    pub implementation_view: String,
    pub dut_instance_name: String,
    pub accesses: Vec<SymbolTestFixtureAccess>,
}
