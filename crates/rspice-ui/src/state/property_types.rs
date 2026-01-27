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
// Property Definition
// =============================================================================

/// Metadata for a property definition.
///
/// Defines the name, type, constraints, and display information for a property.
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

    /// Category for grouping in the property sheet
    pub category: String,
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
        // Resistor
        let mut resistor = PropertySheet::new();
        resistor.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_description("Unique identifier for this resistor")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("R1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        resistor.add(
            PropertyDefinition::new("r")
                .with_display_name("Resistance")
                .with_description("Resistance value in Ohms")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1000.0))
                .with_unit("Ω")
                .with_range(0.0, 1e15)
                .with_order(1)
                .with_category("Electrical")
                .required(),
        );
        resistor.add(
            PropertyDefinition::new("tc1")
                .with_display_name("Temp Coeff 1")
                .with_description("First-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C")
                .with_order(2)
                .with_category("Temperature"),
        );
        resistor.add(
            PropertyDefinition::new("tc2")
                .with_display_name("Temp Coeff 2")
                .with_description("Second-order temperature coefficient")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("1/°C²")
                .with_order(3)
                .with_category("Temperature"),
        );
        self.sheets.insert(ComponentType::Resistor, resistor);

        // Capacitor
        let mut capacitor = PropertySheet::new();
        capacitor.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("C1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        capacitor.add(
            PropertyDefinition::new("c")
                .with_display_name("Capacitance")
                .with_description("Capacitance value in Farads")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("F")
                .with_range(0.0, 1e3)
                .with_order(1)
                .with_category("Electrical")
                .required(),
        );
        capacitor.add(
            PropertyDefinition::new("ic")
                .with_display_name("Initial Voltage")
                .with_description("Initial voltage across capacitor")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(2)
                .with_category("Initial Conditions"),
        );
        self.sheets.insert(ComponentType::Capacitor, capacitor);

        // Inductor
        let mut inductor = PropertySheet::new();
        inductor.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("L1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        inductor.add(
            PropertyDefinition::new("l")
                .with_display_name("Inductance")
                .with_description("Inductance value in Henries")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1e-3))
                .with_unit("H")
                .with_range(0.0, 1e6)
                .with_order(1)
                .with_category("Electrical")
                .required(),
        );
        inductor.add(
            PropertyDefinition::new("ic")
                .with_display_name("Initial Current")
                .with_description("Initial current through inductor")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(2)
                .with_category("Initial Conditions"),
        );
        self.sheets.insert(ComponentType::Inductor, inductor);
    }

    fn register_sources(&mut self) {
        // Voltage Source
        let mut vsource = PropertySheet::new();
        vsource.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("V1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        vsource.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Voltage")
                .with_description("DC voltage value")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("V")
                .with_order(1)
                .with_category("DC"),
        );
        vsource.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("AC analysis magnitude")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_unit("V")
                .with_order(2)
                .with_category("AC"),
        );
        vsource.add(
            PropertyDefinition::new("acphase")
                .with_display_name("AC Phase")
                .with_description("AC analysis phase")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_order(3)
                .with_category("AC"),
        );
        self.sheets.insert(ComponentType::VoltageSource, vsource);

        // Current Source
        let mut isource = PropertySheet::new();
        isource.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("I1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        isource.add(
            PropertyDefinition::new("dc")
                .with_display_name("DC Current")
                .with_description("DC current value")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("A")
                .with_order(1)
                .with_category("DC"),
        );
        isource.add(
            PropertyDefinition::new("ac")
                .with_display_name("AC Magnitude")
                .with_description("AC analysis magnitude")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_unit("A")
                .with_order(2)
                .with_category("AC"),
        );
        self.sheets.insert(ComponentType::CurrentSource, isource);
    }

    fn register_semiconductors(&mut self) {
        // Diode
        let mut diode = PropertySheet::new();
        diode.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("D1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        diode.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("Diode model name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("D"))
                .with_order(1)
                .with_category("Model"),
        );
        diode.add(
            PropertyDefinition::new("area")
                .with_display_name("Area Factor")
                .with_description("Device area multiplier")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(0.001, 1000.0)
                .with_order(2)
                .with_category("Geometry"),
        );
        self.sheets.insert(ComponentType::Diode, diode);

        // NMOS
        let mut nmos = PropertySheet::new();
        nmos.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("M1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        nmos.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("MOSFET model name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("nmos"))
                .with_order(1)
                .with_category("Model"),
        );
        nmos.add(
            PropertyDefinition::new("w")
                .with_display_name("Width")
                .with_description("Channel width")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1u"))
                .with_unit("m")
                .with_range(1e-9, 1e-3)
                .with_order(2)
                .with_category("Geometry")
                .required(),
        );
        nmos.add(
            PropertyDefinition::new("l")
                .with_display_name("Length")
                .with_description("Channel length")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("180n"))
                .with_unit("m")
                .with_range(1e-9, 1e-3)
                .with_order(3)
                .with_category("Geometry")
                .required(),
        );
        nmos.add(
            PropertyDefinition::new("m")
                .with_display_name("Multiplier")
                .with_description("Number of parallel devices")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 1000.0)
                .with_order(4)
                .with_category("Geometry"),
        );
        nmos.add(
            PropertyDefinition::new("nf")
                .with_display_name("# Fingers")
                .with_description("Number of gate fingers")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(1.0, 100.0)
                .with_order(5)
                .with_category("Geometry"),
        );
        self.sheets.insert(ComponentType::Nmos, nmos.clone());

        // PMOS (similar to NMOS)
        let mut pmos = nmos.clone();
        if let Some(def) = pmos.definitions.get_mut("name") {
            def.default_value = PropertyValue::string("M1");
        }
        if let Some(def) = pmos.definitions.get_mut("model") {
            def.default_value = PropertyValue::string("pmos");
        }
        self.sheets.insert(ComponentType::Pmos, pmos);

        // NPN BJT
        let mut npn = PropertySheet::new();
        npn.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("Q1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        npn.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("BJT model name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("npn"))
                .with_order(1)
                .with_category("Model"),
        );
        npn.add(
            PropertyDefinition::new("area")
                .with_display_name("Area Factor")
                .with_description("Emitter area multiplier")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_range(0.001, 1000.0)
                .with_order(2)
                .with_category("Geometry"),
        );
        self.sheets.insert(ComponentType::NpnBjt, npn.clone());

        // PNP BJT
        let mut pnp = npn.clone();
        if let Some(def) = pnp.definitions.get_mut("model") {
            def.default_value = PropertyValue::string("pnp");
        }
        self.sheets.insert(ComponentType::PnpBjt, pnp);
    }

    fn register_controlled_sources(&mut self) {
        // VCVS (Voltage-Controlled Voltage Source)
        let mut vcvs = PropertySheet::new();
        vcvs.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("E1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        vcvs.add(
            PropertyDefinition::new("gain")
                .with_display_name("Voltage Gain")
                .with_description("Output voltage / Input voltage")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(1)
                .with_category("Electrical")
                .required(),
        );
        self.sheets.insert(ComponentType::Vcvs, vcvs);

        // VCCS (Voltage-Controlled Current Source)
        let mut vccs = PropertySheet::new();
        vccs.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("G1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        vccs.add(
            PropertyDefinition::new("gm")
                .with_display_name("Transconductance")
                .with_description("Output current / Input voltage")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1e-3))
                .with_unit("S")
                .with_order(1)
                .with_category("Electrical")
                .required(),
        );
        self.sheets.insert(ComponentType::Vccs, vccs);

        // CCVS (Current-Controlled Voltage Source)
        let mut ccvs = PropertySheet::new();
        ccvs.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("H1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        ccvs.add(
            PropertyDefinition::new("rm")
                .with_display_name("Transresistance")
                .with_description("Output voltage / Input current")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1000.0))
                .with_unit("Ω")
                .with_order(1)
                .with_category("Electrical")
                .required(),
        );
        self.sheets.insert(ComponentType::Ccvs, ccvs);

        // CCCS (Current-Controlled Current Source)
        let mut cccs = PropertySheet::new();
        cccs.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("F1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        cccs.add(
            PropertyDefinition::new("gain")
                .with_display_name("Current Gain")
                .with_description("Output current / Input current")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(1.0))
                .with_order(1)
                .with_category("Electrical")
                .required(),
        );
        self.sheets.insert(ComponentType::Cccs, cccs);
    }
}

// =============================================================================
// Engineering Value Formatting (local helper)
// =============================================================================

/// Format a value with engineering notation (local helper function)
fn format_engineering(value: f64) -> String {
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
}
