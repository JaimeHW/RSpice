/// Name-based selection shared by interactive and prepared print consumers.
/// The index fallback is only for initial selection and older print snapshots.
pub fn measurement_index(
    selected: Option<&str>,
    legacy_index: usize,
    names: &[&str],
) -> Option<usize> {
    let target = selected.or_else(|| {
        names
            .get(legacy_index.min(names.len().saturating_sub(1)))
            .copied()
    })?;
    let mut matches = names
        .iter()
        .enumerate()
        .filter(|(_, name)| **name == target);
    let index = matches.next()?.0;
    matches.next().is_none().then_some(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_measurement_survives_reordering_and_never_substitutes_a_missing_name() {
        assert_eq!(
            measurement_index(Some("gain"), 0, &["gain", "offset"]),
            Some(0)
        );
        assert_eq!(
            measurement_index(Some("gain"), 0, &["offset", "gain"]),
            Some(1)
        );
        assert_eq!(measurement_index(Some("gain"), 0, &["offset"]), None);
        assert_eq!(measurement_index(Some("gain"), 0, &["gain", "gain"]), None);
        assert_eq!(measurement_index(None, 0, &["offset", "gain"]), Some(0));
        assert_eq!(
            measurement_index(None, usize::MAX, &["offset", "gain"]),
            Some(1)
        );
        assert_eq!(measurement_index(None, 0, &[]), None);
    }
}
