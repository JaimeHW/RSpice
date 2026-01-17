//! Device models module
//!
//! Provides behavioral models for circuit components.

mod traits;
mod resistor;
mod capacitor;
mod inductor;
mod saturable_inductor;
mod coupled_inductors;
mod switch;
mod transmission_line;
mod sources;
mod diode;
mod bjt;
mod mosfet;
mod vdmos;
mod jfet;
mod ekv;
pub mod opamp;
pub mod behavioral;
pub mod controlled;

pub use traits::*;
pub use resistor::Resistor;
pub use capacitor::Capacitor;
pub use inductor::Inductor;
pub use saturable_inductor::SaturableInductor;
pub use coupled_inductors::{InductorCoupling, CoupledInductorPair, MultiWindingTransformer};
pub use switch::{VoltageSwitch, CurrentSwitch, SwitchState};
pub use transmission_line::{TransmissionLine, LossyTransmissionLine};
pub use sources::{VoltageSource, CurrentSource};
pub use diode::Diode;
pub use bjt::{Bjt, BjtType};
pub use mosfet::{Mosfet, MosType, MosRegion};
pub use vdmos::{Vdmos, VdmosType, VdmosRegion};
pub use jfet::{Jfet, JfetType, JfetParams};
pub use ekv::EkvMosfet;
pub use behavioral::{BehavioralVoltageSource, BehavioralCurrentSource, BehavioralSources};
pub use controlled::{Vcvs, Vccs, Cccs, Ccvs};

use crate::Value;

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
