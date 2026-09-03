//! Device models module
//!
//! Provides behavioral models for circuit components, organized into:
//! - `passive/` - Resistors, capacitors, inductors
//! - `semiconductor/` - Diodes, BJTs
//! - `mosfet/` - MOSFETs, VDMOSFETs, JFETs, BSIM4, EKV
//! - `veriloga/` - Verilog-A compiled models (optional feature)
// Organized device subdirectories
pub mod mosfet;
pub mod passive;
pub mod semiconductor;

// Other devices (kept at root level)
pub mod behavioral;
pub mod controlled;
mod coupled_transmission_line;
mod cpl_native;
pub mod memristor_pem;
pub mod memristor_team;
pub mod pwl_file;
mod sources;
mod switch;
pub mod thermal;
mod traits;
mod transmission_line;

// Verilog-A support (feature-gated)
#[cfg(feature = "veriloga")]
pub mod veriloga;

// Generated Verilog-A runtime ABI; the built-in registry remains feature-gated inside the module.
pub mod veriloga_builtins;

// Re-export from subdirectories for backwards compatibility
pub use mosfet::{
    B3SoiDd, B3SoiDdModel, B3SoiDdNodes, B3SoiFd, B3SoiFdModel, B3SoiPd, B3SoiPdModel,
    B3SoiPdNodes, BodyMode, Bsim3v3, Bsim3v3Device, Bsim3v3EquationSet, Bsim3v3Model, Bsim4v8,
    Bsim4v8Device, Bsim4v8Model, Ekv3Device, Ekv3Op, EkvMosfet, Jfet, JfetChannelModel, JfetParams,
    JfetType, MosBodyJunctionModel, MosParams, MosRegion, MosType, Mosfet, MosfetIndices, Vdmos,
    VdmosRegion, VdmosType,
};
pub use passive::{
    Capacitor, CoupledInductorPair, Inductor, InductorCoupling, MultiWindingTransformer, Resistor,
    SaturableInductor, SolutionDependentCapacitor, SolutionDependentCapacitorLinearization,
};
pub use semiconductor::{Bjt, BjtType, Diode, DiodeLevel};

// Re-export from root-level modules
pub use behavioral::{
    BehavioralBranchResolution, BehavioralCurrentSource, BehavioralEvaluationError,
    BehavioralReferenceError, BehavioralReferenceReason, BehavioralSources,
    BehavioralVoltageSource,
};
pub use controlled::{Cccs, Ccvs, Vccs, Vcvs};
pub use coupled_transmission_line::CoupledTransmissionLine;
pub use memristor_pem::{
    XYCE_PEM_DEFAULT_NEGATIVE_TABLE_FILE, XYCE_PEM_DEFAULT_POSITIVE_TABLE_FILE,
    XYCE_PEM_MEMRISTOR_LEVEL, XycePemEvaluationMode, XycePemInstanceParams,
    XycePemLegacyTableParseError, XycePemLegacyTableParseErrorKind, XycePemMemristor,
    XycePemMemristorCache, XycePemMemristorError, XycePemModelParams, XycePemPwlPoint,
    XycePemPwlSample, XycePemPwlTable, XycePemPwlTableError,
    parse_xyce_7_10_legacy_two_column_table,
};
pub(crate) use memristor_pem::{
    XYCE_PEM_MAX_TABLE_BYTES, XYCE_PEM_MAX_TABLE_POINTS,
    parse_xyce_7_10_legacy_two_column_table_bounded,
};
pub use memristor_team::{
    XYCE_TEAM_MEMRISTOR_LEVEL, XYCE_TEAM_RESISTANCE_NOISE_STATE_VERSION, XyceTeamEvaluationMode,
    XyceTeamInstanceParams, XyceTeamMemristor, XyceTeamMemristorCache, XyceTeamMemristorError,
    XyceTeamModelParams, XyceTeamResistanceNoiseParams, XyceTeamStateDrive,
};
pub(crate) use memristor_team::{
    XyceTeamResistanceNoiseCheckpoint, XyceTeamResistanceNoiseRuntime,
};
pub use sources::{CurrentSource, VoltageSource};
pub use switch::{CurrentSwitch, GenericSwitch, SwitchState, VoltageSwitch};
pub use traits::*;
pub(crate) use transmission_line::{
    DISTRIBUTED_RLC_COMPACT_ABSTOL_DEFAULT, DISTRIBUTED_RLC_COMPACT_RELTOL_DEFAULT, LtraRgTwoPort,
    TransmissionLineCheckpoint,
};
pub use transmission_line::{LossyTransmissionLine, TransmissionLine};
pub(crate) use transmission_line::{TlineTransientResponse, TxlTransientStamp};

use crate::Value;

/// Family-neutral Newton cache for native Xyce memristors.
///
/// Rows and columns are ordered `(v_pos, v_neg, x)`. Dynamic charge is
/// integrated by the circuit's private unity-capacitor state binding, so this
/// cache contains only the nonlinear `F` contribution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyceMemristorCache {
    pub current: Value,
    /// Xyce retains the previous typed-store value when a family does not
    /// define an incremental resistance at the current operating point.
    pub resistance: Option<Value>,
    pub residual: [Value; 3],
    pub jacobian: [[Value; 3]; 3],
}

/// Native Xyce memristor equation families supported by the engine.
#[derive(Debug, Clone, PartialEq)]
pub enum XyceMemristor {
    Team(XyceTeamMemristor),
    Pem(XycePemMemristor),
}

impl XyceMemristor {
    pub(crate) fn family_name(&self) -> &'static str {
        match self {
            Self::Team(_) => "TEAM",
            Self::Pem(_) => "PEM",
        }
    }

    pub(crate) fn evaluate(
        &self,
        v_pos: Value,
        v_neg: Value,
        x: Value,
        operating_point: bool,
    ) -> Result<XyceMemristorCache, String> {
        self.evaluate_with_resistance_factor(v_pos, v_neg, x, operating_point, 1.0)
    }

    pub(crate) fn evaluate_with_resistance_factor(
        &self,
        v_pos: Value,
        v_neg: Value,
        x: Value,
        operating_point: bool,
        resistance_factor: Value,
    ) -> Result<XyceMemristorCache, String> {
        match self {
            Self::Team(device) => {
                // TEAM's steady-state row is degenerate unless both threshold
                // exponents are unity, so the kernel solves it where that root
                // is reachable and gauges the state to XON otherwise. See
                // `memristor_team`'s module docs.
                let mode = if operating_point {
                    XyceTeamEvaluationMode::DcOperatingPoint
                } else {
                    XyceTeamEvaluationMode::Dynamic
                };
                let cache = device
                    .evaluate_with_mode_and_resistance_factor(
                        v_pos,
                        v_neg,
                        x,
                        mode,
                        resistance_factor,
                    )
                    .map_err(|error| error.to_string())?;
                Ok(XyceMemristorCache {
                    current: cache.current,
                    resistance: Some(cache.resistance),
                    residual: cache.residual,
                    jacobian: cache.jacobian,
                })
            }
            Self::Pem(device) => {
                debug_assert_eq!(resistance_factor, 1.0);
                let mode = if operating_point {
                    XycePemEvaluationMode::DcOperatingPoint
                } else {
                    XycePemEvaluationMode::Dynamic
                };
                let cache = device
                    .evaluate(v_pos, v_neg, x, mode)
                    .map_err(|error| error.to_string())?;
                let mut residual = cache.residual;
                let mut jacobian = cache.jacobian;
                if !operating_point {
                    // Xyce's PEM source form is F=s, Q=-x.  The shared native
                    // memristor state pipeline integrates a private Q=+x
                    // unity capacitor, so stamp the equivalent -s row.
                    residual[2] = -residual[2];
                    jacobian[2].iter_mut().for_each(|value| *value = -*value);
                }
                Ok(XyceMemristorCache {
                    current: cache.terminal.current,
                    resistance: cache.terminal.incremental_resistance,
                    residual,
                    jacobian,
                })
            }
        }
    }

    pub(crate) fn current_output_with_resistance_factor(
        &self,
        v_pos: Value,
        v_neg: Value,
        x: Value,
        resistance_factor: Value,
    ) -> Result<Value, String> {
        match self {
            Self::Team(device) => device
                .evaluate_with_mode_and_resistance_factor(
                    v_pos,
                    v_neg,
                    x,
                    XyceTeamEvaluationMode::Dynamic,
                    resistance_factor,
                )
                .map(|cache| cache.current)
                .map_err(|error| error.to_string()),
            Self::Pem(device) => device
                .terminal_law(v_pos - v_neg, x)
                .map(|terminal| terminal.current)
                .map_err(|error| error.to_string()),
        }
    }

    pub(crate) fn resistance_output_with_factor(
        &self,
        v_pos: Value,
        v_neg: Value,
        x: Value,
        resistance_factor: Value,
    ) -> Result<Option<Value>, String> {
        match self {
            Self::Team(device) => device
                .resistance_with_factor(x, resistance_factor)
                .map(|(resistance, _)| Some(resistance))
                .map_err(|error| error.to_string()),
            Self::Pem(device) => device
                .terminal_law(v_pos - v_neg, x)
                .map(|terminal| terminal.incremental_resistance)
                .map_err(|error| error.to_string()),
        }
    }
}

/// A device instance in the circuit
// The MOSFET variant dwarfs the passives, but nothing holds a collection of
// these: the circuit stores devices in typed per-family columns (`Capacitors`,
// `Bjts`, `Mosfets`, ...) and this enum exists as a public, one-at-a-time
// description at the API boundary. Boxing the variant would change the shape
// of a public enum to save padding on a value that is never held in bulk.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum Device {
    Resistor(Resistor),
    Capacitor(Capacitor),
    Inductor(Inductor),
    SaturableInductor(SaturableInductor),
    VoltageSource(VoltageSource),
    CurrentSource(CurrentSource),
    Diode(Diode),
    Bjt(Bjt),
    Mosfet(Mosfet),
    Vdmos(Vdmos),
    Jfet(Jfet),
}

impl Device {
    /// Get the device name
    pub fn name(&self) -> &str {
        match self {
            Device::Resistor(d) => &d.name,
            Device::Capacitor(d) => &d.name,
            Device::Inductor(d) => &d.name,
            Device::SaturableInductor(d) => &d.name,
            Device::VoltageSource(d) => &d.name,
            Device::CurrentSource(d) => &d.name,
            Device::Diode(d) => &d.name,
            Device::Bjt(d) => &d.name,
            Device::Mosfet(d) => &d.name,
            Device::Vdmos(d) => &d.name,
            Device::Jfet(d) => &d.name,
        }
    }
}

/// Every capability this engine build offers a distributed model pack.
///
/// A pack declares what it needs in `requires.capabilities`; a client installs
/// it only when that set is a subset of this one. The vocabulary is lowercase
/// kebab case and deliberately coarser than [`Device`]: it names what a pack
/// author must be able to rely on, not how this engine happens to implement
/// it, so an internal refactor never invalidates a published pack.
///
/// The set grows as the engine gains device support — adding an entry is the
/// act that makes packs requiring it installable. A pack that requires a
/// string absent from this list is incompatible with this build and is refused
/// with that reason; it is never silently installed on the theory that the
/// requirement might not matter.
#[must_use]
pub const fn engine_capabilities() -> &'static [&'static str] {
    &[
        "behavioral-source",
        "bjt-gp",
        "capacitor",
        "cccs",
        "ccvs",
        "current-source",
        "diode-level1",
        "inductor",
        "jfet-level1",
        "mosfet-level1",
        "mosfet-vdmos",
        "resistor",
        "subckt",
        "vccs",
        "vcvs",
        "voltage-source",
    ]
}

/// Whether this engine build offers one declared pack capability.
#[must_use]
pub fn engine_supports_capability(capability: &str) -> bool {
    engine_capabilities().contains(&capability)
}

/// Device model parameters
#[derive(Debug, Clone)]
pub struct DeviceModel {
    pub name: String,
    pub model_type: ModelType,
    pub params: std::collections::HashMap<String, Value>,
}

/// Types of device models
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    Resistor,
    Capacitor,
    Inductor,
    SaturableInductor,
    Diode,
    NpnBjt,
    PnpBjt,
    Nmos,
    Pmos,
    NVdmos,
    PVdmos,
    Njf,
    Pjf,
}

impl DeviceModel {
    pub fn new(name: String, model_type: ModelType) -> Self {
        Self {
            name,
            model_type,
            params: std::collections::HashMap::new(),
        }
    }

    pub fn with_param(mut self, key: &str, value: Value) -> Self {
        self.params.insert(key.to_string(), value);
        self
    }

    pub fn get_param(&self, key: &str, default: Value) -> Value {
        self.params.get(key).copied().unwrap_or(default)
    }
}

#[cfg(test)]
mod capability_tests {
    use super::{engine_capabilities, engine_supports_capability};

    #[test]
    fn the_capability_vocabulary_is_sorted_unique_and_kebab_case() {
        let capabilities = engine_capabilities();
        assert!(
            capabilities.windows(2).all(|pair| pair[0] < pair[1]),
            "capabilities must be sorted and unique so a reader can scan them"
        );
        for capability in capabilities {
            let kebab = !capability.is_empty()
                && capability
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
                && !capability.starts_with('-')
                && !capability.ends_with('-')
                && !capability.contains("--");
            assert!(
                kebab,
                "{capability:?} is not the lowercase kebab case a pack manifest can declare"
            );
        }
    }

    #[test]
    fn an_undeclared_capability_is_never_supported() {
        for declared in engine_capabilities() {
            assert!(engine_supports_capability(declared));
        }
        for absent in ["", "SUBCKT", "subckt ", "bsim4", "nonexistent-capability"] {
            assert!(
                !engine_supports_capability(absent),
                "{absent:?} must not be treated as offered"
            );
        }
    }
}
