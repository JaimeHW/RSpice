//! What a transient written to a result sink leaves to its caller.

use super::{fft, lead_power};
use crate::analysis::transient::TransientResult;
use crate::result_sink::{
    EventChange, ResultSchema, ResultSink, RunMeta, RunStatus, SinkReceipt, SinkStop,
};
use crate::{AbortSignal, Engine, Netlist, SimulationError, Value};

/// A transient whose rows a result sink kept.
#[derive(Debug)]
pub struct SinkTransient {
    /// Everything the run returns but its time and waveform columns, which are empty.
    pub result: TransientResult,
    /// What the sink kept.
    pub receipt: SinkReceipt,
    /// The sink ran out of disk or memory and the run ended at the accepted point whose row it
    /// refused, before its stop time. Every row the sink took before is kept, and the sink was
    /// ended as stopped.
    pub stopped: Option<SinkStop>,
}

/// The caller's sink, remembering whether it began and the first refusal of a row: the stop
/// that ends the run.
pub(super) struct StopLatch<'s> {
    pub(super) sink: &'s mut dyn ResultSink,
    pub(super) begun: bool,
    pub(super) stop: Option<SinkStop>,
}

impl<'s> StopLatch<'s> {
    pub(super) fn new(sink: &'s mut dyn ResultSink) -> Self {
        Self {
            sink,
            begun: false,
            stop: None,
        }
    }
}

impl ResultSink for StopLatch<'_> {
    fn begin(&mut self, schema: &ResultSchema, meta: &RunMeta) -> Result<(), SinkStop> {
        let begun = self.sink.begin(schema, meta);
        self.begun = begun.is_ok();
        begun
    }

    #[inline]
    fn row(&mut self, x: f64, values: &[f64]) -> Result<(), SinkStop> {
        let written = self.sink.row(x, values);
        if let Err(stop) = &written
            && self.stop.is_none()
        {
            self.stop = Some(stop.clone());
        }
        written
    }

    fn amend_open_row(&mut self, row: u64, column: u32, value: f64) -> Result<(), SinkStop> {
        self.sink.amend_open_row(row, column, value)
    }

    fn events(&mut self, changes: &[EventChange]) -> Result<(), SinkStop> {
        self.sink.events(changes)
    }

    fn end(&mut self, status: RunStatus) -> Result<SinkReceipt, SinkStop> {
        self.sink.end(status)
    }
}

impl Engine {
    /// Complete the outputs of a transient whose rows a result sink kept, once the caller has
    /// read them back into `result`: the `.FFT` spectra and the lead-power outputs, which read
    /// the whole waveforms after the run. `tstop` is the stop time the run was given.
    ///
    /// The run itself wrote its instance parameters as constant columns of its rows, which
    /// [`TransientResult::adopt_instance_parameter`] takes back.
    pub fn finish_transient_from_rows(
        &self,
        netlist: &Netlist,
        result: &mut TransientResult,
        tstop: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        let engine = self.resolved_for_netlist(netlist);
        result.fft_results = fft::evaluate(&engine, netlist, result, tstop, abort)?;
        let requests = lead_power::requested_lead_power(netlist)?;
        lead_power::record_lead_power(&requests, result);
        Ok(())
    }
}
