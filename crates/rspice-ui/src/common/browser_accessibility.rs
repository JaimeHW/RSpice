//! Browser DOM bridge for accessibility bootstrap preferences.

const SPOKEN_FEEDBACK_ATTRIBUTE: &str = "data-rspice-spoken-feedback";

/// Read the DOM-owned spoken-feedback preference, if the browser shell has
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

/// Push a preference change made inside egui back to the semantic DOM
/// bootstrap control. Its MutationObserver updates the button and storage.
pub(crate) fn set_spoken_feedback(enabled: bool) {
    let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    else {
        return;
    };
    if let Err(error) = root.set_attribute(
        SPOKEN_FEEDBACK_ATTRIBUTE,
        if enabled { "true" } else { "false" },
    ) {
        log::warn!("Failed to synchronize browser spoken feedback: {error:?}");
    }
}
