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
//!
//! The sink is begun when the run starts integrating, not when its first row is gathered: until
//! then the run can still name the event signals it will write (event nets, HDL variables,
//! impulses), and row 0, which the run's set-up amends in place, waits in the buffer. Begun, the
//! sink takes row 0 first.

use crate::abort_signal::TransientRow;
use crate::analysis::transient::{ObservedPoint, TransientResult};
use crate::circuit::DeviceOpReport;
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
    /// Then each device operating-point value a sink keeps, as `(device, parameter)` in the order
    /// the run's first report named them.
    device_ops: Vec<(String, String)>,
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
            device_ops: Vec::new(),
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

    /// This layout with a column for each device operating-point value `device_ops` names, after
    /// the store traces.
    pub(super) fn with_device_ops(mut self, device_ops: Vec<(String, String)>) -> Self {
        self.device_ops = device_ops;
        self
    }

    /// The slot of the first device operating-point value, when the rows carry them.
    pub(super) fn device_op_slots(&self) -> Option<usize> {
        (!self.device_ops.is_empty()).then(|| {
            self.voltage_nodes.len()
                + self.current_branches.len()
                + usize::from(self.step_size)
                + self.stores.len()
        })
    }

    /// `(device, parameter)` of each device operating-point column.
    pub(super) fn device_ops(&self) -> &[(String, String)] {
        &self.device_ops
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
            + self.device_ops.len()
            + self.constants.len()
    }

    /// The schema a sink is begun with: x is time, then a real signal per slot.
    ///
    /// Each signal says what it is, so a reader with neither the engine nor the netlist can name
    /// it as the run does: a node's potential is a `voltage` in its discipline's unit, a branch
    /// current a `current` in amperes, and a Verilog-A instance's own unknown, held in a node's
    /// slot, an `instance_current` in amperes when the module names it as a branch and an
    /// `instance_private` unknown otherwise.
    pub(super) fn schema(
        &self,
        node_names: &[String],
        potential_units: &[Option<String>],
        instance_unknowns: &[Option<crate::solver::InstanceUnknown>],
        branch_names: &[String],
    ) -> ResultSchema {
        use crate::solver::InstanceUnknown;
        let mut signals = Vec::with_capacity(self.width());
        for &node in &self.voltage_nodes {
            let potential = || {
                potential_units
                    .get(node)
                    .cloned()
                    .flatten()
                    .unwrap_or_else(|| "V".to_owned())
            };
            let (unit, quantity) = match instance_unknowns.get(node).copied().flatten() {
                None => (potential(), "voltage"),
                Some(InstanceUnknown::BranchCurrent) => ("A".to_owned(), "instance_current"),
                Some(InstanceUnknown::Private) => (potential(), "instance_private"),
            };
            let mut signal = ResultSignal::new(
                node_names.get(node).cloned().unwrap_or_default(),
                unit,
                ResultSignalKind::Real,
            );
            signal.quantity = quantity.to_owned();
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
        for (device, parameter) in &self.device_ops {
            let mut signal = ResultSignal::new(parameter.clone(), "", ResultSignalKind::Real);
            signal.quantity = "device_op".to_owned();
            signal.path = device.clone();
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

/// Each `(device, parameter, value)` a device report and the observables beside it name, in the
/// order the run records them: each device's own values, then its module's quantities, then the
/// observables, which replace a reported value they name again.
fn reported<'r>(
    report: &'r DeviceOpReport,
    observed: &'r [(String, String, Value)],
) -> impl Iterator<Item = (&'r str, &'r str, Value)> {
    report
        .entries
        .iter()
        .flat_map(|entry| {
            let module = entry.module.iter().flat_map(|module| {
                module
                    .quantities
                    .iter()
                    .map(|quantity| (entry.name.as_str(), quantity.name.as_str(), quantity.value))
            });
            entry
                .params
                .iter()
                .map(|&(parameter, value)| (entry.name.as_str(), parameter, value))
                .chain(module)
        })
        .chain(
            observed
                .iter()
                .map(|(device, parameter, value)| (device.as_str(), parameter.as_str(), *value)),
        )
}

/// The `(device, parameter)` columns of a run whose first device report is `report`: each one it
/// names, once, in the order an in-memory run creates their traces.
pub(super) fn device_op_columns(
    report: &DeviceOpReport,
    observed: &[(String, String, Value)],
) -> Vec<(String, String)> {
    let mut columns: Vec<(String, String)> = Vec::new();
    for (device, parameter, _) in reported(report, observed) {
        if !columns.iter().any(|(known_device, known_parameter)| {
            known_device.eq_ignore_ascii_case(device)
                && known_parameter.eq_ignore_ascii_case(parameter)
        }) {
            columns.push((device.to_owned(), parameter.to_owned()));
        }
    }
    columns
}

/// Record a device report at row 0, overwriting what an earlier report there recorded: in the
/// row's device columns when the sink keeps them, else in the result's device traces. Returns how
/// many result values it added.
pub(super) fn record_first_device_ops(
    result: &mut TransientResult,
    rows: &mut TransientRows<'_>,
    report: DeviceOpReport,
    observed: &[(String, String, Value)],
) -> Result<usize, SimulationError> {
    if rows.layout.device_op_slots().is_none() {
        return Ok(result.record_device_op_sample(rows.points(), report, observed));
    }
    let mut values = Vec::new();
    rows.map_device_ops(&report, observed, |index, value| {
        values.push((index, value));
    })?;
    for (index, value) in values {
        rows.amend_first_device_op(index, value)?;
    }
    Ok(0)
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
    /// The schema the sink begins with when the run starts integrating; `None` once begun.
    pending: Option<ResultSchema>,
    /// The sink was begun and took row 0.
    begun: bool,
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
            pending: None,
            begun: false,
        }
    }

    fn sink(&mut self) -> &mut dyn ResultSink {
        match &mut self.target {
            Target::Memory(sink) => sink,
            Target::External(sink) => &mut **sink,
        }
    }

    /// Lay out the sink's columns as `schema`; [`Self::start`] begins it.
    pub(super) fn begin(&mut self, schema: &ResultSchema) -> Result<(), SimulationError> {
        if self.begun || self.pending.is_some() {
            return Err(SimulationError::Circuit(
                "a transient's result rows were laid out twice".to_owned(),
            ));
        }
        self.pending = Some(schema.clone());
        Ok(())
    }

    /// Name one more event signal the sink will take; returns its id. Only before the run starts.
    pub(super) fn declare(&mut self, signal: ResultSignal) -> Result<u32, SimulationError> {
        let Some(schema) = self.pending.as_mut() else {
            return Err(SimulationError::Circuit(format!(
                "event signal '{}' was named after the transient's result sink began",
                signal.name
            )));
        };
        let id = u32::try_from(schema.signals.len()).map_err(|_| {
            SimulationError::Circuit("a transient names more signals than a store holds".to_owned())
        })?;
        schema.signals.push(signal);
        Ok(id)
    }

    /// Begin the sink with every signal the run named and write row 0 to it, once. A sink that
    /// refuses row 0 for want of disk or memory stops the run, as any row's refusal does.
    pub(super) fn start(&mut self) -> Result<(), SimulationError> {
        if self.begun {
            return Ok(());
        }
        let schema = self.pending.take().ok_or_else(|| {
            SimulationError::Circuit("a transient's result rows began without a layout".to_owned())
        })?;
        let meta = RunMeta {
            analysis: "tran".to_owned(),
            ..RunMeta::default()
        };
        self.sink()
            .begin(&schema, &meta)
            .map_err(SimulationError::ResultSink)?;
        self.begun = true;
        if self.points == 0 {
            return Ok(());
        }
        let (time, row) = (self.start, std::mem::take(&mut self.row));
        let written = self.sink().row(time, &row);
        self.row = row;
        match written {
            Ok(()) => Ok(()),
            Err(stop) => self.refused(stop),
        }
    }

    /// Hand event changes to the sink. The changes the run committed before its sink stopped it
    /// are still offered, since they precede the refused row; a sink that refuses them too keeps
    /// what it had.
    pub(super) fn events(
        &mut self,
        changes: &[crate::result_sink::EventChange],
    ) -> Result<(), SimulationError> {
        if changes.is_empty() {
            return Ok(());
        }
        self.start()?;
        match self.sink().events(changes) {
            Ok(()) => Ok(()),
            Err(_) if self.stop.is_some() => Ok(()),
            Err(stop) => self.refused(stop),
        }
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
        if !self.begun {
            // Row 0 waits for the sink to begin; any later row begins it.
            if self.points != 0 {
                self.start()?;
            } else {
                std::mem::swap(&mut self.row, &mut self.next);
                self.start = time;
                self.time = time;
                self.points = 1;
                return Ok(());
            }
        }
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

    /// A refusal for want of disk, memory or storage budget is the run's stop; any other is its
    /// error.
    #[cold]
    fn refused(&mut self, stop: SinkStop) -> Result<(), SimulationError> {
        match stop {
            SinkStop::DiskFull { .. }
            | SinkStop::StorageBudget { .. }
            | SinkStop::MemoryFull { .. }
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
        let first = self.layout.device_op_slots().unwrap_or(self.row.len());
        super::checkpoint::PublishedSource {
            result,
            row: self.last_row(),
            points: self.points,
            device_columns: &self.layout.device_ops,
            device_row: self.row.get(first..).unwrap_or(&[]),
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

    /// Gather a device report into the device columns of the row being gathered; a column the
    /// report does not name is NaN, as an in-memory trace is padded.
    pub(super) fn gather_device_ops(
        &mut self,
        report: &DeviceOpReport,
        observed: &[(String, String, Value)],
    ) -> Result<(), SimulationError> {
        let Some(first) = self.layout.device_op_slots() else {
            return Ok(());
        };
        let count = self.layout.device_ops.len();
        let mut next = std::mem::take(&mut self.next);
        let slots = &mut next[first..first + count];
        slots.fill(Value::NAN);
        let mapped = self.map_device_ops(report, observed, |index, value| slots[index] = value);
        self.next = next;
        mapped
    }

    /// Call `put` with each reported value's device column. The report names its values in the
    /// columns' order, so each is looked for where the last one was found first. A value no
    /// column holds is refused: the sink's columns were fixed by the first report.
    fn map_device_ops(
        &self,
        report: &DeviceOpReport,
        observed: &[(String, String, Value)],
        mut put: impl FnMut(usize, Value),
    ) -> Result<(), SimulationError> {
        let columns = &self.layout.device_ops;
        let named = |index: usize, device: &str, parameter: &str| {
            columns
                .get(index)
                .is_some_and(|(known_device, known_parameter)| {
                    known_device.eq_ignore_ascii_case(device)
                        && known_parameter.eq_ignore_ascii_case(parameter)
                })
        };
        let mut cursor = 0;
        for (device, parameter, value) in reported(report, observed) {
            let index = if named(cursor, device, parameter) {
                cursor
            } else {
                (0..columns.len())
                    .find(|&index| named(index, device, parameter))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "device '{device}' reported operating-point value '{parameter}', \
                             which its first report did not name; a transient written to a \
                             result store keeps the values the first report names"
                        ))
                    })?
            };
            put(index, value);
            cursor = index + 1;
        }
        Ok(())
    }

    /// Overwrite row 0's value of device operating-point column `index`.
    pub(super) fn amend_first_device_op(
        &mut self,
        index: usize,
        value: Value,
    ) -> Result<(), SimulationError> {
        match self.layout.device_op_slots() {
            Some(first) if index < self.layout.device_ops.len() => {
                self.amend_first(first + index, value)
            }
            _ => Err(SimulationError::Circuit(format!(
                "device operating-point column {index} is outside this run's {} columns",
                self.layout.device_ops.len()
            ))),
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
        if !self.begun {
            return Ok(());
        }
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
