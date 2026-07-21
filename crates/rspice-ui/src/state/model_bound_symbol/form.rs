use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolParameterField {
    pub key: String,
    pub label: String,
    pub help: String,
    pub property_type: PropertyType,
    pub default: SymbolParameterDefault,
    pub unit: Option<String>,
    pub constraints: SymbolParameterConstraints,
    pub inheritance: ParameterInheritance,
    pub visibility: SymbolParameterVisibility,
    pub required: bool,
    #[serde(default)]
    pub aliases: Vec<String>,
}

impl SymbolParameterField {
    pub fn to_property_definition(
        &self,
        display_order: i32,
        category: &str,
    ) -> Result<PropertyDefinition, SymbolDefinitionError> {
        let default = self.property_value()?;
        let mut definition = PropertyDefinition::new(&self.key)
            .with_display_name(&self.label)
            .with_description(&self.help)
            .with_type(self.property_type)
            .with_default(default)
            .with_order(display_order)
            .with_category(category);
        if let Some(unit) = &self.unit {
            definition = definition.with_unit(unit);
        }
        if self.required {
            definition = definition.required();
        }
        if !self.inheritance.instance_editable() {
            definition = definition.read_only();
        }
        if let Some(max_length) = self.constraints.max_length {
            definition = definition.with_max_length(max_length);
        }
        definition = definition.with_display_mode(match self.visibility {
            SymbolParameterVisibility::Visible => DisplayMode::Inline,
            SymbolParameterVisibility::Advanced => DisplayMode::Advanced,
            SymbolParameterVisibility::Hidden => DisplayMode::Hidden,
        });
        let minimum = self
            .constraints
            .minimum
            .as_deref()
            .map(parse_engineering)
            .transpose()?;
        let maximum = self
            .constraints
            .maximum
            .as_deref()
            .map(parse_engineering)
            .transpose()?;
        if let (Some(minimum), Some(maximum)) = (minimum, maximum) {
            definition = definition.with_range(minimum, maximum);
        } else {
            definition.min_value = minimum;
            definition.max_value = maximum;
        }
        Ok(definition)
    }

    fn property_value(&self) -> Result<PropertyValue, SymbolDefinitionError> {
        Ok(match &self.default {
            SymbolParameterDefault::Number { engineering, unit } => PropertyValue::Number {
                value: parse_engineering(engineering)?,
                unit: unit.clone().or_else(|| self.unit.clone()),
            },
            SymbolParameterDefault::String { value } => PropertyValue::String(value.clone()),
            SymbolParameterDefault::Expression { value } => {
                PropertyValue::Expression(value.clone())
            }
            SymbolParameterDefault::Enum { selected } => PropertyValue::Enum {
                selected: selected.clone(),
                options: self.constraints.enum_values.clone(),
            },
            SymbolParameterDefault::Boolean { value } => PropertyValue::Boolean(*value),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolParameterSection {
    pub key: String,
    pub label: String,
    pub help: String,
    pub fields: Vec<SymbolParameterField>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct SymbolParameterForm {
    pub revision: u64,
    pub sections: Vec<SymbolParameterSection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolFormDiagnostic {
    pub section_key: String,

    pub field_key: Option<String>,
    pub message: String,
}

impl SymbolParameterForm {
    pub fn sections(&self) -> &[SymbolParameterSection] {
        &self.sections
    }

    pub fn add_section(
        &mut self,
        section: SymbolParameterSection,
    ) -> Result<(), SymbolDefinitionError> {
        if self
            .sections
            .iter()
            .any(|existing| existing.key.eq_ignore_ascii_case(&section.key))
        {
            return Err(SymbolDefinitionError::DuplicateParameterSection(
                section.key,
            ));
        }
        self.sections.push(section);
        Ok(())
    }

    pub fn remove_section(&mut self, key: &str) -> Option<SymbolParameterSection> {
        let index = self
            .sections
            .iter()
            .position(|section| section.key.eq_ignore_ascii_case(key))?;
        Some(self.sections.remove(index))
    }

    pub fn move_section(
        &mut self,
        key: &str,
        destination: usize,
    ) -> Result<(), SymbolDefinitionError> {
        let index = self
            .sections
            .iter()
            .position(|section| section.key.eq_ignore_ascii_case(key))
            .ok_or_else(|| SymbolDefinitionError::UnknownParameterSection(key.to_owned()))?;
        if destination >= self.sections.len() {
            return Err(SymbolDefinitionError::InvalidForm(
                "section destination is outside the form".to_owned(),
            ));
        }
        let section = self.sections.remove(index);
        self.sections.insert(destination, section);
        Ok(())
    }

    pub fn add_field(
        &mut self,
        section_key: &str,
        field: SymbolParameterField,
    ) -> Result<(), SymbolDefinitionError> {
        if self.field(&field.key).is_some() {
            return Err(SymbolDefinitionError::DuplicateParameter(field.key));
        }
        let section = self
            .sections
            .iter_mut()
            .find(|section| section.key.eq_ignore_ascii_case(section_key))
            .ok_or_else(|| {
                SymbolDefinitionError::UnknownParameterSection(section_key.to_owned())
            })?;
        section.fields.push(field);
        Ok(())
    }

    pub fn remove_field(&mut self, key: &str) -> Option<SymbolParameterField> {
        for section in &mut self.sections {
            if let Some(index) = section
                .fields
                .iter()
                .position(|field| field.key.eq_ignore_ascii_case(key))
            {
                return Some(section.fields.remove(index));
            }
        }
        None
    }

    pub fn move_field(
        &mut self,
        key: &str,
        destination_section: &str,
        destination: usize,
    ) -> Result<(), SymbolDefinitionError> {
        let (source_section, source_index) = self
            .sections
            .iter()
            .enumerate()
            .find_map(|(section_index, section)| {
                section
                    .fields
                    .iter()
                    .position(|field| field.key.eq_ignore_ascii_case(key))
                    .map(|field_index| (section_index, field_index))
            })
            .ok_or_else(|| SymbolDefinitionError::UnknownParameter(key.to_owned()))?;
        let destination_section = self
            .sections
            .iter()
            .position(|section| section.key.eq_ignore_ascii_case(destination_section))
            .ok_or_else(|| {
                SymbolDefinitionError::UnknownParameterSection(destination_section.to_owned())
            })?;
        let available = self.sections[destination_section].fields.len()
            - usize::from(source_section == destination_section);
        if destination > available {
            return Err(SymbolDefinitionError::InvalidForm(
                "field destination is outside the section".to_owned(),
            ));
        }
        let field = self.sections[source_section].fields.remove(source_index);
        self.sections[destination_section]
            .fields
            .insert(destination, field);
        Ok(())
    }

    pub fn field(&self, key: &str) -> Option<&SymbolParameterField> {
        self.sections
            .iter()
            .flat_map(|section| &section.fields)
            .find(|field| field.key.eq_ignore_ascii_case(key))
    }

    pub fn fields(&self) -> impl Iterator<Item = &SymbolParameterField> {
        self.sections.iter().flat_map(|section| &section.fields)
    }

    /// Canonical order of parameters that RSpice emits. Model-default fields
    /// are deliberately absent so the referenced model remains authoritative.
    pub fn netlist_parameter_order(&self) -> Vec<String> {
        self.fields()
            .filter(|field| field.inheritance.emitted_by_rspice())
            .map(|field| field.key.clone())
            .collect()
    }

    /// Convert this persistent form into the exact dynamic property sheet used
    /// by a placed model-bound instance.
    pub fn to_property_sheet(&self) -> Result<PropertySheet, SymbolDefinitionError> {
        self.validate()?;
        let mut sheet = PropertySheet::new();
        let mut order = 0i32;
        for section in &self.sections {
            for field in &section.fields {
                let definition = field.to_property_definition(order, &section.label)?;
                definition
                    .validate(&definition.default_value)
                    .map_err(SymbolDefinitionError::InvalidForm)?;
                sheet.add(definition);
                order = order.checked_add(1).ok_or_else(|| {
                    SymbolDefinitionError::InvalidForm(
                        "parameter form contains too many fields".to_owned(),
                    )
                })?;
            }
        }
        Ok(sheet)
    }

    pub fn validate_diagnostics(&self) -> Vec<SymbolFormDiagnostic> {
        let mut diagnostics = Vec::new();
        let mut section_keys = HashSet::new();
        let mut field_spellings = HashMap::<String, String>::new();
        for section in &self.sections {
            if validate_key(&section.key).is_err() {
                form_diag(
                    &mut diagnostics,
                    section,
                    None,
                    "invalid stable section key",
                );
            }
            if !section_keys.insert(section.key.to_ascii_lowercase()) {
                form_diag(&mut diagnostics, section, None, "duplicate section key");
            }
            if section.label.trim().is_empty() || section.help.trim().is_empty() {
                form_diag(
                    &mut diagnostics,
                    section,
                    None,
                    "section label and help are required",
                );
            }
            for field in &section.fields {
                validate_parameter_field(section, field, &mut field_spellings, &mut diagnostics);
            }
        }
        diagnostics
    }

    pub fn validate(&self) -> Result<(), SymbolDefinitionError> {
        if self.revision == 0 {
            return Err(SymbolDefinitionError::InvalidForm(
                "form revision is one-based".to_owned(),
            ));
        }
        let diagnostics = self.validate_diagnostics();
        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(SymbolDefinitionError::InvalidForm(
                diagnostics
                    .iter()
                    .map(|finding| match &finding.field_key {
                        Some(field) => {
                            format!("{}/{field}: {}", finding.section_key, finding.message)
                        }
                        None => format!("{}: {}", finding.section_key, finding.message),
                    })
                    .collect::<Vec<_>>()
                    .join("; "),
            ))
        }
    }
}
