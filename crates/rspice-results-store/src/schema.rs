//! What a segment holds: the shared x axis, the signals, and the run's metadata.

use crate::StoreError;
use crate::format::{Cursor, PutLe};

/// The run's shared x axis (time, frequency or the swept value).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Axis {
    pub name: String,
    pub unit: String,
}

/// How a signal's values are stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalKind {
    /// One f64 per row.
    Real,
    /// Two f64 per row, real part then imaginary part.
    Complex,
    /// A digital net: `EVENTS` of 0, 1, X or Z with strength.
    Logic,
    /// A real-valued event net (`wreal`, XSPICE real nodes).
    RealEvent,
    /// A digital bus of `width` bits (1 to 64): `EVENTS` of value, X mask and Z mask.
    Bus { width: u32 },
}

impl SignalKind {
    /// Value slots this signal takes in each row; event signals take none.
    pub fn slots(self) -> usize {
        match self {
            Self::Real => 1,
            Self::Complex => 2,
            Self::Logic | Self::RealEvent | Self::Bus { .. } => 0,
        }
    }

    pub fn is_event(self) -> bool {
        self.slots() == 0
    }

    fn code(self) -> (u8, u32) {
        match self {
            Self::Real => (0, 0),
            Self::Complex => (1, 0),
            Self::Logic => (2, 0),
            Self::RealEvent => (3, 0),
            Self::Bus { width } => (4, width),
        }
    }

    fn from_code(code: u8, width: u32) -> Option<Self> {
        Some(match code {
            0 => Self::Real,
            1 => Self::Complex,
            2 => Self::Logic,
            3 => Self::RealEvent,
            4 => Self::Bus { width },
            _ => return None,
        })
    }
}

/// One saved signal. Its id is its position in [`Schema::signals`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signal {
    pub name: String,
    pub unit: String,
    pub kind: SignalKind,
    /// What the value measures (`voltage`, `current`, `logic`, ...).
    pub quantity: String,
    /// Hierarchical instance path, empty at the top level.
    pub path: String,
}

impl Signal {
    pub fn new(name: impl Into<String>, unit: impl Into<String>, kind: SignalKind) -> Self {
        Self {
            name: name.into(),
            unit: unit.into(),
            kind,
            quantity: String::new(),
            path: String::new(),
        }
    }
}

/// The column table of a segment.
///
/// Each row is x plus, in signal order, one f64 per real signal and two (real, imaginary) per
/// complex signal: the row's value slots. Event signals have ids but no slots.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Schema {
    pub x: Axis,
    pub signals: Vec<Signal>,
}

impl Schema {
    /// Value slots per row.
    pub fn slots(&self) -> usize {
        self.signals.iter().map(|signal| signal.kind.slots()).sum()
    }

    /// The first value slot of signal `id`, when it is an analog signal.
    pub fn slot_of(&self, id: usize) -> Option<usize> {
        let signal = self.signals.get(id)?;
        if signal.kind.is_event() {
            return None;
        }
        Some(
            self.signals[..id]
                .iter()
                .map(|signal| signal.kind.slots())
                .sum(),
        )
    }

    pub(crate) fn validate(&self) -> Result<(), StoreError> {
        for (id, signal) in self.signals.iter().enumerate() {
            if signal.kind == (SignalKind::Bus { width: 0 }) {
                return Err(StoreError::Schema(format!(
                    "bus `{}` (signal {id}) has no bits",
                    signal.name
                )));
            }
        }
        if u32::try_from(self.signals.len()).is_err() {
            return Err(StoreError::Schema("more than 2^32 signals".into()));
        }
        Ok(())
    }

    pub(crate) fn encode(&self, out: &mut Vec<u8>) {
        out.put_str(&self.x.name);
        out.put_str(&self.x.unit);
        out.put_u32(self.signals.len() as u32);
        for signal in &self.signals {
            let (code, width) = signal.kind.code();
            out.put_u8(code);
            out.put_u32(width);
            out.put_str(&signal.name);
            out.put_str(&signal.unit);
            out.put_str(&signal.quantity);
            out.put_str(&signal.path);
        }
    }

    pub(crate) fn decode(cursor: &mut Cursor<'_>) -> Result<Self, StoreError> {
        let x = Axis {
            name: cursor.string()?,
            unit: cursor.string()?,
        };
        let count = cursor.count(21)?;
        let mut signals = Vec::with_capacity(count);
        for _ in 0..count {
            let code = cursor.u8()?;
            let width = cursor.u32()?;
            let kind = SignalKind::from_code(code, width)
                .ok_or_else(|| cursor.error("unknown signal kind"))?;
            signals.push(Signal {
                name: cursor.string()?,
                unit: cursor.string()?,
                kind,
                quantity: cursor.string()?,
                path: cursor.string()?,
            });
        }
        let schema = Self { x, signals };
        schema
            .validate()
            .map_err(|_| cursor.error("a bus width is outside 1 to 64 bits"))?;
        Ok(schema)
    }
}

/// The run a segment records: the analysis card, sweep values, seed and options.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RunMeta {
    pub analysis: String,
    /// Named sweep or trial values that select this run among its family.
    pub sweep: Vec<(String, f64)>,
    pub seed: Option<u64>,
    pub options: Vec<(String, String)>,
}

impl RunMeta {
    pub(crate) fn encode(&self, out: &mut Vec<u8>) {
        out.put_str(&self.analysis);
        out.put_u32(self.sweep.len() as u32);
        for (name, value) in &self.sweep {
            out.put_str(name);
            out.put_f64(*value);
        }
        match self.seed {
            Some(seed) => {
                out.put_u8(1);
                out.put_u64(seed);
            }
            None => out.put_u8(0),
        }
        out.put_u32(self.options.len() as u32);
        for (name, value) in &self.options {
            out.put_str(name);
            out.put_str(value);
        }
    }

    pub(crate) fn decode(cursor: &mut Cursor<'_>) -> Result<Self, StoreError> {
        let analysis = cursor.string()?;
        let count = cursor.count(12)?;
        let mut sweep = Vec::with_capacity(count);
        for _ in 0..count {
            sweep.push((cursor.string()?, cursor.f64()?));
        }
        let seed = match cursor.u8()? {
            0 => None,
            1 => Some(cursor.u64()?),
            _ => return Err(cursor.error("the seed flag is neither 0 nor 1")),
        };
        let count = cursor.count(8)?;
        let mut options = Vec::with_capacity(count);
        for _ in 0..count {
            options.push((cursor.string()?, cursor.string()?));
        }
        Ok(Self {
            analysis,
            sweep,
            seed,
            options,
        })
    }
}
