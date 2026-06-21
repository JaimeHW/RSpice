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
    VerilogA,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl BrowserTextImportKind {
    fn label(self) -> &'static str {
        match self {
            Self::Netlist => "SPICE deck",
            Self::Project => "project",
            Self::Schematic => "schematic",
            Self::VerilogA => "Verilog-A source",
        }
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
thread_local! {
    static ACTIVE_TEXT_IMPORT: std::cell::Cell<Option<BrowserTextImportKind>> =
        const { std::cell::Cell::new(None) };
    static TEXT_IMPORT_REPAINT_CONTEXT: std::cell::RefCell<Option<egui::Context>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn try_begin_text_import(kind: BrowserTextImportKind) -> Result<(), String> {
    ACTIVE_TEXT_IMPORT.with(|active| match active.get() {
        Some(current) if current == kind => Err(format!(
            "A {} import is already in progress",
            current.label()
        )),
        Some(_) => Err("Another file import is already in progress".to_string()),
        None => {
            active.set(Some(kind));
            Ok(())
        }
    })
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn finish_text_import(kind: BrowserTextImportKind) {
    ACTIVE_TEXT_IMPORT.with(|active| {
        if active.get() == Some(kind) {
            active.set(None);
        }
    });
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
    wasm_bindgen_futures::spawn_local(async move {
        let file = rfd::AsyncFileDialog::new()
            .add_filter(filter_name, filter_extensions)
            .pick_file()
            .await;

        let result = match file {
            Some(file) => {
                let name = file.file_name();
                let bytes = file.read().await;
                String::from_utf8(bytes)
                    .map(|contents| Some(PickedTextFile { name, contents }))
                    .map_err(|error| format!("Selected file is not valid UTF-8: {error}"))
            }
            None => Ok(None),
        };

        complete_text_import(result, on_complete, request_browser_import_repaint);
    });
}

#[cfg(target_arch = "wasm32")]
fn request_browser_import_repaint() {
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

        try_begin_text_import(BrowserTextImportKind::Schematic).expect("schematic starts");
        let blocked = try_begin_text_import(BrowserTextImportKind::Project)
            .expect_err("project import is blocked while schematic import is active");

        assert!(blocked.contains("file import is already in progress"));

        finish_text_import(BrowserTextImportKind::Schematic);
        try_begin_text_import(BrowserTextImportKind::Project)
            .expect("project starts after release");
        let blocked = try_begin_text_import(BrowserTextImportKind::VerilogA)
            .expect_err("Verilog-A import is blocked while project import is active");

        assert!(blocked.contains("file import is already in progress"));

        finish_text_import(BrowserTextImportKind::Project);
        try_begin_text_import(BrowserTextImportKind::Netlist).expect("netlist starts");
        let blocked = try_begin_text_import(BrowserTextImportKind::VerilogA)
            .expect_err("Verilog-A import is blocked while netlist import is active");

        assert!(blocked.contains("file import is already in progress"));

        finish_text_import(BrowserTextImportKind::Netlist);
        try_begin_text_import(BrowserTextImportKind::VerilogA).expect("Verilog-A starts");
        finish_text_import(BrowserTextImportKind::VerilogA);
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
