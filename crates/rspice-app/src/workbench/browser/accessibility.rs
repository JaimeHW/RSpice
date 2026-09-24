//! Browser DOM bridge for accessibility bootstrap preferences.

const SPOKEN_FEEDBACK_ATTRIBUTE: &str = "data-rspice-spoken-feedback";
const CONTEXT_MENU_REQUEST_ATTRIBUTE: &str = "data-rspice-context-menu-request";

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

/// Push an in-canvas preference change back to the semantic DOM bootstrap
/// control. Its MutationObserver updates the button state and browser storage.
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

/// Consume the browser host's one-shot Shift+F10 request when the application
/// canvas currently owns keyboard focus.
///
/// Chromium may reserve the native context-menu gesture before eframe exposes
/// a usable `F10` key event. The host page records that exact gesture on the
/// document root; the schematic menu consumes it here so target resolution
/// still follows the selected/focused engineering object rather than an
/// arbitrary pointer coordinate.
pub(crate) fn take_schematic_context_menu_request() -> bool {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return false;
    };
    if document
        .active_element()
        .and_then(|element| element.get_attribute("id"))
        .as_deref()
        != Some("rspice_canvas")
    {
        return false;
    }
    let Some(root) = document.document_element() else {
        return false;
    };
    if root
        .get_attribute(CONTEXT_MENU_REQUEST_ATTRIBUTE)
        .as_deref()
        != Some("true")
    {
        return false;
    }
    if let Err(error) = root.remove_attribute(CONTEXT_MENU_REQUEST_ATTRIBUTE) {
        log::warn!("Failed to consume browser schematic context-menu request: {error:?}");
    }
    true
}

/// Mirror the active high-level application context into a semantic DOM live
/// region. Egui still owns widget interaction, while this bridge gives browser
/// assistive technology a stable landmark and announces only genuine context
/// changes instead of every paint frame.
pub(crate) fn publish_workspace_context(workspace: &str, document: &str, running: bool) {
    let Some(document_object) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let status = if running {
        format!("RSpice {workspace} workspace. Active document {document}. Simulation running.")
    } else {
        format!("RSpice {workspace} workspace. Active document {document}.")
    };
    if let Some(region) = document_object.get_element_by_id("rspice_semantic_context")
        && region.text_content().as_deref() != Some(status.as_str())
    {
        region.set_text_content(Some(&status));
    }
    if let Some(canvas) = document_object.get_element_by_id("rspice_canvas") {
        let label = format!("RSpice {workspace} workspace, {document}");
        if canvas.get_attribute("aria-label").as_deref() != Some(label.as_str())
            && let Err(error) = canvas.set_attribute("aria-label", &label)
        {
            log::warn!("Failed to synchronize browser canvas label: {error:?}");
        }
        let busy = if running { "true" } else { "false" };
        if canvas.get_attribute("aria-busy").as_deref() != Some(busy)
            && let Err(error) = canvas.set_attribute("aria-busy", busy)
        {
            log::warn!("Failed to synchronize browser canvas busy state: {error:?}");
        }
    }
}
