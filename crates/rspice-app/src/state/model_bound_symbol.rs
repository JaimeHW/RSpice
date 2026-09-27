//! Design materialization, import review and atomic publication of model-bound symbols.

use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

use super::{
    Cell, Component, ComponentType, Library, LibraryCellInstance, Point, PortDirection, PortSpec,
    Rotation, SchematicState, SymbolDocument, SymbolPin, SymbolShape, View, ViewType, Wire,
    WireConnection,
};
pub use rspice_model_library::symbol::*;

const SYMBOL_VIEW_NAME: &str = "symbol";
const PARAMETER_FORM_VIEW_NAME: &str = "parameter_form";
const TEST_FIXTURE_VIEW_NAME: &str = "testbench";

mod construction;
mod definition;
mod import;

pub use construction::*;
pub use definition::*;
pub use import::*;

#[cfg(test)]
mod tests;
