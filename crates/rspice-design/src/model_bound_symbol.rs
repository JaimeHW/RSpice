//! Model-bound symbol materialization, import review, and guarded library construction.

use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

use crate::library::{Cell, Library, View, ViewType};
use crate::schematic::component::{Component, LibraryCellInstance};
use crate::schematic::component_type::ComponentType;
use crate::schematic::document::SchematicDocument;
use crate::schematic::rotation::Rotation;
use crate::schematic::wire::{Wire, WireConnection};
use crate::symbol::{SymbolDocument, SymbolPin, SymbolShape};
use rspice_design_model::{
    Point,
    port::{PortDirection, PortSpec},
};
use rspice_model_library::symbol::*;

const SYMBOL_VIEW_NAME: &str = "symbol";
const PARAMETER_FORM_VIEW_NAME: &str = "parameter_form";
const TEST_FIXTURE_VIEW_NAME: &str = "testbench";

mod construction;
mod definition;
mod fixture;
mod import;

pub use construction::SymbolConstructionPlan;
use definition::serialize_cell;
pub use definition::{
    load_model_bound_symbol, materialize_symbol_document, prepare_symbol_construction,
    store_model_bound_symbol,
};
pub use fixture::build_symbol_test_fixture_document;
pub use import::SymbolDefinitionImport;

#[cfg(test)]
mod tests;
