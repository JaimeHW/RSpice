//! Elaboration of LTspice's NIGBT and PIGBT transistors.
//!
//! The device equations are Hefner and Diebolt's (IEEE Trans. Power
//! Electronics 9(5), 1994, Table I), without the carrier-scattering mobility
//! and velocity-saturation terms their text adds. On top of those LTspice
//! rounds the channel's overdrive with `subthres`, scales `KP` as `T^-1.5` and
//! moves `Vt` by -1 mV/K. Each of these was measured against batch runs of
//! LTspice 26.1.1, which the tests pin.
//!
//! Between its terminals (anode `a`, gate `g`, cathode `k`) the device has an
//! internal drain `d` and emitter `e`. The base charge `Q` is what keeps the
//! emitter junction at its voltage; the device solves for it on every
//! evaluation, so `Q` is never an unknown of the circuit.

use super::*;
use crate::xspice::{
    CmContext, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortDirection, PortSpec, PortType,
    XspiceInstance,
};
use num_complex::Complex64;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::sync::{Arc, OnceLock};

const ELECTRON_CHARGE: Value = 1.6e-19;
const SILICON_PERMITTIVITY: Value = 1.05e-12;
const INTRINSIC_DENSITY: Value = 1.45e10;
const BOLTZMANN_OVER_CHARGE: Value = 8.617333262e-5;
const JUNCTION_BUILT_IN: Value = 0.6;
const CELSIUS_OFFSET: Value = 273.15;
/// The channel's floor, so a drain with no current path still has a voltage.
const CHANNEL_LEAK: Value = 1e-12;
/// The ratio of drain voltage to breakdown that avalanche gain stops at.
const AVALANCHE_LIMIT: Value = 0.999;
/// How far, in thermal voltages, a junction may rise above its last voltage
/// before its Newton trial is compressed.
const ITERATE_WINDOW: Value = 2.0;
const TRANSIENT_WINDOW: Value = 10.0;

const TERMINALS: [&str; 5] = ["a", "g", "k", "d", "e"];
const TERMINAL_COUNT: usize = TERMINALS.len();

const VEB: usize = 0;
const VDS: usize = 1;
const VGS: usize = 2;
const VAE: usize = 3;
const BIAS_COUNT: usize = 4;
/// The derivative slot that stands for `Q` while its own slopes are found.
const CHARGE: usize = BIAS_COUNT;
const SLOPES: usize = BIAS_COUNT + 1;

mod slot {
    use super::{BIAS_COUNT, TERMINAL_COUNT};

    pub(super) const STORED: usize = 5;
    pub(super) const HISTORY: usize = 3 * STORED;
    pub(super) const LAST_VEB: usize = HISTORY;
    pub(super) const CONDUCTANCE: usize = LAST_VEB + 1;
    pub(super) const CAPACITANCE: usize = CONDUCTANCE + TERMINAL_COUNT * BIAS_COUNT;
    pub(super) const COUNT: usize = CAPACITANCE + TERMINAL_COUNT * BIAS_COUNT;
}

/// The stored quantities, each integrated with the circuit's companion form.
#[derive(Clone, Copy)]
enum Stored {
    BaseCharge,
    DrainCharge,
    GateCharge,
    DrainGate,
    EmitterCathode,
}

/// A value and its slopes with respect to the bias and, while `Q` is being
/// differentiated, to `Q`.
#[derive(Clone, Copy, Debug)]
struct Dual {
    value: Value,
    slope: [Value; SLOPES],
}

impl Dual {
    fn constant(value: Value) -> Self {
        Self {
            value,
            slope: [0.0; SLOPES],
        }
    }

    fn seeded(value: Value, index: usize) -> Self {
        let mut slope = [0.0; SLOPES];
        slope[index] = 1.0;
        Self { value, slope }
    }

    fn chain(self, value: Value, derivative: Value) -> Self {
        Self {
            value,
            slope: self.slope.map(|slope| slope * derivative),
        }
    }

    fn ln(self) -> Self {
        self.chain(self.value.ln(), 1.0 / self.value)
    }

    fn exp(self) -> Self {
        let value = self.value.exp();
        self.chain(value, value)
    }

    fn sqrt(self) -> Self {
        let value = self.value.sqrt();
        self.chain(value, 0.5 / value)
    }

    fn sinh(self) -> Self {
        self.chain(self.value.sinh(), self.value.cosh())
    }

    fn tanh(self) -> Self {
        let value = self.value.tanh();
        self.chain(value, 1.0 - value * value)
    }

    fn atanh(self) -> Self {
        self.chain(self.value.atanh(), 1.0 / (1.0 - self.value * self.value))
    }

    fn powf(self, exponent: Value) -> Self {
        self.chain(
            self.value.powf(exponent),
            exponent * self.value.powf(exponent - 1.0),
        )
    }

    fn square(self) -> Self {
        self * self
    }

    fn at_least(self, floor: Value) -> Self {
        if self.value >= floor {
            self
        } else {
            Self::constant(floor)
        }
    }

    fn at_most(self, ceiling: Value) -> Self {
        if self.value <= ceiling {
            self
        } else {
            Self::constant(ceiling)
        }
    }
}

impl Add for Dual {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut slope = self.slope;
        for (slope, other) in slope.iter_mut().zip(other.slope) {
            *slope += other;
        }
        Self {
            value: self.value + other.value,
            slope,
        }
    }
}

impl Add<Value> for Dual {
    type Output = Self;
    fn add(self, other: Value) -> Self {
        Self {
            value: self.value + other,
            slope: self.slope,
        }
    }
}

impl Neg for Dual {
    type Output = Self;
    fn neg(self) -> Self {
        self.chain(-self.value, -1.0)
    }
}

impl Sub for Dual {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl Sub<Value> for Dual {
    type Output = Self;
    fn sub(self, other: Value) -> Self {
        self + (-other)
    }
}

impl Mul for Dual {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut slope = [0.0; SLOPES];
        for (index, slope) in slope.iter_mut().enumerate() {
            *slope = self.slope[index] * other.value + other.slope[index] * self.value;
        }
        Self {
            value: self.value * other.value,
            slope,
        }
    }
}

impl Mul<Value> for Dual {
    type Output = Self;
    fn mul(self, other: Value) -> Self {
        self.chain(self.value * other, other)
    }
}

impl Div for Dual {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let value = self.value / other.value;
        let mut slope = [0.0; SLOPES];
        for (index, slope) in slope.iter_mut().enumerate() {
            *slope = (self.slope[index] - value * other.slope[index]) / other.value;
        }
        Self { value, slope }
    }
}

impl Div<Value> for Dual {
    type Output = Self;
    fn div(self, other: Value) -> Self {
        self.chain(self.value / other, 1.0 / other)
    }
}

/// Branch quantities between the device's nodes, named for their ends.
#[derive(Clone, Copy)]
struct Branches<T> {
    anode_emitter: T,
    emitter_cathode: T,
    emitter_drain: T,
    drain_cathode: T,
    drain_gate: T,
    gate_cathode: T,
}

impl Branches<Dual> {
    /// The current each terminal draws from the circuit, in `TERMINALS` order.
    fn terminals(&self) -> [Dual; TERMINAL_COUNT] {
        [
            self.anode_emitter,
            self.gate_cathode - self.drain_gate,
            -(self.emitter_cathode + self.drain_cathode + self.gate_cathode),
            self.drain_cathode + self.drain_gate - self.emitter_drain,
            self.emitter_cathode + self.emitter_drain - self.anode_emitter,
        ]
    }
}

/// What the static equations hand the stored ones.
struct Evaluation {
    currents: Branches<Dual>,
    base_charge: Dual,
    avalanche: Dual,
    redistribution: Dual,
    drain_charge: Dual,
    drain_gate: Dual,
}

#[derive(Clone, Copy, Debug)]
struct IgbtLaw {
    polarity: Value,
    multiplier: Value,
    area: Value,
    drain_area: Value,
    base_width: Value,
    doping: Value,
    lifetime: Value,
    electron_mobility: Value,
    hole_mobility: Value,
    emitter_saturation: Value,
    transconductance: Value,
    triode_factor: Value,
    transverse_field: Value,
    threshold: Value,
    overlap_threshold: Value,
    subthreshold: Value,
    breakdown: Value,
    breakdown_exponent: Value,
    gate_capacitance: Value,
    overlap_oxide: Value,
    overlap_area: Value,
    thermal_voltage: Value,
    diffusion_length: Value,
    ambipolar_diffusivity: Value,
    junction_critical: Value,
}

impl IgbtLaw {
    fn from_card(
        owner: &str,
        card: &crate::netlist::ModelDef,
        instance_params: &[(String, Value)],
        ambient_kelvin: Value,
    ) -> Result<Self, SimulationError> {
        const READ: [&str; 21] = [
            "AGD", "AREA", "BVF", "BVN", "CGS", "COXD", "JSNE", "KF", "KP", "MUN", "MUP", "NB",
            "TAU", "THETA", "VT", "VTD", "WB", "SUBTHRES", "KFN", "AFN", "TNOM",
        ];
        for (name, _) in &card.params {
            if !READ.iter().any(|known| name.eq_ignore_ascii_case(known)) {
                return Err(SimulationError::Circuit(format!(
                    "{owner}: an IGBT model does not read {name}"
                )));
            }
        }
        const INSTANCE: [&str; 5] = ["AREA", "M", "MULT", "TEMP", "OFF"];
        for (name, _) in instance_params {
            if !INSTANCE
                .iter()
                .any(|known| name.eq_ignore_ascii_case(known))
            {
                return Err(SimulationError::Circuit(format!(
                    "{owner}: an IGBT does not read {name}"
                )));
            }
        }
        let read = |key: &str, default: Value| {
            card.params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(key))
                .map_or(default, |(_, value)| *value)
        };
        let positive = |key: &str, value: Value| {
            if value > 0.0 {
                Ok(value)
            } else {
                Err(SimulationError::Circuit(format!(
                    "{owner}: an IGBT's {key} must be positive"
                )))
            }
        };
        let square_metres = 1e4;
        let area_m2 = instance_param(instance_params, &["AREA"]).unwrap_or(read("AREA", 1e-5));
        let area = positive("area", area_m2)? * square_metres;
        let overlap_area = positive("Agd", read("AGD", 5e-6))? * square_metres;
        if overlap_area >= area {
            return Err(SimulationError::Circuit(format!(
                "{owner}: an IGBT's Agd must be less than its area"
            )));
        }
        let base_width = positive("WB", read("WB", 9e-5))? * 100.0;
        let doping = positive("NB", read("NB", 2e14))?;
        let lifetime = positive("Tau", read("TAU", 7.1e-6))?;
        let electron_mobility = positive("MUN", read("MUN", 1500.0))?;
        let hole_mobility = positive("MUP", read("MUP", 450.0))?;
        let triode_factor = positive("KF", read("KF", 1.0))?;
        let subthreshold = positive("subthres", read("SUBTHRES", 0.02))?;
        let breakdown_factor = positive("BVF", read("BVF", 1.0))?;
        let kelvin = instance_param(instance_params, &["TEMP"])
            .map_or(ambient_kelvin, |celsius| celsius + CELSIUS_OFFSET);
        let nominal = read("TNOM", 27.0) + CELSIUS_OFFSET;
        let thermal_voltage = BOLTZMANN_OVER_CHARGE * kelvin;
        let ambipolar_diffusivity = 2.0 * thermal_voltage * electron_mobility * hole_mobility
            / (electron_mobility + hole_mobility);
        let diffusion_length = (ambipolar_diffusivity * lifetime).sqrt();
        let emitter_saturation = positive("Jsne", read("JSNE", 6.5e-13))? * area;
        let saturation_scale = ELECTRON_CHARGE * area * diffusion_length * INTRINSIC_DENSITY
            / lifetime
            * INTRINSIC_DENSITY
            / doping;
        let saturation = saturation_scale * (base_width / (2.0 * diffusion_length)).tanh();
        Ok(Self {
            polarity: if card.model_type.eq_ignore_ascii_case("PIGBT") {
                -1.0
            } else {
                1.0
            },
            multiplier: instance_param(instance_params, &["M", "MULT"]).unwrap_or(1.0),
            area,
            drain_area: area - overlap_area,
            base_width,
            doping,
            lifetime,
            electron_mobility,
            hole_mobility,
            emitter_saturation,
            transconductance: positive("KP", read("KP", 0.38))? * (kelvin / nominal).powf(-1.5),
            triode_factor,
            transverse_field: read("THETA", 0.02),
            threshold: read("VT", 4.7) - 1e-3 * (kelvin - nominal),
            overlap_threshold: read("VTD", 1e-3),
            subthreshold,
            breakdown: breakdown_factor * 5.34e13 * doping.powf(-0.75),
            breakdown_exponent: positive("BVN", read("BVN", 4.0))?,
            gate_capacitance: read("CGS", 1.24e-8) * (area - overlap_area),
            overlap_oxide: positive("Coxd", read("COXD", 3.5e-8))? * overlap_area,
            overlap_area,
            thermal_voltage,
            diffusion_length,
            ambipolar_diffusivity,
            junction_critical: thermal_voltage
                * (thermal_voltage / (std::f64::consts::SQRT_2 * saturation)).ln(),
        })
    }

    /// `2 * epsilon / (q * NB)`: a depletion width squared per volt.
    fn depletion_per_volt(&self) -> Value {
        2.0 * SILICON_PERMITTIVITY / (ELECTRON_CHARGE * self.doping)
    }

    fn built_in_charge(&self) -> Value {
        self.area
            * (2.0 * SILICON_PERMITTIVITY * ELECTRON_CHARGE * self.doping * JUNCTION_BUILT_IN)
                .sqrt()
    }

    /// `2 * q * NB * epsilon * A^2`: the depletion charge's scale in volts.
    fn depletion_charge_scale(&self) -> Value {
        2.0 * ELECTRON_CHARGE * self.doping * SILICON_PERMITTIVITY * self.area * self.area
    }

    fn dember_scale(&self) -> Value {
        self.ambipolar_diffusivity / self.electron_mobility
    }

    fn depletion_width(&self, vds: Dual) -> Dual {
        ((vds + JUNCTION_BUILT_IN).at_least(1e-3) * self.depletion_per_volt()).sqrt()
    }

    fn neutral_width(&self, depletion: Dual) -> Dual {
        (Dual::constant(self.base_width) - depletion).at_least(1e-3 * self.base_width)
    }

    /// The charge of one carrier per cubic centimetre at the emitter end.
    fn charge_per_density(&self, width: Dual) -> Dual {
        (width / (2.0 * self.diffusion_length)).tanh()
            * (ELECTRON_CHARGE * self.area * self.diffusion_length)
    }

    fn diffusion_voltage(&self, density: Dual) -> Dual {
        let product = (density / (INTRINSIC_DENSITY * INTRINSIC_DENSITY) + 1.0 / self.doping)
            * (density + self.doping);
        product.ln() * self.thermal_voltage
            - ((density + self.doping) / self.doping).ln() * self.dember_scale()
    }

    /// The emitter junction's voltage for a base charge.
    fn junction_voltage(&self, charge: Dual, width: Dual) -> Dual {
        if charge.value < 0.0 {
            -(charge - self.built_in_charge()).square() / self.depletion_charge_scale()
                + JUNCTION_BUILT_IN
        } else {
            self.diffusion_voltage(charge / self.charge_per_density(width))
        }
    }

    /// The carrier density at which the diffusion voltage reaches `veb`.
    fn density_for(&self, veb: Value) -> Value {
        let doping = self.doping;
        let intrinsic2 = INTRINSIC_DENSITY * INTRINSIC_DENSITY;
        let dember = self.dember_scale();
        let thermal = self.thermal_voltage;
        let residual = |density: Value| {
            thermal * ((density / intrinsic2 + 1.0 / doping) * (density + doping)).ln()
                - dember * ((density + doping) / doping).ln()
                - veb
        };
        let per_log = |density: Value| {
            let outer = density / intrinsic2 + 1.0 / doping;
            density
                * (thermal * ((1.0 / intrinsic2) / outer + 1.0 / (density + doping))
                    - dember / (density + doping))
        };
        let low_injection = intrinsic2 / doping * (veb / thermal).exp();
        let high_injection = INTRINSIC_DENSITY * (veb / (2.0 * thermal)).exp();
        let mut density = low_injection.min(high_injection).max(f64::MIN_POSITIVE);
        let (mut low, mut high) = (0.0_f64, f64::INFINITY);
        for _ in 0..80 {
            let error = residual(density);
            if error.abs() <= 1e-14 * (veb + thermal) {
                break;
            }
            if error > 0.0 {
                high = density;
            } else {
                low = density;
            }
            let next = density * (-error / per_log(density)).exp();
            density = if next.is_finite() && next > low && next < high {
                next
            } else if high.is_finite() && low > 0.0 {
                (low * high).sqrt()
            } else if error > 0.0 {
                density * 1e-3
            } else {
                density * 1e3
            };
        }
        density
    }

    fn solved_charge(&self, veb: Value, width: Dual) -> Value {
        if veb <= 0.0 {
            self.built_in_charge()
                - ((JUNCTION_BUILT_IN - veb) * self.depletion_charge_scale()).sqrt()
        } else {
            self.density_for(veb) * self.charge_per_density(width).value
        }
    }

    /// The base charge at this bias, with its slopes over the bias.
    fn base_charge(&self, veb: Dual, width: Dual) -> Dual {
        let solved = self.solved_charge(veb.value, width);
        let error = self.junction_voltage(Dual::seeded(solved, CHARGE), width) - veb;
        let mut slope = [0.0; SLOPES];
        for (index, slope) in slope.iter_mut().enumerate().take(BIAS_COUNT) {
            *slope = -error.slope[index] / error.slope[CHARGE];
        }
        Dual {
            value: solved,
            slope,
        }
    }

    fn base_resistance(&self, charge: Dual, width: Dual, background: Dual) -> Dual {
        let scale = ELECTRON_CHARGE * self.area;
        if charge.value < 0.0 {
            return width / (self.electron_mobility * self.doping * scale);
        }
        let mobility = charge / (charge + background) * self.hole_mobility + self.electron_mobility;
        let density = charge / self.charge_per_density(width);
        let half = width / (2.0 * self.diffusion_length);
        let hyperbolic = (width / self.diffusion_length).sinh();
        let tangent = half.tanh();
        let root = ((density / hyperbolic).square() + self.doping * self.doping).sqrt();
        let argument = root * tangent / (density / hyperbolic * tangent + self.doping);
        let effective = half * root / argument.atanh();
        width / (mobility * effective * scale)
    }

    fn overdrive(&self, vgs: Dual) -> Dual {
        let excess = vgs - self.threshold;
        if excess.value > 0.0 {
            excess + ((excess / -self.subthreshold).exp() + 1.0).ln() * self.subthreshold
        } else {
            ((excess / self.subthreshold).exp() + 1.0).ln() * self.subthreshold
        }
    }

    fn channel_current(&self, vgs: Dual, vds: Dual) -> Dual {
        let overdrive = self.overdrive(vgs);
        let factor = self.triode_factor;
        let current = if vds.value <= overdrive.value / factor {
            (overdrive * vds - vds.square() * (0.5 * factor)) * (factor * self.transconductance)
        } else {
            overdrive.square() * (0.5 * self.transconductance)
        };
        current / (overdrive * self.transverse_field + 1.0) + vds * CHANNEL_LEAK
    }

    fn static_equations(&self, bias: [Dual; BIAS_COUNT]) -> Evaluation {
        let [veb, vds, vgs, vae] = bias;
        let depletion = self.depletion_width(vds);
        let width = self.neutral_width(depletion);
        let background = width * (ELECTRON_CHARGE * self.area * self.doping);
        let charge = self.base_charge(veb, width);
        let stored = charge.at_least(0.0);
        let intrinsic2 = INTRINSIC_DENSITY * INTRINSIC_DENSITY;
        let base_current = stored / self.lifetime
            + (stored / background).square()
                * (4.0 * self.doping * self.doping / intrinsic2 * self.emitter_saturation);
        let anode_current = vae / self.base_resistance(charge, width, background);
        let ratio = self.electron_mobility / self.hole_mobility;
        let hole_diffusivity = self.hole_mobility * self.thermal_voltage;
        let collector_current = anode_current / (1.0 + ratio)
            + stored / width.square() * (ratio / (1.0 + ratio) * 4.0 * hole_diffusivity);
        let channel = self.channel_current(vgs, vds);
        let junction = vds.at_least(0.0);
        let avalanche = (Dual::constant(1.0)
            - (junction / self.breakdown)
                .at_most(AVALANCHE_LIMIT)
                .powf(self.breakdown_exponent))
        .powf(-1.0);
        let generation = ((junction + 1e-9) * self.depletion_per_volt()).sqrt()
            * (ELECTRON_CHARGE * INTRINSIC_DENSITY * self.area / self.lifetime);
        let multiplication =
            (avalanche - 1.0) * (channel + collector_current) + avalanche * generation;
        let redistribution =
            stored * (self.area * SILICON_PERMITTIVITY) / depletion / (background * 3.0);
        let drain_charge = (vds + JUNCTION_BUILT_IN).at_least(1e-3).sqrt()
            * (self.drain_area
                * (2.0 * SILICON_PERMITTIVITY * ELECTRON_CHARGE * self.doping).sqrt());
        let oxide = self.overlap_oxide;
        let drain_gate = if vds.value <= vgs.value - self.overlap_threshold {
            Dual::constant(oxide)
        } else {
            let overlap = ((vds - vgs + self.overlap_threshold).at_least(1e-3)
                * self.depletion_per_volt())
            .sqrt();
            (overlap / (self.overlap_area * SILICON_PERMITTIVITY) + 1.0 / oxide).powf(-1.0)
        };
        Evaluation {
            currents: Branches {
                anode_emitter: anode_current,
                emitter_cathode: collector_current,
                emitter_drain: base_current,
                drain_cathode: channel + multiplication,
                drain_gate: Dual::constant(0.0),
                gate_cathode: Dual::constant(0.0),
            },
            base_charge: charge,
            avalanche,
            redistribution,
            drain_charge,
            drain_gate,
        }
    }

    /// The stored quantities in `Stored` order.
    fn stored_values(&self, device: &Evaluation, bias: [Dual; BIAS_COUNT]) -> [Dual; slot::STORED] {
        let [veb, vds, vgs, _] = bias;
        [
            device.base_charge,
            device.drain_charge,
            vgs * self.gate_capacitance,
            vds - vgs,
            veb + vds,
        ]
    }

    /// Add the stored currents, given each stored quantity's rate of change.
    fn with_rates(&self, device: &Evaluation, rates: [Dual; slot::STORED]) -> Branches<Dual> {
        let rate = |quantity: Stored| rates[quantity as usize];
        let redistribution = device.redistribution * rate(Stored::EmitterCathode);
        let currents = device.currents;
        Branches {
            emitter_cathode: currents.emitter_cathode + redistribution,
            emitter_drain: currents.emitter_drain + rate(Stored::BaseCharge),
            drain_cathode: currents.drain_cathode
                + rate(Stored::DrainCharge)
                + (device.avalanche - 1.0) * redistribution,
            drain_gate: device.drain_gate * rate(Stored::DrainGate),
            gate_cathode: rate(Stored::GateCharge),
            ..currents
        }
    }
}

/// Keep a Newton trial on the junction's exponential from overshooting: past
/// `window` above `old` the voltage grows only logarithmically.
fn limit_junction(new: Value, old: Value, window: Value, thermal: Value, critical: Value) -> Value {
    let base = old.max(0.0);
    if new <= critical || new - base <= window {
        return new;
    }
    let knee = (base + window).max(critical);
    if new <= knee {
        return new;
    }
    knee + thermal * (1.0 + (new - knee) / thermal).ln()
}

fn is_igbt_type(model_type: &str) -> bool {
    model_type.eq_ignore_ascii_case("NIGBT") || model_type.eq_ignore_ascii_case("PIGBT")
}

/// Build the IGBT `element` names, when its card is one.
pub(super) fn build_if_igbt(
    circuit: &mut CircuitData,
    element: &Element,
    card: Option<&crate::netlist::ModelDef>,
    instance_params: &[(String, Value)],
    temperature: Value,
) -> Result<bool, SimulationError> {
    let Some(card) = card.filter(|card| is_igbt_type(&card.model_type)) else {
        return Ok(false);
    };
    let law = IgbtLaw::from_card(&element.name, card, instance_params, temperature)?;
    let model: Arc<dyn CodeModel> = Arc::new(Igbt { law });
    let ports = [
        element.nodes[0].clone(),
        element.nodes[1].clone(),
        element.nodes[2].clone(),
        format!("{}.drain", element.name),
        format!("{}.emitter", element.name),
    ]
    .map(XspicePort::Analog);
    let connections =
        coerce_xspice_connections(circuit, model.ports(), &ports, &element.name, model.name())?;
    let mut instance =
        XspiceInstance::new(element.name.clone(), model, connections, &[], &[], &[], &[]).map_err(
            |error| xspice_creation_error(format!("Failed to create {}", element.name), error),
        )?;
    instance.set_temperature(temperature);
    assign_xspice_output_branches(circuit, &mut instance, &element.name)?;
    instance.init().map_err(|error| {
        SimulationError::Circuit(format!("Failed to initialize {}: {error}", element.name))
    })?;
    circuit.add_xspice_instance(instance);
    Ok(true)
}

struct Igbt {
    law: IgbtLaw,
}

impl Igbt {
    /// The slope of one terminal's current over each node voltage, from its
    /// slopes over the bias differences.
    fn node_slopes(slopes: [Value; BIAS_COUNT]) -> [(&'static str, Value); TERMINAL_COUNT] {
        [
            ("a", slopes[VAE]),
            ("g", slopes[VGS]),
            ("k", -slopes[VDS] - slopes[VGS]),
            ("d", slopes[VDS] - slopes[VEB]),
            ("e", slopes[VEB] - slopes[VAE]),
        ]
    }

    fn stored_slopes(ctx: &CmContext, first: usize, terminal: usize) -> [Value; BIAS_COUNT] {
        std::array::from_fn(|bias| ctx.state(first + terminal * BIAS_COUNT + bias))
    }
}

fn terminal_index(port: &str) -> Option<usize> {
    TERMINALS
        .iter()
        .position(|terminal| port.eq_ignore_ascii_case(terminal))
}

impl CodeModel for Igbt {
    fn name(&self) -> &str {
        "igbt"
    }

    fn description(&self) -> &str {
        "Insulated-gate bipolar transistor with Hefner's base charge equations"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            TERMINALS
                .iter()
                .map(|name| PortSpec {
                    name: (*name).to_string(),
                    direction: PortDirection::InOut,
                    default_type: PortType::Conductance,
                    allowed_types: vec![PortType::Conductance],
                    is_vector: false,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                    description: String::new(),
                })
                .collect()
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(slot::COUNT);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let law = &self.law;
        let sign = law.polarity;
        let voltage = |ctx: &CmContext, port: &str| ctx.input(port);
        let (a, g, k, d, e) = (
            voltage(ctx, "a"),
            voltage(ctx, "g"),
            voltage(ctx, "k"),
            voltage(ctx, "d"),
            voltage(ctx, "e"),
        );
        let raw_veb = sign * (e - d);
        // A transient's Newton trials are rolled back, so the last voltage
        // there is the accepted one and the free window has to cover a step.
        let window = if ctx.is_transient() {
            TRANSIENT_WINDOW
        } else {
            ITERATE_WINDOW
        };
        let veb = limit_junction(
            raw_veb,
            ctx.state(slot::LAST_VEB),
            window * law.thermal_voltage,
            law.thermal_voltage,
            law.junction_critical,
        );
        let bias = [
            Dual::seeded(veb, VEB),
            Dual::seeded(sign * (d - k), VDS),
            Dual::seeded(sign * (g - k), VGS),
            Dual::seeded(sign * (a - e), VAE),
        ];
        let device = law.static_equations(bias);
        let stored = law.stored_values(&device, bias);
        let commits = ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe;

        let dt = ctx.timestep;
        let dynamic = ctx.is_transient() && dt.is_finite() && dt > 0.0;
        let branches = if dynamic {
            let coefficients = ctx.transient_companion_coefficients();
            let conductance = coefficients.capacitor_geq(1.0, dt);
            let mut rates = [Dual::constant(0.0); slot::STORED];
            for (index, value) in stored.iter().enumerate() {
                let value_slot = 3 * index;
                let (previous, older, rate) = (
                    ctx.state_prev(value_slot),
                    ctx.state_prev(value_slot + 1),
                    ctx.state_prev(value_slot + 2),
                );
                let history = coefficients.capacitor_ieq(1.0, dt, previous, older, rate);
                rates[index] = *value * conductance - history;
                if commits {
                    ctx.set_state(value_slot, value.value);
                    ctx.set_state(value_slot + 1, previous);
                    ctx.set_state(value_slot + 2, rates[index].value);
                }
            }
            law.with_rates(&device, rates)
        } else {
            if commits {
                // The bias point is the history a transient starts from.
                for (index, value) in stored.iter().enumerate() {
                    ctx.set_initial_state(3 * index, value.value);
                    ctx.set_initial_state(3 * index + 1, value.value);
                    ctx.set_initial_state(3 * index + 2, 0.0);
                }
            }
            device.currents
        };

        if commits {
            ctx.set_state(slot::LAST_VEB, veb);
        }
        // `with_rates` takes the stored quantities' slopes through the rates,
        // so a rate of the slope itself reads the capacitance out of the same
        // equations when the conductance would otherwise be zero.
        let per_slope = law.with_rates(
            &device,
            stored.map(|value| Dual {
                value: 0.0,
                ..value
            }),
        );
        let reactive = per_slope.terminals();
        let terminals = branches.terminals();
        let static_terminals = device.currents.terminals();
        let rows = TERMINALS.map(|port| ctx.port_node(port).unwrap_or(0).checked_sub(1));
        let voltages = [a, g, k, d, e];
        for (terminal, current) in terminals.iter().enumerate() {
            let at_trial = current.value + current.slope[VEB] * (raw_veb - veb);
            let through = sign * law.multiplier * at_trial;
            ctx.set_output(TERMINALS[terminal], through);
            if let Some(row) = rows[terminal] {
                let slopes: [Value; BIAS_COUNT] =
                    std::array::from_fn(|bias| law.multiplier * current.slope[bias]);
                let mut equivalent = through;
                for (column, (_, slope)) in Self::node_slopes(slopes).into_iter().enumerate() {
                    equivalent -= slope * voltages[column];
                    if let Some(column) = rows[column] {
                        ctx.stamp_conductance(row, column, slope);
                    }
                }
                ctx.stamp_rhs(row, -equivalent);
            }
            for bias in 0..BIAS_COUNT {
                let index = terminal * BIAS_COUNT + bias;
                ctx.set_state(
                    slot::CONDUCTANCE + index,
                    law.multiplier * current.slope[bias],
                );
                ctx.set_state(
                    slot::CAPACITANCE + index,
                    law.multiplier
                        * (reactive[terminal].slope[bias] - static_terminals[terminal].slope[bias]),
                );
            }
        }
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        let Some(terminal) = terminal_index(output_port) else {
            return Vec::new();
        };
        Self::node_slopes(Self::stored_slopes(ctx, slot::CONDUCTANCE, terminal))
            .map(|(port, slope)| (port.to_string(), slope))
            .to_vec()
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        let Some(terminal) = terminal_index(output_port) else {
            return Vec::new();
        };
        let omega = 2.0 * std::f64::consts::PI * frequency;
        let real = Self::node_slopes(Self::stored_slopes(ctx, slot::CONDUCTANCE, terminal));
        let imaginary = Self::node_slopes(Self::stored_slopes(ctx, slot::CAPACITANCE, terminal));
        real.iter()
            .zip(imaginary)
            .map(|((port, conductance), (_, capacitance))| {
                (
                    (*port).to_string(),
                    Complex64::new(*conductance, omega * capacitance),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(pairs: &[(&str, Value)]) -> crate::netlist::ModelDef {
        crate::netlist::ModelDef {
            name: "N1".to_owned(),
            model_type: "NIGBT".to_owned(),
            params: pairs
                .iter()
                .map(|(name, value)| ((*name).to_owned(), *value))
                .collect(),
            expr_params: Vec::new(),
            string_params: Vec::new(),
            string_vector_params: Vec::new(),
            real_vector_params: Vec::new(),
            real_vector_expr_params: Vec::new(),
            integer_vector_params: Vec::new(),
        }
    }

    fn law() -> IgbtLaw {
        IgbtLaw::from_card("Z1", &card(&[]), &[], 300.15).expect("a default card")
    }

    #[test]
    fn the_base_charge_solves_the_junction_voltage_with_its_slopes() {
        let law = law();
        let width = Dual::constant(8e-3);
        for veb in [-5.0, 0.1, 0.5, 0.75, 1.0] {
            let charge = law.base_charge(Dual::seeded(veb, VEB), width);
            let back = law
                .junction_voltage(Dual::constant(charge.value), width)
                .value;
            assert!((back - veb).abs() < 1e-9, "Veb {veb}: {back}");
            let step = 1e-6;
            let nudged = law.base_charge(Dual::seeded(veb + step, VEB), width);
            let numeric = (nudged.value - charge.value) / step;
            assert!(
                (charge.slope[VEB] - numeric).abs() <= 1e-3 * numeric.abs(),
                "Veb {veb}: {} vs {numeric}",
                charge.slope[VEB]
            );
        }
    }

    #[test]
    fn the_static_equations_carry_exact_slopes() {
        let law = law();
        let at = |values: [Value; BIAS_COUNT]| {
            let bias: [Dual; BIAS_COUNT] =
                std::array::from_fn(|index| Dual::seeded(values[index], index));
            law.static_equations(bias).currents.terminals()
        };
        let point = [0.75, 9.0, 10.0, 0.08];
        let base = at(point);
        for bias in 0..BIAS_COUNT {
            let step = 1e-7;
            let (mut high, mut low) = (point, point);
            high[bias] += step;
            low[bias] -= step;
            for terminal in 0..TERMINAL_COUNT {
                let numeric = (at(high)[terminal].value - at(low)[terminal].value) / (2.0 * step);
                let exact = base[terminal].slope[bias];
                assert!(
                    (exact - numeric).abs() <= (1e-4 * numeric.abs()).max(1e-6),
                    "terminal {terminal} bias {bias}: {exact} vs {numeric}"
                );
            }
        }
    }

    #[test]
    fn the_terminal_currents_sum_to_zero() {
        let law = law();
        let bias = [
            Dual::seeded(0.75, VEB),
            Dual::seeded(9.0, VDS),
            Dual::seeded(10.0, VGS),
            Dual::seeded(0.08, VAE),
        ];
        let device = law.static_equations(bias);
        let rates = law
            .stored_values(&device, bias)
            .map(|value| value * 1e7 - 3.0);
        let total = law
            .with_rates(&device, rates)
            .terminals()
            .into_iter()
            .fold(0.0, |sum, current| sum + current.value);
        assert!(total.abs() < 1e-9, "{total}");
    }

    #[test]
    fn a_card_is_refused_for_what_it_cannot_mean() {
        let refusal = IgbtLaw::from_card("Z1", &card(&[("BOGUS", 1.0)]), &[], 300.15)
            .expect_err("an unread parameter");
        assert!(refusal.to_string().contains("BOGUS"), "{refusal}");
        let refusal = IgbtLaw::from_card("Z1", &card(&[("AGD", 1.0)]), &[], 300.15)
            .expect_err("an overlap larger than the die");
        assert!(refusal.to_string().contains("Agd"), "{refusal}");
    }
}
