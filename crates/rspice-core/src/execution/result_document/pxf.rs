//! Cross-check the retained coordinates of one periodic conversion path.

use super::*;
use crate::analysis::pac::sideband_frequency;

fn malformed(detail: impl Into<String>) -> ResultDocumentError {
    ResultDocumentError::Malformed {
        location: "PXF result",
        detail: detail.into(),
    }
}

pub(super) fn validate(
    document: &AnalysisResultDocument,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    let ResultPayload::Pxf(payload) = &document.payload else {
        return Ok(());
    };
    check_abort(abort)?;
    if !payload.fundamental_frequency.is_finite() || payload.fundamental_frequency <= 0.0 {
        return Err(malformed(
            "fundamental frequency must be finite and positive",
        ));
    }
    require_name("PXF input source", &payload.input_source)?;
    require_name("PXF output node", &payload.output_node)?;
    if let Some(reference) = &payload.reference_node {
        require_name("PXF reference node", reference)?;
    }
    if payload.max_sideband < 0 {
        return Err(malformed("a sideband span cannot be negative"));
    }
    // Negating the nonnegative depth is safe, including i32::MAX. In contrast,
    // saturating_abs would admit i32::MIN into that symmetric span.
    let span = -payload.max_sideband..=payload.max_sideband;
    if !span.contains(&payload.input_sideband) || !span.contains(&payload.output_sideband) {
        return Err(malformed(
            "conversion path is outside the analyzed sideband span",
        ));
    }
    let [axis] = document.axes.as_slice() else {
        return Err(malformed(
            "a PXF result requires exactly one frequency-offset axis",
        ));
    };
    if axis.kind != ResultAxisKind::OffsetFrequency || axis.unit != SignalUnit::Hertz {
        return Err(malformed("the PXF axis must be frequency offset in hertz"));
    }
    let AxisValues::Real { values: offsets } = &axis.values else {
        return Err(malformed(
            "the PXF axis must contain real frequency offsets",
        ));
    };
    if offsets.is_empty() {
        return Err(malformed("a PXF sweep needs at least one frequency offset"));
    }
    let mut previous = 0.0;
    for (index, &offset) in offsets.iter().enumerate() {
        if index % ABORT_POLL_STRIDE == 0 {
            check_abort(abort)?;
        }
        if !offset.is_finite() || offset <= previous {
            return Err(malformed(
                "frequency offsets must be finite, positive, and strictly increasing",
            ));
        }
        previous = offset;
        for sideband in [payload.input_sideband, payload.output_sideband] {
            if !sideband_frequency(sideband, payload.fundamental_frequency, offset).is_finite() {
                return Err(malformed(
                    "conversion path has a non-representable absolute frequency",
                ));
            }
        }
    }
    for (index, signal) in document.signals.iter().enumerate() {
        if index % ABORT_POLL_STRIDE == 0 {
            check_abort(abort)?;
        }
        match signal.qualifier {
            None => {}
            Some(SeriesQualifier::PxfConversion { input, output })
                if input == payload.input_sideband && output == payload.output_sideband => {}
            _ => {
                return Err(malformed(
                    "signal qualifier disagrees with the PXF conversion path",
                ));
            }
        }
        if signal.descriptor.canonical_name() != "output_frequency" {
            continue;
        }
        if signal.descriptor.unit() != &SignalUnit::Hertz || signal.qualifier.is_some() {
            return Err(malformed(
                "output frequency must be an unqualified coordinate in hertz",
            ));
        }
        let SeriesValues::Real { samples } = &signal.values else {
            return Err(malformed("output frequency must contain real coordinates"));
        };
        // Projection may omit samples or this entire coordinate. Check every
        // retained value; never fill missing evidence with a calculated value.
        for (index, (&offset, sample)) in offsets.iter().zip(samples).enumerate() {
            if index % ABORT_POLL_STRIDE == 0 {
                check_abort(abort)?;
            }
            if sample.is_some_and(|value| {
                value
                    != sideband_frequency(
                        payload.output_sideband,
                        payload.fundamental_frequency,
                        offset,
                    )
            }) {
                return Err(malformed(
                    "output frequency disagrees with the carrier and offset axis",
                ));
            }
        }
    }
    // Delay samples use interval midpoints, not the primary offset grid. A
    // projection may retain a subsequence, but must preserve its coordinates
    // and order. Scan once without allocating another frequency array.
    let mut intervals = offsets.iter().zip(offsets.iter().skip(1)).enumerate();
    for sample in &payload.group_delay {
        finite("PXF group-delay frequency", sample.frequency)?;
        finite("PXF group delay", sample.delay)?;
        let mut matched = false;
        for (index, (&start, &stop)) in intervals.by_ref() {
            if index % ABORT_POLL_STRIDE == 0 {
                check_abort(abort)?;
            }
            let midpoint = start.midpoint(stop);
            if sample.frequency == midpoint {
                matched = true;
                break;
            }
            if sample.frequency < midpoint {
                break;
            }
        }
        if !matched {
            return Err(malformed(
                "group-delay coordinates must follow the swept interval midpoints",
            ));
        }
    }
    if let Some(gain) = payload.dc_gain {
        finite("PXF DC gain real part", gain.real)?;
        finite("PXF DC gain imaginary part", gain.imaginary)?;
    }
    check_abort(abort)
}
