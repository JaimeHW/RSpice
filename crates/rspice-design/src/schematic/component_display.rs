//! Per-instance label display and its inherited canvas value.

use serde::{Deserialize, Serialize};

/// Instance parameter-label detail rendered on the canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicParameterLabelVisibility {
    ValuesOnly,
    #[default]
    NamesAndValues,
    Hidden,
}

/// Per-instance label visibility override used by reviewed bulk edits.
///
/// `Inherit` delegates to the device-local canvas annotation policy. Explicit
/// values are durable schematic presentation data and therefore participate
/// in schematic undo/redo and project serialization.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComponentDisplayMode {
    #[default]
    Inherit,
    NameAndValue,
    NameOnly,
    ValueOnly,
    Hidden,
}

impl ComponentDisplayMode {
    pub const ALL_EXPLICIT: [Self; 4] = [
        Self::NameAndValue,
        Self::NameOnly,
        Self::ValueOnly,
        Self::Hidden,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Inherit => "inherit",
            Self::NameAndValue => "name and value",
            Self::NameOnly => "name only",
            Self::ValueOnly => "value only",
            Self::Hidden => "hidden",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        let normalized = value.trim().to_ascii_lowercase().replace(['_', '-'], " ");
        match normalized.as_str() {
            "inherit" | "inherited" => Some(Self::Inherit),
            "name and value" | "both" | "full" => Some(Self::NameAndValue),
            "name only" | "name" => Some(Self::NameOnly),
            "value only" | "value" => Some(Self::ValueOnly),
            "hidden" | "hide" | "none" => Some(Self::Hidden),
            _ => None,
        }
    }

    pub const fn show_name(self, inherited: SchematicParameterLabelVisibility) -> bool {
        match self {
            Self::Inherit => matches!(inherited, SchematicParameterLabelVisibility::NamesAndValues),
            Self::NameAndValue | Self::NameOnly => true,
            Self::ValueOnly | Self::Hidden => false,
        }
    }

    pub const fn show_value(self, inherited: SchematicParameterLabelVisibility) -> bool {
        match self {
            Self::Inherit => !matches!(inherited, SchematicParameterLabelVisibility::Hidden),
            Self::NameAndValue | Self::ValueOnly => true,
            Self::NameOnly | Self::Hidden => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_display_mode_parses_mockup_values_and_inherits_canvas_policy() {
        assert_eq!(
            ComponentDisplayMode::parse("name-and-value"),
            Some(ComponentDisplayMode::NameAndValue)
        );
        assert_eq!(
            ComponentDisplayMode::parse("value only"),
            Some(ComponentDisplayMode::ValueOnly)
        );
        assert!(ComponentDisplayMode::parse("sometimes").is_none());
        assert!(
            !ComponentDisplayMode::Inherit.show_name(SchematicParameterLabelVisibility::ValuesOnly)
        );
        assert!(
            ComponentDisplayMode::Inherit.show_value(SchematicParameterLabelVisibility::ValuesOnly)
        );
        assert!(
            !ComponentDisplayMode::Hidden
                .show_value(SchematicParameterLabelVisibility::NamesAndValues)
        );
    }
}
