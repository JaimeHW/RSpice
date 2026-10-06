//! Independent-source specifications, including combined and deferred modes.

use super::record;
use rspice_core::netlist::{SourceDistortionTone, SourceSpec};
use serde_json::{Value, json};

// SPICE uses non-finite sentinels for omitted waveform arguments. Keep
// them distinct from optional fields and from a physical zero in JSON.
fn number(value: f64) -> Value {
    if value.is_nan() {
        json!("default")
    } else if value == f64::INFINITY {
        json!("unbounded")
    } else if value == f64::NEG_INFINITY {
        json!("negative_infinity")
    } else {
        json!(value)
    }
}

macro_rules! numeric_record {
    ($kind:literal, [$($number:ident),*] $(, $field:ident)* $(,)?) => {{
        $(let $number = number(*$number);)*
        record!($kind $(, $number)* $(, $field)*)
    }};
}

pub(super) fn describe(source: &SourceSpec) -> Value {
    use SourceSpec::*;
    match source {
        Distortion { inner, f1, f2 } => {
            let inner = describe(inner);
            let tone = |tone: &SourceDistortionTone| json!({"magnitude":tone.magnitude, "phase":tone.phase});
            let f1 = f1.as_ref().map(tone);
            let f2 = f2.as_ref().map(tone);
            record!("distortion", inner, f1, f2)
        }
        RfPort { inner, port } => {
            let inner = describe(inner);
            let port = json!({"portnum":port.portnum, "z0":port.z0, "power":port.power,
                "frequency":port.frequency, "phase":port.phase, "reference_plane":port.reference_plane});
            record!("rf_port", inner, port)
        }
        Dc(value) => numeric_record!("dc", [value]),
        Ac { magnitude, phase } => numeric_record!("ac", [magnitude, phase]),
        DcAc {
            dc_value,
            ac_magnitude,
            ac_phase,
        } => numeric_record!("dc_ac", [dc_value, ac_magnitude, ac_phase]),
        DcTransient {
            dc_value,
            transient,
        } => {
            let transient = describe(transient);
            numeric_record!("dc_transient", [dc_value], transient)
        }
        AcTransient {
            ac_magnitude,
            ac_phase,
            transient,
        } => {
            let transient = describe(transient);
            numeric_record!("ac_transient", [ac_magnitude, ac_phase], transient)
        }
        DcAcTransient {
            dc_value,
            ac_magnitude,
            ac_phase,
            transient,
        } => {
            let transient = describe(transient);
            numeric_record!(
                "dc_ac_transient",
                [dc_value, ac_magnitude, ac_phase],
                transient
            )
        }
        Pulse {
            v1,
            v2,
            delay,
            rise,
            fall,
            width,
            period,
            pulse_count,
            width_defaults_to_zero,
        } => numeric_record!(
            "pulse",
            [v1, v2, delay, rise, fall, width, period, pulse_count],
            width_defaults_to_zero
        ),
        Sin {
            offset,
            amplitude,
            frequency,
            delay,
            damping,
            phase,
        } => numeric_record!("sin", [offset, amplitude, frequency, delay, damping, phase]),
        Pwl {
            points,
            delay,
            repeat_from,
        } => numeric_record!("pwl", [delay], points, repeat_from),
        PwlFile {
            path,
            time_scale,
            value_scale,
            time_offset,
            value_offset,
            delay,
            repeat_from,
        } => numeric_record!(
            "pwl_file",
            [time_scale, value_scale, time_offset, value_offset, delay],
            path,
            repeat_from
        ),
        Pat {
            vhi,
            vlo,
            delay,
            rise,
            fall,
            sample,
            data,
            repeat_count,
        } => numeric_record!(
            "pat",
            [vhi, vlo, delay, rise, fall, sample],
            data,
            repeat_count
        ),
        Exp {
            v1,
            v2,
            td1,
            tau1,
            td2,
            tau2,
        } => numeric_record!("exp", [v1, v2, td1, tau1, td2, tau2]),
        Sffm {
            offset,
            amplitude,
            carrier_freq,
            modulation_index,
            signal_freq,
            delay,
            phase_modulation,
            phase_carrier,
        } => numeric_record!(
            "sffm",
            [
                offset,
                amplitude,
                carrier_freq,
                modulation_index,
                signal_freq,
                delay,
                phase_modulation,
                phase_carrier
            ]
        ),
        Am {
            offset,
            modulation_offset,
            modulation_amplitude,
            modulating_freq,
            carrier_freq,
            delay,
            phase_modulation,
            phase_carrier,
        } => numeric_record!(
            "am",
            [
                offset,
                modulation_offset,
                modulation_amplitude,
                modulating_freq,
                carrier_freq,
                delay,
                phase_modulation,
                phase_carrier
            ]
        ),
        TrNoise {
            na,
            nt,
            nalpha,
            namp,
            rts_amplitude,
            rts_capture,
            rts_emit,
        } => numeric_record!(
            "trnoise",
            [na, nt, nalpha, namp, rts_amplitude, rts_capture, rts_emit]
        ),
        TrRandom {
            distribution,
            sample_interval,
            delay,
            parameter1,
            parameter2,
        } => numeric_record!(
            "trrandom",
            [sample_interval, delay, parameter1, parameter2],
            distribution
        ),
    }
}
