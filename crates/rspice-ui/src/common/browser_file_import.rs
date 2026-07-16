#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PickedTextFile {
    pub name: String,
    pub contents: String,
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BrowserTextImportKind {
    Netlist,
    Project,
    Schematic,
    ShortcutProfile,
    VerilogA,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl BrowserTextImportKind {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Netlist => "SPICE deck",
            Self::Project => "project",
            Self::Schematic => "schematic",
            Self::ShortcutProfile => "shortcut profile",
            Self::VerilogA => "Verilog-A source",
        }
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) const fn max_bytes(self) -> u64 {
        match self {
            Self::ShortcutProfile => {
                crate::common::shortcut_profile_workflow::MAX_SHORTCUT_PROFILE_BYTES
            }
            Self::Netlist | Self::Project | Self::Schematic | Self::VerilogA => {
                crate::io::project_io::MAX_PROJECT_FILE_BYTES
            }
        }
    }
}

/// Exact authority for one browser text-picker invocation. JavaScript picker
/// promises are not reliably abortable, so every completion must present this
/// lease before it can mutate application state or release the shared gate.
#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextImportToken {
    generation: u64,
    kind: BrowserTextImportKind,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl TextImportToken {
    #[cfg(target_arch = "wasm32")]
    pub(crate) const fn kind(self) -> BrowserTextImportKind {
        self.kind
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
thread_local! {
    static ACTIVE_TEXT_IMPORT: std::cell::Cell<Option<TextImportToken>> =
        const { std::cell::Cell::new(None) };
    static NEXT_TEXT_IMPORT_GENERATION: std::cell::Cell<u64> = const { std::cell::Cell::new(1) };
    static TEXT_IMPORT_REPAINT_CONTEXT: std::cell::RefCell<Option<egui::Context>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn try_begin_text_import(
    kind: BrowserTextImportKind,
) -> Result<TextImportToken, String> {
    ACTIVE_TEXT_IMPORT.with(|active| match active.get() {
        Some(current) if current.kind == kind => Err(format!(
            "A {} import is already in progress",
            current.kind.label()
        )),
        Some(_) => Err("Another file import is already in progress".to_string()),
        None => {
            let generation = NEXT_TEXT_IMPORT_GENERATION.with(|next| {
                let generation = next.get();
                next.set(generation.wrapping_add(1).max(1));
                generation
            });
            let token = TextImportToken { generation, kind };
            active.set(Some(token));
            Ok(token)
        }
    })
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn finish_text_import(token: TextImportToken) -> bool {
    ACTIVE_TEXT_IMPORT.with(|active| {
        if active.get() == Some(token) {
            active.set(None);
            true
        } else {
            false
        }
    })
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn text_import_is_current(token: TextImportToken) -> bool {
    ACTIVE_TEXT_IMPORT.with(|active| active.get() == Some(token))
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn cancel_active_text_import() -> Option<TextImportToken> {
    ACTIVE_TEXT_IMPORT.with(std::cell::Cell::take)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn active_text_import_kind() -> Option<BrowserTextImportKind> {
    ACTIVE_TEXT_IMPORT.with(|active| active.get().map(TextImportToken::kind))
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn register_text_import_repaint_context(ctx: &egui::Context) {
    TEXT_IMPORT_REPAINT_CONTEXT.with(|slot| {
        *slot.borrow_mut() = Some(ctx.clone());
    });
}

#[cfg(any(test, target_arch = "wasm32"))]
fn request_registered_import_repaint() -> bool {
    TEXT_IMPORT_REPAINT_CONTEXT.with(|slot| {
        let Some(ctx) = slot.borrow().as_ref().cloned() else {
            return false;
        };
        ctx.request_repaint();
        true
    })
}

#[cfg(any(test, target_arch = "wasm32"))]
fn complete_text_import(
    result: Result<Option<PickedTextFile>, String>,
    on_complete: impl FnOnce(Result<Option<PickedTextFile>, String>),
    request_repaint: impl FnOnce(),
) {
    on_complete(result);
    request_repaint();
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn pick_text_file(
    filter_name: &'static str,
    filter_extensions: &'static [&'static str],
    on_complete: impl FnOnce(Result<Option<PickedTextFile>, String>) + 'static,
) {
    // The initiating workflow owns the active import lease. Capture its limit
    // before starting the non-abortable browser promise so a later replacement
    // picker cannot change the authority applied to this read.
    let max_bytes = active_text_import_kind()
        .map(BrowserTextImportKind::max_bytes)
        .unwrap_or(crate::io::project_io::MAX_PROJECT_FILE_BYTES);
    wasm_bindgen_futures::spawn_local(async move {
        let file = rfd::AsyncFileDialog::new()
            .add_filter(filter_name, filter_extensions)
            .pick_file()
            .await;

        let result = match file {
            Some(file) => {
                let name = file.file_name();
                let size = file.inner().size();
                if !size.is_finite() || size < 0.0 || size > max_bytes as f64 {
                    Err(format!(
                        "Selected {} exceeds the supported {}-byte size limit",
                        name, max_bytes
                    ))
                } else {
                    let bytes = file.read().await;
                    if bytes.len() as u64 > max_bytes {
                        Err(format!(
                            "Selected {} exceeds the supported {}-byte size limit",
                            name, max_bytes
                        ))
                    } else {
                        String::from_utf8(bytes)
                            .map(|contents| Some(PickedTextFile { name, contents }))
                            .map_err(|error| format!("Selected file is not valid UTF-8: {error}"))
                    }
                }
            }
            None => Ok(None),
        };

        complete_text_import(result, on_complete, request_browser_import_repaint);
    });
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn request_browser_import_repaint() {
    if request_registered_import_repaint() {
        return;
    }

    use wasm_bindgen::JsCast as _;

    if let Some(window) = web_sys::window() {
        let callback = wasm_bindgen::closure::Closure::<dyn FnMut()>::once(|| {});
        let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
        callback.forget();
    }
}

#[cfg(test)]
fn reset_text_import_gate_for_tests() {
    ACTIVE_TEXT_IMPORT.with(|active| active.set(None));
    NEXT_TEXT_IMPORT_GENERATION.with(|next| next.set(1));
}

#[cfg(test)]
fn clear_text_import_repaint_context_for_tests() {
    TEXT_IMPORT_REPAINT_CONTEXT.with(|slot| {
        *slot.borrow_mut() = None;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn text_import_gate_serializes_schematic_and_project_pickers() {
        reset_text_import_gate_for_tests();

        let schematic =
            try_begin_text_import(BrowserTextImportKind::Schematic).expect("schematic starts");
        let blocked = try_begin_text_import(BrowserTextImportKind::Project)
            .expect_err("project import is blocked while schematic import is active");

        assert!(blocked.contains("file import is already in progress"));

        assert!(finish_text_import(schematic));
        let project = try_begin_text_import(BrowserTextImportKind::Project)
            .expect("project starts after release");
        let blocked = try_begin_text_import(BrowserTextImportKind::VerilogA)
            .expect_err("Verilog-A import is blocked while project import is active");

        assert!(blocked.contains("file import is already in progress"));

        assert!(finish_text_import(project));
        let netlist =
            try_begin_text_import(BrowserTextImportKind::Netlist).expect("netlist starts");
        let blocked = try_begin_text_import(BrowserTextImportKind::VerilogA)
            .expect_err("Verilog-A import is blocked while netlist import is active");

        assert!(blocked.contains("file import is already in progress"));

        assert!(finish_text_import(netlist));
        let veriloga =
            try_begin_text_import(BrowserTextImportKind::VerilogA).expect("Verilog-A starts");
        assert!(finish_text_import(veriloga));
    }

    #[test]
    fn cancelled_picker_late_completion_cannot_release_replacement_picker() {
        reset_text_import_gate_for_tests();
        let cancelled =
            try_begin_text_import(BrowserTextImportKind::Project).expect("first picker starts");
        assert_eq!(cancel_active_text_import(), Some(cancelled));
        assert!(!text_import_is_current(cancelled));

        let replacement =
            try_begin_text_import(BrowserTextImportKind::Project).expect("replacement starts");
        assert_ne!(cancelled, replacement);
        assert!(!finish_text_import(cancelled));
        assert!(text_import_is_current(replacement));
        assert!(finish_text_import(replacement));
    }

    #[test]
    fn text_import_completion_runs_callback_then_wakes_ui() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let callback_events = Rc::clone(&events);
        let wake_events = Rc::clone(&events);

        complete_text_import(
            Ok(None),
            |_| callback_events.borrow_mut().push("callback"),
            || wake_events.borrow_mut().push("wake"),
        );

        assert_eq!(&*events.borrow(), &["callback", "wake"]);
    }

    #[test]
    fn registered_repaint_context_is_requested_when_import_completes() {
        let ctx = egui::Context::default();
        register_text_import_repaint_context(&ctx);

        request_registered_import_repaint();

        assert!(
            ctx.has_requested_repaint(),
            "browser import completion should wake the egui repaint loop"
        );
        clear_text_import_repaint_context_for_tests();
    }
}
