//! Shared library-dialog helpers, including library/cell/view name
//! validation.

pub(crate) const LIBRARY_CATALOG_STALE_MESSAGE: &str =
    "The library catalog changed after this dialog opened. Cancel and reopen the operation.";

pub(in crate::workbench) fn validate_lcv_name(name: &str, field_label: &str) -> Option<String> {
    if name.is_empty() {
        return Some(format!("{field_label} cannot be empty"));
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Some(format!(
            "{field_label} must contain only letters, numbers, and underscores"
        ));
    }
    None
}
