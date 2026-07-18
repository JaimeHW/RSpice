//! Device models module
//!
//! Provides behavioral models for circuit components, organized into:
//! - `passive/` - Resistors, capacitors, inductors
//! - `semiconductor/` - Diodes, BJTs
//! - `mosfet/` - MOSFETs, VDMOSFETs, JFETs, BSIM4, EKV
//! - `veriloga/` - Verilog-A compiled models (optional feature)
#![allow(clippy::large_enum_variant)]

// Organized device subdirectories
pub mod mosfet;
pub mod passive;
pub mod semiconductor;

// Other devices (kept at root level)
pub mod behavioral;
pub mod controlled;
mod coupled_transmission_line;
mod cpl_native;
pub mod ffi;
pub mod gan_hemt;
pub mod memristor_pem;
pub mod memristor_team;
pub mod model_bypass;
pub mod opamp;
pub mod pwl_file;
mod sources;
mod switch;
pub mod thermal;
mod traits;
mod transmission_line;
pub mod tristate;

// Verilog-A support (feature-gated)
#[cfg(feature = "veriloga")]
pub mod veriloga;

// Generated Verilog-A runtime ABI; the built-in registry remains feature-gated inside the module.
pub mod veriloga_generated;

// Batch device storage for SIMD acceleration (feature-gated)
#[cfg(feature = "simd")]
pub mod batch;

// Re-export from subdirectories for backwards compatibility
pub use mosfet::{
    B3SoiDd, B3SoiDdModel, B3SoiFd, B3SoiFdModel, B3SoiPd, B3SoiPdModel, BodyMode, Bsim3v3,
    Bsim3v3Device, Bsim3v3EquationSet, Bsim3v3Model, Bsim4, Bsim4Params, Bsim4Type, Bsim4v8,
    Bsim4v8Device, Bsim4v8Model, Ekv3Device, Ekv3Op, EkvMosfet, Jfet, JfetChannelModel, JfetParams,
    JfetType, MosBodyJunctionModel, MosParams, MosRegion, MosType, Mosfet, MosfetIndices, Vdmos,
    VdmosRegion, VdmosType,
};
pub use passive::{
    Capacitor, CoupledInductorPair, Inductor, InductorCoupling, MultiWindingTransformer, Resistor,
    SaturableInductor,
};
pub use semiconductor::{Bjt, BjtType, Diode};

// Re-export from root-level modules
pub use behavioral::{BehavioralCurrentSource, BehavioralSources, BehavioralVoltageSource};
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
    XYCE_TEAM_MEMRISTOR_LEVEL, XyceTeamInstanceParams, XyceTeamMemristor, XyceTeamMemristorCache,
    XyceTeamMemristorError, XyceTeamModelParams,
};
pub use sources::{CurrentSource, VoltageSource};
pub use switch::{CurrentSwitch, GenericSwitch, SwitchState, VoltageSwitch};
pub use traits::*;
pub use transmission_line::{LossyTransmissionLine, TransmissionLine};
pub(crate) use transmission_line::{TlineTransientResponse, TxlTransientStamp};

use crate::Value;

/// Family-neutral Newton cache for native Xyce memristors.
///
/// Rows and columns are ordered `(v_pos, v_neg, x)`. Dynamic charge is
/// integrated by the circuit's private unity-capacitor state binding, so this
/// cache contains only the nonlinear `F` contribution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct XyceMemristorCache {
    pub current: Value,
    /// Xyce retains the previous typed-store value when a family does not
    /// define an incremental resistance at the current operating point.
    pub resistance: Option<Value>,
    pub residual: [Value; 3],
    pub jacobian: [[Value; 3]; 3],
}

/// Native Xyce memristor equation families supported by the engine.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum XyceMemristor {
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
        match self {
            Self::Team(device) => {
                let cache = device
                    .evaluate(v_pos, v_neg, x)
                    .map_err(|error| error.to_string())?;
                let mut residual = cache.residual;
                let mut jacobian = cache.jacobian;
                if operating_point
                    && residual[2] == 0.0
                    && jacobian[2].iter().all(|derivative| *derivative == 0.0)
                {
                    // TEAM's threshold deadband has no physical DC equation.
                    // Select x=0 only for that rank-deficient operating-point
                    // row; transient always retains the dynamic equation.
                    residual[2] = x;
                    jacobian[2] = [0.0, 0.0, 1.0];
                }
                Ok(XyceMemristorCache {
                    current: cache.current,
                    resistance: Some(cache.resistance),
                    residual,
                    jacobian,
                })
            }
            Self::Pem(device) => {
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

    pub(crate) fn current_output(
        &self,
        v_pos: Value,
        v_neg: Value,
        x: Value,
    ) -> Result<Value, String> {
        match self {
            Self::Team(device) => device
                .evaluate(v_pos, v_neg, x)
                .map(|cache| cache.current)
                .map_err(|error| error.to_string()),
            Self::Pem(device) => device
                .terminal_law(v_pos - v_neg, x)
                .map(|terminal| terminal.current)
                .map_err(|error| error.to_string()),
        }
    }

    pub(crate) fn resistance_output(
        &self,
        v_pos: Value,
        v_neg: Value,
        x: Value,
    ) -> Result<Option<Value>, String> {
        match self {
            Self::Team(device) => device
                .resistance(x)
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
