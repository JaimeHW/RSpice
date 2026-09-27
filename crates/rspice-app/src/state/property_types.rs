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
    DisplayMode, PropertyDefinition, PropertySheet, PropertyType, PropertyValue,
    VisibilityCondition, format_engineering,
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
pub struct PropertyCatalog {
    sheets: HashMap<ComponentType, PropertySheet>,
}

impl Default for PropertyCatalog {
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
    use super::{ComponentType, PropertyCatalog};

    #[test]
    fn every_static_component_family_has_a_typed_editor_schema() {
        let registry = PropertyCatalog::new();
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
}
