//! Versioned, model-bound symbol definitions and atomic library construction.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{
    Cell, Component, ComponentType, DisplayMode, Library, LibraryCellInstance, Point,
    PortDirection, PortSpec, PropertyDefinition, PropertySheet, PropertyType, PropertyValue,
    Rotation, SchematicState, SymbolDocument, SymbolPin, SymbolShape, View, ViewType, Wire,
    WireConnection,
};

pub const MODEL_BOUND_SYMBOL_SCHEMA_VERSION: u32 = 1;
pub const MODEL_BOUND_SYMBOL_METADATA_KEY: &str = "rspice.symbol.definition.v1";
pub const SYMBOL_PARAMETER_FORM_METADATA_KEY: &str = "rspice.symbol.parameter_form.v1";
pub const SYMBOL_IMPORT_SOURCE_METADATA_KEY: &str = "rspice.symbol.import_source.v1";

const SYMBOL_VIEW_NAME: &str = "symbol";
const PARAMETER_FORM_VIEW_NAME: &str = "parameter_form";
const TEST_FIXTURE_VIEW_NAME: &str = "testbench";
const MAX_DEFINITION_BYTES: usize = 2 * 1024 * 1024;
const MAX_IMPORTED_GRAPHIC_BYTES: usize = 1024 * 1024;

mod construction;
mod contracts;
mod definition;
mod form;
mod import;
mod validation;

pub use construction::*;
pub use contracts::*;
pub use definition::*;
pub use form::*;
pub use import::*;
pub use validation::*;

#[cfg(test)]
mod tests;
