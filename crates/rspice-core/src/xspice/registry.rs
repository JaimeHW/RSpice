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
        self.register(Arc::new(super::models::Gain::default()));
        self.register(Arc::new(super::models::Summer::default()));
        self.register(Arc::new(super::models::Multiplier::default()));
        self.register(Arc::new(super::models::Divider::default()));
        self.register(Arc::new(super::models::Limiter::default()));
        self.register(Arc::new(super::models::Integrator::default()));
        self.register(Arc::new(super::models::Differentiator::default()));
        self.register(Arc::new(super::models::AnalogSwitch::default()));
        self.register(Arc::new(super::models::SampleHold::default()));

        // A/D and D/A bridges
        self.register(Arc::new(super::models::AdcBridge::default()));
        self.register(Arc::new(super::models::DacBridge::default()));

        // Digital sources
        self.register(Arc::new(super::models::DigitalSource::default()));

        // Digital gates
        self.register(Arc::new(super::models::DigitalInverter::default()));
        self.register(Arc::new(super::models::DigitalBuffer::default()));
        self.register(Arc::new(super::models::DigitalAnd::default()));
        self.register(Arc::new(super::models::DigitalNand::default()));
        self.register(Arc::new(super::models::DigitalOr::default()));
        self.register(Arc::new(super::models::DigitalNor::default()));
        self.register(Arc::new(super::models::DigitalXor::default()));
        self.register(Arc::new(super::models::DigitalXnor::default()));
        self.register(Arc::new(super::models::DigitalTristate::default()));
        self.register(Arc::new(super::models::DigitalPullup::default()));
        self.register(Arc::new(super::models::DigitalPulldown::default()));

        // Flip-flops and latches
        self.register(Arc::new(super::models::DFlipFlop::default()));
        self.register(Arc::new(super::models::JkFlipFlop::default()));
        self.register(Arc::new(super::models::TFlipFlop::default()));
        self.register(Arc::new(super::models::SrFlipFlop::default()));
        self.register(Arc::new(super::models::DLatch::default()));
        self.register(Arc::new(super::models::SrLatch::default()));

        // State machine
        self.register(Arc::new(super::models::DigitalStateMachine::default()));

        // Memory
        self.register(Arc::new(super::models::DigitalRam::default()));
        self.register(Arc::new(super::models::DigitalRom::default()));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_basic() {
        let registry = CodeModelRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_registry_with_builtins() {
        let registry = CodeModelRegistry::with_builtins();
        assert!(!registry.is_empty());

        // Should have all the built-in models registered
        assert!(registry.contains("gain"), "Should have gain model");
        assert!(registry.contains("summer"), "Should have summer model");
        assert!(registry.contains("mult"), "Should have multiplier model");
        assert!(registry.contains("divider"), "Should have divider model");
        assert!(registry.contains("limit"), "Should have limiter model");
        assert!(
            registry.contains("integrator"),
            "Should have integrator model"
        );
        assert!(
            registry.contains("differentiator"),
            "Should have differentiator model"
        );
        assert!(
            registry.contains("aswitch"),
            "Should have analog switch model"
        );
        assert!(registry.contains("s_h"), "Should have sample/hold model");

        // A/D D/A bridges
        assert!(registry.contains("adc_bridge"), "Should have ADC bridge");
        assert!(registry.contains("dac_bridge"), "Should have DAC bridge");

        // Digital gates
        assert!(registry.contains("d_inverter"), "Should have inverter");
        assert!(registry.contains("d_buffer"), "Should have buffer");
        assert!(registry.contains("d_and"), "Should have AND gate");
        assert!(registry.contains("d_or"), "Should have OR gate");
        assert!(registry.contains("d_xor"), "Should have XOR gate");
        assert!(registry.contains("d_nand"), "Should have NAND gate");
        assert!(registry.contains("d_nor"), "Should have NOR gate");
        assert!(registry.contains("d_xnor"), "Should have XNOR gate");
        assert!(registry.contains("d_tristate"), "Should have tristate");
        assert!(registry.contains("d_pullup"), "Should have pullup");
        assert!(registry.contains("d_pulldown"), "Should have pulldown");

        // Flip-flops and latches
        assert!(registry.contains("d_dff"), "Should have D flip-flop");
        assert!(registry.contains("d_jkff"), "Should have JK flip-flop");
        assert!(registry.contains("d_tff"), "Should have T flip-flop");
        assert!(registry.contains("d_srff"), "Should have SR flip-flop");
        assert!(registry.contains("d_dlatch"), "Should have D latch");
        assert!(registry.contains("d_srlatch"), "Should have SR latch");

        // Memory and state machine
        assert!(registry.contains("d_state"), "Should have state machine");
        assert!(registry.contains("d_ram"), "Should have RAM");
        assert!(registry.contains("d_rom"), "Should have ROM");

        // Digital source
        assert!(registry.contains("d_source"), "Should have digital source");
    }

    #[test]
    fn test_registry_get_model() {
        let registry = CodeModelRegistry::with_builtins();

        let gain = registry.get("gain");
        assert!(gain.is_some());
        assert_eq!(gain.unwrap().name(), "gain");

        // Case insensitive
        let gain_upper = registry.get("GAIN");
        assert!(gain_upper.is_some());

        // Non-existent model
        let nonexistent = registry.get("nonexistent_model");
        assert!(nonexistent.is_none());
    }

    #[test]
    fn test_registry_model_names() {
        let registry = CodeModelRegistry::with_builtins();
        let names = registry.model_names();

        // Should have many models
        assert!(names.len() >= 30, "Should have at least 30 built-in models");

        // Check some specific names are present
        assert!(names.contains(&"gain"));
        assert!(names.contains(&"d_and"));
        assert!(names.contains(&"adc_bridge"));
    }

    #[test]
    fn test_registry_categories() {
        let registry = CodeModelRegistry::with_builtins();

        let analog_models = registry.analog_models();
        let digital_models = registry.digital_models();
        let mixed_models = registry.mixed_models();

        // Verify analog models don't include digital-only models
        for model in &analog_models {
            assert!(
                !model.is_digital_only(),
                "Analog category should not contain digital-only models: {}",
                model.name()
            );
        }

        // Verify digital models don't include analog-only models
        for model in &digital_models {
            assert!(
                !model.is_analog_only(),
                "Digital category should not contain analog-only models: {}",
                model.name()
            );
        }

        // Mixed models should have both analog and digital ports (bridges)
        assert!(
            !mixed_models.is_empty(),
            "Should have mixed-signal models (bridges)"
        );
    }
}
