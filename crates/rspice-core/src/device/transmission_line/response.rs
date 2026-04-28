use super::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct TlineTransientResponse {
    self_conductance: Value,
    mutual_conductance: Value,
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
            i_eq_port1,
            i_eq_port2,
        }
    }

    #[inline]
    pub(crate) fn self_conductance(self) -> Value {
        self.self_conductance
    }

    #[inline]
    pub(crate) fn mutual_conductance(self) -> Value {
        self.mutual_conductance
    }

    #[inline]
    pub(crate) fn i_eq_port1(self) -> Value {
        self.i_eq_port1
    }

    #[inline]
    pub(crate) fn i_eq_port2(self) -> Value {
        self.i_eq_port2
    }

    #[inline]
    pub(crate) fn port_currents(self, v1: Value, v2: Value) -> (Value, Value) {
        (
            self.self_conductance * v1 + self.mutual_conductance * v2 - self.i_eq_port1,
            self.self_conductance * v2 + self.mutual_conductance * v1 - self.i_eq_port2,
        )
    }
}
