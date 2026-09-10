//! Simulator parameters shared by interpreted, JIT and generated models.

use crate::Value;

/// Numeric simulator queries supported by the runtime contract.
///
/// A recognized name may still be unavailable in a particular analysis. The
/// availability bit, rather than a numeric sentinel, decides whether to use a
/// model's fallback. The order is also the native/Wasm query-slot order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum SimulationParameter {
    Gmin,
    Pnjmaxi,
    Tnom,
    SimulatorVersion,
    SimulatorSubversion,
    Gdev,
    Imax,
    Imelt,
    Iteration,
    Scale,
    Shrink,
    SourceScaleFactor,
    TimeUnit,
    TimePrecision,
}

impl SimulationParameter {
    pub const ALL: [Self; 14] = [
        Self::Gmin,
        Self::Pnjmaxi,
        Self::Tnom,
        Self::SimulatorVersion,
        Self::SimulatorSubversion,
        Self::Gdev,
        Self::Imax,
        Self::Imelt,
        Self::Iteration,
        Self::Scale,
        Self::Shrink,
        Self::SourceScaleFactor,
        Self::TimeUnit,
        Self::TimePrecision,
    ];
    pub const COUNT: usize = Self::ALL.len();

    pub const fn name(self) -> &'static str {
        match self {
            Self::Gmin => "gmin",
            Self::Pnjmaxi => "pnjmaxi",
            Self::Tnom => "tnom",
            Self::SimulatorVersion => "simulatorVersion",
            Self::SimulatorSubversion => "simulatorSubversion",
            Self::Gdev => "gdev",
            Self::Imax => "imax",
            Self::Imelt => "imelt",
            Self::Iteration => "iteration",
            Self::Scale => "scale",
            Self::Shrink => "shrink",
            Self::SourceScaleFactor => "sourceScaleFactor",
            Self::TimeUnit => "timeUnit",
            Self::TimePrecision => "timePrecision",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|parameter| parameter.name().eq_ignore_ascii_case(name))
    }
}

/// Simulator-owned values visible to Verilog-A `$simparam` calls.
///
/// Unavailable and explicitly configured zero are distinct. Values are kept
/// contiguous for generated machine code; availability has a separate mask so
/// no NaN sentinel can break equality or leak into model arithmetic.
#[derive(Debug, Clone, Copy)]
pub struct GeneratedSimulationParameters {
    values: [Value; SimulationParameter::COUNT],
    available: u64,
}

impl PartialEq for GeneratedSimulationParameters {
    fn eq(&self, other: &Self) -> bool {
        self.available == other.available
            && self
                .values
                .iter()
                .zip(other.values)
                .all(|(left, right)| left.to_bits() == right.to_bits())
    }
}

impl GeneratedSimulationParameters {
    pub const fn new() -> Self {
        let mut values = [0.0; SimulationParameter::COUNT];
        values[SimulationParameter::Gmin as usize] = crate::DEFAULT_GMIN;
        values[SimulationParameter::Tnom as usize] = 27.0;
        values[SimulationParameter::SimulatorVersion as usize] = 1.0;
        Self {
            values,
            available: (1 << SimulationParameter::Gmin as u8)
                | (1 << SimulationParameter::Tnom as u8)
                | (1 << SimulationParameter::SimulatorVersion as u8)
                | (1 << SimulationParameter::SimulatorSubversion as u8),
        }
    }

    #[inline]
    pub fn get(&self, name: &str) -> Option<Value> {
        SimulationParameter::from_name(name).and_then(|parameter| self.get_parameter(parameter))
    }

    #[inline]
    pub fn get_parameter(&self, parameter: SimulationParameter) -> Option<Value> {
        (self.available & (1 << parameter as u8) != 0).then_some(self.values[parameter as usize])
    }

    /// Install a finite value, or explicitly make a query unavailable.
    /// Invalid updates leave the complete parameter store unchanged.
    pub fn try_set(
        &mut self,
        parameter: SimulationParameter,
        value: Option<Value>,
    ) -> Result<(), &'static str> {
        if value.is_some_and(|value| !value.is_finite()) {
            return Err("simulation parameter must be finite");
        }
        let mask = 1 << parameter as u8;
        self.values[parameter as usize] = value.unwrap_or(0.0);
        if value.is_some() {
            self.available |= mask;
        } else {
            self.available &= !mask;
        }
        Ok(())
    }

    #[inline]
    pub fn set_gmin(&mut self, value: Value) {
        self.try_set(
            SimulationParameter::Gmin,
            value.is_finite().then_some(value.max(0.0)),
        )
        .expect("filtered gmin is finite");
    }

    #[inline]
    pub fn set_pnjmaxi(&mut self, value: Option<Value>) {
        self.try_set(
            SimulationParameter::Pnjmaxi,
            value.filter(|value| value.is_finite() && *value >= 0.0),
        )
        .expect("filtered pnjmaxi is finite");
    }
}

impl Default for GeneratedSimulationParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{GeneratedSimulationParameters, SimulationParameter};

    #[test]
    fn simulation_parameters_distinguish_defaults_zero_and_unavailable_values() {
        let mut parameters = GeneratedSimulationParameters::new();
        assert_eq!(parameters.get("TNOM"), Some(27.0));
        assert_eq!(parameters.get("simulatorVersion"), Some(1.0));
        assert_eq!(parameters.get("simulatorSubversion"), Some(0.0));
        assert_eq!(parameters.get("missing"), None);
        assert_eq!(parameters.get("gdev"), None);
        for parameter in SimulationParameter::ALL {
            assert_eq!(
                SimulationParameter::from_name(parameter.name()),
                Some(parameter)
            );
            parameters.try_set(parameter, Some(0.0)).unwrap();
            assert_eq!(parameters.get(parameter.name()), Some(0.0));
            let positive_zero = parameters;
            parameters.try_set(parameter, Some(-0.0)).unwrap();
            assert_ne!(
                parameters, positive_zero,
                "signed-zero updates must propagate"
            );
            assert_eq!(
                parameters.get(parameter.name()).unwrap().to_bits(),
                (-0.0_f64).to_bits()
            );
            let before = parameters;
            for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                assert!(parameters.try_set(parameter, Some(invalid)).is_err());
                assert_eq!(parameters, before);
            }
            parameters.try_set(parameter, None).unwrap();
            assert_eq!(parameters.get(parameter.name()), None);
        }
        assert_eq!(parameters.available, 0);
        assert!(parameters.values.iter().all(|value| *value == 0.0));
    }
}
