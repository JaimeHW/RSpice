//! The time cursor on event lanes: a right-continuous readout, and stepping
//! to the selected lane's previous or next event instant.
//!
//! At an event instant a lane reads the value from that instant on, as a
//! measure's `FIND … AT=` does. Stepping lands exactly on a recorded instant,
//! never on a pixel's time, so a reading taken after a step is the value the
//! event committed.

use super::model::{BusLane, LogicLane, RealLane};
use rspice_results::events::projection::BusRadix;

/// One lane the cursor can be read on and stepped along.
#[derive(Clone, Copy)]
pub enum EventLane<'a> {
    Logic(&'a LogicLane),
    Bus(&'a BusLane, BusRadix),
    Real(&'a RealLane),
}

/// What a lane reads at the cursor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneReadout {
    pub text: String,
    /// Why a bus word is not in the lane's radix, when it is not.
    pub fallback: Option<&'static str>,
}

/// Which way the cursor steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeStep {
    Previous,
    Next,
}

impl EventLane<'_> {
    pub fn name(&self) -> &str {
        match self {
            Self::Logic(lane) => lane.name(),
            Self::Bus(lane, _) => lane.name(),
            Self::Real(lane) => lane.name(),
        }
    }

    /// The value at `t`, right-continuously; `None` before the first event.
    pub fn readout_at(&self, t: f64, significant_digits: usize) -> Option<LaneReadout> {
        match self {
            Self::Logic(lane) => lane.level_at(t).map(|level| LaneReadout {
                text: level.symbol().to_owned(),
                fallback: None,
            }),
            Self::Bus(lane, radix) => lane.spell_at(t, *radix).map(|word| LaneReadout {
                text: word.text().to_owned(),
                fallback: word.fallback(),
            }),
            Self::Real(lane) => lane.value_at(t).map(|value| LaneReadout {
                text: rspice_ui_kit::plot::fmt_significant(value, significant_digits, ""),
                fallback: None,
            }),
        }
    }

    /// The event instant one step from `t`: strictly after it for
    /// [`EdgeStep::Next`], strictly before it for [`EdgeStep::Previous`].
    pub fn step_from(&self, t: f64, step: EdgeStep) -> Option<f64> {
        match (self, step) {
            (Self::Logic(lane), EdgeStep::Next) => lane.next_edge(t),
            (Self::Logic(lane), EdgeStep::Previous) => lane.previous_edge(t),
            (Self::Bus(lane, _), EdgeStep::Next) => lane.next_edge(t),
            (Self::Bus(lane, _), EdgeStep::Previous) => lane.previous_edge(t),
            (Self::Real(lane), EdgeStep::Next) => lane.next_edge(t),
            (Self::Real(lane), EdgeStep::Previous) => lane.previous_edge(t),
        }
    }
}
