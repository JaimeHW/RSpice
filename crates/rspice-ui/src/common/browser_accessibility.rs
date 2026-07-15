//! Browser DOM bridge for accessibility bootstrap preferences.

const SPOKEN_FEEDBACK_ATTRIBUTE: &str = "data-rspice-spoken-feedback";

/// Read the DOM-owned spoken-feedback preference, if the host page has
/// published it. The semantic DOM bootstrap control owns this attribute so a
/// screen-reader user can enable feedback before entering egui's canvas.
pub(crate) fn spoken_feedback_override() -> Option<bool> {
    let root = web_sys::window()?.document()?.document_element()?;
    match root.get_attribute(SPOKEN_FEEDBACK_ATTRIBUTE)?.as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}
