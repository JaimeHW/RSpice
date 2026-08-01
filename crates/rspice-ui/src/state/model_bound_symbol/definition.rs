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

    #[cfg(test)]
    pub fn validate_template(
        &self,
        pins: &[SymbolPinDefinition],
        form: &SymbolParameterForm,
    ) -> Result<(), SymbolDefinitionError> {
        validate_executable_template(self, pins, form)
    }

    /// Render the constrained template without evaluating arbitrary text.
    /// Values are substituted only into the validated placeholders.
    #[cfg(test)]
    pub fn render(
        &self,
        name: &str,
        pins: &[SymbolPinDefinition],
        ordered_nodes: &[String],
        parameters: &HashMap<String, String>,
        form: &SymbolParameterForm,
    ) -> Result<String, SymbolDefinitionError> {
        self.validate_template(pins, form)?;
        if ordered_nodes.len() != pins.len() {
            return Err(SymbolDefinitionError::InvalidNetlist(format!(
                "expected {} ordered nodes, received {}",
                pins.len(),
                ordered_nodes.len()
            )));
        }
        if name.trim().is_empty() || name.chars().any(char::is_whitespace) {
            return Err(SymbolDefinitionError::InvalidNetlist(
                "instance name is empty or contains whitespace".to_owned(),
            ));
        }
        let model = self
            .model
            .as_ref()
            .map(|model| model.model.as_str())
            .unwrap_or_default();
        let prefix_length = self.device_prefix.len();
        let Some(reference_prefix) = name.get(..prefix_length) else {
            return Err(SymbolDefinitionError::InvalidNetlist(format!(
                "reference `{name}` must begin with `{}` and have a nonempty suffix",
                self.device_prefix
            )));
        };
        if !reference_prefix.eq_ignore_ascii_case(&self.device_prefix) {
            return Err(SymbolDefinitionError::InvalidNetlist(format!(
                "reference `{name}` must begin with `{}`",
                self.device_prefix
            )));
        }
        let unprefixed_name = &name[prefix_length..];
        if unprefixed_name.is_empty() {
            return Err(SymbolDefinitionError::InvalidNetlist(format!(
                "reference `{name}` must have a nonempty suffix"
            )));
        }
        if !unprefixed_name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(SymbolDefinitionError::InvalidNetlist(format!(
                "reference `{name}` must use an ASCII alphanumeric or underscore suffix"
            )));
        }
        let mut rendered = self
            .template
            .replace("{name}", unprefixed_name)
            .replace("{ref}", name)
            .replace("{nodes}", &ordered_nodes.join(" "))
            .replace("{model}", model);
        let mut parameter_tokens = Vec::new();
        for key in &self.parameter_order {
            if let Some(value) = parameters.get(key) {
                parameter_tokens.push(format!("{key}={value}"));
            }
        }
        rendered = rendered.replace("{params}", &parameter_tokens.join(" "));
        Ok(rendered.split_whitespace().collect::<Vec<_>>().join(" "))
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

    #[cfg(test)]
    pub fn to_json_pretty(&self) -> Result<String, SymbolDefinitionError> {
        self.validate()?;
        serde_json::to_string_pretty(self)
            .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))
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

    pub fn load_from_view(view: &View) -> Result<Option<Self>, SymbolDefinitionError> {
        let Some(encoded) = view.metadata.get(MODEL_BOUND_SYMBOL_METADATA_KEY) else {
            return Ok(None);
        };
        Self::from_json_bytes(encoded.as_bytes(), &view.name).map(Some)
    }

    pub fn store_in_view(&self, view: &mut View) -> Result<(), SymbolDefinitionError> {
        self.validate()?;
        let mut candidate = view.clone();
        project_definition_metadata(self, &mut candidate.metadata)?;
        if self.generated_views.symbol || view.view_type == ViewType::Symbol {
            self.symbol_document()
                .store_in_view(&mut candidate)
                .map_err(SymbolDefinitionError::Serialization)?;
        }
        candidate.modified = true;
        *view = candidate;
        Ok(())
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

    pub fn build_plan(
        &self,
        library: &Library,
    ) -> Result<SymbolConstructionPlan, SymbolDefinitionError> {
        self.validate()?;

        if library.read_only {
            return Err(SymbolDefinitionError::ReadOnlyLibrary(library.name.clone()));
        }
        if library.name != self.identity.library {
            return Err(SymbolDefinitionError::LibraryIdentityMismatch {
                expected: self.identity.library.clone(),
                actual: library.name.clone(),
            });
        }
        let before = library.get_cell(&self.identity.cell).cloned();
        if let SymbolSourceContract::ExistingSchematicPins { schematic_view, .. } = &self.source {
            let view = before
                .as_ref()
                .and_then(|cell| cell.get_view(schematic_view))
                .ok_or_else(|| {
                    SymbolDefinitionError::SourcePinMismatch(format!(
                        "existing schematic view `{schematic_view}` does not exist in {}/{}",
                        self.identity.library, self.identity.cell
                    ))
                })?;
            if !matches!(view.view_type, ViewType::Schematic | ViewType::Testbench) {
                return Err(SymbolDefinitionError::SourcePinMismatch(format!(
                    "existing source view `{schematic_view}` is not a schematic/testbench"
                )));
            }
        }
        if let Some(existing) = &before
            && let Some(current) = definition_from_cell(existing)?
            && current.identity.revision >= self.identity.revision
        {
            return Err(SymbolDefinitionError::NonMonotonicRevision {
                current: current.identity.revision,
                proposed: self.identity.revision,
            });
        }
        let mut after = before
            .clone()
            .unwrap_or_else(|| Cell::new(&self.identity.cell));
        after.name = self.identity.cell.clone();
        project_definition_metadata(self, &mut after.metadata)?;

        if self.generated_views.symbol {
            let mut view = after
                .get_view(SYMBOL_VIEW_NAME)
                .cloned()
                .unwrap_or_else(|| View::new(SYMBOL_VIEW_NAME, ViewType::Symbol));
            view.view_type = ViewType::Symbol;
            self.store_in_view(&mut view)?;
            after.add_view(view);
        }
        if self.generated_views.parameter_form {
            let mut view = after
                .get_view(PARAMETER_FORM_VIEW_NAME)
                .cloned()
                .unwrap_or_else(|| View::new(PARAMETER_FORM_VIEW_NAME, ViewType::Custom));
            project_definition_metadata(self, &mut view.metadata)?;
            view.metadata.insert(
                SYMBOL_PARAMETER_FORM_METADATA_KEY.to_owned(),
                serde_json::to_string(&self.parameter_form)
                    .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
            );
            view.modified = true;
            after.add_view(view);
        }
        if self.generated_views.simulation_test_fixture {
            let mut view = after
                .get_view(TEST_FIXTURE_VIEW_NAME)
                .cloned()
                .unwrap_or_else(|| View::new(TEST_FIXTURE_VIEW_NAME, ViewType::Testbench));
            project_definition_metadata(self, &mut view.metadata)?;
            view.metadata.insert(
                "test_fixture.contract".to_owned(),
                "pin_access_harness.v1".to_owned(),
            );
            view.metadata.insert(
                "test_fixture.buffer".to_owned(),
                serde_json::to_string(&self.test_fixture_contract()?)
                    .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
            );
            view.modified = true;
            after.add_view(view);
        }

        if let SymbolSourceContract::Model { model, .. } = &self.source {
            let view_name = model.implementation_view.view_name();
            let mut view = after
                .get_view(view_name)
                .cloned()
                .unwrap_or_else(|| View::new(view_name, model.implementation_view.view_type()));
            view.view_type = model.implementation_view.view_type();
            project_definition_metadata(self, &mut view.metadata)?;
            view.file_path = model.source_path.as_ref().map(std::path::PathBuf::from);
            if let Some(module_name) = &model.module_name {
                view.metadata
                    .insert("veriloga.module".to_owned(), module_name.clone());
            }
            view.modified = true;
            after.add_view(view);
        }

        let expected_cell_json = before.as_ref().map(serialize_cell).transpose()?;
        let after_cell_json = serialize_cell(&after)?;
        Ok(SymbolConstructionPlan {
            library: library.name.clone(),
            cell: self.identity.cell.clone(),
            expected_cell_json,
            before,
            after,
            after_cell_json,
        })
    }

    pub fn symbol_document(&self) -> SymbolDocument {
        symbol_document_for(self)
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

    /// Build the editable pin-access harness represented by
    /// `test_fixture_contract`. It never invents a stimulus or analysis.
    pub fn build_test_fixture_schematic(&self) -> Result<SchematicState, SymbolDefinitionError> {
        let contract = self.test_fixture_contract()?;
        let model = self.netlist.model.as_ref();
        let ports = contract
            .accesses
            .iter()
            .map(|access| PortSpec {
                name: access.port_name.clone(),
                direction: access.direction,
            })
            .collect::<Vec<_>>();
        let mut binding = LibraryCellInstance::new(
            &contract.library,
            &contract.cell,
            &contract.implementation_view,
        );
        binding.bind_interface(&ports);
        binding.source_path = model
            .and_then(|model| model.source_path.as_ref())
            .map(std::path::PathBuf::from);
        binding.module_name = model
            .and_then(|model| model.module_name.clone())
            .or_else(|| model.map(|model| model.model.clone()));
        binding.netlist_template = Some(self.netlist.template.clone());
        binding.model_section = model.and_then(|model| model.section.clone());
        binding.reference_prefix = Some(self.netlist.device_prefix.clone());
        binding.parameter_order = self.netlist.parameter_order.clone();

        let mut schematic = SchematicState::default();
        let dut = Component::new(1, ComponentType::CellInstance, Point::new(200, 200))
            .with_library_cell(binding)
            .with_name_value(&contract.dut_instance_name, &contract.cell);
        schematic.components.push(dut);

        let document = self.symbol_document();
        let mut ordered_pins = self.pins.iter().collect::<Vec<_>>();
        ordered_pins.sort_by_key(|pin| pin.order);

        let mut component_id = 2u64;
        let mut wire_id = 1u64;
        for (access, definition_pin) in contract.accesses.iter().zip(ordered_pins) {
            let offset = document
                .pin(&access.port_name)
                .and_then(|pin| pin.position)
                .ok_or_else(|| {
                    SymbolDefinitionError::InvalidNetlist(
                        "authored symbol has an unplaced test-fixture terminal".to_owned(),
                    )
                })?;
            let dut_terminal = Point::new(200 + offset.x, 200 + offset.y);
            let (port_position, rotation) = match definition_pin.side {
                SymbolPinSide::Left => (Point::new(60, dut_terminal.y), Rotation::R180),
                SymbolPinSide::Right => (Point::new(340, dut_terminal.y), Rotation::R0),
                SymbolPinSide::Top => (Point::new(dut_terminal.x, 60), Rotation::R270),
                SymbolPinSide::Bottom => (Point::new(dut_terminal.x, 340), Rotation::R90),
            };
            let mut port = Component::new(component_id, ComponentType::Port, port_position)
                .with_rotation(rotation)
                .with_name_value("", &access.port_name);
            port.params = format!(
                "dir={} signal_type={} discipline={} interface_order={} documentation={}_pin_access",
                access.direction.keyword(),
                match access.electrical_type {
                    SymbolElectricalType::Logic => "logic",
                    SymbolElectricalType::Power | SymbolElectricalType::Ground => "power",
                    _ => "analog",
                },
                if access.electrical_type == SymbolElectricalType::Logic {
                    "logic"
                } else {
                    "electrical"
                },
                access.order,
                access.port_name
            );
            let port_id = port.id;
            let (_, port_terminal) = port.terminal_positions()[0];
            schematic.components.push(port);
            let route = if dut_terminal.x == port_terminal.x || dut_terminal.y == port_terminal.y {
                vec![dut_terminal, port_terminal]
            } else {
                vec![
                    dut_terminal,
                    Point::new(port_terminal.x, dut_terminal.y),
                    port_terminal,
                ]
            };
            schematic.wires.push(Wire::new(wire_id, route));
            schematic
                .connections
                .push(WireConnection::new(wire_id, 0, 1, &access.port_name));
            schematic
                .connections
                .push(WireConnection::new(wire_id, 1, port_id, "P"));
            component_id += 1;
            wire_id += 1;

            if access.ground {
                let ground = Component::new(
                    component_id,
                    ComponentType::Ground,
                    Point::new(dut_terminal.x, dut_terminal.y.saturating_add(60)),
                );
                let ground_id = ground.id;
                let (_, ground_terminal) = ground.terminal_positions()[0];
                schematic.components.push(ground);
                schematic
                    .wires
                    .push(Wire::new(wire_id, vec![dut_terminal, ground_terminal]));
                schematic
                    .connections
                    .push(WireConnection::new(wire_id, 0, 1, &access.port_name));
                schematic
                    .connections
                    .push(WireConnection::new(wire_id, 1, ground_id, "GND"));
                component_id += 1;
                wire_id += 1;
            }
        }
        schematic.is_dirty = true;
        schematic.needs_fit = true;
        Ok(schematic)
    }
}

fn symbol_document_for(definition: &ModelBoundSymbolDefinition) -> SymbolDocument {
    let half_width = 40;
    let half_height = 40.max(((definition.pins.len() as i32 + 1) / 2) * 10);
    let body = if let Some(imported) = &definition.imported_graphic {
        imported.shapes.clone()
    } else {
        match definition.graphic_template {
            SymbolGraphicTemplate::OperationalAmplifier5Pin => vec![SymbolShape::Polyline {
                points: vec![
                    Point::new(-half_width, -half_height),
                    Point::new(half_width, 0),
                    Point::new(-half_width, half_height),
                ],
                closed: true,
            }],
            SymbolGraphicTemplate::RectangularIc | SymbolGraphicTemplate::RfNPort => {
                vec![SymbolShape::Polyline {
                    points: vec![
                        Point::new(-half_width, -half_height),
                        Point::new(half_width, -half_height),
                        Point::new(half_width, half_height),
                        Point::new(-half_width, half_height),
                    ],
                    closed: true,
                }]
            }
        }
    };
    if let Some(imported) = &definition.imported_graphic
        && !imported.pin_anchors.is_empty()
    {
        let pins =
            if definition.pins.is_empty() {
                imported
                    .pin_anchors
                    .iter()
                    .map(|anchor| {
                        SymbolPin::new(&anchor.name, PortDirection::InOut, Some(anchor.position))
                    })
                    .collect()
            } else {
                let mut ordered = definition.pins.iter().collect::<Vec<_>>();
                ordered.sort_by_key(|pin| pin.order);
                ordered
                    .into_iter()
                    .zip(&imported.pin_anchors)
                    .map(|(pin, anchor)| {
                        let offset = match pin.side {
                            SymbolPinSide::Left | SymbolPinSide::Right => anchor.position.y,
                            SymbolPinSide::Top | SymbolPinSide::Bottom => anchor.position.x,
                        };
                        SymbolPin::new(&pin.name, pin.direction, Some(anchor.position))
                            .with_contract(pin.electrical_type, pin.side, offset)
                    })
                    .collect()
            };
        return SymbolDocument {
            pins,
            body,
            origin: Point::origin(),
            name_anchor: Point::new(-half_width, -half_height - 30),
            value_anchor: Point::new(-half_width, half_height + 30),
        };
    }
    let mut side_counts = HashMap::<SymbolPinSide, usize>::new();
    for pin in &definition.pins {
        *side_counts.entry(pin.side).or_default() += 1;
    }
    let mut side_indexes = HashMap::<SymbolPinSide, usize>::new();
    let mut ordered = definition.pins.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|pin| pin.order);
    let pins = ordered
        .into_iter()
        .map(|pin| {
            let index = side_indexes.entry(pin.side).or_default();
            let count = side_counts[&pin.side];
            let centered = (*index as i32 * 2 + 1 - count as i32) * 10;
            *index += 1;
            let position = match pin.side {
                SymbolPinSide::Left => Point::new(-half_width - 20, centered),
                SymbolPinSide::Right => Point::new(half_width + 20, centered),
                SymbolPinSide::Top => Point::new(centered, -half_height - 20),
                SymbolPinSide::Bottom => Point::new(centered, half_height + 20),
            };
            SymbolPin::new(&pin.name, pin.direction, Some(position)).with_contract(
                pin.electrical_type,
                pin.side,
                centered,
            )
        })
        .collect();
    SymbolDocument {
        pins,
        body,
        origin: Point::origin(),
        name_anchor: Point::new(-half_width, -half_height - 30),
        value_anchor: Point::new(-half_width, half_height + 30),
    }
}

fn project_definition_metadata(
    definition: &ModelBoundSymbolDefinition,
    metadata: &mut HashMap<String, String>,
) -> Result<(), SymbolDefinitionError> {
    metadata.insert(
        MODEL_BOUND_SYMBOL_METADATA_KEY.to_owned(),
        serde_json::to_string(definition)
            .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    metadata.insert(
        SYMBOL_PARAMETER_FORM_METADATA_KEY.to_owned(),
        serde_json::to_string(&definition.parameter_form)
            .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    let mut pins = definition.pins.iter().collect::<Vec<_>>();
    pins.sort_by_key(|pin| pin.order);
    let names = pins.iter().map(|pin| pin.name.clone()).collect::<Vec<_>>();
    let encoded_ports = serde_json::to_string(&names)
        .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?;
    metadata.insert("netlist.ports".to_owned(), encoded_ports.clone());
    metadata.insert("netlist.terminals".to_owned(), encoded_ports);
    metadata.insert(
        "netlist.template".to_owned(),
        definition.netlist.template.clone(),
    );
    metadata.insert(
        "reference.prefix".to_owned(),
        definition.netlist.device_prefix.clone(),
    );
    metadata.insert(
        "netlist.parameter_order".to_owned(),
        serde_json::to_string(&definition.netlist.parameter_order)
            .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    metadata.insert(
        "model.family".to_owned(),
        definition
            .netlist
            .model
            .as_ref()
            .map(|model| model.model.clone())
            .unwrap_or_else(|| "unbound".to_owned()),
    );
    if let Some(model) = &definition.netlist.model {
        if let Some(source_path) = &model.source_path {
            metadata.insert("netlist.source_path".to_owned(), source_path.clone());
        }
        if let Some(section) = &model.section {
            metadata.insert("netlist.section".to_owned(), section.clone());
            metadata.insert("model.section".to_owned(), section.clone());
        } else {
            metadata.remove("netlist.section");
            metadata.remove("model.section");
        }
        metadata.insert(
            "netlist.implementation_view".to_owned(),
            model.implementation_view.view_name().to_owned(),
        );
        if let Some(module_name) = &model.module_name {
            metadata.insert("veriloga.module".to_owned(), module_name.clone());
        }
    } else {
        metadata.remove("netlist.source_path");
        metadata.remove("netlist.section");
        metadata.remove("model.section");
        metadata.remove("netlist.implementation_view");
        metadata.remove("veriloga.module");
    }
    #[derive(Serialize)]
    struct PlacementParameter<'a> {
        name: &'a str,
        aliases: &'a [String],
        required: bool,
        default: String,
    }
    let parameters = definition
        .parameter_form
        .fields()
        .filter(|field| field.inheritance.emitted_by_rspice())
        .map(|field| PlacementParameter {
            name: &field.key,
            aliases: &field.aliases,
            required: field.required,
            default: field.default.display_string(),
        })
        .collect::<Vec<_>>();
    metadata.insert(
        "cdf.parameter_contract".to_owned(),
        serde_json::to_string(&parameters)
            .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    metadata.insert(
        "cdf.parameter_inheritance".to_owned(),
        serde_json::to_string(
            &definition
                .parameter_form
                .fields()
                .map(|field| (field.key.clone(), field.inheritance))
                .collect::<BTreeMap<_, _>>(),
        )
        .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    metadata.insert(
        "netlist.cell_defaults".to_owned(),
        serde_json::to_string(
            &definition
                .parameter_form
                .fields()
                .filter(|field| field.inheritance == ParameterInheritance::CellDefault)
                .map(|field| (field.key.clone(), field.default.display_string()))
                .collect::<BTreeMap<_, _>>(),
        )
        .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    if let Some(imported) = &definition.imported_graphic {
        metadata.insert(
            SYMBOL_IMPORT_SOURCE_METADATA_KEY.to_owned(),
            serde_json::to_string(imported)
                .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
        );
    } else {
        metadata.remove(SYMBOL_IMPORT_SOURCE_METADATA_KEY);
    }
    Ok(())
}

fn definition_from_cell(
    cell: &Cell,
) -> Result<Option<ModelBoundSymbolDefinition>, SymbolDefinitionError> {
    let Some(encoded) = cell.metadata.get(MODEL_BOUND_SYMBOL_METADATA_KEY) else {
        return Ok(None);
    };
    ModelBoundSymbolDefinition::from_json_bytes(encoded.as_bytes(), &cell.name).map(Some)
}

pub(super) fn serialize_cell(cell: &Cell) -> Result<String, SymbolDefinitionError> {
    serde_json::to_string(cell)
        .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))
}

fn stable_digest(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}
