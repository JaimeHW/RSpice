//! Verilog-A Import Dialog
//!
//! Modal dialog for importing and compiling Verilog-A model files.
//! Provides file selection, compilation progress, and error display.

use dioxus::prelude::*;

use super::veriloga_inspector::{ParameterInfo, VerilogAModelInfo};
use crate::theme::Theme;

/// Import state for tracking compilation progress
#[derive(Debug, Clone, PartialEq)]
pub enum ImportState {
    /// Ready to select a file
    Ready,
    /// Compiling the selected file
    Compiling,
    /// Compilation succeeded
    Success(VerilogAModelInfo),
    /// Compilation failed with error
    Error(String),
}

impl Default for ImportState {
    fn default() -> Self {
        Self::Ready
    }
}

/// Props for VerilogA import dialog
#[derive(Props, Clone, PartialEq)]
pub struct VerilogAImportDialogProps {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Callback to close the dialog
    pub on_close: EventHandler<()>,
    /// Callback when a model is successfully imported
    pub on_import: EventHandler<VerilogAModelInfo>,
}

/// Verilog-A Import Dialog Component
///
/// A modal dialog that allows users to:
/// 1. Select a Verilog-A (.va) file
/// 2. See compilation progress
/// 3. Preview the compiled model
/// 4. Import the model into the library
#[component]
pub fn VerilogAImportDialog(props: VerilogAImportDialogProps) -> Element {
    let import_state = use_signal(ImportState::default);
    let selected_file = use_signal(String::new);
    let source_preview = use_signal(String::new);

    if !props.visible {
        return rsx! {};
    }

    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Theme colors
    let bg_primary = th.bg_primary();
    let bg_secondary = th.bg_secondary();
    let border = th.border();
    let text_primary = th.text_primary();
    let text_secondary = th.text_secondary();
    let text_muted = th.text_muted();
    let accent_primary = th.accent_primary();
    let accent_success = th.accent_success();
    let accent_error = th.accent_error();

    let on_close = props.on_close.clone();
    let on_import = props.on_import.clone();

    // File picker handler
    let handle_file_select = move |_| {
        #[cfg(feature = "rfd")]
        {
            let mut import_state = import_state.clone();
            let mut selected_file = selected_file.clone();
            let mut source_preview = source_preview.clone();

            // Spawn async file dialog
            spawn(async move {
                if let Some(file) = rfd::AsyncFileDialog::new()
                    .add_filter("Verilog-A", &["va"])
                    .add_filter("All files", &["*"])
                    .pick_file()
                    .await
                {
                    let path = file.path().to_string_lossy().to_string();
                    selected_file.set(path.clone());

                    // Read file preview
                    if let Ok(content) = std::fs::read_to_string(file.path()) {
                        let preview: String =
                            content.lines().take(20).collect::<Vec<_>>().join("\n");
                        source_preview.set(preview);
                    }

                    // Compile the file
                    import_state.set(ImportState::Compiling);

                    let compiler = rspice_veriloga::VerilogACompiler::default();
                    match compiler.compile_file(file.path()) {
                        Ok(model) => {
                            let info = VerilogAModelInfo::from(&model);
                            import_state.set(ImportState::Success(info));
                        }
                        Err(e) => {
                            import_state.set(ImportState::Error(e.to_string()));
                        }
                    }
                }
            });
        }
    };

    // Import handler
    let handle_import = {
        let import_state = import_state.clone();
        move |_| {
            if let ImportState::Success(ref info) = *import_state.read() {
                on_import.call(info.clone());
                on_close.call(());
            }
        }
    };

    rsx! {
        // Backdrop
        div {
            style: "position: fixed; inset: 0; background: rgba(0,0,0,0.6); \
                    z-index: 1000; display: flex; align-items: center; justify-content: center;",
            onclick: move |_| on_close.call(()),

            // Dialog
            div {
                style: format!("width: 500px; max-height: 80vh; background: {bg_secondary}; \
                        border: 1px solid {border}; border-radius: {}; \
                        box-shadow: 0 8px 32px rgba(0,0,0,0.4); overflow: hidden;",
                        Theme::RADIUS_LG),
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    style: format!("display: flex; justify-content: space-between; align-items: center; \
                            padding: 16px 20px; border-bottom: 1px solid {border};"),

                    span {
                        style: format!("font-size: {}; font-weight: 600; color: {text_primary};",
                                      Theme::FONT_SIZE_LG),
                        "Import Verilog-A Model"
                    }

                    button {
                        style: "background: none; border: none; color: {text_muted}; \
                                cursor: pointer; font-size: 18px; padding: 4px;",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                // Content
                div {
                    style: "padding: 20px; max-height: 60vh; overflow-y: auto;",

                    // File selection
                    div {
                        style: "margin-bottom: 20px;",

                        label {
                            style: format!("display: block; font-size: {}; color: {text_secondary}; margin-bottom: 8px;",
                                          Theme::FONT_SIZE_SM),
                            "Verilog-A Source File"
                        }

                        div {
                            style: "display: flex; gap: 8px;",

                            input {
                                r#type: "text",
                                readonly: true,
                                value: "{selected_file}",
                                placeholder: "Select a .va file...",
                                style: format!("flex: 1; padding: 8px 12px; background: {bg_primary}; \
                                        border: 1px solid {border}; border-radius: {}; \
                                        color: {text_primary}; font-size: {};",
                                        Theme::RADIUS_SM, Theme::FONT_SIZE_SM),
                            }

                            button {
                                style: format!("padding: 8px 16px; background: {accent_primary}; \
                                        border: none; border-radius: {}; color: white; \
                                        font-size: {}; font-weight: 500; cursor: pointer;",
                                        Theme::RADIUS_SM, Theme::FONT_SIZE_SM),
                                onclick: handle_file_select,
                                "Browse..."
                            }
                        }
                    }

                    // Source preview
                    if !source_preview.read().is_empty() {
                        div {
                            style: "margin-bottom: 20px;",

                            label {
                                style: format!("display: block; font-size: {}; color: {text_secondary}; margin-bottom: 8px;",
                                              Theme::FONT_SIZE_SM),
                                "Source Preview"
                            }

                            pre {
                                style: format!("padding: 12px; background: {bg_primary}; \
                                        border-radius: {}; font-family: {}; font-size: 11px; \
                                        color: {text_muted}; overflow-x: auto; max-height: 150px;",
                                        Theme::RADIUS_SM, Theme::FONT_MONO),
                                "{source_preview}"
                            }
                        }
                    }

                    // State-dependent content
                    match &*import_state.read() {
                        ImportState::Ready => rsx! {},

                        ImportState::Compiling => rsx! {
                            div {
                                style: format!("padding: 20px; text-align: center; color: {text_secondary};"),
                                div {
                                    style: "font-size: 24px; margin-bottom: 8px; animation: spin 1s linear infinite;",
                                    "⟳"
                                }
                                "Compiling..."
                            }
                        },

                        ImportState::Success(info) => rsx! {
                            div {
                                style: format!("padding: 16px; background: {accent_success}15; \
                                        border: 1px solid {accent_success}33; border-radius: {};",
                                        Theme::RADIUS_MD),

                                div {
                                    style: format!("display: flex; align-items: center; gap: 8px; \
                                            margin-bottom: 12px; color: {accent_success}; font-weight: 600;"),
                                    span { "✓" }
                                    span { "Compilation Successful" }
                                }

                                div {
                                    style: "display: grid; grid-template-columns: auto 1fr; gap: 8px 16px; \
                                            font-size: 12px;",

                                    span { style: "color: {text_secondary};", "Model:" }
                                    span { style: "color: {text_primary}; font-weight: 500;", "{info.name}" }

                                    span { style: "color: {text_secondary};", "Terminals:" }
                                    span { style: "color: {text_primary};", "{info.terminals.len()}" }

                                    span { style: "color: {text_secondary};", "Parameters:" }
                                    span { style: "color: {text_primary};", "{info.parameters.len()}" }
                                }
                            }
                        },

                        ImportState::Error(error) => rsx! {
                            div {
                                style: format!("padding: 16px; background: {accent_error}15; \
                                        border: 1px solid {accent_error}33; border-radius: {};",
                                        Theme::RADIUS_MD),

                                div {
                                    style: format!("display: flex; align-items: center; gap: 8px; \
                                            margin-bottom: 12px; color: {accent_error}; font-weight: 600;"),
                                    span { "✗" }
                                    span { "Compilation Failed" }
                                }

                                pre {
                                    style: format!("font-family: {}; font-size: 11px; color: {accent_error}; \
                                            white-space: pre-wrap; margin: 0;", Theme::FONT_MONO),
                                    "{error}"
                                }
                            }
                        },
                    }
                }

                // Footer
                div {
                    style: format!("display: flex; justify-content: flex-end; gap: 12px; \
                            padding: 16px 20px; border-top: 1px solid {border};"),

                    button {
                        style: format!("padding: 8px 20px; background: transparent; \
                                border: 1px solid {border}; border-radius: {}; \
                                color: {text_secondary}; font-size: {}; cursor: pointer;",
                                Theme::RADIUS_SM, Theme::FONT_SIZE_SM),
                        onclick: move |_| on_close.call(()),
                        "Cancel"
                    }

                    if matches!(*import_state.read(), ImportState::Success(_)) {
                        button {
                            style: format!("padding: 8px 20px; background: {accent_success}; \
                                    border: none; border-radius: {}; color: white; \
                                    font-size: {}; font-weight: 500; cursor: pointer;",
                                    Theme::RADIUS_SM, Theme::FONT_SIZE_SM),
                            onclick: handle_import,
                            "Import Model"
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_state_default() {
        let state = ImportState::default();
        assert!(matches!(state, ImportState::Ready));
    }

    #[test]
    fn test_import_state_compiling() {
        let state = ImportState::Compiling;
        assert!(matches!(state, ImportState::Compiling));
    }

    #[test]
    fn test_import_state_success() {
        let info = VerilogAModelInfo {
            name: "test_model".to_string(),
            source_path: "test.va".to_string(),
            is_compiled: true,
            error: None,
            terminals: vec!["p".to_string(), "n".to_string()],
            parameters: vec![],
            internal_nodes: 0,
        };
        let state = ImportState::Success(info.clone());
        if let ImportState::Success(ref s) = state {
            assert_eq!(s.name, "test_model");
            assert_eq!(s.terminals.len(), 2);
        } else {
            panic!("Expected Success state");
        }
    }

    #[test]
    fn test_import_state_error() {
        let state = ImportState::Error("Syntax error on line 5".to_string());
        if let ImportState::Error(ref e) = state {
            assert!(e.contains("Syntax error"));
        } else {
            panic!("Expected Error state");
        }
    }

    #[test]
    fn test_import_state_equality() {
        assert_eq!(ImportState::Ready, ImportState::Ready);
        assert_eq!(ImportState::Compiling, ImportState::Compiling);
        assert_ne!(ImportState::Ready, ImportState::Compiling);
    }
}
