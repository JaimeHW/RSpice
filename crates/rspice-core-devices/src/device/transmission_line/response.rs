use super::*;

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct TlineTransientResponse {
    self_conductance: Value,
    mutual_conductance: Value,
    mutual_current_coefficient: Value,
    i_eq_port1: Value,
    i_eq_port2: Value,
}

impl TlineTransientResponse {
    #[inline]
    pub(in crate::device::transmission_line) fn uncoupled(
        conductance: Value,
        i_eq_port1: Value,
        i_eq_port2: Value,
    ) -> Self {
        Self {
            self_conductance: conductance,
            mutual_conductance: 0.0,
            mutual_current_coefficient: 0.0,
            i_eq_port1,
            i_eq_port2,
        }
    }

    #[inline]
    pub(in crate::device::transmission_line) fn ltra_rc(
        self_conductance: Value,
        mutual_conductance: Value,
        mutual_current_coefficient: Value,
        i_eq_port1: Value,
        i_eq_port2: Value,
    ) -> Self {
        Self {
            self_conductance,
            mutual_conductance,
            mutual_current_coefficient,
            i_eq_port1,
            i_eq_port2,
        }
    }

    #[inline]
    #[doc(hidden)]
    pub fn self_conductance(self) -> Value {
        self.self_conductance
    }

    #[inline]
    #[doc(hidden)]
    pub fn mutual_conductance(self) -> Value {
        self.mutual_conductance
    }

    #[inline]
    #[doc(hidden)]
    pub fn mutual_current_coefficient(self) -> Value {
        self.mutual_current_coefficient
    }

    #[inline]
    #[doc(hidden)]
    pub fn i_eq_port1(self) -> Value {
        self.i_eq_port1
    }

    #[inline]
    #[doc(hidden)]
    pub fn i_eq_port2(self) -> Value {
        self.i_eq_port2
    }

    #[inline]
    #[doc(hidden)]
    pub fn port_currents(self, v1: Value, v2: Value) -> (Value, Value) {
        (
            self.self_conductance * v1 + self.mutual_conductance * v2 - self.i_eq_port1,
            self.self_conductance * v2 + self.mutual_conductance * v1 - self.i_eq_port2,
        )
    }
}
