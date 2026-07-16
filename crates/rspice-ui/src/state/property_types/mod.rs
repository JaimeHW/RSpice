//! Property Types System
//!
//! Commercial-grade type-safe property definitions matching Cadence Virtuoso CDF (Component
//! Description Format). Provides:
//! - Type-safe property values (Number, String, Expression, Enum, Boolean)
//! - Property definitions with metadata (units, ranges, defaults)
//! - Per-component property sheets
//! - Expression parsing and validation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Property Value Types
// =============================================================================

/// A typed property value.
///
/// Represents the different types of values a component property can hold,
/// matching Cadence CDF property types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyValue {
    /// Numeric value with optional unit (e.g., 1000.0 Ohms)
    Number { value: f64, unit: Option<String> },

    /// String value (e.g., net names, labels)
    String(String),

    /// Expression that can reference other parameters (e.g., "2*vdd", "W/L")
    Expression(String),

    /// Enumerated value (e.g., "nmos" or "pmos" for transistor type)
    Enum {
        selected: String,
        options: Vec<String>,
    },

    /// Boolean flag (e.g., "include body diode")
    Boolean(bool),
}

impl Default for PropertyValue {
    fn default() -> Self {
        PropertyValue::String(String::new())
    }
}

impl PropertyValue {
    /// Create a new number property
    pub fn number(value: f64) -> Self {
        PropertyValue::Number { value, unit: None }
    }

    /// Create a new number property with a unit
    pub fn number_with_unit(value: f64, unit: impl Into<String>) -> Self {
        PropertyValue::Number {
            value,
            unit: Some(unit.into()),
        }
    }

    /// Create a new string property
    pub fn string(value: impl Into<String>) -> Self {
        PropertyValue::String(value.into())
    }

    /// Create a new expression property
    pub fn expression(expr: impl Into<String>) -> Self {
        PropertyValue::Expression(expr.into())
    }

    /// Create a new enum property
    pub fn enumeration(selected: impl Into<String>, options: Vec<String>) -> Self {
        PropertyValue::Enum {
            selected: selected.into(),
            options,
        }
    }

    /// Create a new boolean property
    pub fn boolean(value: bool) -> Self {
        PropertyValue::Boolean(value)
    }

    /// Get the value as a display string
    pub fn display_string(&self) -> String {
        match self {
            PropertyValue::Number { value, unit } => {
                if let Some(u) = unit {
                    format!("{}{}", format_engineering(*value), u)
                } else {
                    format_engineering(*value)
                }
            }
            PropertyValue::String(s) => s.clone(),
            PropertyValue::Expression(e) => format!("{{{}}}", e),
            PropertyValue::Enum { selected, .. } => selected.clone(),
            PropertyValue::Boolean(b) => if *b { "yes" } else { "no" }.to_string(),
        }
    }

    /// Get the raw numeric value if this is a Number
    pub fn as_number(&self) -> Option<f64> {
        match self {
            PropertyValue::Number { value, .. } => Some(*value),
            _ => None,
        }
    }

    /// Get the raw string value if this is a String
    pub fn as_string(&self) -> Option<&str> {
        match self {
            PropertyValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Check if this is an expression
    pub fn is_expression(&self) -> bool {
        matches!(self, PropertyValue::Expression(_))
    }
}

// =============================================================================
// Display Mode & Visibility Control
// =============================================================================

/// How a property should be displayed in the property dialog.
///
/// Matches Cadence CDF displayMode behavior for commercial-grade UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DisplayMode {
    /// Standard inline display in property dialog (default)
    #[default]
    Inline,
    /// Display as read-only (grayed out, not editable)
    Readonly,
    /// Hidden from the property dialog (internal use)
    Hidden,
    /// Only shown when "Show Advanced" is enabled
    Advanced,
}

/// Conditions for property visibility in the dialog.
///
/// Enables dynamic property visibility based on other property values,
/// matching Cadence CDF conditional visibility behavior.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum VisibilityCondition {
    /// Always visible (default)
    #[default]
    Always,
    /// Only visible when value differs from default
    WhenNonDefault,
    /// Visible when the specified property equals the given value
    WhenPropertyEquals { property: String, value: String },
    /// Visible when the specified property is non-empty
    WhenPropertySet(String),
}

// =============================================================================
// Property Definition
// =============================================================================

/// Metadata for a property definition.
///
/// Defines the name, type, constraints, and display information for a property.
/// Extended with commercial-grade UI control fields matching Cadence CDF.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDefinition {
    /// Internal name (e.g., "w", "l", "r")
    pub name: String,

    /// Display name for UI (e.g., "Width", "Length", "Resistance")
    pub display_name: String,

    /// Description/tooltip
    pub description: String,

    /// Property type hint
    pub prop_type: PropertyType,

    /// Default value
    pub default_value: PropertyValue,

    /// Unit for numeric properties (e.g., "m", "Ω", "F")
    pub unit: Option<String>,

    /// Minimum value for numeric properties
    pub min_value: Option<f64>,

    /// Maximum value for numeric properties
    pub max_value: Option<f64>,

    /// Whether this property is required
    pub required: bool,

    /// Whether this property is read-only
    pub read_only: bool,

    /// Order in the property sheet (lower = higher in list)
    pub display_order: i32,

    /// Category for grouping in the property sheet (becomes tabs)
    pub category: String,

    // =========================================================================
    // Commercial-Grade UI Control Fields
    // =========================================================================
    /// How this property is displayed in the dialog
    pub display_mode: DisplayMode,

    /// Condition for visibility (dynamic show/hide)
    pub visibility_condition: VisibilityCondition,

    /// Whether to show this property's value on the schematic symbol
    pub show_on_schematic: bool,

    /// Whether this property can be edited directly on the schematic
    pub editable_on_schematic: bool,
}

impl Default for PropertyDefinition {
    fn default() -> Self {
        Self {
            name: String::new(),
            display_name: String::new(),
            description: String::new(),
            prop_type: PropertyType::String,
            default_value: PropertyValue::String(String::new()),
            unit: None,
            min_value: None,
            max_value: None,
            required: false,
            read_only: false,
            display_order: 0,
            category: "General".to_string(),
            // Commercial-grade UI control defaults
            display_mode: DisplayMode::default(),
            visibility_condition: VisibilityCondition::default(),
            show_on_schematic: false,
            editable_on_schematic: false,
        }
    }
}

impl PropertyDefinition {
    /// Create a new property definition builder
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            display_name: name.clone(),
            name,
            ..Default::default()
        }
    }

    /// Set the display name
    pub fn with_display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = name.into();
        self
    }

    /// Set the description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Set the property type
    pub fn with_type(mut self, prop_type: PropertyType) -> Self {
        self.prop_type = prop_type;
        self
    }

    /// Set the default value
    pub fn with_default(mut self, value: PropertyValue) -> Self {
        self.default_value = value;
        self
    }

    /// Set the unit
    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    /// Set the numeric range
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.min_value = Some(min);
        self.max_value = Some(max);
        self
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Mark as read-only
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    /// Set display order
    pub fn with_order(mut self, order: i32) -> Self {
        self.display_order = order;
        self
    }

    /// Set category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    // =========================================================================
    // Commercial-Grade UI Control Builders
    // =========================================================================

    /// Set the display mode
    pub fn with_display_mode(mut self, mode: DisplayMode) -> Self {
        self.display_mode = mode;
        self
    }

    /// Mark as advanced (only shown in advanced mode)
    pub fn advanced(mut self) -> Self {
        self.display_mode = DisplayMode::Advanced;
        self
    }

    /// Mark as hidden (never shown in dialog)
    pub fn hidden(mut self) -> Self {
        self.display_mode = DisplayMode::Hidden;
        self
    }

    /// Set visibility condition
    pub fn with_visibility(mut self, condition: VisibilityCondition) -> Self {
        self.visibility_condition = condition;
        self
    }

    /// Show only when non-default
    pub fn show_when_nondefault(mut self) -> Self {
        self.visibility_condition = VisibilityCondition::WhenNonDefault;
        self
    }

    /// Mark to display on schematic symbol
    pub fn show_on_schematic(mut self) -> Self {
        self.show_on_schematic = true;
        self
    }

    /// Mark as editable directly on schematic
    pub fn editable_on_schematic(mut self) -> Self {
        self.editable_on_schematic = true;
        self.show_on_schematic = true; // Editable implies visible
        self
    }

    /// Validate a value against this definition
    pub fn validate(&self, value: &PropertyValue) -> Result<(), String> {
        // Check required
        if self.required {
            match value {
                PropertyValue::String(s) if s.is_empty() => {
                    return Err(format!("{} is required", self.display_name));
                }
                PropertyValue::Number { value, .. } if value.is_nan() => {
                    return Err(format!("{} is required", self.display_name));
                }
                _ => {}
            }
        }

        // Check numeric range
        if let PropertyValue::Number { value: v, .. } = value {
            if let Some(min) = self.min_value
                && *v < min
            {
                return Err(format!(
                    "{} must be at least {}",
                    self.display_name,
                    format_engineering(min)
                ));
            }
            if let Some(max) = self.max_value
                && *v > max
            {
                return Err(format!(
                    "{} must be at most {}",
                    self.display_name,
                    format_engineering(max)
                ));
            }
        }

        // Check enum value is valid
        if let PropertyValue::Enum { selected, options } = value
            && !options.contains(selected)
        {
            return Err(format!(
                "{} must be one of: {}",
                self.display_name,
                options.join(", ")
            ));
        }

        Ok(())
    }
}

/// Property type hint for UI rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PropertyType {
    /// Numeric value
    Number,
    /// Text string
    #[default]
    String,
    /// Expression (can contain variables)
    Expression,
    /// Enumerated list
    Enum,
    /// Boolean flag
    Boolean,
}

// =============================================================================
// Property Sheet
// =============================================================================

/// A collection of property definitions for a component type.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PropertySheet {
    /// Property definitions indexed by name
    definitions: HashMap<String, PropertyDefinition>,

    /// Ordered list of property names for display
    order: Vec<String>,
}

impl PropertySheet {
    /// Create a new empty property sheet
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a property definition
    pub fn add(&mut self, def: PropertyDefinition) {
        let name = def.name.clone();
        if !self.order.contains(&name) {
            self.order.push(name.clone());
        }
        self.definitions.insert(name, def);
    }

    /// Get a property definition by name
    pub fn get(&self, name: &str) -> Option<&PropertyDefinition> {
        self.definitions.get(name)
    }

    /// Iterate over all property definitions in display order
    pub fn iter(&self) -> impl Iterator<Item = &PropertyDefinition> {
        let mut sorted: Vec<_> = self.definitions.values().collect();
        sorted.sort_by_key(|d| d.display_order);
        sorted.into_iter()
    }

    /// Get property names in order
    pub fn names(&self) -> &[String] {
        &self.order
    }

    /// Get the number of properties
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// Get definitions grouped by category
    pub fn by_category(&self) -> HashMap<String, Vec<&PropertyDefinition>> {
        let mut result: HashMap<String, Vec<&PropertyDefinition>> = HashMap::new();
        for def in self.definitions.values() {
            result.entry(def.category.clone()).or_default().push(def);
        }
        // Sort each category by display order
        for defs in result.values_mut() {
            defs.sort_by_key(|d| d.display_order);
        }
        result
    }
}

// =============================================================================
// Component Property Registry
// =============================================================================

use crate::state::ComponentType;

mod registry;

/// Registry of property sheets for all component types.
///
/// This provides the "CDF" (Component Description Format) equivalent,
/// defining what properties each component type supports.
#[derive(Debug, Clone)]
pub struct PropertyRegistry {
    sheets: HashMap<ComponentType, PropertySheet>,
}

impl Default for PropertyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Engineering Value Formatting (local helper)
// =============================================================================

/// Format a value with engineering notation (SI prefixes)
pub fn format_engineering(value: f64) -> String {
    let abs_value = value.abs();

    let (scaled, suffix) = if abs_value >= 1e12 {
        (value / 1e12, "T")
    } else if abs_value >= 1e9 {
        (value / 1e9, "G")
    } else if abs_value >= 1e6 {
        // `M` is milli in SPICE-compatible engineering input. Emit the
        // unambiguous spelling so a displayed property round-trips through
        // the editor parser without a nine-order-of-magnitude change.
        (value / 1e6, "Meg")
    } else if abs_value >= 1e3 {
        (value / 1e3, "k")
    } else if abs_value >= 1.0 || abs_value == 0.0 {
        (value, "")
    } else if abs_value >= 1e-3 {
        (value * 1e3, "m")
    } else if abs_value >= 1e-6 {
        (value * 1e6, "u")
    } else if abs_value >= 1e-9 {
        (value * 1e9, "n")
    } else if abs_value >= 1e-12 {
        (value * 1e12, "p")
    } else if abs_value >= 1e-15 {
        (value * 1e15, "f")
    } else {
        (value * 1e18, "a")
    };

    // Format with appropriate precision
    let eps = 1e-9;
    let is_int = (scaled.round() - scaled).abs() < eps;

    if is_int {
        format!("{:.0}{}", scaled.round(), suffix)
    } else {
        format!("{:.3}{}", scaled, suffix)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::format_engineering;

    #[test]
    fn engineering_display_uses_unambiguous_mega_for_editor_round_trip() {
        let displayed = format_engineering(3.3e6);
        assert_eq!(displayed, "3.300Meg");
        assert_eq!(
            crate::properties::parse_engineering_value(&displayed).unwrap(),
            3.3e6
        );
        assert_eq!(
            crate::properties::parse_engineering_value("3.3M").unwrap(),
            3.3e-3
        );
    }
}
