//! Borrowed signal-independent arithmetic view; ownership keeps the units.
use super::*;

/// Borrowed, typed singular history for shared physical post-processing.
#[derive(Clone, Copy)]
pub enum ImpulseTraceRef<'a> {
    Current(&'a CurrentImpulseTrace),
    Voltage(&'a VoltageImpulseTrace),
}

/// A signed order-zero action, in ampere-seconds or volt-seconds as
/// determined by the typed trace that supplied it.
#[derive(Clone, Copy)]
pub struct ImpulsePointValue {
    pub time: Value,
    pub coefficient: Value,
}

impl<'a> ImpulseTraceRef<'a> {
    pub fn complete(self) -> bool {
        match self {
            Self::Current(trace) => trace.complete,
            Self::Voltage(trace) => trace.complete,
        }
    }

    pub fn validate(self, start: Value, stop: Value) -> Result<(), String> {
        match self {
            Self::Current(trace) => trace.validate(start, stop),
            Self::Voltage(trace) => trace.validate(start, stop),
        }
    }
    pub(crate) fn identity(self) -> (u8, *const ()) {
        match self {
            Self::Current(trace) => (0, std::ptr::from_ref(trace).cast()),
            Self::Voltage(trace) => (1, std::ptr::from_ref(trace).cast()),
        }
    }

    pub fn point(self, index: usize) -> Option<ImpulsePointValue> {
        match self {
            Self::Current(trace) => trace.points.get(index).map(|point| ImpulsePointValue {
                time: point.time,
                coefficient: point.charge_coulombs,
            }),
            Self::Voltage(trace) => trace.points.get(index).map(|point| ImpulsePointValue {
                time: point.time,
                coefficient: point.volt_seconds,
            }),
        }
    }

    pub fn points(self) -> impl ExactSizeIterator<Item = ImpulsePointValue> + Clone {
        let count = match self {
            Self::Current(trace) => trace.points.len(),
            Self::Voltage(trace) => trace.points.len(),
        };
        (0..count).map(move |index| self.point(index).expect("bounded impulse index"))
    }

    pub fn derivatives(self) -> &'a [ImpulseDerivative] {
        match self {
            Self::Current(trace) => &trace.derivatives,
            Self::Voltage(trace) => &trace.derivatives,
        }
    }
}

impl std::fmt::Display for ImpulseTraceRef<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Current(trace) => trace.owner.fmt(formatter),
            Self::Voltage(trace) => write!(formatter, "V({})", trace.node_name),
        }
    }
}
