//! Read-only observation of the real browser workbench's rendered controls.
//!
//! This module is excluded from ordinary builds. It provides no commands,
//! state mutation, file injection, or authorization overrides: qualification
//! drives the canvas through the browser's normal pointer and keyboard input.
//! A 100 ms poll keeps observation responsive while idle. This instrumented
//! image is for functional qualification, not idle-work or performance budgets.

#[derive(Default)]
pub(crate) struct ControlObserver {
    last_request: Option<String>,
}

impl egui::Plugin for ControlObserver {
    fn debug_name(&self) -> &'static str {
        "browser qualification controls"
    }

    fn setup(&mut self, ctx: &egui::Context) {
        ctx.enable_accesskit();
    }

    fn output_hook(&mut self, ctx: &egui::Context, output: &mut egui::FullOutput) {
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
        // egui can discard an initial layout pass and immediately draw again.
        // Only the pass the backend will paint is usable as click authority.
        if !output.platform_output.request_discard_reasons.is_empty()
            && output.platform_output.num_completed_passes
                < ctx.options(|options| options.max_passes.get())
        {
            return;
        }
        let Some(tree) = &output.platform_output.accesskit_update else {
            return;
        };
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let Some(root) = document.document_element() else {
            return;
        };
        let request = root
            .get_attribute("data-rspice-qualification-request")
            .unwrap_or_default();
        if self.last_request.as_ref() == Some(&request) {
            return;
        }
        let snapshot = serde_json::json!({
            "schema": 1,
            "request": request,
            "frame": ctx.cumulative_frame_nr(),
            "pixels_per_point": output.pixels_per_point,
            "tree": tree,
        });
        let publish = || -> Result<(), wasm_bindgen::JsValue> {
            let element = match document.get_element_by_id("rspice_qualification_snapshot") {
                Some(element) => element,
                None => {
                    let element = document.create_element("script")?;
                    element.set_attribute("type", "application/json")?;
                    element.set_id("rspice_qualification_snapshot");
                    root.append_child(&element)?;
                    element
                }
            };
            element.set_text_content(Some(&snapshot.to_string()));
            Ok(())
        };
        match publish() {
            Ok(()) => self.last_request = Some(request),
            Err(error) => log::error!("Could not publish browser control snapshot: {error:?}"),
        }
    }
}
