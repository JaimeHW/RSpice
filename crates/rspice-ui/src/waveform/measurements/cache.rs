use super::aggregate::{calculate_all_measurements, calculate_measurements_in_range};
use super::types::TraceMeasurements;
use crate::waveform::state::TraceData;

// Measurement Cache
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TraceSignature {
    sample_count: usize,
    x_first_bits: u64,
    x_last_bits: u64,
    y_first_bits: u64,
    y_last_bits: u64,
}

impl TraceSignature {
    fn from_trace(trace: &TraceData) -> Self {
        Self {
            sample_count: trace.len(),
            x_first_bits: trace.x.first().copied().unwrap_or(0.0).to_bits(),
            x_last_bits: trace.x.last().copied().unwrap_or(0.0).to_bits(),
            y_first_bits: trace.y.first().copied().unwrap_or(0.0).to_bits(),
            y_last_bits: trace.y.last().copied().unwrap_or(0.0).to_bits(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RangeKey {
    start_bits: u64,
    end_bits: u64,
}

impl RangeKey {
    fn from_range((start, end): (f64, f64)) -> Self {
        Self {
            start_bits: start.to_bits(),
            end_bits: end.to_bits(),
        }
    }
}

#[derive(Debug, Clone)]
struct MeasurementCacheEntry {
    signature: TraceSignature,
    range: Option<RangeKey>,
    measurements: TraceMeasurements,
}

/// Runtime cache for waveform measurement panel results.
///
/// The measurement panel is rendered every frame. Without caching, all metrics
/// are recomputed per trace on each redraw, which scales poorly with large
/// sample sets. This cache stores the last computed result per trace index and
/// cursor-range mode, and refreshes only when inputs change.
#[derive(Debug, Clone, Default)]
pub struct MeasurementCache {
    entries: Vec<Option<MeasurementCacheEntry>>,
}

impl MeasurementCache {
    /// Clear all cached entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Shrink cached entries to match current trace count.
    pub fn truncate_to_trace_count(&mut self, trace_count: usize) {
        self.entries.truncate(trace_count);
    }

    /// Get cached measurements for a trace/range key or compute and cache them.
    pub fn get_or_compute<'a>(
        &'a mut self,
        trace_index: usize,
        trace: &TraceData,
        range: Option<(f64, f64)>,
    ) -> &'a TraceMeasurements {
        if self.entries.len() <= trace_index {
            self.entries.resize_with(trace_index + 1, || None);
        }

        let signature = TraceSignature::from_trace(trace);
        let range_key = range.map(RangeKey::from_range);

        let refresh_required = self.entries[trace_index]
            .as_ref()
            .is_none_or(|entry| entry.signature != signature || entry.range != range_key);

        if refresh_required {
            let measurements = if let Some((start, end)) = range {
                calculate_measurements_in_range(trace, start, end)
            } else {
                calculate_all_measurements(trace)
            };

            self.entries[trace_index] = Some(MeasurementCacheEntry {
                signature,
                range: range_key,
                measurements,
            });
        }

        &self.entries[trace_index]
            .get_or_insert_with(|| MeasurementCacheEntry {
                signature,
                range: range_key,
                measurements: if let Some((start, end)) = range {
                    calculate_measurements_in_range(trace, start, end)
                } else {
                    calculate_all_measurements(trace)
                },
            })
            .measurements
    }
}

// =============================================================================
