//! Property Types System
//!
//! Commercial-grade type-safe property definitions matching Cadence Virtuoso CDF (Component
//! Description Format). Provides:
//! - Type-safe property values (Number, String, Expression, Enum, Boolean)
//! - Property definitions with metadata (units, ranges, defaults)
//! - Per-component property sheets
//! - Expression parsing and validation

use crate::quantity::EngineeringPrecision;
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

    /// Maximum Unicode scalar count for string or expression properties.
    pub max_length: Option<usize>,

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
            max_length: None,
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

    /// Set the maximum length for string or expression values.
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
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

    /// Set the condition under which this property appears in the editor.
    pub fn with_visibility(mut self, condition: VisibilityCondition) -> Self {
        self.visibility_condition = condition;
        self
    }

    /// Show this property only while another property carries a value.
    ///
    /// Instance parameters that a device only reads through a model card —
    /// sheet geometry, thermal material constants — stay hidden until the
    /// instance is actually bound to one.
    pub fn when_set(self, property: impl Into<String>) -> Self {
        self.with_visibility(VisibilityCondition::WhenPropertySet(property.into()))
    }

    /// Validate a value against this definition
    pub fn validate(&self, value: &PropertyValue) -> Result<(), String> {
        if let PropertyValue::Number { value, .. } = value
            && !value.is_finite()
        {
            return Err(format!("{} must be finite", self.display_name));
        }

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

        if let Some(max_length) = self.max_length {
            let observed = match value {
                PropertyValue::String(value) | PropertyValue::Expression(value) => {
                    Some(value.chars().count())
                }
                _ => None,
            };
            if let Some(observed) = observed
                && observed > max_length
            {
                return Err(format!(
                    "{} must contain at most {max_length} characters",
                    self.display_name
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
    #[cfg(test)]
    pub fn names(&self) -> &[String] {
        &self.order
    }
}

// =============================================================================
// Component Property Registry
// =============================================================================

use crate::state::ComponentType;

mod registry;
mod source_contract;

pub use source_contract::{
    ContractStrength, SourceContractFinding, SourceFields, TRRANDOM_DISTRIBUTIONS,
    source_contract_findings, trrandom_distribution_number,
};

/// Registry of property sheets for all component types.
///
/// This provides the "CDF" (Component Description Format) equivalent,
/// defining what properties each component type supports.
#[derive(Debug, Clone)]
pub struct PropertyRegistry {
    sheets: HashMap<ComponentType, PropertySheet>,
    /// Transaction-scoped schema for the one model-bound cell instance being
    /// edited. Generic cell instances deliberately have no global fallback:
    /// their legal parameters come from their exact library master.
    cell_instance_sheet: Option<PropertySheet>,
}

impl Default for PropertyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Engineering Value Formatting
// =============================================================================

/// Format a property value with engineering notation (SI prefixes).
///
/// Three decimals, so a field's text does not change width as the value
/// behind it changes, and `Meg` rather than the `M` a SPICE-compatible
/// parser reads back as milli.
pub fn format_engineering(value: f64) -> String {
    crate::quantity::format_engineering_value_with(value, EngineeringPrecision::Fixed(3))
}

/// The magnitude [`format_engineering`] produces, spelled for a reader rather
/// than for a parser.
///
/// Two suffixes differ, and both differences are the point.
///
/// `Meg` is SPICE's unambiguous spelling of 10^6, and in a deck it has to be:
/// `M` there means *milli*, so a megahertz written back as `1M` is a
/// thousandth of what it was. That constraint belongs to the deck. Beside a
/// unit on a surface it is worse than noise — `≥ 1.000 Meg Hz` is not how a
/// limit of one megahertz is written anywhere in the field, and a reader has
/// to translate it before they can hold it against a datasheet.
///
/// `u` is the ASCII stand-in the SPICE input grammar requires for micro; `µ`
/// is the prefix itself, and the bundled face carries that glyph — the plot
/// axes and the layout formatter already paint it.
///
/// So deck text keeps [`format_engineering`] and display text beside a unit
/// takes this. The digits are the other function's, derived rather than
/// recomputed: two formatters rounding independently would eventually print
/// two different numbers for one value on two surfaces, which is the failure a
/// shared spelling exists to prevent.
#[must_use]
pub fn format_engineering_display(value: f64) -> String {
    display_spelling(format_engineering(value))
}

/// [`format_engineering_display`] under a chosen decimal policy.
///
/// The three-decimal default lines a column up, which is what a field wants
/// and what a name does not: a swept point named "Point 27.500" pads a value
/// the operator typed as 27.5, and four significant digits cannot tell two
/// neighbouring points of a fine sweep apart. A name takes the digits it needs
/// and drops the trailing zeros it does not.
#[must_use]
pub fn format_engineering_display_with(
    value: f64,
    precision: crate::quantity::EngineeringPrecision,
) -> String {
    display_spelling(crate::quantity::format_engineering_value_with(
        value, precision,
    ))
}

/// Retype a deck magnitude's suffix for a reader.
fn display_spelling(spice: String) -> String {
    // Longest suffix first, so `Meg` is never read as something shorter.
    for (deck, display) in [("Meg", "M"), ("u", "µ")] {
        if let Some(digits) = spice.strip_suffix(deck) {
            return format!("{digits}{display}");
        }
    }
    spice
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::{
        ComponentType, PropertyDefinition, PropertyRegistry, PropertyValue, format_engineering,
        format_engineering_display,
    };

    #[test]
    fn every_static_component_family_has_a_typed_editor_schema() {
        let registry = PropertyRegistry::new();
        let missing = ComponentType::ALL
            .into_iter()
            .filter(|kind| *kind != ComponentType::CellInstance)
            .filter(|kind| registry.get(*kind).is_none())
            .collect::<Vec<_>>();
        assert!(
            missing.is_empty(),
            "component editor schemas are missing for {missing:?}"
        );
    }

    /// The property editor's column of the shared engineering table: three
    /// decimals on every fraction, so a field's width does not move as the
    /// value behind it does.
    #[test]
    fn property_text_keeps_its_three_decimals() {
        for (value, _, three_decimal, _) in crate::quantity::engineering::PRECISION_CHARACTERIZATION
        {
            assert_eq!(
                &format_engineering(*value),
                three_decimal,
                "property-editor form of {value}"
            );
        }
    }

    #[test]
    fn engineering_display_uses_unambiguous_mega_for_editor_round_trip() {
        let displayed = format_engineering(3.3e6);
        assert_eq!(displayed, "3.300Meg");
        assert_eq!(
            crate::quantity::parse_engineering_value(&displayed).unwrap(),
            3.3e6
        );
        assert_eq!(
            crate::quantity::parse_engineering_value("3.3M").unwrap(),
            3.3e-3
        );
    }

    /// The display spelling differs from the deck spelling in exactly two
    /// places, and agrees with it everywhere else.
    ///
    /// The two are the ones that would otherwise be read wrong beside a unit:
    /// `Meg`, which reads as a word rather than a prefix, and `u`, which is
    /// the ASCII stand-in for a prefix the bundled face can actually paint.
    /// Every other decade has to come through untouched, because a display
    /// formatter that quietly rounded differently from the deck one would put
    /// two numbers for one value on two surfaces.
    #[test]
    fn the_display_spelling_changes_the_prefix_and_never_the_number() {
        assert_eq!(format_engineering_display(1.0e6), "1M");
        assert_eq!(format_engineering_display(1.5e3), "1.500k");
        assert_eq!(format_engineering_display(2.5e-3), "2.500m");
        assert_eq!(format_engineering_display(4.7e-6), "4.700µ");
        assert_eq!(format_engineering_display(-3.3e6), "-3.300M");

        // Every decade the deck formatter can reach, walked: the digits are
        // its digits, and only a `Meg` or a `u` is allowed to move.
        for exponent in -18..=13 {
            for mantissa in [1.0_f64, 1.5, -4.7] {
                let value = mantissa * 10.0_f64.powi(exponent);
                let deck = format_engineering(value);
                let display = format_engineering_display(value);
                let expected = deck
                    .strip_suffix("Meg")
                    .map(|digits| format!("{digits}M"))
                    .or_else(|| deck.strip_suffix('u').map(|digits| format!("{digits}µ")))
                    .unwrap_or_else(|| deck.clone());
                assert_eq!(display, expected, "{value:e} deck `{deck}`");
                assert!(
                    !display.contains("Meg"),
                    "{value:e} still reads as a word beside a unit: {display}"
                );
                assert!(
                    !display.ends_with('u'),
                    "{value:e} still uses the ASCII stand-in: {display}"
                );
            }
        }
    }

    /// The one glyph this formatter introduces has to be one the bundled face
    /// can paint, or every spec limit under a millisecond shows a box where
    /// its prefix should be.
    ///
    /// Asked by rasterizing it, not by asking the font. The face is
    /// monospaced, so a missing glyph advances exactly as far as a present
    /// one and layout width cannot tell them apart; and `Fonts::has_glyph`
    /// answers for one face rather than for the family's fallbacks, so it says
    /// "no" to characters this application demonstrably paints. What the
    /// prefix actually has to do is leave ink, and that is what is measured.
    #[test]
    fn the_micro_prefix_is_a_glyph_the_bundled_face_carries() {
        let rows = |text: &str| crate::ui::raster::glyph_ink_rows(text, crate::ui::tokens::FS_0);
        let micro = rows("µ");
        assert!(
            micro.len() >= 3,
            "the micro prefix rasterized to {} row(s) of ink: {micro:?}",
            micro.len()
        );
        // A control that is definitely not the prefix, so a harness that
        // reported ink for everything fails here rather than passing.
        assert!(rows(" ").is_empty(), "a space rasterized ink");
    }

    #[test]
    fn property_schema_rejects_every_non_finite_number() {
        let definition = PropertyDefinition::new("value").with_display_name("Value");

        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                definition.validate(&PropertyValue::number(value)),
                Err("Value must be finite".to_owned())
            );
        }
    }
}
