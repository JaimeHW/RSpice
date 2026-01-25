//! About Dialog
//!
//! Professional about dialog displaying version, copyright, and system information.
//! Follows commercial EDA patterns with comprehensive product information.

use dioxus::prelude::*;

use crate::theme::Theme;

//=============================================================================
// About Dialog Component
//=============================================================================

/// Props for the about dialog
#[derive(Props, Clone, PartialEq)]
pub struct AboutDialogProps {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Handler for closing the dialog
    pub on_close: EventHandler<()>,
}

/// About dialog component
#[component]
pub fn AboutDialog(props: AboutDialogProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    let theme = use_context::<Signal<Theme>>();

    rsx! {
        // Backdrop
        div {
            class: "about-dialog-backdrop",
            style: "
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.7);
                display: flex;
                align-items: center;
                justify-content: center;
                z-index: 10000;
            ",
            onclick: move |_| props.on_close.call(()),

            // Dialog
            div {
                class: "about-dialog",
                style: format!(
                    "
                    background: {};
                    border: 1px solid {};
                    border-radius: 12px;
                    padding: 32px;
                    min-width: 480px;
                    max-width: 560px;
                    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
                    ",
                    theme.read().bg_secondary(),
                    theme.read().border()
                ),
                onclick: move |e| e.stop_propagation(),

                // Header with logo and title
                div {
                    style: "
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        margin-bottom: 24px;
                    ",

                    // Logo placeholder
                    div {
                        style: format!(
                            "
                            width: 80px;
                            height: 80px;
                            border-radius: 16px;
                            background: linear-gradient(135deg, {} 0%, {} 100%);
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            font-size: 36px;
                            font-weight: bold;
                            color: white;
                            margin-bottom: 16px;
                            ",
                            theme.read().accent_primary(),
                            "#6366f1"
                        ),
                        "R"
                    }

                    // Product name
                    h1 {
                        style: format!(
                            "
                            font-size: 28px;
                            font-weight: 700;
                            color: {};
                            margin: 0 0 4px 0;
                            ",
                            theme.read().text_primary()
                        ),
                        "RSpice"
                    }

                    // Tagline
                    p {
                        style: format!(
                            "
                            font-size: 14px;
                            color: {};
                            margin: 0;
                            ",
                            theme.read().text_secondary()
                        ),
                        "The Circuit Simulator, Reimagined"
                    }
                }

                // Version info
                div {
                    style: format!(
                        "
                        background: {};
                        border-radius: 8px;
                        padding: 16px;
                        margin-bottom: 24px;
                        ",
                        theme.read().bg_tertiary()
                    ),

                    InfoRow { label: "Version", value: env!("CARGO_PKG_VERSION").to_string() }
                    InfoRow { label: "Build", value: build_info() }
                    InfoRow { label: "Platform", value: platform_info() }
                    InfoRow { label: "Rust", value: rust_version() }
                }

                // Copyright
                p {
                    style: format!(
                        "
                        text-align: center;
                        font-size: 12px;
                        color: {};
                        margin: 0 0 8px 0;
                        ",
                        theme.read().text_muted()
                    ),
                    "© 2024-2026 RSpice Contributors"
                }

                p {
                    style: format!(
                        "
                        text-align: center;
                        font-size: 11px;
                        color: {};
                        margin: 0 0 24px 0;
                        ",
                        theme.read().text_muted()
                    ),
                    "Licensed under the RSpice Personal Use License"
                }

                // Close button
                button {
                    style: format!(
                        "
                        width: 100%;
                        padding: 12px 24px;
                        background: {};
                        border: none;
                        border-radius: 8px;
                        color: white;
                        font-size: 14px;
                        font-weight: 500;
                        cursor: pointer;
                        transition: opacity 0.15s ease;
                        ",
                        theme.read().accent_primary()
                    ),
                    onclick: move |_| props.on_close.call(()),
                    "Close"
                }
            }
        }
    }
}

//=============================================================================
// Helper Components
//=============================================================================

#[derive(Props, Clone, PartialEq)]
struct InfoRowProps {
    label: &'static str,
    value: String,
}

#[component]
fn InfoRow(props: InfoRowProps) -> Element {
    let theme = use_context::<Signal<Theme>>();

    rsx! {
        div {
            style: "
                display: flex;
                justify-content: space-between;
                padding: 6px 0;
            ",

            span {
                style: format!("color: {};", theme.read().text_secondary()),
                "{props.label}"
            }

            span {
                style: format!(
                    "
                    color: {};
                    font-family: {};
                    ",
                    theme.read().text_primary(),
                    Theme::FONT_MONO
                ),
                "{props.value}"
            }
        }
    }
}

//=============================================================================
// Info Helpers
//=============================================================================

/// Get build information
fn build_info() -> String {
    // In production, use build-time info
    #[cfg(debug_assertions)]
    return "Debug".to_string();

    #[cfg(not(debug_assertions))]
    return "Release".to_string();
}

/// Get platform information
fn platform_info() -> String {
    #[cfg(target_os = "windows")]
    return "Windows".to_string();

    #[cfg(target_os = "macos")]
    return "macOS".to_string();

    #[cfg(target_os = "linux")]
    return "Linux".to_string();

    #[cfg(target_arch = "wasm32")]
    return "Web (WASM)".to_string();

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_arch = "wasm32"
    )))]
    return std::env::consts::OS.to_string();
}

/// Get Rust version
fn rust_version() -> String {
    // This would need to be set at build time for accurate info
    "1.85+".to_string()
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_info_not_empty() {
        let info = build_info();
        assert!(!info.is_empty());
    }

    #[test]
    fn test_platform_info_not_empty() {
        let info = platform_info();
        assert!(!info.is_empty());
    }

    #[test]
    fn test_rust_version_not_empty() {
        let version = rust_version();
        assert!(!version.is_empty());
    }
}
