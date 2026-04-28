//! Code Model Registry
//!
//! Central registry for discovering and instantiating XSPICE code models.
//! Supports both built-in models and dynamically loaded external models.

use super::traits::CodeModel;
use std::collections::HashMap;
use std::sync::Arc;

//=============================================================================
// Registry
//=============================================================================

/// Registry for XSPICE code models
///
/// Provides a central lookup for all available code models.
/// Models are registered by name and can be instantiated for circuit use.
#[derive(Default)]
pub struct CodeModelRegistry {
    /// Registered models by name (lowercase)
    models: HashMap<String, Arc<dyn CodeModel>>,
}

impl CodeModelRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a registry with all built-in models registered
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        registry.register_builtins();
        registry
    }

    /// Register a code model
    ///
    /// The model is registered under its name (case-insensitive).
    pub fn register(&mut self, model: Arc<dyn CodeModel>) {
        let name = model.name().to_lowercase();
        self.models.insert(name, model);
    }

    /// Get a code model by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn CodeModel>> {
        self.models.get(&name.to_lowercase()).cloned()
    }

    /// Check if a model is registered
    pub fn contains(&self, name: &str) -> bool {
        self.models.contains_key(&name.to_lowercase())
    }

    /// Get all registered model names
    pub fn model_names(&self) -> Vec<&str> {
        self.models.values().map(|m| m.name()).collect()
    }

    /// Get the number of registered models
    pub fn len(&self) -> usize {
        self.models.len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.models.is_empty()
    }

    /// Register all built-in models
    pub fn register_builtins(&mut self) {
        // Analog behavioral models
        self.register(Arc::new(super::models::Gain));
        self.register(Arc::new(super::models::Summer));
        self.register(Arc::new(super::models::Multiplier));
        self.register(Arc::new(super::models::Divider));
        self.register(Arc::new(super::models::Limiter));
        self.register(Arc::new(super::models::Integrator));
        self.register(Arc::new(super::models::Differentiator));
        self.register(Arc::new(super::models::AnalogSwitch));
        self.register(Arc::new(super::models::SampleHold));

        // A/D and D/A bridges
        self.register(Arc::new(super::models::AdcBridge));
        self.register(Arc::new(super::models::DacBridge));

        // Digital sources
        self.register(Arc::new(super::models::DigitalSource));

        // Digital gates
        self.register(Arc::new(super::models::DigitalInverter));
        self.register(Arc::new(super::models::DigitalBuffer));
        self.register(Arc::new(super::models::DigitalAnd));
        self.register(Arc::new(super::models::DigitalNand));
        self.register(Arc::new(super::models::DigitalOr));
        self.register(Arc::new(super::models::DigitalNor));
        self.register(Arc::new(super::models::DigitalXor));
        self.register(Arc::new(super::models::DigitalXnor));
        self.register(Arc::new(super::models::DigitalTristate));
        self.register(Arc::new(super::models::DigitalPullup));
        self.register(Arc::new(super::models::DigitalPulldown));

        // Flip-flops and latches
        self.register(Arc::new(super::models::DFlipFlop));
        self.register(Arc::new(super::models::JkFlipFlop));
        self.register(Arc::new(super::models::TFlipFlop));
        self.register(Arc::new(super::models::SrFlipFlop));
        self.register(Arc::new(super::models::DLatch));
        self.register(Arc::new(super::models::SrLatch));

        // State machine
        self.register(Arc::new(super::models::DigitalStateMachine));

        // Memory
        self.register(Arc::new(super::models::DigitalRam));
        self.register(Arc::new(super::models::DigitalRom));
    }

    /// Get models by category
    pub fn analog_models(&self) -> Vec<Arc<dyn CodeModel>> {
        self.models
            .values()
            .filter(|m| m.is_analog_only())
            .cloned()
            .collect()
    }

    /// Get digital models
    pub fn digital_models(&self) -> Vec<Arc<dyn CodeModel>> {
        self.models
            .values()
            .filter(|m| m.is_digital_only())
            .cloned()
            .collect()
    }

    /// Get mixed-signal models (have both analog and digital ports)
    pub fn mixed_models(&self) -> Vec<Arc<dyn CodeModel>> {
        self.models
            .values()
            .filter(|m| !m.is_analog_only() && !m.is_digital_only())
            .cloned()
            .collect()
    }
}

impl std::fmt::Debug for CodeModelRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CodeModelRegistry")
            .field("models", &self.model_names())
            .finish()
    }
}

//=============================================================================
// Tests
//=============================================================================

