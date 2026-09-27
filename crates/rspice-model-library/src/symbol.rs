//! Portable model-bound symbol definitions, typed forms and retained graphics.
//!
//! Design documents and publication transactions consume these contracts in the application.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::{fmt, path::Path};

use rspice_app_types::property::{
    DisplayMode, PropertyDefinition, PropertySheet, PropertyType, PropertyValue,
};
use rspice_design_model::{
    Point,
    port::{PortDirection, PortSpec},
    symbol_geometry::SymbolShape,
};
use serde::{Deserialize, Serialize};

pub const MODEL_BOUND_SYMBOL_SCHEMA_VERSION: u32 = 1;
pub const MODEL_BOUND_SYMBOL_METADATA_KEY: &str = "rspice.symbol.definition.v1";
pub const SYMBOL_PARAMETER_FORM_METADATA_KEY: &str = "rspice.symbol.parameter_form.v1";
pub const SYMBOL_IMPORT_SOURCE_METADATA_KEY: &str = "rspice.symbol.import_source.v1";
const MAX_DEFINITION_BYTES: usize = 2 * 1024 * 1024;
const MAX_IMPORTED_GRAPHIC_BYTES: usize = 1024 * 1024;

mod contracts;
mod definition;
mod form;
mod graphics;
mod template;
mod validation;

pub use rspice_design_model::symbol_pin::{SymbolElectricalType, SymbolPinSide};

pub use contracts::*;
pub use definition::*;
pub use form::*;
use graphics::validate_imported_graphic;
pub use graphics::{SymbolImportFormat, SymbolImportReport, validate_import_pin_anchors};
pub use template::validate_library_netlist_template;
pub use validation::SymbolDefinitionError;
use validation::{
    form_diag, parse_engineering, validate_identity, validate_key, validate_netlist,
    validate_parameter_field, validate_pins, validate_source,
};

#[cfg(test)]
mod tests;
