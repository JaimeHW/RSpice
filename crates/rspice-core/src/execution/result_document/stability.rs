//! Validate measured crossover pairs and independently measured DC evidence.

use super::*;

pub(super) fn normalize_legacy(
    document: &mut AnalysisResultDocument,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    if document.schema_version >= 15 || !matches!(document.payload, ResultPayload::Stb(_)) {
        return Ok(());
    }
    for scalar in &mut document.scalars {
        check_abort(abort)?;
        if document.schema_version < 14 && scalar.name == "dc_loop_gain" {
            return Err(invalid(
                "zero-frequency evidence requires document version 14",
            ));
        }
        if document.schema_version < 14 && scalar.name == "dc_loop_gain_db" {
            // Versions 1-13 called the first AC sample DC. Preserve the
            // historical number under its actual meaning, never certify it.
            scalar.name = "sweep_start_gain_db".into();
            scalar.display_name = "Loop gain at sweep start".into();
        }
    }
    // Older margin infinities meant that the sweep located no crossing.
    // Relabel that absence without inventing a zero-frequency measurement.
    let mut absent = [true; 2];
    for scalar in &document.scalars {
        check_abort(abort)?;
        let slot = match scalar.name.as_str() {
            "gain_margin_frequency" => &mut absent[0],
            "phase_margin_frequency" => &mut absent[1],
            _ => continue,
        };
        *slot = !matches!(scalar.value, ScalarValue::Real { value: Some(value) } if value.is_finite() && value > 0.0);
    }
    for scalar in &document.scalars {
        check_abort(abort)?;
        let slot = match scalar.name.as_str() {
            "gain_margin_db" => &mut absent[0],
            "phase_margin_degrees" => &mut absent[1],
            _ => continue,
        };
        *slot |= matches!(
            scalar.value,
            ScalarValue::Unavailable {
                reason: ScalarUnavailability::PositiveInfinity
                    | ScalarUnavailability::NegativeInfinity
            }
        );
    }
    for scalar in &mut document.scalars {
        check_abort(abort)?;
        if scalar.name == "conditionally_stable" {
            scalar.name = "multiple_unity_gain_crossovers".into();
            scalar.display_name = "Multiple unity-gain crossovers".into();
        }
        let missing = match scalar.name.as_str() {
            "gain_margin_db" | "gain_margin_frequency" => absent[0],
            "phase_margin_degrees" | "phase_margin_frequency" | "unity_gain_bandwidth" => absent[1],
            _ => continue,
        };
        if missing {
            scalar.value = ScalarValue::Unavailable {
                reason: ScalarUnavailability::NoCrossover,
            };
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
    validate_margins(document, abort)?;
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
        location: "STB evidence",
        detail: detail.into(),
    }
}

fn validate_margins(
    document: &AnalysisResultDocument,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    if document.schema_version < 15 {
        return Ok(());
    }
    let mut scalars = [None; 7];
    for scalar in &document.scalars {
        check_abort(abort)?;
        let slot = match scalar.name.as_str() {
            "gain_margin_db" => &mut scalars[0],
            "gain_margin_frequency" => &mut scalars[1],
            "phase_margin_degrees" => &mut scalars[2],
            "phase_margin_frequency" => &mut scalars[3],
            "unity_gain_bandwidth" => &mut scalars[4],
            "unity_gain_crossovers" => &mut scalars[5],
            "multiple_unity_gain_crossovers" => &mut scalars[6],
            "conditionally_stable" => {
                return Err(invalid(
                    "multiple crossings do not establish conditional stability",
                ));
            }
            _ => continue,
        };
        *slot = Some(scalar);
    }
    let [
        Some(gain),
        Some(gain_frequency),
        Some(phase),
        Some(phase_frequency),
        Some(bandwidth),
        Some(count),
        Some(multiple),
    ] = scalars
    else {
        return Err(invalid(
            "STB requires explicit margin and crossover evidence",
        ));
    };
    let absent = ScalarValue::Unavailable {
        reason: ScalarUnavailability::NoCrossover,
    };
    for (margin, frequency, unit) in [
        (gain, gain_frequency, SignalUnit::Custom("dB".into())),
        (phase, phase_frequency, SignalUnit::Degree),
    ] {
        if margin.unit != Some(unit) || frequency.unit != Some(SignalUnit::Hertz) {
            return Err(invalid("STB margin and frequency units are inconsistent"));
        }
        match (&margin.value, &frequency.value) {
            (
                ScalarValue::Real { value: Some(value) },
                ScalarValue::Real {
                    value: Some(frequency),
                },
            ) if value.is_finite() && frequency.is_finite() && *frequency > 0.0 => {}
            (value, frequency) if *value == absent && *frequency == absent => {}
            _ => {
                return Err(invalid(
                    "a measured STB margin requires a finite value and a positive crossover frequency; absent pairs require NoCrossover",
                ));
            }
        }
    }
    if bandwidth.unit != Some(SignalUnit::Hertz) || bandwidth.value != phase_frequency.value {
        return Err(invalid(
            "unity-gain bandwidth must name the phase-margin crossover",
        ));
    }
    let (ScalarValue::Count { value: count }, ScalarValue::Boolean { value: multiple }) =
        (&count.value, &multiple.value)
    else {
        return Err(invalid(
            "STB crossover observations require a count and boolean",
        ));
    };
    if *multiple != (*count > 1) || (*count == 0) != (phase.value == absent) {
        return Err(invalid(
            "STB crossover observations disagree with the measured phase margin",
        ));
    }
    Ok(())
}
