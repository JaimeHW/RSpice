//! A result that holds its own samples: every window and event history it reads from a results
//! store, read into memory.

use std::collections::HashMap;
use std::sync::Arc;

use super::AnalysisResult;
use crate::analysis_payload::AnalysisResultPayload;
use crate::events::points::{EventPoint, EventPoints};
use crate::waveform::{ColumnReadFailure, RetainedWaveform, WaveformValues, WindowLead};

/// A stored window, by its column and the rows it holds: two equal keys are the same samples.
type WindowKey = (usize, u64, u64, u8, u64);

impl AnalysisResult<RetainedWaveform> {
    /// Read every sample and event change the result reads from a results store into memory, so
    /// that it holds them itself: what a file that outlives the run's store writes in place of
    /// naming the store. Waveforms that share a stored window, as a run's waveforms share its
    /// time axis, share one copy of it. The result is the same data, and digests as the same
    /// values held in memory do.
    ///
    /// A window or history the store cannot give back fails the whole result and says which.
    pub fn read_stored_into_memory(&mut self) -> Result<(), ColumnReadFailure> {
        let mut read = HashMap::new();
        for waveform in &mut self.waveforms {
            let complex = waveform
                .complex
                .iter_mut()
                .flat_map(|complex| [&mut complex.real, &mut complex.imag]);
            for values in [&mut waveform.x, &mut waveform.y]
                .into_iter()
                .chain(complex)
            {
                values_in_memory(values, &mut read)?;
            }
        }
        if let Some(AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
            ..
        }) = self.result_payload.as_mut()
        {
            for trace in digital_traces {
                points_in_memory(&mut trace.points)?;
            }
            for trace in real_traces {
                points_in_memory(&mut trace.points)?;
            }
        }
        Ok(())
    }
}

fn values_in_memory(
    values: &mut WaveformValues,
    read: &mut HashMap<WindowKey, Arc<Vec<f64>>>,
) -> Result<(), ColumnReadFailure> {
    let Some((source, window)) = values.stored_window() else {
        return Ok(());
    };
    let (lead, lead_value) = match window.lead {
        WindowLead::None => (0, 0),
        WindowLead::Value(value) => (1, value.to_bits()),
        WindowLead::Interpolated(fraction) => (2, fraction.to_bits()),
    };
    let column = Arc::as_ptr(source).cast::<()>().addr();
    let key = (column, window.first, window.rows, lead, lead_value);
    let samples = match read.get(&key) {
        Some(samples) => Arc::clone(samples),
        None => {
            let samples = Arc::new(values.try_values()?.clone());
            read.insert(key, Arc::clone(&samples));
            samples
        }
    };
    *values = WaveformValues::from(samples);
    Ok(())
}

fn points_in_memory<P: EventPoint>(points: &mut EventPoints<P>) -> Result<(), ColumnReadFailure> {
    if points.is_stored() {
        *points = EventPoints::from(points.try_points()?.clone());
    }
    Ok(())
}

#[cfg(test)]
mod tests;
