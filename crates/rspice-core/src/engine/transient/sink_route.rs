//! What a transient written to a result sink leaves to its caller.

use super::{fft, lead_power};
use crate::analysis::transient::TransientResult;
use crate::{AbortSignal, Engine, Netlist, SimulationError, Value};

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
