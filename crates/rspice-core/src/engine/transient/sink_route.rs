//! Which transient outputs read the whole waveforms once the run ends.

use crate::{Netlist, SimulationError};

/// The first output of `netlist` that reads a transient's whole waveforms once the run ends: an
/// `.FFT` card, a lead-power output or an instance-parameter output.
///
/// A run that writes its rows to a result sink does not keep them, and refuses such a deck before
/// it solves. This answers from the deck alone, before anything is built, and answers for every
/// request the deck makes, including one no device would answer.
pub fn transient_output_reading_waveforms(
    netlist: &Netlist,
) -> Result<Option<&'static str>, SimulationError> {
    if !netlist.fft_analyses.is_empty() {
        return Ok(Some(".FFT"));
    }
    if !super::lead_power::requested_lead_power(netlist)?.is_empty() {
        return Ok(Some("a lead-power output"));
    }
    if !crate::circuit::requested_instance_parameters(netlist).is_empty() {
        return Ok(Some("an instance-parameter output"));
    }
    Ok(None)
}
