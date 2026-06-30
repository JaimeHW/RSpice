//! Code Model Registry
//!
//! Central registry for discovering and instantiating XSPICE code models.
//! Supports both built-in models and dynamically loaded external models.

use super::traits::CodeModel;
use std::borrow::Cow;
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

#[inline]
fn canonical_model_key(name: &str) -> String {
    name.to_lowercase()
}

#[inline]
fn canonical_model_lookup_key(name: &str) -> Cow<'_, str> {
    if name.chars().any(char::is_uppercase) {
        Cow::Owned(canonical_model_key(name))
    } else {
        Cow::Borrowed(name)
    }
}

const BUILTIN_MODEL_NAMES: &[&str] = &[
    "adc_bridge",
    "astate",
    "aswitch",
    "bidi_bridge",
    "capacitoric",
    "climit",
    "cmeter",
    "core",
    "cpline",
    "cpmlin",
    "d_and",
    "d_buffer",
    "d_cosim",
    "d_dff",
    "d_dlatch",
    "d_dt",
    "d_fdiv",
    "d_genlut",
    "d_inverter",
    "d_jkff",
    "d_lut",
    "d_nand",
    "d_nor",
    "d_open_c",
    "d_open_e",
    "d_or",
    "d_osc",
    "d_process",
    "d_pulldown",
    "d_pullup",
    "d_pwm",
    "d_ram",
    "d_source",
    "d_srff",
    "d_srlatch",
    "d_state",
    "d_tff",
    "d_to_real",
    "d_tristate",
    "d_xnor",
    "d_xor",
    "dac_bridge",
    "delay",
    "differentiator",
    "divide",
    "divider",
    "file_source",
    "filesource",
    "gain",
    "hyst",
    "icm_spice2poly",
    "ilimit",
    "inductoric",
    "int",
    "integrator",
    "lcouple",
    "limit",
    "lmeter",
    "memristor",
    "mlin",
    "msopen",
    "mult",
    "multi_input_pwl",
    "nco",
    "oneshot",
    "potentiometer",
    "print_param_types",
    "pswitch",
    "pwl",
    "pwlts",
    "real_delay",
    "real_gain",
    "real_to_v",
    "s_h",
    "s_xfer",
    "seegen",
    "sidiode",
    "sine",
    "slew",
    "spice2poly",
    "square",
    "summer",
    "table2d",
    "table3d",
    "tline",
    "triangle",
    "xfer",
    "zener",
];

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

    /// Get all built-in model names without constructing model instances.
    pub fn builtin_model_names() -> &'static [&'static str] {
        BUILTIN_MODEL_NAMES
    }

    /// Check whether a name belongs to a built-in code model.
    pub fn is_builtin_model_name(name: &str) -> bool {
        BUILTIN_MODEL_NAMES
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(name))
    }

    /// Register a code model
    ///
    /// The model is registered under its name (case-insensitive).
    pub fn register(&mut self, model: Arc<dyn CodeModel>) {
        let name = canonical_model_key(model.name());
        self.models.insert(name, model);
    }

    /// Get a code model by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn CodeModel>> {
        let key = canonical_model_lookup_key(name);
        self.models.get(key.as_ref()).cloned()
    }

    /// Check if a model is registered
    pub fn contains(&self, name: &str) -> bool {
        let key = canonical_model_lookup_key(name);
        self.models.contains_key(key.as_ref())
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
        self.register(Arc::new(super::models::DivideAlias));
        self.register(Arc::new(super::models::PiecewiseLinear));
        self.register(Arc::new(super::models::PiecewiseLinearTimeSeries));
        self.register(Arc::new(super::models::FileSource));
        self.register(Arc::new(super::models::FileSourceAlias));
        self.register(Arc::new(super::models::Table2D));
        self.register(Arc::new(super::models::Table3D));
        self.register(Arc::new(super::models::MultiInputPwl));
        self.register(Arc::new(super::models::Spice2Poly));
        self.register(Arc::new(super::models::IcmSpice2Poly));
        self.register(Arc::new(super::models::Xfer));
        self.register(Arc::new(super::models::SXfer));
        self.register(Arc::new(super::models::SineOscillator));
        self.register(Arc::new(super::models::SquareOscillator));
        self.register(Arc::new(super::models::TriangleOscillator));
        self.register(Arc::new(super::models::Limiter));
        self.register(Arc::new(super::models::ControlledLimiter));
        self.register(Arc::new(super::models::HysteresisBlock));
        self.register(Arc::new(super::models::AnalogDelayLine));
        self.register(Arc::new(super::models::AnalogStateReturn));
        self.register(Arc::new(super::models::AnalogOneShot));
        self.register(Arc::new(super::models::Integrator));
        self.register(Arc::new(super::models::IntegratorAlias));
        self.register(Arc::new(super::models::Differentiator));
        self.register(Arc::new(super::models::DifferentiatorAlias));
        self.register(Arc::new(super::models::SlewRateFollower));
        self.register(Arc::new(super::models::AnalogSwitch));
        self.register(Arc::new(super::models::SampleHold));
        self.register(Arc::new(super::models::Potentiometer));
        self.register(Arc::new(super::models::Pswitch));
        self.register(Arc::new(super::models::Sidiode));
        self.register(Arc::new(super::models::Zener));
        self.register(Arc::new(super::models::Memristor));
        self.register(Arc::new(super::models::Core));
        self.register(Arc::new(super::models::CapacitorIc));
        self.register(Arc::new(super::models::InductorIc));
        self.register(Arc::new(super::models::CapacitanceMeter));
        self.register(Arc::new(super::models::InductanceMeter));
        self.register(Arc::new(super::models::LcCouple));
        self.register(Arc::new(super::models::Ilimit));
        self.register(Arc::new(super::models::SeeGenerator));
        self.register(Arc::new(super::models::CoupledMicrostripLine));
        self.register(Arc::new(super::models::CoupledTransmissionLine));
        self.register(Arc::new(super::models::GenericTransmissionLine));
        self.register(Arc::new(super::models::MicrostripLine));
        self.register(Arc::new(super::models::MicrostripOpenEnd));

        // Debug/example models
        self.register(Arc::new(super::models::PrintParamTypes));

        // A/D and D/A bridges
        self.register(Arc::new(super::models::AdcBridge));
        self.register(Arc::new(super::models::DacBridge));
        self.register(Arc::new(super::models::BidiBridge));

        // Real-valued event models
        self.register(Arc::new(super::models::DigitalToReal));
        self.register(Arc::new(super::models::RealGain));
        self.register(Arc::new(super::models::RealDelay));
        self.register(Arc::new(super::models::RealToVoltage));

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
        self.register(Arc::new(super::models::DigitalOpenCollector));
        self.register(Arc::new(super::models::DigitalOpenEmitter));
        self.register(Arc::new(super::models::DigitalLookupTable));
        self.register(Arc::new(super::models::DigitalGenericLookupTable));
        self.register(Arc::new(super::models::DigitalOscillator));
        self.register(Arc::new(super::models::DigitalPwmOscillator));
        self.register(Arc::new(super::models::NumericallyControlledOscillator));
        self.register(Arc::new(super::models::DigitalProcess));
        self.register(Arc::new(super::models::DigitalCosim));

        // Flip-flops and latches
        self.register(Arc::new(super::models::DFlipFlop));
        self.register(Arc::new(super::models::JkFlipFlop));
        self.register(Arc::new(super::models::TFlipFlop));
        self.register(Arc::new(super::models::SrFlipFlop));
        self.register(Arc::new(super::models::DLatch));
        self.register(Arc::new(super::models::SrLatch));
        self.register(Arc::new(super::models::DigitalFrequencyDivider));

        // State machine
        self.register(Arc::new(super::models::DigitalStateMachine));

        // Memory
        self.register(Arc::new(super::models::DigitalRam));
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

    const OFFICIAL_NGSPICE_46_XSPICE_MODELS: &[&str] = &[
        "adc_bridge",
        "astate",
        "aswitch",
        "bidi_bridge",
        "capacitoric",
        "climit",
        "cmeter",
        "core",
        "cpline",
        "cpmlin",
        "d_and",
        "d_buffer",
        "d_cosim",
        "d_dff",
        "d_dlatch",
        "d_dt",
        "d_fdiv",
        "d_genlut",
        "d_inverter",
        "d_jkff",
        "d_lut",
        "d_nand",
        "d_nor",
        "d_open_c",
        "d_open_e",
        "d_or",
        "d_osc",
        "d_process",
        "d_pulldown",
        "d_pullup",
        "d_pwm",
        "d_ram",
        "d_source",
        "d_srff",
        "d_srlatch",
        "d_state",
        "d_tff",
        "d_to_real",
        "d_tristate",
        "d_xnor",
        "d_xor",
        "dac_bridge",
        "delay",
        "divide",
        "filesource",
        "gain",
        "hyst",
        "ilimit",
        "inductoric",
        "int",
        "lcouple",
        "limit",
        "lmeter",
        "memristor",
        "mlin",
        "msopen",
        "mult",
        "multi_input_pwl",
        "oneshot",
        "potentiometer",
        "print_param_types",
        "pswitch",
        "pwl",
        "pwlts",
        "real_delay",
        "real_gain",
        "real_to_v",
        "s_xfer",
        "seegen",
        "sidiode",
        "sine",
        "slew",
        "spice2poly",
        "square",
        "summer",
        "table2d",
        "table3d",
        "tline",
        "triangle",
        "xfer",
        "zener",
    ];

    fn has_builtin_or_native_lowering(registry: &CodeModelRegistry, name: &str) -> bool {
        registry.contains(name)
    }

    #[test]
    fn static_builtin_catalog_matches_registered_model_names() {
        let registry = CodeModelRegistry::with_builtins();
        let mut registered = registry.model_names();
        registered.sort_unstable();

        let mut catalog = CodeModelRegistry::builtin_model_names().to_vec();
        catalog.sort_unstable();

        assert_eq!(registered, catalog);
        assert!(CodeModelRegistry::is_builtin_model_name("DAC_BRIDGE"));
        assert!(!CodeModelRegistry::is_builtin_model_name("d_rom"));
    }

    #[test]
    fn registry_lookup_remains_case_insensitive() {
        let registry = CodeModelRegistry::with_builtins();

        let lower = registry
            .get("dac_bridge")
            .expect("lowercase dac_bridge resolves");
        let mixed = registry
            .get("DAC_Bridge")
            .expect("mixed-case dac_bridge resolves");

        assert_eq!(lower.name(), "dac_bridge");
        assert_eq!(mixed.name(), "dac_bridge");
        assert!(registry.contains("DAC_BRIDGE"));
        assert!(!registry.contains("d_rom"));
    }

    #[test]
    fn builtins_cover_official_ngspice_46_xspice_catalog() {
        let registry = CodeModelRegistry::with_builtins();
        let missing: Vec<&str> = OFFICIAL_NGSPICE_46_XSPICE_MODELS
            .iter()
            .copied()
            .filter(|name| !has_builtin_or_native_lowering(&registry, name))
            .collect();

        assert!(
            missing.is_empty(),
            "missing official ngspice 46 XSPICE models: {missing:?}"
        );
    }

    #[test]
    fn builtins_do_not_register_unimplemented_digital_rom_stub() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("d_ram"));
        assert!(
            !registry.contains("d_rom"),
            "d_rom must fail closed until a real code model is implemented"
        );
    }

    #[test]
    fn builtins_register_official_external_digital_process() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("d_process"));
    }

    #[test]
    fn builtins_register_official_external_digital_cosim() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("d_cosim"));
    }

    #[test]
    fn builtins_register_official_analog_aliases() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("divide"));
        assert!(registry.contains("int"));
        assert!(registry.contains("d_dt"));
    }

    #[test]
    fn builtins_register_lookup_models() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("pwl"));
        assert!(registry.contains("pwlts"));
        assert!(registry.contains("filesource"));
        assert!(registry.contains("file_source"));
        assert!(registry.contains("table2d"));
        assert!(registry.contains("table3d"));
    }

    #[test]
    fn builtins_register_official_waveform_oscillators() {
        let registry = CodeModelRegistry::with_builtins();

        for name in ["sine", "square", "triangle"] {
            assert!(registry.contains(name), "{name} must be registered");
        }
    }

    #[test]
    fn builtins_register_official_multi_input_pwl() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("multi_input_pwl"));
    }

    #[test]
    fn builtins_register_official_hysteresis_block() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("hyst"));
    }

    #[test]
    fn builtins_register_official_delay_line() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("delay"));
    }

    #[test]
    fn builtins_register_official_analog_state_return() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("astate"));
    }

    #[test]
    fn builtins_register_official_oneshot() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("oneshot"));
    }

    #[test]
    fn builtins_register_official_spice2poly() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("spice2poly"));
        assert!(registry.contains("icm_spice2poly"));
    }

    #[test]
    fn builtins_register_official_xfer() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("xfer"));
        assert!(registry.contains("s_xfer"));
    }

    #[test]
    fn builtins_register_official_potentiometer() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("potentiometer"));
    }

    #[test]
    fn builtins_register_official_pswitch() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("pswitch"));
    }

    #[test]
    fn builtins_register_official_sidiode() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("sidiode"));
    }

    #[test]
    fn builtins_register_official_zener() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("zener"));
    }

    #[test]
    fn builtins_register_official_ilimit() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("ilimit"));
    }

    #[test]
    fn builtins_register_official_real_event_models() {
        let registry = CodeModelRegistry::with_builtins();

        for name in ["d_to_real", "real_gain", "real_delay", "real_to_v"] {
            assert!(registry.contains(name), "{name} must be registered");
        }
    }

    #[test]
    fn builtins_register_official_slew_rate_follower() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("slew"));
    }

    #[test]
    fn builtins_register_official_open_output_digital_buffers() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("d_open_c"));
        assert!(registry.contains("d_open_e"));
    }

    #[test]
    fn builtins_register_official_bidirectional_bridge() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("bidi_bridge"));
    }

    #[test]
    fn builtins_register_official_digital_lookup_tables() {
        let registry = CodeModelRegistry::with_builtins();

        assert!(registry.contains("d_lut"));
        assert!(registry.contains("d_genlut"));
        assert!(registry.contains("d_fdiv"));
        assert!(registry.contains("d_osc"));
        assert!(registry.contains("d_pwm"));
    }
}
