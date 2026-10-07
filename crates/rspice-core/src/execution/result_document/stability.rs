//! Qualify zero-frequency evidence separately from the positive-frequency sweep.

use super::*;

pub(super) fn normalize_legacy(
    document: &mut AnalysisResultDocument,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    if document.schema_version >= 14 || !matches!(document.payload, ResultPayload::Stb(_)) {
        return Ok(());
    }
    for scalar in &mut document.scalars {
        check_abort(abort)?;
        if scalar.name == "dc_loop_gain" {
            return Err(invalid(
                "zero-frequency evidence requires document version 14",
            ));
        }
        if scalar.name == "dc_loop_gain_db" {
            // Versions 1-13 called the first AC sample DC. Preserve the
            // historical number under its actual meaning, never certify it.
            scalar.name = "sweep_start_gain_db".into();
            scalar.display_name = "Loop gain at sweep start".into();
        }
    }
    Ok(())
}

pub(super) fn validate(
    document: &AnalysisResultDocument,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    if !matches!(document.payload, ResultPayload::Stb(_)) {
        return Ok(());
    }
    let mut raw = None;
    let mut db = None;
    for scalar in &document.scalars {
        check_abort(abort)?;
        match scalar.name.as_str() {
            "dc_loop_gain" => raw = Some(scalar),
            "dc_loop_gain_db" => db = Some(scalar),
            _ => {}
        }
    }
    if document.schema_version < 14 {
        return if raw.is_some() || db.is_some() {
            Err(invalid(
                "legacy STB must retain its first sample as sweep-start gain, not DC",
            ))
        } else {
            Ok(())
        };
    }
    let (Some(raw), Some(db)) = (raw, db) else {
        return Err(invalid(
            "STB requires explicit DC return-ratio and magnitude availability",
        ));
    };
    if raw.unit != Some(SignalUnit::Dimensionless)
        || db.unit != Some(SignalUnit::Custom("dB".into()))
    {
        return Err(invalid(
            "DC return ratio requires dimensionless units and its magnitude requires dB",
        ));
    }
    let ScalarValue::Complex { value } = raw.value else {
        return Err(invalid(
            "DC return ratio must be complex or explicitly unmeasured",
        ));
    };
    let margins = crate::analysis::stb::StabilityMargins {
        dc_loop_gain: value.map(Into::into),
        ..Default::default()
    };
    let expected = match margins.dc_gain_db() {
        None => ScalarValue::Real { value: None },
        Some(value) if value == f64::NEG_INFINITY => ScalarValue::Unavailable {
            reason: ScalarUnavailability::NegativeInfinity,
        },
        Some(value) => ScalarValue::Real { value: Some(value) },
    };
    if db.value != expected {
        return Err(invalid(
            "DC magnitude disagrees with its independently measured return ratio",
        ));
    }
    Ok(())
}

fn invalid(detail: &str) -> ResultDocumentError {
    ResultDocumentError::Malformed {
        location: "STB DC evidence",
        detail: detail.into(),
    }
}
