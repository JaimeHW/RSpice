//! Source value extraction helpers shared by engine builders and analyses.

use crate::Value;
use crate::netlist::SourceSpec;
/// Extract DC value from a SourceSpec enum
pub(crate) fn extract_dc_value(spec: &SourceSpec) -> Value {
    match spec {
        SourceSpec::RfPort { inner, .. } => extract_dc_value(inner),
        SourceSpec::Dc(v) => *v,
        SourceSpec::DcAc { dc_value, .. } => *dc_value,
        SourceSpec::DcTransient { dc_value, .. } => *dc_value,
        SourceSpec::DcAcTransient { dc_value, .. } => *dc_value,
        SourceSpec::Pulse { v1, .. } => *v1, // Use initial value
        // ngspice's TRANOP evaluates the waveform at t=0, where SIN holds
        // VO + VA*sin(PHASE) (vsrcload.c); phase is stored in radians.
        SourceSpec::Sin {
            offset,
            amplitude,
            phase,
            ..
        } => offset + amplitude * phase.sin(),
        SourceSpec::Pwl { points, .. } => points.first().map(|(_, v)| *v).unwrap_or(0.0),
        SourceSpec::Pat {
            vhi,
            vlo,
            delay,
            rise,
            fall,
            sample,
            data,
            repeat_count,
        } => crate::circuit::VoltageSources::evaluate_pat_source(
            *vhi,
            *vlo,
            *delay,
            *rise,
            *fall,
            *sample,
            data,
            *repeat_count,
            0.0,
        ),
        // ngspice's TRANOP sees the waveform value at t=0, not just the
        // offset; otherwise the transient starts from a wrong bias and
        // glitches at the first step. Falls back to the offset when the
        // file cannot be read (the transient path warns about it).
        SourceSpec::PwlFile {
            path,
            time_scale,
            value_scale,
            time_offset,
            value_offset,
            delay,
            repeat_from,
        } => crate::circuit::VoltageSources::load_pwl_waveform_cached(
            path,
            *time_scale,
            *value_scale,
            *time_offset,
            *value_offset,
        )
        .map(|wf| {
            if 0.0 < *delay {
                0.0
            } else {
                wf.value_at_repeating(0.0, *repeat_from)
            }
        })
        .unwrap_or(*value_offset),
        SourceSpec::Exp { v1, .. } => *v1,
        SourceSpec::Ac { .. } => 0.0, // AC sources have no DC component
        // ngspice's SFFM/AM evaluate to exactly 0 at t <= TD (vsrcload.c),
        // which is what the operating point sees at time zero. TRNOISE is
        // zero-mean, so the operating point sees 0 as well.
        SourceSpec::Sffm { .. } | SourceSpec::Am { .. } | SourceSpec::TrNoise { .. } => 0.0,
    }
}

/// Extract AC value (magnitude, phase in radians) from a SourceSpec enum
///
/// Only an explicit AC keyword contributes: like ngspice, a transient
/// waveform such as SIN never excites small-signal AC analysis on its
/// own (vsrcacld.c stamps VSRCacGiven values exclusively). Harmonic
/// balance, which does interpret SIN drives, falls back to the waveform
/// specification separately through its periodic-source spectrum builder.
pub(crate) fn extract_ac_value(spec: &SourceSpec) -> (Value, Value) {
    match spec {
        SourceSpec::RfPort { inner, .. } => extract_ac_value(inner),
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
        _ => (0.0, 0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sin_waveforms_do_not_excite_small_signal_ac() {
        // ngspice-46 oracle: .ac on a deck whose only stimulus is
        // SIN(0 1 1k) reports exactly zero at every frequency.
        let spec = SourceSpec::Sin {
            offset: 0.0,
            amplitude: 1.0,
            frequency: 1.0e3,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        };
        assert_eq!(extract_ac_value(&spec), (0.0, 0.0));

        let wrapped = SourceSpec::DcTransient {
            dc_value: 0.5,
            transient: Box::new(spec),
        };
        assert_eq!(extract_ac_value(&wrapped), (0.0, 0.0));
    }

    #[test]
    fn explicit_ac_keyword_still_drives_small_signal_ac() {
        let spec = SourceSpec::DcAc {
            dc_value: 0.0,
            ac_magnitude: 2.0,
            ac_phase: 0.25,
        };
        assert_eq!(extract_ac_value(&spec), (2.0, 0.25));
    }

    #[test]
    fn sin_operating_point_value_includes_phase() {
        // TRANOP evaluates the waveform at t=0: VO + VA*sin(PHASE).
        let spec = SourceSpec::Sin {
            offset: 1.0,
            amplitude: 2.0,
            frequency: 1.0e3,
            delay: 0.0,
            damping: 0.0,
            phase: std::f64::consts::FRAC_PI_2,
        };
        assert_eq!(extract_dc_value(&spec), 3.0);
    }
}
