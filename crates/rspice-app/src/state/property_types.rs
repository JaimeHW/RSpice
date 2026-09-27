//! Property Types System
//!
//! Commercial-grade type-safe property definitions matching Cadence Virtuoso CDF (Component
//! Description Format). Provides:
//! - Type-safe property values (Number, String, Expression, Enum, Boolean)
//! - Property definitions with metadata (units, ranges, defaults)
//! - Per-component property sheets
//! - Expression parsing and validation

use std::collections::HashMap;

pub use rspice_app_types::property::{
    DisplayMode, PropertyDefinition, PropertySheet, PropertyType, PropertyValue, VisibilityCondition,
    format_engineering,
};

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

pub use rspice_app_types::quantity::{format_engineering_display, format_engineering_display_with};

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::{ComponentType, PropertyRegistry};

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
}
