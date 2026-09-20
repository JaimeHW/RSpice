//! Shared test selection, independent of the offline generator's corpus loader.

/// Comma-separated model names selected by the shipped-model oracle runner.
pub(crate) fn shipped_model_filter() -> Option<String> {
    std::env::var("RSPICE_NATIVE_SHIPPED_MODEL_FILTER").ok()
}

pub(crate) fn shipped_model_filter_allows(name: &str) -> bool {
    let Some(filter) = shipped_model_filter() else {
        return true;
    };
    filter
        .split(',')
        .map(str::trim)
        .any(|candidate| candidate.eq_ignore_ascii_case(name))
}
