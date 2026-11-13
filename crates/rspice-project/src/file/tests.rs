//! Project parser bounds and decoding checks.

use super::*;

#[test]
fn in_memory_project_text_is_size_checked_before_parsing() {
    assert!(validate_project_text_size(MAX_PROJECT_FILE_BYTES as usize).is_ok());
    let error = validate_project_text_size(MAX_PROJECT_FILE_BYTES as usize + 1)
        .expect_err("oversized project text is rejected");
    assert!(matches!(error, ProjectIoError::InvalidData(_)));
    assert!(error.to_string().contains("supported maximum"));
}
