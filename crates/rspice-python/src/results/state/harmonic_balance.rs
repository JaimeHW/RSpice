//! Pickle state of harmonic-balance spectra and continuation limitations.
//!
//! The spectral series shape is shared by node voltages and branch currents,
//! and the declared limitations of a continuation travel as stable tags, so a
//! restored result still states exactly which approximations produced it.

use super::*;

/// One named complex spectrum: `(name, coefficients, harmonic frequencies)`.
///
/// `SpectralVoltage` and `SpectralBranchCurrent` share this shape.
pub(crate) type SpectralSeriesState = (String, Vec<(f64, f64)>, Vec<f64>);

/// One reactive element's retained HB spectra:
/// `(device, kind, voltage phasors, current phasors, DC current is exact)`.
pub(crate) type HbReactiveState = (String, String, Vec<(f64, f64)>, Vec<(f64, f64)>, bool);

/// Complex values indexed `[frequency][sideband][node]` or
/// `[frequency][output sideband][input sideband]`.
pub(crate) type ComplexGridState = Vec<Vec<Vec<(f64, f64)>>>;

pub(crate) fn spectral_series_state(
    name: &str,
    coefficients: &[Complex64],
    frequencies: &[f64],
) -> SpectralSeriesState {
    (
        name.to_string(),
        complex_state(coefficients),
        frequencies.to_vec(),
    )
}

/// `HbReactiveKind` travels as its lowercase element name.
pub(crate) fn hb_reactive_kind_label(kind: HbReactiveKind) -> &'static str {
    match kind {
        HbReactiveKind::Capacitor => "capacitor",
        HbReactiveKind::Inductor => "inductor",
    }
}

pub(crate) fn rebuild_hb_reactive(
    state: HbReactiveState,
) -> PyResult<rspice_core::analysis::HbReactiveSpectrum> {
    let (device_name, kind, voltage, current, dc_current_is_exact) = state;
    Ok(rspice_core::analysis::HbReactiveSpectrum {
        device_name,
        kind: hb_reactive_kind_from_label(&kind)?,
        voltage_coefficients: complex_from_state(voltage),
        current_coefficients: complex_from_state(current),
        dc_current_is_exact,
    })
}

fn hb_reactive_kind_from_label(label: &str) -> PyResult<HbReactiveKind> {
    match label {
        "capacitor" => Ok(HbReactiveKind::Capacitor),
        "inductor" => Ok(HbReactiveKind::Inductor),
        other => Err(crate::errors::value_error(format!(
            "unknown reactive element kind '{other}' in pickled state"
        ))),
    }
}

/// Continuation limitations travel as stable snake-case tags rather than
/// ordinals, so a state pickled by one build still reads correctly under a
/// later one that reorders the enum.
pub(crate) fn hb_limitation_label(limitation: &HbContinuationLimitation) -> &'static str {
    match limitation {
        HbContinuationLimitation::NonlinearVoltageSourcesUseNortonEquivalent => {
            "nonlinear_voltage_sources_use_norton_equivalent"
        }
        HbContinuationLimitation::InductorDcCurrentUsesShortSurrogate => {
            "inductor_dc_current_uses_short_surrogate"
        }
        HbContinuationLimitation::VerilogAInternalStateNotRetained => {
            "veriloga_internal_state_not_retained"
        }
    }
}

pub(crate) fn hb_limitation_from_label(label: &str) -> PyResult<HbContinuationLimitation> {
    match label {
        "nonlinear_voltage_sources_use_norton_equivalent" => {
            Ok(HbContinuationLimitation::NonlinearVoltageSourcesUseNortonEquivalent)
        }
        "inductor_dc_current_uses_short_surrogate" => {
            Ok(HbContinuationLimitation::InductorDcCurrentUsesShortSurrogate)
        }
        "veriloga_internal_state_not_retained" => {
            Ok(HbContinuationLimitation::VerilogAInternalStateNotRetained)
        }
        other => Err(crate::errors::value_error(format!(
            "unknown continuation limitation '{other}' in pickled state"
        ))),
    }
}
