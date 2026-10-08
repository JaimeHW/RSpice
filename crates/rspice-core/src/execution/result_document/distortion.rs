//! Validate tone and product identities together with their physical coordinates.

use super::*;
use crate::analysis::DistortionProduct;
use crate::analysis::distortion::{checked_second_frequency, validate_frequency};

fn malformed(detail: impl Into<String>) -> ResultDocumentError {
    ResultDocumentError::Malformed {
        location: "distortion result",
        detail: detail.into(),
    }
}

pub(super) fn validate(
    document: &AnalysisResultDocument,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    let ResultPayload::Distortion(payload) = &document.payload else {
        return Ok(());
    };
    check_abort(abort)?;
    let [axis] = document.axes.as_slice() else {
        return Err(malformed(
            "a distortion sweep requires exactly one F1 frequency axis",
        ));
    };
    if axis.kind != ResultAxisKind::Frequency || axis.unit != SignalUnit::Hertz {
        return Err(malformed(
            "the distortion F1 axis must be frequency in hertz",
        ));
    }
    let AxisValues::Real {
        values: frequencies,
    } = &axis.values
    else {
        return Err(malformed(
            "the distortion F1 axis must contain real frequencies",
        ));
    };
    let first = *frequencies
        .first()
        .ok_or_else(|| malformed("a distortion sweep needs at least one point"))?;
    let f2 = payload
        .f2_over_f1
        .map(|ratio| checked_second_frequency(first, ratio))
        .transpose()
        .map_err(malformed)?;
    for (index, &f1) in frequencies.iter().enumerate() {
        if index % ABORT_POLL_STRIDE == 0 {
            check_abort(abort)?;
        }
        validate_frequency(f1, f2)
            .map_err(|detail| malformed(format!("{detail} at F1 index {index} ({f1})")))?;
    }

    let allowed = DistortionProduct::for_mode(f2.is_some());
    if payload.products.len() > allowed.len() {
        return Err(malformed("too many distortion products for this tone mode"));
    }
    // There are at most six products. A short prefix scan avoids allocating
    // a second inventory and still permits intentionally projected subsets.
    for (index, product) in payload.products.iter().enumerate() {
        check_abort(abort)?;
        let identity = product.product.to_core();
        if !allowed.contains(&identity)
            || payload
                .products
                .iter()
                .take(index)
                .any(|prior| prior.product == product.product)
        {
            return Err(malformed(format!(
                "duplicate or unsupported distortion product {}",
                identity.label()
            )));
        }
        if product.order != identity.order() {
            return Err(malformed(format!(
                "product {} must have Volterra order {}",
                identity.label(),
                identity.order()
            )));
        }
        if product.frequencies.len() != frequencies.len() {
            return Err(malformed(format!(
                "product {} frequency count disagrees with F1",
                identity.label()
            )));
        }
        for (row, (&f1, &actual)) in frequencies.iter().zip(&product.frequencies).enumerate() {
            if row % ABORT_POLL_STRIDE == 0 {
                check_abort(abort)?;
            }
            if identity.physical_frequency(f1, f2) != Some(actual) {
                return Err(malformed(format!(
                    "product {} frequency at index {row} disagrees with F1 and F2",
                    identity.label()
                )));
            }
        }
    }
    for (index, signal) in document.signals.iter().enumerate() {
        if index % ABORT_POLL_STRIDE == 0 {
            check_abort(abort)?;
        }
        match signal.qualifier {
            None
            | Some(SeriesQualifier::DistortionFundamental {
                tone: DistortionTone::F1,
            }) => {}
            Some(SeriesQualifier::DistortionFundamental {
                tone: DistortionTone::F2,
            }) if f2.is_some() => {}
            Some(SeriesQualifier::DistortionProduct { product })
                if payload
                    .products
                    .iter()
                    .any(|entry| entry.product == product) => {}
            _ => {
                return Err(malformed(
                    "signal qualifier does not name a retained distortion tone or product",
                ));
            }
        }
    }
    check_abort(abort)
}
