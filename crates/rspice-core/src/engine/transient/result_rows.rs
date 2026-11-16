//! The rows a transient run writes to its result sink.
//!
//! A run's saved analog values at one accepted point are one row: the saved node voltages in node
//! order, then the saved branch currents in branch order. The engine gathers them into a buffer it
//! reuses and hands the row to the sink; only accepted points get here, so the sink never sees a
//! rejected trial. A run that returns an in-memory result writes to a [`VecSink`], whose columns
//! become the result's waveforms when the run ends, bit for bit what the engine gathered.
//!
//! The last row written stays readable: the sample hook publishes it, and a checkpoint taken
//! where the published row was not the restart state carries it.
//!
//! A caller's sink that runs out of disk or memory refuses a row and keeps every row it took
//! before. The run then ends at that accepted point, as it would at its stop time: the refusal is
//! kept as the run's stop, every later row is dropped, and the time loop does not take another
//! step.

use crate::abort_signal::TransientRow;
use crate::analysis::transient::{ObservedPoint, TransientResult};
use crate::result_sink::{
    ResultAxis, ResultSchema, ResultSignal, ResultSignalKind, ResultSink, RunMeta, SinkStop,
    VecSink,
};
use crate::{SimulationError, Value};

/// What each value slot of a row records, compiled once from the run's capture plan.
#[derive(Debug, Clone, Default)]
pub(super) struct TransientRowLayout {
    /// The node index of each voltage slot, ascending.
    voltage_nodes: Vec<usize>,
    /// The branch index (solved branches, then derived ones) of each current slot, ascending.
    current_branches: Vec<usize>,
    /// Nodes and branch traces the result has, saved or not.
    num_nodes: usize,
    num_branches: usize,
    /// A sink keeps each row's step, the one that reached its point, after the currents.
    step_size: bool,
    /// Then the value each per-point store trace (a memristor's resistance) holds, by name.
    stores: Vec<String>,
    /// Constant columns last: `(device, parameter, value)` of each instance parameter a sink
    /// keeps with the rows.
    constants: Vec<(String, String, Value)>,
}

impl TransientRowLayout {
    /// The saved slots of a run with `num_nodes` nodes and `num_branches` branch traces.
    pub(super) fn new(
        voltages: &[bool],
        branch_currents: &[bool],
        num_nodes: usize,
        num_branches: usize,
    ) -> Self {
        let saved = |retain: &[bool], count: usize| -> Vec<usize> {
            retain
                .iter()
                .take(count)
                .enumerate()
                .filter_map(|(index, &keep)| keep.then_some(index))
                .collect()
        };
        Self {
            voltage_nodes: saved(voltages, num_nodes),
            current_branches: saved(branch_currents, num_branches),
            num_nodes,
            num_branches,
            step_size: false,
            stores: Vec::new(),
            constants: Vec::new(),
        }
    }

    /// This layout with the columns a sink keeps that a run in memory holds beside its rows:
    /// the step sizes, then the store traces named `stores`.
    pub(super) fn with_step_and_stores(mut self, stores: Vec<String>) -> Self {
        self.step_size = true;
        self.stores = stores;
        self
    }

    /// The slot of each row's step size, when the row carries it.
    pub(super) fn step_slot(&self) -> Option<usize> {
        self.step_size
            .then(|| self.voltage_nodes.len() + self.current_branches.len())
    }

    /// The slot of the first store trace, when the rows carry them.
    pub(super) fn store_slots(&self) -> Option<usize> {
        (!self.stores.is_empty()).then(|| {
            self.voltage_nodes.len() + self.current_branches.len() + usize::from(self.step_size)
        })
    }

    /// This layout with `constants` as constant columns after the currents.
    pub(super) fn with_constants(mut self, constants: Vec<(String, String, Value)>) -> Self {
        self.constants = constants;
        self
    }

    pub(super) fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    pub(super) fn num_branches(&self) -> usize {
        self.num_branches
    }

    pub(super) fn voltage_nodes(&self) -> &[usize] {
        &self.voltage_nodes
    }

    pub(super) fn current_branches(&self) -> &[usize] {
        &self.current_branches
    }

    /// Value slots in a row.
    pub(super) fn width(&self) -> usize {
        self.voltage_nodes.len()
            + self.current_branches.len()
            + usize::from(self.step_size)
            + self.stores.len()
            + self.constants.len()
    }

    /// The schema a sink is begun with: x is time, then a real signal per slot.
    pub(super) fn schema(
        &self,
        node_names: &[String],
        potential_units: &[Option<String>],
        branch_names: &[String],
    ) -> ResultSchema {
        let mut signals = Vec::with_capacity(self.width());
        for &node in &self.voltage_nodes {
            let mut signal = ResultSignal::new(
                node_names.get(node).cloned().unwrap_or_default(),
                potential_units
                    .get(node)
                    .cloned()
                    .flatten()
                    .unwrap_or_else(|| "V".to_owned()),
                ResultSignalKind::Real,
            );
            signal.quantity = "voltage".to_owned();
            signals.push(signal);
        }
        for &branch in &self.current_branches {
            let mut signal = ResultSignal::new(
                branch_names.get(branch).cloned().unwrap_or_default(),
                "A",
                ResultSignalKind::Real,
            );
            signal.quantity = "current".to_owned();
            signals.push(signal);
        }
        if self.step_size {
            let mut signal = ResultSignal::new("step", "s", ResultSignalKind::Real);
            signal.quantity = "step_size".to_owned();
            signals.push(signal);
        }
        for name in &self.stores {
            let mut signal = ResultSignal::new(name.clone(), "", ResultSignalKind::Real);
            signal.quantity = "store".to_owned();
            signals.push(signal);
        }
        for (device, parameter, _) in &self.constants {
            let mut signal = ResultSignal::new(parameter.clone(), "", ResultSignalKind::Real);
            signal.quantity = "instance_parameter".to_owned();
            signal.path = device.clone();
            signals.push(signal);
        }
        ResultSchema {
            x: ResultAxis {
                name: "time".to_owned(),
                unit: "s".to_owned(),
            },
            signals,
        }
    }
}

enum Target<'s> {
    /// Rows the run returns in its own result.
    Memory(VecSink),
    /// Rows the caller's sink keeps.
    External(&'s mut dyn ResultSink),
}

/// A transient run's writer of rows: the sink, the layout, and the last row written.
pub(super) struct TransientRows<'s> {
    target: Target<'s>,
    layout: TransientRowLayout,
    /// The last row written.
    row: Vec<Value>,
    /// The row being gathered; swapped with `row` once the sink takes it.
    next: Vec<Value>,
    time: Value,
    /// The time of the first row written.
    start: Value,
    points: usize,
    /// The rows kept in memory have moved into the run's result.
    filled: bool,
    /// The caller's sink refused a row for want of disk or memory: the run ends at that point.
    stop: Option<SinkStop>,
}

impl<'s> TransientRows<'s> {
    /// Rows the run keeps in memory, with room for `capacity` of them reserved up front.
    pub(super) fn in_memory(layout: TransientRowLayout, capacity: usize) -> Self {
        Self::with_target(layout, Target::Memory(VecSink::with_capacity(capacity)))
    }

    /// Rows written to the caller's `sink`.
    pub(super) fn to_sink(layout: TransientRowLayout, sink: &'s mut dyn ResultSink) -> Self {
        Self::with_target(layout, Target::External(sink))
    }

    fn with_target(layout: TransientRowLayout, target: Target<'s>) -> Self {
        let width = layout.width();
        // The constant columns are written once into both buffers; gathering a row fills only
        // the voltages and currents before them.
        let mut row = vec![0.0; width];
        let constants = width - layout.constants.len();
        for (slot, (_, _, value)) in row[constants..].iter_mut().zip(&layout.constants) {
            *slot = *value;
        }
        Self {
            target,
            layout,
            next: row.clone(),
            row,
            time: 0.0,
            start: 0.0,
            points: 0,
            filled: false,
            stop: None,
        }
    }

    fn sink(&mut self) -> &mut dyn ResultSink {
        match &mut self.target {
            Target::Memory(sink) => sink,
            Target::External(sink) => &mut **sink,
        }
    }

    /// Begin the sink with `schema`.
    pub(super) fn begin(&mut self, schema: &ResultSchema) -> Result<(), SimulationError> {
        let meta = RunMeta {
            analysis: "tran".to_owned(),
            ..RunMeta::default()
        };
        self.sink()
            .begin(schema, &meta)
            .map_err(SimulationError::ResultSink)
    }

    pub(super) fn layout(&self) -> &TransientRowLayout {
        &self.layout
    }

    /// The layout and the buffer the next row is gathered into, in slot order.
    pub(super) fn gather(&mut self) -> (&TransientRowLayout, &mut [Value]) {
        (&self.layout, &mut self.next)
    }

    /// Write the gathered row as the accepted point at `time`. Once the caller's sink stopped
    /// the run, the row is dropped.
    #[inline]
    pub(super) fn push(&mut self, time: Value) -> Result<(), SimulationError> {
        let written = match &mut self.target {
            Target::Memory(sink) => sink.row(time, &self.next),
            Target::External(_) if self.stop.is_some() => return Ok(()),
            Target::External(sink) => sink.row(time, &self.next),
        };
        if let Err(stop) = written {
            return self.refused(stop);
        }
        std::mem::swap(&mut self.row, &mut self.next);
        if self.points == 0 {
            self.start = time;
        }
        self.time = time;
        self.points += 1;
        Ok(())
    }

    /// A refusal for want of disk or memory is the run's stop; any other is its error.
    #[cold]
    fn refused(&mut self, stop: SinkStop) -> Result<(), SimulationError> {
        match stop {
            SinkStop::DiskFull { .. } | SinkStop::MemoryFull { .. }
                if matches!(self.target, Target::External(_)) =>
            {
                self.stop = Some(stop);
                Ok(())
            }
            stop => Err(SimulationError::ResultSink(stop)),
        }
    }

    /// The caller's sink stopped the run: the time loop ends at the point whose row it refused.
    pub(super) fn stopped(&self) -> bool {
        self.stop.is_some()
    }

    /// Accepted points written so far.
    pub(super) fn points(&self) -> usize {
        self.points
    }

    /// The time of the last point written.
    pub(super) fn time(&self) -> Value {
        self.time
    }

    /// The last row written, as the sample hook publishes it.
    pub(super) fn last_row(&self) -> TransientRow<'_> {
        TransientRow {
            values: &self.row,
            voltage_nodes: &self.layout.voltage_nodes,
            current_branches: &self.layout.current_branches,
        }
    }

    /// The last point written, as the sample hook reports it.
    pub(super) fn observed(&self) -> ObservedPoint<'_> {
        ObservedPoint {
            time: self.time,
            start: self.start,
            points: self.points,
            row: self.last_row(),
        }
    }

    /// The last row and the result beside it, which a checkpoint taken here carries when the row
    /// was not the restart state.
    pub(super) fn published<'r>(
        &'r self,
        result: &'r TransientResult,
    ) -> super::checkpoint::PublishedSource<'r> {
        super::checkpoint::PublishedSource {
            result,
            row: self.last_row(),
            points: self.points,
        }
    }

    /// The rows stay in this process: the run keeps them in memory rather than writing them to
    /// the caller's sink. Rows a sink takes are not result values the run holds.
    pub(super) fn keeps_rows(&self) -> bool {
        matches!(self.target, Target::Memory(_))
    }

    /// Values the run has written that its result does not hold yet: the time and every saved
    /// slot of each row, until the rows a run kept in memory move into its result. Rows written
    /// to the caller's sink are held by the sink, not by the run.
    pub(super) fn unfilled_values(&self) -> usize {
        if self.filled || !self.keeps_rows() {
            return 0;
        }
        self.points
            .saturating_mul(self.layout.width().saturating_add(1))
    }

    /// Overwrite node `node`'s voltage in row 0 while it is the only row; a node the run does not
    /// save is left alone.
    pub(super) fn amend_first_voltage(
        &mut self,
        node: usize,
        value: Value,
    ) -> Result<(), SimulationError> {
        if node >= self.layout.num_nodes {
            return Err(SimulationError::Circuit(format!(
                "node index {node} is outside this run's {} nodes",
                self.layout.num_nodes
            )));
        }
        match self.layout.voltage_nodes.binary_search(&node) {
            Ok(slot) => self.amend_first(slot, value),
            Err(_) => Ok(()),
        }
    }

    /// Overwrite branch `branch`'s current in row 0 while it is the only row; a branch the run
    /// does not save is left alone.
    pub(super) fn amend_first_current(
        &mut self,
        branch: usize,
        value: Value,
    ) -> Result<(), SimulationError> {
        if branch >= self.layout.num_branches {
            return Err(SimulationError::Circuit(format!(
                "branch index {branch} is outside this run's {} branch traces",
                self.layout.num_branches
            )));
        }
        match self.layout.current_branches.binary_search(&branch) {
            Ok(slot) => self.amend_first(self.layout.voltage_nodes.len() + slot, value),
            Err(_) => Ok(()),
        }
    }

    fn amend_first(&mut self, slot: usize, value: Value) -> Result<(), SimulationError> {
        if self.stop.is_some() {
            return Ok(());
        }
        if self.points != 1 {
            return Err(SimulationError::Circuit(format!(
                "row 0 can only be amended before row 1; the run has written {} rows",
                self.points
            )));
        }
        self.row[slot] = value;
        let column = u32::try_from(slot).map_err(|_| {
            SimulationError::Circuit(format!("value slot {slot} does not fit a row index"))
        })?;
        self.sink()
            .amend_open_row(0, column, value)
            .map_err(SimulationError::ResultSink)
    }

    /// Move the rows a run kept in memory into `result`'s time and waveform columns. A run that
    /// wrote to the caller's sink leaves them empty.
    pub(super) fn fill_result(&mut self, result: &mut TransientResult) {
        let Target::Memory(sink) = &mut self.target else {
            return;
        };
        self.filled = true;
        let (time, mut columns, _) = std::mem::take(sink).into_parts();
        result.time = time;
        let mut columns = columns.iter_mut();
        for (&node, column) in self.layout.voltage_nodes.iter().zip(columns.by_ref()) {
            if let Some(voltages) = result.voltages.get_mut(node) {
                *voltages = std::mem::take(column);
            }
        }
        for (&branch, column) in self.layout.current_branches.iter().zip(columns) {
            if let Some(currents) = result.branch_currents.get_mut(branch) {
                *currents = std::mem::take(column);
            }
        }
    }
}
