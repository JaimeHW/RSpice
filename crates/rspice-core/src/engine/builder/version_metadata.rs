/// Convert SPICE model VERSION metadata to the finite CMC-style scalar used
/// by the native and generated compact models.
///
/// Xyce preserves ordinary decimal input, but for versions containing more
/// than one dot it recursively converts the suffix and weights it by 0.1. The
/// grouping is intentionally component-based rather than digit concatenation:
/// `4.10.6` becomes `4 + 0.1 * 10.6 == 5.06`.
pub(super) fn parse_dotted_version_metadata(value: &str) -> Option<f64> {
    let normalized = value.trim().trim_matches('"').trim_matches('\'').trim();
    if normalized.is_empty() {
        return None;
    }
    if let Ok(version) = normalized.parse::<f64>() {
        return version.is_finite().then_some(version);
    }

    let parts = normalized.split('.').collect::<Vec<_>>();
    if parts.len() < 3
        || parts
            .iter()
            .any(|part| part.is_empty() || !part.chars().all(|ch| ch.is_ascii_digit()))
    {
        return None;
    }

    let tail = format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1])
        .parse::<f64>()
        .ok()
        .filter(|version| version.is_finite())?;
    parts[..parts.len() - 2]
        .iter()
        .rev()
        .try_fold(tail, |suffix, part| {
            let component = part.parse::<f64>().ok().filter(|value| value.is_finite())?;
            let version = component + 0.1 * suffix;
            version.is_finite().then_some(version)
        })
}
