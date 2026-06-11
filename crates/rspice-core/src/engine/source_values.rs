//! Source value extraction helpers shared by engine builders and analyses.

use crate::Value;
use crate::netlist::SourceSpec;
/// Extract DC value from a SourceSpec enum
pub(crate) fn extract_dc_value(spec: &SourceSpec) -> Value {
    match spec {
        SourceSpec::Dc(v) => *v,
        SourceSpec::DcAc { dc_value, .. } => *dc_value,
        SourceSpec::DcTransient { dc_value, .. } => *dc_value,
        SourceSpec::DcAcTransient { dc_value, .. } => *dc_value,
        SourceSpec::Pulse { v1, .. } => *v1, // Use initial value
        SourceSpec::Sin { offset, .. } => *offset, // Use DC offset
        SourceSpec::Pwl { points } => points.first().map(|(_, v)| *v).unwrap_or(0.0),
        SourceSpec::PwlFile { value_offset, .. } => *value_offset, // Use value offset as DC
        SourceSpec::Exp { v1, .. } => *v1,
        SourceSpec::Ac { .. } => 0.0, // AC sources have no DC component
        // ngspice's SFFM/AM evaluate to exactly 0 at t <= TD (vsrcload.c),
        // which is what the operating point sees at time zero. TRNOISE is
        // zero-mean, so the operating point sees 0 as well.
        SourceSpec::Sffm { .. } | SourceSpec::Am { .. } | SourceSpec::TrNoise { .. } => 0.0,
    }
}

/// Extract AC value (magnitude, phase in radians) from a SourceSpec enum
/// Returns (magnitude, phase) tuple, defaulting to (0.0, 0.0) for non-AC sources
pub(crate) fn extract_ac_value(spec: &SourceSpec) -> (Value, Value) {
    match spec {
        SourceSpec::Ac { magnitude, phase } => (*magnitude, *phase),
        SourceSpec::DcAc {
            ac_magnitude,
            ac_phase,
            ..
        } => (*ac_magnitude, *ac_phase),
        SourceSpec::DcAcTransient {
            ac_magnitude,
            ac_phase,
            ..
        } => (*ac_magnitude, *ac_phase),
        SourceSpec::DcTransient { transient, .. } => extract_ac_value(transient),
        // Sin sources have AC component at their frequency
        SourceSpec::Sin {
            amplitude, phase, ..
        } => (*amplitude, *phase),
        // Other sources don't have explicit AC component in HB sense
        _ => (0.0, 0.0),
    }
}
