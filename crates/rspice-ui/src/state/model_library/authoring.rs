//! Project-authored device models.
//!
//! Models a project defines itself rather than inheriting from a PDK. They
//! carry their own revision so a design can name the exact model text it was
//! simulated against.

use std::collections::{BTreeMap, BTreeSet};

use super::DeviceModel;

/// Complete editable definition for one project-owned SPICE `.model` card.
///
/// This is a transaction input, not a second persisted model database. The
/// manager renders it to canonical source, reparses that source through the
/// core parser, and publishes the bytes and browse projection atomically.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectModelDefinition {
    pub name: String,
    pub spice_type: String,
    pub description: String,
    pub numeric_parameters: BTreeMap<String, f64>,
    pub string_parameters: BTreeMap<String, String>,
}

impl ProjectModelDefinition {
    #[must_use]
    pub fn from_device_model(model: &DeviceModel) -> Self {
        Self {
            name: model.name.clone(),
            spice_type: model
                .spice_type
                .clone()
                .unwrap_or_else(|| model.model_type.display_name().to_ascii_uppercase()),
            description: model.description.clone(),
            numeric_parameters: model
                .parameters
                .iter()
                .map(|(name, value)| (name.clone(), *value))
                .collect(),
            string_parameters: model
                .string_parameters
                .iter()
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        validate_identifier("model name", &self.name, false)?;
        validate_identifier("model type", &self.spice_type, true)?;
        if self.description.chars().any(|character| character == '\0') {
            return Err("Model description contains a NUL character".to_owned());
        }

        let mut canonical_names = BTreeSet::new();
        for (name, value) in &self.numeric_parameters {
            validate_parameter_name(name)?;
            let canonical = name.to_ascii_lowercase();
            if !canonical_names.insert(canonical) {
                return Err(format!(
                    "Model parameter '{name}' is declared more than once (parameter names are case-insensitive)"
                ));
            }
            if !value.is_finite() {
                return Err(format!(
                    "Model parameter '{name}' must be finite, got {value}"
                ));
            }
        }
        for (name, value) in &self.string_parameters {
            validate_parameter_name(name)?;
            let canonical = name.to_ascii_lowercase();
            if !canonical_names.insert(canonical) {
                return Err(format!(
                    "Model parameter '{name}' is declared more than once (parameter names are case-insensitive)"
                ));
            }
            if value.is_empty()
                || value.chars().any(|character| {
                    character.is_whitespace()
                        || character.is_control()
                        || matches!(character, '(' | ')' | '=')
                })
            {
                return Err(format!(
                    "String model parameter '{name}' must be one non-empty SPICE token without whitespace, parentheses, or '='"
                ));
            }
        }
        Ok(())
    }

    pub fn canonical_source(&self) -> Result<String, String> {
        self.validate()?;
        let mut source = String::new();
        for line in self
            .description
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            source.push_str("* ");
            source.push_str(line);
            source.push('\n');
        }
        source.push_str(".model ");
        source.push_str(&self.name);
        source.push(' ');
        source.push_str(&self.spice_type.to_ascii_uppercase());
        if self.numeric_parameters.is_empty() && self.string_parameters.is_empty() {
            source.push('\n');
            return Ok(source);
        }
        source.push_str(" (");
        for (name, value) in &self.numeric_parameters {
            source.push(' ');
            source.push_str(&name.to_ascii_uppercase());
            source.push('=');
            source.push_str(&value.to_string());
        }
        for (name, value) in &self.string_parameters {
            source.push(' ');
            source.push_str(&name.to_ascii_uppercase());
            source.push_str("=\"");
            for character in value.chars() {
                if matches!(character, '\\' | '"') {
                    source.push('\\');
                }
                source.push(character);
            }
            source.push('"');
        }
        source.push_str(" )\n");
        Ok(source)
    }
}

fn validate_identifier(label: &str, value: &str, require_alpha_first: bool) -> Result<(), String> {
    if value.is_empty() || value.trim() != value {
        return Err(format!(
            "{label} must not be empty or contain outer whitespace"
        ));
    }
    let mut characters = value.chars();
    let first = characters.next().expect("non-empty identifier");
    if !(first.is_ascii_alphabetic()
        || first == '_'
        || (!require_alpha_first && first.is_ascii_digit()))
        || !characters.all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '.' | '$' | '-')
        })
    {
        return Err(format!("{label} '{value}' is not a safe SPICE identifier"));
    }
    Ok(())
}

fn validate_parameter_name(value: &str) -> Result<(), String> {
    validate_identifier("model parameter", value, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_source_is_deterministic_and_rejects_case_collisions() {
        let mut definition = ProjectModelDefinition {
            name: "nch_owned".to_owned(),
            spice_type: "nmos".to_owned(),
            description: "Project model".to_owned(),
            numeric_parameters: BTreeMap::from([
                ("vth0".to_owned(), 0.48),
                ("level".to_owned(), 1.0),
            ]),
            string_parameters: BTreeMap::from([("version_tag".to_owned(), "r1".to_owned())]),
        };
        assert_eq!(
            definition.canonical_source().expect("valid source"),
            "* Project model\n.model nch_owned NMOS ( LEVEL=1 VTH0=0.48 VERSION_TAG=\"r1\" )\n"
        );

        definition.string_parameters =
            BTreeMap::from([("version_tag".to_owned(), "r\\1".to_owned())]);
        assert_eq!(
            definition.canonical_source().expect("escaped source"),
            "* Project model\n.model nch_owned NMOS ( LEVEL=1 VTH0=0.48 VERSION_TAG=\"r\\\\1\" )\n"
        );

        definition
            .string_parameters
            .insert("VTH0".to_owned(), "duplicate".to_owned());
        assert!(
            definition
                .validate()
                .expect_err("collision")
                .contains("more than once")
        );
    }
}
