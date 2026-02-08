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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VisibilityCondition {
    /// Always visible (default)
    Always,
    /// Only visible when value differs from default
    WhenNonDefault,
    /// Visible when the specified property equals the given value
    WhenPropertyEquals { property: String, value: String },
    /// Visible when the specified property is non-empty
    WhenPropertySet(String),
}

impl Default for VisibilityCondition {
    fn default() -> Self {
        VisibilityCondition::Always
    }
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
            if let Some(min) = self.min_value {
                if *v < min {
                    return Err(format!(
                        "{} must be at least {}",
                        self.display_name,
                        format_engineering(min)
                    ));
                }
            }
            if let Some(max) = self.max_value {
                if *v > max {
                    return Err(format!(
                        "{} must be at most {}",
                        self.display_name,
                        format_engineering(max)
                    ));
                }
            }
        }

        // Check enum value is valid
        if let PropertyValue::Enum { selected, options } = value {
            if !options.contains(selected) {
                return Err(format!(
                    "{} must be one of: {}",
                    self.display_name,
                    options.join(", ")
                ));
            }
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

impl PropertyRegistry {
    /// Create a new registry with default property sheets for all component types
    pub fn new() -> Self {
        let mut registry = Self {
            sheets: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    /// Get the property sheet for a component type
    pub fn get(&self, comp_type: ComponentType) -> Option<&PropertySheet> {
        self.sheets.get(&comp_type)
    }

    /// Register default property sheets for all standard components
    fn register_defaults(&mut self) {
        self.register_passive_components();
        self.register_sources();
        self.register_semiconductors();
        self.register_controlled_sources();
    }

    fn register_passive_components(&mut self) {
        self.register_resistor();
        self.register_capacitor();
        self.register_inductor();
    }

    /// Register Resistor with commercial-grade parameters
    fn register_resistor(&mut self) {
        let mut sheet = PropertySheet::new();

        // Instance category (order 0-9)
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this resistor")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("R1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Electrical category (order 10-19)
        sheet.add(
            PropertyDefinition::new("r")
                .with_display_name("Resistance")
                .with_description("Resistance value in Ohms")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1k"))
                .with_unit("Ω")
                .with_range(0.0, 1e15)
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel resistors")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("scale")
                .with_display_name("Scale Factor")
                .with_description("Resistance scaling factor")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(12)
                .with_category("Electrical"),
        );

        // Temperature category (order 20-29)
        sheet.add(
            PropertyDefinition::new("tc1")
                .with_display_name("Temp Coeff 1")
                .with_description("First-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C")
                .with_order(20)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("tc2")
                .with_display_name("Temp Coeff 2")
                .with_description("Second-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C²")
                .with_order(21)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("tce")
                .with_display_name("Temp Exp Coeff")
                .with_description("Exponential temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("%/°C")
                .with_order(22)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(23)
                .with_category("Temperature"),
        );

        // Noise category (order 30-39)
        sheet.add(
            PropertyDefinition::new("noisy")
                .with_display_name("Noisy")
                .with_description("Enable thermal noise generation")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(30)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::Resistor, sheet);
    }

    /// Register Capacitor with commercial-grade parameters
    fn register_capacitor(&mut self) {
        let mut sheet = PropertySheet::new();

        // Instance category
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("C1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Electrical category
        sheet.add(
            PropertyDefinition::new("c")
                .with_display_name("Capacitance")
                .with_description("Capacitance value in Farads")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1u"))
                .with_unit("F")
                .with_range(0.0, 1e3)
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel capacitors")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("scale")
                .with_display_name("Scale Factor")
                .with_description("Capacitance scaling factor")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(12)
                .with_category("Electrical"),
        );

        // Voltage coefficients (order 20-29) - for voltage-dependent capacitance
        sheet.add(
            PropertyDefinition::new("vc1")
                .with_display_name("Voltage Coeff 1")
                .with_description("First-order voltage coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/V")
                .with_order(20)
                .with_category("Voltage Coefficients"),
        );
        sheet.add(
            PropertyDefinition::new("vc2")
                .with_display_name("Voltage Coeff 2")
                .with_description("Second-order voltage coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/V²")
                .with_order(21)
                .with_category("Voltage Coefficients"),
        );

        // Temperature coefficients
        sheet.add(
            PropertyDefinition::new("tc1")
                .with_display_name("Temp Coeff 1")
                .with_description("First-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C")
                .with_order(30)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("tc2")
                .with_display_name("Temp Coeff 2")
                .with_description("Second-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C²")
                .with_order(31)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(32)
                .with_category("Temperature"),
        );

        // Initial conditions
        sheet.add(
            PropertyDefinition::new("ic")
                .with_display_name("Initial Voltage")
                .with_description("Initial voltage across capacitor for transient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(40)
                .with_category("Initial Conditions"),
        );

        self.sheets.insert(ComponentType::Capacitor, sheet);
    }

    /// Register Inductor with commercial-grade parameters
    fn register_inductor(&mut self) {
        let mut sheet = PropertySheet::new();

        // Instance category
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("L1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Electrical category
        sheet.add(
            PropertyDefinition::new("l")
                .with_display_name("Inductance")
                .with_description("Inductance value in Henries")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1m"))
                .with_unit("H")
                .with_range(0.0, 1e6)
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel inductors")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("scale")
                .with_display_name("Scale Factor")
                .with_description("Inductance scaling factor")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(12)
                .with_category("Electrical"),
        );

        // Resistance (for lossy inductors)
        sheet.add(
            PropertyDefinition::new("r")
                .with_display_name("Series Resistance")
                .with_description("Series DC resistance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("Ω")
                .with_order(13)
                .with_category("Electrical"),
        );

        // Temperature coefficients
        sheet.add(
            PropertyDefinition::new("tc1")
                .with_display_name("Temp Coeff 1")
                .with_description("First-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C")
                .with_order(20)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("tc2")
                .with_display_name("Temp Coeff 2")
                .with_description("Second-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C²")
                .with_order(21)
                .with_category("Temperature"),
        );
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(22)
                .with_category("Temperature"),
        );

        // Initial conditions
        sheet.add(
            PropertyDefinition::new("ic")
                .with_display_name("Initial Current")
                .with_description("Initial current through inductor for transient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(30)
                .with_category("Initial Conditions"),
        );

        // Mutual inductance coupling
        sheet.add(
            PropertyDefinition::new("coupling_factor")
                .with_display_name("Coupling Factor")
                .with_description("K-factor for mutual inductance (0-1)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_range(0.0, 1.0)
                .with_order(31)
                .with_category("Coupling"),
        );
        sheet.add(
            PropertyDefinition::new("coupled_to")
                .with_display_name("Coupled To")
                .with_description("Name of coupled inductor")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(32)
                .with_category("Coupling"),
        );

        self.sheets.insert(ComponentType::Inductor, sheet);
    }

    // =========================================================================
    // Spectre-Parity Helper Functions
    // =========================================================================
    // These helper functions add standard parameter groups to source sheets,
    // ensuring consistency and reducing code duplication across all source types.

    /// Add AC small-signal parameters to a source sheet.
    ///
    /// Parameters: ac (magnitude), acphase (phase in degrees)
    /// Used by: All voltage and current sources for AC analysis
    fn add_ac_params(sheet: &mut PropertySheet, unit: &str, ac_default: f64) {
        sheet.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("Small-signal AC magnitude for AC analysis")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(ac_default))
                .with_unit(unit)
                .with_order(20)
                .with_category("AC"),
        );
        sheet.add(
            PropertyDefinition::new("acphase")
                .with_display_name("AC Phase")
                .with_description("Small-signal AC phase in degrees")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(21)
                .with_category("AC"),
        );
    }

    /// Add Advanced AC analysis parameters (XF/PAC) to a source sheet.
    ///
    /// Parameters: xfmag, pacmag, pacdbm, pacphase
    /// Used by: All sources for transfer function and periodic AC analysis
    fn add_advanced_ac_params(sheet: &mut PropertySheet, unit: &str) {
        sheet.add(
            PropertyDefinition::new("xfmag")
                .with_display_name("XF Magnitude")
                .with_description("Transfer function (XF) analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(unit)
                .with_order(30)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacmag")
                .with_display_name("PAC Magnitude")
                .with_description("Periodic AC analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(unit)
                .with_order(31)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacdbm")
                .with_display_name("PAC Power (dBm)")
                .with_description("Periodic AC power in dBm (alternative to pacmag)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::NEG_INFINITY))
                .with_unit("dBm")
                .with_order(32)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacphase")
                .with_display_name("PAC Phase")
                .with_description("Periodic AC phase in degrees")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(33)
                .with_category("Advanced AC")
                .advanced(),
        );
    }

    /// Add parasitic element parameters to a source sheet.
    ///
    /// Parameters: rs (series resistance), rp (parallel resistance), cpar (parasitic capacitance)
    /// Note: Current sources typically omit `rs` as it would change topology.
    fn add_parasitics_params(sheet: &mut PropertySheet, include_series_resistance: bool) {
        if include_series_resistance {
            sheet.add(
                PropertyDefinition::new("rs")
                    .with_display_name("Series Resistance")
                    .with_description("Internal series resistance (0 = ideal)")
                    .with_type(PropertyType::Expression)
                    .with_default(PropertyValue::number(0.0))
                    .with_unit("Ω")
                    .with_range(0.0, 1e15)
                    .with_order(40)
                    .with_category("Parasitics"),
            );
        }
        sheet.add(
            PropertyDefinition::new("rp")
                .with_display_name("Parallel Resistance")
                .with_description("Parallel leakage resistance (inf = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::INFINITY))
                .with_unit("Ω")
                .with_range(0.0, f64::INFINITY)
                .with_order(41)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parasitic Capacitance")
                .with_description("Parasitic shunt capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("F")
                .with_range(0.0, 1e-3)
                .with_order(42)
                .with_category("Parasitics"),
        );
    }

    /// Add noise contribution parameter to a source sheet.
    ///
    /// Parameters: isnoisy (boolean to enable/disable noise contribution)
    fn add_noise_params(sheet: &mut PropertySheet) {
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution in noise analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(50)
                .with_category("Noise"),
        );
    }

    fn register_sources(&mut self) {
        // DC Voltage Source
        self.register_vsource_dc();
        // AC Voltage Source
        self.register_vsource_ac();
        // Transient Voltage Sources
        self.register_vsource_pulse();
        self.register_vsource_sin();
        self.register_vsource_pwl();
        self.register_vsource_exp();
        self.register_vsource_sffm();

        // DC Current Source
        self.register_isource_dc();
        // AC Current Source
        self.register_isource_ac();
        // Transient Current Sources
        self.register_isource_pulse();
        self.register_isource_sin();
        self.register_isource_pwl();
        self.register_isource_exp();
        self.register_isource_noise();
    }

    /// Register DC Voltage Source with Spectre-parity parameters.
    ///
    /// Implements the complete vsource parameter set matching Cadence Spectre:
    /// - DC bias value
    /// - AC small-signal parameters (magnitude, phase)
    /// - Advanced AC analysis (XF, PAC)
    /// - Parasitics (series/parallel resistance, capacitance)
    /// - Noise contribution control
    fn register_vsource_dc(&mut self) {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this voltage source")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // DC Category (order 10-19)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Voltage")
                .with_description("DC voltage value. Used as operating point for AC analysis.")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("DC")
                .required(),
        );

        // =========================================================================
        // AC Category (order 20-29) - Small-signal AC analysis
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("Small-signal AC magnitude for frequency analysis")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(20)
                .with_category("AC"),
        );
        sheet.add(
            PropertyDefinition::new("acphase")
                .with_display_name("AC Phase")
                .with_description("Small-signal AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(21)
                .with_category("AC"),
        );

        // =========================================================================
        // Advanced AC Category (order 30-39) - XF/PAC analysis
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("xfmag")
                .with_display_name("XF Magnitude")
                .with_description("Transfer function (XF) analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(30)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacmag")
                .with_display_name("PAC Magnitude")
                .with_description("Periodic AC analysis magnitude (linear)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(31)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacdbm")
                .with_display_name("PAC dBm")
                .with_description("Periodic AC magnitude in dBm (alternative to pacmag)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(f64::NEG_INFINITY))
                .with_unit("dBm")
                .with_order(32)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacphase")
                .with_display_name("PAC Phase")
                .with_description("Periodic AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(33)
                .with_category("Advanced AC")
                .advanced(),
        );

        // =========================================================================
        // Parasitics Category (order 40-49) - Non-ideal source characteristics
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("rs")
                .with_display_name("Series Resistance")
                .with_description("Internal series resistance (0 = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("Ω")
                .with_range(0.0, 1e15)
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("rp")
                .with_display_name("Parallel Resistance")
                .with_description("Internal parallel (shunt) resistance (inf = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::INFINITY))
                .with_unit("Ω")
                .with_order(41)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parallel Capacitance")
                .with_description("Parasitic shunt capacitance (0 = none)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("F")
                .with_range(0.0, 1e3)
                .with_order(42)
                .with_category("Parasitics"),
        );

        // =========================================================================
        // Noise Category (order 50-59)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution in noise analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(50)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::VoltageSource, sheet);
    }

    /// Register AC Voltage Source with Spectre-parity parameters.
    ///
    /// Similar to DC source but with AC magnitude = 1V by default (the primary parameter).
    /// Used primarily for AC small-signal analysis.
    fn register_vsource_ac(&mut self) {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this AC voltage source")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // DC Category (order 10-19) - DC offset for operating point
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Offset")
                .with_description("DC offset voltage for operating point calculation")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("DC"),
        );

        // =========================================================================
        // AC Category (order 20-29) - Primary parameters for AC source
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("Small-signal AC magnitude (primary parameter)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0)) // Default to 1V for AC source
                .with_unit("V")
                .with_order(20)
                .with_category("AC")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("acphase")
                .with_display_name("AC Phase")
                .with_description("Small-signal AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(21)
                .with_category("AC"),
        );

        // =========================================================================
        // Advanced AC Category (order 30-39) - XF/PAC analysis
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("xfmag")
                .with_display_name("XF Magnitude")
                .with_description("Transfer function (XF) analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(30)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacmag")
                .with_display_name("PAC Magnitude")
                .with_description("Periodic AC analysis magnitude (linear)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(31)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacdbm")
                .with_display_name("PAC dBm")
                .with_description("Periodic AC magnitude in dBm (alternative to pacmag)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(f64::NEG_INFINITY))
                .with_unit("dBm")
                .with_order(32)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacphase")
                .with_display_name("PAC Phase")
                .with_description("Periodic AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(33)
                .with_category("Advanced AC")
                .advanced(),
        );

        // =========================================================================
        // Parasitics Category (order 40-49)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("rs")
                .with_display_name("Series Resistance")
                .with_description("Internal series resistance (0 = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("Ω")
                .with_range(0.0, 1e15)
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("rp")
                .with_display_name("Parallel Resistance")
                .with_description("Internal parallel (shunt) resistance (inf = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::INFINITY))
                .with_unit("Ω")
                .with_order(41)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parallel Capacitance")
                .with_description("Parasitic shunt capacitance (0 = none)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("F")
                .with_range(0.0, 1e3)
                .with_order(42)
                .with_category("Parasitics"),
        );

        // =========================================================================
        // Noise Category (order 50-59)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution in noise analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(50)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::VoltageSourceAc, sheet);
    }

    /// Register Pulse Voltage Source with all SPICE PULSE parameters
    fn register_vsource_pulse(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // PULSE(V1 V2 TD TR TF PW PER)
        sheet.add(
            PropertyDefinition::new("v1")
                .with_display_name("Initial Value (V1)")
                .with_description("Initial voltage before pulse")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("Pulse")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("v2")
                .with_display_name("Pulsed Value (V2)")
                .with_description("Voltage during pulse")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_unit("V")
                .with_order(11)
                .with_category("Pulse")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time (TD)")
                .with_description("Time delay before first pulse")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(12)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("tr")
                .with_display_name("Rise Time (TR)")
                .with_description("Rise time from V1 to V2")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-9))
                .with_unit("s")
                .with_order(13)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("tf")
                .with_display_name("Fall Time (TF)")
                .with_description("Fall time from V2 to V1")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-9))
                .with_unit("s")
                .with_order(14)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("pw")
                .with_display_name("Pulse Width (PW)")
                .with_description("Duration at V2")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(15)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("per")
                .with_display_name("Period (PER)")
                .with_description("Period of pulse waveform")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(2e-6))
                .with_unit("s")
                .with_order(16)
                .with_category("Pulse"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "V", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "V");
        Self::add_parasitics_params(&mut sheet, true);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::VoltageSourcePulse, sheet);
    }

    /// Register Sinusoidal Voltage Source with all SPICE SIN parameters
    fn register_vsource_sin(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // SIN(VO VA FREQ TD THETA PHASE)
        sheet.add(
            PropertyDefinition::new("vo")
                .with_display_name("DC Offset (VO)")
                .with_description("DC offset voltage")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("va")
                .with_display_name("Amplitude (VA)")
                .with_description("Peak amplitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_unit("V")
                .with_order(11)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("freq")
                .with_display_name("Frequency (FREQ)")
                .with_description("Frequency in Hz")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e6))
                .with_unit("Hz")
                .with_order(12)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time (TD)")
                .with_description("Time delay before sinusoid starts")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(13)
                .with_category("Sinusoid"),
        );
        sheet.add(
            PropertyDefinition::new("theta")
                .with_display_name("Damping (THETA)")
                .with_description("Damping factor (1/tau)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/s")
                .with_order(14)
                .with_category("Sinusoid"),
        );
        sheet.add(
            PropertyDefinition::new("phase")
                .with_display_name("Phase (PHASE)")
                .with_description("Phase offset in degrees")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_order(15)
                .with_category("Sinusoid"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "V", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "V");
        Self::add_parasitics_params(&mut sheet, true);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::VoltageSourceSin, sheet);
    }

    /// Register PWL (Piecewise Linear) Voltage Source
    fn register_vsource_pwl(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // PWL data string (time-value pairs)
        sheet.add(
            PropertyDefinition::new("pwl_data")
                .with_display_name("PWL Data")
                .with_description("Time-value pairs: t1 v1 t2 v2 ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("0 0 1u 1 2u 0"))
                .with_order(10)
                .with_category("PWL")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time")
                .with_description("Time delay before PWL starts")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(11)
                .with_category("PWL"),
        );
        sheet.add(
            PropertyDefinition::new("repeat")
                .with_display_name("Repeat")
                .with_description("Repeat the waveform periodically")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(12)
                .with_category("PWL"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "V", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "V");
        Self::add_parasitics_params(&mut sheet, true);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::VoltageSourcePwl, sheet);
    }

    /// Register Exponential Voltage Source
    fn register_vsource_exp(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // EXP(V1 V2 TD1 TAU1 TD2 TAU2)
        sheet.add(
            PropertyDefinition::new("v1")
                .with_display_name("Initial Value (V1)")
                .with_description("Initial voltage")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("Exponential")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("v2")
                .with_display_name("Peak Value (V2)")
                .with_description("Target voltage of first exponential")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_unit("V")
                .with_order(11)
                .with_category("Exponential")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td1")
                .with_display_name("Rise Delay (TD1)")
                .with_description("Time delay for rising exponential")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(12)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("tau1")
                .with_display_name("Rise Time Const (TAU1)")
                .with_description("Time constant for rising exponential")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(13)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("td2")
                .with_display_name("Fall Delay (TD2)")
                .with_description("Time delay for falling exponential")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(5e-6))
                .with_unit("s")
                .with_order(14)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("tau2")
                .with_display_name("Fall Time Const (TAU2)")
                .with_description("Time constant for falling exponential")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(15)
                .with_category("Exponential"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "V", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "V");
        Self::add_parasitics_params(&mut sheet, true);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::VoltageSourceExp, sheet);
    }

    /// Register SFFM (Single-Frequency FM) Voltage Source
    fn register_vsource_sffm(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // SFFM(VO VA FC MDI FS)
        sheet.add(
            PropertyDefinition::new("vo")
                .with_display_name("DC Offset (VO)")
                .with_description("DC offset voltage")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(10)
                .with_category("SFFM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("va")
                .with_display_name("Amplitude (VA)")
                .with_description("Carrier amplitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_unit("V")
                .with_order(11)
                .with_category("SFFM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("fc")
                .with_display_name("Carrier Freq (FC)")
                .with_description("Carrier frequency")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e6))
                .with_unit("Hz")
                .with_order(12)
                .with_category("SFFM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("mdi")
                .with_display_name("Mod Index (MDI)")
                .with_description("Modulation index")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_order(13)
                .with_category("SFFM"),
        );
        sheet.add(
            PropertyDefinition::new("fs")
                .with_display_name("Signal Freq (FS)")
                .with_description("Signal (modulating) frequency")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e3))
                .with_unit("Hz")
                .with_order(14)
                .with_category("SFFM"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "V", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "V");
        Self::add_parasitics_params(&mut sheet, true);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::VoltageSourceSffm, sheet);
    }

    /// Register DC Current Source with Spectre-parity parameters.
    ///
    /// Mirror of vsource DC parameters but with current units (A instead of V).
    fn register_isource_dc(&mut self) {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this current source")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // DC Category (order 10-19)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Current")
                .with_description("DC current value. Used as operating point for AC analysis.")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(10)
                .with_category("DC")
                .required(),
        );

        // =========================================================================
        // AC Category (order 20-29) - Small-signal AC analysis
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("Small-signal AC magnitude for frequency analysis")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(20)
                .with_category("AC"),
        );
        sheet.add(
            PropertyDefinition::new("acphase")
                .with_display_name("AC Phase")
                .with_description("Small-signal AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(21)
                .with_category("AC"),
        );

        // =========================================================================
        // Advanced AC Category (order 30-39) - XF/PAC analysis
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("xfmag")
                .with_display_name("XF Magnitude")
                .with_description("Transfer function (XF) analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(30)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacmag")
                .with_display_name("PAC Magnitude")
                .with_description("Periodic AC analysis magnitude (linear)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(31)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacdbm")
                .with_display_name("PAC dBm")
                .with_description("Periodic AC magnitude in dBm (alternative to pacmag)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(f64::NEG_INFINITY))
                .with_unit("dBm")
                .with_order(32)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacphase")
                .with_display_name("PAC Phase")
                .with_description("Periodic AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(33)
                .with_category("Advanced AC")
                .advanced(),
        );

        // =========================================================================
        // Parasitics Category (order 40-49) - Non-ideal source characteristics
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("rp")
                .with_display_name("Parallel Resistance")
                .with_description("Internal parallel resistance (inf = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::INFINITY))
                .with_unit("Ω")
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parallel Capacitance")
                .with_description("Parasitic shunt capacitance (0 = none)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("F")
                .with_range(0.0, 1e3)
                .with_order(41)
                .with_category("Parasitics"),
        );

        // =========================================================================
        // Noise Category (order 50-59)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution in noise analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(50)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::CurrentSource, sheet);
    }

    /// Register AC Current Source with Spectre-parity parameters.
    ///
    /// Similar to DC current source but with AC magnitude = 1A by default.
    fn register_isource_ac(&mut self) {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this AC current source")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // DC Category (order 10-19)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Offset")
                .with_description("DC offset current for operating point calculation")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(10)
                .with_category("DC"),
        );

        // =========================================================================
        // AC Category (order 20-29) - Primary parameters for AC source
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("Small-signal AC magnitude (primary parameter)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0)) // Default to 1A for AC source
                .with_unit("A")
                .with_order(20)
                .with_category("AC")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("acphase")
                .with_display_name("AC Phase")
                .with_description("Small-signal AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(21)
                .with_category("AC"),
        );

        // =========================================================================
        // Advanced AC Category (order 30-39)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("xfmag")
                .with_display_name("XF Magnitude")
                .with_description("Transfer function (XF) analysis magnitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(30)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacmag")
                .with_display_name("PAC Magnitude")
                .with_description("Periodic AC analysis magnitude (linear)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(31)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacdbm")
                .with_display_name("PAC dBm")
                .with_description("Periodic AC magnitude in dBm (alternative to pacmag)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(f64::NEG_INFINITY))
                .with_unit("dBm")
                .with_order(32)
                .with_category("Advanced AC")
                .advanced(),
        );
        sheet.add(
            PropertyDefinition::new("pacphase")
                .with_display_name("PAC Phase")
                .with_description("Periodic AC phase angle")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(33)
                .with_category("Advanced AC")
                .advanced(),
        );

        // =========================================================================
        // Parasitics Category (order 40-49)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("rp")
                .with_display_name("Parallel Resistance")
                .with_description("Internal parallel resistance (inf = ideal)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(f64::INFINITY))
                .with_unit("Ω")
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("cpar")
                .with_display_name("Parallel Capacitance")
                .with_description("Parasitic shunt capacitance (0 = none)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("F")
                .with_range(0.0, 1e3)
                .with_order(41)
                .with_category("Parasitics"),
        );

        // =========================================================================
        // Noise Category (order 50-59)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution in noise analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(50)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::CurrentSourceAc, sheet);
    }

    /// Register Pulse Current Source
    fn register_isource_pulse(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // PULSE parameters for current
        sheet.add(
            PropertyDefinition::new("i1")
                .with_display_name("Initial Value (I1)")
                .with_description("Initial current before pulse")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(10)
                .with_category("Pulse")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("i2")
                .with_display_name("Pulsed Value (I2)")
                .with_description("Current during pulse")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-3))
                .with_unit("A")
                .with_order(11)
                .with_category("Pulse")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time (TD)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(12)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("tr")
                .with_display_name("Rise Time (TR)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-9))
                .with_unit("s")
                .with_order(13)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("tf")
                .with_display_name("Fall Time (TF)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-9))
                .with_unit("s")
                .with_order(14)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("pw")
                .with_display_name("Pulse Width (PW)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(15)
                .with_category("Pulse"),
        );
        sheet.add(
            PropertyDefinition::new("per")
                .with_display_name("Period (PER)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(2e-6))
                .with_unit("s")
                .with_order(16)
                .with_category("Pulse"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        // Current sources don't have series resistance (would change node topology)
        Self::add_ac_params(&mut sheet, "A", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "A");
        Self::add_parasitics_params(&mut sheet, false);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::CurrentSourcePulse, sheet);
    }

    /// Register Sinusoidal Current Source
    fn register_isource_sin(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // SIN parameters for current
        sheet.add(
            PropertyDefinition::new("io")
                .with_display_name("DC Offset (IO)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(10)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("ia")
                .with_display_name("Amplitude (IA)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-3))
                .with_unit("A")
                .with_order(11)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("freq")
                .with_display_name("Frequency")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e6))
                .with_unit("Hz")
                .with_order(12)
                .with_category("Sinusoid")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(13)
                .with_category("Sinusoid"),
        );
        sheet.add(
            PropertyDefinition::new("theta")
                .with_display_name("Damping")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/s")
                .with_order(14)
                .with_category("Sinusoid"),
        );
        sheet.add(
            PropertyDefinition::new("phase")
                .with_display_name("Phase")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_order(15)
                .with_category("Sinusoid"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "A", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "A");
        Self::add_parasitics_params(&mut sheet, false);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::CurrentSourceSin, sheet);
    }

    /// Register PWL Current Source
    fn register_isource_pwl(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("pwl_data")
                .with_display_name("PWL Data")
                .with_description("Time-value pairs: t1 i1 t2 i2 ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("0 0 1u 1m 2u 0"))
                .with_order(10)
                .with_category("PWL")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(11)
                .with_category("PWL"),
        );
        sheet.add(
            PropertyDefinition::new("repeat")
                .with_display_name("Repeat")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(12)
                .with_category("PWL"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "A", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "A");
        Self::add_parasitics_params(&mut sheet, false);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::CurrentSourcePwl, sheet);
    }

    /// Register Exponential Current Source
    fn register_isource_exp(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // EXP parameters
        sheet.add(
            PropertyDefinition::new("i1")
                .with_display_name("Initial Value (I1)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(10)
                .with_category("Exponential")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("i2")
                .with_display_name("Peak Value (I2)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-3))
                .with_unit("A")
                .with_order(11)
                .with_category("Exponential")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td1")
                .with_display_name("Rise Delay")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(12)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("tau1")
                .with_display_name("Rise Time Const")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(13)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("td2")
                .with_display_name("Fall Delay")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(5e-6))
                .with_unit("s")
                .with_order(14)
                .with_category("Exponential"),
        );
        sheet.add(
            PropertyDefinition::new("tau2")
                .with_display_name("Fall Time Const")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(15)
                .with_category("Exponential"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics, Noise
        Self::add_ac_params(&mut sheet, "A", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "A");
        Self::add_parasitics_params(&mut sheet, false);
        Self::add_noise_params(&mut sheet);

        self.sheets.insert(ComponentType::CurrentSourceExp, sheet);
    }

    /// Register Noise Current Source (for noise analysis)
    fn register_isource_noise(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Current")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(1)
                .with_category("DC"),
        );
        sheet.add(
            PropertyDefinition::new("noise_type")
                .with_display_name("Noise Type")
                .with_description("Type of noise source")
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(
                    "white",
                    vec![
                        "white".to_string(),
                        "flicker".to_string(),
                        "shot".to_string(),
                    ],
                ))
                .with_order(10)
                .with_category("Noise"),
        );
        sheet.add(
            PropertyDefinition::new("noiseval")
                .with_display_name("Noise Value")
                .with_description("Noise spectral density (A²/Hz)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-24))
                .with_unit("A²/Hz")
                .with_order(11)
                .with_category("Noise"),
        );
        sheet.add(
            PropertyDefinition::new("kf")
                .with_display_name("Flicker Coeff (KF)")
                .with_description("Flicker noise coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(12)
                .with_category("Noise"),
        );
        sheet.add(
            PropertyDefinition::new("af")
                .with_display_name("Flicker Exp (AF)")
                .with_description("Flicker noise exponent")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(13)
                .with_category("Noise"),
        );

        // Spectre-parity categories: AC, Advanced AC, Parasitics (noise source already has noise params)
        Self::add_ac_params(&mut sheet, "A", 0.0);
        Self::add_advanced_ac_params(&mut sheet, "A");
        Self::add_parasitics_params(&mut sheet, false);

        // Add isnoisy flag for consistency (noise source is always noisy by definition)
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Enable noise contribution (always true for noise source)")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(200)
                .with_category("Noise"),
        );

        self.sheets.insert(ComponentType::CurrentSourceNoise, sheet);
    }

    fn register_semiconductors(&mut self) {
        // Diode with commercial-grade parameters
        self.register_diode();

        // MOSFET (NMOS/PMOS) with full Spectre-compatible parameters
        self.register_mosfet();

        // BJT (NPN/PNP) with commercial-grade parameters
        self.register_bjt();

        // JFET (NJFET/PJFET)
        self.register_jfet();
    }

    /// Register diode with all SPICE-standard parameters
    fn register_diode(&mut self) {
        let mut diode = PropertySheet::new();

        // Instance category
        diode.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this diode instance")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("D1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Model category
        diode.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("Diode model name from library")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("D"))
                .with_order(10)
                .with_category("Model"),
        );

        // Geometry category
        diode.add(
            PropertyDefinition::new("area")
                .with_display_name("Area Factor")
                .with_description("Junction area relative to model default")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1e-6, 1e6)
                .with_order(20)
                .with_category("Geometry"),
        );
        diode.add(
            PropertyDefinition::new("pj")
                .with_display_name("Perimeter")
                .with_description("Junction perimeter")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("m")
                .with_order(21)
                .with_category("Geometry"),
        );
        diode.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel devices")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(22)
                .with_category("Geometry"),
        );

        // Temperature category
        diode.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(30)
                .with_category("Temperature"),
        );

        // Initial Conditions category
        diode.add(
            PropertyDefinition::new("off")
                .with_display_name("Initially Off")
                .with_description("Start in off state for DC operating point")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(40)
                .with_category("Initial Conditions"),
        );
        diode.add(
            PropertyDefinition::new("ic")
                .with_display_name("Initial Voltage")
                .with_description("Initial voltage across diode for transient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(41)
                .with_category("Initial Conditions"),
        );

        self.sheets.insert(ComponentType::Diode, diode);
    }

    /// Register MOSFET (NMOS/PMOS) with all Spectre-compatible parameters
    fn register_mosfet(&mut self) {
        let nmos = self.create_mosfet_sheet("M1", "nmos");
        self.sheets.insert(ComponentType::Nmos, nmos);

        let pmos = self.create_mosfet_sheet("M1", "pmos");
        self.sheets.insert(ComponentType::Pmos, pmos);
    }

    /// Create a MOSFET property sheet with commercial-grade parameters
    fn create_mosfet_sheet(&self, default_name: &str, default_model: &str) -> PropertySheet {
        let mut sheet = PropertySheet::new();

        // =========================================================================
        // Instance Category (order 0-9)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this MOSFET instance")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_name))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // =========================================================================
        // Model Category (order 10-19)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("MOSFET model name from PDK library")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_model))
                .with_order(10)
                .with_category("Model"),
        );

        // =========================================================================
        // Geometry Category (order 20-39)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("w")
                .with_display_name("Width")
                .with_description("Channel width (drawn)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1u"))
                .with_unit("m")
                .with_range(1e-9, 1e-3)
                .with_order(20)
                .with_category("Geometry")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("l")
                .with_display_name("Length")
                .with_description("Channel length (drawn)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("180n"))
                .with_unit("m")
                .with_range(1e-9, 1e-3)
                .with_order(21)
                .with_category("Geometry")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel devices (total W = m × nf × w)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(22)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("nf")
                .with_display_name("# Fingers")
                .with_description("Number of gate fingers")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 1000.0)
                .with_order(23)
                .with_category("Geometry"),
        );

        // =========================================================================
        // Parasitics Category (order 40-59)
        // Source/Drain areas and perimeters for junction capacitance
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("as")
                .with_display_name("Source Area")
                .with_description("Source diffusion area for junction capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m²")
                .with_order(40)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("ad")
                .with_display_name("Drain Area")
                .with_description("Drain diffusion area for junction capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m²")
                .with_order(41)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("ps")
                .with_display_name("Source Perimeter")
                .with_description("Source diffusion perimeter for sidewall capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(42)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("pd")
                .with_display_name("Drain Perimeter")
                .with_description("Drain diffusion perimeter for sidewall capacitance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(43)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("nrd")
                .with_display_name("Drain Squares")
                .with_description("Number of squares for drain series resistance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(44)
                .with_category("Parasitics"),
        );
        sheet.add(
            PropertyDefinition::new("nrs")
                .with_display_name("Source Squares")
                .with_description("Number of squares for source series resistance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(45)
                .with_category("Parasitics"),
        );

        // =========================================================================
        // Stress Category (order 60-79) - STI stress effects for advanced nodes
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("sa")
                .with_display_name("SA Distance")
                .with_description("Distance from gate edge to STI on source side")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(60)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sb")
                .with_display_name("SB Distance")
                .with_description("Distance from gate edge to STI on drain side")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(61)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sd")
                .with_display_name("SD Spacing")
                .with_description("Source-drain spacing for multi-finger devices")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(62)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("sca")
                .with_display_name("SCA")
                .with_description("Integral of first distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(63)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("scb")
                .with_display_name("SCB")
                .with_description("Integral of second distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(64)
                .with_category("Stress"),
        );
        sheet.add(
            PropertyDefinition::new("scc")
                .with_display_name("SCC")
                .with_description("Integral of third distribution function for STI stress")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(65)
                .with_category("Stress"),
        );

        // =========================================================================
        // Temperature Category (order 80-89)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(80)
                .with_category("Temperature"),
        );

        // =========================================================================
        // Initial Conditions Category (order 90-99)
        // =========================================================================
        sheet.add(
            PropertyDefinition::new("off")
                .with_display_name("Initially Off")
                .with_description("Start in off state for DC operating point analysis")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(90)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("region")
                .with_display_name("Region Hint")
                .with_description("Estimated operating region for convergence aid")
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(
                    "auto",
                    vec![
                        "auto".to_string(),
                        "off".to_string(),
                        "triode".to_string(),
                        "sat".to_string(),
                        "subth".to_string(),
                    ],
                ))
                .with_order(91)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vds")
                .with_display_name("IC VDS")
                .with_description("Initial drain-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(92)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vgs")
                .with_display_name("IC VGS")
                .with_description("Initial gate-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(93)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vbs")
                .with_display_name("IC VBS")
                .with_description("Initial bulk-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(94)
                .with_category("Initial Conditions"),
        );

        sheet
    }

    /// Register BJT (NPN/PNP) with commercial-grade parameters
    fn register_bjt(&mut self) {
        let npn = self.create_bjt_sheet("Q1", "npn");
        self.sheets.insert(ComponentType::NpnBjt, npn);

        let pnp = self.create_bjt_sheet("Q1", "pnp");
        self.sheets.insert(ComponentType::PnpBjt, pnp);
    }

    /// Create a BJT property sheet with commercial-grade parameters
    fn create_bjt_sheet(&self, default_name: &str, default_model: &str) -> PropertySheet {
        let mut sheet = PropertySheet::new();

        // Instance category
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this BJT instance")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_name))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Model category
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("BJT model name from library")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_model))
                .with_order(10)
                .with_category("Model"),
        );

        // Geometry category
        sheet.add(
            PropertyDefinition::new("area")
                .with_display_name("Area Factor")
                .with_description("Emitter area multiplier relative to model default")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1e-6, 1e6)
                .with_order(20)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("areab")
                .with_display_name("Base Area Factor")
                .with_description("Base area multiplier for parasitic capacitance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1e-6, 1e6)
                .with_order(21)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("areac")
                .with_display_name("Collector Area Factor")
                .with_description("Collector area multiplier for parasitic capacitance")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1e-6, 1e6)
                .with_order(22)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel devices")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(23)
                .with_category("Geometry"),
        );

        // Temperature category
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(30)
                .with_category("Temperature"),
        );

        // Initial Conditions category
        sheet.add(
            PropertyDefinition::new("off")
                .with_display_name("Initially Off")
                .with_description("Start in off state for DC operating point")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(40)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("region")
                .with_display_name("Region Hint")
                .with_description("Estimated operating region for convergence aid")
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(
                    "auto",
                    vec![
                        "auto".to_string(),
                        "off".to_string(),
                        "fwd".to_string(),
                        "rev".to_string(),
                        "sat".to_string(),
                    ],
                ))
                .with_order(41)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vbe")
                .with_display_name("IC VBE")
                .with_description("Initial base-emitter voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(42)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vce")
                .with_display_name("IC VCE")
                .with_description("Initial collector-emitter voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(43)
                .with_category("Initial Conditions"),
        );

        sheet
    }

    /// Register JFET (NJFET/PJFET) with commercial-grade parameters
    fn register_jfet(&mut self) {
        let njfet = self.create_jfet_sheet("J1", "njfet");
        self.sheets.insert(ComponentType::Njfet, njfet);

        let pjfet = self.create_jfet_sheet("J1", "pjfet");
        self.sheets.insert(ComponentType::Pjfet, pjfet);
    }

    /// Create a JFET property sheet with commercial-grade parameters
    fn create_jfet_sheet(&self, default_name: &str, default_model: &str) -> PropertySheet {
        let mut sheet = PropertySheet::new();

        // Instance category
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this JFET instance")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_name))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Model category
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("JFET model name from library")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(default_model))
                .with_order(10)
                .with_category("Model"),
        );

        // Geometry category
        sheet.add(
            PropertyDefinition::new("area")
                .with_display_name("Area Factor")
                .with_description("Device area multiplier")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1e-6, 1e6)
                .with_order(20)
                .with_category("Geometry"),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel devices")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(21)
                .with_category("Geometry"),
        );

        // Temperature category
        sheet.add(
            PropertyDefinition::new("dtemp")
                .with_display_name("Temp Rise")
                .with_description("Instance temperature rise above ambient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°C")
                .with_order(30)
                .with_category("Temperature"),
        );

        // Initial Conditions category
        sheet.add(
            PropertyDefinition::new("off")
                .with_display_name("Initially Off")
                .with_description("Start in off state for DC operating point")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(false))
                .with_order(40)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vds")
                .with_display_name("IC VDS")
                .with_description("Initial drain-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(41)
                .with_category("Initial Conditions"),
        );
        sheet.add(
            PropertyDefinition::new("ic_vgs")
                .with_display_name("IC VGS")
                .with_description("Initial gate-source voltage for transient analysis")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(42)
                .with_category("Initial Conditions"),
        );

        sheet
    }

    fn register_controlled_sources(&mut self) {
        self.register_vcvs();
        self.register_vccs();
        self.register_ccvs();
        self.register_cccs();
    }

    /// Register VCVS (Voltage-Controlled Voltage Source) with commercial parameters
    fn register_vcvs(&mut self) {
        let mut sheet = PropertySheet::new();

        // Instance
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("E1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        // Electrical - Gain
        sheet.add(
            PropertyDefinition::new("gain")
                .with_display_name("Voltage Gain")
                .with_description("Output voltage / Input voltage (linear multiplier)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel sources")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );

        // Polynomial coefficients (for nonlinear behavior)
        sheet.add(
            PropertyDefinition::new("poly")
                .with_display_name("Poly Coefficients")
                .with_description("Polynomial coefficients: output = c0 + c1*v + c2*v² + ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(20)
                .with_category("Polynomial"),
        );

        // AC/Small-signal parameters
        sheet.add(
            PropertyDefinition::new("ac_gain")
                .with_display_name("AC Gain")
                .with_description("AC analysis gain (if different from DC)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_order(30)
                .with_category("AC"),
        );
        sheet.add(
            PropertyDefinition::new("ac_phase")
                .with_display_name("AC Phase")
                .with_description("AC phase shift")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_order(31)
                .with_category("AC"),
        );

        // Limiting parameters
        sheet.add(
            PropertyDefinition::new("vmax")
                .with_display_name("Max Output")
                .with_description("Maximum output voltage (clipping)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e308))
                .with_unit("V")
                .with_order(40)
                .with_category("Limits"),
        );
        sheet.add(
            PropertyDefinition::new("vmin")
                .with_display_name("Min Output")
                .with_description("Minimum output voltage (clipping)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(-1e308))
                .with_unit("V")
                .with_order(41)
                .with_category("Limits"),
        );

        self.sheets.insert(ComponentType::Vcvs, sheet);
    }

    /// Register VCCS (Voltage-Controlled Current Source) with commercial parameters
    fn register_vccs(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("G1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        sheet.add(
            PropertyDefinition::new("gm")
                .with_display_name("Transconductance")
                .with_description("Output current / Input voltage")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-3))
                .with_unit("S")
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );

        // Polynomial
        sheet.add(
            PropertyDefinition::new("poly")
                .with_display_name("Poly Coefficients")
                .with_description("Polynomial: output = c0 + c1*v + c2*v² ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(20)
                .with_category("Polynomial"),
        );

        // AC parameters
        sheet.add(
            PropertyDefinition::new("ac_gm")
                .with_display_name("AC Transconductance")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("S")
                .with_order(30)
                .with_category("AC"),
        );

        // Limiting
        sheet.add(
            PropertyDefinition::new("imax")
                .with_display_name("Max Output")
                .with_description("Maximum output current")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e308))
                .with_unit("A")
                .with_order(40)
                .with_category("Limits"),
        );
        sheet.add(
            PropertyDefinition::new("imin")
                .with_display_name("Min Output")
                .with_description("Minimum output current")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(-1e308))
                .with_unit("A")
                .with_order(41)
                .with_category("Limits"),
        );

        self.sheets.insert(ComponentType::Vccs, sheet);
    }

    /// Register CCVS (Current-Controlled Voltage Source) with commercial parameters
    fn register_ccvs(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("H1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        sheet.add(
            PropertyDefinition::new("rm")
                .with_display_name("Transresistance")
                .with_description("Output voltage / Input current")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1000.0))
                .with_unit("Ω")
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );

        sheet.add(
            PropertyDefinition::new("vref")
                .with_display_name("Sensing Branch")
                .with_description("Name of voltage source sensing control current")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(12)
                .with_category("Electrical"),
        );

        // Polynomial
        sheet.add(
            PropertyDefinition::new("poly")
                .with_display_name("Poly Coefficients")
                .with_description("Polynomial: output = c0 + c1*i + c2*i² ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(20)
                .with_category("Polynomial"),
        );

        // Limiting
        sheet.add(
            PropertyDefinition::new("vmax")
                .with_display_name("Max Output")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e308))
                .with_unit("V")
                .with_order(40)
                .with_category("Limits"),
        );
        sheet.add(
            PropertyDefinition::new("vmin")
                .with_display_name("Min Output")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(-1e308))
                .with_unit("V")
                .with_order(41)
                .with_category("Limits"),
        );

        self.sheets.insert(ComponentType::Ccvs, sheet);
    }

    /// Register CCCS (Current-Controlled Current Source) with commercial parameters
    fn register_cccs(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("F1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );

        sheet.add(
            PropertyDefinition::new("gain")
                .with_display_name("Current Gain")
                .with_description("Output current / Input current")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 10000.0)
                .with_order(11)
                .with_category("Electrical"),
        );

        sheet.add(
            PropertyDefinition::new("vref")
                .with_display_name("Sensing Branch")
                .with_description("Name of voltage source sensing control current")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(12)
                .with_category("Electrical"),
        );

        // Polynomial
        sheet.add(
            PropertyDefinition::new("poly")
                .with_display_name("Poly Coefficients")
                .with_description("Polynomial: output = c0 + c1*i + c2*i² ...")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(20)
                .with_category("Polynomial"),
        );

        // Limiting
        sheet.add(
            PropertyDefinition::new("imax")
                .with_display_name("Max Output")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e308))
                .with_unit("A")
                .with_order(40)
                .with_category("Limits"),
        );
        sheet.add(
            PropertyDefinition::new("imin")
                .with_display_name("Min Output")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(-1e308))
                .with_unit("A")
                .with_order(41)
                .with_category("Limits"),
        );

        self.sheets.insert(ComponentType::Cccs, sheet);
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
        (value / 1e6, "M")
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
    use super::*;

    // =========================================================================
    // PropertyValue Tests
    // =========================================================================

    #[test]
    fn test_property_value_number() {
        let v = PropertyValue::number(1000.0);
        assert_eq!(v.as_number(), Some(1000.0));
        assert_eq!(v.display_string(), "1k");
    }

    #[test]
    fn test_property_value_number_with_unit() {
        let v = PropertyValue::number_with_unit(1000.0, "Ω");
        assert_eq!(v.display_string(), "1kΩ");
    }

    #[test]
    fn test_property_value_string() {
        let v = PropertyValue::string("R1");
        assert_eq!(v.as_string(), Some("R1"));
        assert_eq!(v.display_string(), "R1");
    }

    #[test]
    fn test_property_value_expression() {
        let v = PropertyValue::expression("2*vdd");
        assert!(v.is_expression());
        assert_eq!(v.display_string(), "{2*vdd}");
    }

    #[test]
    fn test_property_value_enum() {
        let v = PropertyValue::enumeration("nmos", vec!["nmos".to_string(), "pmos".to_string()]);
        assert_eq!(v.display_string(), "nmos");
    }

    #[test]
    fn test_property_value_boolean() {
        let v = PropertyValue::boolean(true);
        assert_eq!(v.display_string(), "yes");
        let v = PropertyValue::boolean(false);
        assert_eq!(v.display_string(), "no");
    }

    // =========================================================================
    // PropertyDefinition Tests
    // =========================================================================

    #[test]
    fn test_property_definition_builder() {
        let def = PropertyDefinition::new("resistance")
            .with_display_name("Resistance")
            .with_description("The resistance value")
            .with_type(PropertyType::Number)
            .with_unit("Ω")
            .with_range(0.0, 1e12)
            .with_category("Electrical")
            .required();

        assert_eq!(def.name, "resistance");
        assert_eq!(def.display_name, "Resistance");
        assert_eq!(def.unit, Some("Ω".to_string()));
        assert_eq!(def.min_value, Some(0.0));
        assert_eq!(def.max_value, Some(1e12));
        assert!(def.required);
    }

    #[test]
    fn test_property_definition_validate_required() {
        let def = PropertyDefinition::new("name")
            .with_type(PropertyType::String)
            .required();

        // Empty string should fail
        let result = def.validate(&PropertyValue::string(""));
        assert!(result.is_err());

        // Non-empty should pass
        let result = def.validate(&PropertyValue::string("R1"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_property_definition_validate_range() {
        let def = PropertyDefinition::new("value")
            .with_type(PropertyType::Number)
            .with_range(0.0, 1000.0);

        // In range
        assert!(def.validate(&PropertyValue::number(500.0)).is_ok());

        // Below range
        assert!(def.validate(&PropertyValue::number(-1.0)).is_err());

        // Above range
        assert!(def.validate(&PropertyValue::number(1001.0)).is_err());
    }

    #[test]
    fn test_property_definition_validate_enum() {
        let def = PropertyDefinition::new("type").with_type(PropertyType::Enum);

        // Valid selection
        let valid =
            PropertyValue::enumeration("nmos", vec!["nmos".to_string(), "pmos".to_string()]);
        assert!(def.validate(&valid).is_ok());

        // Invalid selection
        let invalid =
            PropertyValue::enumeration("invalid", vec!["nmos".to_string(), "pmos".to_string()]);
        assert!(def.validate(&invalid).is_err());
    }

    // =========================================================================
    // PropertySheet Tests
    // =========================================================================

    #[test]
    fn test_property_sheet_add_and_get() {
        let mut sheet = PropertySheet::new();
        sheet.add(PropertyDefinition::new("r").with_display_name("Resistance"));
        sheet.add(PropertyDefinition::new("tc1").with_display_name("Temp Coeff 1"));

        assert_eq!(sheet.len(), 2);
        assert!(sheet.get("r").is_some());
        assert!(sheet.get("tc1").is_some());
        assert!(sheet.get("nonexistent").is_none());
    }

    #[test]
    fn test_property_sheet_order() {
        let mut sheet = PropertySheet::new();
        sheet.add(PropertyDefinition::new("c").with_order(2));
        sheet.add(PropertyDefinition::new("a").with_order(0));
        sheet.add(PropertyDefinition::new("b").with_order(1));

        let names: Vec<_> = sheet.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_property_sheet_by_category() {
        let mut sheet = PropertySheet::new();
        sheet.add(PropertyDefinition::new("name").with_category("Instance"));
        sheet.add(PropertyDefinition::new("r").with_category("Electrical"));
        sheet.add(PropertyDefinition::new("tc1").with_category("Temperature"));
        sheet.add(PropertyDefinition::new("tc2").with_category("Temperature"));

        let by_cat = sheet.by_category();
        assert_eq!(by_cat.get("Instance").map(|v| v.len()), Some(1));
        assert_eq!(by_cat.get("Electrical").map(|v| v.len()), Some(1));
        assert_eq!(by_cat.get("Temperature").map(|v| v.len()), Some(2));
    }

    // =========================================================================
    // PropertyRegistry Tests
    // =========================================================================

    #[test]
    fn test_registry_has_all_common_types() {
        let registry = PropertyRegistry::new();

        // Passives
        assert!(registry.get(ComponentType::Resistor).is_some());
        assert!(registry.get(ComponentType::Capacitor).is_some());
        assert!(registry.get(ComponentType::Inductor).is_some());

        // Sources
        assert!(registry.get(ComponentType::VoltageSource).is_some());
        assert!(registry.get(ComponentType::CurrentSource).is_some());

        // Semiconductors
        assert!(registry.get(ComponentType::Diode).is_some());
        assert!(registry.get(ComponentType::Nmos).is_some());
        assert!(registry.get(ComponentType::Pmos).is_some());
        assert!(registry.get(ComponentType::NpnBjt).is_some());
        assert!(registry.get(ComponentType::PnpBjt).is_some());

        // Controlled sources
        assert!(registry.get(ComponentType::Vcvs).is_some());
        assert!(registry.get(ComponentType::Vccs).is_some());
        assert!(registry.get(ComponentType::Ccvs).is_some());
        assert!(registry.get(ComponentType::Cccs).is_some());
    }

    #[test]
    fn test_registry_resistor_properties() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Resistor).unwrap();

        assert!(sheet.get("name").is_some());
        assert!(sheet.get("r").is_some());
        assert!(sheet.get("tc1").is_some());
        assert!(sheet.get("tc2").is_some());

        let r_def = sheet.get("r").unwrap();
        assert!(r_def.required);
        assert_eq!(r_def.unit, Some("Ω".to_string()));
    }

    #[test]
    fn test_registry_mosfet_properties() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Nmos).unwrap();

        assert!(sheet.get("name").is_some());
        assert!(sheet.get("model").is_some());
        assert!(sheet.get("w").is_some());
        assert!(sheet.get("l").is_some());
        assert!(sheet.get("m").is_some());
        assert!(sheet.get("nf").is_some());

        let w_def = sheet.get("w").unwrap();
        assert!(w_def.required);
        assert_eq!(w_def.prop_type, PropertyType::Expression);
    }

    // =========================================================================
    // Passive Component Commercial-Grade Parameter Tests
    // =========================================================================

    #[test]
    fn test_registry_resistor_commercial_parameters() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Resistor).unwrap();

        // Basic parameters
        assert!(sheet.get("name").is_some());
        assert!(sheet.get("r").is_some());

        // Multiplier and scale
        assert!(sheet.get("m").is_some()); // multiplier
        assert!(sheet.get("scale").is_some()); // scale factor

        // Temperature coefficients
        assert!(sheet.get("tc1").is_some());
        assert!(sheet.get("tc2").is_some());
        assert!(sheet.get("tce").is_some()); // exponential temp coefficient
        assert!(sheet.get("dtemp").is_some()); // temp rise

        // Noise
        assert!(sheet.get("noisy").is_some());

        // Verify resistance uses Expression type for design variables
        let r_def = sheet.get("r").unwrap();
        assert_eq!(r_def.prop_type, PropertyType::Expression);
    }

    #[test]
    fn test_registry_capacitor_commercial_parameters() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Capacitor).unwrap();

        // Basic
        assert!(sheet.get("name").is_some());
        assert!(sheet.get("c").is_some());

        // Multiplier and scale
        assert!(sheet.get("m").is_some());
        assert!(sheet.get("scale").is_some());

        // Voltage coefficients
        assert!(sheet.get("vc1").is_some());
        assert!(sheet.get("vc2").is_some());

        // Temperature
        assert!(sheet.get("tc1").is_some());
        assert!(sheet.get("tc2").is_some());
        assert!(sheet.get("dtemp").is_some());

        // Initial conditions
        assert!(sheet.get("ic").is_some());

        // Verify capacitance uses Expression type
        let c_def = sheet.get("c").unwrap();
        assert_eq!(c_def.prop_type, PropertyType::Expression);
    }

    #[test]
    fn test_registry_inductor_commercial_parameters() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Inductor).unwrap();

        // Basic
        assert!(sheet.get("name").is_some());
        assert!(sheet.get("l").is_some());

        // Multiplier and scale
        assert!(sheet.get("m").is_some());
        assert!(sheet.get("scale").is_some());

        // Series resistance (lossy inductor)
        assert!(sheet.get("r").is_some());

        // Temperature
        assert!(sheet.get("tc1").is_some());
        assert!(sheet.get("tc2").is_some());
        assert!(sheet.get("dtemp").is_some());

        // Initial conditions
        assert!(sheet.get("ic").is_some());

        // Mutual inductance coupling
        assert!(sheet.get("coupling_factor").is_some());
        assert!(sheet.get("coupled_to").is_some());

        // Verify inductance uses Expression type
        let l_def = sheet.get("l").unwrap();
        assert_eq!(l_def.prop_type, PropertyType::Expression);
    }

    // =========================================================================
    // Controlled Source Commercial-Grade Parameter Tests
    // =========================================================================

    #[test]
    fn test_registry_vcvs_commercial_parameters() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Vcvs).unwrap();

        // Basic
        assert!(sheet.get("name").is_some());
        assert!(sheet.get("gain").is_some());
        assert!(sheet.get("m").is_some()); // multiplier

        // Polynomial behavior
        assert!(sheet.get("poly").is_some());

        // AC parameters
        assert!(sheet.get("ac_gain").is_some());
        assert!(sheet.get("ac_phase").is_some());

        // Output limiting
        assert!(sheet.get("vmax").is_some());
        assert!(sheet.get("vmin").is_some());

        // Verify gain uses Expression type
        let gain_def = sheet.get("gain").unwrap();
        assert_eq!(gain_def.prop_type, PropertyType::Expression);
    }

    #[test]
    fn test_registry_vccs_commercial_parameters() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Vccs).unwrap();

        assert!(sheet.get("name").is_some());
        assert!(sheet.get("gm").is_some());
        assert!(sheet.get("m").is_some());

        // Polynomial
        assert!(sheet.get("poly").is_some());

        // AC
        assert!(sheet.get("ac_gm").is_some());

        // Limiting
        assert!(sheet.get("imax").is_some());
        assert!(sheet.get("imin").is_some());
    }

    #[test]
    fn test_registry_ccvs_commercial_parameters() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Ccvs).unwrap();

        assert!(sheet.get("name").is_some());
        assert!(sheet.get("rm").is_some()); // transresistance
        assert!(sheet.get("m").is_some());
        assert!(sheet.get("vref").is_some()); // sensing branch

        // Polynomial
        assert!(sheet.get("poly").is_some());

        // Limiting
        assert!(sheet.get("vmax").is_some());
        assert!(sheet.get("vmin").is_some());
    }

    #[test]
    fn test_registry_cccs_commercial_parameters() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Cccs).unwrap();

        assert!(sheet.get("name").is_some());
        assert!(sheet.get("gain").is_some());
        assert!(sheet.get("m").is_some());
        assert!(sheet.get("vref").is_some());

        // Polynomial
        assert!(sheet.get("poly").is_some());

        // Limiting
        assert!(sheet.get("imax").is_some());
        assert!(sheet.get("imin").is_some());
    }

    // =========================================================================
    // Engineering Formatting Tests
    // =========================================================================

    #[test]
    fn test_format_engineering_basic() {
        assert_eq!(format_engineering(1000.0), "1k");
        assert_eq!(format_engineering(1e6), "1M");
        assert_eq!(format_engineering(1e-3), "1m");
        assert_eq!(format_engineering(1e-6), "1u");
        assert_eq!(format_engineering(1e-9), "1n");
        assert_eq!(format_engineering(1e-12), "1p");
    }

    #[test]
    fn test_format_engineering_fractional() {
        assert_eq!(format_engineering(4700.0), "4.700k");
        assert_eq!(format_engineering(2.2e6), "2.200M");
    }

    #[test]
    fn test_format_engineering_zero() {
        assert_eq!(format_engineering(0.0), "0");
    }

    // =========================================================================
    // Source Registration Tests
    // =========================================================================

    #[test]
    fn test_registry_all_source_types_registered() {
        let registry = PropertyRegistry::new();

        // DC Sources
        assert!(
            registry.get(ComponentType::VoltageSource).is_some(),
            "VoltageSource"
        );
        assert!(
            registry.get(ComponentType::CurrentSource).is_some(),
            "CurrentSource"
        );

        // AC Sources
        assert!(
            registry.get(ComponentType::VoltageSourceAc).is_some(),
            "VoltageSourceAc"
        );
        assert!(
            registry.get(ComponentType::CurrentSourceAc).is_some(),
            "CurrentSourceAc"
        );

        // Transient Voltage Sources
        assert!(
            registry.get(ComponentType::VoltageSourcePulse).is_some(),
            "VoltageSourcePulse"
        );
        assert!(
            registry.get(ComponentType::VoltageSourceSin).is_some(),
            "VoltageSourceSin"
        );
        assert!(
            registry.get(ComponentType::VoltageSourcePwl).is_some(),
            "VoltageSourcePwl"
        );
        assert!(
            registry.get(ComponentType::VoltageSourceExp).is_some(),
            "VoltageSourceExp"
        );
        assert!(
            registry.get(ComponentType::VoltageSourceSffm).is_some(),
            "VoltageSourceSffm"
        );

        // Transient Current Sources
        assert!(
            registry.get(ComponentType::CurrentSourcePulse).is_some(),
            "CurrentSourcePulse"
        );
        assert!(
            registry.get(ComponentType::CurrentSourceSin).is_some(),
            "CurrentSourceSin"
        );
        assert!(
            registry.get(ComponentType::CurrentSourcePwl).is_some(),
            "CurrentSourcePwl"
        );
        assert!(
            registry.get(ComponentType::CurrentSourceExp).is_some(),
            "CurrentSourceExp"
        );
        assert!(
            registry.get(ComponentType::CurrentSourceNoise).is_some(),
            "CurrentSourceNoise"
        );
    }

    #[test]
    fn test_registry_vsource_pulse_properties() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSourcePulse).unwrap();

        // PULSE(V1 V2 TD TR TF PW PER)
        assert!(sheet.get("name").is_some());
        assert!(sheet.get("v1").is_some()); // Initial value
        assert!(sheet.get("v2").is_some()); // Pulsed value
        assert!(sheet.get("td").is_some()); // Delay time
        assert!(sheet.get("tr").is_some()); // Rise time
        assert!(sheet.get("tf").is_some()); // Fall time
        assert!(sheet.get("pw").is_some()); // Pulse width
        assert!(sheet.get("per").is_some()); // Period

        let v1 = sheet.get("v1").unwrap();
        assert!(v1.required);
        assert_eq!(v1.unit, Some("V".to_string()));
    }

    #[test]
    fn test_registry_vsource_sin_properties() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSourceSin).unwrap();

        // SIN(VO VA FREQ TD THETA PHASE)
        assert!(sheet.get("vo").is_some()); // DC offset
        assert!(sheet.get("va").is_some()); // Amplitude
        assert!(sheet.get("freq").is_some()); // Frequency
        assert!(sheet.get("td").is_some()); // Delay
        assert!(sheet.get("theta").is_some()); // Damping
        assert!(sheet.get("phase").is_some()); // Phase

        let freq = sheet.get("freq").unwrap();
        assert!(freq.required);
        assert_eq!(freq.unit, Some("Hz".to_string()));
    }

    #[test]
    fn test_registry_vsource_pwl_properties() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSourcePwl).unwrap();

        assert!(sheet.get("pwl_data").is_some());
        assert!(sheet.get("td").is_some());
        assert!(sheet.get("repeat").is_some());

        let pwl_data = sheet.get("pwl_data").unwrap();
        assert!(pwl_data.required);
        assert_eq!(pwl_data.prop_type, PropertyType::String);
    }

    #[test]
    fn test_registry_vsource_exp_properties() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSourceExp).unwrap();

        // EXP(V1 V2 TD1 TAU1 TD2 TAU2)
        assert!(sheet.get("v1").is_some());
        assert!(sheet.get("v2").is_some());
        assert!(sheet.get("td1").is_some());
        assert!(sheet.get("tau1").is_some());
        assert!(sheet.get("td2").is_some());
        assert!(sheet.get("tau2").is_some());
    }

    #[test]
    fn test_registry_vsource_sffm_properties() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSourceSffm).unwrap();

        // SFFM(VO VA FC MDI FS)
        assert!(sheet.get("vo").is_some()); // DC offset
        assert!(sheet.get("va").is_some()); // Amplitude
        assert!(sheet.get("fc").is_some()); // Carrier frequency
        assert!(sheet.get("mdi").is_some()); // Modulation index
        assert!(sheet.get("fs").is_some()); // Signal frequency

        let fc = sheet.get("fc").unwrap();
        assert!(fc.required);
        assert_eq!(fc.unit, Some("Hz".to_string()));
    }

    #[test]
    fn test_registry_isource_noise_properties() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::CurrentSourceNoise).unwrap();

        assert!(sheet.get("dc").is_some());
        assert!(sheet.get("noise_type").is_some());
        assert!(sheet.get("noiseval").is_some());
        assert!(sheet.get("kf").is_some()); // Flicker coefficient
        assert!(sheet.get("af").is_some()); // Flicker exponent

        // Check noise_type is enum
        let noise_type = sheet.get("noise_type").unwrap();
        assert_eq!(noise_type.prop_type, PropertyType::Enum);
    }

    // =========================================================================
    // Semiconductor Parameter Category Tests
    // =========================================================================

    #[test]
    fn test_registry_diode_parameter_categories() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Diode).unwrap();

        // Instance category
        assert!(sheet.get("name").is_some());

        // Model category
        assert!(sheet.get("model").is_some());

        // Geometry category
        assert!(sheet.get("area").is_some());
        assert!(sheet.get("m").is_some()); // multiplier
        assert!(sheet.get("pj").is_some()); // perimeter

        // Initial conditions
        assert!(sheet.get("ic").is_some());
        assert!(sheet.get("off").is_some());

        // Temperature
        assert!(sheet.get("dtemp").is_some());
    }

    #[test]
    fn test_registry_mosfet_parameter_categories() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Nmos).unwrap();

        // Geometry category
        assert!(sheet.get("w").is_some());
        assert!(sheet.get("l").is_some());
        assert!(sheet.get("nf").is_some()); // fingers
        assert!(sheet.get("m").is_some()); // multiplier

        // Parasitics
        assert!(sheet.get("ad").is_some()); // drain area
        assert!(sheet.get("as").is_some()); // source area
        assert!(sheet.get("pd").is_some()); // drain perimeter
        assert!(sheet.get("ps").is_some()); // source perimeter
        assert!(sheet.get("nrd").is_some()); // drain squares
        assert!(sheet.get("nrs").is_some()); // source squares

        // Stress effects (for advanced nodes)
        assert!(sheet.get("sa").is_some());
        assert!(sheet.get("sb").is_some());
        assert!(sheet.get("sd").is_some());

        // Initial conditions
        assert!(sheet.get("ic_vds").is_some());
        assert!(sheet.get("ic_vgs").is_some());
        assert!(sheet.get("ic_vbs").is_some());
    }

    #[test]
    fn test_registry_bjt_parameter_categories() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::NpnBjt).unwrap();

        // Geometry
        assert!(sheet.get("area").is_some());
        assert!(sheet.get("m").is_some());
        assert!(sheet.get("areab").is_some());
        assert!(sheet.get("areac").is_some());

        // Initial conditions
        assert!(sheet.get("ic_vbe").is_some());
        assert!(sheet.get("ic_vce").is_some());

        // Operating region
        assert!(sheet.get("off").is_some());

        // Temperature
        assert!(sheet.get("dtemp").is_some());
    }

    #[test]
    fn test_registry_jfet_parameter_categories() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::Njfet).unwrap();

        // Instance and Geometry
        assert!(sheet.get("name").is_some());
        assert!(sheet.get("area").is_some());
        assert!(sheet.get("m").is_some());

        // Initial conditions
        assert!(sheet.get("ic_vds").is_some());
        assert!(sheet.get("ic_vgs").is_some());
        assert!(sheet.get("off").is_some());

        // Temperature
        assert!(sheet.get("dtemp").is_some());
    }

    #[test]
    fn test_source_properties_use_expression_type() {
        // Verify that value properties use Expression type for design variable support
        let registry = PropertyRegistry::new();

        // DC Source
        let vsource = registry.get(ComponentType::VoltageSource).unwrap();
        assert_eq!(
            vsource.get("dc").unwrap().prop_type,
            PropertyType::Expression
        );

        // Sin Source frequency
        let vsin = registry.get(ComponentType::VoltageSourceSin).unwrap();
        assert_eq!(
            vsin.get("freq").unwrap().prop_type,
            PropertyType::Expression
        );

        // Pulse timing
        let vpulse = registry.get(ComponentType::VoltageSourcePulse).unwrap();
        assert_eq!(
            vpulse.get("pw").unwrap().prop_type,
            PropertyType::Expression
        );
        assert_eq!(
            vpulse.get("per").unwrap().prop_type,
            PropertyType::Expression
        );
    }

    // =========================================================================
    // Spectre-Parity Source Parameter Tests
    // =========================================================================
    // These tests ensure commercial-grade parity with Cadence Spectre source
    // parameter sets including DC, AC, Advanced AC (XF/PAC), Parasitics, and Noise.

    #[test]
    fn test_spectre_parity_dc_voltage_source_categories() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSource).unwrap();

        // =====================================================================
        // Instance Category
        // =====================================================================
        let name = sheet
            .get("name")
            .expect("DC vsource must have 'name' property");
        assert!(name.required, "Instance name must be required");
        assert_eq!(name.category, "Instance");
        assert_eq!(name.prop_type, PropertyType::String);

        // =====================================================================
        // DC Category
        // =====================================================================
        let dc = sheet.get("dc").expect("DC vsource must have 'dc' property");
        assert!(dc.required, "DC voltage must be required");
        assert_eq!(dc.category, "DC");
        assert_eq!(dc.unit, Some("V".to_string()));
        assert_eq!(dc.prop_type, PropertyType::Expression);

        // =====================================================================
        // AC Category - Small-signal analysis parameters
        // =====================================================================
        let ac = sheet
            .get("ac")
            .expect("DC vsource must have 'ac' property for AC analysis");
        assert_eq!(ac.category, "AC");
        assert_eq!(ac.unit, Some("V".to_string()));

        let acphase = sheet
            .get("acphase")
            .expect("DC vsource must have 'acphase' property");
        assert_eq!(acphase.category, "AC");
        assert_eq!(acphase.unit, Some("°".to_string()));
        assert!(
            acphase.min_value.is_some() && acphase.max_value.is_some(),
            "AC phase should have range validation"
        );

        // =====================================================================
        // Advanced AC Category - XF/PAC analysis (Spectre-specific)
        // =====================================================================
        let xfmag = sheet
            .get("xfmag")
            .expect("DC vsource must have 'xfmag' for XF analysis");
        assert_eq!(xfmag.category, "Advanced AC");
        assert_eq!(
            xfmag.display_mode,
            DisplayMode::Advanced,
            "XF magnitude should be marked as advanced"
        );

        let pacmag = sheet
            .get("pacmag")
            .expect("DC vsource must have 'pacmag' for PAC analysis");
        assert_eq!(pacmag.category, "Advanced AC");
        assert_eq!(pacmag.display_mode, DisplayMode::Advanced);

        let pacdbm = sheet
            .get("pacdbm")
            .expect("DC vsource must have 'pacdbm' (dBm alternative)");
        assert_eq!(pacdbm.category, "Advanced AC");
        assert_eq!(pacdbm.unit, Some("dBm".to_string()));

        let pacphase = sheet
            .get("pacphase")
            .expect("DC vsource must have 'pacphase'");
        assert_eq!(pacphase.category, "Advanced AC");
        assert_eq!(pacphase.unit, Some("°".to_string()));

        // =====================================================================
        // Parasitics Category - Non-ideal source characteristics
        // =====================================================================
        let rs = sheet
            .get("rs")
            .expect("DC vsource must have 'rs' (series resistance)");
        assert_eq!(rs.category, "Parasitics");
        assert_eq!(rs.unit, Some("Ω".to_string()));

        let rp = sheet
            .get("rp")
            .expect("DC vsource must have 'rp' (parallel resistance)");
        assert_eq!(rp.category, "Parasitics");

        let cpar = sheet
            .get("cpar")
            .expect("DC vsource must have 'cpar' (parasitic capacitance)");
        assert_eq!(cpar.category, "Parasitics");
        assert_eq!(cpar.unit, Some("F".to_string()));

        // =====================================================================
        // Noise Category
        // =====================================================================
        let isnoisy = sheet
            .get("isnoisy")
            .expect("DC vsource must have 'isnoisy'");
        assert_eq!(isnoisy.category, "Noise");
        assert_eq!(isnoisy.prop_type, PropertyType::Boolean);
    }

    #[test]
    fn test_spectre_parity_ac_voltage_source_defaults() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSourceAc).unwrap();

        // AC source should default to 1V AC magnitude (primary purpose)
        let ac = sheet.get("ac").unwrap();
        match &ac.default_value {
            PropertyValue::Number { value, .. } => {
                assert_eq!(*value, 1.0, "AC source should default to 1V")
            }
            other => panic!("AC magnitude should default to 1.0, got {:?}", other),
        }

        // DC offset should default to 0V
        let dc = sheet.get("dc").unwrap();
        match &dc.default_value {
            PropertyValue::Number { value, .. } => {
                assert_eq!(*value, 0.0, "DC offset should default to 0V")
            }
            other => panic!("DC offset should default to 0.0, got {:?}", other),
        }

        // AC source should have all the same categories as DC source
        assert!(
            sheet.get("xfmag").is_some(),
            "AC source must have Advanced AC params"
        );
        assert!(sheet.get("pacmag").is_some());
        assert!(
            sheet.get("rs").is_some(),
            "AC source must have Parasitics params"
        );
        assert!(
            sheet.get("isnoisy").is_some(),
            "AC source must have Noise params"
        );
    }

    #[test]
    fn test_spectre_parity_dc_current_source_categories() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::CurrentSource).unwrap();

        // =====================================================================
        // Instance Category
        // =====================================================================
        let name = sheet.get("name").expect("DC isource must have 'name'");
        assert_eq!(name.category, "Instance");
        match &name.default_value {
            PropertyValue::String(s) => {
                assert_eq!(s, "I1", "Current source should default to I1")
            }
            other => panic!("Name should default to 'I1', got {:?}", other),
        }

        // =====================================================================
        // DC Category - Current unit (A instead of V)
        // =====================================================================
        let dc = sheet.get("dc").expect("DC isource must have 'dc'");
        assert_eq!(
            dc.unit,
            Some("A".to_string()),
            "Current source uses Amperes"
        );
        assert!(dc.required);

        // =====================================================================
        // AC Category - Current unit
        // =====================================================================
        let ac = sheet.get("ac").expect("DC isource must have 'ac'");
        assert_eq!(ac.unit, Some("A".to_string()));

        let acphase = sheet
            .get("acphase")
            .expect("DC isource must have 'acphase'");
        assert_eq!(acphase.unit, Some("°".to_string()));

        // =====================================================================
        // Advanced AC Category
        // =====================================================================
        let xfmag = sheet.get("xfmag").expect("DC isource must have 'xfmag'");
        assert_eq!(xfmag.unit, Some("A".to_string()));

        let pacmag = sheet.get("pacmag").expect("DC isource must have 'pacmag'");
        assert_eq!(pacmag.unit, Some("A".to_string()));

        // =====================================================================
        // Parasitics Category - Current source only has parallel elements
        // =====================================================================
        assert!(
            sheet.get("rp").is_some(),
            "Current source must have parallel resistance"
        );
        assert!(
            sheet.get("cpar").is_some(),
            "Current source must have parasitic capacitance"
        );
        // Note: Current sources don't have series resistance (rs) - that would change topology

        // =====================================================================
        // Noise Category
        // =====================================================================
        assert!(sheet.get("isnoisy").is_some());
    }

    #[test]
    fn test_spectre_parity_ac_current_source_defaults() {
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::CurrentSourceAc).unwrap();

        // AC current source should default to 1A
        let ac = sheet.get("ac").unwrap();
        match &ac.default_value {
            PropertyValue::Number { value, .. } => {
                assert_eq!(*value, 1.0, "AC current source should default to 1A")
            }
            other => panic!("AC magnitude should default to 1.0, got {:?}", other),
        }

        // Should have all Spectre-parity categories
        assert!(sheet.get("xfmag").is_some());
        assert!(sheet.get("pacmag").is_some());
        assert!(sheet.get("pacdbm").is_some());
        assert!(sheet.get("pacphase").is_some());
        assert!(sheet.get("rp").is_some());
        assert!(sheet.get("cpar").is_some());
        assert!(sheet.get("isnoisy").is_some());
    }

    #[test]
    fn test_spectre_parity_parameter_ordering() {
        // Verify ordering follows Spectre convention:
        // Instance (0-9) < DC (10-19) < AC (20-29) < Advanced AC (30-39) < Parasitics (40-49) < Noise (50+)
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSource).unwrap();

        let name_order = sheet.get("name").unwrap().display_order;
        let dc_order = sheet.get("dc").unwrap().display_order;
        let ac_order = sheet.get("ac").unwrap().display_order;
        let xfmag_order = sheet.get("xfmag").unwrap().display_order;
        let rs_order = sheet.get("rs").unwrap().display_order;
        let isnoisy_order = sheet.get("isnoisy").unwrap().display_order;

        assert!(
            name_order < dc_order,
            "Instance params should come before DC"
        );
        assert!(dc_order < ac_order, "DC params should come before AC");
        assert!(
            ac_order < xfmag_order,
            "AC params should come before Advanced AC"
        );
        assert!(
            xfmag_order < rs_order,
            "Advanced AC should come before Parasitics"
        );
        assert!(
            rs_order < isnoisy_order,
            "Parasitics should come before Noise"
        );
    }

    #[test]
    fn test_spectre_parity_category_grouping() {
        // Verify by_category returns correct groupings for tab display
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSource).unwrap();

        let categories = sheet.by_category();

        // Should have 6 categories for DC voltage source
        assert!(categories.iter().any(|(cat, _)| cat == "Instance"));
        assert!(categories.iter().any(|(cat, _)| cat == "DC"));
        assert!(categories.iter().any(|(cat, _)| cat == "AC"));
        assert!(categories.iter().any(|(cat, _)| cat == "Advanced AC"));
        assert!(categories.iter().any(|(cat, _)| cat == "Parasitics"));
        assert!(categories.iter().any(|(cat, _)| cat == "Noise"));

        // Verify each category has expected properties
        for (cat, props) in &categories {
            match cat.as_str() {
                "Instance" => assert!(props.iter().any(|p| p.name == "name")),
                "DC" => assert!(props.iter().any(|p| p.name == "dc")),
                "AC" => {
                    assert!(props.iter().any(|p| p.name == "ac"));
                    assert!(props.iter().any(|p| p.name == "acphase"));
                }
                "Advanced AC" => {
                    assert!(props.iter().any(|p| p.name == "xfmag"));
                    assert!(props.iter().any(|p| p.name == "pacmag"));
                    assert!(props.iter().any(|p| p.name == "pacdbm"));
                    assert!(props.iter().any(|p| p.name == "pacphase"));
                }
                "Parasitics" => {
                    assert!(props.iter().any(|p| p.name == "rs"));
                    assert!(props.iter().any(|p| p.name == "rp"));
                    assert!(props.iter().any(|p| p.name == "cpar"));
                }
                "Noise" => assert!(props.iter().any(|p| p.name == "isnoisy")),
                _ => {} // Other categories are OK
            }
        }
    }

    #[test]
    fn test_spectre_parity_phase_range_validation() {
        // All phase parameters should have ±360° range
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSource).unwrap();

        let phase_params = ["acphase", "pacphase"];
        for param_name in phase_params {
            let param = sheet
                .get(param_name)
                .expect(&format!("Must have {}", param_name));
            let min = param
                .min_value
                .expect(&format!("{} must have min_value", param_name));
            let max = param
                .max_value
                .expect(&format!("{} must have max_value", param_name));
            assert_eq!(min, -360.0, "{} min should be -360°", param_name);
            assert_eq!(max, 360.0, "{} max should be 360°", param_name);
        }
    }

    #[test]
    fn test_spectre_parity_non_negative_parasitics() {
        // Parasitic values should have non-negative ranges where appropriate
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSource).unwrap();

        // Series resistance should be >= 0
        let rs = sheet.get("rs").unwrap();
        if let Some(min) = rs.min_value {
            assert!(min >= 0.0, "Series resistance cannot be negative");
        }

        // Parasitic capacitance should be >= 0
        let cpar = sheet.get("cpar").unwrap();
        if let Some(min) = cpar.min_value {
            assert!(min >= 0.0, "Parasitic capacitance cannot be negative");
        }
    }

    #[test]
    fn test_spectre_parity_advanced_params_marked_advanced() {
        // Advanced AC parameters should be marked as advanced (hidden by default in simple UIs)
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSource).unwrap();

        let advanced_params = ["xfmag", "pacmag", "pacdbm", "pacphase"];
        for param_name in advanced_params {
            let param = sheet
                .get(param_name)
                .expect(&format!("Must have {}", param_name));
            assert_eq!(
                param.display_mode,
                DisplayMode::Advanced,
                "{} should be marked as advanced for cleaner UI",
                param_name
            );
        }
    }

    #[test]
    fn test_spectre_parity_property_count_per_source() {
        // Verify enhanced sources have expected number of properties
        let registry = PropertyRegistry::new();

        // DC Voltage Source: 12 properties
        // name, dc, ac, acphase, xfmag, pacmag, pacdbm, pacphase, rs, rp, cpar, isnoisy
        let vsource = registry.get(ComponentType::VoltageSource).unwrap();
        assert!(
            vsource.len() >= 12,
            "DC voltage source should have at least 12 Spectre-parity properties, got {}",
            vsource.len()
        );

        // DC Current Source: 11 properties (no rs - current sources don't have series resistance)
        // name, dc, ac, acphase, xfmag, pacmag, pacdbm, pacphase, rp, cpar, isnoisy
        let isource = registry.get(ComponentType::CurrentSource).unwrap();
        assert!(
            isource.len() >= 11,
            "DC current source should have at least 11 Spectre-parity properties, got {}",
            isource.len()
        );
    }

    #[test]
    fn test_spectre_parity_dbm_defaults() {
        // PAC dBm should default to -inf (disabled) as per Spectre convention
        let registry = PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSource).unwrap();

        let pacdbm = sheet.get("pacdbm").unwrap();
        match &pacdbm.default_value {
            PropertyValue::Number { value, .. } => {
                assert!(
                    value.is_infinite() && value.is_sign_negative(),
                    "pacdbm should default to -inf (disabled), got {}",
                    value
                );
            }
            other => panic!("pacdbm should default to -inf, got {:?}", other),
        }
    }

    #[test]
    fn test_spectre_parity_parallel_resistance_defaults() {
        // Parallel resistance should default to infinity (ideal source)
        let registry = PropertyRegistry::new();

        let vsource = registry.get(ComponentType::VoltageSource).unwrap();
        let rp = vsource.get("rp").unwrap();
        match &rp.default_value {
            PropertyValue::Number { value, .. } => {
                assert!(
                    value.is_infinite() && value.is_sign_positive(),
                    "Parallel resistance should default to +inf (ideal), got {}",
                    value
                );
            }
            other => panic!("rp should default to +inf, got {:?}", other),
        }

        let isource = registry.get(ComponentType::CurrentSource).unwrap();
        let rp = isource.get("rp").unwrap();
        match &rp.default_value {
            PropertyValue::Number { value, .. } => {
                assert!(value.is_infinite() && value.is_sign_positive());
            }
            other => panic!("Current source rp should default to +inf, got {:?}", other),
        }
    }

    #[test]
    fn test_spectre_parity_noisy_defaults_to_true() {
        // isnoisy should default to true (sources contribute noise by default)
        let registry = PropertyRegistry::new();

        for component_type in [
            ComponentType::VoltageSource,
            ComponentType::VoltageSourceAc,
            ComponentType::CurrentSource,
            ComponentType::CurrentSourceAc,
        ] {
            let sheet = registry.get(component_type).unwrap();
            let isnoisy = sheet
                .get("isnoisy")
                .expect(&format!("{:?} must have isnoisy", component_type));
            match &isnoisy.default_value {
                PropertyValue::Boolean(b) => {
                    assert!(*b, "{:?} isnoisy should default to true", component_type);
                }
                other => panic!(
                    "{:?} isnoisy should be boolean true, got {:?}",
                    component_type, other
                ),
            }
        }
    }

    // =========================================================================
    // Semiconductor Spectre-Parity Tests
    // =========================================================================

    #[test]
    fn test_spectre_parity_mosfet_categories() {
        let registry = PropertyRegistry::new();
        let nmos = registry.get(ComponentType::Nmos).unwrap();
        let pmos = registry.get(ComponentType::Pmos).unwrap();

        // Both NMOS and PMOS should have same category structure
        for (name, sheet) in [("NMOS", nmos), ("PMOS", pmos)] {
            // Instance category
            assert!(
                sheet.get("name").is_some(),
                "{} must have instance name",
                name
            );

            // Model category
            let model = sheet
                .get("model")
                .expect(&format!("{} must have model", name));
            assert_eq!(model.category, "Model");

            // Geometry category - essential for MOSFET sizing
            let w = sheet.get("w").expect(&format!("{} must have width", name));
            assert_eq!(w.category, "Geometry");
            assert!(w.required, "Width should be required for MOSFETs");

            let l = sheet.get("l").expect(&format!("{} must have length", name));
            assert_eq!(l.category, "Geometry");
            assert!(l.required, "Length should be required for MOSFETs");

            let nf = sheet
                .get("nf")
                .expect(&format!("{} must have nf (fingers)", name));
            assert_eq!(nf.category, "Geometry");

            let m = sheet
                .get("m")
                .expect(&format!("{} must have multiplier", name));
            assert_eq!(m.category, "Geometry");

            // Parasitics category - S/D areas and perimeters
            let as_param = sheet
                .get("as")
                .expect(&format!("{} must have source area", name));
            assert_eq!(as_param.category, "Parasitics");

            let ad = sheet
                .get("ad")
                .expect(&format!("{} must have drain area", name));
            assert_eq!(ad.category, "Parasitics");

            let ps = sheet
                .get("ps")
                .expect(&format!("{} must have source perimeter", name));
            assert_eq!(ps.category, "Parasitics");

            let pd = sheet
                .get("pd")
                .expect(&format!("{} must have drain perimeter", name));
            assert_eq!(pd.category, "Parasitics");

            // Stress category - STI effects for advanced nodes
            assert!(
                sheet.get("sa").is_some(),
                "{} must have SA for STI stress",
                name
            );
            assert!(sheet.get("sb").is_some(), "{} must have SB", name);
            assert!(sheet.get("sd").is_some(), "{} must have SD", name);

            // Temperature category
            let dtemp = sheet
                .get("dtemp")
                .expect(&format!("{} must have dtemp", name));
            assert_eq!(dtemp.category, "Temperature");

            // Initial Conditions category
            assert!(sheet.get("off").is_some(), "{} must have off flag", name);
            assert!(sheet.get("ic_vgs").is_some(), "{} must have IC VGS", name);
            assert!(sheet.get("ic_vds").is_some(), "{} must have IC VDS", name);
            assert!(sheet.get("ic_vbs").is_some(), "{} must have IC VBS", name);
        }
    }

    #[test]
    fn test_spectre_parity_mosfet_geometry_constraints() {
        let registry = PropertyRegistry::new();
        let nmos = registry.get(ComponentType::Nmos).unwrap();

        // Width should have reasonable range (1nm to 1mm)
        let w = nmos.get("w").unwrap();
        assert!(w.min_value.is_some(), "Width should have minimum");
        assert!(w.max_value.is_some(), "Width should have maximum");
        assert!(
            w.min_value.unwrap() >= 1e-9,
            "Min width should be at least 1nm"
        );
        assert!(
            w.max_value.unwrap() <= 1e-3,
            "Max width should be at most 1mm"
        );

        // Length should have reasonable range
        let l = nmos.get("l").unwrap();
        assert!(l.min_value.is_some() && l.max_value.is_some());

        // Multiplier should be >= 1
        let m = nmos.get("m").unwrap();
        assert!(m.min_value.unwrap() >= 1.0, "Multiplier must be at least 1");

        // Fingers should be >= 1
        let nf = nmos.get("nf").unwrap();
        assert!(nf.min_value.unwrap() >= 1.0, "Fingers must be at least 1");
    }

    #[test]
    fn test_spectre_parity_diode_categories() {
        let registry = PropertyRegistry::new();
        let diode = registry.get(ComponentType::Diode).unwrap();

        // Instance
        assert!(diode.get("name").is_some());

        // Model
        let model = diode.get("model").expect("Diode must have model");
        assert_eq!(model.category, "Model");

        // Geometry
        let area = diode.get("area").expect("Diode must have area");
        assert_eq!(area.category, "Geometry");
        let pj = diode.get("pj").expect("Diode must have perimeter");
        assert_eq!(pj.category, "Geometry");
        let m = diode.get("m").expect("Diode must have multiplier");
        assert_eq!(m.category, "Geometry");

        // Temperature
        let dtemp = diode.get("dtemp").expect("Diode must have dtemp");
        assert_eq!(dtemp.category, "Temperature");

        // Initial Conditions
        let off = diode.get("off").expect("Diode must have off flag");
        assert_eq!(off.category, "Initial Conditions");
        let ic = diode.get("ic").expect("Diode must have initial voltage");
        assert_eq!(ic.category, "Initial Conditions");
    }

    #[test]
    fn test_spectre_parity_bjt_categories() {
        let registry = PropertyRegistry::new();
        let npn = registry.get(ComponentType::NpnBjt).unwrap();
        let pnp = registry.get(ComponentType::PnpBjt).unwrap();

        for (name, sheet) in [("NPN", npn), ("PNP", pnp)] {
            // Instance
            assert!(
                sheet.get("name").is_some(),
                "{} must have instance name",
                name
            );

            // Model
            let model = sheet
                .get("model")
                .expect(&format!("{} must have model", name));
            assert_eq!(model.category, "Model");

            // Geometry - BJT has multiple area factors
            let area = sheet
                .get("area")
                .expect(&format!("{} must have area", name));
            assert_eq!(area.category, "Geometry");

            // Spectre has separate area factors for E, B, C
            let areab = sheet
                .get("areab")
                .expect(&format!("{} must have base area", name));
            assert_eq!(areab.category, "Geometry");

            let areac = sheet
                .get("areac")
                .expect(&format!("{} must have collector area", name));
            assert_eq!(areac.category, "Geometry");

            let m = sheet
                .get("m")
                .expect(&format!("{} must have multiplier", name));
            assert_eq!(m.category, "Geometry");

            // Temperature
            let dtemp = sheet
                .get("dtemp")
                .expect(&format!("{} must have dtemp", name));
            assert_eq!(dtemp.category, "Temperature");

            // Initial Conditions
            let off = sheet
                .get("off")
                .expect(&format!("{} must have off flag", name));
            assert_eq!(off.category, "Initial Conditions");

            // Region hint (Spectre feature for convergence)
            let region = sheet
                .get("region")
                .expect(&format!("{} must have region hint", name));
            assert_eq!(region.category, "Initial Conditions");
        }
    }

    #[test]
    fn test_spectre_parity_jfet_categories() {
        let registry = PropertyRegistry::new();
        let njfet = registry.get(ComponentType::Njfet).unwrap();
        let pjfet = registry.get(ComponentType::Pjfet).unwrap();

        for (name, sheet) in [("NJFET", njfet), ("PJFET", pjfet)] {
            // Instance
            assert!(
                sheet.get("name").is_some(),
                "{} must have instance name",
                name
            );

            // Model
            let model = sheet
                .get("model")
                .expect(&format!("{} must have model", name));
            assert_eq!(model.category, "Model");

            // Geometry
            let area = sheet
                .get("area")
                .expect(&format!("{} must have area", name));
            assert_eq!(area.category, "Geometry");

            let m = sheet
                .get("m")
                .expect(&format!("{} must have multiplier", name));
            assert_eq!(m.category, "Geometry");

            // Temperature
            let dtemp = sheet
                .get("dtemp")
                .expect(&format!("{} must have dtemp", name));
            assert_eq!(dtemp.category, "Temperature");

            // Initial Conditions
            let off = sheet
                .get("off")
                .expect(&format!("{} must have off flag", name));
            assert_eq!(off.category, "Initial Conditions");

            // JFET should have IC for VGS and VDS
            assert!(sheet.get("ic_vgs").is_some(), "{} must have IC VGS", name);
            assert!(sheet.get("ic_vds").is_some(), "{} must have IC VDS", name);
        }
    }

    // =========================================================================
    // Controlled Source Spectre-Parity Tests
    // =========================================================================

    #[test]
    fn test_spectre_parity_vcvs_categories() {
        let registry = PropertyRegistry::new();
        let vcvs = registry.get(ComponentType::Vcvs).unwrap();

        // Instance
        let name = vcvs.get("name").expect("VCVS must have name");
        assert_eq!(name.category, "Instance");

        // Electrical - main gain parameter
        let gain = vcvs.get("gain").expect("VCVS must have gain");
        assert_eq!(gain.category, "Electrical");
        assert!(gain.required, "Gain is essential for VCVS");

        let m = vcvs.get("m").expect("VCVS must have multiplier");
        assert_eq!(m.category, "Electrical");

        // Polynomial (for nonlinear behavior - Spectre feature)
        let poly = vcvs
            .get("poly")
            .expect("VCVS must have polynomial coefficients");
        assert_eq!(poly.category, "Polynomial");

        // AC parameters
        let ac_gain = vcvs.get("ac_gain").expect("VCVS must have AC gain");
        assert_eq!(ac_gain.category, "AC");

        // Limits (saturation - Spectre feature)
        let vmax = vcvs.get("vmax").expect("VCVS must have max output limit");
        assert_eq!(vmax.category, "Limits");
        let vmin = vcvs.get("vmin").expect("VCVS must have min output limit");
        assert_eq!(vmin.category, "Limits");
    }

    #[test]
    fn test_spectre_parity_vccs_categories() {
        let registry = PropertyRegistry::new();
        let vccs = registry.get(ComponentType::Vccs).unwrap();

        // Instance
        assert!(vccs.get("name").is_some());

        // Electrical - transconductance (output current / input voltage)
        let gm = vccs.get("gm").expect("VCCS must have transconductance");
        assert_eq!(gm.category, "Electrical");
        assert!(gm.required, "Transconductance is essential for VCCS");

        // Output limits (current limits)
        let imax = vccs.get("imax").expect("VCCS must have max current limit");
        assert_eq!(imax.category, "Limits");
    }

    #[test]
    fn test_spectre_parity_ccvs_categories() {
        let registry = PropertyRegistry::new();
        let ccvs = registry.get(ComponentType::Ccvs).unwrap();

        // Instance
        assert!(ccvs.get("name").is_some());

        // Electrical - transresistance (output voltage / input current)
        let rm = ccvs.get("rm").expect("CCVS must have transresistance");
        assert_eq!(rm.category, "Electrical");
        assert!(rm.required, "Transresistance is essential for CCVS");

        // Sensing reference (voltage source for current sensing)
        let vref = ccvs
            .get("vref")
            .expect("CCVS must have sensing branch reference");
        assert_eq!(vref.category, "Electrical");

        // Output limits
        let vmax = ccvs.get("vmax").expect("CCVS must have max voltage limit");
        assert_eq!(vmax.category, "Limits");
    }

    #[test]
    fn test_spectre_parity_cccs_categories() {
        let registry = PropertyRegistry::new();
        let cccs = registry.get(ComponentType::Cccs).unwrap();

        // Instance
        assert!(cccs.get("name").is_some());

        // Electrical - current gain
        let gain = cccs.get("gain").expect("CCCS must have current gain");
        assert_eq!(gain.category, "Electrical");
        assert!(gain.required, "Current gain is essential for CCCS");

        // Sensing reference
        let vref = cccs
            .get("vref")
            .expect("CCCS must have sensing branch reference");
        assert_eq!(vref.category, "Electrical");

        // Polynomial (for nonlinear behavior)
        let poly = cccs.get("poly").expect("CCCS must have polynomial");
        assert_eq!(poly.category, "Polynomial");

        // Output limits
        let imax = cccs.get("imax").expect("CCCS must have max current limit");
        assert_eq!(imax.category, "Limits");
    }

    #[test]
    fn test_spectre_parity_controlled_source_gain_defaults() {
        let registry = PropertyRegistry::new();

        // All controlled sources should default to unity gain
        let vcvs = registry.get(ComponentType::Vcvs).unwrap();
        let gain = vcvs.get("gain").unwrap();
        match &gain.default_value {
            PropertyValue::Number { value, .. } => {
                assert_eq!(*value, 1.0, "VCVS gain should default to 1.0");
            }
            other => panic!("VCVS gain should be number 1.0, got {:?}", other),
        }

        let cccs = registry.get(ComponentType::Cccs).unwrap();
        let gain = cccs.get("gain").unwrap();
        match &gain.default_value {
            PropertyValue::Number { value, .. } => {
                assert_eq!(*value, 1.0, "CCCS gain should default to 1.0");
            }
            other => panic!("CCCS gain should be number 1.0, got {:?}", other),
        }
    }

    #[test]
    fn test_spectre_parity_transient_sources_have_all_categories() {
        let registry = PropertyRegistry::new();

        // All transient sources should have AC, Parasitics, and Noise categories
        let transient_sources = [
            ComponentType::VoltageSourcePulse,
            ComponentType::VoltageSourceSin,
            ComponentType::VoltageSourcePwl,
            ComponentType::VoltageSourceExp,
            ComponentType::VoltageSourceSffm,
            ComponentType::CurrentSourcePulse,
            ComponentType::CurrentSourceSin,
            ComponentType::CurrentSourcePwl,
            ComponentType::CurrentSourceExp,
            ComponentType::CurrentSourceNoise,
        ];

        for comp_type in transient_sources {
            let sheet = registry
                .get(comp_type)
                .expect(&format!("{:?} must be registered", comp_type));

            // AC parameters
            assert!(
                sheet.get("ac").is_some(),
                "{:?} must have AC magnitude",
                comp_type
            );
            assert!(
                sheet.get("acphase").is_some(),
                "{:?} must have AC phase",
                comp_type
            );

            // Advanced AC (Spectre XF/PAC)
            assert!(
                sheet.get("xfmag").is_some(),
                "{:?} must have XF magnitude",
                comp_type
            );
            assert!(
                sheet.get("pacmag").is_some(),
                "{:?} must have PAC magnitude",
                comp_type
            );

            // Parasitics - voltage sources have rs, current sources don't
            if comp_type.spice_prefix() == "V" {
                assert!(
                    sheet.get("rs").is_some(),
                    "{:?} (voltage) must have series resistance",
                    comp_type
                );
            }
            assert!(
                sheet.get("rp").is_some(),
                "{:?} must have parallel resistance",
                comp_type
            );
            assert!(
                sheet.get("cpar").is_some(),
                "{:?} must have parasitic capacitance",
                comp_type
            );

            // Noise
            assert!(
                sheet.get("isnoisy").is_some(),
                "{:?} must have noise flag",
                comp_type
            );
        }
    }

    #[test]
    fn test_spectre_parity_all_semiconductors_have_dtemp() {
        // All semiconductor devices must have instance temperature (dtemp)
        let registry = PropertyRegistry::new();

        let semiconductors = [
            ComponentType::Diode,
            ComponentType::Nmos,
            ComponentType::Pmos,
            ComponentType::NpnBjt,
            ComponentType::PnpBjt,
            ComponentType::Njfet,
            ComponentType::Pjfet,
        ];

        for comp_type in semiconductors {
            let sheet = registry
                .get(comp_type)
                .expect(&format!("{:?} must be registered", comp_type));
            let dtemp = sheet.get("dtemp").expect(&format!(
                "{:?} must have dtemp for temperature analysis",
                comp_type
            ));
            assert_eq!(
                dtemp.unit,
                Some("°C".to_string()),
                "{:?} dtemp should have °C unit",
                comp_type
            );
        }
    }

    #[test]
    fn test_spectre_parity_all_semiconductors_have_off_flag() {
        // All semiconductor devices must have 'off' flag for DC analysis convergence
        let registry = PropertyRegistry::new();

        let semiconductors = [
            ComponentType::Diode,
            ComponentType::Nmos,
            ComponentType::Pmos,
            ComponentType::NpnBjt,
            ComponentType::PnpBjt,
            ComponentType::Njfet,
            ComponentType::Pjfet,
        ];

        for comp_type in semiconductors {
            let sheet = registry
                .get(comp_type)
                .expect(&format!("{:?} must be registered", comp_type));
            let off = sheet.get("off").expect(&format!(
                "{:?} must have 'off' flag for DC analysis",
                comp_type
            ));
            assert_eq!(
                off.prop_type,
                PropertyType::Boolean,
                "{:?} 'off' should be boolean",
                comp_type
            );
            // Should default to false (device on)
            match &off.default_value {
                PropertyValue::Boolean(b) => {
                    assert!(!*b, "{:?} 'off' should default to false", comp_type);
                }
                other => panic!("{:?} 'off' should be boolean, got {:?}", comp_type, other),
            }
        }
    }
}
