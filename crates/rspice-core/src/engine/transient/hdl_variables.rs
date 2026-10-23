//! The HDL nets and variables a run's `.ESAVE` names (`.esave xtb.pass
//! xtb.mv`), published as ordinary digital and real traces under their
//! instance paths.
//!
//! A real or a scalar takes one trace slot and a vector one per bit. A
//! vector's bits trace as `name[k]`, most significant first, under a bus the
//! engine declares as `name`, the way a VCD reader names a vector's bits. The
//! HDL host publishes a variable's value at the end of each time step it
//! changed in, through the observer its value change dump uses, and adds
//! every variable's value at each accepted point; both arrive here by slot
//! and are recorded as changes, like an event node's.

#[cfg(feature = "veriloga")]
use crate::SimulationError;
use crate::Value;
use crate::analysis::transient::{DigitalBusDeclaration, TransientResult};
use crate::xspice::event_trace::{EventTracePoint, settle_trace_points};

/// A run's traced HDL variables and the traces they publish to.
#[derive(Debug, Default)]
pub(super) struct HdlVariableCapture {
    /// Per slot: its trace's name.
    slots: Vec<String>,
    /// Per slot: its trace's index in the result, from its first value.
    traces: Vec<Option<usize>>,
    /// The publications of one accepted point, kept for their capacity.
    points: Vec<EventTracePoint>,
    /// One bus over the bits of each vector variable.
    buses: Vec<DigitalBusDeclaration>,
}

impl HdlVariableCapture {
    /// Lay out the slots of these variables, each with the name it is
    /// traced under, and have the circuit publish them.
    #[cfg(feature = "veriloga")]
    pub(super) fn configure(
        circuit: &mut crate::circuit::CircuitData,
        variables: &[(usize, String, crate::xspice::verilog::HdlVariable)],
    ) -> Result<Self, SimulationError> {
        let mut capture = Self::default();
        let mut wiring = Vec::with_capacity(variables.len());
        for (host, name, variable) in variables {
            wiring.push((*host, variable, capture.slots.len()));
            let Some((msb, lsb)) = variable.range else {
                capture.slots.push(name.clone());
                continue;
            };
            let members: Vec<String> = if msb >= lsb {
                (lsb..=msb)
                    .rev()
                    .map(|index| format!("{name}[{index}]"))
                    .collect()
            } else {
                (msb..=lsb)
                    .map(|index| format!("{name}[{index}]"))
                    .collect()
            };
            debug_assert_eq!(members.len(), variable.slots());
            capture.slots.extend(members.iter().cloned());
            capture.buses.push(
                DigitalBusDeclaration::new(
                    name.clone(),
                    msb,
                    lsb,
                    members,
                    crate::analysis::transient::DigitalBusSource::Engine,
                )
                .map_err(|error| {
                    SimulationError::Circuit(format!("esave: HDL vector '{name}': {error}"))
                })?,
            );
        }
        capture.traces = vec![None; capture.slots.len()];
        circuit.trace_hdl_variables(&wiring);
        Ok(capture)
    }

    /// The buses over the vector variables' bits.
    pub(super) fn buses(&self) -> &[DigitalBusDeclaration] {
        &self.buses
    }

    /// Record what the variables published up to the accepted `time`, and
    /// their values there. Returns how many result values it added.
    pub(super) fn record(
        &mut self,
        result: &mut TransientResult,
        circuit: &mut crate::circuit::CircuitData,
        time: Value,
    ) -> usize {
        if self.slots.is_empty() {
            return 0;
        }
        self.points.clear();
        #[cfg(feature = "veriloga")]
        circuit.drain_hdl_variable_traces(time, &mut self.points);
        #[cfg(not(feature = "veriloga"))]
        let _ = (circuit, time);
        settle_trace_points(&mut self.points);
        let mut added = 0usize;
        for point in &self.points {
            let (Some(name), Some(trace)) =
                (self.slots.get(point.node), self.traces.get_mut(point.node))
            else {
                continue;
            };
            added = added.saturating_add(result.record_named_event_value(
                point.time,
                name,
                point.value,
                trace,
            ));
        }
        added
    }

    /// Mark every digital trace a variable publishes with the HDL's time
    /// precision, as the nets an HDL observes are marked. A real slot's
    /// index is into the real traces, which carry no precision; the name
    /// tells the two apart.
    #[cfg(feature = "veriloga")]
    pub(super) fn mark_precision(&self, result: &mut TransientResult, precision: Option<Value>) {
        let Some(precision) = precision else {
            return;
        };
        for (name, trace) in self.slots.iter().zip(&self.traces) {
            if let Some(trace) = trace
                .and_then(|index| result.digital_traces.get_mut(index))
                .filter(|trace| trace.node_name == *name)
            {
                trace.hdl_time_precision = Some(precision);
            }
        }
    }
}
