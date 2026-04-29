use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::{DeviceModel, ModelType, ProcessCorner};

/// A PDK model library
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelLibrary {
    /// Library name (e.g., "tsmc180_1p8v")
    pub name: String,
    /// PDK name
    pub pdk_name: String,
    /// Technology node (e.g., "180nm", "65nm")
    pub technology_node: String,
    /// Root path on disk
    pub root_path: Option<PathBuf>,
    /// Device models
    pub models: HashMap<String, DeviceModel>,
    /// Process corners
    pub corners: HashMap<String, ProcessCorner>,
    /// Currently selected corner
    pub selected_corner: Option<String>,
    /// Version string
    pub version: String,
    /// Is expanded in browser
    pub expanded: bool,
}

impl ModelLibrary {
    /// Create a new library
    pub fn new(name: impl Into<String>) -> Self {
        let mut lib = Self {
            name: name.into(),
            ..Default::default()
        };
        // Add standard corners by default
        for corner in ProcessCorner::standard_corners() {
            if corner.is_default {
                lib.selected_corner = Some(corner.name.clone());
            }
            lib.corners.insert(corner.name.clone(), corner);
        }
        lib
    }

    /// Set technology
    pub fn with_technology(mut self, pdk: impl Into<String>, node: impl Into<String>) -> Self {
        self.pdk_name = pdk.into();
        self.technology_node = node.into();
        self
    }

    /// Add a model
    pub fn add_model(&mut self, model: DeviceModel) {
        self.models.insert(model.name.clone(), model);
    }

    /// Get a model by name
    pub fn get_model(&self, name: &str) -> Option<&DeviceModel> {
        self.models.get(name)
    }

    /// Get models by type
    pub fn models_by_type(&self, model_type: ModelType) -> Vec<&DeviceModel> {
        self.models
            .values()
            .filter(|m| m.model_type == model_type)
            .collect()
    }

    /// Select a corner
    pub fn select_corner(&mut self, name: &str) -> bool {
        if self.corners.contains_key(name) {
            self.selected_corner = Some(name.to_string());
            true
        } else {
            false
        }
    }

    /// Get the selected corner
    pub fn current_corner(&self) -> Option<&ProcessCorner> {
        self.selected_corner
            .as_ref()
            .and_then(|name| self.corners.get(name))
    }

    /// Get model count
    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// Get corner count
    pub fn corner_count(&self) -> usize {
        self.corners.len()
    }
}
