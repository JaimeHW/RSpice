//! Shared identities for typed result conversion and publication manifests.

/// A descriptor alone does not identify a response: several sidebands or
/// distortion products deliberately share it. Preserve the qualifier using
/// the direct PAC/DISTO export spellings where applicable.
pub(super) fn qualified_name(
    signal: &rspice_core::execution::result_document::ResultSignal,
) -> String {
    use rspice_core::execution::result_document::{DistortionTone, SeriesQualifier};
    let name = signal.descriptor().display_name();
    match signal.qualifier() {
        None => name.to_string(),
        Some(SeriesQualifier::PacSideband { sideband }) => format!("{name}:sb{sideband}"),
        Some(SeriesQualifier::DistortionFundamental { tone }) => {
            let tone = match tone {
                DistortionTone::F1 => "f1",
                DistortionTone::F2 => "f2",
            };
            format!("peak({tone}:{name})")
        }
        Some(SeriesQualifier::DistortionProduct { product }) => {
            format!("peak({}:{name})", product.label())
        }
        Some(SeriesQualifier::PxfConversion { input, output }) => {
            format!("{name}:sb{input}->sb{output}")
        }
    }
}

/// A manifest describes one sample of each series. Its schema therefore stays
/// compatible when coordinates retain different numbers of points, including
/// a one-point result or early model completion.
pub(super) fn point_descriptor(
    signal: &rspice_core::execution::result_document::ResultSignal,
) -> Result<rspice_core::execution::SignalDescriptor, rspice_core::execution::SignalSchemaError> {
    use rspice_core::execution::{SignalDescriptor, SignalShape};
    let descriptor = signal.descriptor();
    let name = qualified_name(signal);
    let canonical = if signal.qualifier().is_some() {
        &name
    } else {
        descriptor.canonical_name()
    };
    SignalDescriptor::new(
        canonical,
        &name,
        descriptor.kind(),
        descriptor.unit().clone(),
        descriptor.value_type(),
        SignalShape::Scalar,
        descriptor.owner().clone(),
    )
}
