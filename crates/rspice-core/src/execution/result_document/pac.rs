//! Cross-check PAC coordinates and conversion identities before publication.

use super::*;
use crate::analysis::pac::sideband_frequency;

fn malformed(detail: impl Into<String>) -> ResultDocumentError {
    ResultDocumentError::Malformed {
        location: "PAC result",
        detail: detail.into(),
    }
}

pub(super) fn validate(
    document: &AnalysisResultDocument,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    let ResultPayload::Pac(payload) = &document.payload else {
        return Ok(());
    };
    check_abort(abort)?;
    if !payload.fundamental_frequency.is_finite() || payload.fundamental_frequency <= 0.0 {
        return Err(malformed(
            "fundamental frequency must be finite and positive",
        ));
    }
    if !payload.residual.is_finite() || payload.residual < 0.0 {
        return Err(malformed("residual norm must be finite and nonnegative"));
    }
    let range = payload.sideband_minimum..=payload.sideband_maximum;
    if range.is_empty() || payload.sidebands.is_empty() {
        return Err(malformed(
            "a PAC result requires a nonempty sideband range and retained sidebands",
        ));
    }
    let [axis] = document.axes.as_slice() else {
        return Err(malformed(
            "a PAC result requires exactly one frequency-offset axis",
        ));
    };
    if axis.kind != ResultAxisKind::OffsetFrequency || axis.unit != SignalUnit::Hertz {
        return Err(malformed("the PAC axis must be frequency offset in hertz"));
    }
    let AxisValues::Real { values: offsets } = &axis.values else {
        return Err(malformed(
            "the PAC axis must contain real frequency offsets",
        ));
    };
    if offsets.is_empty() {
        return Err(malformed("a PAC sweep needs at least one frequency offset"));
    }
    for (index, &offset) in offsets.iter().enumerate() {
        if index % ABORT_POLL_STRIDE == 0 {
            check_abort(abort)?;
        }
        if !offset.is_finite() || offset < 0.0 {
            return Err(malformed(format!(
                "frequency offset at index {index} must be finite and nonnegative"
            )));
        }
        for endpoint in [payload.sideband_minimum, payload.sideband_maximum] {
            if !sideband_frequency(endpoint, payload.fundamental_frequency, offset).is_finite() {
                return Err(malformed(format!(
                    "sideband {endpoint} at index {index} has a non-representable absolute frequency"
                )));
            }
        }
    }

    let mut previous = None;
    for band in &payload.sidebands {
        check_abort(abort)?;
        if !range.contains(&band.sideband) || previous.is_some_and(|prior| prior >= band.sideband) {
            return Err(malformed(
                "retained sidebands must be unique, ascending, and inside the declared range",
            ));
        }
        previous = Some(band.sideband);
        if band.frequency_offsets.len() != offsets.len()
            || band.absolute_frequencies.len() != offsets.len()
        {
            return Err(malformed(format!(
                "sideband {} frequency count disagrees with the primary axis",
                band.sideband
            )));
        }
        for (index, ((&offset, &retained_offset), &absolute)) in offsets
            .iter()
            .zip(&band.frequency_offsets)
            .zip(&band.absolute_frequencies)
            .enumerate()
        {
            if index % ABORT_POLL_STRIDE == 0 {
                check_abort(abort)?;
            }
            if retained_offset != offset
                || absolute
                    != sideband_frequency(band.sideband, payload.fundamental_frequency, offset)
            {
                return Err(malformed(format!(
                    "sideband {} coordinates at index {index} disagree with the carrier and offset axis",
                    band.sideband
                )));
            }
        }
    }
    for (index, signal) in document.signals.iter().enumerate() {
        if index % ABORT_POLL_STRIDE == 0 {
            check_abort(abort)?;
        }
        match signal.qualifier {
            None => {}
            Some(SeriesQualifier::PacSideband { sideband })
                if payload
                    .sidebands
                    .binary_search_by_key(&sideband, |band| band.sideband)
                    .is_ok() => {}
            _ => {
                return Err(malformed(
                    "signal qualifier does not name a retained PAC sideband",
                ));
            }
        }
    }
    if let Some(matrix) = &payload.conversion_matrix {
        check_abort(abort)?;
        // The retained matrix may be sparse, including rows for sideband zero
        // when its node spectra were withheld. Validate only retained paths;
        // never allocate the full sideband-square population on import.
        let mut paths = std::collections::HashSet::new();
        paths
            .try_reserve(matrix.entries.len())
            .map_err(|_| ResultDocumentError::AllocationFailed)?;
        for (index, entry) in matrix.entries.iter().enumerate() {
            if index % ABORT_POLL_STRIDE == 0 {
                check_abort(abort)?;
            }
            if entry.frequency_index >= offsets.len()
                || !range.contains(&entry.input_sideband)
                || !range.contains(&entry.output_sideband)
            {
                return Err(malformed(
                    "conversion matrix entry is outside the frequency or sideband range",
                ));
            }
            if !paths.insert((
                entry.frequency_index,
                entry.input_sideband,
                entry.output_sideband,
            )) {
                return Err(malformed(
                    "conversion matrix repeats a path at one frequency",
                ));
            }
            finite("PAC conversion element real part", entry.value.real)?;
            finite(
                "PAC conversion element imaginary part",
                entry.value.imaginary,
            )?;
        }
    }
    check_abort(abort)
}
